// TODO: serde_jsonのようなValueを定義する
// use serde_json::Value;

pub trait TryToHashMap {
  // fn try_to_hashmap(&self) -> Result<HashMap<&str, Value>, Error> {
  //   Ok(hashmap!(self)?)
  // }
  fn try_to_hashmap(&self) -> Result<(), ()> {
    println!("try");
    Ok(())
  }
}
