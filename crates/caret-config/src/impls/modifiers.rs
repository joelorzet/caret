#[allow(unused_imports)]
use crate::prelude::*;

impl Modifiers {
    pub const NONE: Modifiers = Modifiers(0);
    pub const CTRL: Modifiers = Modifiers(1 << 0);
    pub const ALT: Modifiers = Modifiers(1 << 1);
    pub const SHIFT: Modifiers = Modifiers(1 << 2);
    /// Command on macOS, Windows key elsewhere.
    pub const SUPER: Modifiers = Modifiers(1 << 3);

    pub const fn contains(self, other: Modifiers) -> bool {
        self.0 & other.0 == other.0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn union(self, other: Modifiers) -> Modifiers {
        Modifiers(self.0 | other.0)
    }

    pub fn insert(&mut self, other: Modifiers) {
        self.0 |= other.0;
    }

    pub fn remove(&mut self, other: Modifiers) {
        self.0 &= !other.0;
    }

    pub const fn without_shift(self) -> Modifiers {
        Modifiers(self.0 & !Modifiers::SHIFT.0)
    }

    pub fn from_name(name: &str) -> Option<Modifiers> {
        match name {
            "ctrl" | "control" | "c" => Some(Modifiers::CTRL),
            // Every alias parses everywhere, whatever the platform calls
            // it, so a config file carries between machines.
            "alt" | "opt" | "option" | "meta" => Some(Modifiers::ALT),
            "shift" => Some(Modifiers::SHIFT),
            "cmd" | "command" | "super" | "win" => Some(Modifiers::SUPER),
            _ => None,
        }
    }
}

impl core::ops::BitOr for Modifiers {
    type Output = Modifiers;
    fn bitor(self, rhs: Modifiers) -> Modifiers {
        self.union(rhs)
    }
}

impl core::fmt::Display for Modifiers {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (m, name) in Platform::current().modifier_names() {
            if self.contains(m) {
                write!(f, "{name}+")?;
            }
        }
        Ok(())
    }
}

impl core::fmt::Debug for Modifiers {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_empty() {
            f.write_str("none")
        } else {
            write!(f, "{self}")
        }
    }
}
