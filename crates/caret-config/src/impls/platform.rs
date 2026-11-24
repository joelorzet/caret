#[allow(unused_imports)]
use crate::prelude::*;

impl Platform {
    pub const fn current() -> Platform {
        if cfg!(target_os = "macos") {
            Platform::MacOs
        } else if cfg!(target_os = "windows") {
            Platform::Windows
        } else {
            Platform::Other
        }
    }

    /// What this platform prints each modifier as.
    ///
    /// A Mac has no Alt key. The one beside Command is Option, and a shortcut
    /// shown as `alt+left` sends people looking for a key that is not on the
    /// keyboard in front of them.
    pub const fn modifier_names(self) -> [(Modifiers, &'static str); 4] {
        let (command, option) = match self {
            Platform::MacOs => ("cmd", "option"),
            Platform::Windows => ("win", "alt"),
            Platform::Other => ("super", "alt"),
        };
        [
            (Modifiers::SUPER, command),
            (Modifiers::CTRL, "ctrl"),
            (Modifiers::ALT, option),
            (Modifiers::SHIFT, "shift"),
        ]
    }
}
