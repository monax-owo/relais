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
  let (mut desktop, mut mobile) = {
    let config = state.config.read().unwrap();
    (config.agent_desktop.clone(), config.agent_mobile.clone())
  };

  if desktop.trim().is_empty() {
    desktop = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0".into();

    state.config.write().unwrap().agent_desktop = desktop.to_string();
    state.config.save().err_to_string()?;
  }

  if mobile.trim().is_empty() {
    mobile = "Mozilla/5.0 (Linux; Android 13; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Mobile Safari/537.36".into();

    state.config.write().unwrap().agent_mobile = mobile.to_string();
    state.config.save().err_to_string()?;
  }

  let desktop = HSTRING::from(desktop);
  let mobile = HSTRING::from(mobile);

  let (window, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;
  let atomic = Arc::clone(&window_data.mobile_mode);

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
