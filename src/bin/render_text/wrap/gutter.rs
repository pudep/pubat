use crate::app_state::ViewPort;
use crate::wrap::util::*;
use ropey::Rope;
fn push_char(char: char, times: u16) -> String {
  let mut times = times;
  let mut string = String::new();
  while times != 0 {
    string.push(char);
    times -= 1;
  }
  string
}
fn replace_char(to: char, times: u16) -> String{
  let mut times = times;
  let mut string = String::new();
  while times != 0 {
    string.push(to);
    times -= 1;
  }

  string
}

fn pop_dynamic(str: &str, to_replace_char_size: u8, vec_char: &[char]) -> String {
  let mut string = String::new();
  let mut num = to_replace_char_size;
  string.push_str(str);
  while num != 0 {
    string.pop();
    num -= 1;
  }

  let mut i = 0;
  while i != vec_char.len() {
    string.push(vec_char[i]);
    i += 1;
  }

  string
}
fn gutter(idx: usize, rope: &Rope, viewport: &mut ViewPort) {
  let gutter = push_char(' ', 1);
  let last_line_num_width = rope.lines().len().to_string().len().max(1) as u16;
  let padding = replace_char(' ', last_line_num_width);

  let cline: u8 = 1;
  let cline_len = cline.to_string().len() as u8;
  let vec_char: Vec<char> = cline.to_string().chars().collect();

  let dynamic_number_width = pop_dynamic(&padding, cline_len, &vec_char);

  let line_number = idx + viewport.scroll_offset + 1;
  let num_width = push_space_char(last_line_num_width as u16);

  let line = format!("{gutter}{dynamic_number_width}");
}
