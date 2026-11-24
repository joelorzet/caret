#[allow(unused_imports)]
use crate::prelude::*;

/// A mouse report, in window cells counted from zero.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MouseEvent {
    pub button: MouseButton,
    pub kind: MouseKind,
    pub col: u16,
    pub row: u16,
    pub mods: Modifiers,
}
