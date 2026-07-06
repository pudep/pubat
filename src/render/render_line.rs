use crate::prelude::normal::*;
use crate::render::highlight;
use crate::wrap::util::*;
use crate::{initialize_viewport, state::*};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::Paragraph;
use std::path::Path;

pub fn render(frame: &mut Frame, rope: &Rope, viewport: &mut ViewPort, file_path: &Path) {
  let terminal_area = frame.area();
  initialize_viewport::init(viewport, rope);
  let total_lines = rope.len_lines();
  let gutter_width = total_lines.to_string().len() as u16 + 1;

  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Length(gutter_width), Constraint::Min(0)])
    .split(terminal_area);

  let mut text_content = String::new();
  let mut line_numbers = String::new();

  for (rope_idx, rope_line) in rope.lines().skip(viewport.scroll_offset).enumerate() {
    let rope_string = rope_line.to_string();
    let rope_string = rope_string.trim_end_matches(['\n', '\r']);
    let wrapped_text = smart_soft_wrap(chunks[1].width, rope_string);

    let visual_rows = wrapped_text.matches('\n').count().max(1);

    text_content.push_str(&wrapped_text);

    let line_num = rope_idx + viewport.scroll_offset + 1;
    line_numbers.push_str(&format!(
      "{:>width$}\n",
      line_num,
      width = (gutter_width - 1) as usize
    ));
    for _ in 1..visual_rows {
      line_numbers.push_str(&format!(
        "{:>width$}\n",
        "",
        width = (gutter_width - 1) as usize
      ));
    }
  }

  let ext = file_path
    .extension()
    .map(|e| e.to_string_lossy().to_string())
    .unwrap_or("txt".to_string());

  let highlighted_text = highlight::highlight_file(&text_content, &ext);

  frame.render_widget(
    Paragraph::new(line_numbers)
      .style(ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray)),
    chunks[0],
  );
  frame.render_widget(Paragraph::new(highlighted_text), chunks[1]);
}
