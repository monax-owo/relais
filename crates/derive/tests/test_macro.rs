use derive::TryToHashMap;

#[test]
fn run() {
  #[derive(Debug, TryToHashMap)]
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
  dbg!(val);
  // println!("{}", val.try_to_hashmap());
}
