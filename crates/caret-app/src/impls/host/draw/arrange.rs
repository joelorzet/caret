//! The panel for moving panes about by hand.
//!
//! Part of the drawing layer: one region of the window per file, so a
//! change to one of them is a change to one file.

#[allow(unused_imports)]
use super::*;

impl Session {
    /// The arrange panel's frame.
    pub(crate) fn arrange_rect(&self) -> Rect {
        arrange_rect(self.size)
    }

    /// The part of the panel the layout is drawn into.
    ///
    /// The same rectangle the diagram is resolved against, so a click and a
    /// move land on the box they appear to.
    pub(crate) fn arrange_canvas(&self) -> Rect {
        arrange_canvas(arrange_rect(self.size))
    }

    /// The arrange panel: the layout as a diagram, in its own window.
    ///
    /// The diagram is resolved by the same engine that lays out the real
    /// tab, just into a smaller rectangle, so the boxes are the shape the
    /// panes would be. Each box carries the last few lines its terminal
    /// printed, which is what makes a numbered diagram recognisable as the
    /// thing you are actually running.
    pub(super) fn draw_arrange(&self, surface: &mut Surface) {
        let Some(state) = self.rearrange.as_ref() else {
            return;
        };
        let ui = &self.theme.ui;
        let rect = self.arrange_rect();
        if rect.w < ARRANGE_MIN_W || rect.h < ARRANGE_MIN_H {
            return;
        }

        surface.fill(rect, ui.panel_bg);
        draw_border(surface, rect, ui.border_focused, ui.panel_bg);
        surface.text(
            rect.x + 2,
            rect.y,
            rect.w.saturating_sub(4),
            " Arrange panes ",
            Style::new(ui.accent, ui.panel_bg).bold(),
        );

        let canvas = arrange_canvas(rect);
        let resolved = state.draft.resolve(canvas);
        let selected = state.draft.focus();

        for (index, (id, box_rect)) in resolved.panes.iter().enumerate() {
            // The pane's own number, so moving a box about does not renumber
            // it and the panel reads the same next time it is opened.
            let number = self.pane_number(*id).unwrap_or(index + 1);
            self.draw_arrange_box(surface, *id, *box_rect, number, *id == selected);
        }

        self.draw_hints(
            surface,
            (rect.x + 2, rect.bottom() - 2),
            rect.w.saturating_sub(4),
            // Accept and cancel come first: the hint row drops what it
            // cannot fit, and those two are the ones that must survive.
            &[
                ("enter", "apply"),
                ("esc", "cancel"),
                ("arrows", "select"),
                ("shift+arrows", "move"),
                ("t", "turn"),
                ("r", "rotate"),
                ("=", "even"),
            ],
            ui.panel_bg,
        );
    }

    fn draw_arrange_box(
        &self,
        surface: &mut Surface,
        id: PaneId,
        rect: Rect,
        number: usize,
        selected: bool,
    ) {
        let ui = &self.theme.ui;
        if rect.w < 3 || rect.h < 1 {
            return;
        }

        let bg = if selected {
            ui.selection_ui
        } else {
            self.theme.bg
        };
        surface.fill_rounded(rect, bg, ui.panel_bg);

        let label = format!(" {number} ");
        let width = label.chars().count() as u16;
        let (badge_fg, badge_bg) = if selected {
            (self.theme.cursor_text, ui.accent)
        } else {
            (ui.panel_fg, ui.panel_bg)
        };
        surface.fill_rounded(Rect::new(rect.x, rect.y, width, 1), badge_bg, bg);
        surface.text_over(
            rect.x,
            rect.y,
            width,
            &label,
            Style::new(badge_fg, badge_bg).bold(),
        );

        let name = self.pane_name(id);
        let room = rect.w.saturating_sub(width + 1);
        surface.text(
            rect.x + width + 1,
            rect.y,
            room,
            &truncate(&name, room as usize),
            Style::new(
                if selected {
                    ui.tab_active_fg
                } else {
                    ui.chrome_fg
                },
                bg,
            ),
        );

        // What the terminal last printed, so a box is recognisable as the
        // thing running in it rather than just a number.
        let rows = rect.h.saturating_sub(1) as usize;
        let body = rect.w.saturating_sub(2) as usize;
        let quiet = Style::new(ui.tab_inactive_fg.blend(bg, 0.25), bg);
        for (offset, line) in self.pane_tail(id, rows, body).iter().enumerate() {
            surface.text(
                rect.x + 1,
                rect.y + 1 + offset as u16,
                rect.w.saturating_sub(2),
                line,
                quiet,
            );
        }
    }

    /// What to call a pane in the arrange panel.
    ///
    /// Where it is, before what it calls itself: a shell's title is usually
    /// `user@host`, which is the same for every pane and says nothing about
    /// which one you are about to move.
    fn pane_name(&self, id: PaneId) -> String {
        let Some(pane) = self.pane(id) else {
            return "exited".into();
        };
        let here = pane.terminal.cwd().map(|cwd| {
            std::path::Path::new(cwd)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| cwd.to_string())
        });
        match here {
            Some(name) if !name.is_empty() => name,
            _ => pane.terminal.title().trim().to_string(),
        }
    }

    /// The last `rows` lines a pane has on screen, blank tail trimmed.
    fn pane_tail(&self, id: PaneId, rows: usize, width: usize) -> Vec<String> {
        let Some(pane) = self.pane(id) else {
            return Vec::new();
        };
        if rows == 0 || width == 0 {
            return Vec::new();
        }
        let screen = pane.terminal.screen();
        let mut lines: Vec<String> = Vec::new();
        for y in (0..screen.rows()).rev() {
            if lines.len() >= rows {
                break;
            }
            let text: String = screen
                .display_line(y)
                .iter()
                .map(|cell| if cell.ch == '\0' { ' ' } else { cell.ch })
                .collect();
            let text = text.trim_end();
            if text.is_empty() && lines.is_empty() {
                continue;
            }
            lines.push(truncate(text, width));
        }
        lines.reverse();
        lines
    }
}

/// Smallest panel worth drawing: below this the diagram has no room.
const ARRANGE_MIN_W: u16 = 30;

const ARRANGE_MIN_H: u16 = 10;

/// The arrange panel's frame for a window of this size.
pub fn arrange_rect(size: (u16, u16)) -> Rect {
    let w = (size.0 * 3 / 4).clamp(ARRANGE_MIN_W.min(size.0), 120.min(size.0));
    let h = (size.1 * 3 / 5).clamp(ARRANGE_MIN_H.min(size.1), 36.min(size.1));
    Rect::new(
        size.0.saturating_sub(w) / 2,
        size.1.saturating_sub(h) / 3,
        w,
        h,
    )
}

/// The diagram's area inside the panel: the frame less its border, the title
/// row and the hint row.
pub fn arrange_canvas(panel: Rect) -> Rect {
    Rect::new(
        panel.x + 2,
        panel.y + 2,
        panel.w.saturating_sub(4),
        panel.h.saturating_sub(5),
    )
}
