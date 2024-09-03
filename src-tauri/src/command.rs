use tauri::{AppHandle, State};

use super::util;
use crate::util::{ErrToString, SourceAppState, WindowData};

#[tauri::command]
#[specta::specta]
pub fn exit(app: AppHandle) -> Result<(), String> {
  util::exit_0(&app).err_to_string()
}

#[tauri::command]
#[specta::specta]
pub fn get_windows(state: State<'_, SourceAppState>) -> Vec<WindowData> {
  state.get_windows()
}
