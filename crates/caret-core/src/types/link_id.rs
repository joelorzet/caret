#[allow(unused_imports)]
use crate::prelude::*;

/// Which marked link a cell belongs to, or none.
///
/// It rides in the two bytes the cell already padded out to, so carrying it
/// costs no memory at all: a cell is still the sixteen bytes
/// `types/cell.rs` asserts. The target itself lives once in the screen's
/// [`LinkTable`], because the same URL is usually printed across a whole run
/// of cells.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
#[repr(transparent)]
pub struct LinkId(pub(crate) u16);
