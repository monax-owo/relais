use std::sync::{atomic::Ordering, Arc};

use crate::{
  util::{ErrToString, SourceAppState},
  view::util::to_window,
};

use specta::specta;
use tauri::{command, State, WebviewWindow};
use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Settings2;
use windows::core::{w, Interface, PCWSTR};

#[command]
#[specta]
pub fn toggle_user_agent(
  ctrl: WebviewWindow,
  state: State<'_, SourceAppState>,
) -> Result<bool, String> {
  let window = to_window(&ctrl).err_to_string()?;
  let window_data = state.get_window_data(window.label()).err_to_string()?;
  let atomic = Arc::clone(&window_data.mobile_mode);
  let condition = atomic.load(Ordering::Acquire);

  set_user_agent(ctrl, state, !condition)?;

  Ok(!condition)
}

#[command]
#[specta]
pub fn set_user_agent(
  ctrl: WebviewWindow,
  state: State<'_, SourceAppState>,
  value: bool,
) -> Result<(), String> {
  const MOBILE: PCWSTR = w!("Mozilla/5.0 (Linux; Android 13; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Mobile Safari/537.36");
  const DESKTOP: PCWSTR = w!("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0");

  let window = to_window(&ctrl).err_to_string()?;
  let window_data = state.get_window_data(window.label()).err_to_string()?;
  let atomic = Arc::clone(&window_data.mobile_mode);

  window
    .with_webview(move |webview| {
      #[cfg(windows)]
      unsafe {
        let controller = webview.controller();
        let webview = controller.CoreWebView2().unwrap();
        let settings_2: ICoreWebView2Settings2 = webview.Settings().unwrap().cast().unwrap();
        settings_2
          .SetUserAgent(if value { DESKTOP } else { MOBILE })
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
pub fn get_user_agent(
  ctrl: WebviewWindow,
  state: State<'_, SourceAppState>,
) -> Result<bool, String> {
  let window = to_window(&ctrl).err_to_string()?;
  let window_data = state.get_window_data(window.label()).err_to_string()?;
  let atomic = Arc::clone(&window_data.mobile_mode);

  Ok(atomic.load(Ordering::Acquire))
}
