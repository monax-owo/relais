use tauri::{AppHandle, State, WebviewUrl, WebviewWindow};
use uuid::Uuid;

use crate::util::{ErrToString, SourceAppState};

use super::util;

#[tauri::command]
#[specta::specta]
pub async fn view_create(
  app: AppHandle,
  state: State<'_, SourceAppState>,
  url: String,
  title: Option<String>,
  label: Option<String>,
) -> Result<(), String> {
  let url = if !url.starts_with("http") {
    format!("https://{}", url)
  } else {
    url
  };
  let parse_url = url::Url::parse(&url).err_to_string()?;
  let title = title.unwrap_or_default();
  let label =
    label.unwrap_or(util::WINDOW_LABEL_PREFIX.to_string() + Uuid::new_v4().to_string().as_str());

  util::view_create(&app, state, WebviewUrl::External(parse_url), title, label).unwrap();

  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn window_focus(_app: AppHandle, window: WebviewWindow) -> Result<(), String> {
  util::window_focus(&window).err_to_string()?;

  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn window_hide(window: WebviewWindow) -> Result<(), String> {
  util::window_hide(&window).err_to_string()?;

  Ok(())
}
