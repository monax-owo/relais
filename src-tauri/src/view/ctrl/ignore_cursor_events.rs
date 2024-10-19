use anyhow::bail;
use windows::Win32::{
  Foundation::HWND,
  UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_TRANSPARENT},
};

pub fn set_ignore_cursor_events(hwnd: HWND, value: bool) -> anyhow::Result<()> {
  unsafe {
    let prev = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);

    let style = if value {
      prev | WS_EX_TRANSPARENT.0 as isize
    } else {
      prev & !(WS_EX_TRANSPARENT.0 as isize)
    };

    let res = SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style);
    if res == 0 {
      bail!("")
    }
  }

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
  pub fn toggle_ignore_cursor_events(
    ctrl: WebviewWindow,
    state: State<'_, AppState>,
  ) -> Result<bool, String> {
    let (_, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
    let atomic = Arc::clone(&window_data.pointer_ignore);
    let condition = atomic.load(Ordering::Acquire);

    set_ignore_cursor_events(ctrl, state, !condition)?;

    Ok(!condition)
  }

  #[command]
  #[specta]
  pub fn set_ignore_cursor_events(
    ctrl: WebviewWindow,
    state: State<'_, AppState>,
    value: bool,
  ) -> Result<(), String> {
    let (window, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
    let atomic = Arc::clone(&window_data.pointer_ignore);

    super::set_ignore_cursor_events(window.hwnd().unwrap(), value).err_to_string()?;
    atomic.store(value, Ordering::Release);

    Ok(())
  }

  #[command]
  #[specta]
  pub fn get_ignore_cursor_events(
    ctrl: WebviewWindow,
    state: State<'_, AppState>,
  ) -> Result<bool, String> {
    let (_, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
    let atomic = Arc::clone(&window_data.pointer_ignore);

    Ok(atomic.load(Ordering::Acquire))
  }
}
