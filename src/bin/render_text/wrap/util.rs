use ropey::Rope;

use crate::{app_state::ViewPort, wrap::gutter};

pub fn smart_soft_wrap(
  rope_idx: u16,
  terminal_width: u16,
  content: &str,
  rope: &Rope,
  viewport: &mut ViewPort,
) -> String {
  let clamp_term_width = terminal_width.saturating_sub(5).max(3);
  let options = textwrap::Options::new(clamp_term_width as usize)
    .word_splitter(textwrap::WordSplitter::NoHyphenation)
    .word_separator(textwrap::WordSeparator::AsciiSpace);

  let wrapped_text_vec: Vec<&str> = textwrap::wrap(content, options)
    .into_iter()
    .map(|cow| match cow {
      std::borrow::Cow::Borrowed(s) => s,
      std::borrow::Cow::Owned(_) => unreachable!("The Cow::Owned text is unreachable! Take a look at : smart_soft_wrap()"),
    })
    .collect();

  let wrap_vector_len = wrapped_text_vec.len();

  let mut content_line = String::new();
  for (wrap_idx, line) in wrapped_text_vec.into_iter().enumerate() {
    content_line
      .push_str(&(gutter::row_display(rope_idx, wrap_idx, wrap_vector_len, rope, viewport, line)));
  }

  content_line
}
