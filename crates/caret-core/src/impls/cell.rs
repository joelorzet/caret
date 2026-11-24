#[allow(unused_imports)]
use crate::prelude::*;

impl Cell {
    pub const EMPTY: Cell = Cell {
        ch: ' ',
        fg: Color::DEFAULT,
        bg: Color::DEFAULT,
        flags: Flags::NONE,
        link: LinkId::NONE,
    };

    pub const fn blank(bg: Color) -> Cell {
        Cell {
            ch: ' ',
            fg: Color::DEFAULT,
            bg,
            flags: Flags::NONE,
            link: LinkId::NONE,
        }
    }

    pub fn is_blank(&self) -> bool {
        (self.ch == ' ' || self.ch == '\0')
            && !self.flags.any_underline()
            && !self.flags.contains(Flags::STRIKE)
    }

    pub fn width(&self) -> u16 {
        if self.flags.contains(Flags::WIDE) {
            2
        } else {
            1
        }
    }

    pub const fn wide_tail(fg: Color, bg: Color, flags: Flags) -> Cell {
        Cell {
            ch: ' ',
            fg,
            bg,
            flags: flags.union(Flags::WIDE_TAIL),
            link: LinkId::NONE,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::EMPTY
    }
}
