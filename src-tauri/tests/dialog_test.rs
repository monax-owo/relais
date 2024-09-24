use tauri::test::{mock_builder, mock_context, noop_assets};

// #[serial_test::serial]
// #[test]
fn _dialog_test() {
  mock_builder().build(mock_context(noop_assets())).unwrap();
}
