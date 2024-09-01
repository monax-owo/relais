use std::sync::{atomic::Ordering, Arc};

use anyhow::bail;
// use serde::{Deserialize, Serialize};
// use specta::Type;
use tauri::{AppHandle, Manager, PhysicalPosition, WebviewWindow};
use windows::Win32::{
  Foundation::{COLORREF, HWND},
  UI::WindowsAndMessaging::{
    SetLayeredWindowAttributes, SetWindowPos, HWND_NOTOPMOST, HWND_TOPMOST, LWA_ALPHA, SWP_NOMOVE,
    SWP_NOSIZE,
  },
};

use crate::SourceAppState;

pub const CTRL_WINDOW_SIZE: (u32, u32) = (40, 320);
pub const WINDOW_LABEL_PREFIX: &str = "window_";
pub const CTRL_LABEL_PREFIX: &str = "ctrl_";
pub const MIN_INNER_SIZE: (f64, f64) = (400.0, 400.0);

pub fn _window_hide(window: &WebviewWindow) -> anyhow::Result<()> {
  window.hide()?;
  // window.set_always_on_top(false)?;
  Ok(())
}

pub fn _mini(window: &WebviewWindow) -> anyhow::Result<()> {
  window.minimize()?;
  Ok(())
}

pub fn close(app: &AppHandle, arc: &Arc<(WebviewWindow, WebviewWindow)>) -> anyhow::Result<()> {
  let state = app.state::<SourceAppState>();
  let label = arc.0.label();
  arc.1.close().unwrap();
  arc.0.close().unwrap();
  state.remove_window(label)?;
  state.sync_windows(app);
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

// TODO: ズームをセットする関数がv2にあると思う
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

pub fn to_ctrl_window_label<'a, T: Into<&'a str>>(label: T) -> String {
  CTRL_LABEL_PREFIX.to_string() + label.into()
}

pub fn to_window_label<'a, T: Into<&'a str>>(label: T) -> String {
  label.into().replacen(CTRL_LABEL_PREFIX, "", 1)
}

pub fn _close_window(app: AppHandle, label: String) -> Result<(), ()> {
  let Some(window) = app.get_webview_window(&label) else {
    return Err(());
  };
  window.close().map_err(|_| ())?;
  let state = app.state::<SourceAppState>();
  state.remove_window(&label).map_err(|_| ())?;
  state.sync_windows(&app);

  Ok(())
}

pub fn ctrl_pos(pos: PhysicalPosition<i32>) -> PhysicalPosition<i32> {
  const OFFSET: (i32, i32) = (40, 0);
  PhysicalPosition::new(pos.x + OFFSET.0, pos.y + OFFSET.1)
}

pub fn _window_pos(pos: PhysicalPosition<i32>) -> PhysicalPosition<i32> {
  const OFFSET: (i32, i32) = (40, 0);
  PhysicalPosition::new(pos.x - OFFSET.0, pos.y - OFFSET.1)
}
