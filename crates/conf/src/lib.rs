use std::{
  fs::{create_dir, File},
  io::{BufReader, BufWriter, Read, Write},
  ops::Deref,
  path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use toml_edit::Item;

pub mod default {
  pub fn key() -> String {
    String::from("teststest")
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmptyConfig {}

#[derive(Debug)]
pub struct AppConfig<T> {
  file_path: PathBuf,
  config: T,
}

// todo:必要になったらBuilder patternにする(AppConfigBuilder)
// TODO:hashmapにする？->DocumentMutを使う、ただし値のSetが面倒
// TODO:save/loadのときのみDocumentMutを使ってset/getのときはInnerAppConfigを使いたい
// TODO:DocumentMutとstructを変換する方法を調べる
// 1.文字列を中継させて変換
// 2.Index::index(str)でなんやかんやする？
// https://github.com/toml-rs/toml/issues/691
// 3.Item::Table()
// 4.一旦1で作ってみる。getやget_mutで代入する方式にする<-これでいいかな
impl<T: for<'de> Deserialize<'de> + Serialize> AppConfig<T> {
  pub fn new<P: AsRef<Path>>(path: P, data: T) -> anyhow::Result<Self> {
    let path = path.as_ref();
    if !path.exists() {
      File::create(path)?;
    }
    let parent = path.parent().context("no parent")?;
    if !parent.exists() {
      create_dir(parent)?;
    }
    if !path.is_file() {
      bail!("path is not file")
    }

    let mut conf = Self {
      file_path: path.to_path_buf(),
      config: data,
    };
    Self::load(&mut conf)?;

    Ok(conf)
  }
}

pub trait Configurable {
  /// selfの内容をファイルに書き込むメソッド
  fn save(&self) -> anyhow::Result<()>;
  /// ファイルの内容をselfに書き込むメソッド
  fn load(&mut self) -> anyhow::Result<()>;
  /// 値を取得する
  fn get(&self, key: &str) -> &Item;
  /// 値を代入する
  fn get_mut(&self, key: &str) -> &mut Item;
}

// TODO:data指定をせずに、型のみ指定できるようにする
impl<T: Serialize + for<'de> Deserialize<'de>> Configurable for AppConfig<T> {
  fn save(&self) -> anyhow::Result<()> {
    let mut writer = BufWriter::new(File::create(&self.file_path)?);

    let serialized = toml::to_string(&self.config)?;
    writer.write_all(serialized.as_bytes())?;

    Ok(())
  }

  fn load(&mut self) -> anyhow::Result<()> {
    let mut reader = BufReader::new(File::open(&self.file_path)?);

    let content = {
      let mut buf = String::new();
      reader.read_to_string(&mut buf)?;
      buf
    };
    let deserialized = toml::from_str::<T>(&content)?;
    self.config = deserialized;

    Ok(())
  }

  fn get(&self, _key: &str) -> &Item {
    todo!();
  }
  // fn set<K: AsRef<str>, V: Into<Item>>(&self, key: K, val: V) -> anyhow::Result<()> {
  fn get_mut(&self, _key: &str) -> &mut Item {
    todo!();
  }
}

impl<T: for<'de> Deserialize<'de> + Serialize> Deref for AppConfig<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.config
  }
}
