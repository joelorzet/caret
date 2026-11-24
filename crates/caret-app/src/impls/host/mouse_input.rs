#[allow(unused_imports)]
use crate::prelude::*;

use crate::impls::host::draw::{agent_rect, dock_body, DOCK_HEADER};

/// Rows the wheel moves per notch.
const WHEEL_LINES: usize = 3;

impl Session {
    pub(crate) fn on_mouse(&mut self, event: MouseEvent) {
        if self.palette.is_some() {
            let _ = self.palette_mouse(event);
            return;
        }

        // The arrange panel is modal, like the palette: it takes the mouse
        // and nothing reaches the panes it is covering.
        if self.is_rearranging() {
            self.rearrange_mouse(event);
            return;
        }

        // A preview is modal, so it gets the mouse before anything beneath.
        if self.preview.is_some() && self.preview_mouse(event) {
            return;
        }

        // A drag in progress owns the mouse until the button comes up, or
        // letting go over a pane would hand it the release.
        if self.is_resizing() {
            match event.kind {
                MouseKind::Release => self.end_resize_drag(),
                _ => self.continue_resize_drag(event),
            }
            return;
        }
        if presses(&event) && !event.is_wheel() && self.begin_resize_drag(event) {
            return;
        }

        let frame = self.chrome.layout(self.window_rect());

        if let Some(bar) = frame.tab_bar {
            if bar.contains(event.col, event.row) {
                self.tab_bar_click(bar, event);
                return;
            }
        }

        if let Some(rect) = frame.left {
            if rect.contains(event.col, event.row) {
                self.explorer_click(rect, event);
                return;
            }
        }

        if let Some((rect, agent)) = frame.right.zip(self.agent) {
            if rect.contains(event.col, event.row) {
                if presses(&event) {
                    self.focus = FocusTarget::Agent;
                    self.needs_draw = true;
                }
                self.deliver_mouse(agent, agent_rect(rect), event);
                return;
            }
        }

        let resolved = self.workspace.active_tab().resolve(frame.center);
        let hit = resolved
            .panes
            .iter()
            .copied()
            .find(|(_, rect)| rect.contains(event.col, event.row));
        if let Some((id, rect)) = hit {
            if presses(&event) {
                self.focus = FocusTarget::Panes;
                self.workspace.active_tab_mut().set_focus(id);
                self.needs_draw = true;
            }
            self.deliver_mouse(id, rect, event);
        }
    }

    fn tab_bar_click(&mut self, bar: Rect, event: MouseEvent) {
        if !presses(&event) {
            return;
        }
        let slot = self
            .tab_bar_slots(bar)
            .into_iter()
            .find(|(_, rect)| rect.contains(event.col, event.row));
        if let Some((index, _)) = slot {
            self.workspace.select_tab(index);
            self.focus = FocusTarget::Panes;
            self.needs_draw = true;
        }
    }

    fn explorer_click(&mut self, rect: Rect, event: MouseEvent) {
        self.needs_draw = true;

        if event.is_wheel() {
            let key = if event.button == MouseButton::WheelUp {
                PanelKey::Up
            } else {
                PanelKey::Down
            };
            for _ in 0..WHEEL_LINES {
                self.explorer.handle(key);
            }
            return;
        }
        if !presses(&event) {
            return;
        }

        self.focus = FocusTarget::Explorer;

        // Where the listing starts comes from the same geometry that drew it;
        // working it out again from the dock's own rectangle is how a click
        // ends up selecting a different file than the one under the pointer.
        let body = dock_body(rect, true);
        let first = body.y + DOCK_HEADER;
        if event.row < first {
            return;
        }
        let index = self.explorer.offset() + (event.row - first) as usize;
        self.explorer.select(index);
    }

