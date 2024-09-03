pub mod command {
  use std::{
    sync::{atomic::Ordering, Arc},
    u8,
  };

  use crate::{util::ErrToString, view::util, SourceAppState};
  use tauri::{AppHandle, Manager, State, WebviewWindow};

  #[tauri::command]
  #[specta::specta]
  pub fn view_minimize(ctrl: WebviewWindow) -> Result<(), String> {
    util::window_minimize(&util::to_window(&ctrl).err_to_string()?).err_to_string()?;

    Ok(())
  }
  #[tauri::command]
  #[specta::specta]
  pub fn view_close(app: AppHandle, label: String) -> Result<(), String> {
    // TODO
    util::view_close(app, label.to_string()).err_to_string()?;

    Ok(())
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
  pub fn view_zoomin(app: AppHandle, ctrl: WebviewWindow) -> Result<(), String> {
    util::set_zoom(&app, &util::to_window(&ctrl).err_to_string()?, 0.1).err_to_string()?;

    Ok(())
  }
  #[tauri::command]
  #[specta::specta]
  pub fn view_zoomout(app: AppHandle, ctrl: WebviewWindow) -> Result<(), String> {
    util::set_zoom(&app, &util::to_window(&ctrl).err_to_string()?, -0.1).err_to_string()?;

    Ok(())
  }
  #[tauri::command]
  #[specta::specta]
  pub fn toggle_transparent() {
    todo!()
  }
  #[tauri::command]
  #[specta::specta]
  pub fn set_transparent(
    ctrl: WebviewWindow,
    state: State<'_, SourceAppState>,
    alpha: u8,
  ) -> Result<(), String> {
    let window = util::to_window(&ctrl).err_to_string()?;
    util::set_transparent(window.hwnd().err_to_string()?, alpha).err_to_string()?;
    let transparent = !alpha == 255;
    dbg!(transparent);
    state.overlay.store(transparent, Ordering::Release);
    Ok(())
  }
  #[tauri::command]
  #[specta::specta]
  pub fn get_transparent(state: State<'_, SourceAppState>) -> Result<bool, String> {
    Ok(state.overlay.load(Ordering::Acquire))
  }
  #[tauri::command]
  #[specta::specta]
  pub fn set_pointer_ignore() {
    todo!()
  }
  #[tauri::command]
  #[specta::specta]
  pub fn view_drag(ctrl: WebviewWindow) -> Result<(), String> {
    let window = util::to_window(&ctrl).err_to_string()?;
    window.start_dragging().err_to_string()?;

    Ok(())
  }
}
