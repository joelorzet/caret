#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum LayoutKind {
    Pane,
    Split {
        axis: Axis,
        children: Vec<LayoutNode>,
    },
}
