use crate::app_state::*;
use crate::prelude::normal::*;
use crate::wrap::util::*;
pub fn render(frame: &mut Frame, rope: &Rope, viewport: &mut ViewPort) {
  let mut content = String::new();
  let terminal_area = frame.area();

  for (_, line) in rope.lines().skip(viewport.scroll_offset).enumerate() {
    let line = line.to_string();
    let wrapped_line = smart_soft_wrap(terminal_area.width, &line);
    content.push_str(&wrapped_line);
  }

  frame.render_widget(Paragraph::new(content), terminal_area);
}
