#[test]
fn run() {
  use crate::TryToHashMap;
  #[derive(TryToHashMap)]
  struct TestStruct<'a> {
    one: u32,
    two: i32,
    three: f64,
    four: bool,
    five: &'a str,
  }
}
