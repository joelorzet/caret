#[allow(unused_imports)]
use crate::prelude::*;

/// An off-screen character buffer for one frame.
///
/// Everything is composed here first so a frame reaches the terminal as a
/// single write, rather than as interleaved escape sequences that would tear.
pub struct Surface {
    pub(crate) cols: u16,
    pub(crate) rows: u16,
    pub(crate) cells: Vec<SurfaceCell>,
    /// The text of the cells that show more than one character, which the
    /// cell points at rather than holds. A frame of ordinary text leaves
    /// this empty, so no cell is any bigger for the ones that are not.
    pub(crate) clusters: Vec<Box<str>>,
}
