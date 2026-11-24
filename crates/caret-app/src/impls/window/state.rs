#[allow(unused_imports)]
use crate::prelude::*;

use std::time::Duration;

use winit::window::CursorIcon;

use crate::impls::window::keys;

/// How long the cursor goes on blinking after the last key or click.
const BLINK_REST: Duration = Duration::from_secs(10);

impl WindowState {
    /// Whether the cursor should be blinking in this window at this moment.
    ///
    /// Three things have to hold: the configuration and the program in the
    /// pane both want it, the window has the keyboard, and somebody has used
    /// it recently. A window nobody is working in holds a steady cursor and
    /// draws nothing at all.
    pub(crate) fn blinking(&self) -> bool {
        self.focused && self.typed_at.elapsed() < BLINK_REST && self.session.cursor_blinks()
    }

    /// Puts a resize pointer over a border that can be dragged.
    ///
    /// A border that resizes but gives no sign of it is a border nobody
    /// finds; this is the only hint that dragging is possible at all.
    pub(crate) fn show_resize_shape(&mut self, cell: (u16, u16)) {
        let wanted = match self.session.resize_axis_at(cell.0, cell.1) {
            Some(Axis::Horizontal) => CursorIcon::ColResize,
            Some(Axis::Vertical) => CursorIcon::RowResize,
            None => CursorIcon::Default,
        };
        if wanted != self.shape {
            self.shape = wanted;
            self.window.set_cursor(wanted);
        }
    }

    pub(crate) fn mouse(&mut self, button: MouseButton, kind: MouseKind) {
        self.session.handle_mouse(MouseEvent {
            button,
            kind,
            col: self.pointer.0,
            row: self.pointer.1,
            mods: keys::modifiers(self.modifiers),
        });
    }
}
