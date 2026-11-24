#[allow(unused_imports)]
use crate::prelude::*;

impl Style {
    pub fn new(fg: Rgb, bg: Rgb) -> Style {
        Style {
            fg,
            bg,
            flags: Flags::NONE,
        }
    }

    pub fn bold(mut self) -> Style {
        self.flags.insert(Flags::BOLD);
        self
    }
}
