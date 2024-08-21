// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use serde::{Deserialize, Serialize};
use std::{error::Error, os::raw::c_void};
use tauri::{
  generate_context, generate_handler, App, Builder, CustomMenuItem, Manager, SystemTray,
  SystemTrayEvent, SystemTrayMenu, WindowEvent,
};

mod command;

use command::*;

#[cfg(target_os = "windows")]
use windows::Win32::{
  Foundation::{BOOL, HWND},
  Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_TRANSITIONS_FORCEDISABLED},
};

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  tauri_specta::ts::export(
    specta::collect_types![exit, main_window_focus, open_url],
    "../src/lib/generated/specta/bindings.ts",
  )
  .expect("failed to generate types");

  let builder = Builder::default();

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

      window_focus(&main_window)?;

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
        _ => (),
      },
      _ => (),
    })
    .invoke_handler(generate_handler![exit, main_window_focus, open_url])
    .run(generate_context!())
    .expect("error while running tauri application");
}
