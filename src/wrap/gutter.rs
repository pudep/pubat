use ropey::Rope;
fn replace_char(to: char, times: u16) -> String {
  let mut times = times;
  let mut string = String::new();
  while times != 0 {
    string.push(to);
    times -= 1;
  }

  string
}

pub fn row_display(
  wrap_idx: usize,
  wrap_vec_len: usize,
  rope: &Rope,
  line: String,
) -> String {
  let last_line_num_width = rope.lines().len().to_string().len().max(1) as u16;
  let padding = replace_char(' ', last_line_num_width);

  if wrap_idx == 0 {
    format!("{padding}{line}\n")
  } else if wrap_idx < (wrap_vec_len - 1) {
    format!("{padding} {line}\n")
  } else {
    line.to_string()
  }
}
