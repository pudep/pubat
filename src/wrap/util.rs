use ropey::Rope;
use crate::{wrap::gutter};

pub fn smart_soft_wrap(
  terminal_width: u16,
  rope_string: &str,
  rope: &Rope,
) -> String {
  let clamp_term_width = terminal_width.saturating_sub(5).max(3);
  let options = textwrap::Options::new(clamp_term_width as usize)
    .word_splitter(textwrap::WordSplitter::NoHyphenation)
    .word_separator(textwrap::WordSeparator::AsciiSpace);

  let wrapped_text_vec: Vec<String> = textwrap::wrap(rope_string, options)
    .into_iter()
    .map(|cow| cow.into_owned())
    .collect();

  let wrap_vector_len = wrapped_text_vec.len();

  let mut unified_wrapped_string = String::new();
  for (wrap_idx, line) in wrapped_text_vec.into_iter().enumerate() {
    unified_wrapped_string
      .push_str(&(gutter::row_display( wrap_idx, wrap_vector_len, rope, line)));
  }

  unified_wrapped_string
}
