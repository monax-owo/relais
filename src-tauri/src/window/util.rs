use anyhow::{bail, Context};
use serde_json::Value;
use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc, Mutex,
};
use tauri::{
  AppHandle, Listener, Manager, PhysicalPosition, PhysicalSize, State, WebviewUrl, WebviewWindow,
  WebviewWindowBuilder, WindowEvent,
};
use windows::Win32::{
  Foundation::{COLORREF, HWND},
  UI::WindowsAndMessaging::{
    SetLayeredWindowAttributes, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, HWND_NOTOPMOST,
    HWND_TOPMOST, LWA_ALPHA, SWP_NOMOVE, SWP_NOSIZE, WS_EX_LAYERED,
  },
};

use crate::{SourceAppState, SourceWindowData};

pub const CTRL_SIZE: (u32, u32) = (40, 320);
pub const WINDOW_MIN_INNER_SIZE: (f64, f64) = (400.0, 400.0);
pub const WINDOW_LABEL_PREFIX: &str = "window_";
pub const CTRL_LABEL_PREFIX: &str = "ctrl_";

pub fn view_create(
  app: &AppHandle,
  state: State<'_, SourceAppState>,
  url: WebviewUrl,
  title: String,
  label: String,
) -> anyhow::Result<()> {
  let skip_taskbar = cfg!(not(debug_assertions));

  let window = WebviewWindowBuilder::new(app, &label, url)
    .decorations(false)
    .initialization_script(include_str!("./init.js"))
    .maximizable(false)
    .min_inner_size(WINDOW_MIN_INNER_SIZE.0, WINDOW_MIN_INNER_SIZE.1)
    .minimizable(true)
    .title(&title)
    .transparent(true)
    .zoom_hotkeys_enabled(true)
    .build()?;

  let ctrl_window =
    WebviewWindowBuilder::new(app, to_ctrl_label(&*label), WebviewUrl::App("/ctrl".into()))
      // .parent(&window)
      // .unwrap()
      .decorations(false)
      .maximizable(false)
      .minimizable(false)
      .resizable(false)
      .skip_taskbar(skip_taskbar)
      .title("ctrl")
      .transparent(true)
      .build()?;

  //   let handle_window = window.hwnd().map_err(|_| ())?;
  //   let handle_ctrl_window = window.hwnd().map_err(|_| ())?;

  let window_data = SourceWindowData {
    title,
    label: label.clone(),
    pin: Arc::from(AtomicBool::from(false)),
    zoom: Arc::from(Mutex::from(1.0)),
  };

  state.add_window(window_data)?;
  state.sync_windows(app);

  window.set_position(ctrl_pos(ctrl_window.outer_position()?))?;

  {
    let arc = Arc::new((window, ctrl_window));
    let (ref window, ref ctrl_window) = *Arc::clone(&arc);
    let window_hwnd = arc.0.hwnd()?;

    unsafe {
      SetWindowLongPtrW(window_hwnd, GWL_EXSTYLE, WS_EX_LAYERED.0 as isize);
    }

    dbg!(&window.label());
    dbg!(&ctrl_window.label());

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

    window.on_window_event(|e| match e {
      WindowEvent::Resized(_) => (),
      WindowEvent::Moved(_) => (),
      WindowEvent::CloseRequested { .. } => (),
      WindowEvent::Destroyed => (),
      WindowEvent::Focused(_) => (),
      WindowEvent::ScaleFactorChanged { .. } => (),
      WindowEvent::DragDrop(_) => (),
      WindowEvent::ThemeChanged(_) => (),
      _ => (),
    });

    ctrl_window.on_window_event(|e| match e {
      WindowEvent::Resized(_) => (),
      WindowEvent::Moved(_) => (),
      WindowEvent::CloseRequested { .. } => (),
      WindowEvent::Destroyed => (),
      WindowEvent::Focused(_) => (),
      WindowEvent::ScaleFactorChanged { .. } => (),
      WindowEvent::DragDrop(_) => (),
      WindowEvent::ThemeChanged(_) => (),
      _ => (),
    });

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
        diff_x + CTRL_SIZE.0,
        diff_y + CTRL_SIZE.1,
      ))?;
      Ok(())
    })()?;
  }

  // ctrl_window.hide()?;

  Ok(())
}

