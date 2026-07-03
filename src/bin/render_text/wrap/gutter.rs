use crate::app_state::ViewPort;
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
fn replace_char(to: char, times: u16) -> String {
  let mut times = times;
  let mut string = String::new();
  while times != 0 {
    string.push(to);
    times -= 1;
  }

  string
}

fn pop_dynamic(padding: &str, to_replace_char_size: u16, vec_char: &[char]) -> String {
  let mut string = String::new();
  let mut num = to_replace_char_size;
  string.push_str(padding);
  while num != 0 {
    string.pop();
    num -= 1;
  }

  let mut i = 0;
  while i != vec_char.len() {
    string.push(vec_char[i]);
    i += 1;
  }

  // as per control flow it must push that
  // space shows after number of num line.
  string.push(' ');
  string
}
pub fn row_display(
  rope_idx: u16,
  wrap_idx: usize,
  rope: &Rope,
  viewport: &mut ViewPort,
  line: &str,
) -> String {
  let gutter = push_char(' ', 1);
  let last_line_num_width = rope.lines().len().to_string().len().max(1) as u16;
  let padding = replace_char(' ', last_line_num_width);

  let line_number = rope_idx + viewport.scroll_offset as u16 + 1;
  let vec_char: Vec<char> = line_number.to_string().chars().collect();
  let dynamic_number_width = pop_dynamic(&padding, line_number.to_string().len() as u16, &vec_char);

  if wrap_idx == 0 {
    format!("{gutter}{dynamic_number_width}{line}\n")
  } else {
    // read the pop_dynamic algo to know.
    // padding + a space
    // remeber the space must be outside the brace otherwise formatting will be affected.
    format!("{gutter}{padding} {line}\n")
  }
}
