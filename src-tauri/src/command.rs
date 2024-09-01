use tauri::{AppHandle, WebviewWindow};

use super::util;

#[tauri::command]
#[specta::specta]
pub fn exit(app: AppHandle, _window: WebviewWindow) -> Result<(), String> {
  util::exit_0(&app).map_err(|e| e.to_string())
}
