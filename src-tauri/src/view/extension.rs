pub mod util {}

pub mod command {
  use crate::{
    util::{ErrToString, AppState},
    view::util::to_window,
  };
  use anyhow::Context;
  use specta::specta;
  use tauri::{command, AppHandle, State, WebviewWindow};
  use tauri_plugin_dialog::DialogExt;
  use webview2_com::Microsoft::Web::WebView2::Win32::{ICoreWebView2Profile7, ICoreWebView2_13};
  use windows::core::Interface;

  #[command]
  #[specta]
  pub fn test(
    app: AppHandle,
    ctrl: WebviewWindow,
    _state: State<'_, AppState>,
  ) -> Result<(), String> {
    let window = to_window(&ctrl).err_to_string()?;
    window
      .with_webview(move |webview| {
        #[cfg(target_os = "windows")]
        (|| -> anyhow::Result<()> {
          unsafe {
            let controller = webview.controller();
            let webview = controller.CoreWebView2().unwrap();
            let _webview_profile_7 = webview
              .cast::<ICoreWebView2_13>()?
              .Profile()?
              .cast::<ICoreWebView2Profile7>()?;

            let _path = app
              .dialog()
              .file()
              .blocking_pick_folder()
              .context("failed to get dir path")
              .unwrap();
            // let res = webview_profile_7.AddBrowserExtension(path.to_str().unwrap().into(), handler);

            // Ok(())
            todo!()
          }
        })()
        .unwrap();
      })
      .err_to_string()?;

    println!("test command");
    Ok(())
  }
}
