#[allow(unused_imports)]
use crate::prelude::*;

impl Default for CursorStyle {
    fn default() -> Self {
        CursorStyle {
            shape: CursorShape::Block,
            blink: true,
        }
    }
}

impl CursorStyle {
    pub fn from_decscusr(param: u16) -> CursorStyle {
        match param {
            0 | 1 => CursorStyle {
                shape: CursorShape::Block,
                blink: true,
            },
            2 => CursorStyle {
                shape: CursorShape::Block,
                blink: false,
            },
            3 => CursorStyle {
                shape: CursorShape::Underline,
                blink: true,
            },
            4 => CursorStyle {
                shape: CursorShape::Underline,
                blink: false,
            },
            5 => CursorStyle {
                shape: CursorShape::Bar,
                blink: true,
            },
            6 => CursorStyle {
                shape: CursorShape::Bar,
                blink: false,
            },
            _ => CursorStyle::default(),
        }
    }
}
