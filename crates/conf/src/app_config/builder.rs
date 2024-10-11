use std::{
  fs::{create_dir_all, File},
  path::{Path, PathBuf},
  sync::RwLock,
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
  pub(crate) fn new<P>(path: P) -> Self
  where
    P: AsRef<Path>,
  {
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
  /// set data to builder and return builder.
  pub fn data<U>(self, data: U) -> AppConfigBuilder<U>
  where
    U: for<'de> Deserialize<'de> + Serialize,
  {
    AppConfigBuilder {
      file_path: self.file_path,
      data,
    }
  }

  /// Building Self.
  /// # Errors
  /// This function will return an error if build failed.
  pub fn build(self) -> anyhow::Result<AppConfig<T>> {
    let file_path = self.file_path;
    {
      let parent = file_path.parent().context("no parent")?;
      if !parent.exists() {
        create_dir_all(parent)?;
      }
    }
    if !file_path.exists() {
      File::create(&file_path)?;
    }
    if !file_path.is_file() {
      bail!("path is not file")
    }

    let mut conf = AppConfig {
      file_path,
      config: RwLock::new(self.data),
    };
    AppConfig::load(&mut conf)?;

    Ok(conf)
  }
}
