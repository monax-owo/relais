pub(crate) mod builder;

use std::{
  fs::{read_to_string, File},
  io::{BufReader, BufWriter, Read, Write},
  ops::{Deref, DerefMut},
  path::{Path, PathBuf},
  sync::RwLock,
};

use serde::{Deserialize, Serialize};

use crate::{AppConfigBuilder, Configurable};

#[derive(Debug, Deserialize, Serialize)]
pub struct EmptyConfig {}

#[derive(Debug)]
pub struct AppConfig<T = EmptyConfig> {
  pub file_path: PathBuf,
  config: RwLock<T>,
}

impl<T> AppConfig<T>
where
  T: for<'de> Deserialize<'de> + Serialize,
{
  pub fn open<P>(path: P) -> AppConfigBuilder<EmptyConfig>
  where
    P: AsRef<Path>,
  {
    AppConfigBuilder::new(path)
  }
}

impl<T> Configurable for AppConfig<T>
where
  T: Serialize + for<'de> Deserialize<'de>,
{
  fn save(&self) -> anyhow::Result<()> {
    let mut writer = BufWriter::new(File::create(&self.file_path)?);

    let serialized = toml::to_string(&self.config)?;
    writer.write_all(serialized.as_bytes())?;

    Ok(())
  }

  // TODO:パースに失敗したらファイル名をoldにして新しいconfigfileを作る
  // ConfigurableHookみたいなトレイトがTに実装されていたらそれを先に呼び出すみたいなことしたい
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
    *self.config.write().unwrap() = deserialized;

    Ok(())
  }
}

impl<T> Deref for AppConfig<T>
where
  T: for<'de> Deserialize<'de> + Serialize,
{
  type Target = RwLock<T>;

  fn deref(&self) -> &Self::Target {
    &self.config
  }
}

impl<T> DerefMut for AppConfig<T>
where
  T: for<'de> Deserialize<'de> + Serialize,
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.config
  }
}
