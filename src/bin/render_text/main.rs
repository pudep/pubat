mod app;
mod app_state;
mod initialize_viewport;
mod prelude;
mod read;
mod render;
mod wrap;
use app::app;
use app_state::ViewPort;
use prelude::normal::*;
use read::read;

fn main() -> result::Result<(), Box<dyn Error>> {
  let buf = read()?;
  let mut viewport = ViewPort::new();
  initialize_viewport::init(&mut viewport, &buf.rope);
  ratatui::run(|terminal| app(terminal, &buf.rope, &mut viewport))?;
  Ok(())
}