pub fn toggle_transparent(
  app: &AppHandle,
  window: &WebviewWindow,
  ctrl_window: &WebviewWindow,
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

pub fn set_transparent(hwnd: HWND, alpha: u8) -> anyhow::Result<()> {
  unsafe { SetLayeredWindowAttributes(hwnd, COLORREF(0), alpha, LWA_ALPHA)? };

  Ok(())
}

pub fn set_pin(window: &WebviewWindow, value: bool) -> Result<(), String> {
  // window.set_always_on_top(value).map_err(|v| v.to_string())?;
  let hwndinsertafter = if value { HWND_TOPMOST } else { HWND_NOTOPMOST };
  unsafe {
    SetWindowPos(
      window.hwnd().unwrap(),
      hwndinsertafter,
      0,
      0,
      0,
      0,
      SWP_NOMOVE | SWP_NOSIZE,
    )
    .map_err(|e| e.to_string())?
  }

  Ok(())
}

// TODO: f64の代わりにパーセントを使う
pub fn set_zoom(app: &AppHandle, window: &WebviewWindow, diff: f64) -> anyhow::Result<()> {
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
    window.set_zoom(scale)?;

    *lock += diff;
  } else {
    *lock = 1.0;
  }

  Ok(())
}

pub fn to_ctrl_label<'a, T: Into<&'a str>>(label: T) -> String {
  CTRL_LABEL_PREFIX.to_string() + label.into()
}

pub fn to_window_label<'a, T: Into<&'a str>>(label: T) -> String {
  label.into().replacen(CTRL_LABEL_PREFIX, "", 1)
}

pub fn ctrl_pos(pos: PhysicalPosition<i32>) -> PhysicalPosition<i32> {
  const OFFSET: (i32, i32) = (40, 0);
  PhysicalPosition::new(pos.x + OFFSET.0, pos.y + OFFSET.1)
}

pub fn window_pos(pos: PhysicalPosition<i32>) -> PhysicalPosition<i32> {
  const OFFSET: (i32, i32) = (40, 0);
  PhysicalPosition::new(pos.x - OFFSET.0, pos.y - OFFSET.1)
}

pub fn to_ctrl(window: &WebviewWindow) -> anyhow::Result<WebviewWindow> {
  window
    .get_webview_window(&to_ctrl_label(window.label()))
    .context("window is not found")
}

pub fn to_window(ctrl: &WebviewWindow) -> anyhow::Result<WebviewWindow> {
  ctrl
    .get_webview_window(&to_window_label(ctrl.label()))
    .context("ctrl is not found")
}

pub fn view_close(app: AppHandle, label: String) -> anyhow::Result<()> {
  let window = app
    .get_webview_window(&label)
    .context("failed to get window")?;
  window.close()?;
  let state = app.state::<SourceAppState>();
  state.remove_window(&label)?;
  state.sync_windows(&app);

  Ok(())
}

pub fn close(app: &AppHandle, arc: &Arc<(WebviewWindow, WebviewWindow)>) -> anyhow::Result<()> {
  let state = app.state::<SourceAppState>();
  let label = arc.0.label();
  arc.1.close()?;
  arc.0.close()?;
  state.remove_window(label)?;
  state.sync_windows(app);

  Ok(())
}

pub fn window_focus(window: &WebviewWindow) -> anyhow::Result<()> {
  window.show()?;
  window.set_focus()?;
  // window.set_always_on_top(true)?;

  Ok(())
}

pub fn window_hide(window: &WebviewWindow) -> anyhow::Result<()> {
  window.hide()?;
  // window.set_always_on_top(false)?;

  Ok(())
}

pub fn window_minimize(window: &WebviewWindow) -> anyhow::Result<()> {
  window.minimize()?;

  Ok(())
}
