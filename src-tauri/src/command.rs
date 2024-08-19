use tauri::{AppHandle, Window};

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
  window.set_always_on_top(true)?;
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
  window.set_always_on_top(false)?;
  Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn main_window_hide(_app: AppHandle, window: Window) -> Result<(), String> {
  window_hide(&window).map_err(|e| e.to_string())?;
  Ok(())
}
