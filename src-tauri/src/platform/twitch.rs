use crate::Store;
use tauri::{
  api::shell::{open, Program},
  AppHandle, Manager, Window,
};
use url::Url;

const CLIENT_ID: &str = "9be7sjh036h3v8enjgk89md0gyg9fd";
const SCOPES: &str = "user_read user:read:broadcast bits:read chat:read channel:read:redemptions";

// .map_err(|e| e.to_string())?

#[tauri::command]
#[specta::specta]
pub fn authorize(app: AppHandle, _window: Window) -> Result<(), String> {
  // let client = &app.state::<Store>().client;
  let base_url = "https://id.twitch.tv/oauth2/authorize";
  let querys = [
    ("client_id", CLIENT_ID),
    ("redirect_uri", "http://localhost"),
    ("response_type", "token"),
    ("scope", SCOPES),
  ];
  let url = Url::parse_with_params(&base_url, querys).map_err(|e| e.to_string())?;
  dbg!(&url);
  open(&app.shell_scope(), url, None).map_err(|e| e.to_string())?;
  Ok(())
}
