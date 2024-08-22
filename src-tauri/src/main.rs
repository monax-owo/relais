// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{
  generate_context, generate_handler, App, Builder, CustomMenuItem, Manager, SystemTray,
  SystemTrayEvent, SystemTrayMenu, WindowEvent,
};

mod command;

use command::*;

// #[cfg(target_os = "windows")]
// use {
//   std::os::raw::c_void,
//   windows::Win32::{
//     Foundation::{BOOL, HWND},
//     Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_TRANSITIONS_FORCEDISABLED},
//   },
// };

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct AppState {
  windows: Mutex<Vec<WindowData>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct WindowData {
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
}

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  tauri_specta::ts::export(
    specta::collect_types![exit, main_window_focus, open_window],
    "../src/lib/generated/specta/bindings.ts",
  )
  .expect("failed to generate types");

  let builder = Builder::default();
  let mut state = AppState {
    windows: Mutex::new(vec![]),
  };
  builder
    .setup(move |app: &mut App| {
      let handle = app.handle();
      let main_window = app.get_window("main").expect("Failed to get main window");

      #[cfg(debug_assertions)]
      {
        // main_window.open_devtools();

        // if cfg!(target_os = "windows") {
        //   if let Ok(hwnd) = main_window.hwnd() {
        //     unsafe {
        //       let _ = DwmSetWindowAttribute::<HWND>(
        //         hwnd,
        //         DWMWA_TRANSITIONS_FORCEDISABLED,
        //         &mut BOOL::from(true) as *mut _ as *mut c_void,
        //         std::mem::size_of::<BOOL>() as u32,
        //       );
        //     }
        //   }
        // }
      }

      #[cfg(not(debug_assertions))]
      {
        window_focus(&main_window)?;
      }

      let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "Show window"))
        .add_item(CustomMenuItem::new("quit", "Quit"));

      let _tray_handle = SystemTray::new()
        .with_menu(tray_menu)
        .with_tooltip("Relais")
        .on_event(move |e| match &e {
          SystemTrayEvent::LeftClick { .. } => {
            // window_focus(&main_window).expect("failed to focusing main window")
          }
          SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "show" => window_focus(&main_window).expect("failed to focusing main window"),
            "quit" => exit_0(&handle).expect("Failed to remove tasktray icon"),
            _ => (),
          },
          _ => (),
        })
        .build(app)?;
      Ok(())
    })
    .on_window_event(move |e| match e.event() {
      WindowEvent::Destroyed => println!("destroy!"),
      WindowEvent::CloseRequested { api, .. } => match e.window().label() {
        "main" => api.prevent_close(),
        "test" => todo!(),
        _ => (),
      },
      _ => (),
    })
    .manage(state)
    .invoke_handler(generate_handler![exit, main_window_focus, open_window])
    .run(generate_context!())
    .expect("error while running tauri application");
}
