#[allow(unused_imports)]
use crate::prelude::*;

impl Dock {
    pub const fn new(kind: DockKind, cells: u16) -> Dock {
        Dock {
            kind,
            cells,
            visible: false,
        }
    }

    pub fn toggle(&mut self) -> bool {
        self.visible = !self.visible;
        self.visible
    }

    pub fn resize(&mut self, cells: u16) {
        self.cells = cells.clamp(Self::MIN_CELLS, Self::MAX_CELLS);
    }
}

impl Dock {
    pub const MIN_CELLS: u16 = 12;
    pub const MAX_CELLS: u16 = 120;
}
