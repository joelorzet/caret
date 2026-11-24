#[allow(unused_imports)]
use crate::prelude::*;

impl LinkId {
    pub const NONE: LinkId = LinkId(0);

    pub const fn is_none(self) -> bool {
        self.0 == 0
    }
}
