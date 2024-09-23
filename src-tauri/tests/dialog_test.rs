use tauri::{test::mock_app, WebviewUrl, WebviewWindowBuilder};

#[test]
fn dialog() -> anyhow::Result<()> {
  let app = mock_app();
  WebviewWindowBuilder::new(&app, "test", WebviewUrl::default()).build()?;
  Ok(())
}
