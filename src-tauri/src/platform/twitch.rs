use crate::Store;
use axum::{
  extract::{Query, State},
  routing::get,
  Router,
};
use serde::{Deserialize, Serialize};
use std::{
  net::{Ipv4Addr, SocketAddrV4},
  sync::Arc,
};
use tauri::{
  api::shell::open,
  AppHandle, Manager, Window,
};
use tokio::{
  net::TcpListener,
  sync::{
    mpsc::{channel, Sender},
    Mutex,
  },
};
use url::Url;

const AUTH_URL: &str = "https://id.twitch.tv/oauth2/authorize";
const CLIENT_ID: &str = "9be7sjh036h3v8enjgk89md0gyg9fd";
const SCOPES: &str = "user_read user:read:broadcast bits:read chat:read channel:read:redemptions";

const PORT: u16 = 80;
const ADDR: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT);
const AUTH_ROUTE: &str = "/auth";
const HOST: &str = "http://localhost";

#[derive(Deserialize, Serialize)]
struct ResQuery {
  code: String,
}

#[tauri::command]
#[specta::specta]
pub async fn authorize(app: AppHandle, _window: Window) -> Result<String, String> {
  let store = &app.state::<Store>();
  let token = store.twitch_token.clone();
  let base = format!("{}{}", HOST, AUTH_ROUTE);
  let querys = [
    ("client_id", CLIENT_ID),
    ("redirect_uri", &base),
    ("response_type", "code"),
    ("scope", SCOPES),
  ];
  let url = Url::parse_with_params(AUTH_URL, querys).map_err(|e| e.to_string())?;
  open(&app.shell_scope(), url, None).map_err(|e| e.to_string())?;
  let (tx, mut rx) = channel::<()>(1);
  let app = Router::new().route(
    AUTH_ROUTE,
    get(|q, t| handle_auth(q, t, tx)).with_state(token.clone()),
  );
  let listner = TcpListener::bind(ADDR).await.map_err(|e| e.to_string())?;
  axum::serve(listner, app)
    .with_graceful_shutdown(async move { rx.recv().await.unwrap() })
    .await
    .map_err(|e| e.to_string())?;
  Ok(token.clone().lock().await.to_string())
}

// #[axum::debug_handler]
async fn handle_auth(
  q: Query<ResQuery>,
  State(token): State<Arc<Mutex<String>>>,
  tx: Sender<()>,
) -> String {
  let mut lock = token.lock().await;
  *lock = q.code.to_string();
  tx.send(()).await.unwrap();
  "success".to_string()
}
