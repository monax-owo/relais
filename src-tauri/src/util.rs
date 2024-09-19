use anyhow::Context;
use conf::AppConfig;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{
  fmt::Display,
  path::Path,
  sync::{
    atomic::{AtomicBool, AtomicU8, Ordering},
    Arc, Mutex, RwLock,
  },
};
use tauri::{AppHandle, Emitter};

pub const CONFIGFILE_NAME: &str = "relaisrc.toml";

#[derive(Debug)]
pub struct AppState<T = Conf>
where
  T: for<'de> Deserialize<'de> + Serialize,
{
  pub config: AppConfig<T>,
  pub(crate) agent: RwLock<String>,
  pub(crate) windows: Mutex<Vec<WindowData>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct SerdeAppState {
  pub config: String,
  pub agent: String,
  pub windows: Vec<SerdeWindowData>,
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
pub struct SerdeWindowData {
  title: String,
  label: String,
  pointer_ignore: bool,
  mobile_mode: bool,
  transparent: (bool, u8),
  pin: bool,
  zoom: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct Conf {
  pub agent: String,
  pub windows: Vec<SerdeWindowData>,
}

// TODO:何故か勝手にDefaultトレイトが実装される。rust 1.80の影響？

impl Default for Conf {
  fn default() -> Self {
    Self::new()
  }
}

impl Conf {
  pub fn new() -> Self {
    Self {
      agent: String::new(),
      windows: Vec::new(),
    }
  }
}

impl<T: for<'de> Deserialize<'de> + Serialize> AppState<T> {
  pub fn new<P: AsRef<Path>>(config_path: P, data: T) -> anyhow::Result<Self> {
    Ok(Self {
      config: AppConfig::new(config_path, data)?,
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
    let vec = windows.clone().into_iter().map(|v| v.into()).collect();
    handle
      .emit::<Vec<SerdeWindowData>>("update_windows", vec)
      .unwrap();
  }

  pub fn get_window_data(&self, label: &str) -> anyhow::Result<WindowData> {
    let lock = self.windows.lock().unwrap();
    lock
      .iter()
      .find(|v| v.label.as_str() == label)
      .cloned()
      .context("failed to get window data")
  }

  pub fn get_windows(&self) -> Vec<SerdeWindowData> {
    let lock = self.windows.lock().unwrap();
    lock.clone().into_iter().map(|v| v.into()).collect()
  }
  // window
}

impl AppState<Conf> {
  // config
  pub fn write_conf(&mut self) {
    self.config.windows = self.get_windows();
  }
}

impl WindowData {
  pub fn new(title: String, label: String) -> Self {
    Self {
      title,
      label,
      pointer_ignore: Arc::from(AtomicBool::new(false)),
      mobile_mode: Arc::from(AtomicBool::new(false)),
      transparent: Arc::from((AtomicBool::new(false), AtomicU8::new(127))),
      pin: Arc::from(AtomicBool::new(false)),
      zoom: Arc::from(Mutex::new(100)),
    }
  }
}

impl From<WindowData> for SerdeWindowData {
  fn from(v: WindowData) -> Self {
    Self {
      title: v.title,
      label: v.label,
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

pub trait ErrToString<T, E: Display> {
  fn err_to_string(self) -> Result<T, String>;
}

impl<T, E: Display> ErrToString<T, E> for Result<T, E> {
  fn err_to_string(self) -> Result<T, String> {
    self.map_err(|e| e.to_string())
  }
}

pub fn exit_0(handle: &AppHandle) -> anyhow::Result<()> {
  handle
    .remove_tray_by_id("tray")
    .context("tray is not found")?;
  handle.exit(0);
  Ok(())
}
