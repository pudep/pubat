mod app;
mod key;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  app::init::init()?;
  Ok(())
}
