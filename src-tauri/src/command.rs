use std::sync::Arc;

// use serde::{Deserialize, Serialize};
// use specta::Type;
use tauri::{
  AppHandle, Manager, PhysicalPosition, PhysicalSize, State, Window, WindowBuilder, WindowEvent,
  WindowUrl,
};
use uuid::Uuid;

use crate::{AppState, WindowData};

#[specta::specta]
pub fn export_types(_a: WindowData) {}

//
pub fn exit_0(handle: &AppHandle) -> anyhow::Result<()> {
  handle.tray_handle().destroy()?;
  handle.exit(0);
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn exit(app: AppHandle, _window: Window) -> Result<(), String> {
  exit_0(&app).map_err(|e| e.to_string())
}
//

//
pub fn _window_focus(window: &Window) -> anyhow::Result<()> {
  window.show()?;
  window.set_focus()?;
  // window.set_always_on_top(true)?;
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn window_focus(_app: AppHandle, window: Window) -> Result<(), String> {
  _window_focus(&window).map_err(|e| e.to_string())?;
  Ok(())
}
//

//
pub fn _window_hide(window: &Window) -> anyhow::Result<()> {
  window.hide()?;
  // window.set_always_on_top(false)?;
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn window_hide(_app: AppHandle, window: Window) -> Result<(), String> {
  _window_hide(&window).map_err(|e| e.to_string())?;
  Ok(())
}
//

//
const CTRL_WINDOW_SIZE: (u32, u32) = (40, 160);
//
#[tauri::command]
#[specta::specta]
pub async fn open_window(
  app: AppHandle,
  _window: Window,
  state: State<'_, AppState>,
  url: String,
  title: Option<String>,
  label: Option<String>,
) -> Result<(), ()> {
  let url = if url.starts_with("http") {
    url
  } else {
    format!("https://{}", url)
  };
  let parse_url = url::Url::parse(&url).map_err(|_| ())?;
  dbg!(&parse_url);

  // create window
  // TODO: ウィンドウの位置を決める、innersizeだとずれてしまうのでoutersizeを指定する方法を探す
  // BuilderではなくWindowに対してset_sizeすれば良さそう？
  let title = title.unwrap_or_default();
  let label = label.unwrap_or(Uuid::new_v4().to_string());
  let window = WindowBuilder::new(&app, &label, WindowUrl::External(parse_url))
    .decorations(false)
    .title(&title)
    .transparent(true)
    .build()
    .map_err(|_| ())?;

  let ctrl_window = WindowBuilder::new(
    &app,
    "ctrl_".to_string() + &label,
    WindowUrl::App("/ctrl".into()),
  )
  .decorations(false)
  .maximizable(false)
  .minimizable(false)
  .resizable(false)
  .skip_taskbar(true)
  .title("ctrl")
  .transparent(true)
  .build()
  .map_err(|_| ())?;

  // windows crate 0.39.0
  // set child window
  #[cfg(target_os = "windows")]
  {
    use windows::Win32::UI::WindowsAndMessaging::SetParent;

    let handle_window = window.hwnd().map_err(|_| ())?;
    let handle_ctrl_window = window.hwnd().map_err(|_| ())?;

    unsafe {
      println!("unsafe");
      let _handle = SetParent(handle_ctrl_window, handle_window);
    }
  }

  let window_data = WindowData {
    title,
    label: label.clone(),
    zoom: 1.0,
  };

  state.add_window(window_data).map_err(|_| ())?;
  state.sync_windows(&app);

  window
    .set_position(ctrl_pos(ctrl_window.outer_position().map_err(|_| ())?))
    .map_err(|_| ())?;

  ctrl_window.hide().map_err(|_| ())?;

  {
    let arc = Arc::new((window, ctrl_window));
    let (ref window, ref ctrl_window) = *Arc::clone(&arc);

    // ctrlで動かそうとするとwindowを1度フォーカスしてからctrlをフォーカスしている
    // if window closing, when remove if from window list
    window.on_window_event({
      let arc = arc.clone();
      move |e| match *e {
        WindowEvent::CloseRequested { .. } => {
          _close_window(app.clone(), label.clone()).unwrap();
        }
        WindowEvent::Focused(state) => {
          if state {
            if !arc.1.is_visible().unwrap() {
              arc.1.show().unwrap();
              dbg!("here");
            };
            arc
              .0
              .set_position(ctrl_pos(arc.1.outer_position().unwrap()))
              .unwrap();
            println!("window focus");
          }
        }
        WindowEvent::Resized(_) => {
          arc
            .1
            .set_position(window_pos(arc.0.outer_position().unwrap()))
            .unwrap();
        }
        _ => (),
      }
    });

    ctrl_window.on_window_event({
      let arc = arc.clone();
      move |e| match *e {
        WindowEvent::Focused(state) => {
          if state {
            if arc.0.is_minimized().unwrap() {
              arc.0.unminimize().unwrap();
              dbg!("here");
            }
            arc
              .0
              .set_position(ctrl_pos(arc.1.outer_position().unwrap()))
              .unwrap();
            if !arc.0.is_visible().unwrap() {
              arc.0.show().unwrap();
              dbg!("here");
            }
            println!("ctrl focus");
          } else if !arc.0.is_focused().unwrap() {
            arc.1.hide().unwrap();
            // ここが何故か実行される
            dbg!("here");
            println!("here");
          }
        }
        WindowEvent::Moved(pos) => {
          arc.0.set_position(ctrl_pos(pos)).unwrap();
        }
        _ => (),
      }
    });

    (|| -> anyhow::Result<()> {
      let diff_x = ctrl_window.outer_size()?.width - ctrl_window.inner_size()?.width;
      let diff_y = ctrl_window.outer_size()?.height - ctrl_window.inner_size()?.height;
      ctrl_window.set_size(PhysicalSize::new(
        diff_x + CTRL_WINDOW_SIZE.0,
        diff_y + CTRL_WINDOW_SIZE.1,
      ))?;
      Ok(())
    })()
    .map_err(|_| ())?;
  }

  Ok(())
}
//

//
fn _close_window(app: AppHandle, label: String) -> Result<(), ()> {
  let Some(window) = app.get_window(&label) else {
    return Err(());
  };
  window.close().map_err(|_| ())?;
  let state = app.state::<AppState>();
  state.remove_window(&label).map_err(|_| ())?;
  state.sync_windows(&app);

  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn close_window(
  app: AppHandle,
  _window: Window,
  // state: State<'_, AppState>,
  label: String,
) -> Result<(), ()> {
  _close_window(app, label.to_string()).map_err(|_| ())?;

  Ok(())
}
//

//
fn ctrl_pos(pos: PhysicalPosition<i32>) -> PhysicalPosition<i32> {
  const OFFSET: (i32, i32) = (40, 0);
  PhysicalPosition::new(pos.x + OFFSET.0, pos.y + OFFSET.1)
}

//
fn window_pos(pos: PhysicalPosition<i32>) -> PhysicalPosition<i32> {
  const OFFSET: (i32, i32) = (40, 0);
  PhysicalPosition::new(pos.x - OFFSET.0, pos.y - OFFSET.1)
}
