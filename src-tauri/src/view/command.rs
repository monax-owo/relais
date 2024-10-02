use super::{ctrl, util};
use crate::util::{AppState, ErrToString};

use specta::specta;
use tauri::{command, AppHandle, State, WebviewUrl, WebviewWindow};

#[command]
#[specta]
pub async fn view_create(
  app: AppHandle,
  state: State<'_, AppState>,
  url: String,
) -> Result<(), String> {
  let url = if !url.starts_with("http") {
    String::from("https://") + &url
  } else {
    url
  };
  let parse_url = url::Url::parse(&url).err_to_string()?;
  ctrl::view_create(&app, &state, WebviewUrl::External(parse_url)).unwrap();

  Ok(())
}

#[command]
#[specta]
pub fn window_focus(_app: AppHandle, window: WebviewWindow) -> Result<(), String> {
  util::window_focus(&window).err_to_string()?;

  Ok(())
}

#[command]
#[specta]
pub fn window_hide(window: WebviewWindow) -> Result<(), String> {
  util::window_hide(&window).err_to_string()?;

  Ok(())
}
