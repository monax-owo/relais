pub mod command {
  use crate::{
    util::{ErrToString, SourceAppState},
    view::util::{self, to_window},
  };

  use specta::specta;
  use std::sync::{atomic::Ordering, Arc};
  use tauri::{command, AppHandle, State, WebviewWindow};

  #[command]
  #[specta]
  pub fn view_minimize(ctrl: WebviewWindow) -> Result<(), String> {
    util::window_minimize(&to_window(&ctrl).err_to_string()?).err_to_string()?;

    Ok(())
  }

  #[command]
  #[specta]
  pub fn view_close(app: AppHandle, label: String) -> Result<(), String> {
    // TODO
    util::view_close(app, label.to_string()).err_to_string()?;

    Ok(())
  }

  #[command]
  #[specta]
  pub fn toggle_ignore_cursor_events(_ctrl: WebviewWindow, _state: State<'_, SourceAppState>) {
    todo!()
  }

  #[command]
  #[specta]
  pub fn set_ignore_cursor_events(
    ctrl: WebviewWindow,
    state: State<'_, SourceAppState>,
    value: bool,
  ) -> Result<(), String> {
    let window = to_window(&ctrl).err_to_string()?;
    let window_data = state.get_window_data(window.label()).err_to_string()?;
    let atomic = Arc::clone(&window_data.ignore);

    window.set_ignore_cursor_events(value).err_to_string()?;
    atomic.store(value, Ordering::Release);

    Ok(())
  }

  #[command]
  #[specta]
  pub fn get_ignore_cursor_events(
    ctrl: WebviewWindow,
    state: State<'_, SourceAppState>,
  ) -> Result<bool, String> {
    let window = to_window(&ctrl).err_to_string()?;
    let window_data = state.get_window_data(window.label()).err_to_string()?;
    let atomic = Arc::clone(&window_data.ignore);

    Ok(atomic.load(Ordering::Acquire))
  }

  #[command]
  #[specta]
  pub fn toggle_pin(ctrl: WebviewWindow, state: State<'_, SourceAppState>) -> Result<bool, String> {
    let window = to_window(&ctrl).err_to_string()?;
    let window_data = state.get_window_data(window.label()).err_to_string()?;
    let atomic = Arc::clone(&window_data.pin);
    let pinned = atomic.load(Ordering::Acquire);

    util::set_pin(&window, !pinned)?;
    atomic.store(!pinned, Ordering::Release);

    Ok(!pinned)
  }

  #[command]
  #[specta]
  pub fn set_pin() {}

  #[command]
  #[specta]
  pub fn get_pin() {}

  #[command]
  #[specta]
  pub fn view_zoomin(ctrl: WebviewWindow, state: State<'_, SourceAppState>) -> Result<(), String> {
    util::set_zoom(&to_window(&ctrl).err_to_string()?, state, 0.1).err_to_string()?;

    Ok(())
  }

  #[command]
  #[specta]
  pub fn view_zoomout(ctrl: WebviewWindow, state: State<'_, SourceAppState>) -> Result<(), String> {
    util::set_zoom(&to_window(&ctrl).err_to_string()?, state, -0.1).err_to_string()?;

    Ok(())
  }

  #[command]
  #[specta]
  pub fn toggle_transparent(
    ctrl: WebviewWindow,
    state: State<'_, SourceAppState>,
    alpha: u8,
  ) -> Result<bool, String> {
    let window = to_window(&ctrl).err_to_string()?;
    let window_hwnd = window.hwnd().err_to_string()?;
    let condition = state.overlay.load(Ordering::Acquire);
    if condition {
      // 不透明
      util::set_transparent(window_hwnd, 255).unwrap();
    } else {
      // 半透明
      util::set_transparent(window_hwnd, alpha).unwrap();
    };

    state.overlay.store(!condition, Ordering::Release);

    Ok(!condition)
  }

  #[command]
  #[specta]
  pub fn set_transparent(
    ctrl: WebviewWindow,
    state: State<'_, SourceAppState>,
    alpha: u8,
  ) -> Result<(), String> {
    let window = to_window(&ctrl).err_to_string()?;
    util::set_transparent(window.hwnd().err_to_string()?, alpha).err_to_string()?;
    state.overlay.store(!alpha == 255, Ordering::Release);

    Ok(())
  }

  #[command]
  #[specta]
  pub fn get_transparent(state: State<'_, SourceAppState>) -> Result<bool, String> {
    Ok(state.overlay.load(Ordering::Acquire))
  }

  #[command]
  #[specta]
  pub fn view_drag(ctrl: WebviewWindow) -> Result<(), String> {
    let window = to_window(&ctrl).err_to_string()?;
    window.start_dragging().err_to_string()?;

    Ok(())
  }
}
