pub mod ignore_cursor_events;
pub mod pin;
pub mod transparent;
pub mod user_agent;

use anyhow::Context;
use configu::Configurable;
use std::sync::{atomic::Ordering, Arc};
use tauri::{AppHandle, Manager, PhysicalPosition, State, WebviewUrl, WebviewWindow, WebviewWindowBuilder, WindowEvent};
use uuid::Uuid;
use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Settings2;
use windows::{
  core::{Interface, PWSTR},
  Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::{
      Shell::{DefSubclassProc, SetWindowSubclass},
      WindowsAndMessaging::{SetWindowLongPtrW, ShowWindow, GWL_EXSTYLE, SW_HIDE, SW_SHOWNORMAL, WM_ACTIVATEAPP, WS_EX_LAYERED},
    },
  },
};

use crate::util::{AppState, WindowData};

use super::util::{to_ctrl_label, window_pos, WINDOW_LABEL_PREFIX};

pub const WINDOW_MIN_INNER_SIZE: (f64, f64) = (360.0, 200.0);
pub const CTRL_SIZE: (f64, f64) = (40.0, 360.0);

pub fn view_create(app: &AppHandle, state: &State<'_, AppState>, url: WebviewUrl) -> anyhow::Result<()> {
  dbg!("create");
  let app = app.clone();
  let skip_taskbar = cfg!(not(debug_assertions));

  let title = "no title".to_string();
  let label = WINDOW_LABEL_PREFIX.to_string() + Uuid::new_v4().to_string().as_str();

  let window = WebviewWindowBuilder::new(&app, &label, url.clone())
    .decorations(false)
    .focused(true)
    .maximizable(false)
    .min_inner_size(WINDOW_MIN_INNER_SIZE.0, WINDOW_MIN_INNER_SIZE.1)
    .minimizable(true)
    .title(&title)
    .zoom_hotkeys_enabled(true)
    .build()?;

  let ctrl_window = WebviewWindowBuilder::new(&app, to_ctrl_label(&*label), WebviewUrl::App("/ctrl".into()))
    .parent(&window)?
    .inner_size(CTRL_SIZE.0, CTRL_SIZE.1)
    .decorations(false)
    .maximizable(false)
    .minimizable(false)
    .resizable(false)
    .skip_taskbar(skip_taskbar)
    .title("ctrl")
    .build()?;

  state.add_window(WindowData::new(title, label, url))?;
  state.emit_windows(&app);
  sync_windows(state)?;

  window.set_position({
    const OFFSET: (i32, i32) = (CTRL_SIZE.0 as i32, 0);
    let outer = ctrl_window.outer_position()?;
    PhysicalPosition::new(outer.x + OFFSET.0, outer.y + OFFSET.1)
  })?;

  {
    let arc = Arc::new((window, ctrl_window, app));
    let (ref window, ref _ctrl_window, ref app) = *Arc::clone(&arc);
    let window_hwnd = arc.0.hwnd()?;
    let ctrl_hwnd = arc.1.hwnd()?;

    window.on_window_event({
      let arc = Arc::clone(&arc);
      move |e| match e {
        WindowEvent::Moved(pos) => arc.1.set_position(window_pos(*pos)).unwrap(),
        WindowEvent::CloseRequested { .. } => {
          println!("close");
          let state = arc.2.state::<AppState>();
          state.remove_window(arc.0.label()).unwrap();
          state.emit_windows(&arc.2);
          sync_windows(&state).unwrap();
        }
        _ => (),
      }
    });

    if state.config.read().unwrap().agent_mobile.is_empty() {
      user_agent(app, window)
    }

    unsafe {
      SetWindowLongPtrW(window_hwnd, GWL_EXSTYLE, WS_EX_LAYERED.0 as isize);
      SetWindowLongPtrW(ctrl_hwnd, GWL_EXSTYLE, WS_EX_LAYERED.0 as isize);
      if !SetWindowSubclass(ctrl_hwnd, Some(ctrl_proc), 0, 0).as_bool() {
        println!("SetWindowSubclass failed");
      }
    }
  }

  Ok(())
}

