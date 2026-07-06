mod app;
mod cmd;
mod initialize_viewport;
mod prelude;
mod read;
mod render;
mod state;
mod wrap;
use prelude::normal::*;

fn main() -> result::Result<(), Box<dyn Error>> {
  cmd::cli()?;
  Ok(())
}
