#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    #[inline]
    pub fn cols(&self) -> u16 {
        self.cols
    }

    #[inline]
    pub fn rows(&self) -> u16 {
        self.rows
    }

    #[inline]
    pub fn history_len(&self) -> u32 {
        self.len - self.rows as u32
    }

    #[inline]
    pub(crate) fn ring(&self, l: u32) -> usize {
        debug_assert!(l < self.cap);
        let i = self.head as u64 + l as u64;
        (i % self.cap as u64) as usize
    }

    #[inline]
    pub(crate) fn slot(&self, l: u32) -> usize {
        self.slots[self.ring(l)] as usize
    }

    #[inline]
    pub(crate) fn set_slot(&mut self, l: u32, slot: u32) {
        let r = self.ring(l);
        self.slots[r] = slot;
    }

    #[inline]
    pub(crate) fn vp_base(&self) -> u32 {
        self.len - self.rows as u32
    }

    #[inline]
    pub(crate) fn cells_of(&self, slot: usize) -> &[Cell] {
        let w = self.cols as usize;
        &self.cells[slot * w..slot * w + w]
    }

    #[inline]
    pub(crate) fn cells_of_mut(&mut self, slot: usize) -> &mut [Cell] {
        let w = self.cols as usize;
        &mut self.cells[slot * w..slot * w + w]
    }

    pub fn logical_line(&self, l: u32) -> &[Cell] {
        self.cells_of(self.slot(l.min(self.len - 1)))
    }

    pub fn line(&self, y: u16) -> &[Cell] {
        self.logical_line(self.vp_base() + y.min(self.rows - 1) as u32)
    }

    pub fn line_mut(&mut self, y: u16) -> &mut [Cell] {
        let l = self.vp_base() + y.min(self.rows - 1) as u32;
        let slot = self.slot(l);
        self.meta[slot].0 |= RowMeta::DIRTY;
        self.cells_of_mut(slot)
    }

    pub fn display_line(&self, y: u16) -> &[Cell] {
        self.logical_line(self.display_base() + y.min(self.rows - 1) as u32)
    }

    pub fn display_meta(&self, y: u16) -> RowMeta {
        let l = self.display_base() + y.min(self.rows - 1) as u32;
        self.meta[self.slot(l)]
    }

    #[inline]
    pub(crate) fn display_base(&self) -> u32 {
        self.vp_base() - self.scroll_offset.min(self.history_len())
    }

    /// Whether a stored row continues onto the next one, by its place in
    /// the whole buffer rather than on the screen.
    pub fn wrapped_row(&self, l: u32) -> bool {
        self.meta[self.slot(l.min(self.len.saturating_sub(1)))].is_wrapped()
    }

    pub fn is_wrapped(&self, y: u16) -> bool {
        let l = self.vp_base() + y.min(self.rows - 1) as u32;
        self.meta[self.slot(l)].is_wrapped()
    }

    pub(crate) fn set_wrapped(&mut self, y: u16, on: bool) {
        let l = self.vp_base() + y.min(self.rows - 1) as u32;
        let slot = self.slot(l);
        if on {
            self.meta[slot].0 |= RowMeta::WRAPPED;
        } else {
            self.meta[slot].0 &= !RowMeta::WRAPPED;
        }
    }
}
