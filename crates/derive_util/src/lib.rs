// TODO: serde_jsonのようなValueを定義する-> std::any::Anyを使う

use syn::parse::Parse;

pub trait TryToHashMap {
  // fn try_to_hashmap(&self) -> Result<HashMap<&str, Value>, Error> {
  //   Ok(hashmap!(self)?)
  // }
  fn try_to_hashmap(&self) -> Result<(), ()>;
}

pub struct Input {
  // struct_name:
}

impl Parse for Input {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    dbg!(input);
    let result = Self {};
    Ok(result)
  }
}
