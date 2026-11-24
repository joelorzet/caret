#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ColorKind {
    Default,
    Indexed(u8),
    Rgb(Rgb),
}
