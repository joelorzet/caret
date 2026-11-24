#[allow(unused_imports)]
use crate::prelude::*;

/// Everything the painter needs to draw the cursor.
///
/// The shape comes from the application over DECSCUSR, the colour and the
/// alpha from the config and theme, and `alpha` is already zero when a
/// blinking cursor is in its off phase — so the painter decides nothing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CursorPaint {
    pub at: (u16, u16),
    pub shape: CursorShape,
    pub colour: Rgb,
    /// What a character under a filled cursor is redrawn in.
    pub text: Rgb,
    pub alpha: u8,
}
