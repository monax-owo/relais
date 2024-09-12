pub mod ignore_cursor_events;
pub mod pin;
pub mod transparent;
pub mod user_agent;

pub mod command {
  use specta::specta;
  use tauri::{command, AppHandle, State, WebviewWindow};

  use crate::{
    util::{ErrToString, AppState},
    view::util::{self, to_window},
  };

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
  pub fn view_zoomin(ctrl: WebviewWindow, state: State<'_, AppState>) -> Result<(), String> {
    util::set_zoom(&to_window(&ctrl).err_to_string()?, state, 0.1).err_to_string()?;

    Ok(())
  }

  #[command]
  #[specta]
  pub fn view_zoomout(ctrl: WebviewWindow, state: State<'_, AppState>) -> Result<(), String> {
    util::set_zoom(&to_window(&ctrl).err_to_string()?, state, -0.1).err_to_string()?;

    Ok(())
  }

  #[command]
  #[specta]
  pub fn view_drag(ctrl: WebviewWindow) -> Result<(), String> {
    let window = to_window(&ctrl).err_to_string()?;
    window.start_dragging().err_to_string()?;

    Ok(())
  }

  #[command]
  #[specta]
  pub fn get_status(
    ctrl: WebviewWindow,
    state: State<'_, AppState>,
  ) -> Result<(bool, bool, bool, bool), String> {
    let window = to_window(&ctrl).err_to_string()?;
    let window_data = state.get_window_data(window.label()).err_to_string()?;

    let status = (
      state.overlay.load(std::sync::atomic::Ordering::Acquire),
      window_data.pin.load(std::sync::atomic::Ordering::Acquire),
      window_data
        .ignore
        .load(std::sync::atomic::Ordering::Acquire),
      window_data
        .mobile_mode
        .load(std::sync::atomic::Ordering::Acquire),
    );

    Ok(status)
  }
}
