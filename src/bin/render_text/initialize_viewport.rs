use crate::app_state::ViewPort;
use ropey::Rope;

pub fn init(viewport: &mut ViewPort, rope: &Rope) {
  if rope.lines().len() != 0 {
    viewport.max_line = (rope.lines().len() - 1) as u32;
  } else if rope.lines().len() == 0 {
    viewport.max_line = (rope.lines().len()) as u32;
  }
}
