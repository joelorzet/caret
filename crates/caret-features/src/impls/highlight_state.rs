#[allow(unused_imports)]
use crate::prelude::*;

impl HighlightState {
    /// True when the previous line ended inside a block comment, so the next
    /// one starts inside it too.
    pub fn is_inside_block_comment(&self) -> bool {
        self.in_block_comment
    }
}
