pub mod buffer;
pub struct ViewPort {
  pub scroll_offset: usize,
  pub total_lines: u32,
}

impl ViewPort {
  /// This will initialize a new viewport
  /// Call only once per project.
  pub fn new() -> Self {
    ViewPort {
      scroll_offset: 0_usize,
      total_lines: 0,
    }
  }

  /// Use this for KeyCode::Up => {}
  /// It will auto clamp at Line 1.
  pub fn clamp_negative(&mut self, b: usize) {
    let result = self.scroll_offset.saturating_sub(b);
    self.scroll_offset = result;
  }

  /// Use this for KeyCode::Down => {}
  /// It needs a max boundary to clamp at.
  pub fn clamp_positive(&mut self, add: usize) {
    // let area = self.area;

    // if self.scroll_offset < 

    let result = self
      .scroll_offset
      .saturating_add(add).clamp(self.scroll_offset, self.total_lines.saturating_sub(1) as usize);
    self.scroll_offset = result;
  }
}
