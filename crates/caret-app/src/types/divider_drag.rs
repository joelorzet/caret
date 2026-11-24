#[allow(unused_imports)]
use crate::prelude::*;

/// A border being dragged with the mouse.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DividerDrag {
    pub target: DragTarget,
    /// Where the pointer was when the border last moved, so a drag resizes by
    /// the distance travelled rather than jumping to the pointer.
    pub anchor: u16,
}
