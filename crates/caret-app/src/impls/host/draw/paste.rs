//! The question a paste asks before it is sent.
//!
//! Part of the drawing layer: one region of the window per file, so a
//! change to one of them is a change to one file.

#[allow(unused_imports)]
use super::*;

/// How many lines of the paste to show. Enough to judge it by; a longer
/// one is judged by its shape and its warnings instead.
const SHOWN: usize = 8;

/// Border above and below, and the footer.
const CHROME: u16 = 3;

impl Session {
    pub(super) fn draw_paste(&self, surface: &mut Surface) {
        let Some(pending) = self.pending_paste.as_ref() else {
            return;
        };
        let ui = &self.theme.ui;

        let shown: Vec<&str> = pending.text.lines().take(SHOWN).collect();
        let mut notes = notes(&pending.review);
        if pending.review.lines > shown.len() {
            notes.push(format!(
                "{} more line(s) not shown",
                pending.review.lines - shown.len()
            ));
        }

        let body = notes.len() + 1 + shown.len();
        let width = (self.size.0 * 3 / 4).clamp(
            MIN_PALETTE_WIDTH.min(self.size.0),
            MAX_PALETTE_WIDTH.min(self.size.0),
        );
        let height = (body as u16 + CHROME).min(self.size.1);
        let rect = Rect::new(
            self.size.0.saturating_sub(width) / 2,
            self.size.1.saturating_sub(height) / 3,
            width,
            height,
        );

        surface.fill(rect, ui.panel_bg);
        draw_border(surface, rect, ui.border_focused, ui.panel_bg);
        surface.text(
            rect.x + 2,
            rect.y,
            rect.w.saturating_sub(4),
            "Paste",
            Style::new(ui.accent, ui.panel_bg).bold(),
        );

        let inner = rect.w.saturating_sub(4);
        let mut y = rect.y + 1;
        for (index, note) in notes.iter().enumerate() {
            // The first line counts; the rest are reasons to look twice.
            let colour = if index == 0 { ui.chrome_fg } else { ui.accent };
            surface.text(rect.x + 2, y, inner, note, Style::new(colour, ui.panel_bg));
            y += 1;
        }
        y += 1;
        for line in &shown {
            surface.text(
                rect.x + 2,
                y,
                inner,
                &visible(line),
                Style::new(ui.chrome_fg, ui.panel_bg),
            );
            y += 1;
        }

        self.draw_hints(
            surface,
            (rect.x + 2, rect.bottom() - 1),
            inner,
            &[("enter", "send"), ("esc", "drop")],
            ui.panel_bg,
        );
    }
}

/// What is worth knowing before sending a paste, a line at a time so that
/// nothing important is cut off at the edge of the panel.
pub fn notes(review: &PasteReview) -> Vec<String> {
    let mut notes = vec![format!(
        "{} line(s), {} character(s)",
        review.lines, review.chars
    )];
    if review.runs_on_arrival {
        notes.push("ends with a newline, so it runs on arrival".to_string());
    }
    if !review.controls.is_empty() {
        notes.push(format!("contains {}", review.controls.join(", ")));
    }
    notes
}

/// A line with its control characters shown rather than obeyed.
pub fn visible(line: &str) -> String {
    line.chars()
        .map(|ch| match ch {
            '\t' => ' ',
            c if c.is_control() => '\u{fffd}',
            c => c,
        })
        .collect()
}
