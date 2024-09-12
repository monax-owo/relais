use super::util;
use crate::util::{Conf, ErrToString, SourceAppState, WindowData};

use specta::specta;
use tauri::{command, AppHandle, State};

#[command]
#[specta]
pub fn exit(app: AppHandle) -> Result<(), String> {
  util::exit_0(&app).err_to_string()
}

#[command]
#[specta]
pub fn get_windows(state: State<'_, SourceAppState>) -> Vec<WindowData> {
  state.get_windows()
}

#[command]
#[specta]
pub fn get_config(state: State<'_, SourceAppState>) -> Conf {
  state.config.clone()
}
