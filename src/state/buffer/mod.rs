use ropey::Rope;

pub mod io;

#[derive(Debug)]
pub struct Buffer {
  pub rope: Rope,
}
impl Default for Buffer {
  fn default() -> Self {
    Buffer { rope: Rope::new() }
  }
}

impl Buffer {
  pub fn new() -> Self {
    Self::default()
  }
}
