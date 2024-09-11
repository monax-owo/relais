use std::{
    fs::{create_dir, File},
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};

use anyhow::{bail, Context};
use toml_edit::DocumentMut;

pub mod default {
    pub fn key() -> String {
        String::from("teststest")
    }
}

#[derive(Debug)]
pub struct AppConfig {
    file_path: PathBuf,
    config: Mutex<DocumentMut>,
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
impl AppConfig {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let parent = path.parent().context("no parent")?;
        if !parent.exists() {
            create_dir(parent)?;
        }
        if !path.is_file() {
            bail!("path is not file")
        }

        let conf = Self {
            file_path: path.to_path_buf(),
            config: Mutex::new(DocumentMut::default()),
        };
        Self::load(&conf)?;

        Ok(conf)
    }
}

pub trait Configurable {
    /// selfの内容をファイルに書き込むメソッド
    fn save(&self) -> anyhow::Result<()>;
    /// ファイルの内容をselfに書き込むメソッド
    fn load(&self) -> anyhow::Result<()>;
    /// 値を代入する
    fn set(&self) -> anyhow::Result<()>;
    /// 値を取得する
    fn get(&self) -> anyhow::Result<()>;
}

impl Configurable for AppConfig {
    fn save(&self) -> anyhow::Result<()> {
        let mut writer = BufWriter::new(File::create(&self.file_path)?);
        let lock = self.config.lock().unwrap();

        let serialized = &*lock.to_string();
        writer.write_all(serialized.as_bytes())?;

        Ok(())
    }

    fn load(&self) -> anyhow::Result<()> {
        let mut reader = BufReader::new(File::open(&self.file_path)?);
        let mut lock = self.config.lock().unwrap();

        let content = {
            let mut buf = String::new();
            reader.read_to_string(&mut buf)?;
            buf
        };
        let deserialized = content.parse::<DocumentMut>()?;
        *lock = deserialized;

        Ok(())
    }

    // fn set<K: AsRef<str>, V: Into<Item>>(&self, key: K, val: V) -> anyhow::Result<()> {
    fn set(&self) -> anyhow::Result<()> {
        // let lock = self.config.lock().unwrap();
        // *lock
        Ok(())
    }

    fn get(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
