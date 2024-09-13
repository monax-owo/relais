use super::util;
use crate::util::{AppState, ErrToString};

use specta::specta;
use tauri::{command, AppHandle, State, WebviewUrl, WebviewWindow};
use uuid::Uuid;

#[command]
#[specta]
pub async fn view_create(
  app: AppHandle,
  state: State<'_, AppState>,
  url: String,
  label: Option<String>,
) -> Result<(), String> {
  let url = if !url.starts_with("http") {
    format!("https://{}", url)
  } else {
    url
  };
  let parse_url = url::Url::parse(&url).err_to_string()?;
  let label =
    label.unwrap_or(util::WINDOW_LABEL_PREFIX.to_string() + Uuid::new_v4().to_string().as_str());

  util::view_create(&app, state, WebviewUrl::External(parse_url), label).unwrap();

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
