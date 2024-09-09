use std::path::Path;

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::util::SourceAppState;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct AppConfig {
  key: String,
}

impl AppConfig {
  pub fn build<P: AsRef<Path>>(_path: P) -> Self {
    // let path = path.as_ref();
    // let mut builder = config::Config::builder();
    // builder = builder.set_default("key", "value").unwrap();
    // dbg!(&path);
    // if path.exists() {
    //   builder = builder.add_source(config::File::with_name(path.to_str().unwrap()));
    // }
    // builder.build().unwrap()
    todo!()
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
