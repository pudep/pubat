use std::sync::LazyLock;

use ratatui::style::{Color, Modifier, Style as RStyle};
use ratatui::text::{Line, Span};
use std::io::Cursor;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Color as SynColor, FontStyle, Style as SynStyle, Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME: LazyLock<Theme> = LazyLock::new(load_moon_theme);

// The tmTheme's global background, resolved once. Falls back to a sane
// default if the theme somehow doesn't define one.
static BG_COLOR: LazyLock<Color> = LazyLock::new(|| {
  THEME
    .settings
    .background
    .map(|SynColor { r, g, b, .. }| Color::Rgb(r, g, b))
    .unwrap_or(Color::Rgb(34, 36, 54)) // tokyonight moon bg fallback
});

const MOON_THEME_BYTES: &[u8] = include_bytes!("../tokyonight_moon.tmTheme");

fn load_moon_theme() -> Theme {
  ThemeSet::load_from_reader(&mut Cursor::new(MOON_THEME_BYTES))
    .expect("bundled tokyonight_moon.tmTheme should always parse")
}

/// Public getter so the render layer can paint the pane background
/// (e.g. via `Block::default().style(Style::default().bg(highlight::bg_color())))`.
pub fn bg_color() -> Color {
  *BG_COLOR
}

fn resolve_syntax<'a>(extension: &str, first_line: &str) -> &'a SyntaxReference {
  SYNTAX_SET
    .find_syntax_by_extension(extension)
    .or_else(|| SYNTAX_SET.find_syntax_by_first_line(first_line))
    .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text())
}

pub fn highlight_file(source: &str, extension: &str) -> Vec<Line<'static>> {
  let first_line = source.lines().next().unwrap_or("");
  let syntax = resolve_syntax(extension, first_line);
  let mut h = HighlightLines::new(syntax, &THEME);

  LinesWithEndings::from(source)
    .map(|line| {
      let spans: Vec<Span<'static>> = match h.highlight_line(line, &SYNTAX_SET) {
        Ok(ranges) => ranges.into_iter().map(|(s, t)| to_span(s, t)).collect(),
        Err(_) => vec![Span::styled(
          line.trim_end_matches(['\n', '\r']).to_string(),
          RStyle::default().bg(bg_color()),
        )],
      };
      Line::from(spans)
    })
    .collect()
}

fn to_span(style: SynStyle, text: &str) -> Span<'static> {
  let fg = style.foreground;
  let mut rstyle = RStyle::default()
    .fg(Color::Rgb(fg.r, fg.g, fg.b))
    .bg(bg_color());

  if style.font_style.contains(FontStyle::BOLD) {
    rstyle = rstyle.add_modifier(Modifier::BOLD);
  }
  if style.font_style.contains(FontStyle::UNDERLINE) {
    rstyle = rstyle.add_modifier(Modifier::UNDERLINED);
  }
  // Italics intentionally not applied — many terminals render tmTheme
  // italics badly (or as reverse-video), so we skip Modifier::ITALIC.

  let text = text.trim_end_matches(['\n', '\r']);
  Span::styled(text.to_string(), rstyle)
}
