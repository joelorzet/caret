#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    pub fn is_damaged(&self, y: u16) -> bool {
        self.all_dirty || self.display_meta(y).is_dirty()
    }

    pub fn is_fully_damaged(&self) -> bool {
        self.all_dirty
    }

    pub fn damage_all(&mut self) {
        self.all_dirty = true;
    }

    pub fn damage_row(&mut self, y: u16) {
        let l = self.vp_base() + y.min(self.rows - 1) as u32;
        let slot = self.slot(l);
        self.meta[slot].0 |= RowMeta::DIRTY;
    }

    pub fn clear_damage(&mut self) {
        self.all_dirty = false;
        for m in self.meta.iter_mut() {
            m.0 &= !RowMeta::DIRTY;
        }
    }
}
