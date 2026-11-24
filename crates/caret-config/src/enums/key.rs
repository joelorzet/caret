#[allow(unused_imports)]
use crate::prelude::*;

/// A key, independent of keyboard layout.
///
/// `Char` holds the character the keyboard actually produced, so a shifted
/// key is written as the shifted character (`D`, `%`) rather than as
/// `shift+d`. Shift is therefore ignored when matching `Char` bindings, and
/// significant for the named keys below.
///
/// Space has no variant of its own: it is `Char(' ')`, which is what a
/// keyboard sends. Writing `space` in a binding parses to the same thing.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Char(char),
    Enter,
    Tab,
    Backspace,
    Escape,
    Delete,
    Insert,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    F(u8),
}
