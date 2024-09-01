use std::sync::{atomic::Ordering, Arc};
use tauri::{AppHandle, Manager, State, WebviewUrl, WebviewWindow};
use uuid::Uuid;

use crate::SourceAppState;

use super::util;

#[tauri::command]
#[specta::specta]
pub async fn view_create(
  app: AppHandle,
  state: State<'_, SourceAppState>,
  url: String,
  title: Option<String>,
  label: Option<String>,
) -> Result<(), ()> {
  let url = if !url.starts_with("http") {
    format!("https://{}", url)
  } else {
    url
  };
  let parse_url = url::Url::parse(&url).map_err(|_| ())?;
  let title = title.unwrap_or_default();
  let label =
    label.unwrap_or(util::WINDOW_LABEL_PREFIX.to_string() + Uuid::new_v4().to_string().as_str());

  util::view_create(&app, state, WebviewUrl::External(parse_url), title, label).unwrap();

  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn get_transparent(state: State<'_, SourceAppState>) -> Result<bool, String> {
  Ok(state.overlay.load(Ordering::Acquire))
}

#[tauri::command]
#[specta::specta]
pub fn toggle_pin(
  app: AppHandle,
  window: WebviewWindow,
  state: State<'_, SourceAppState>,
) -> Result<bool, String> {
  let Some(window_data) = state.get_window_data(&util::to_window_label(window.label())) else {
    return Err("failed to get window data".to_string());
  };
  let atomic = Arc::clone(&window_data.pin);
  let pinned = atomic.load(Ordering::Acquire);
  dbg!(pinned);
  util::set_pin(
    &app
      .get_webview_window(&util::to_window_label(window.label()))
      .unwrap(),
    !pinned,
  )?;
  atomic.store(!pinned, Ordering::Release);
  Ok(!pinned)
}

#[tauri::command]
#[specta::specta]
pub fn view_close(app: AppHandle, label: String) -> Result<(), ()> {
  util::view_close(app, label.to_string()).map_err(|_| ())?;
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn window_focus(_app: AppHandle, window: WebviewWindow) -> Result<(), String> {
  util::window_focus(&window).map_err(|e| e.to_string())?;
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn window_hide(window: WebviewWindow) -> Result<(), String> {
  util::window_hide(&window).map_err(|e| e.to_string())?;
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn view_minimize(window: WebviewWindow) -> Result<(), String> {
  util::view_minimize(&window).map_err(|e| e.to_string())?;
  Ok(())
}
