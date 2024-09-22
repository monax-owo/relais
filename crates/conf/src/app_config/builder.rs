use std::{
  fs::{create_dir_all, File},
  path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};

use crate::Configurable;

use super::{AppConfig, EmptyConfig};

#[derive(Debug)]
pub struct AppConfigBuilder<T = EmptyConfig> {
  file_path: PathBuf,
  data: T,
}

impl AppConfigBuilder<EmptyConfig> {
  pub fn new<P: AsRef<Path>>(path: P) -> Self {
    Self {
      file_path: path.as_ref().to_path_buf(),
      data: EmptyConfig {},
    }
  }
}

impl<T> AppConfigBuilder<T>
where
  T: for<'de> Deserialize<'de> + Serialize,
{
  // TODO:トレイト境界を2回書かないといけないのか？
  pub fn data<U>(self, data: U) -> AppConfigBuilder<U>
  where
    U: for<'de> Deserialize<'de> + Serialize,
  {
    AppConfigBuilder {
      file_path: self.file_path,
      data,
    }
  }
  pub fn build(self) -> anyhow::Result<AppConfig<T>> {
    let path = self.file_path;
    let parent = path.parent().context("no parent")?;
    if !parent.exists() {
      create_dir_all(parent)?;
    }
    if !path.exists() {
      File::create(&path)?;
    }
    if !path.is_file() {
      bail!("path is not file")
    }

    let mut conf = AppConfig {
      file_path: path,
      config: self.data,
    };
    AppConfig::load(&mut conf)?;

    Ok(conf)
  }
}
