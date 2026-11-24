#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CursorStyle {
    pub shape: CursorShape,
    pub blink: bool,
}
