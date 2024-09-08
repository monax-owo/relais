use std::path::Path;

use config::Config;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::util::SourceAppState;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct AppConfig<'a> {
  key: String,
  test: &'a str,
}

impl AppConfig<'_> {
  pub fn build<P: AsRef<Path>>(path: P) -> Config {
    let path = path.as_ref();
    let mut builder = config::Config::builder();
    builder = builder.set_default("key", "value").unwrap();
    dbg!(&path);
    if path.exists() {
      builder = builder.add_source(config::File::with_name(path.to_str().unwrap()));
    }
    builder.build().unwrap()
  }

  pub fn try_deserialize(config: &Config) -> anyhow::Result<AppConfig<'static>> {
    Ok(config.clone().try_deserialize::<AppConfig>()?)
  }
}

impl SourceAppState {
  pub fn _write_config() {
    todo!()
  }
  pub fn _read_config() {
    todo!()
  }
}
