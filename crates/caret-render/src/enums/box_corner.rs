#[allow(unused_imports)]
use crate::prelude::*;

/// Which way a rounded corner bulges.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BoxCorner {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}
