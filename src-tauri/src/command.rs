use super::util;
use crate::util::{AppState, Conf, ErrToString, SAppState, SWindowDatas};

use specta::specta;
use tauri::{command, AppHandle, State};

#[command]
#[specta]
pub fn exit(app: AppHandle) -> Result<(), String> {
  util::exit_0(&app).err_to_string()
}

#[command]
#[specta]
pub fn get_windows(state: State<'_, AppState>) -> SWindowDatas {
  state.get_windows()
}

#[command]
#[specta]
pub fn get_config(state: State<'_, AppState>) -> Conf {
  state.config.clone()
}

#[command]
#[specta]
pub fn get_state(state: State<'_, AppState>) -> Result<SAppState, String> {
  SAppState::try_from(state.inner()).err_to_string()
}
