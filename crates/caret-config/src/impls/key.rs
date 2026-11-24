#[allow(unused_imports)]
use crate::prelude::*;

impl Key {
    pub fn from_name(name: &str) -> Option<Key> {
        let lower = name.to_ascii_lowercase();
        if let Some(n) = lower.strip_prefix('f') {
            if let Ok(n) = n.parse::<u8>() {
                if (1..=24).contains(&n) {
                    return Some(Key::F(n));
                }
            }
        }
        Some(match lower.as_str() {
            "enter" | "return" => Key::Enter,
            "tab" => Key::Tab,
            "backspace" | "bs" => Key::Backspace,
            "esc" | "escape" => Key::Escape,
            "space" => Key::Char(' '),
            "del" | "delete" => Key::Delete,
            "ins" | "insert" => Key::Insert,
            "left" => Key::Left,
            "right" => Key::Right,
            "up" => Key::Up,
            "down" => Key::Down,
            "home" => Key::Home,
            "end" => Key::End,
            "pageup" | "pgup" => Key::PageUp,
            "pagedown" | "pgdn" | "pagedn" => Key::PageDown,
            "plus" => Key::Char('+'),
            "minus" | "dash" => Key::Char('-'),
            "slash" => Key::Char('/'),
            "comma" => Key::Char(','),
            "period" | "dot" => Key::Char('.'),
            "semicolon" => Key::Char(';'),
            "quote" => Key::Char('\''),
            "dquote" => Key::Char('"'),
            "backslash" => Key::Char('\\'),
            "lbracket" | "bracketleft" => Key::Char('['),
            "rbracket" | "bracketright" => Key::Char(']'),
            _ => {
                let mut chars = name.chars();
                let first = chars.next()?;
                if chars.next().is_some() {
                    return None;
                }
                Key::Char(first)
            }
        })
    }

    pub const fn is_char(self) -> bool {
        matches!(self, Key::Char(_))
    }
}

impl core::fmt::Display for Key {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Key::Char(' ') => f.write_str("space"),
            Key::Char(c) => write!(f, "{c}"),
            Key::F(n) => write!(f, "f{n}"),
            Key::Enter => f.write_str("enter"),
            Key::Tab => f.write_str("tab"),
            Key::Backspace => f.write_str("backspace"),
            Key::Escape => f.write_str("esc"),
            Key::Delete => f.write_str("delete"),
            Key::Insert => f.write_str("insert"),
            Key::Left => f.write_str("left"),
            Key::Right => f.write_str("right"),
            Key::Up => f.write_str("up"),
            Key::Down => f.write_str("down"),
            Key::Home => f.write_str("home"),
            Key::End => f.write_str("end"),
            Key::PageUp => f.write_str("pageup"),
            Key::PageDown => f.write_str("pagedown"),
        }
    }
}
