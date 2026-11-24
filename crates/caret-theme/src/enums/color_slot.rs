#[allow(unused_imports)]
use crate::prelude::*;

/// What a `Color::DEFAULT` cell resolves to.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ColorSlot {
    Foreground,
    Background,
}
