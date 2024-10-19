use windows::Win32::{
  Foundation::{COLORREF, HWND},
  UI::WindowsAndMessaging::{SetLayeredWindowAttributes, LWA_ALPHA},
};

pub fn set_transparent(hwnd: HWND, alpha: u8) -> anyhow::Result<()> {
  unsafe {
    SetLayeredWindowAttributes(hwnd, COLORREF(0), alpha, LWA_ALPHA)?;
  };

  Ok(())
}

pub mod command {
  use std::sync::{atomic::Ordering, Arc};

  use specta::specta;
  use tauri::{command, State, WebviewWindow};

  use crate::{
    util::{AppState, ErrToString},
    view::util::ctrl_to_window_and_data,
  };

  #[command]
  #[specta]
  pub fn toggle_transparent(ctrl: WebviewWindow, state: State<'_, AppState>, alpha: u8) -> Result<bool, String> {
    let (_, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
    let atomic = Arc::clone(&window_data.transparent);
    let data = (atomic.0.load(Ordering::Acquire), atomic.1.load(Ordering::Acquire));
    let condition = data.0;

    set_transparent(ctrl, state, if condition { 255 } else { alpha })?;

    Ok(!condition)
  }

  #[command]
  #[specta]
  pub fn set_transparent(ctrl: WebviewWindow, state: State<'_, AppState>, alpha: u8) -> Result<(), String> {
    let (window, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
    let atomic = Arc::clone(&window_data.transparent);

    super::set_transparent(window.hwnd().unwrap(), alpha).err_to_string()?;

    atomic.0.store(alpha != 255, Ordering::Release);

    Ok(())
  }

  #[command]
  #[specta]
  pub fn get_transparent(ctrl: WebviewWindow, state: State<'_, AppState>) -> Result<(bool, u8), String> {
    let (_, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
    let atomic = Arc::clone(&window_data.transparent);
    let data = (atomic.0.load(Ordering::Acquire), atomic.1.load(Ordering::Acquire));

    Ok(data)
  }
}
