use crate::{
  util::{ErrToString, SourceAppState},
  view::util::to_window,
};

use specta::specta;
use tauri::{command, State, WebviewWindow};
use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Settings2;
use windows::core::{w, Interface};

#[command]
#[specta]
pub fn toggle_user_agent(
  ctrl: WebviewWindow,
  _state: State<'_, SourceAppState>,
) -> Result<bool, String> {
  let window = to_window(&ctrl).err_to_string()?;
  window
    .with_webview(|webview| {
      #[cfg(windows)]
      unsafe {
        let controller = webview.controller();
        let webview = controller.CoreWebView2().unwrap();
        let settings_2: ICoreWebView2Settings2 = webview.Settings().unwrap().cast().unwrap();
        settings_2.SetUserAgent(w!("Mozilla/5.0 (Linux; Android 13; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Mobile Safari/537.36")).unwrap();
        webview.Reload().unwrap();
      }
    })
    .err_to_string()?;
  Ok(true)
}

#[command]
#[specta]
pub fn set_user_agent(
  ctrl: WebviewWindow,
  _state: State<'_, SourceAppState>,
  value: bool,
) -> Result<(), String> {
  let window = to_window(&ctrl).err_to_string()?;
  window
    .with_webview(|webview| {
      #[cfg(windows)]
      unsafe {
        let controller = webview.controller();
        let webview = controller.CoreWebView2().unwrap();
        let settings_2: ICoreWebView2Settings2 = webview.Settings().unwrap().cast().unwrap();
        settings_2.SetUserAgent(w!("Mozilla/5.0 (Linux; Android 13; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Mobile Safari/537.36")).unwrap();
        webview.Reload().unwrap();
      }
    })
    .err_to_string()?;
  Ok(())
}

#[command]
#[specta]
pub fn get_user_agent(_ctrl: WebviewWindow, _state: State<'_, SourceAppState>) {
  // todo!()
}
