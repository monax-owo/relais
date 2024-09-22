use std::fmt::Debug;

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
