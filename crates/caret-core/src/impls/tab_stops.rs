#[allow(unused_imports)]
use crate::prelude::*;

impl TabStops {
    pub const DEFAULT_INTERVAL: u16 = 8;

    pub fn new(cols: u16) -> TabStops {
        let mut stops = TabStops {
            bits: vec![0u64; words(cols)].into_boxed_slice(),
            cols,
        };
        stops.reset();
        stops
    }

    pub fn reset(&mut self) {
        self.clear_all();
        let mut x = Self::DEFAULT_INTERVAL;
        while x < self.cols {
            self.set(x);
            x += Self::DEFAULT_INTERVAL;
        }
    }

    pub fn clear_all(&mut self) {
        self.bits.fill(0);
    }

    pub fn set(&mut self, col: u16) {
        if col < self.cols {
            self.bits[col as usize / 64] |= 1 << (col % 64);
        }
    }

    pub fn unset(&mut self, col: u16) {
        if col < self.cols {
            self.bits[col as usize / 64] &= !(1 << (col % 64));
        }
    }

    pub fn is_stop(&self, col: u16) -> bool {
        col < self.cols && self.bits[col as usize / 64] & (1 << (col % 64)) != 0
    }

    pub fn next_stop(&self, col: u16) -> u16 {
        let last = self.cols.saturating_sub(1);
        let mut x = col.saturating_add(1);
        while x < last {
            if self.is_stop(x) {
                return x;
            }
            x += 1;
        }
        last
    }

    pub fn prev_stop(&self, col: u16) -> u16 {
        let mut x = col;
        while x > 0 {
            x -= 1;
            if self.is_stop(x) {
                return x;
            }
        }
        0
    }

    pub fn resize(&mut self, cols: u16) {
        if cols == self.cols {
            return;
        }
        let mut bits = vec![0u64; words(cols)].into_boxed_slice();
        let keep = self.bits.len().min(bits.len());
        bits[..keep].copy_from_slice(&self.bits[..keep]);

        let old_cols = self.cols;
        self.bits = bits;
        self.cols = cols;

        if cols > old_cols {
            let mut x = old_cols.next_multiple_of(Self::DEFAULT_INTERVAL);
            while x < cols {
                self.set(x);
                x += Self::DEFAULT_INTERVAL;
            }
        }
    }
}

fn words(cols: u16) -> usize {
    (cols as usize).div_ceil(64).max(1)
}
