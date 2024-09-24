use std::sync::{atomic::Ordering, Arc};

use crate::{
  util::{AppState, ErrToString},
  view::util::ctrl_to_window_and_data,
};

use conf::Configurable;
use specta::specta;
use tauri::{command, State, WebviewWindow};
use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Settings2;
use windows::core::{Interface, HSTRING};

#[command]
#[specta]
pub fn toggle_user_agent(ctrl: WebviewWindow, state: State<'_, AppState>) -> Result<bool, String> {
  let (_, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
  let atomic = Arc::clone(&window_data.mobile_mode);
  let condition = atomic.load(Ordering::Acquire);

  set_user_agent(ctrl, state, !condition)?;

  Ok(!condition)
}

#[command]
#[specta]
// todo:モバイル用サイトのドメインを切り替える
pub fn set_user_agent(
  ctrl: WebviewWindow,
  state: State<'_, AppState>,
  value: bool,
) -> Result<(), String> {
  let read_desktop = state.agent_desktop.read().unwrap();
  let mut read_desktop_trim = read_desktop.trim();

  if read_desktop_trim.is_empty() {
    read_desktop_trim = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0";
    state.config.save().err_to_string()?;
  }

  let read_mobile = state.agent_mobile.read().unwrap();
  let mut read_mobile_trim = read_mobile.trim();
  if read_mobile_trim.is_empty() {
    read_mobile_trim = "Mozilla/5.0 (Linux; Android 13; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Mobile Safari/537.36";
    state.config.save().err_to_string()?;
  }

  let desktop = HSTRING::from(read_desktop_trim);
  let mobile = HSTRING::from(read_mobile_trim);

  let (window, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
  let atomic = Arc::clone(&window_data.mobile_mode);
  dbg!(&atomic.load(Ordering::Acquire));
  window
    .with_webview(move |webview| {
      #[cfg(windows)]
      unsafe {
        let controller = webview.controller();
        let webview = controller.CoreWebView2().unwrap();
        let settings_2: ICoreWebView2Settings2 = webview.Settings().unwrap().cast().unwrap();
        settings_2
          .SetUserAgent(&if value { mobile } else { desktop })
          .unwrap();
        webview.Reload().unwrap();
      }
    })
    .err_to_string()?;
  atomic.store(value, Ordering::Release);

  Ok(())
}

#[command]
#[specta]
pub fn get_user_agent(ctrl: WebviewWindow, state: State<'_, AppState>) -> Result<bool, String> {
  let (_, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
  let atomic = Arc::clone(&window_data.mobile_mode);

  Ok(atomic.load(Ordering::Acquire))
}
