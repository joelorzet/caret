#[allow(unused_imports)]
use crate::prelude::*;

impl Terminal {
    pub fn new(cols: u16, rows: u16, scrollback: u32) -> Terminal {
        Terminal {
            primary: Screen::new(cols, rows, scrollback),
            alt: Screen::new(cols, rows, 0),
            on_alt: false,
            modes: Modes::default(),
            tabs: TabStops::new(cols),
            cursor_style: CursorStyle::default(),
            title: String::new(),
            cwd: None,
            reply: Vec::new(),
            events: Vec::new(),
        }
    }

    #[inline]
    pub fn screen(&self) -> &Screen {
        if self.on_alt {
            &self.alt
        } else {
            &self.primary
        }
    }

    #[inline]
    pub fn screen_mut(&mut self) -> &mut Screen {
        if self.on_alt {
            &mut self.alt
        } else {
            &mut self.primary
        }
    }

    #[inline]
    pub fn is_alt(&self) -> bool {
        self.on_alt
    }

    pub fn enter_alt_screen(&mut self) {
        if self.on_alt {
            return;
        }
        self.primary.save_cursor();
        self.on_alt = true;
        self.modes.insert(Modes::ALT_SCREEN);
        self.alt.reset();
        self.alt.damage_all();
    }

    pub fn exit_alt_screen(&mut self) {
        if !self.on_alt {
            return;
        }
        self.on_alt = false;
        self.modes.remove(Modes::ALT_SCREEN);
        self.primary.restore_cursor();
        self.primary.damage_all();
    }

    #[inline]
    pub fn modes(&self) -> Modes {
        self.modes
    }

    pub fn set_mode(&mut self, m: Modes, on: bool) {
        self.modes.set(m, on);
    }

    #[inline]
    pub fn wrap_enabled(&self) -> bool {
        self.modes.contains(Modes::WRAP)
    }

    #[inline]
    pub fn insert_mode(&self) -> bool {
        self.modes.contains(Modes::INSERT)
    }

    #[inline]
    pub fn tabs(&self) -> &TabStops {
        &self.tabs
    }

    #[inline]
    pub fn tabs_mut(&mut self) -> &mut TabStops {
        &mut self.tabs
    }

    #[inline]
    pub fn cursor_style(&self) -> CursorStyle {
        self.cursor_style
    }

    pub fn set_cursor_style(&mut self, style: CursorStyle) {
        self.cursor_style = style;
    }

    #[inline]
    pub fn cursor_visible(&self) -> bool {
        self.modes.contains(Modes::CURSOR_VISIBLE)
    }

    #[inline]
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        let title = title.into();
        if title != self.title {
            self.title = title;
            self.events.push(TerminalEvent::TitleChanged);
        }
    }

    pub fn cwd(&self) -> Option<&str> {
        self.cwd.as_deref()
    }

    pub fn set_cwd(&mut self, cwd: impl Into<String>) {
        let cwd = cwd.into();
        if self.cwd.as_deref() != Some(cwd.as_str()) {
            self.cwd = Some(cwd);
            self.events.push(TerminalEvent::CwdChanged);
        }
    }

    pub fn reply(&mut self, bytes: &[u8]) {
        self.reply.extend_from_slice(bytes);
    }

    pub fn take_reply(&mut self) -> Vec<u8> {
        core::mem::take(&mut self.reply)
    }

    pub fn has_reply(&self) -> bool {
        !self.reply.is_empty()
    }

    pub fn push_event(&mut self, event: TerminalEvent) {
        self.events.push(event);
    }

    pub fn drain_events(&mut self) -> impl Iterator<Item = TerminalEvent> + '_ {
        self.events.drain(..)
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.primary.resize(cols, rows);
        self.alt.resize(cols, rows);
        self.tabs.resize(cols);
    }

    #[inline]
    pub fn cols(&self) -> u16 {
        self.screen().cols()
    }

    #[inline]
    pub fn rows(&self) -> u16 {
        self.screen().rows()
    }

    pub fn reset(&mut self) {
        self.exit_alt_screen();
        self.primary.reset();
        self.alt.reset();
        self.modes = Modes::default();
        self.tabs.reset();
        self.cursor_style = CursorStyle::default();
        self.title.clear();
        self.cwd = None;
        self.reply.clear();
    }

    pub fn memory_bytes(&self) -> usize {
        self.primary.memory_bytes() + self.alt.memory_bytes()
    }
}
