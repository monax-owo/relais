// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{atomic::AtomicBool, Arc, Mutex};

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{
  generate_context, generate_handler, App, AppHandle, Builder, CustomMenuItem, Manager, SystemTray,
  SystemTrayEvent, SystemTrayMenu, WindowEvent,
};

mod command;
mod window;

use command::*;
use window::*;

// #[cfg(target_os = "windows")]
// use {
//   std::os::raw::c_void,
//   windows::Win32::{
//     Foundation::{BOOL, HWND},
//     Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_TRANSITIONS_FORCEDISABLED},
//   },
// };

// TODO: アプリ全体かウィンドウごとに半透明にするか
#[derive(Debug, Type)]
pub struct AppState {
  windows: Mutex<Vec<WindowData>>,
  pub overlay: AtomicBool,
}

// #[derive(Debug, Type)]
// pub struct SerializeAppState {
//   windows: Vec<SerializeWindowData>,
//   pub overlay: AtomicBool,
// }

#[derive(Debug, Clone, Type)]
pub struct WindowData {
  title: String,
  label: String,
  zoom: Arc<Mutex<f64>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct SerializeWindowData {
  title: String,
  label: String,
  zoom: f64,
}

impl AppState {
  pub fn add_window(&self, window: WindowData) -> anyhow::Result<()> {
    let mut lock = self.windows.lock().unwrap();
    lock.push(window);
    dbg!(&lock);
    Ok(())
  }

  // labelに一致する値がなかったらErrにする
  pub fn remove_window(&self, label: &str) -> anyhow::Result<()> {
    let mut lock = self.windows.lock().unwrap();
    lock.retain(|v| v.label.as_str() != label);
    dbg!(&lock);
    Ok(())
  }

  pub fn sync_windows(&self, handle: &AppHandle) {
    let windows = self.windows.lock().unwrap();
    let vec = windows.clone().into_iter().map(|v| v.into()).collect();
    handle
      .emit_all::<Vec<SerializeWindowData>>("update_windows", vec)
      .unwrap();
  }

  pub fn get_window_data(&self, label: &str) -> Option<SerializeWindowData> {
    let lock = self.windows.lock().unwrap();
    lock
      .iter()
      .find(|v| v.label.as_str() == label)
      .map(|v| SerializeWindowData::from(v.clone()))
  }
}

impl From<WindowData> for SerializeWindowData {
  fn from(v: WindowData) -> Self {
    Self {
      title: v.title,
      label: v.label,
      zoom: *v.zoom.lock().unwrap(),
    }
  }
}

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  tauri_specta::ts::export(
    specta::collect_types![
      export_types,
      exit,
      window_focus,
      window_hide,
      open_window,
      close_window,
      get_transparent
    ],
    "../src/lib/generated/specta/bindings.ts",
  )
  .expect("failed to generate types");

  let builder = Builder::default();
  let state = AppState {
    windows: Mutex::new(vec![]),
    overlay: AtomicBool::new(false),
  };
  builder
    .setup(move |app: &mut App| {
      let handle = app.handle();
      let main_window = Arc::new(app.get_window("main").expect("Failed to get main window"));
      //
      #[cfg(not(debug_assertions))]
      {
        _window_focus(&main_window)?;
      }
      //

      //
      main_window.on_window_event({
        let main_window = Arc::clone(&main_window);
        move |e| {
          if let WindowEvent::CloseRequested { api, .. } = e {
            api.prevent_close();
            main_window.hide().unwrap();
          }
        }
      });
      // main_window.listen("", |_| {});
      //

      //
      const MENU_SHOW: &str = "show";
      const MENU_TOGGLE: &str = "toggle";
      const MENU_QUIT: &str = "quit";

      let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(MENU_SHOW, "Show"))
        .add_item(CustomMenuItem::new(MENU_TOGGLE, "Toggle Overlay"))
        .add_item(CustomMenuItem::new(MENU_QUIT, "Quit"));

      let _tray_handle = SystemTray::new()
        .with_menu(tray_menu)
        .with_tooltip("Relais")
        .on_event(move |e| match &e {
          SystemTrayEvent::LeftClick { .. } => _window_focus(&main_window).unwrap(),
          SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            MENU_SHOW => _window_focus(&main_window).unwrap(),
            MENU_TOGGLE => (),
            MENU_QUIT => exit_0(&handle).expect("Failed to remove tasktray icon"),
            _ => (),
          },
          _ => (),
        })
        .build(app)?;
      //

      Ok(())
    })
    .on_window_event(move |e| match e.event() {
      WindowEvent::Destroyed => println!("destroy!"),
      WindowEvent::ThemeChanged(theme) => println!("theme = {:?}", theme),
      _ => (),
    })
    .manage(state)
    .invoke_handler(generate_handler![
      exit,
      window_focus,
      window_hide,
      open_window,
      close_window,
      get_transparent
    ])
    .run(generate_context!())
    .expect("error while running tauri application");
}
