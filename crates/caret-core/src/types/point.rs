#[allow(unused_imports)]
use crate::prelude::*;

/// A position in the grid, addressed by *logical* row.
///
/// Logical rather than viewport rows, so a selection survives new output
/// scrolling the view: the rows it names do not move.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Point {
    pub row: u32,
    pub col: u16,
}
