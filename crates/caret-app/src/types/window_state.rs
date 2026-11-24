#[allow(unused_imports)]
use crate::prelude::*;

use std::sync::Arc;
use std::time::Instant;

use winit::keyboard::ModifiersState;
use winit::window::{CursorIcon, Window};

use caret_ui::{StripHit, StripLayout};

/// One window: the terminal in it, and everything that is true of that
/// window rather than of the program.
///
/// What a window does not own is the event loop and the glyphs, which are
/// the same for every window at a given scale and are kept once.
pub struct WindowState {
    pub(crate) session: Session,
    pub(crate) window: Arc<Window>,
    /// Held because the surface was made from it and outliving it would
    /// leave the surface drawing into nothing.
    pub(crate) _context: softbuffer::Context<Arc<Window>>,
    pub(crate) surface: softbuffer::Surface<Arc<Window>, Arc<Window>>,
    pub(crate) scale: f32,
    pub(crate) modifiers: ModifiersState,
    pub(crate) pointer: (u16, u16),
    /// Where the pointer is in pixels, for the chrome, which is not on the
    /// cell grid.
    pub(crate) pointer_px: (f64, f64),
    pub(crate) hover: StripHit,
    /// The shape the pointer is currently wearing.
    pub(crate) shape: CursorIcon,
    pub(crate) strip: StripLayout,
    pub(crate) blink: Blink,
    /// Whether this window has the keyboard. An unfocused window has no
    /// business blinking a cursor at a nobody, and blinking is the one thing
    /// that makes an idle terminal redraw.
    pub(crate) focused: bool,
    /// When this window last had output, a key or a click.
    pub(crate) active_at: Instant,
    /// When somebody last typed or clicked in it, which is what the cursor
    /// blinks for.
    pub(crate) typed_at: Instant,
    pub(crate) title: String,
}
