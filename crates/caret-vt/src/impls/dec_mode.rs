#[allow(unused_imports)]
use crate::prelude::*;

impl DecMode {
    pub const fn from_code(code: u16) -> Option<DecMode> {
        Some(match code {
            1 => DecMode::ApplicationCursor,
            5 => DecMode::ReverseVideo,
            6 => DecMode::Origin,
            7 => DecMode::AutoWrap,
            12 => DecMode::CursorBlink,
            25 => DecMode::CursorVisible,
            47 => DecMode::AltScreen,
            1000 => DecMode::MouseClick,
            1002 => DecMode::MouseDrag,
            1003 => DecMode::MouseMotion,
            1004 => DecMode::FocusEvents,
            1006 => DecMode::MouseSgr,
            1007 => DecMode::AltScroll,
            1047 => DecMode::AltScreenClear,
            1048 => DecMode::SaveCursor,
            1049 => DecMode::AltScreenSaveClear,
            2004 => DecMode::BracketedPaste,
            2026 => DecMode::SynchronisedUpdate,
            _ => return None,
        })
    }
}

impl AnsiMode {
    pub const fn from_code(code: u16) -> Option<AnsiMode> {
        Some(match code {
            4 => AnsiMode::Insert,
            20 => AnsiMode::LineFeedNewLine,
            _ => return None,
        })
    }
}
