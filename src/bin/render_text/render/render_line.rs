use crate::prelude::normal::*;
use crate::wrap::util::*;
use crate::{app_state::*, initialize_viewport};
pub fn render(frame: &mut Frame, rope: &Rope, viewport: &mut ViewPort) {
  let mut content_memory = String::new();
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
      content_memory.push_str(&wrapped_content_line);
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
      content_memory.push_str(&wrapped_content_line);
    }
  }

  frame.render_widget(Paragraph::new(content_memory), terminal_area);
}
