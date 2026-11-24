#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SurfaceCell {
    pub ch: char,
    pub fg: Rgb,
    pub bg: Rgb,
    pub flags: Flags,
    pub round: Corners,
    /// What shows outside a rounded corner: whatever this cell is laid on
    /// top of, which is not always the window's background.
    pub under: Rgb,
}
