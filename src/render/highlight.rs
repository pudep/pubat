// This is ai generated for this project

use std::sync::LazyLock;

use ratatui::style::{Color, Modifier, Style as RStyle};
use ratatui::text::{Line, Span};
use std::io::Cursor;
use syntect::easy::HighlightLines;
use syntect::highlighting::{FontStyle, Style as SynStyle, Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

// Loaded once, reused for the lifetime of the program.
static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME: LazyLock<Theme> = LazyLock::new(load_moon_theme);

const MOON_THEME_BYTES: &[u8] = include_bytes!("../tokyonight_moon.tmTheme");

fn load_moon_theme() -> Theme {
  ThemeSet::load_from_reader(&mut Cursor::new(MOON_THEME_BYTES))
    .expect("bundled tokyonight_moon.tmTheme should always parse")
}

/// Pick the best syntax for a file: try extension, then filename, then
/// first-line heuristics (shebangs, XML declarations, etc.), else plain text.
fn resolve_syntax<'a>(extension: &str, first_line: &str) -> &'a SyntaxReference {
  SYNTAX_SET
    .find_syntax_by_extension(extension)
    .or_else(|| SYNTAX_SET.find_syntax_by_first_line(first_line))
    .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text())
}

/// Highlight an entire source string, returning owned, render-ready lines.
/// Call this once per file load/reload — not per frame or per scroll —
/// and cache the result.
pub fn highlight_file(source: &str, extension: &str) -> Vec<Line<'static>> {
  let first_line = source.lines().next().unwrap_or("");
  let syntax = resolve_syntax(extension, first_line);
  let mut h = HighlightLines::new(syntax, &THEME);

  LinesWithEndings::from(source)
    .map(|line| {
      let spans: Vec<Span<'static>> = match h.highlight_line(line, &SYNTAX_SET) {
        Ok(ranges) => ranges.into_iter().map(|(s, t)| to_span(s, t)).collect(),
        // Malformed input shouldn't crash a file viewer — fall back
        // to an unstyled span for this line.
        Err(_) => vec![Span::raw(line.trim_end_matches(['\n', '\r']).to_string())],
      };
      Line::from(spans)
    })
    .collect()
}

fn to_span(style: SynStyle, text: &str) -> Span<'static> {
  let fg = style.foreground;
  let mut rstyle = RStyle::default().fg(Color::Rgb(fg.r, fg.g, fg.b));

  if style.font_style.contains(FontStyle::BOLD) {
    rstyle = rstyle.add_modifier(Modifier::BOLD);
  }
  if style.font_style.contains(FontStyle::ITALIC) {
    rstyle = rstyle.add_modifier(Modifier::ITALIC);
  }
  if style.font_style.contains(FontStyle::UNDERLINE) {
    rstyle = rstyle.add_modifier(Modifier::UNDERLINED);
  }

  // Strip the trailing newline captured by LinesWithEndings so ratatui
  // doesn't render an embedded line break inside a single Line/Span.
  let text = text.trim_end_matches(['\n', '\r']);
  Span::styled(text.to_string(), rstyle)
}
