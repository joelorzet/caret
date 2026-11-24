#[allow(unused_imports)]
use crate::prelude::*;

impl KeyBinding {
    pub const fn new(mods: Modifiers, key: Key) -> KeyBinding {
        KeyBinding { mods, key }
    }

    pub const fn plain(key: Key) -> KeyBinding {
        KeyBinding {
            mods: Modifiers::NONE,
            key,
        }
    }

    pub const fn ch(c: char) -> KeyBinding {
        KeyBinding::plain(Key::Char(c))
    }

    /// Parses `ctrl+b`, `cmd+shift+]`, `%`, `f5`.
    ///
    /// `shift+` with a single character is rejected: the shifted character
    /// itself is unambiguous across layouts, so write `D` rather than
    /// `shift+d`.
    pub fn parse(spec: &str) -> Result<KeyBinding, KeyParseError> {
        let spec = spec.trim();
        let err = |reason| KeyParseError {
            input: spec.to_string(),
            reason,
        };

        if spec.is_empty() {
            return Err(err("empty binding"));
        }
        if spec.chars().count() == 1 {
            let c = spec.chars().next().expect("length checked");
            return Ok(KeyBinding::ch(c));
        }

        let parts: Vec<&str> = spec.split('+').collect();
        // A trailing empty part means the key itself is '+'.
        let (key_part, mod_parts) = if parts.last() == Some(&"") {
            ("plus", &parts[..parts.len().saturating_sub(2)])
        } else {
            (parts[parts.len() - 1], &parts[..parts.len() - 1])
        };

        let mut mods = Modifiers::NONE;
        for p in mod_parts {
            let name = p.trim().to_ascii_lowercase();
            match Modifiers::from_name(&name) {
                Some(m) => mods.insert(m),
                None => return Err(err("unknown modifier")),
            }
        }

        let key = Key::from_name(key_part.trim()).ok_or_else(|| err("unknown key"))?;

        if key.is_char() && mods.contains(Modifiers::SHIFT) {
            return Err(err("write the shifted character instead of shift+"));
        }

        Ok(KeyBinding { mods, key })
    }

    /// True when `pressed` should trigger this binding.
    ///
    /// Shift is ignored for character keys because the character already
    /// reflects it, and compared for named keys where it does not.
    pub fn matches(&self, pressed: &KeyBinding) -> bool {
        if self.key != pressed.key {
            return false;
        }
        if self.key.is_char() {
            self.mods.without_shift() == pressed.mods.without_shift()
        } else {
            self.mods == pressed.mods
        }
    }
}

impl core::fmt::Display for KeyBinding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}", self.mods, self.key)
    }
}
