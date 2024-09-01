// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::Config;
use serde::{Deserialize, Serialize};
use specta::Type;
use specta_typescript::Typescript;
use std::{
  collections::HashMap,
  env,
  path::PathBuf,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
  },
};
use tauri::{
  generate_context,
  menu::{MenuBuilder, MenuItem},
  tray::{TrayIconBuilder, TrayIconEvent},
  App, AppHandle, Builder, Emitter, Manager, WindowEvent,
};
use tauri_specta::collect_commands;

mod command;
mod window;
mod util;

use command::*;
use window::command::*;

// #[cfg(target_os = "windows")]
// use {
//   std::os::raw::c_void,
//   windows::Win32::{
//     Foundation::{BOOL, HWND},
//     Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_TRANSITIONS_FORCEDISABLED},
//   },
// };

const CONFIGFILE_NAME: &str = "relaisrc.toml";

// TODO: アプリ全体かウィンドウごとに半透明にするか
#[derive(Debug)]
pub struct SourceAppState {
  config: Config,
  windows: Mutex<Vec<SourceWindowData>>,
  pub overlay: AtomicBool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct AppState {
  config: String,
  windows: Vec<WindowData>,
  pub overlay: bool,
}

#[derive(Debug, Clone)]
pub struct SourceWindowData {
  title: String,
  label: String,
  pin: Arc<AtomicBool>,
  zoom: Arc<Mutex<f64>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct WindowData {
  title: String,
  label: String,
  pin: bool,
  zoom: f64,
}

impl SourceAppState {
  pub fn add_window(&self, window: SourceWindowData) -> anyhow::Result<()> {
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
      .emit::<Vec<WindowData>>("update_windows", vec)
      .unwrap();
  }

  pub fn get_window_data(&self, label: &str) -> Option<SourceWindowData> {
    let lock = self.windows.lock().unwrap();
    lock.iter().find(|v| v.label.as_str() == label).cloned()
  }
}

impl From<SourceWindowData> for WindowData {
  fn from(v: SourceWindowData) -> Self {
    Self {
      title: v.title,
      label: v.label,
      pin: v.pin.clone().load(Ordering::Acquire),
      zoom: *v.zoom.lock().unwrap(),
    }
  }
}

// #[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let specta = tauri_specta::Builder::new()
    .commands(collect_commands![
      exit,
      window_focus,
      window_hide,
      open_window,
      close_window,
      get_transparent,
      toggle_pin,
      mini
    ])
    .typ::<WindowData>();
  #[cfg(debug_assertions)]
  specta
    .export(
      Typescript::default(),
      "../src/lib/generated/specta/bindings.ts",
    )
    .expect("failed to generate types");

  let path = (|| -> anyhow::Result<PathBuf> {
    Ok(env::current_exe()?.parent().unwrap().join(CONFIGFILE_NAME))
  })()
  .unwrap();
  let config = {
    let mut builder = Config::builder().set_default("key", "value").unwrap();
    if path.exists() {
      builder = builder.add_source(config::File::with_name(path.to_str().unwrap()));
    }
    builder.build().unwrap()
  };

  let state = SourceAppState {
    config,
    windows: Mutex::new(vec![]),
    overlay: AtomicBool::new(false),
  };

  Builder::default()
    .invoke_handler(specta.invoke_handler())
    .setup(move |app: &mut App| {
      let _handle = app.handle();
      specta.mount_events(app);

      let main_window = Arc::new(
        app
          .get_webview_window("main")
          .expect("Failed to get main window"),
      );
      //
      #[cfg(not(debug_assertions))]
      {
        _window_focus(&main_window)?;
      }
      {
        let state = app.state::<SourceAppState>();
        println!(
          "{:?}",
          state
            .config
            .clone()
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
        );
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
      // const MENU_SHOW: &str = "show";
      // const MENU_TOGGLE: &str = "toggle";
      // const MENU_QUIT: &str = "quit";

      let tray_menu = MenuBuilder::new(app)
        .items(&[
          &MenuItem::new(app, "Show", true, None::<&str>)?,
          &MenuItem::new(app, "Toggle Overlay", true, None::<&str>)?,
          &MenuItem::new(app, "Quit", true, None::<&str>)?,
        ])
        .build()?;
      let _tray_handle = TrayIconBuilder::with_id("tray")
        .menu(&tray_menu)
        .tooltip("Relais")
        .on_tray_icon_event(move |_tray, e| {
          if let TrayIconEvent::Click { .. } = &e {
            window::util::window_focus(&main_window).unwrap()
          }
        })
        .build(app)?;
      //

      Ok(())
    })
    .on_window_event(move |_window, e| match e {
      WindowEvent::Destroyed => println!("destroy!"),
      WindowEvent::ThemeChanged(theme) => println!("theme = {:?}", theme),
      _ => (),
    })
    .manage(state)
    .run(generate_context!())
    .expect("error while running tauri application");
}
