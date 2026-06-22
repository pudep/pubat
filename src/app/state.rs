use crossterm::cursor;
use ropey::Rope;

struct Buffer {
  bufdata: Rope,
  cursor: (u16, u16),
}

impl Buffer {
  fn new(&mut self) -> Self {
    Buffer {
      bufdata: Rope::new(),
      cursor: (0, 0),
    }
  }

  fn reader(&mut self, string: &str) -> &mut Self {
    self.bufdata = Rope::from_str(string);
    self
  }

  fn get_cursor_pos(&mut self) -> &mut Self {
    let (col, row) = cursor::position().unwrap();
    self.cursor = (col, row);
    self
  }
}
