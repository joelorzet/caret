#[allow(unused_imports)]
use crate::prelude::*;

/// Where each region of the window lands. `None` means hidden.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Frame {
    pub tab_bar: Option<Rect>,
    pub left: Option<Rect>,
    pub center: Rect,
    pub right: Option<Rect>,
    pub status: Option<Rect>,
}
