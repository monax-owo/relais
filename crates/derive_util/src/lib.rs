use std::{any::Any, collections::HashMap};

pub trait TryToHashMap {
  fn try_to_hashmap(&self) -> HashMap<&str, &dyn Any>;
}
