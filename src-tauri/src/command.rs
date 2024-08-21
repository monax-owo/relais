// use serde::{Deserialize, Serialize};
// use specta::Type;
use tauri::{AppHandle, Window, WindowBuilder};
use url::Url;

pub fn exit_0(handle: &AppHandle) -> anyhow::Result<()> {
  handle.tray_handle().destroy()?;
  handle.exit(0);
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn exit(app: AppHandle, _window: Window) -> Result<(), String> {
  exit_0(&app).map_err(|e| e.to_string())
}

pub fn window_focus(window: &Window) -> anyhow::Result<()> {
  window.show()?;
  window.set_focus()?;
  // window.set_always_on_top(true)?;
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn main_window_focus(_app: AppHandle, window: Window) -> Result<(), String> {
  window_focus(&window).map_err(|e| e.to_string())?;
  Ok(())
}

pub fn window_hide(window: &Window) -> anyhow::Result<()> {
  window.hide()?;
  // window.set_always_on_top(false)?;
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn main_window_hide(_app: AppHandle, window: Window) -> Result<(), String> {
  window_hide(&window).map_err(|e| e.to_string())?;
  Ok(())
}

pub async fn open_with_window(handle: &AppHandle, label: &str, url: Url) -> anyhow::Result<Window> {
  let window = WindowBuilder::new(handle, label, tauri::WindowUrl::External(url))
    .transparent(true)
    .build()
    .unwrap();
  Ok(window)
}

#[tauri::command]
#[specta::specta]
pub async fn open_url(app: AppHandle, label: &str, url: String) -> Result<String, String> {
  let url = if url.starts_with("http") {
    url
  } else {
    format!("https://{}", url)
  };
  let parse_url = url::Url::parse(&url).map_err(|e| e.to_string())?;
  dbg!(&parse_url);
  let window = open_with_window(&app, label, parse_url).await.unwrap();
  println!("s");
  Ok(window.label().to_string())
}
