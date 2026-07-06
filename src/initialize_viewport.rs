use crate::state::ViewPort;
use ratatui::Frame;
use ropey::Rope;

pub fn init(viewport: &mut ViewPort, rope: &Rope, frame: &mut Frame) {
  viewport.total_lines = (rope.lines().len()) as u32;
}
