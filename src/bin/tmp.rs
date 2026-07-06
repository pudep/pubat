use ratatui::{
  DefaultTerminal, Frame,
  crossterm::event::{self, Event, KeyCode, KeyEventKind},
  layout::{Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, Borders, Paragraph},
};
use std::io;

struct App {
  lines: Vec<String>,
  cursor: usize,
  top: usize,
  scrolloff: usize,
}

impl App {
  fn new(lines: Vec<String>) -> Self {
    Self {
      lines,
      cursor: 0,
      top: 0,
      scrolloff: 4,
    }
  }

  fn len(&self) -> usize {
    self.lines.len()
  }

  fn move_cursor(&mut self, delta: isize) {
    let last = self.len() as isize - 1;
    self.cursor = (self.cursor as isize + delta).clamp(0, last.max(0)) as usize;
  }

  fn goto_top(&mut self) {
    self.cursor = 0;
  }

  fn goto_bottom(&mut self) {
    self.cursor = self.len().saturating_sub(1);
  }

  /// Reconcile `top` against `cursor`, honoring `scrolloff`, for a given
  /// viewport height (number of visible text lines, borders excluded).
  ///
  /// Invariant we try to maintain (when the buffer is long enough):
  ///   top + scrolloff <= cursor <= top + viewport_height - 1 - scrolloff
  fn adjust_scroll(&mut self, viewport_height: usize) {
    if viewport_height == 0 {
      return;
    }
    let len = self.len();

    // Shrink scrolloff automatically for tiny viewports/buffers, the
    // same way Vim does rather than deadlocking the cursor in place.
    let max_scrolloff = viewport_height.saturating_sub(1) / 2;
    let so = self.scrolloff.min(max_scrolloff);

    // Cursor too close to the top edge -> pull the window up.
    if self.cursor < self.top + so {
      self.top = self.cursor.saturating_sub(so);
    }

    // Cursor too close to the bottom edge -> push the window down.
    if self.cursor + so + 1 > self.top + viewport_height {
      self.top = self.cursor + so + 1 - viewport_height;
    }

    // Never scroll past the point where the last line would leave a
    // ragged gap at the bottom of the viewport.
    let max_top = len.saturating_sub(viewport_height);
    self.top = self.top.min(max_top);
  }
}

fn main() -> io::Result<()> {
  let lines: Vec<String> = (1..=80)
    .map(|n| format!("This is line number {n} of the fixed buffer."))
    .collect();
  let mut app = App::new(lines);

  let terminal = ratatui::init();
  let result = run(terminal, &mut app);
  ratatui::restore();
  result
}

fn run(mut terminal: DefaultTerminal, app: &mut App) -> io::Result<()> {
  loop {
    terminal.draw(|f| ui(f, app))?;

    if let Event::Key(key) = event::read()? {
      // Ignore key-release events (some terminals report both).
      if key.kind != KeyEventKind::Press {
        continue;
      }
      match key.code {
        KeyCode::Char('q') | KeyCode::Esc => break,
        KeyCode::Down | KeyCode::Char('j') => app.move_cursor(1),
        KeyCode::Up | KeyCode::Char('k') => app.move_cursor(-1),
        KeyCode::Char('g') => app.goto_top(),
        KeyCode::Char('G') => app.goto_bottom(),
        KeyCode::Char('+') => app.scrolloff = app.scrolloff.saturating_add(1),
        KeyCode::Char('-') => app.scrolloff = app.scrolloff.saturating_sub(1),
        _ => {}
      }
    }
  }
  Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
  let size = f.area();

  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Min(3),    // buffer view
      Constraint::Length(1), // status line
      Constraint::Length(1), // help line
    ])
    .split(size);

  let text_area: Rect = chunks[0];
  // Inner height available for text lines (borders take 2 rows).
  let viewport_height = text_area.height.saturating_sub(2).max(1) as usize;

  // This is the core step: recompute `top` from `cursor` + `scrolloff`
  // every frame, using the *actual* rendered viewport height.
  app.adjust_scroll(viewport_height);

  let end = (app.top + viewport_height).min(app.len());
  let num_width = app.len().to_string().len().max(2);

  let lines: Vec<Line> = (app.top..end)
    .map(|i| {
      let is_cursor = i == app.cursor;

      // Hybrid line numbers: absolute number on the cursor line,
      // relative distance elsewhere (like `relativenumber` + `number`).
      let gutter = if is_cursor {
        format!("{:>width$}", i + 1, width = num_width)
      } else {
        let dist = i.abs_diff(app.cursor);
        format!("{:>width$}", dist, width = num_width)
      };

      let gutter_style = if is_cursor {
        Style::default()
          .fg(Color::Yellow)
          .add_modifier(Modifier::BOLD)
      } else {
        Style::default().fg(Color::DarkGray)
      };

      let text_style = if is_cursor {
        Style::default()
          .fg(Color::Black)
          .bg(Color::Yellow)
          .add_modifier(Modifier::BOLD)
      } else {
        Style::default()
      };

      Line::from(vec![
        Span::styled(format!(" {gutter} \u{2502} "), gutter_style),
        Span::styled(app.lines[i].clone(), text_style),
      ])
    })
    .collect();

  let title = format!(
    " buffer  [{}/{}]  top={}  scrolloff={} ",
    app.cursor + 1,
    app.len(),
    app.top,
    app.scrolloff
  );

  let paragraph = Paragraph::new(lines).block(
    Block::default()
      .borders(Borders::ALL)
      .title(title)
      .border_style(Style::default().fg(Color::Cyan)),
  );
  f.render_widget(paragraph, text_area);

  let status = Line::from(vec![
    Span::styled(
      " NORMAL ",
      Style::default()
        .bg(Color::Blue)
        .fg(Color::Black)
        .add_modifier(Modifier::BOLD),
    ),
    Span::raw(format!(" line {} of {}", app.cursor + 1, app.len())),
  ]);
  f.render_widget(Paragraph::new(status), chunks[1]);

  let help = Paragraph::new(Line::from(
    "j/\u{2193} down   k/\u{2191} up   g top   G bottom   +/- scrolloff   q quit",
  ))
  .style(Style::default().fg(Color::DarkGray));
  f.render_widget(help, chunks[2]);
}
