use crate::app_state::*;
use crate::prelude::normal::*;
use crate::wrap::util::*;
pub fn render(frame: &mut Frame, rope: &Rope, viewport: &mut ViewPort) {
  let mut content_memory = String::new();
  let terminal_area = frame.area();

  for (rope_idx, line) in rope.lines().skip(viewport.scroll_offset).enumerate() {
    let content = line.to_string();
    let wrapped_content_line = smart_soft_wrap(rope_idx as u16,terminal_area.width, &content, rope, viewport);
    content_memory.push_str(&wrapped_content_line);
  }

  frame.render_widget(Paragraph::new(content_memory), terminal_area);
}
