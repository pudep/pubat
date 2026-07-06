mod cmd;
mod app;
mod state;
mod initialize_viewport;
mod prelude;
mod read;
mod render;
mod wrap;
use prelude::normal::*;

fn main() -> result::Result<(), Box<dyn Error>> {
  cmd::cli()?;
  Ok(())
}
