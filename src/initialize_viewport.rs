use crate::state::ViewPort;
use ropey::Rope;

pub fn init(viewport: &mut ViewPort, rope: &Rope) {
  viewport.total_lines = (rope.lines().len()) as u32;
}
