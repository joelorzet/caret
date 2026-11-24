#[allow(unused_imports)]
use crate::prelude::*;

impl Modes {
    pub const NONE: Modes = Modes(0);
    /// DECAWM: printing past the last column wraps.
    pub const WRAP: Modes = Modes(1 << 0);
    /// IRM: printed characters shift the rest of the line right.
    pub const INSERT: Modes = Modes(1 << 1);
    /// DECOM: cursor addressing is relative to the scroll region.
    pub const ORIGIN: Modes = Modes(1 << 2);
    /// DECCKM: arrow keys send SS3 rather than CSI.
    pub const APP_CURSOR: Modes = Modes(1 << 3);
    /// DECNKM: the keypad sends application sequences.
    pub const APP_KEYPAD: Modes = Modes(1 << 4);
    /// DECTCEM: the cursor is drawn.
    pub const CURSOR_VISIBLE: Modes = Modes(1 << 5);
    pub const ALT_SCREEN: Modes = Modes(1 << 6);
    /// DECSCNM: foreground and background swap screen-wide.
    pub const REVERSE_VIDEO: Modes = Modes(1 << 7);
    /// LNM: a line feed also carriage-returns.
    pub const LINE_FEED_NL: Modes = Modes(1 << 8);
    /// 2004: pasted text is delimited by CSI 200~ / 201~.
    pub const BRACKETED_PASTE: Modes = Modes(1 << 9);
    /// 1004: focus changes are reported.
    pub const FOCUS_EVENTS: Modes = Modes(1 << 10);
    /// 1000: button presses and releases are reported.
    pub const MOUSE_CLICK: Modes = Modes(1 << 11);
    /// 1002: motion is reported while a button is held.
    pub const MOUSE_DRAG: Modes = Modes(1 << 12);
    /// 1003: all motion is reported.
    pub const MOUSE_MOTION: Modes = Modes(1 << 13);
    /// 1006: mouse reports use the SGR encoding.
    pub const MOUSE_SGR: Modes = Modes(1 << 14);
    /// 1007: the wheel sends arrow keys on the alternate screen.
    pub const ALT_SCROLL: Modes = Modes(1 << 15);
    /// 2026: output is buffered until the application ends the frame.
    pub const SYNC_UPDATE: Modes = Modes(1 << 16);

    #[inline]
    pub const fn contains(self, other: Modes) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    pub const fn intersects(self, other: Modes) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn union(self, other: Modes) -> Modes {
        Modes(self.0 | other.0)
    }

    pub fn insert(&mut self, other: Modes) {
        self.0 |= other.0;
    }

    pub fn remove(&mut self, other: Modes) {
        self.0 &= !other.0;
    }

    pub fn set(&mut self, other: Modes, on: bool) {
        if on {
            self.insert(other)
        } else {
            self.remove(other)
        }
    }

    pub const fn mouse_reporting(self) -> bool {
        self.intersects(Modes(
            Modes::MOUSE_CLICK.0 | Modes::MOUSE_DRAG.0 | Modes::MOUSE_MOTION.0,
        ))
    }
}

impl Default for Modes {
    fn default() -> Self {
        Modes::WRAP.union(Modes::CURSOR_VISIBLE)
    }
}
