use std::sync::{atomic::Ordering, Arc};

use specta::specta;
use tauri::{command, State, WebviewWindow};

use crate::{
  util::{AppState, ErrToString},
  view::util::{self, ctrl_to_window_and_data},
};

#[command]
#[specta]
pub fn toggle_transparent(
  ctrl: WebviewWindow,
  state: State<'_, AppState>,
  alpha: u8,
) -> Result<bool, String> {
  let (_, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
  let atomic = Arc::clone(&window_data.overlay);
  let condition = atomic.load(Ordering::Acquire);

  if condition {
    // 不透明
    set_transparent(ctrl, state, 255)?;
  } else {
    // 半透明
    set_transparent(ctrl, state, alpha)?;
  };
  dbg!(condition);
  Ok(!condition)
}

#[command]
#[specta]
pub fn set_transparent(
  ctrl: WebviewWindow,
  state: State<'_, AppState>,
  alpha: u8,
) -> Result<(), String> {
  let (window, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
  let atomic = Arc::clone(&window_data.overlay);

  util::set_transparent(window.hwnd().err_to_string()?, alpha).err_to_string()?;
  atomic.store(alpha != 255, Ordering::Release);

  Ok(())
}

#[command]
#[specta]
pub fn get_transparent(ctrl: WebviewWindow, state: State<'_, AppState>) -> Result<bool, String> {
  let (_, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
  let atomic = Arc::clone(&window_data.overlay);

  Ok(atomic.load(Ordering::Acquire))
}
