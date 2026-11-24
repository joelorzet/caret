//! The command, theme and settings list.
//!
//! Part of the drawing layer: one region of the window per file, so a
//! change to one of them is a change to one file.

#[allow(unused_imports)]
use super::*;

impl Session {
    /// The command palette.
    pub(super) fn draw_palette(&self, surface: &mut Surface) {
        let Some(palette) = self.palette.as_ref() else {
            return;
        };
        let ui = &self.theme.ui;

        let Some(rect) = self.palette_rect() else {
            return;
        };

        surface.fill(rect, ui.panel_bg);
        draw_border(surface, rect, ui.border_focused, ui.panel_bg);
        surface.text(
            rect.x + 2,
            rect.y,
            rect.w.saturating_sub(4),
            palette.title(),
            Style::new(ui.accent, ui.panel_bg).bold(),
        );

        // Query line, with a caret so it reads as something to type into.
        surface.text(
            rect.x + 2,
            rect.y + 1,
            rect.w.saturating_sub(4),
            "\u{203a}",
            Style::new(ui.accent, ui.panel_bg),
        );
        if let Some((_, label)) = &palette.capturing {
            // The list stays up behind it, so what is being rebound is still
            // on screen while the key is pressed.
            surface.text(
                rect.x + 4,
                rect.y + 1,
                rect.w.saturating_sub(6),
                &format!("press the new shortcut for {label}, esc to cancel"),
                Style::new(ui.accent, ui.panel_bg).bold(),
            );
        } else if palette.query.is_empty() {
            // An empty line beside a caret looks like a thing that is not
            // working. Saying what typing does costs nothing.
            surface.text(
                rect.x + 4,
                rect.y + 1,
                rect.w.saturating_sub(6),
                palette.placeholder(),
                Style::new(ui.tab_inactive_fg, ui.panel_bg),
            );
        } else {
            surface.text(
                rect.x + 4,
                rect.y + 1,
                rect.w.saturating_sub(6),
                &palette.query,
                Style::new(ui.tab_active_fg, ui.panel_bg),
            );
        }
        for x in rect.x + 1..rect.right() - 1 {
            surface.put(
                x,
                rect.y + 2,
                '\u{2500}',
                Style::new(ui.border, ui.panel_bg),
            );
        }

        if palette.rows.is_empty() {
            surface.text(
                rect.x + 2,
                rect.y + 3,
                rect.w.saturating_sub(4),
                "no matches",
                Style::new(ui.tab_inactive_fg, ui.panel_bg),
            );
        }

        let hint_width = (rect.w / 3) as usize;
        for (row, line) in palette.visible() {
            let y = rect.y + Self::LIST_TOP + (row - palette.offset) as u16;
            if y >= rect.bottom() - 1 {
                break;
            }

            // The topic, said once above the entries it covers.
            let Some(item) = palette.item(line) else {
                if let PaletteRow::Heading(topic, entries) = line {
                    surface.text(
                        rect.x + 2,
                        y,
                        rect.w.saturating_sub(4),
                        topic,
                        Style::new(ui.accent, ui.panel_bg).bold(),
                    );
                    // How many are under it, so a collapsed-looking group is
                    // obviously short rather than obviously cut off.
                    let count = entries.to_string();
                    let at = rect
                        .right()
                        .saturating_sub(2 + count.chars().count() as u16);
                    surface.text(
                        at,
                        y,
                        rect.right() - at,
                        &count,
                        Style::new(ui.tab_inactive_fg, ui.panel_bg),
                    );
                }
                continue;
            };

            let chosen = row == palette.selected;
            let bg = if chosen { ui.selection_ui } else { ui.panel_bg };
            if chosen {
                surface.fill_rounded(Rect::new(rect.x + 1, y, rect.w - 2, 1), bg, ui.panel_bg);
            }

            surface.put(
                rect.x + 1,
                y,
                if chosen { '\u{258e}' } else { ' ' },
                Style {
                    fg: ui.accent,
                    bg,
                    flags: Flags::NONE,
                },
            );

            // Entries sit in from their heading, so the grouping reads down
            // the left edge without a word being repeated on every line.
            let label_x = rect.x + 2 + ENTRY_INDENT;
            let label_width = rect.w.saturating_sub(3 + ENTRY_INDENT + hint_width as u16);
            surface.text_over(
                label_x,
                y,
                label_width,
                &truncate(&item.label, label_width as usize),
                Style {
                    fg: if chosen {
                        ui.tab_active_fg
                    } else {
                        ui.chrome_fg
                    },
                    bg,
                    flags: if chosen { Flags::BOLD } else { Flags::NONE },
                },
            );

            if !item.hint.is_empty() && hint_width > 2 {
                let hint = truncate(&item.hint, hint_width - 1);
                let at = rect.right().saturating_sub(1 + hint.chars().count() as u16);
                surface.text_over(
                    at,
                    y,
                    rect.right() - at,
                    &hint,
                    Style {
                        fg: ui.tab_inactive_fg,
                        bg,
                        flags: Flags::NONE,
                    },
                );
            }
        }

        self.draw_hints(
            surface,
            (rect.x + 2, rect.bottom() - 1),
            rect.w.saturating_sub(4),
            palette.hints(),
            ui.panel_bg,
        );
    }

    pub(crate) fn reachable(&self, keys: &str) -> bool {
        self.native_window || !keys.contains("cmd+")
    }
}

/// How far an entry sits in from the heading above it.
const ENTRY_INDENT: u16 = 2;

/// Rows the palette spends on its border, query line, rule and footer.
pub(crate) const PALETTE_CHROME: u16 = 4;

pub(crate) const MIN_PALETTE_WIDTH: u16 = 44;

pub(crate) const MAX_PALETTE_WIDTH: u16 = 98;
