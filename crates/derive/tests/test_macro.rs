use derive::{hashmap, HashMap};
use derive_util::Input;
use quote::quote;
use serde::Serialize;
use syn::parse2;

// #[test]
// fn test_1() {
//   #[derive(Debug, HashMap, Serialize)]
//   struct TestStruct<'a> {
//     one: u32,
//     two: i32,
//     three: f64,
//     four: bool,
//     five: &'a str,
//   }
//   let val = TestStruct {
//     one: 36354,
//     two: -24535,
//     three: 48476.805,
//     four: true,
//     five: "test text",
//   };
//   dbg!(&val);
//   val.try_to_hashmap().unwrap();
//   // println!("{}", );
// }

#[test]
fn test_2() {
  struct TestStruct<'a> {
    one: u32,
    two: i32,
    three: f64,
    four: bool,
    five: &'a str,
  }
  let val = TestStruct {
    one: 36354,
    two: -24535,
    three: 48476.805,
    four: true,
    five: "test text",
  };
  // hashmap!(val);
}

#[test]
fn test_3() {
  struct TestStruct<'a> {
    one: u32,
    two: i32,
    three: f64,
    four: bool,
    five: &'a str,
  }
  let val = TestStruct {
    one: 36354,
    two: -24535,
    three: 48476.805,
    four: true,
    five: "test text",
  };

  let token = quote! {
    val
  };
  println!("aaaaaaaaa {:?}", token);
  let res = parse2::<Input>(token).unwrap();
}
