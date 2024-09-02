use tauri::{AppHandle, WebviewWindow};

use super::util;
use crate::util::ErrToString;

#[tauri::command]
#[specta::specta]
pub fn exit(app: AppHandle, _window: WebviewWindow) -> Result<(), String> {
  util::exit_0(&app).err_to_string()
}
