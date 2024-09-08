use std::sync::atomic::Ordering;

use specta::specta;
use tauri::{command, State, WebviewWindow};

use crate::{
  util::{ErrToString, SourceAppState},
  view::util::{self, to_window},
};

#[command]
#[specta]
pub fn toggle_transparent(
  ctrl: WebviewWindow,
  state: State<'_, SourceAppState>,
  alpha: u8,
) -> Result<bool, String> {
  let condition = state.overlay.load(Ordering::Acquire);

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
  state: State<'_, SourceAppState>,
  alpha: u8,
) -> Result<(), String> {
  let window = to_window(&ctrl).err_to_string()?;
  util::set_transparent(window.hwnd().err_to_string()?, alpha).err_to_string()?;
  state.overlay.store(alpha != 255, Ordering::Release);

  Ok(())
}

#[command]
#[specta]
pub fn get_transparent(state: State<'_, SourceAppState>) -> Result<bool, String> {
  Ok(state.overlay.load(Ordering::Acquire))
}
