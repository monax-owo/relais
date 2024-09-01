use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc, Mutex,
};

use serde_json::Value;
// use serde::{Deserialize, Serialize};
// use specta::Type;
use tauri::{
  AppHandle, Listener, Manager, PhysicalSize, State, WebviewUrl, WebviewWindow,
  WebviewWindowBuilder,
};
use uuid::Uuid;
use windows::Win32::UI::WindowsAndMessaging::{SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_LAYERED};

use crate::{SourceAppState, SourceWindowData};

use super::util::*;

#[tauri::command]
#[specta::specta]
pub fn window_hide(window: WebviewWindow) -> Result<(), String> {
  _window_hide(&window).map_err(|e| e.to_string())?;
  Ok(())
}

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
  let label =
    label.unwrap_or(WINDOW_LABEL_PREFIX.to_string() + Uuid::new_v4().to_string().as_str());
  let window = WebviewWindowBuilder::new(&app, &label, WebviewUrl::External(parse_url))
    .decorations(false)
    .initialization_script(include_str!("./init.js"))
    // .maximizable(false)
    .min_inner_size(MIN_INNER_SIZE.0, MIN_INNER_SIZE.1)
    // .minimizable(true)
    .title(&title)
    .transparent(true)
    .build()
    .map_err(|_| ())?;

  let ctrl_window = WebviewWindowBuilder::new(
    &app,
    to_ctrl_window_label(&*label),
    WebviewUrl::App("/ctrl".into()),
  )
  // .parent(&window)
  // .unwrap()
  .decorations(false)
  // .maximizable(false)
  // .minimizable(false)
  .resizable(false)
  // .skip_taskbar(true)
  .title("ctrl")
  .transparent(true)
  .build()
  .map_err(|_| ())?;

  dbg!(window.label());
  dbg!(ctrl_window.label());

  // windows crate 0.39.0
  // set child window
  // #[cfg(target_os = "windows")]
  // {
  //   use windows::Win32::UI::WindowsAndMessaging::SetParent;

  //   let handle_window = window.hwnd().map_err(|_| ())?;
  //   let handle_ctrl_window = window.hwnd().map_err(|_| ())?;

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

  // ctrl_window.hide().map_err(|_| ())?;

  {
    let arc = Arc::new((window, ctrl_window));
    let (ref _window, ref ctrl_window) = *Arc::clone(&arc);
    let window_hwnd = arc.0.hwnd().map_err(|_| ())?;

    unsafe {
      SetWindowLongPtrW(window_hwnd, GWL_EXSTYLE, WS_EX_LAYERED.0 as isize);
    }

    // AppStateのoverlayが無効のときのみctrlを表示+有効のときはwindowを半透明にする
    // if window closing, when remove if from window list

    // window.on_window_event({
    //   let arc = arc.clone();
    //   let app = app.clone();
    //   move |e| match *e {
    //     WindowEvent::CloseRequested { .. } => close(&app, &arc).unwrap(),
    //     WindowEvent::Focused(state) => {
    //       if state {
    //         arc.1.show().unwrap();

    //         arc
    //           .0
    //           .set_position(ctrl_pos(arc.1.outer_position().unwrap()))
    //           .unwrap();
    //       } else if !arc.1.is_focused().unwrap() {
    //         arc.1.hide().unwrap();
    //       }
    //     }
    //     WindowEvent::Resized(_) => {
    //       arc
    //         .1
    //         .set_position(window_pos(arc.0.outer_position().unwrap()))
    //         .unwrap();
    //     }
    //     _ => (),
    //   }
    // });

    // ctrl_window.on_window_event({
    //   let arc = arc.clone();
    //   let app = app.clone();
    //   move |e| match *e {
    //     WindowEvent::Focused(state) => {
    //       if state
    //         && !app
    //           .state::<SourceAppState>()
    //           .overlay
    //           .load(Ordering::Acquire)
    //       {
    //         if arc.0.is_minimized().unwrap() {
    //           arc.0.unminimize().unwrap();
    //         }
    //         arc
    //           .0
    //           .set_position(ctrl_pos(arc.1.outer_position().unwrap()))
    //           .unwrap();
    //       } else if !arc.0.is_focused().unwrap() && !arc.1.is_focused().unwrap() {
    //         arc.1.hide().unwrap();
    //       }
    //     }
    //     // arc.0.start_dragging()
    //     WindowEvent::Moved(pos) => {
    //       arc.0.set_position(ctrl_pos(pos)).unwrap();
    //     }
    //     _ => (),
    //   }
    // });

    // commandに切り分けたほうが良さそう<-commandに分けないと動作がおかしい
    // 実装し直す<-commandにするだけで良さそう
    ctrl_window.listen("ctrl", {
      let arc = arc.clone();
      let app = app.clone();
      move |e| {
        let payload = serde_json::from_str::<Value>(e.payload()).unwrap();
        match payload {
          Value::Null => todo!(),
          Value::Bool(_) => todo!(),
          Value::Number(_) => todo!(),
          Value::String(v) => match v.as_str() {
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

#[tauri::command]
#[specta::specta]
pub fn mini(window: WebviewWindow) -> Result<(), String> {
  _mini(&window).map_err(|e| e.to_string())?;
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn get_transparent(state: State<'_, SourceAppState>) -> Result<bool, String> {
  Ok(state.overlay.load(Ordering::Acquire))
}

#[tauri::command]
#[specta::specta]
pub fn toggle_pin(
  app: AppHandle,
  window: WebviewWindow,
  state: State<'_, SourceAppState>,
) -> Result<bool, String> {
  let Some(window_data) = state.get_window_data(&to_window_label(window.label())) else {
    return Err("failed to get window data".to_string());
  };
  let atomic = Arc::clone(&window_data.pin);
  let pinned = atomic.load(Ordering::Acquire);
  dbg!(pinned);
  set_pin(
    &app
      .get_webview_window(&to_window_label(window.label()))
      .unwrap(),
    !pinned,
  )?;
  atomic.store(!pinned, Ordering::Release);
  Ok(!pinned)
}

#[tauri::command]
#[specta::specta]
pub fn close_window(app: AppHandle, label: String) -> Result<(), ()> {
  _close_window(app, label.to_string()).map_err(|_| ())?;

  Ok(())
}
