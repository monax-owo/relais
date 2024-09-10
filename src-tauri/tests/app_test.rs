use std::env;

use app_lib::{
  self,
  util::{self, SourceAppState},
};
#[test]
fn conf() {
  let current_exe = env::current_exe().unwrap();
  let current_dir = current_exe.parent().unwrap();
  let path = current_dir.join(util::CONFIGFILE_NAME);
  let state = SourceAppState::new(path).unwrap();

  println!("{:?}", *state.c());
}
