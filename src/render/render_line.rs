use crate::prelude::normal::*;
use crate::render::highlight;
use crate::wrap::util::*;
use crate::{initialize_viewport, state::*};
use std::path::Path;
pub fn render(frame: &mut Frame, rope: &Rope, viewport: &mut ViewPort, file_path: &Path) {
  let mut line = String::new();
  let terminal_area = frame.area();
  initialize_viewport::init(viewport, rope, frame);

  if terminal_area.height as usize >= rope.lines().len() {
    for (rope_idx, rope_line) in rope.lines().enumerate() {
      let rope_string = rope_line.to_string();
      let wrapped_content_line = smart_soft_wrap(
        rope_idx as u16,
        terminal_area.width,
        &rope_string,
        rope,
        viewport,
      );
      line.push_str(&wrapped_content_line);
    }
  } else {
    for (rope_idx, rope_line) in rope.lines().skip(viewport.scroll_offset).enumerate() {
      let rope_string = rope_line.to_string();
      let wrapped_content_line = smart_soft_wrap(
        rope_idx as u16,
        terminal_area.width,
        &rope_string,
        rope,
        viewport,
      );
      line.push_str(&wrapped_content_line);
    }
  }

  let ext = if let Some(ext) = file_path.extension() {
    format!("{}", ext.to_string_lossy())
  } else {
    "txt".to_string()
  };

  let line = highlight::highlight_file(&line, &ext);
  frame.render_widget(Paragraph::new(line), terminal_area);
}
