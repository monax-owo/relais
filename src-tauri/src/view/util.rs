use crate::util::{AppState, ErrToString, WindowData};

use anyhow::Context;
use std::sync::Arc;
use tauri::{AppHandle, Manager, PhysicalPosition, State, WebviewWindow};

pub const WINDOW_LABEL_PREFIX: &str = "window_";
pub const CTRL_LABEL_PREFIX: &str = "ctrl_";

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

pub fn _to_ctrl(window: &WebviewWindow) -> Result<WebviewWindow, String> {
  window
    .get_webview_window(&to_ctrl_label(window.label()))
    .context("window is not found")
    .err_to_string()
}

pub fn to_window(ctrl: &WebviewWindow) -> Result<WebviewWindow, String> {
  ctrl
    .get_webview_window(&to_window_label(ctrl.label()))
    .context("ctrl is not found")
    .err_to_string()
}

pub fn ctrl_to_window_and_data(
  ctrl: &WebviewWindow,
  state: &State<'_, AppState>,
) -> Result<(WebviewWindow, WindowData), String> {
  let window = to_window(ctrl)?;
  let window_data = state.get_window_data(window.label()).err_to_string()?;
  Ok((window, window_data))
}

pub fn view_close(_app: AppHandle, ctrl: &WebviewWindow) -> Result<(), String> {
  let window = to_window(ctrl)?;
  window.close().err_to_string()?;

  Ok(())
}

pub fn _close(app: &AppHandle, arc: &Arc<(WebviewWindow, WebviewWindow)>) -> anyhow::Result<()> {
  let state = app.state::<AppState>();
  let label = arc.0.label();
  arc.1.close()?;
  arc.0.close()?;
  state.remove_window(label)?;
  state.emit_windows(app);

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
