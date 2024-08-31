use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc, Mutex,
};

use anyhow::bail;
use serde_json::Value;
// use serde::{Deserialize, Serialize};
// use specta::Type;
use tauri::{
  AppHandle, Manager, PhysicalPosition, PhysicalSize, State, Window, WindowBuilder, WindowEvent,
  WindowUrl,
};
use uuid::Uuid;
use windows::Win32::{
  Foundation::HWND,
  UI::WindowsAndMessaging::{
    SetLayeredWindowAttributes, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, HWND_NOTOPMOST,
    HWND_TOPMOST, LWA_ALPHA, SWP_NOMOVE, SWP_NOSIZE, WS_EX_LAYERED,
  },
};

use crate::{SourceAppState, SourceWindowData};

//
pub fn _window_hide(window: &Window) -> anyhow::Result<()> {
  window.hide()?;
  // window.set_always_on_top(false)?;
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn window_hide(window: Window) -> Result<(), String> {
  _window_hide(&window).map_err(|e| e.to_string())?;
  Ok(())
}
//

//
const CTRL_WINDOW_SIZE: (u32, u32) = (40, 320);
const LABEL_PREFIX: &str = "ctrl_";
const MIN_INNER_SIZE: (f64, f64) = (400.0, 400.0);
//

#[tauri::command]
#[specta::specta]
pub async fn open_window(
  app: AppHandle,
  state: State<'_, SourceAppState>,
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
    .min_inner_size(MIN_INNER_SIZE.0, MIN_INNER_SIZE.1)
    .minimizable(true)
    .title(&title)
    .transparent(true)
    .build()
    .map_err(|_| ())?;

  let ctrl_window = WindowBuilder::new(
    &app,
    to_ctrl_window_label(&*label),
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

  let window_data = SourceWindowData {
    title,
    label: label.clone(),
    pin: Arc::from(AtomicBool::from(false)),
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
          if state
            && !app
              .state::<SourceAppState>()
              .overlay
              .load(Ordering::Acquire)
          {
            if arc.0.is_minimized().unwrap() {
              arc.0.unminimize().unwrap();
            }
            arc
              .0
              .set_position(ctrl_pos(arc.1.outer_position().unwrap()))
              .unwrap();
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

    // commandに切り分けたほうが良さそう
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
              "transparent" => {
                toggle_transparent(&app, &arc.0, &arc.1, 128).unwrap();
              }
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
//

//
fn close(app: &AppHandle, arc: &Arc<(Window, Window)>) -> anyhow::Result<()> {
  let state = app.state::<SourceAppState>();
  let label = arc.0.label();
  arc.1.close()?;
  arc.0.close()?;
  state.remove_window(label)?;
  state.sync_windows(app);
  Ok(())
}
//

//
pub fn toggle_transparent(
  app: &AppHandle,
  window: &Window,
  ctrl_window: &Window,
  alpha: u8,
) -> anyhow::Result<bool> {
  let state = app.state::<SourceAppState>();
  let window_hwnd = window.hwnd()?;
  let condition = state.overlay.load(Ordering::Acquire);
  // TODO: カーソル通過
  if condition {
    // 不透明
    set_transparent(window_hwnd, 255).unwrap();
    ctrl_window.show().unwrap();

    state.overlay.store(false, Ordering::Release);
  } else {
    // 半透明
    set_transparent(window_hwnd, alpha).unwrap();
    ctrl_window.hide().unwrap();

    state.overlay.store(true, Ordering::Release);
  };
  // unsafe {}
  Ok(!condition)
}

fn set_transparent(hwnd: HWND, alpha: u8) -> anyhow::Result<()> {
  let res = unsafe { SetLayeredWindowAttributes(hwnd, 0, alpha, LWA_ALPHA) };
  if res.as_bool() {
    Ok(())
  } else {
    bail!("")
  }
}

#[tauri::command]
#[specta::specta]
pub fn get_transparent(state: State<'_, SourceAppState>) -> Result<bool, String> {
  Ok(state.overlay.load(Ordering::Acquire))
}
//

//
#[tauri::command]
#[specta::specta]
pub fn toggle_pin(
  app: AppHandle,
  window: Window,
  state: State<'_, SourceAppState>,
) -> Result<bool, String> {
  let Some(window_data) = state.get_window_data(&to_window_label(window.label())) else {
    return Err("failed to get window data".to_string());
  };
  let atomic = Arc::clone(&window_data.pin);
  let pinned = atomic.load(Ordering::Acquire);
  dbg!(pinned);
  set_pin(
    &app.get_window(&to_window_label(window.label())).unwrap(),
    !pinned,
  )?;
  atomic.store(!pinned, Ordering::Release);
  Ok(!pinned)
}

fn set_pin(window: &Window, value: bool) -> Result<(), String> {
  // window.set_always_on_top(value).map_err(|v| v.to_string())?;
  let hwndinsertafter = if value { HWND_TOPMOST } else { HWND_NOTOPMOST };
  let res = unsafe {
    SetWindowPos(
      window.hwnd().unwrap(),
      hwndinsertafter,
      0,
      0,
      0,
      0,
      SWP_NOMOVE | SWP_NOSIZE,
    )
  }
  .as_bool();

  if !res {
    return Err("".to_string());
  }

  Ok(())
}
//

//
fn set_zoom(app: &AppHandle, window: &Window, diff: f64) -> anyhow::Result<()> {
  let state = app.state::<SourceAppState>();
  let Some(window_data) = state.get_window_data(window.label()) else {
    bail!("failed to get window data");
  };
  let zoom = window_data.zoom.clone();
  let mut lock = zoom.lock().unwrap();
  dbg!(*lock);

  let scale = *lock + diff;
  // TODO: 20%~500%
  if scale > 0.2 {
    window.with_webview({
      move |webview| {
        #[cfg(windows)]
        unsafe {
          // see https://docs.rs/webview2-com/0.19.1/webview2_com/Microsoft/Web/WebView2/Win32/struct.ICoreWebView2Controller.html
          webview.controller().SetZoomFactor(scale).unwrap();
        }
      }
    })?;

    *lock += diff;
  } else {
    *lock += 1.0;
  }

  Ok(())
}
//

//
pub fn to_ctrl_window_label<'a, T: Into<&'a str>>(label: T) -> String {
  LABEL_PREFIX.to_string() + label.into()
}

pub fn to_window_label<'a, T: Into<&'a str>>(label: T) -> String {
  label.into().replacen(LABEL_PREFIX, "", 1)
}
//

//
fn _close_window(app: AppHandle, label: String) -> Result<(), ()> {
  let Some(window) = app.get_window(&label) else {
    return Err(());
  };
  window.close().map_err(|_| ())?;
  let state = app.state::<SourceAppState>();
  state.remove_window(&label).map_err(|_| ())?;
  state.sync_windows(&app);

  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn close_window(app: AppHandle, label: String) -> Result<(), ()> {
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
