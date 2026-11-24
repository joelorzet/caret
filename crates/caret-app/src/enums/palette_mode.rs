#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PaletteMode {
    Commands,
    Themes,
    /// Every setting and its current value, with the file behind it.
    Settings,
}
