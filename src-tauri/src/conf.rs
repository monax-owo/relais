use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
pub struct AppConfig<'a> {
  key: String,
  test: &'a str,
}
