//! Where the cursor is and what it looks like.
//!
//! Part of the drawing layer: one region of the window per file, so a
//! change to one of them is a change to one file.

#[allow(unused_imports)]
use super::*;

impl Session {
    /// Where the cursor sits in window cells, and how it should look.
    /// The cursor as the painter wants it: where, what shape, what colour.
    ///
    /// The shape is the application's, over DECSCUSR. The colour and the fill
    /// are the person's, from the config, because whether a cursor blinks is
    /// taste rather than something a program should decide. A program that
    /// explicitly asked for a steady cursor still gets one.
    pub(crate) fn cursor_paint(&self, target: Option<(PaneId, Rect)>) -> Option<CursorPaint> {
        let at = self.cursor_cell(target)?;
        let style = self.pane(target?.0)?.terminal.cursor_style();
        let fill = self.config.terminal.cursor_fill;

        Some(CursorPaint {
            at,
            shape: style.shape,
            colour: self
                .config
                .terminal
                .cursor_color
                .unwrap_or(self.theme.cursor),
            text: self.theme.cursor_text,
            alpha: fill.alpha(),
        })
    }

    /// Whether the cursor should be made to appear and disappear.
    ///
    /// Asked of whichever pane owns the cursor: with the agent focused the
    /// cursor is its, and the pane behind it has no say in how it behaves.
    pub fn cursor_blinks(&self) -> bool {
        let owner = match self.focus {
            FocusTarget::Agent => self.agent,
            FocusTarget::Explorer => None,
            FocusTarget::Panes => Some(self.workspace.active_pane()),
        };
        let Some(pane) = owner.and_then(|id| self.pane(id)) else {
            return false;
        };
        self.config.terminal.cursor_fill.blinks() && pane.terminal.cursor_style().blink
    }

    pub(crate) fn cursor_cell(&self, cursor: Option<(PaneId, Rect)>) -> Option<(u16, u16)> {
        if self.palette.is_some() {
            return None;
        }
        let (id, rect) = cursor?;
        let pane = self.pane(id)?;
        let screen = pane.terminal.screen();
        if !pane.terminal.cursor_visible() || screen.scroll_offset() > 0 {
            return None;
        }
        let position = screen.cursor();
        Some((
            rect.x + position.x.min(rect.w.saturating_sub(1)),
            rect.y + position.y.min(rect.h.saturating_sub(1)),
        ))
    }

    /// Puts the real cursor where the focused terminal's cursor is, so the
    /// host terminal draws it with the user's own style and blink.
    pub(super) fn place_cursor(&self, target: Option<(PaneId, Rect)>) -> String {
        const HIDE: &str = "\x1b[?25l";
        if self.palette.is_some() {
            return HIDE.to_string();
        }
        let Some((id, rect)) = target else {
            return HIDE.to_string();
        };
        let Some(pane) = self.pane(id) else {
            return HIDE.to_string();
        };
        let screen = pane.terminal.screen();
        // Scrolled back into history, the cursor is not on screen.
        if !pane.terminal.cursor_visible() || screen.scroll_offset() > 0 {
            return HIDE.to_string();
        }
        let cursor = screen.cursor();
        let x = rect.x + cursor.x.min(rect.w.saturating_sub(1));
        let y = rect.y + cursor.y.min(rect.h.saturating_sub(1));
        format!("\x1b[{};{}H\x1b[?25h", y + 1, x + 1)
    }
}
