#[allow(unused_imports)]
use crate::prelude::*;

impl RowMeta {
    pub(crate) const DIRTY: u8 = 1 << 0;
    pub(crate) const WRAPPED: u8 = 1 << 1;

    pub fn is_dirty(self) -> bool {
        self.0 & Self::DIRTY != 0
    }

    pub fn is_wrapped(self) -> bool {
        self.0 & Self::WRAPPED != 0
    }
}
