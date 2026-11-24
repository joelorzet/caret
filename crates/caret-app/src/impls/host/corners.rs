#[allow(unused_imports)]
use crate::prelude::*;

impl Corners {
    pub const NONE: Corners = Corners(0);
    pub const TOP_LEFT: Corners = Corners(1);
    pub const TOP_RIGHT: Corners = Corners(2);
    pub const BOTTOM_RIGHT: Corners = Corners(4);
    pub const BOTTOM_LEFT: Corners = Corners(8);
    pub const LEFT: Corners = Corners(1 | 8);
    pub const RIGHT: Corners = Corners(2 | 4);

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn contains(self, other: Corners) -> bool {
        self.0 & other.0 == other.0
    }

    pub const fn union(self, other: Corners) -> Corners {
        Corners(self.0 | other.0)
    }
}
