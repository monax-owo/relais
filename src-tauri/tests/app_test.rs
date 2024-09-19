use std::{collections::HashMap, env, fs::File, io::Write, path::PathBuf, sync::LazyLock};

use app_lib::{
  self,
  util::{AppState, CONFIGFILE_NAME},
};
use conf::Configurable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct TestConf {
  u8: u8,
  u16: u16,
  u32: u32,
  u64: u64,
  i8: i8,
  i16: i16,
  i32: i32,
  i64: i64,
  f32: f32,
  f64: f64,
  arr: [i32; 4],
  bool: bool,
  char: char,
  str: Box<str>,
  string: String,
  vec: Vec<i32>,
  hash_map: HashMap<Box<str>, i32>,
}

impl TestConf {
  fn _random() -> Self {
    // TODO:設定をランダムに変える
    todo!()
  }
}

static PATH: LazyLock<PathBuf> = LazyLock::new(|| {
  env::current_dir()
    .unwrap()
    .parent()
    .unwrap()
    .join("temp")
    .join("test")
    .join(CONFIGFILE_NAME)
});

static CONTENT: LazyLock<String> = LazyLock::new(|| {
  toml::to_string_pretty(&TestConf::default()).expect("failed to initialize CONTENT")
});

// TODO:set/get,save/loadのテストを分ける
#[test]
fn save_load() {
  dbg!(PATH.to_str());
  dbg!(CONTENT.as_str());

  let mut state = setup();
  state.config.save().expect("failed to save configfile");

  println!("{:#?}", state.config);
  state.config.load().unwrap();
}

fn setup() -> AppState<TestConf> {
  let mut file = File::options()
    .write(true)
    .create(true)
    .truncate(true)
    .open(PATH.as_path())
    .expect("could not open configfile");
  file
    .write_all(CONTENT.as_bytes())
    .expect("failed to writing to configfile");
  AppState::new(PATH.as_path(), TestConf::default()).unwrap()
}
