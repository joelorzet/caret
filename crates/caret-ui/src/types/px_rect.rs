#[allow(unused_imports)]
use crate::prelude::*;

/// A rectangle in device pixels.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PxRect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}
