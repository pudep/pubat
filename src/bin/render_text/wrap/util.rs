use ropey::Rope;

use crate::{app_state::ViewPort, wrap::gutter};

pub fn smart_soft_wrap(
  rope_idx: u16,
  terminal_width: u16,
  content: &str,
  rope: &Rope,
  viewport: &mut ViewPort,
) -> String {
  let i = terminal_width.saturating_sub(5).max(3);
  let options = textwrap::Options::new(i as usize)
    .word_splitter(textwrap::WordSplitter::NoHyphenation)
    .word_separator(textwrap::WordSeparator::AsciiSpace);

  let wrapped_text: Vec<&str> = textwrap::wrap(content, options)
    .into_iter()
    .map(|cow| match cow {
      std::borrow::Cow::Borrowed(s) => s,
      std::borrow::Cow::Owned(_) => unreachable!("The text was unreachable"),
    })
    .collect();

  let mut content_line = String::new();
  for (wrapped_idx, line) in wrapped_text.into_iter().enumerate() {
    content_line.push_str(&(gutter::row_display(rope_idx,wrapped_idx, rope, viewport, line)));
  }

  content_line
}
