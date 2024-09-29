use std::sync::{atomic::Ordering, Arc};

use specta::specta;
use tauri::{command, State, WebviewWindow};

use crate::{
  util::{AppState, ErrToString},
  view::util::ctrl_to_window_and_data,
};

#[command]
#[specta]
pub fn toggle_pin(ctrl: WebviewWindow, state: State<'_, AppState>) -> Result<bool, String> {
  let (_, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
  let atomic = Arc::clone(&window_data.pin);
  let condition = atomic.load(Ordering::Acquire);

  set_pin(ctrl, state, !condition)?;

  Ok(!condition)
}

#[command]
#[specta]
pub fn set_pin(ctrl: WebviewWindow, state: State<'_, AppState>, value: bool) -> Result<(), String> {
  let (window, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
  let atomic = Arc::clone(&window_data.pin);

  super::set_pin(window.hwnd().unwrap(), value).err_to_string()?;
  atomic.store(value, Ordering::Release);

  Ok(())
}

#[command]
#[specta]
pub fn get_pin(ctrl: WebviewWindow, state: State<'_, AppState>) -> Result<bool, String> {
  let (_, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
  let atomic = Arc::clone(&window_data.pin);

  Ok(atomic.load(Ordering::Acquire))
}
