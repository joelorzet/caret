#[allow(unused_imports)]
use crate::prelude::*;

/// Whether a theme is meant for a dark or light background. Drives how the
/// chrome colors are derived and which cursor text color reads better.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Appearance {
    Dark,
    Light,
}
