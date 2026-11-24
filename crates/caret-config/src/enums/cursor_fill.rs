#[allow(unused_imports)]
use crate::prelude::*;

/// How the cursor is painted, as distinct from what shape it is.
///
/// The shape follows the application, since an editor switches to a bar in
/// insert mode and expects that to work, but whether the cursor blinks is a
/// matter
/// of taste, so it stays with the person rather than the program.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum CursorFill {
    #[default]
    Blink,
    Solid,
    /// Always drawn, but letting the character under it show through.
    Translucent,
}
