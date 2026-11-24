#[allow(unused_imports)]
use crate::prelude::*;

impl StripLayout {
    pub fn close_for(&self, index: usize) -> Option<PxRect> {
        self.closes.get(index).copied()
    }
}
