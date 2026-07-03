use ropey::Rope;
use crate::app_state::ViewPort;

pub fn init(viewport: &mut ViewPort, rope: &Rope){
  viewport.max_line = (rope.lines().len() + 1) as u32; 
}
