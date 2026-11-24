#[allow(unused_imports)]
use crate::prelude::*;

impl PxRect {
    pub const fn new(x: i32, y: i32, w: u32, h: u32) -> PxRect {
        PxRect { x, y, w, h }
    }

    pub const fn right(self) -> i32 {
        self.x + self.w as i32
    }

    pub const fn bottom(self) -> i32 {
        self.y + self.h as i32
    }

    pub fn contains(self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }

    pub fn inset(self, dx: u32, dy: u32) -> PxRect {
        PxRect {
            x: self.x + dx as i32,
            y: self.y + dy as i32,
            w: self.w.saturating_sub(dx * 2),
            h: self.h.saturating_sub(dy * 2),
        }
    }

    /// Centres a box of this size inside the rectangle.
    pub fn centred(self, w: u32, h: u32) -> PxRect {
        PxRect {
            x: self.x + (self.w.saturating_sub(w) / 2) as i32,
            y: self.y + (self.h.saturating_sub(h) / 2) as i32,
            w,
            h,
        }
    }
}
