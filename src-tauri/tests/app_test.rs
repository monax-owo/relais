use std::{collections::HashMap, env, fs::File, io::Write, path::PathBuf, sync::LazyLock};

use app_lib::{
  self,
  util::{AppState, CONFIGFILE_NAME},
};
use configu::Configurable;
use serde::{Deserialize, Serialize};

// TODO:add tuple
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

// TODO:HashMapにも最小値/最大値を決める
impl TestConf {
  /// return maximum value of Self.
  pub fn _min() -> Self {
    const ARR: [i32; 4] = [i32::MIN, i32::MIN, i32::MIN, i32::MIN];
    Self {
      u8: u8::MIN,
      u16: u16::MIN,
      u32: u32::MIN,
      u64: u64::MIN,
      i8: i8::MIN,
      i16: i16::MIN,
      i32: i32::MIN,
      i64: i64::MIN,
      f32: f32::MIN,
      f64: f64::MIN,
      arr: ARR,
      bool: false,
      char: 'a',
      str: "min".into(),
      string: String::from("minimum value"),
      vec: Vec::from(ARR),
      hash_map: HashMap::new(),
    }
  }
  /// return maximum value of Self.
  pub fn _max() -> Self {
    const ARR: [i32; 4] = [i32::MAX, i32::MAX, i32::MAX, i32::MAX];
    Self {
      u8: u8::MAX,
      u16: u16::MAX,
      u32: u32::MAX,
      u64: u64::MAX,
      i8: i8::MAX,
      i16: i16::MAX,
      i32: i32::MAX,
      i64: i64::MAX,
      f32: f32::MAX,
      f64: f64::MAX,
      arr: ARR,
      bool: false,
      char: 'z',
      str: "max".into(),
      string: String::from("maximum value"),
      vec: Vec::from(ARR),
      hash_map: HashMap::new(),
    }
  }
  fn _random() -> Self {
    // todo:設定をランダムに変える
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

static CONTENT: LazyLock<String> = LazyLock::new(|| toml::to_string_pretty(&TestConf::default()).expect("failed to initialize CONTENT"));

/// # Panics
/// Panics if failure to get/set state.
#[serial_test::serial]
#[test]
fn save_and_load() {
  dbg!(PATH.to_str());
  dbg!(CONTENT.as_str());

  let mut state = initialize_state();
  state.config.save().expect("failed to save configfile");

  println!("{:#?}", state.config);
  state.config.load().unwrap();
}

/// Helper for initialize state.
/// # Panics
/// Panics if .
fn initialize_state() -> AppState<TestConf> {
  let state = AppState::new(PATH.as_path(), |b| b.data(TestConf::default())).unwrap();
  let mut file = File::options()
    .write(true)
    .create(true)
    .truncate(true)
    .open(PATH.as_path())
    .expect("could not open configfile");
  file.write_all(CONTENT.as_bytes()).expect("failed to writing to configfile");
  state
}
