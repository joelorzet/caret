#[allow(unused_imports)]
use crate::prelude::*;

/// The sixteen ANSI colors.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Palette {
    pub normal: [Rgb; 8],
    pub bright: [Rgb; 8],
}
