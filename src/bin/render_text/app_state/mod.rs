pub struct ViewPort {
  pub scroll_offset: usize,
}

impl ViewPort {
  pub fn new() -> Self {
    ViewPort { scroll_offset: 0_usize }
  }

  pub fn clamp_negative(&mut self, b: usize){
    let result = self.scroll_offset.saturating_sub(b);
    self.scroll_offset = result;
  }

  pub fn clamp_positive(&mut self, b: usize){
    let result = self.scroll_offset.saturating_add(b);
    self.scroll_offset = result;
  }
}
