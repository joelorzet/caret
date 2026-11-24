#[allow(unused_imports)]
use crate::prelude::*;

/// Which corners of a cell's background are rounded.
///
/// A selection bar is a run of cells, so the rounding belongs to the two cells
/// at its ends rather than to the run; this is how they are told apart.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Corners(pub u8);
