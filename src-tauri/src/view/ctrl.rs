pub mod ignore_cursor_events;
pub mod pin;
pub mod transparent;
pub mod user_agent;

pub mod command {
  use specta::specta;
  use tauri::{command, AppHandle, State, WebviewWindow};

  use crate::{
    util::{ErrToString, SourceAppState},
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
  pub fn view_drag(ctrl: WebviewWindow) -> Result<(), String> {
    let window = to_window(&ctrl).err_to_string()?;
    window.start_dragging().err_to_string()?;

    Ok(())
  }
}
