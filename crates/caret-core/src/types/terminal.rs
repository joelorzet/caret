#[allow(unused_imports)]
use crate::prelude::*;

pub struct Terminal {
    pub(crate) primary: Screen,
    /// No scrollback: a pager's redraws must not fill history.
    pub(crate) alt: Screen,
    pub(crate) on_alt: bool,

    pub(crate) modes: Modes,
    pub(crate) tabs: TabStops,
    pub(crate) cursor_style: CursorStyle,
    pub(crate) title: String,
    /// Reported by the shell through OSC 7.
    pub(crate) cwd: Option<String>,

    pub(crate) reply: Vec<u8>,
    pub(crate) events: Vec<TerminalEvent>,
}
