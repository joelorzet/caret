#[allow(unused_imports)]
use crate::prelude::*;

/// The keys a panel reacts to.
///
/// Deliberately not the application's `Key` type: a panel should not need the
/// keybinding vocabulary, and this keeps the crate dependency-free.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PanelKey {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Home,
    End,
    PageUp,
    PageDown,
    Escape,
    /// Shift and an arrow: move while extending a selection.
    SelectUp,
    SelectDown,
    HalfPageUp,
    HalfPageDown,
    Char(char),
}
