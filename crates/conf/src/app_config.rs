mod builder;

use std::{
  fs::{create_dir_all, read_to_string, File},
  io::{BufReader, BufWriter, Read, Write},
  ops::{Deref, DerefMut},
  path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};

use crate::Configurable;

#[derive(Debug, Deserialize, Serialize)]
pub struct EmptyConfig {}

#[derive(Debug)]
pub struct AppConfig<T> {
  pub file_path: PathBuf,
  config: T,
}

impl<T> AppConfig<T>
where
  T: for<'de> Deserialize<'de> + Serialize,
{
  pub fn new<P: AsRef<Path>>(path: P, data: T) -> anyhow::Result<Self> {
    let path = path.as_ref();
    let parent = path.parent().context("no parent")?;
    if !parent.exists() {
      create_dir_all(parent)?;
    }
    if !path.exists() {
      File::create(path)?;
    }
    if !path.is_file() {
      bail!("path is not file")
    }

    let mut conf = Self {
      file_path: path.to_path_buf(),
      config: data,
    };
    Self::load(&mut conf)?;

    Ok(conf)
  }
}

impl<T: Serialize + for<'de> Deserialize<'de>> Configurable for AppConfig<T> {
  fn save(&self) -> anyhow::Result<()> {
    let mut writer = BufWriter::new(File::create(&self.file_path)?);

    let serialized = toml::to_string(&self.config)?;
    writer.write_all(serialized.as_bytes())?;

    Ok(())
  }

  fn load(&mut self) -> anyhow::Result<()> {
    let file = File::open(&self.file_path)?;
    let mut reader = BufReader::new(file);

    if read_to_string(&self.file_path)?.is_empty() {
      self.save()?;
    }

    let content = {
      let mut buf = String::new();
      reader.read_to_string(&mut buf)?;
      buf
    };

    let deserialized = toml::from_str::<T>(&content)?;
    self.config = deserialized;

    Ok(())
  }
}

impl<T: for<'de> Deserialize<'de> + Serialize> Deref for AppConfig<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.config
  }
}

impl<T: for<'de> Deserialize<'de> + Serialize> DerefMut for AppConfig<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.config
  }
}
