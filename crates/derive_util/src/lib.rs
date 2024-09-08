// TODO: serde_jsonのようなValueを定義する-> std::any::Anyを使う

use std::{any::Any, collections::HashMap};

pub trait TryToHashMap {
  fn try_to_hashmap(&self) -> HashMap<&str, &dyn Any>;
  // fn try_to_hashmap(&self) -> Result<(), ()>;
}
