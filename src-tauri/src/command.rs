use anyhow::Context;
// use serde::{Deserialize, Serialize};
// use specta::Type;
use tauri::{AppHandle, WebviewWindow};

use crate::WindowData;

#[specta::specta]
pub fn export_types(_a: WindowData) {}

//
pub fn exit_0(handle: &AppHandle) -> anyhow::Result<()> {
  handle
    .remove_tray_by_id("tray")
    .context("tray is not found")?;
  handle.exit(0);
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn exit(app: AppHandle, _window: WebviewWindow) -> Result<(), String> {
  exit_0(&app).map_err(|e| e.to_string())
}
//

//
pub fn _window_focus(window: &WebviewWindow) -> anyhow::Result<()> {
  window.show()?;
  window.set_focus()?;
  // window.set_always_on_top(true)?;
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn window_focus(_app: AppHandle, window: WebviewWindow) -> Result<(), String> {
  _window_focus(&window).map_err(|e| e.to_string())?;
  Ok(())
}
//
