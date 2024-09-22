use std::fmt::{Debug, Display};

use anyhow::Context;
use tauri::AppHandle;

pub trait ErrToString<T, E: Display> {
  fn err_to_string(self) -> Result<T, String>;
}

impl<T, E: Display> ErrToString<T, E> for Result<T, E> {
  fn err_to_string(self) -> Result<T, String> {
    self.map_err(|e| e.to_string())
  }
}

// TODO:unwrapの代わりにユーザーにエラー内容を伝えるトレイト/メソッドを作る
pub trait UnwrapWithDialog<T, E>
where
  E: Debug,
{
  fn unwrap_with_dialog(self) -> T;
}

impl<T, E> UnwrapWithDialog<T, E> for Result<T, E>
where
  E: Debug,
{
  fn unwrap_with_dialog(self) -> T {
    match self {
      Ok(t) => t,
      Err(e) => {
        panic!(
          "called `Result::unwrap_with_dialog()` on an `Err` value: {:?}",
          &e
        );
      }
    }
  }
}

pub fn exit_0(handle: &AppHandle) -> anyhow::Result<()> {
  handle
    .remove_tray_by_id("tray")
    .context("tray is not found")?;
  handle.exit(0);
  Ok(())
}
