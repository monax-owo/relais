use std::sync::{atomic::Ordering, Arc};

use specta::specta;
use tauri::{command, State, WebviewWindow};

use crate::{
  util::{ErrToString, SourceAppState},
  view::util::to_window,
};

#[command]
#[specta]
pub fn toggle_ignore_cursor_events(
  ctrl: WebviewWindow,
  state: State<'_, SourceAppState>,
) -> Result<bool, String> {
  let window = to_window(&ctrl).err_to_string()?;
  let window_data = state.get_window_data(window.label()).err_to_string()?;
  let atomic = Arc::clone(&window_data.ignore);
  let condition = atomic.load(Ordering::Acquire);

  set_ignore_cursor_events(ctrl, state, !condition)?;

  Ok(!condition)
}

#[command]
#[specta]
pub fn set_ignore_cursor_events(
  ctrl: WebviewWindow,
  state: State<'_, SourceAppState>,
  value: bool,
) -> Result<(), String> {
  let window = to_window(&ctrl).err_to_string()?;
  let window_data = state.get_window_data(window.label()).err_to_string()?;
  let atomic = Arc::clone(&window_data.ignore);

  window.set_ignore_cursor_events(value).err_to_string()?;
  atomic.store(value, Ordering::Release);

  Ok(())
}

#[command]
#[specta]
pub fn get_ignore_cursor_events(
  ctrl: WebviewWindow,
  state: State<'_, SourceAppState>,
) -> Result<bool, String> {
  let window = to_window(&ctrl).err_to_string()?;
  let window_data = state.get_window_data(window.label()).err_to_string()?;
  let atomic = Arc::clone(&window_data.ignore);

  Ok(atomic.load(Ordering::Acquire))
}
