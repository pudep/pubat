pub struct ViewPort {
  pub scroll_offset: usize,
  pub max_line: u32,
}

impl ViewPort {
  /// This will initialize a new viewport
  /// Call only once per project.
  pub fn new() -> Self {
    ViewPort {
      scroll_offset: 0_usize,
      max_line: 0,
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
    let result = self.scroll_offset.saturating_add(add).clamp(0_usize, self.max_line as usize);
    self.scroll_offset = result;
  }
}
