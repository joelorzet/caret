#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    #[inline]
    pub fn region(&self) -> ScrollRegion {
        self.region
    }

    pub fn set_region(&mut self, top: u16, bottom: u16) {
        let bottom = bottom.min(self.rows - 1);
        if top >= bottom {
            self.region = ScrollRegion {
                top: 0,
                bottom: self.rows - 1,
            };
        } else {
            self.region = ScrollRegion { top, bottom };
        }
    }

    pub fn reset_region(&mut self) {
        self.region = ScrollRegion {
            top: 0,
            bottom: self.rows - 1,
        };
    }
}
