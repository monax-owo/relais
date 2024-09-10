use std::{
  fs::OpenOptions,
  io::{BufReader, Read},
  path::Path,
};

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::util::SourceAppState;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[derive(Default)]
pub struct AppConfig {
  configfile_path: String,
  key: String,
}


// todo:必要になったらBuilder patternにする(AppConfigBuilder)
impl AppConfig {
  pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
    let path = path.as_ref();
    let file = OpenOptions::new()
      .create(true)
      .read(true)
      .write(true)
      .open(path)?;
    let mut buf_reader = BufReader::new(&file);
    let mut buf = String::new();
    buf_reader.read_to_string(&mut buf)?;
    let config = toml::from_str::<Self>(&buf).unwrap();

    Ok(config)
  }
}

impl SourceAppState {
  pub fn _write(&self) -> anyhow::Result<()> {
    todo!()
  }
  pub fn _read(&self) -> anyhow::Result<()> {
    todo!()
  }
}
