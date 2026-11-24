#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StripHit {
    Tab(usize),
    Close(usize),
    NewTab,
    /// Bare strip: on a platform with a hidden title bar this is the drag
    /// handle for the window.
    Background,
}
