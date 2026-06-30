use std::{error::Error, result};

use crossterm::event::{Event, KeyCode, KeyModifiers};
use pude::state;
use ratatui::{DefaultTerminal, Frame, widgets::Paragraph};
use ropey::Rope;

struct ViewPort {
  scroll_offset: usize,
}

impl ViewPort {
  fn new() -> Self {
    ViewPort { scroll_offset: 0 }
  }
}

/// my read
fn read() -> result::Result<state::buffer::Buffer, Box<dyn Error>> {
  let mut buf = state::buffer::Buffer::new();
  let path = std::env::home_dir()
    .unwrap()
    .join("impl")
    .join("rust")
    .join("pude")
    .join("src")
    .join("txt.txt");
  buf.read(path)?;
  Ok(buf)
}

fn main() -> result::Result<(), Box<dyn Error>> {
  let buf = read()?;
  let mut viewport = ViewPort::new();
  ratatui::run(|terminal| app(terminal, &buf.rope, &mut viewport))?;
  Ok(())
}

fn app(
  terminal: &mut DefaultTerminal,
  rope: &Rope,
  viewport: &mut ViewPort,
) -> std::io::Result<()> {
  loop {
    terminal.draw(|frame| render(frame, rope, viewport))?;
    if let Event::Key(key) = crossterm::event::read()? {
      match key.code {
        KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => {
          break Ok(());
        }

        KeyCode::Up => {
          viewport.scroll_offset += 1;
        }
        KeyCode::Down => {
          viewport.scroll_offset += 0;
        }
        _ => {}
      }
    }
  }
}

fn render(frame: &mut Frame, rope: &Rope, viewport: &mut ViewPort) {
  let terminal_area = frame.area();
  let frame_height = terminal_area.height;
  let frame_width = terminal_area.width;

  let line_number_width = rope.lines().len().to_string().len().max(5);
  let gutter_padding = " ";
  let next_number_padding = " ";

  let mut content = String::new();
  for (idx, line) in rope.lines().skip(viewport.scroll_offset).enumerate() {
    let line = line.to_string();

    // idx == 0; scrolloffset say = 10 and current line is 11 therefore 0 + 10 + extra 1 == 11!
    let line_number = idx + viewport.scroll_offset + 1;
    let display_line = format!("{gutter_padding}{line_number}{next_number_padding}{line}");
    content.push_str(&display_line);
  }

  frame.render_widget(Paragraph::new(content), terminal_area);
}
