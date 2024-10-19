use configu::{AppConfig, AppConfigBuilder};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{
  path::Path,
  sync::{
    atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering},
    Arc, Mutex,
  },
};
use tauri::{AppHandle, WebviewUrl};
use tauri_specta::Event;

use crate::view::event::UpdateState;

pub const CONFIGFILE_NAME: &str = "relaisrc.toml";

#[derive(Debug)]
pub struct AppState<T = Conf>
where
  T: for<'de> Deserialize<'de> + Serialize,
{
  pub config: AppConfig<T>,
  pub(crate) windows: Mutex<WindowDataList>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct SAppState {
  pub config: String,
  pub windows: SWindowList,
}

#[derive(Debug, Clone)]
pub struct WindowData {
  pub title: String,
  pub label: String,
  pub url: WebviewUrl,
  pub(crate) pointer_ignore: Arc<AtomicBool>,
  pub(crate) mobile_mode: Arc<AtomicBool>,
  pub(crate) transparent: Arc<(AtomicBool, AtomicU8)>,
  pub(crate) pin: Arc<AtomicBool>,
  pub(crate) zoom: Arc<AtomicU32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct SWindowData {
  pub title: Box<str>,
  pub label: Box<str>,
  pub url: Box<str>,
  pub pointer_ignore: bool,
  pub mobile_mode: bool,
  pub transparent: (bool, u8),
  pub pin: bool,
  pub zoom: u32,
}

pub type WindowDataList = Vec<WindowData>;
pub type SWindowList = Vec<SWindowData>;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct Conf {
  pub agent_desktop: String,
  pub agent_mobile: String,
  pub shortcut_key: String,
  pub windows: SWindowList,
}

impl Default for Conf {
  fn default() -> Self {
    Self::new()
  }
}

impl Conf {
  pub fn new() -> Self {
    Self {
      agent_desktop: String::new(),
      agent_mobile: String::new(),
      shortcut_key: "ctrl+alt+r".into(),
      windows: Vec::new(),
    }
  }
}

impl<T> AppState<T>
where
  T: for<'de> Deserialize<'de> + Serialize,
{
  pub fn new<P, F>(config_path: P, f: F) -> anyhow::Result<Self>
  where
    P: AsRef<Path>,
    F: Fn(AppConfigBuilder) -> AppConfigBuilder<T>,
  {
    Ok(Self {
      config: f(AppConfig::<T>::open(config_path)).build()?,
      windows: Mutex::new(Vec::new()),
    })
  }

  // window
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

  pub fn emit_windows(&self, handle: &AppHandle) {
    let windows = self.windows.lock().unwrap();
    let vec = windows.iter().map(|v| v.into()).collect();
    UpdateState(vec).emit(handle).unwrap();
  }

  // TODO:ResultではなくOptionの方がいい
  // TODO:lockを関数の外で取得しないといけない(寿命が足りないから)
  /// labelから合致するウィンドウを探して返す関数
  pub fn get_window_data(&self, label: &str) -> Option<WindowData> {
    self
      .windows
      .lock()
      .unwrap()
      .iter()
      .find(|v| v.label.as_str() == label)
      .cloned()
  }

  pub fn get_windows(&self) -> SWindowList {
    let lock = self.windows.lock().unwrap();
    lock.iter().map(|v| v.into()).collect()
  }
  //
}

impl TryFrom<&AppState> for SAppState {
  type Error = anyhow::Error;

  fn try_from(v: &AppState) -> Result<Self, Self::Error> {
    Ok(Self {
      config: "".into(),
      windows: v.windows.lock().unwrap().iter().map(|v| v.into()).collect(),
    })
  }
}

impl WindowData {
  pub fn new(title: String, label: String, url: WebviewUrl) -> Self {
    Self {
      title,
      label,
      url,
      pointer_ignore: Arc::new(AtomicBool::new(false)),
      mobile_mode: Arc::new(AtomicBool::new(false)),
      transparent: Arc::new((AtomicBool::new(false), AtomicU8::new(127))),
      pin: Arc::new(AtomicBool::new(false)),
      zoom: Arc::new(AtomicU32::new(100)),
    }
  }
}

impl From<&WindowData> for SWindowData {
  fn from(v: &WindowData) -> Self {
    Self {
      title: v.title.as_str().into(),
      label: v.label.as_str().into(),
      url: v.url.to_string().into(),
      pointer_ignore: Arc::clone(&v.pointer_ignore).load(Ordering::Acquire),
      mobile_mode: Arc::clone(&v.mobile_mode).load(Ordering::Acquire),
      transparent: {
        let arc = Arc::clone(&v.transparent);
        (arc.0.load(Ordering::Acquire), arc.1.load(Ordering::Acquire))
      },
      pin: Arc::clone(&v.pin).load(Ordering::Acquire),
      zoom: v.zoom.load(Ordering::Acquire),
    }
  }
}
