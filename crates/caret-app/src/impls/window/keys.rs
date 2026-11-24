#[allow(unused_imports)]
use crate::prelude::*;

use winit::event::{ElementState, MouseButton as WinitButton};
use winit::keyboard::{Key as WinitKey, ModifiersState, NamedKey};

/// Translates a window keypress into a binding plus the bytes a child expects.
///
/// `text` is what the platform says the key produced, which already accounts
/// for the keyboard layout and dead keys; deriving it ourselves would get
/// every non-US layout wrong.
pub fn key_press(logical: &WinitKey, text: Option<&str>, mods: ModifiersState) -> Option<KeyPress> {
    let modifiers = modifiers(mods);

    let key = match logical {
        WinitKey::Named(named) => named_key(*named)?,
        WinitKey::Character(s) => Key::Char(s.chars().next()?),
        _ => return None,
    };

    let bytes = match (&key, text) {
        // A control combination produces no text, so the C0 byte is derived.
        (Key::Char(c), _) if modifiers.contains(Modifiers::CTRL) => control_byte(*c),
        (Key::Char(_), Some(text)) if !text.is_empty() => text.as_bytes().to_vec(),
        (Key::Char(c), _) => c.to_string().into_bytes(),
        _ => Vec::new(),
    };

    Some(KeyPress {
        binding: KeyBinding::new(modifiers, key),
        bytes,
    })
}

/// `ctrl+a` is byte 1, through `ctrl+z` at 26.
fn control_byte(c: char) -> Vec<u8> {
    let lower = c.to_ascii_lowercase();
    match lower {
        'a'..='z' => vec![lower as u8 - b'a' + 1],
        '@' | ' ' => vec![0],
        '[' => vec![27],
        '\\' => vec![28],
        ']' => vec![29],
        _ => c.to_string().into_bytes(),
    }
}

fn named_key(named: NamedKey) -> Option<Key> {
    Some(match named {
        NamedKey::Enter => Key::Enter,
        NamedKey::Tab => Key::Tab,
        NamedKey::Backspace => Key::Backspace,
        NamedKey::Escape => Key::Escape,
        NamedKey::Space => Key::Char(' '),
        NamedKey::Delete => Key::Delete,
        NamedKey::Insert => Key::Insert,
        NamedKey::ArrowLeft => Key::Left,
        NamedKey::ArrowRight => Key::Right,
        NamedKey::ArrowUp => Key::Up,
        NamedKey::ArrowDown => Key::Down,
        NamedKey::Home => Key::Home,
        NamedKey::End => Key::End,
        NamedKey::PageUp => Key::PageUp,
        NamedKey::PageDown => Key::PageDown,
        NamedKey::F1 => Key::F(1),
        NamedKey::F2 => Key::F(2),
        NamedKey::F3 => Key::F(3),
        NamedKey::F4 => Key::F(4),
        NamedKey::F5 => Key::F(5),
        NamedKey::F6 => Key::F(6),
        NamedKey::F7 => Key::F(7),
        NamedKey::F8 => Key::F(8),
        NamedKey::F9 => Key::F(9),
        NamedKey::F10 => Key::F(10),
        NamedKey::F11 => Key::F(11),
        NamedKey::F12 => Key::F(12),
        _ => return None,
    })
}

pub fn modifiers(mods: ModifiersState) -> Modifiers {
    let mut out = Modifiers::NONE;
    if mods.shift_key() {
        out.insert(Modifiers::SHIFT);
    }
    if mods.control_key() {
        out.insert(Modifiers::CTRL);
    }
    if mods.alt_key() {
        out.insert(Modifiers::ALT);
    }
    if mods.super_key() {
        out.insert(Modifiers::SUPER);
    }
    out
}

pub fn mouse_button(button: WinitButton) -> Option<MouseButton> {
    Some(match button {
        WinitButton::Left => MouseButton::Left,
        WinitButton::Middle => MouseButton::Middle,
        WinitButton::Right => MouseButton::Right,
        _ => return None,
    })
}

pub fn mouse_kind(state: ElementState) -> MouseKind {
    match state {
        ElementState::Pressed => MouseKind::Press,
        ElementState::Released => MouseKind::Release,
    }
}
