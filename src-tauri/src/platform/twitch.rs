use crate::Store;
use percent_encoding::{self, utf8_percent_encode, NON_ALPHANUMERIC};
use tauri::{
  api::shell::{open, Program},
  AppHandle, Manager, Runtime, Window,
};
use url::Url;

const CLIENT_ID: &str = "9be7sjh036h3v8enjgk89md0gyg9fd";
const SCOPES: &str = "user_read user:read:broadcast bits:read chat:read channel:read:redemptions";

#[tauri::command]
#[specta::specta]
pub async fn authorize(app: AppHandle, window: Window) -> anyhow::Result<()> {
  // let client = &app.state::<Store>().client;
  let base_url = "https://id.twitch.tv/oauth2/authorize";
  let scopes = utf8_percent_encode(SCOPES, NON_ALPHANUMERIC).to_string();
  let querys = [("client_id", CLIENT_ID), ("scope", &scopes)];
  let url = Url::parse_with_params(&base_url, querys)?;
  open(&app.shell_scope(), url, Some(Program::Open))?;
  Ok(())
}
