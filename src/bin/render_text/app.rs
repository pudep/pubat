use crate::app_state::ViewPort;
use crate::prelude::normal::*;
use crate::render::render_line::render;
pub fn app(
  terminal: &mut DefaultTerminal,
  rope: &Rope,
  viewport: &mut ViewPort,
) -> std::io::Result<()> {
  loop {
    terminal.draw(|frame| render(frame, rope, viewport))?;
    if let Event::Key(key) = crossterm::event::read()? {
      match key.code {
        KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => break Ok(()),
        KeyCode::Up => {
          viewport.clamp_negative(1_usize);
        }
        KeyCode::Down => {
          viewport.clamp_positive(1_usize);
        }
        _ => {}
      }
    }
  }
}
