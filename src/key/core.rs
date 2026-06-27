use crate::prelude::cterm::all::*;
use crate::prelude::std::all::*;
pub fn key_pressed(cursor_pos: &mut (u16, u16)) -> Result<bool, Box<dyn Error>> {
  if let Event::Key(key) = crossterm::event::read()? {
    match key.code {
      KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => return Ok(true),
      KeyCode::Up => {
        cursor_pos.1 = cursor_pos.1.saturating_sub(1);
      }
      KeyCode::Down => {
        cursor_pos.1 += 1;
      }
      KeyCode::Left => {
        cursor_pos.0 = cursor_pos.0.saturating_sub(1);
      }
      KeyCode::Right => {
        cursor_pos.0 += 1;
      }
      _ => {}
    }
  }
  Ok(false)
}