extern "system" fn ctrl_proc(hwnd: HWND, umsg: u32, wparam: WPARAM, lparam: LPARAM, _uidsubclass: usize, _dwrefdata: usize) -> LRESULT {
  match umsg {
    // フォーカスが別のウィンドウから移ったら
    WM_ACTIVATEAPP => {
      let res = if wparam.0 > 0 {
        println!("focus");
        unsafe { ShowWindow(hwnd, SW_SHOWNORMAL) }
      } else {
        println!("unfocus");
        unsafe { ShowWindow(hwnd, SW_HIDE) }
      };
      LRESULT(res.0 as isize)
    }
    _ => unsafe { DefSubclassProc(hwnd, umsg, wparam, lparam) },
  }
}

pub fn view_restore(app: &AppHandle, state: &State<'_, AppState>) -> anyhow::Result<()> {
  let windows = state.config.read().unwrap().windows.clone();
  for window in windows {
    view_create(app, state, WebviewUrl::External(window.url.parse()?))?;
  }

  Ok(())
}

pub fn set_zoom(window: &WebviewWindow, state: State<'_, AppState>, diff: i32) -> anyhow::Result<()> {
  let window_data = state.get_window_data(window.label()).context("failure to get window data")?;
  let zoom = Arc::clone(&window_data.zoom);
  let val = zoom.load(Ordering::Acquire).saturating_add_signed(diff).clamp(20, 500);

  let scale = val as f64 / 100.0;
  window.set_zoom(scale)?;
  zoom.store(val, Ordering::Release);

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
        let state = app.state::<AppState>();
        state.config.write().unwrap().agent_desktop = pwstr.to_string().unwrap();
      }
    })
    .unwrap();
}

// TODO:pin,mobile_mode等の設定も保存する
pub(super) fn sync_windows(state: &State<'_, AppState>) -> anyhow::Result<()> {
  state.config.write().unwrap().windows = state.get_windows();
  state.config.save()?;

  Ok(())
}

pub mod command {
  use configu::Configurable;
  use specta::specta;
  use tauri::{command, AppHandle, State, WebviewWindow};

  use crate::{
    util::{AppState, ErrToString},
    view::util::{self, ctrl_to_window_and_data, to_window},
  };

  use super::set_zoom;

  #[command]
  #[specta]
  pub fn view_minimize(ctrl: WebviewWindow) -> Result<(), String> {
    util::window_minimize(&to_window(&ctrl)?).err_to_string()?;

    Ok(())
  }

  #[command]
  #[specta]
  pub fn view_close(app: AppHandle, state: State<'_, AppState>, ctrl: WebviewWindow) -> Result<(), String> {
    util::view_close(app, &ctrl).err_to_string()?;
    super::sync_windows(&state).err_to_string()?;

    Ok(())
  }

  #[command]
  #[specta]
  pub fn view_zoom(ctrl: WebviewWindow, state: State<'_, AppState>, diff: i32) -> Result<(), String> {
    set_zoom(&to_window(&ctrl)?, state, diff).err_to_string()?;

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
  pub fn get_status(ctrl: WebviewWindow, state: State<'_, AppState>) -> Result<((bool, u8), bool, bool, bool), String> {
    let (_, window_data) = ctrl_to_window_and_data(&ctrl, &state)?;

    let status = (
      (
        window_data.transparent.0.load(std::sync::atomic::Ordering::Acquire),
        window_data.transparent.1.load(std::sync::atomic::Ordering::Acquire),
      ),
      window_data.pin.load(std::sync::atomic::Ordering::Acquire),
      window_data.pointer_ignore.load(std::sync::atomic::Ordering::Acquire),
      window_data.mobile_mode.load(std::sync::atomic::Ordering::Acquire),
    );

    Ok(status)
  }

  #[command]
  #[specta]
  pub fn sync_windows(state: State<'_, AppState>) -> Result<(), String> {
    super::sync_windows(&state).err_to_string()?;
    state.config.save().err_to_string()?;

    Ok(())
  }
}
