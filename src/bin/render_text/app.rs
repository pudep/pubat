use std::path::PathBuf;

use crate::prelude::normal::*;
use crate::render::render_line::render;
use crate::state::ViewPort;

pub fn app(
  terminal: &mut DefaultTerminal,
  rope: &Rope,
  viewport: &mut ViewPort,
  path: &PathBuf,
) -> std::io::Result<()> {
  loop {
    terminal.draw(|frame| render(frame, rope, viewport, path))?;

    let viewport_height = terminal.size()?.height as usize;
    let file_fits_in_viewport = viewport_height >= rope.lines().len();

    if let Event::Key(key) = crossterm::event::read()? {
      match key.code {
        KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => break Ok(()),
        KeyCode::Up if !file_fits_in_viewport => {
          viewport.clamp_negative(1_usize);
        }
        KeyCode::Down if !file_fits_in_viewport => {
          viewport.clamp_positive(1_usize);
        }
        _ => {}
      }
    }
  }
}
