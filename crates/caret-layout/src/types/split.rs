#[allow(unused_imports)]
use crate::prelude::*;

/// An n-ary split rather than a binary one. A new pane takes half of the pane
/// it split, as in tmux, but it joins the existing split as a true sibling
/// instead of nesting a new one. That keeps `equalize` and border resizing
/// operating on the row the user sees rather than on hidden nesting.
#[derive(Clone)]
pub struct Split {
    pub(crate) axis: Axis,
    pub(crate) children: Vec<Node>,
    /// Parallel to `children`. Normalised on every edit.
    pub(crate) weights: Vec<f32>,
}
