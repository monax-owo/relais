use anyhow::Context;
use tauri::{AppHandle, WebviewWindow};

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
