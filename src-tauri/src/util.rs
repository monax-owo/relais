use anyhow::Context;
use tauri::AppHandle;

pub fn exit_0(handle: &AppHandle) -> anyhow::Result<()> {
  handle
    .remove_tray_by_id("tray")
    .context("tray is not found")?;
  handle.exit(0);
  Ok(())
}
