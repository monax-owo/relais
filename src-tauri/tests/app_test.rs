use std::{env, io::stdin};

use app_lib::{
  self,
  util::{self, SourceAppState},
};
use conf::Configurable;

#[test]
fn conf() {
  let path = if cfg!(debug_assertions) {
    env::current_dir().unwrap().parent().unwrap().join("temp")
  } else {
    env::current_exe().unwrap().parent().unwrap().to_path_buf()
  }
  .join(util::CONFIGFILE_NAME);
  dbg!(&path);

  let state = SourceAppState::new(path).unwrap();

  println!("{:?}", state.config);
  wait();
  state.config.load().unwrap();
  println!("{:?}", state.config);
}

fn wait() {
  println!("wait...");
  stdin().read_line(&mut String::new()).unwrap();
}
