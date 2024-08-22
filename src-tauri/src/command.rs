// use serde::{Deserialize, Serialize};
// use specta::Type;
use tauri::{AppHandle, Manager, State, Window, WindowBuilder, WindowEvent};

use crate::{AppState, WindowData};

//
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
//

//
pub fn _window_focus(window: &Window) -> anyhow::Result<()> {
  window.show()?;
  window.set_focus()?;
  // window.set_always_on_top(true)?;
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn window_focus(_app: AppHandle, window: Window) -> Result<(), String> {
  _window_focus(&window).map_err(|e| e.to_string())?;
  Ok(())
}
//

//
pub fn _window_hide(window: &Window) -> anyhow::Result<()> {
  window.hide()?;
  // window.set_always_on_top(false)?;
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn window_hide(_app: AppHandle, window: Window) -> Result<(), String> {
  _window_hide(&window).map_err(|e| e.to_string())?;
  Ok(())
}
//

//
#[tauri::command]
#[specta::specta]
pub async fn open_window(
  app: AppHandle,
  _window: Window,
  state: State<'_, AppState>,
  label: String,
  url: String,
  title: Option<String>,
) -> Result<(), ()> {
  // もうちょっといい書き方あるだろ
  let url = if url.starts_with("http") {
    url
  } else {
    format!("https://{}", url)
  };
  let parse_url = url::Url::parse(&url).map_err(|_| ())?;
  dbg!(&parse_url);

  // create window
  let title = title.unwrap_or_default();
  let window = WindowBuilder::new(&app, &label, tauri::WindowUrl::External(parse_url))
    .transparent(true)
    .title(&title)
    .build()
    .unwrap();

  let window_data = WindowData {
    title,
    label: label.clone(),
    zoom: 1.0,
  };
  state.add_window(window_data).map_err(|_| ())?;

  // if window closing, when remove if from window list
  window.on_window_event(move |e| if let WindowEvent::CloseRequested { .. } = *e {
    _close_window(app.clone(), label.clone()).unwrap();
  });

  Ok(())
}
//

//
fn _close_window(app: AppHandle, label: String) -> Result<(), ()> {
  let Some(window) = app.get_window(&label) else {
    return Err(());
  };
  window.close().map_err(|_| ())?;
  let state = app.state::<AppState>();
  state.remove_window(&label).map_err(|_| ())?;
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn close_window(
  app: AppHandle,
  _window: Window,
  // state: State<'_, AppState>,
  label: String,
) -> Result<(), ()> {
  _close_window(app, label.to_string()).map_err(|_| ())?;
  Ok(())
}
//

//
#[tauri::command]
#[specta::specta]
pub fn get_windows(
  _app: AppHandle,
  _window: Window,
  state: State<'_, AppState>,
) -> Vec<WindowData> {
  state.windows.lock().unwrap().to_vec()
}
