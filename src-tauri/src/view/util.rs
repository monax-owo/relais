use crate::util::{AppState, ErrToString, WindowData};

use anyhow::{bail, Context};
use std::{
  mem::transmute,
  sync::{Arc, OnceLock},
};
use tauri::{
  AppHandle, Manager, PhysicalPosition, PhysicalSize, State, WebviewUrl, WebviewWindow,
  WebviewWindowBuilder, WindowEvent,
};
use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Settings2;
use windows::{
  core::{Interface, PWSTR},
  Win32::{
    Foundation::{COLORREF, HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
      CallWindowProcW, SetLayeredWindowAttributes, SetWindowLongPtrW, SetWindowPos, GWLP_WNDPROC,
      GWL_EXSTYLE, HWND_NOTOPMOST, HWND_TOPMOST, LWA_ALPHA, SWP_NOMOVE, SWP_NOSIZE, WM_NCACTIVATE,
      WNDPROC, WS_EX_LAYERED,
    },
  },
};

pub const CTRL_SIZE: (u32, u32) = (40, 360);
pub const WINDOW_MIN_INNER_SIZE: (f64, f64) = (400.0, 400.0);
pub const WINDOW_LABEL_PREFIX: &str = "window_";
pub const CTRL_LABEL_PREFIX: &str = "ctrl_";

pub fn view_create(
  app: &AppHandle,
  state: State<'_, AppState>,
  url: WebviewUrl,
  label: String,
) -> anyhow::Result<()> {
  let app = app.clone();
  let skip_taskbar = cfg!(not(debug_assertions));

  let title = "".to_string();
  let window = WebviewWindowBuilder::new(&app, &label, url)
    .decorations(false)
    .initialization_script(include_str!("./init.js"))
    .maximizable(false)
    .min_inner_size(WINDOW_MIN_INNER_SIZE.0, WINDOW_MIN_INNER_SIZE.1)
    .minimizable(true)
    .title(&title)
    .transparent(true)
    .zoom_hotkeys_enabled(true)
    .build()?;

  let ctrl_window = WebviewWindowBuilder::new(
    &app,
    to_ctrl_label(&*label),
    WebviewUrl::App("/ctrl".into()),
  )
  // .parent(&window)?
  .decorations(false)
  .maximizable(false)
  .minimizable(false)
  .resizable(false)
  .skip_taskbar(skip_taskbar)
  .title("ctrl")
  .transparent(true)
  .build()?;

  let window_data = WindowData::new(title, label);
  state.add_window(window_data)?;
  state.sync_windows(&app);

  window.set_position(ctrl_pos(ctrl_window.outer_position()?))?;

  {
    let arc = Arc::new((window, ctrl_window));
    let app = Arc::new(app);
    let (ref window, ref ctrl_window) = *Arc::clone(&arc);
    let window_hwnd = arc.0.hwnd()?;
    let ctrl_hwnd = arc.1.hwnd()?;

    dbg!(&window.label());
    dbg!(&ctrl_window.label());

    static PREV_PROC: OnceLock<WNDPROC> = OnceLock::new();
    PREV_PROC
      .set(unsafe {
        let res = SetWindowLongPtrW(ctrl_hwnd, GWLP_WNDPROC, ctrl_proc as isize);
        if res == 0 {
          bail!("")
        }
        transmute(res)
      })
      .unwrap();

    unsafe {
      SetWindowLongPtrW(window_hwnd, GWL_EXSTYLE, WS_EX_LAYERED.0 as isize);
    }

    window.on_window_event({
      let arc = Arc::clone(&arc);
      // let app = app.clone();
      move |e| if let WindowEvent::Moved(pos) = e { arc.1.set_position(window_pos(*pos)).unwrap() }
    });

    if state.agent.read().unwrap().is_empty() {
      user_agent(&app, window)
    }

    (|| -> anyhow::Result<()> {
      let diff_x = ctrl_window.outer_size()?.width - ctrl_window.inner_size()?.width;
      let diff_y = ctrl_window.outer_size()?.height - ctrl_window.inner_size()?.height;
      ctrl_window.set_size(PhysicalSize::new(
        diff_x + CTRL_SIZE.0,
        diff_y + CTRL_SIZE.1,
      ))?;
      Ok(())
    })()?;

    unsafe extern "system" fn ctrl_proc(
      hwnd: HWND,
      umsg: u32,
      wparam: WPARAM,
      lparam: LPARAM,
      _uidsubclass: usize,
      _dwrefdata: usize,
    ) -> LRESULT {
      match umsg {
        WM_NCACTIVATE if wparam.0 == 1 => {
          println!("WM_SETFOCUS");
          LRESULT(0)
        }
        _ => {
          // DefWindowProcW(hwnd, umsg, wparam, lparam);
          CallWindowProcW(*PREV_PROC.get().unwrap(), hwnd, umsg, wparam, lparam)
        }
      }
    }
  }

  Ok(())
}

