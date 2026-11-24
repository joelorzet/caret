#[allow(unused_imports)]
use crate::prelude::*;

/// One glyph of a shaped run, and how many cells it covers.
///
/// A ligature covers the cells its characters would have taken, which is
/// what makes it drawable in a grid: the font draws `!=` as one glyph two
/// columns wide, and the cells underneath are still two cells.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub glyph: u16,
    pub cells: u16,
}
