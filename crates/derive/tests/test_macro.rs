use derive::ToHashMap;

#[test]
fn test_1() {
  #[derive(Debug, ToHashMap)]
  // #[derive(Debug)]
  struct TestStruct {
    one: u32,
    two: i32,
    three: f64,
    four: bool,
    five: &'static str,
  }

  let val = TestStruct {
    one: 36354,
    two: -24535,
    three: 48476.805,
    four: true,
    five: "test text",
  };
  dbg!(&val);
  println!("{:?}", val.try_to_hashmap());
}
