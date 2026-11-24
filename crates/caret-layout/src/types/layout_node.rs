#[allow(unused_imports)]
use crate::prelude::*;

/// A node in a layout template: the shape, before any panes exist.
#[derive(Clone, PartialEq, Debug)]
pub struct LayoutNode {
    /// Relative share of the parent. Zero or negative means an equal share.
    pub weight: f32,
    pub kind: LayoutKind,
}
