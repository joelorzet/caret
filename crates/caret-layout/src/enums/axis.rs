#[allow(unused_imports)]
use crate::prelude::*;

/// The direction children of a split are laid out along.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Axis {
    /// Children sit side by side, separated by vertical dividers.
    Horizontal,
    /// Children stack top to bottom, separated by horizontal dividers.
    Vertical,
}
