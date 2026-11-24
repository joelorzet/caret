#[allow(unused_imports)]
use crate::prelude::*;

impl Flags {
    pub const NONE: Flags = Flags(0);
    pub const BOLD: Flags = Flags(1 << 0);
    pub const DIM: Flags = Flags(1 << 1);
    pub const ITALIC: Flags = Flags(1 << 2);
    pub const UNDERLINE: Flags = Flags(1 << 3);
    pub const BLINK: Flags = Flags(1 << 4);
    pub const REVERSE: Flags = Flags(1 << 5);
    pub const HIDDEN: Flags = Flags(1 << 6);
    pub const STRIKE: Flags = Flags(1 << 7);
    pub const WIDE: Flags = Flags(1 << 8);
    /// Right half of a double-width character. Never drawn directly.
    pub const WIDE_TAIL: Flags = Flags(1 << 9);
    pub const UNDERLINE_DOUBLE: Flags = Flags(1 << 10);
    pub const UNDERLINE_CURLY: Flags = Flags(1 << 11);
    /// This cell's `char` is a handle into the screen's cluster table
    /// rather than the character to draw. Positional, like the wide-char
    /// markers, so it is not copied from the pen.
    pub const CLUSTER: Flags = Flags(1 << 12);

    /// Flags copied from the pen onto printed cells. Excludes the wide-char
    /// markers, which are positional rather than part of the SGR state.
    pub const SGR_MASK: Flags = Flags(0x0cff);

    pub const fn contains(self, other: Flags) -> bool {
        self.0 & other.0 == other.0
    }

    pub const fn intersects(self, other: Flags) -> bool {
        self.0 & other.0 != 0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn insert(&mut self, other: Flags) {
        self.0 |= other.0;
    }

    pub fn remove(&mut self, other: Flags) {
        self.0 &= !other.0;
    }

    pub fn set(&mut self, other: Flags, on: bool) {
        if on {
            self.insert(other)
        } else {
            self.remove(other)
        }
    }

    pub const fn union(self, other: Flags) -> Flags {
        Flags(self.0 | other.0)
    }

    pub const fn intersection(self, other: Flags) -> Flags {
        Flags(self.0 & other.0)
    }

    pub const fn bits(self) -> u16 {
        self.0
    }

    pub const fn from_bits(bits: u16) -> Flags {
        Flags(bits)
    }

    pub const fn any_underline(self) -> bool {
        self.intersects(Flags(
            Flags::UNDERLINE.0 | Flags::UNDERLINE_DOUBLE.0 | Flags::UNDERLINE_CURLY.0,
        ))
    }
}

impl core::ops::BitOr for Flags {
    type Output = Flags;
    fn bitor(self, rhs: Flags) -> Flags {
        self.union(rhs)
    }
}

impl core::ops::BitOrAssign for Flags {
    fn bitor_assign(&mut self, rhs: Flags) {
        self.insert(rhs)
    }
}

impl core::fmt::Debug for Flags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        const NAMES: [(Flags, &str); 13] = [
            (Flags::BOLD, "BOLD"),
            (Flags::DIM, "DIM"),
            (Flags::ITALIC, "ITALIC"),
            (Flags::UNDERLINE, "UNDERLINE"),
            (Flags::BLINK, "BLINK"),
            (Flags::REVERSE, "REVERSE"),
            (Flags::HIDDEN, "HIDDEN"),
            (Flags::STRIKE, "STRIKE"),
            (Flags::WIDE, "WIDE"),
            (Flags::WIDE_TAIL, "WIDE_TAIL"),
            (Flags::UNDERLINE_DOUBLE, "UNDERLINE_DOUBLE"),
            (Flags::UNDERLINE_CURLY, "UNDERLINE_CURLY"),
            (Flags::CLUSTER, "CLUSTER"),
        ];
        if self.is_empty() {
            return f.write_str("NONE");
        }
        let mut first = true;
        for (flag, name) in NAMES {
            if self.contains(flag) {
                if !first {
                    f.write_str("|")?;
                }
                f.write_str(name)?;
                first = false;
            }
        }
        Ok(())
    }
}
