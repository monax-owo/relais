use std::sync::Arc;

// use serde::{Deserialize, Serialize};
// use specta::Type;
use tauri::{
  AppHandle, Manager, PhysicalPosition, State, Window, WindowBuilder, WindowEvent, WindowUrl,
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
  let title = title.unwrap_or_default();
  let label = label.unwrap_or(Uuid::new_v4().to_string());
  let window = WindowBuilder::new(&app, &label, WindowUrl::External(parse_url))
    .decorations(false)
    .title(&title)
    .transparent(true)
    .build()
    .unwrap();

  let ctrl_window = WindowBuilder::new(
    &app,
    "ctrl_".to_string() + &label,
    WindowUrl::App("/ctrl".into()),
  )
  // .resizable(false)
  .skip_taskbar(true)
  .title("")
  .transparent(true)
  .build()
  .unwrap();

  let window_data = WindowData {
    title,
    label: label.clone(),
    zoom: 1.0,
  };

  state.add_window(window_data).map_err(|_| ())?;
  state.sync_windows(&app);

  {
    let pos = ctrl_window.outer_position().unwrap();
    window.set_position(ctrl_pos(pos.x, pos.y)).unwrap();
  }

  {
    let arc = Arc::new((window, ctrl_window));
    let (ref window, ref ctrl_window) = *Arc::clone(&arc);
    // if window closing, when remove if from window list
    window.on_window_event({
      let arc = arc.clone();
      move |e| match *e {
        WindowEvent::CloseRequested { .. } => {
          _close_window(app.clone(), label.clone()).unwrap();
        }
        WindowEvent::Focused(state) if state => arc.1.show().unwrap(),
        _ => (),
      }
    });

    ctrl_window.on_window_event({
      let arc = arc.clone();
      move |e| match *e {
        WindowEvent::Focused(state) if state => arc.0.show().unwrap(),
        WindowEvent::Moved(pos) => {
          arc.0.set_position(ctrl_pos(pos.x, pos.y)).unwrap();
        }
        _ => (),
      }
    });
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
// PhysicalPositionを渡せるようにしたほうがRustらしいと思う
fn ctrl_pos(x: i32, y: i32) -> PhysicalPosition<i32> {
  const OFFSET: (i32, i32) = (100, 100);
  PhysicalPosition::new(x + OFFSET.0, y + OFFSET.1)
}
