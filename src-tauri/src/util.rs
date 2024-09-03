use anyhow::Context;
use config::Config;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{
  fmt::Display,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
  },
};
use tauri::{AppHandle, Emitter};

pub const CONFIGFILE_NAME: &str = "relaisrc.toml";

// TODO: アプリ全体かウィンドウごとに半透明にするか<-ウィンドウごとにする
#[derive(Debug)]
pub struct SourceAppState {
  pub(crate) config: Config,
  pub(crate) windows: Mutex<Vec<SourceWindowData>>,
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
  pub title: String,
  pub label: String,
  pub pin: Arc<AtomicBool>,
  pub zoom: Arc<Mutex<f64>>,
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
