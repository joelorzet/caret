#[allow(unused_imports)]
use crate::prelude::*;

impl Rect {
    pub const fn new(x: u16, y: u16, w: u16, h: u16) -> Rect {
        Rect { x, y, w, h }
    }

    /// Exclusive right edge.
    pub const fn right(&self) -> u16 {
        self.x + self.w
    }

    /// Exclusive bottom edge.
    pub const fn bottom(&self) -> u16 {
        self.y + self.h
    }

    pub const fn is_empty(&self) -> bool {
        self.w == 0 || self.h == 0
    }

    pub const fn area(&self) -> u32 {
        self.w as u32 * self.h as u32
    }

    pub const fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }

    pub fn inset(&self, dx: u16, dy: u16) -> Rect {
        Rect {
            x: self.x + dx,
            y: self.y + dy,
            w: self.w.saturating_sub(dx * 2),
            h: self.h.saturating_sub(dy * 2),
        }
    }

    pub fn overlap_x(&self, other: &Rect) -> u16 {
        self.right()
            .min(other.right())
            .saturating_sub(self.x.max(other.x))
    }

    pub fn overlap_y(&self, other: &Rect) -> u16 {
        self.bottom()
            .min(other.bottom())
            .saturating_sub(self.y.max(other.y))
    }
}
