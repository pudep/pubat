use crate::prelude::cterm::all::*;
use crate::prelude::std::all::*;
pub fn key_pressed() -> Result<bool, Box<dyn Error>> {
  let mut stdout = stdout();
  if let Event::Key(key) = crossterm::event::read()? {
    match key.code {
      KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => {
        return Ok(true);
      }
      KeyCode::Backspace => {
        let (col, _) = cursor::position().unwrap();
        if col > 0 {
          execute!(stdout, cursor::MoveLeft(1), Print(' '), cursor::MoveLeft(1))?;
        }
      }
      KeyCode::Left => {
        execute!(stdout, cursor::MoveLeft(1))?;
      }
      KeyCode::Right => {
        execute!(stdout, cursor::MoveRight(1))?;
      }
      KeyCode::Enter => {
        execute!(stdout, Print("\r\n"))?;
      }
      KeyCode::Up => {
        execute!(stdout, cursor::MoveUp(1))?;
      }
      KeyCode::Down => {
        execute!(stdout, cursor::MoveDown(1))?;
      }
      KeyCode::Char(random_char) => {
        execute!(stdout, Print(random_char))?;
      }
      _ => {}
    }
  }
  Ok(false)
}
