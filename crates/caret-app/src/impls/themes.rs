use caret_core::Rgb;
use caret_theme::{Appearance, Theme};

const RESET: &str = "\x1b[0m";

fn bg(c: Rgb) -> String {
    format!("\x1b[48;2;{};{};{}m", c.r, c.g, c.b)
}

fn fg(c: Rgb) -> String {
    format!("\x1b[38;2;{};{};{}m", c.r, c.g, c.b)
}

/// Previews each theme using truecolor escapes, so the colors shown are the
/// ones the renderer will use.
pub fn render(themes: &[Theme]) -> String {
    let width = themes
        .iter()
        .map(|t| t.name.chars().count())
        .max()
        .unwrap_or(0);

    let mut out = format!("{} themes\n", themes.len());
    for theme in themes {
        let kind = match theme.appearance {
            Appearance::Dark => "dark ",
            Appearance::Light => "light",
        };

        out.push_str(&format!(
            "\n{:<width$}  {kind}  {:>4.1}:1  ",
            theme.name,
            theme.text_contrast(),
            width = width
        ));
        for c in theme.palette.iter() {
            out.push_str(&format!("{}  {RESET}", bg(c)));
        }
        out.push('\n');

        out.push_str(&format!(
            "{:<width$}  {}\n",
            "",
            theme.description,
            width = width
        ));
        out.push_str(&format!("{:<width$}  ", "", width = width));
        out.push_str(&sample(theme));
        out.push('\n');
    }
    out
}

/// A line of plausible shell output, drawn in the theme's own colors.
fn sample(theme: &Theme) -> String {
    let b = bg(theme.bg);
    let mut line = String::new();
    line.push_str(&b);
    line.push_str(&format!("{} ", fg(theme.ui.accent)));
    line.push_str(&format!("{}~/project ", fg(theme.indexed(4))));
    line.push_str(&format!("{}$ ", fg(theme.indexed(2))));
    line.push_str(&format!("{}cargo ", fg(theme.fg)));
    line.push_str(&format!("{}build ", fg(theme.indexed(3))));
    line.push_str(&format!("{}--release", fg(theme.indexed(6))));
    line.push_str(&format!("{}  error", fg(theme.indexed(1))));
    line.push_str(&format!("{}  warn", fg(theme.indexed(11))));
    line.push_str(&format!(
        "{}{} selected {RESET}",
        bg(theme.selection_bg),
        fg(theme.selection_fg)
    ));
    line
}
