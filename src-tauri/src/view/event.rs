use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

use crate::util::SWindowList;

#[derive(Debug, Clone, Deserialize, Serialize, Type, Event)]
pub struct UpdateState(pub SWindowList);

#[derive(Debug, Clone, Deserialize, Serialize, Type, Event)]
pub struct UpdateWindows(());
