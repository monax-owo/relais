// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use specta_typescript::Typescript;
use std::{env, panic, sync::Arc};
use tauri::{
  generate_context,
  image::Image,
  menu::{MenuBuilder, MenuItem},
  tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
  App, Builder, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent,
};
use tauri_plugin_global_shortcut::ShortcutState;
use tauri_specta::{collect_commands, collect_events};
use util::{exit_0, AppState, Conf, SerDeAppState, SerDeWindowData};
use view::{
  event::{UpdateState, UpdateWindows},
  util::window_focus,
};

pub mod command;
pub mod util;
pub mod view;

const MAIN_LABEL: &str = "main";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // custom panic hook
  let hook = panic::take_hook();
  panic::set_hook(Box::new(move |panic_hook_info| {
    println!(
      r#"
------------
hook!
------------
    "#
    );
    hook(panic_hook_info);
  }));
  //

  // specta
  let specta = tauri_specta::Builder::new()
    .commands(collect_commands![
      command::exit,
      command::get_config,
      command::get_state,
      command::get_windows,
      view::command::view_create,
      view::command::window_focus,
      view::command::window_hide,
      view::ctrl::command::get_status,
      view::ctrl::command::sync_windows,
      view::ctrl::command::view_close,
      view::ctrl::command::view_drag,
      view::ctrl::command::view_minimize,
      view::ctrl::command::view_zoom,
      view::ctrl::ignore_cursor_events::get_ignore_cursor_events,
      view::ctrl::ignore_cursor_events::set_ignore_cursor_events,
      view::ctrl::ignore_cursor_events::toggle_ignore_cursor_events,
      view::ctrl::pin::command::get_pin,
      view::ctrl::pin::command::set_pin,
      view::ctrl::pin::command::toggle_pin,
      view::ctrl::transparent::get_transparent,
      view::ctrl::transparent::set_transparent,
      view::ctrl::transparent::toggle_transparent,
      view::ctrl::user_agent::get_user_agent,
      view::ctrl::user_agent::set_user_agent,
      view::ctrl::user_agent::toggle_user_agent,
      view::extension::command::test,
    ])
    .constant("CTRL_LABEL_PREFIX", view::util::CTRL_LABEL_PREFIX)
    .constant("WINDOW_LABEL_PREFIX", view::util::WINDOW_LABEL_PREFIX)
    .events(collect_events![UpdateState, UpdateWindows])
    .typ::<SerDeAppState>()
    .typ::<SerDeWindowData>();
  #[cfg(debug_assertions)]
  specta
    .export(
      Typescript::default(),
      "../src/lib/generated/specta/bindings.ts",
    )
    .expect("failed to generate types");
  //

  // state
  let path = if cfg!(debug_assertions) {
    env::current_dir().unwrap().parent().unwrap().join("temp")
  } else {
    env::current_exe().unwrap().parent().unwrap().to_path_buf()
  }
  .join(util::CONFIGFILE_NAME);
  let state = util::AppState::new(&path, |b| b.data(Conf::new())).unwrap();
  //

  //
  Builder::default()
    .invoke_handler(specta.invoke_handler())
    .setup(move |app: &mut App| {
      let handle = app.handle();
      let state = app.state::<AppState>();
      specta.mount_events(app);

      let main_window = Arc::new(
        WebviewWindowBuilder::new(app, MAIN_LABEL, WebviewUrl::App("".into()))
          .title("Relais")
          .inner_size(400.0, 260.0)
          .min_inner_size(400.0, 260.0)
          .build()
          .expect("failed to create main window"),
      );

      #[cfg(not(debug_assertions))]
      {
        view::util::window_focus(&main_window)?;
      }
      println!("{:#?}", state.config);

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
      //

      // TODO:TOGGLEですべてのウィンドウをshow/hideする
      // TODO:クリップボードのurlを開くメニューを追加
      // tray menu
      {
        const SHOW: &str = "show";
        const TOGGLE: &str = "toggle";
        const QUIT: &str = "quit";
        let tray_icon = {
          // Image::from_path("icons/icon.png").unwrap()
          Image::from_bytes(include_bytes!("../icons/128x128.png")).unwrap()
        };
        let tray_menu = MenuBuilder::new(app)
          .items(&[
            &MenuItem::with_id(app, SHOW, "Show", true, None::<&str>)?,
            &MenuItem::with_id(app, TOGGLE, "Toggle Overlay", true, None::<&str>)?,
            &MenuItem::with_id(app, QUIT, "Quit", true, None::<&str>)?,
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
              SHOW => window_focus(&main_window).unwrap(),
              TOGGLE => (),
              QUIT => exit_0(app).unwrap(),
              _ => (),
            }
          })
          .build(app)?;
      }
      //

      // shortcut
      handle
        .plugin(
          tauri_plugin_global_shortcut::Builder::new()
            .with_shortcut(state.config.read().unwrap().shortcut_key.as_str())
            .unwrap()
            .with_handler(move |_app, shortcut, e| {
              if e.state == ShortcutState::Pressed {
                println!("press:{}", shortcut.into_string());
              }
            })
            .build(),
        )
        .expect("failed to set global shortcut");
      //

      // restore views from config
      view::ctrl::view_restore(handle, &state).expect("failed to restore views");
      //

      Ok(())
    })
    .on_window_event(move |_window, e| match e {
      WindowEvent::ScaleFactorChanged { .. } => println!("scale changed"),
      WindowEvent::ThemeChanged(theme) => println!("theme = {:#?}", theme),
      _ => (),
    })
    .manage(state)
    .run(generate_context!())
    .expect("error while running tauri application");
}
