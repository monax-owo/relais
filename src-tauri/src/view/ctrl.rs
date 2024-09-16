pub mod ignore_cursor_events;
pub mod pin;
pub mod transparent;
pub mod user_agent;

pub mod command {
  use specta::specta;
  use tauri::{command, AppHandle, State, WebviewWindow};

  use crate::{
    util::{AppState, ErrToString},
    view::util::{self, ctrl_to_window_and_data, to_window},
  };

  #[command]
  #[specta]
  pub fn view_minimize(ctrl: WebviewWindow) -> Result<(), String> {
    util::window_minimize(&to_window(&ctrl)?).err_to_string()?;

    Ok(())
  }

  #[command]
  #[specta]
  pub fn view_close(app: AppHandle, ctrl: WebviewWindow) -> Result<(), String> {
    util::view_close(app, &ctrl).err_to_string()?;

    Ok(())
  }

  #[command]
  #[specta]
  pub fn view_zoomin(ctrl: WebviewWindow, state: State<'_, AppState>) -> Result<(), String> {
    util::set_zoom(&to_window(&ctrl)?, state, 0.1).err_to_string()?;

    Ok(())
  }

  #[command]
  #[specta]
  pub fn view_zoomout(ctrl: WebviewWindow, state: State<'_, AppState>) -> Result<(), String> {
    util::set_zoom(&to_window(&ctrl)?, state, -0.1).err_to_string()?;

    Ok(())
  }

  #[command]
  #[specta]
  pub fn view_drag(ctrl: WebviewWindow) -> Result<(), String> {
    let window = to_window(&ctrl)?;
    window.start_dragging().err_to_string()?;

    Ok(())
  }

  #[command]
  #[specta]
  pub fn get_status(
    ctrl: WebviewWindow,
    state: State<'_, AppState>,
  ) -> Result<((bool, u8), bool, bool, bool), String> {
    let (_, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;

    let status = (
      (
        window_data
          .transparent
          .0
          .load(std::sync::atomic::Ordering::Acquire),
        window_data
          .transparent
          .1
          .load(std::sync::atomic::Ordering::Acquire),
      ),
      window_data.pin.load(std::sync::atomic::Ordering::Acquire),
      window_data
        .pointer_ignore
        .load(std::sync::atomic::Ordering::Acquire),
      window_data
        .mobile_mode
        .load(std::sync::atomic::Ordering::Acquire),
    );

    Ok(status)
  }
}
