use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

use crate::util::SWindowDatas;

#[derive(Debug, Clone, Deserialize, Serialize, Type, Event)]
pub struct UpdateState(pub SWindowDatas);

#[derive(Debug, Clone, Deserialize, Serialize, Type, Event)]
pub struct UpdateWindows(());
