#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Dock {
    pub kind: DockKind,
    pub cells: u16,
    pub visible: bool,
}
