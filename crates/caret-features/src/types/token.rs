#[allow(unused_imports)]
use crate::prelude::*;

/// A run of characters, in character offsets rather than bytes so it lines up
/// with the grid.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub kind: TokenKind,
}
