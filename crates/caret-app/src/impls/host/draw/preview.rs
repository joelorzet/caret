//! The file preview, including its syntax colouring.
//!
//! Part of the drawing layer: one region of the window per file, so a
//! change to one of them is a change to one file.

#[allow(unused_imports)]
use super::*;

impl Session {
    /// The file preview: numbered, coloured, and selectable.
    pub(super) fn draw_preview(&self, surface: &mut Surface) {
        let Some(preview) = self.preview.as_ref() else {
            return;
        };
        let ui = &self.theme.ui;

        let rect = self.preview_rect();

        surface.fill(rect, self.theme.bg);
        draw_border(surface, rect, ui.border_focused, self.theme.bg);

        let name = preview
            .path()
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();

        let mut x = rect.x + 2;
        surface.text(
            x,
            rect.y,
            rect.w.saturating_sub(4),
            &name,
            Style::new(ui.accent, self.theme.bg).bold(),
        );
        x += name.chars().count() as u16 + 2;

        let detail = match preview.error() {
            Some(error) => error.to_string(),
            None => format!(
                "{}   {} lines{}",
                preview.language().name(),
                preview.len(),
                if preview.truncated() {
                    "   truncated"
                } else {
                    ""
                }
            ),
        };
        surface.text(
            x,
            rect.y,
            rect.right().saturating_sub(x + 2),
            &detail,
            Style::new(ui.tab_inactive_fg, self.theme.bg),
        );

        // While lines are selected, say how many: the highlight alone does not
        // tell you at a glance.
        if preview.is_selecting() {
            let (first, last) = preview.selection();
            let note = format!(
                "lines {}-{}  {} selected",
                first + 1,
                last + 1,
                preview.selected_lines()
            );
            let width = note.chars().count() as u16 + 2;
            if rect.w > width + 4 {
                surface.text(
                    rect.right().saturating_sub(width),
                    rect.y,
                    width,
                    &note,
                    Style::new(ui.accent, self.theme.bg).bold(),
                );
            }
        }

        // A gutter wide enough for the largest line number keeps the text
        // from shifting as it scrolls.
        let gutter = preview.len().to_string().chars().count().max(2) as u16 + 1;
        let (first, last) = preview.selection();

        for (row, (index, line, tokens)) in preview.visible().enumerate() {
            let y = rect.y + 1 + row as u16;
            if y >= rect.bottom() - 1 {
                break;
            }

            let selected = preview.is_selecting() && index >= first && index <= last;
            let on_cursor = index == preview.cursor();
            let bg = if selected {
                ui.selection_ui
            } else {
                self.theme.bg
            };
            if selected {
                surface.fill_rounded(Rect::new(rect.x + 1, y, rect.w - 2, 1), bg, ui.panel_bg);
            }

            surface.put(
                rect.x + 1,
                y,
                if on_cursor { '\u{258e}' } else { ' ' },
                Style {
                    fg: ui.accent,
                    bg,
                    flags: Flags::NONE,
                },
            );
            surface.text(
                rect.x + 2,
                y,
                gutter,
                &format!("{:>width$}", index + 1, width = gutter as usize),
                Style {
                    fg: ui.tab_inactive_fg,
                    bg,
                    flags: Flags::NONE,
                },
            );

            let text_x = rect.x + 3 + gutter;
            let room = rect.right().saturating_sub(text_x + 1);
            self.draw_code(surface, (text_x, y), room, line, tokens, bg);
        }

        let hints: &[(&str, &str)] = if preview.error().is_some() {
            &[("q", "close")]
        } else {
            &[
                ("j/k", "move"),
                ("shift+arrows", "select"),
                ("y", "copy"),
                ("e", "edit"),
                ("q", "close"),
            ]
        };
        self.draw_hints(
            surface,
            (rect.x + 2, rect.bottom() - 1),
            rect.w.saturating_sub(4),
            hints,
            self.theme.bg,
        );
    }

    /// One line of source, coloured by its tokens.
    fn draw_code(
        &self,
        surface: &mut Surface,
        at: (u16, u16),
        width: u16,
        line: &str,
        tokens: &[Token],
        bg: Rgb,
    ) {
        let (x, y) = at;
        let syntax = &self.syntax;
        for (column, ch) in line.chars().take(width as usize).enumerate() {
            let kind = tokens
                .iter()
                .find(|token| column >= token.start && column < token.end)
                .map(|token| token.kind);

            // Weight carries the distinction too, which matters most on a
            // monochrome theme where the colours sit close together.
            let (fg, flags) = match kind {
                Some(TokenKind::Comment) => (syntax.comment, Flags::NONE),
                Some(TokenKind::Text) => (syntax.string, Flags::NONE),
                Some(TokenKind::Number) => (syntax.number, Flags::NONE),
                Some(TokenKind::Keyword) => (syntax.keyword, Flags::BOLD),
                Some(TokenKind::Type) => (syntax.type_name, Flags::BOLD),
                Some(TokenKind::Function) => (syntax.function, Flags::BOLD),
                Some(TokenKind::Property) => (syntax.property, Flags::NONE),
                Some(TokenKind::Plain) | None => (syntax.plain, Flags::NONE),
            };
            surface.put(x + column as u16, y, ch, Style { fg, bg, flags });
        }
    }
}
