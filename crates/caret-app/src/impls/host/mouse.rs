#[allow(unused_imports)]
use crate::prelude::*;

/// The button field of an SGR mouse report: a button number in the low bits
/// with modifier and motion flags set above it.
mod button {
    pub const LEFT: u16 = 0;
    pub const MIDDLE: u16 = 1;
    pub const RIGHT: u16 = 2;
    /// No button, which is what a bare motion report carries.
    pub const NONE: u16 = 3;
    pub const SHIFT: u16 = 4;
    pub const ALT: u16 = 8;
    pub const CTRL: u16 = 16;
    /// Set when the pointer moved rather than a button changing.
    pub const MOTION: u16 = 32;
    /// The wheel reports as buttons of its own, past the physical ones.
    pub const WHEEL_UP: u16 = 64;
    pub const WHEEL_DOWN: u16 = 65;
}

/// Release reports end in `m`, everything else in `M`.
const RELEASE: char = 'm';
const PRESS: char = 'M';

/// Decodes an SGR mouse report: `CSI < button ; col ; row M` for a press or
/// motion, `m` for a release.
///
/// Only the SGR encoding is accepted. The original X10 form packs coordinates
/// into single bytes and so cannot address a window wider than 223 columns.
pub fn decode_sgr(params: &str, final_byte: u8) -> Option<MouseEvent> {
    let mut parts = params.split(';');
    let flags: u16 = parts.next()?.parse().ok()?;
    let col: u16 = parts.next()?.parse().ok()?;
    let row: u16 = parts.next()?.parse().ok()?;

    let mut mods = Modifiers::NONE;
    if flags & 4 != 0 {
        mods.insert(Modifiers::SHIFT);
    }
    if flags & 8 != 0 {
        mods.insert(Modifiers::ALT);
    }
    if flags & 16 != 0 {
        mods.insert(Modifiers::CTRL);
    }

    let wheel = flags & 64 != 0;
    let motion = flags & 32 != 0;

    let button = match (wheel, flags & 3) {
        (true, 0) => MouseButton::WheelUp,
        (true, 1) => MouseButton::WheelDown,
        (true, _) => return None,
        (false, 0) => MouseButton::Left,
        (false, 1) => MouseButton::Middle,
        (false, 2) => MouseButton::Right,
        (false, _) => MouseButton::None,
    };

    let kind = if wheel {
        MouseKind::Press
    } else if motion {
        MouseKind::Motion
    } else if final_byte == b'm' {
        MouseKind::Release
    } else {
        MouseKind::Press
    };

    // Reports are one-based; cells are not.
    Some(MouseEvent {
        button,
        kind,
        col: col.saturating_sub(1),
        row: row.saturating_sub(1),
        mods,
    })
}

impl MouseEvent {
    pub fn is_wheel(&self) -> bool {
        matches!(self.button, MouseButton::WheelUp | MouseButton::WheelDown)
    }

    /// Re-encodes the report for a child, with coordinates relative to `rect`.
    pub fn encode_sgr(&self, rect: Rect) -> Option<String> {
        let col = self.col.checked_sub(rect.x)? + 1;
        let row = self.row.checked_sub(rect.y)? + 1;

        let mut flags: u16 = match self.button {
            MouseButton::Left => button::LEFT,
            MouseButton::Middle => button::MIDDLE,
            MouseButton::Right => button::RIGHT,
            MouseButton::None => button::NONE,
            MouseButton::WheelUp => button::WHEEL_UP,
            MouseButton::WheelDown => button::WHEEL_DOWN,
        };
        if self.kind == MouseKind::Motion {
            flags |= button::MOTION;
        }
        if self.mods.contains(Modifiers::SHIFT) {
            flags |= button::SHIFT;
        }
        if self.mods.contains(Modifiers::ALT) {
            flags |= button::ALT;
        }
        if self.mods.contains(Modifiers::CTRL) {
            flags |= button::CTRL;
        }

        let final_byte = if self.kind == MouseKind::Release {
            RELEASE
        } else {
            PRESS
        };
        Some(format!("\x1b[<{flags};{col};{row}{final_byte}"))
    }
}
