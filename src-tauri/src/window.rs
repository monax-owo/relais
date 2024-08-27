use std::sync::{atomic::Ordering, Arc, Mutex};

use anyhow::bail;
use serde_json::Value;
// use serde::{Deserialize, Serialize};
// use specta::Type;
use tauri::{
  AppHandle, Manager, PhysicalPosition, PhysicalSize, State, Window, WindowBuilder, WindowEvent,
  WindowUrl,
};
use uuid::Uuid;
use windows::Win32::UI::WindowsAndMessaging::{
  SetLayeredWindowAttributes, SetWindowLongPtrW, GWL_EXSTYLE, LWA_ALPHA, WS_EX_LAYERED,
};

use crate::{AppState, WindowData};

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
const CTRL_WINDOW_SIZE: (u32, u32) = (40, 260);
const LABEL_PREFIX: &str = "ctrl_";
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

  // create window
  let title = title.unwrap_or_default();
  let label = label.unwrap_or(Uuid::new_v4().to_string());
  let window = WindowBuilder::new(&app, &label, WindowUrl::External(parse_url))
    .decorations(false)
    .initialization_script(include_str!("./init.js"))
    .maximizable(false)
    .min_inner_size(500.0, 500.0)
    .minimizable(true)
    .title(&title)
    .transparent(true)
    .build()
    .map_err(|_| ())?;

  let ctrl_window = WindowBuilder::new(
    &app,
    LABEL_PREFIX.to_string() + &label,
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
  // #[cfg(target_os = "windows")]
  // {
  //   use windows::Win32::UI::WindowsAndMessaging::SetParent;

  //   let handle_window = window.hwnd().map_err(|_| ())?;
  //   let handle_ctrl_window = window.hwnd().map_err(|_| ())?;

  //   unsafe {
  //     println!("unsafe");
  //     let _handle = SetParent(handle_ctrl_window, handle_window);
  //   }
  // }

  let window_data = WindowData {
    title,
    label: label.clone(),
    zoom: Arc::from(Mutex::from(1.0)),
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
    let window_hwnd = arc.0.hwnd().map_err(|_| ())?;

    unsafe {
      SetWindowLongPtrW(window_hwnd, GWL_EXSTYLE, WS_EX_LAYERED.0 as isize);
    }

    // TODO: zoom
    // AppStateのoverlayが無効のときのみctrlを表示+有効のときはwindowを半透明にする
    // if window closing, when remove if from window list
    window.on_window_event({
      let arc = arc.clone();
      let app = app.clone();
      move |e| match *e {
        WindowEvent::CloseRequested { .. } => close(&app, &arc).unwrap(),
        WindowEvent::Focused(state) => {
          if state {
            arc.1.show().unwrap();

            arc
              .0
              .set_position(ctrl_pos(arc.1.outer_position().unwrap()))
              .unwrap();
          } else if !arc.1.is_focused().unwrap() {
            arc.1.hide().unwrap();
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
      let app = app.clone();
      move |e| match *e {
        WindowEvent::Focused(state) => {
          if state && !app.state::<AppState>().overlay.load(Ordering::Acquire) {
            dbg!("a");
            if arc.0.is_minimized().unwrap() {
              arc.0.unminimize().unwrap();
            }
            arc
              .0
              .set_position(ctrl_pos(arc.1.outer_position().unwrap()))
              .unwrap();
            // if !arc.0.is_visible().unwrap() {
            //   arc.0.show().unwrap();
            // }
          } else if !arc.0.is_focused().unwrap() && !arc.1.is_focused().unwrap() {
            arc.1.hide().unwrap();
          }
        }
        // arc.0.start_dragging()
        WindowEvent::Moved(pos) => {
          arc.0.set_position(ctrl_pos(pos)).unwrap();
        }
        _ => (),
      }
    });

    ctrl_window.listen("ctrl", {
      let arc = arc.clone();
      let app = app.clone();
      move |e| {
        if let Some(v) = e.payload() {
          let payload = serde_json::from_str::<Value>(v).unwrap();
          match payload {
            Value::Null => todo!(),
            Value::Bool(_) => todo!(),
            Value::Number(_) => todo!(),
            Value::String(v) => match v.as_str() {
              "mini" => arc.0.minimize().unwrap(),
              "close" => close(&app, &arc).unwrap(),
              // "transparent" => toggle_transparent(&app).unwrap(),
              "transparent" => arc
                .1
                .emit_all(
                  "transparent",
                  toggle_transparent(&app, &arc.0, &arc.1, 128).unwrap(),
                )
                .unwrap(),
              "zoomout" => set_zoom(&app, &arc.0, -0.1).unwrap(),
              "zoomin" => set_zoom(&app, &arc.0, 0.1).unwrap(),
              _ => println!("did not match: {}", v),
            },
            Value::Array(_) => todo!(),
            Value::Object(_) => todo!(),
          }
        }
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

  app.get_window("main").unwrap().hide().unwrap();

  Ok(())
}

fn close(app: &AppHandle, arc: &Arc<(Window, Window)>) -> anyhow::Result<()> {
  let state = app.state::<AppState>();
  let labels = [arc.0.label(), arc.1.label()];
  arc.1.close()?;
  arc.0.close()?;
  state.remove_window(labels[0])?;
  state.remove_window(labels[1])?;
  state.sync_windows(app);
  Ok(())
}

pub fn toggle_transparent(
  app: &AppHandle,
  window: &Window,
  ctrl_window: &Window,
  alpha: u8,
) -> anyhow::Result<bool> {
  let state = app.state::<AppState>();
  let window_hwnd = window.hwnd()?;
  let condition = state.overlay.load(Ordering::Acquire);
  // TODO
  // もし半透明モードなら反転
  if condition {
    // 不透明
    let res = unsafe { SetLayeredWindowAttributes(window_hwnd, 0, 255, LWA_ALPHA) };
    let res_as_bool = res.as_bool();
    if !res_as_bool {
      bail!("failed to set window style");
    }

    state.overlay.store(false, Ordering::Release);
  } else {
    // 半透明
    let _res = unsafe { SetLayeredWindowAttributes(window_hwnd, 0, alpha, LWA_ALPHA) };

    ctrl_window.hide().unwrap();
    dbg!("hide");

    state.overlay.store(true, Ordering::Release);
  };
  // unsafe {}
  Ok(!condition)
}

fn set_zoom(app: &AppHandle, window: &Window, diff: f64) -> anyhow::Result<()> {
  let state = app.state::<AppState>();
  let Some(window_data) = state.get_window_data(window.label()) else {
    bail!("failed to get window data");
  };

  window.with_webview(move |webview| {
    #[cfg(windows)]
    unsafe {
      // see https://docs.rs/webview2-com/0.19.1/webview2_com/Microsoft/Web/WebView2/Win32/struct.ICoreWebView2Controller.html
      webview
        .controller()
        .SetZoomFactor(window_data.zoom + diff)
        .unwrap();
    }
  })?;

  // TODO: window_data.zoomを上書きできるようにする
  // WindowDataと別にSerialize用のWindowData型を作ったほうがいいかも
  // window_data.zoom.set

  Ok(())
}
//

//
#[tauri::command]
#[specta::specta]
pub fn get_transparent(state: State<'_, AppState>) -> Result<bool, &str> {
  Ok(state.overlay.load(Ordering::Acquire))
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
