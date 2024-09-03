// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::Config;
use specta_typescript::Typescript;
use std::{
  collections::HashMap,
  env,
  sync::{atomic::AtomicBool, Arc, Mutex},
};
use tauri::{
  generate_context,
  image::Image,
  menu::{MenuBuilder, MenuItem},
  tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
  App, Builder, Manager, WindowEvent,
};
use tauri_specta::collect_commands;
use util::{exit_0, SourceAppState, WindowData};
use view::util::window_focus;

mod command;
mod util;
mod view;

// TODO: specta,event "update_windows"
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let specta = tauri_specta::Builder::new()
    .commands(collect_commands![
      command::exit,
      command::get_windows,
      view::command::view_create,
      view::command::window_focus,
      view::command::window_hide,
      view::ctrl::command::get_transparent,
      view::ctrl::command::set_pointer_ignore,
      view::ctrl::command::set_transparent,
      view::ctrl::command::toggle_pin,
      view::ctrl::command::toggle_transparent,
      view::ctrl::command::view_close,
      view::ctrl::command::view_drag,
      view::ctrl::command::view_minimize,
      view::ctrl::command::view_zoomin,
      view::ctrl::command::view_zoomout,
    ])
    .typ::<WindowData>()
    .constant("WINDOW_LABEL_PREFIX", view::util::WINDOW_LABEL_PREFIX)
    .constant("CTRL_LABEL_PREFIX", view::util::CTRL_LABEL_PREFIX);
  #[cfg(debug_assertions)]
  specta
    .export(
      Typescript::default(),
      "../src/lib/generated/specta/bindings.ts",
    )
    .expect("failed to generate types");

  let current_dir = env::current_dir().unwrap();
  let path = current_dir.join(util::CONFIGFILE_NAME);

  let config = {
    let mut builder = Config::builder().set_default("key", "value").unwrap();
    if path.exists() {
      builder = builder.add_source(config::File::with_name(path.to_str().unwrap()));
    }
    builder.build().unwrap()
  };

  let state = util::SourceAppState {
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
        view::util::window_focus(&main_window)?;
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
      const MENU_SHOW: &str = "show";
      const MENU_TOGGLE: &str = "toggle";
      const MENU_QUIT: &str = "quit";

      let tray_icon = {
        // Image::from_path("icons/icon.png").unwrap()
        Image::from_bytes(include_bytes!("../icons/icon.png")).unwrap()
      };
      let tray_menu = MenuBuilder::new(app)
        .items(&[
          &MenuItem::with_id(app, MENU_SHOW, "Show", true, None::<&str>)?,
          &MenuItem::with_id(app, MENU_TOGGLE, "Toggle Overlay", true, None::<&str>)?,
          &MenuItem::with_id(app, MENU_QUIT, "Quit", true, None::<&str>)?,
        ])
        .build()?;
      let _tray_handle = TrayIconBuilder::with_id("tray")
        .icon(tray_icon)
        .menu(&tray_menu)
        .tooltip("Relais")
        .on_tray_icon_event({
          let main_window = Arc::clone(&main_window);
          move |_tray, e| {
            if let TrayIconEvent::Click {
              button: MouseButton::Left,
              ..
            } = e
            {
              view::util::window_focus(&main_window).unwrap()
            }
          }
        })
        .on_menu_event({
          let main_window = Arc::clone(&main_window);
          move |app, e| match e.id().as_ref() {
            MENU_SHOW => window_focus(&main_window).unwrap(),
            MENU_TOGGLE => (),
            MENU_QUIT => exit_0(app).unwrap(),
            _ => (),
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