    /// Hands the report to the child when it asked for one, and otherwise
    /// treats the wheel as scrollback movement.
    fn deliver_mouse(&mut self, id: PaneId, rect: Rect, event: MouseEvent) {
        let Some(pane) = self.pane(id) else { return };
        let modes = pane.terminal.modes();
        let alt = pane.terminal.is_alt();

        // Shift bypasses an application's mouse grab, so text can still be
        // selected inside vim or less. Every other terminal does this.
        let shift = event.mods.contains(Modifiers::SHIFT);

        if modes.mouse_reporting() && !shift {
            // Motion floods the child unless it actually asked for it.
            let wants_motion = modes.intersects(Modes::MOUSE_DRAG.union(Modes::MOUSE_MOTION));
            if event.kind == MouseKind::Motion && !wants_motion {
                return;
            }
            if let Some(report) = event.encode_sgr(rect) {
                let _ = self.ptys.write(PtyId(id.0), report.as_bytes());
            }
            return;
        }

        if !event.is_wheel() {
            self.select_with_mouse(id, rect, event);
            return;
        }
        let up = event.button == MouseButton::WheelUp;

        if alt {
            // The alternate screen has no scrollback, so the wheel sends
            // arrow keys instead, which is what other terminals do and what
            // pagers expect.
            let key = if up { "\x1b[A" } else { "\x1b[B" };
            let burst = key.repeat(WHEEL_LINES);
            let _ = self.ptys.write(PtyId(id.0), burst.as_bytes());
            return;
        }

        let lines = WHEEL_LINES as i32;
        if let Some(pane) = self.pane_mut(id) {
            pane.terminal
                .screen_mut()
                .scroll_display(if up { lines } else { -lines });
            self.needs_draw = true;
        }
    }
}

impl Session {
    /// Press begins a selection, motion extends it, release leaves it in
    /// place for the copy command.
    fn select_with_mouse(&mut self, id: PaneId, rect: Rect, event: MouseEvent) {
        let col = event
            .col
            .saturating_sub(rect.x)
            .min(rect.w.saturating_sub(1));
        let y = event
            .row
            .saturating_sub(rect.y)
            .min(rect.h.saturating_sub(1));

        let Some(pane) = self.pane(id) else { return };
        let point = Point::new(pane.terminal.screen().display_row(y), col);

        match event.kind {
            MouseKind::Press if event.button == MouseButton::Left => {
                let clicks = self.click_count(&event);
                let separators = self.config.terminal.word_separators.clone();

                let selection = {
                    let screen = match self.pane(id) {
                        Some(pane) => pane.terminal.screen(),
                        None => return,
                    };
                    match clicks {
                        2 => screen
                            .word_at(point, &separators)
                            .map(|(a, b)| Selection::spanning(a, b, SelectionMode::Word)),
                        n if n >= 3 => {
                            let (a, b) = screen.line_at(point);
                            Some(Selection::spanning(a, b, SelectionMode::Line))
                        }
                        _ => Some(Selection::new(point, SelectionMode::Cell)),
                    }
                };

                self.selection = selection.map(|s| (id, s));
                self.dragging = clicks == 1;
                self.needs_draw = true;
            }
            MouseKind::Motion if self.dragging => {
                if let Some((owner, selection)) = self.selection.as_mut() {
                    if *owner == id {
                        selection.extend(point);
                        self.needs_draw = true;
                    }
                }
            }
            MouseKind::Release => {
                self.dragging = false;
                // A click that selected nothing is just a click.
                let empty = self
                    .selection
                    .as_ref()
                    .is_some_and(|(_, s)| s.is_empty() && s.mode() == SelectionMode::Cell);
                if empty {
                    self.selection = None;
                    self.needs_draw = true;
                }
            }
            _ => {}
        }
    }

    /// Counts repeated clicks in the same cell, cycling 1-2-3 so a fourth
    /// click starts a fresh character selection.
    fn click_count(&mut self, event: &MouseEvent) -> u8 {
        const WINDOW: std::time::Duration = std::time::Duration::from_millis(400);
        let now = std::time::Instant::now();

        let count = match self.last_click {
            Some((at, col, row, previous))
                if now.duration_since(at) < WINDOW && col == event.col && row == event.row =>
            {
                previous % 3 + 1
            }
            _ => 1,
        };
        self.last_click = Some((now, event.col, event.row, count));
        count
    }
}

/// A left-button press, which is what focus and selection react to.
fn presses(event: &MouseEvent) -> bool {
    event.kind == MouseKind::Press && event.button == MouseButton::Left
}
