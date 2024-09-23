use anyhow::Context;
use conf::{AppConfig, AppConfigBuilder};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{
  path::Path,
  sync::{
    atomic::{AtomicBool, AtomicU8, Ordering},
    Arc, Mutex, RwLock,
  },
};
use tauri::AppHandle;
use tauri_specta::Event;

use crate::view::event::UpdateState;

pub const CONFIGFILE_NAME: &str = "relaisrc.toml";

#[derive(Debug)]
pub struct AppState<T = Conf>
where
  T: for<'de> Deserialize<'de> + Serialize,
{
  pub config: AppConfig<T>,
  pub(crate) agent: RwLock<String>,
  pub(crate) windows: Mutex<WindowDatas>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct SAppState {
  pub config: String,
  pub agent: String,
  pub windows: SWindowDatas,
}

#[derive(Debug, Clone)]
pub struct WindowData {
  pub title: String,
  pub label: String,
  pub(crate) pointer_ignore: Arc<AtomicBool>,
  pub(crate) mobile_mode: Arc<AtomicBool>,
  pub(crate) transparent: Arc<(AtomicBool, AtomicU8)>,
  pub(crate) pin: Arc<AtomicBool>,
  pub(crate) zoom: Arc<Mutex<u32>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct SWindowData {
  title: Box<str>,
  label: Box<str>,
  pointer_ignore: bool,
  mobile_mode: bool,
  transparent: (bool, u8),
  pin: bool,
  zoom: u32,
}

pub type WindowDatas = Vec<WindowData>;
pub type SWindowDatas = Vec<SWindowData>;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct Conf {
  pub agent: String,
  pub shortcut_key: String,
  pub windows: SWindowDatas,
}

impl Default for Conf {
  fn default() -> Self {
    Self::new()
  }
}

impl Conf {
  pub fn new() -> Self {
    Self {
      agent: String::new(),
      shortcut_key: "ctrl+alt+r".into(),
      windows: Vec::new(),
    }
  }
}

// TODO:綺麗な実装にする
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
      agent: RwLock::new(String::default()),
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

  pub fn get_window_data(&self, label: &str) -> anyhow::Result<WindowData> {
    let lock = self.windows.lock().unwrap();
    lock
      .iter()
      .find(|v| v.label.as_str() == label)
      .cloned()
      .context("failed to get window data")
  }

  pub fn get_windows(&self) -> SWindowDatas {
    let lock = self.windows.lock().unwrap();
    lock.iter().map(|v| v.into()).collect()
  }
  // window
}

impl AppState<Conf> {
  // config

  pub fn write_conf(&mut self) {
    self.config.windows = self.get_windows();
  }
}

impl TryFrom<&AppState> for SAppState {
  type Error = anyhow::Error;

  fn try_from(v: &AppState) -> Result<Self, Self::Error> {
    Ok(Self {
      config: "".into(),
      agent: v.agent.read().unwrap().to_string(),
      windows: v.windows.lock().unwrap().iter().map(|v| v.into()).collect(),
    })
  }
}

impl WindowData {
  pub fn new(title: String, label: String) -> Self {
    Self {
      title,
      label,
      pointer_ignore: Arc::new(AtomicBool::new(false)),
      mobile_mode: Arc::new(AtomicBool::new(false)),
      transparent: Arc::new((AtomicBool::new(false), AtomicU8::new(127))),
      pin: Arc::new(AtomicBool::new(false)),
      zoom: Arc::new(Mutex::new(100)),
    }
  }
}

impl From<&WindowData> for SWindowData {
  fn from(v: &WindowData) -> Self {
    Self {
      title: v.title.as_str().into(),
      label: v.label.as_str().into(),
      pointer_ignore: Arc::clone(&v.pointer_ignore).load(Ordering::Acquire),
      mobile_mode: Arc::clone(&v.mobile_mode).load(Ordering::Acquire),
      transparent: {
        let arc = Arc::clone(&v.transparent);
        (arc.0.load(Ordering::Acquire), arc.1.load(Ordering::Acquire))
      },
      pin: Arc::clone(&v.pin).load(Ordering::Acquire),
      zoom: *v.zoom.lock().unwrap(),
    }
  }
}