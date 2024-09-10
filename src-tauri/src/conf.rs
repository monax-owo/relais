use std::{
  fs::{create_dir, File, OpenOptions},
  io::{BufReader, BufWriter, Read, Write},
  path::{Path, PathBuf},
  sync::Mutex,
};

use anyhow::Context;
use default::*;
use serde::{Deserialize, Serialize};
use specta::Type;

pub mod default {
  pub fn key() -> String {
    String::from("teststest")
  }
}

#[derive(Debug, Type)]
pub struct AppConfig {
  file_path: PathBuf,
  config: Mutex<InnerAppConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct InnerAppConfig {
  #[serde(default = "key")]
  key: String,
}

// todo:必要になったらBuilder patternにする(AppConfigBuilder)
// TODO:hashmapにする？
impl AppConfig {
  pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
    let path = path.as_ref();
    let parent = path.parent().context("no parent")?;
    if !parent.exists() {
      create_dir(parent)?;
    }
    let file = OpenOptions::new()
      .create(true)
      .truncate(false)
      .read(true)
      .write(true)
      .open(path)?;

    let mut buf_reader = BufReader::new(&file);
    let mut buf = String::new();
    buf_reader.read_to_string(&mut buf)?;
    let config = toml::from_str::<InnerAppConfig>(&buf).unwrap();

    Ok(Self {
      file_path: path.to_path_buf(),
      config: Mutex::new(config),
    })
  }
}

pub trait Configurable {
  /// selfの内容をファイルに書き込むメソッド
  fn save(&self) -> anyhow::Result<()>;
  /// ファイルの内容をselfに書き込むメソッド
  fn load(&self) -> anyhow::Result<()>;
}

impl Configurable for AppConfig {
  fn save(&self) -> anyhow::Result<()> {
    let mut writer = BufWriter::new(File::create(&self.file_path)?);
    let lock = self.config.lock().unwrap();

    let serialized = toml::to_string(&*lock)?;
    writer.write(serialized.as_bytes())?;

    Ok(())
  }

  fn load(&self) -> anyhow::Result<()> {
    let mut reader = BufReader::new(File::open(&self.file_path)?);
    let mut lock = self.config.lock().unwrap();

    let content = {
      let mut buf = String::new();
      reader.read_to_string(&mut buf)?;
      buf
    };
    let deserialized = toml::from_str(&content)?;
    *lock = deserialized;

    Ok(())
  }
}
