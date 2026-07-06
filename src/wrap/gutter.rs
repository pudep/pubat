pub fn format_string(wrap_idx: usize, line: String) -> String {
  if wrap_idx == 0 {
    format!("{line}\n")
  } else {
    line.to_string()
  }
}