pub fn set_transparent(hwnd: HWND, alpha: u8) -> anyhow::Result<()> {
  dbg!("0");
  unsafe {
    // bug: ignore_cursorと相性が悪い。よくわからない。
    SetLayeredWindowAttributes(hwnd, COLORREF(0), alpha, LWA_ALPHA)?;
  };
  dbg!("1");
  Ok(())
}

pub fn set_pin(window: &WebviewWindow, value: bool) -> Result<(), String> {
  // window.set_always_on_top(value).map_err(|v| v.to_string())?;
  let hwndinsertafter = if value { HWND_TOPMOST } else { HWND_NOTOPMOST };
  unsafe {
    SetWindowPos(
      window.hwnd().unwrap(),
      hwndinsertafter,
      0,
      0,
      0,
      0,
      SWP_NOMOVE | SWP_NOSIZE,
    )
    .map_err(|e| e.to_string())?
  }

  Ok(())
}

// TODO: f64の代わりにパーセントを使う
pub fn set_zoom(
  window: &WebviewWindow,
  state: State<'_, AppState>,
  diff: f64,
) -> anyhow::Result<()> {
  let window_data = state.get_window_data(window.label())?;
  let zoom = window_data.zoom.clone();
  let mut lock = zoom.lock().unwrap();

  let scale = *lock + diff;
  // TODO: 20%~500%
  if scale > 0.2 {
    window.set_zoom(scale)?;

    *lock += diff;
  } else {
    *lock = 1.0;
  }

  dbg!(*lock);

  Ok(())
}

pub fn user_agent(app: &AppHandle, window: &WebviewWindow) {
  window
    .with_webview({
      let app = app.clone();
      move |webview| unsafe {
        let controller = webview.controller();
        let webview = controller.CoreWebView2().unwrap();
        let settings_2: ICoreWebView2Settings2 = webview.Settings().unwrap().cast().unwrap();
        let mut pwstr = PWSTR::null();
        settings_2.UserAgent(&mut pwstr).unwrap();
        *app.state::<AppState>().agent.write().unwrap() = pwstr.to_string().unwrap();
      }
    })
    .unwrap();
}

pub fn to_ctrl_label<'a, T: Into<&'a str>>(label: T) -> String {
  CTRL_LABEL_PREFIX.to_string() + label.into()
}

pub fn to_window_label<'a, T: Into<&'a str>>(label: T) -> String {
  label.into().replacen(CTRL_LABEL_PREFIX, "", 1)
}

pub fn ctrl_pos(pos: PhysicalPosition<i32>) -> PhysicalPosition<i32> {
  const OFFSET: (i32, i32) = (40, 0);
  PhysicalPosition::new(pos.x + OFFSET.0, pos.y + OFFSET.1)
}

pub fn window_pos(pos: PhysicalPosition<i32>) -> PhysicalPosition<i32> {
  const OFFSET: (i32, i32) = (40, 0);
  PhysicalPosition::new(pos.x - OFFSET.0, pos.y - OFFSET.1)
}

pub fn _to_ctrl(window: &WebviewWindow) -> Result<WebviewWindow, String> {
  window
    .get_webview_window(&to_ctrl_label(window.label()))
    .context("window is not found")
    .err_to_string()
}

pub fn to_window(ctrl: &WebviewWindow) -> Result<WebviewWindow, String> {
  ctrl
    .get_webview_window(&to_window_label(ctrl.label()))
    .context("ctrl is not found")
    .err_to_string()
}

pub fn ctrl_to_window_and_data(
  ctrl: &WebviewWindow,
  state: &State<'_, AppState>,
) -> Result<(WebviewWindow, WindowData), String> {
  let window = to_window(ctrl)?;
  let window_data = state.get_window_data(window.label()).err_to_string()?;
  Ok((window, window_data))
}

pub fn view_close(app: AppHandle, label: String) -> anyhow::Result<()> {
  let window = app
    .get_webview_window(&label)
    .context("failed to get window")?;
  window.close()?;
  let state = app.state::<AppState>();
  state.remove_window(&label)?;
  state.sync_windows(&app);

  Ok(())
}

pub fn _close(app: &AppHandle, arc: &Arc<(WebviewWindow, WebviewWindow)>) -> anyhow::Result<()> {
  let state = app.state::<AppState>();
  let label = arc.0.label();
  arc.1.close()?;
  arc.0.close()?;
  state.remove_window(label)?;
  state.sync_windows(app);

  Ok(())
}

pub fn window_focus(window: &WebviewWindow) -> anyhow::Result<()> {
  window.show()?;
  window.set_focus()?;
  // window.set_always_on_top(true)?;

  Ok(())
}

pub fn window_hide(window: &WebviewWindow) -> anyhow::Result<()> {
  window.hide()?;
  // window.set_always_on_top(false)?;

  Ok(())
}

pub fn window_minimize(window: &WebviewWindow) -> anyhow::Result<()> {
  window.minimize()?;

  Ok(())
}
