#[allow(unused_imports)]
use crate::prelude::*;

impl Session {
    pub(crate) fn preview_key(&mut self, press: &KeyPress) -> std::io::Result<()> {
        let Some(preview) = self.preview.as_mut() else {
            return Ok(());
        };
        let Some(key) = crate::impls::host::session::panel_key(&press.binding) else {
            return Ok(());
        };

        self.needs_draw = true;
        match preview.handle(key) {
            PanelOutcome::Dismiss => self.preview = None,
            PanelOutcome::Copy(text) => self.copy_text(&text),
            PanelOutcome::OpenEditor(path) => {
                self.open_editor(path)?;
            }
            PanelOutcome::Redraw | PanelOutcome::Ignored => {}
            // A preview cannot open panes or further previews.
            _ => {}
        }
        Ok(())
    }

    /// Where the preview panel sits.
    ///
    /// Drawing, hit testing and the preview's own line count all come from
    /// here. They disagreed once, and the cursor walked off the bottom of the
    /// frame and appeared to stop moving.
    pub(crate) fn preview_rect(&self) -> Rect {
        /// Four fifths of the window, but never so small that a line of
        /// code has nowhere to go.
        const SHARE: u16 = 4;
        const OF: u16 = 5;
        const MIN_WIDTH: u16 = 30;
        const MIN_HEIGHT: u16 = 8;

        let width = (self.size.0 * SHARE / OF).clamp(MIN_WIDTH.min(self.size.0), self.size.0);
        let height = (self.size.1 * SHARE / OF).clamp(MIN_HEIGHT.min(self.size.1), self.size.1);
        Rect::new(
            (self.size.0.saturating_sub(width)) / 2,
            (self.size.1.saturating_sub(height)) / 2,
            width,
            height,
        )
    }

    /// Rows of file inside that frame: a border and the title above, the hint
    /// row and a border below.
    pub(crate) fn preview_rows(&self) -> usize {
        self.preview_rect().h.saturating_sub(2).max(1) as usize
    }

    /// Clicks, drags and the wheel inside the preview.
    pub(crate) fn preview_mouse(&mut self, event: MouseEvent) -> bool {
        let rect = self.preview_rect();
        let Some(preview) = self.preview.as_mut() else {
            return false;
        };
        if !rect.contains(event.col, event.row) {
            return true;
        }

        if event.is_wheel() {
            let delta = if event.button == MouseButton::WheelUp {
                -3
            } else {
                3
            };
            preview.scroll(delta);
            self.needs_draw = true;
            return true;
        }

        // The first content row sits just inside the border.
        let row = event.row.saturating_sub(rect.y + 1) as usize;
        match event.kind {
            MouseKind::Press if event.button == MouseButton::Left => {
                preview.click(row);
            }
            MouseKind::Motion if self.dragging => {
                preview.drag(row);
            }
            _ => {}
        }
        if event.kind == MouseKind::Press {
            self.dragging = true;
        }
        if event.kind == MouseKind::Release {
            self.dragging = false;
        }
        self.needs_draw = true;
        true
    }

    /// Puts text on the system clipboard and says how much went.
    pub(crate) fn copy_text(&mut self, text: &str) {
        let lines = text.lines().count().max(1);
        self.message = match clipboard::copy(text, self.native_window) {
            Ok(()) => format!(" copied {lines} line(s)"),
            Err(e) => format!(" copy failed: {e}"),
        };
        self.needs_draw = true;
    }
}
