// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::{error::Error, os::raw::c_void, sync::Arc};
use tauri::{
  generate_context, generate_handler, App, Builder, CustomMenuItem, Manager, PhysicalPosition,
  PhysicalSize, SystemTray, SystemTrayEvent, SystemTrayMenu, Window, WindowEvent,
};
use tokio::sync::Mutex;
use twitch::authorize;

mod command;
mod platform;

use command::*;
use platform::*;

#[cfg(target_os = "windows")]
use windows::Win32::{
  Foundation::{BOOL, HWND},
  Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_TRANSITIONS_FORCEDISABLED},
};

pub struct Store {
  client: Client,
  twitch_token: Arc<Mutex<String>>,
}

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  tauri_specta::ts::export(
    specta::collect_types![authorize, exit, main_window_focus],
    "../src/lib/generated/specta/bindings.ts",
  )
  .expect("failed to generate types");

  let builder = Builder::default();
  let state = Store {
    client: Client::new(),
    twitch_token: Arc::new(Mutex::new(String::default())),
  };

  builder
    .setup(
      move |app: &mut App| -> Result<_, Box<(dyn Error + 'static)>> {
        let handle = app.handle();
        let main_window = app.get_window("main").expect("Failed to get main window");

        set_pos(&main_window);
        window_hide(&main_window)?;

        #[cfg(debug_assertions)]
        {
          let res: anyhow::Result<()> = {
            main_window.open_devtools();
            println!("is dev");

            // main_window.set_ignore_cursor_events(true).unwrap();
            // ts側でbodyにカーソルが乗っているときだけtrueにする？

            Ok(())
          };

          if res.is_ok() {
            println!("success");
          } else {
            println!("failure");
          }
        }

        if cfg!(target_os = "windows") {
          if let Ok(hwnd) = main_window.hwnd() {
            unsafe {
              let _ = DwmSetWindowAttribute::<HWND>(
                hwnd,
                DWMWA_TRANSITIONS_FORCEDISABLED,
                &mut BOOL::from(true) as *mut _ as *mut c_void,
                std::mem::size_of::<BOOL>() as u32,
              );
            }
          }
        }

        let tray_menu = SystemTrayMenu::new()
          .add_item(CustomMenuItem::new("show", "Show window"))
          .add_item(CustomMenuItem::new("quit", "Quit"));

        let _tray_handle = SystemTray::new()
          .with_menu(tray_menu)
          .with_tooltip("Relais")
          .on_event(move |e| match &e {
            SystemTrayEvent::LeftClick { .. } => {
              window_focus(&main_window).expect("failed to focusing main window")
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
              "show" => window_focus(&main_window).expect("failed to focusing main window"),
              "quit" => exit_0(&handle).expect("Failed to remove tasktray icon"),
              &_ => (),
            },
            _ => (),
          })
          .build(app)?;
        Ok(())
      },
    )
    .on_window_event(move |e| match e.event() {
      WindowEvent::Resized(_) => set_pos(e.window()),
      WindowEvent::Destroyed => println!("destroy!"),
      WindowEvent::Focused(focus) => {
        if !*focus {
          window_hide(e.window()).expect("failed to hide window");
        }
      }
      _ => (),
    })
    .manage(state)
    .invoke_handler(generate_handler![authorize, exit, main_window_focus])
    .run(generate_context!())
    .expect("error while running tauri application");
}

fn set_pos(window: &Window) {
  const OFFSET: u32 = 1;
  let monitor = window.current_monitor().unwrap().unwrap();
  let monitor_size = monitor.size();
  let size = [
    monitor_size.width - (OFFSET * 2),
    monitor_size.height - (OFFSET * 2),
  ];
  let size = PhysicalSize::new(size[0], size[1]);
  let pos = PhysicalPosition::new(OFFSET, OFFSET);
  dbg!(&size);
  dbg!(&pos);
  window.set_size(size).expect("Failed to set size");
  window.set_position(pos).expect("Failed to set position");
  // 0,0だとYoutubeが止まる。原因不明。ウィンドウがかぶさると動画が再生されないようになっている？
  // |->1pxだけ隙間を開けた

  println!("set position");
}
