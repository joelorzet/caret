#[allow(unused_imports)]
use crate::prelude::*;

/// The window's pixel buffer, with its dimensions, so drawing helpers take a
/// rectangle rather than six loose numbers.
///
/// Pixels are `0x00RRGGBB`, which is what softbuffer presents.
pub struct Canvas<'a> {
    pub pixels: &'a mut [u32],
    pub width: usize,
    pub height: usize,
}
