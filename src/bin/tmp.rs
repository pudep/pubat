use crossterm::event::{self, Event, KeyCode};
use ratatui::{
  Terminal,
  style::{Color, Style},
  text::{Line, Span},
  widgets::Paragraph,
};

fn main() -> std::io::Result<()> {
  let mut terminal = ratatui::init();
  let lines = vec!["Line 1", "Line 2", "Line 3", "Line 4", "Line 5"];
  let mut cursor: usize = 0; // index into lines

  loop {
    terminal.draw(|frame| {
      let rendered: Vec<Line> = lines
        .iter()
        .enumerate()
        .map(|(i, text)| {
          let style = if i == cursor {
            Style::default().bg(Color::Rgb(50, 50, 50))
          } else {
            Style::default()
          };
          Line::from(Span::styled(*text, style))
        })
        .collect();

      frame.render_widget(Paragraph::new(rendered), frame.area());
    })?;

    if let Event::Key(key) = event::read()? {
      match key.code {
        KeyCode::Up => cursor = cursor.saturating_sub(1),
        KeyCode::Down => cursor = (cursor + 1).min(lines.len() - 1),
        KeyCode::Char('q') => break,
        _ => {}
      }
    }
  }

  ratatui::restore();
  Ok(())
}
