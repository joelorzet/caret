#[allow(unused_imports)]
use crate::prelude::*;

/// The smallest window worth opening: below this the chrome owns the screen.
const MIN_COLS: u16 = 20;
const MIN_ROWS: u16 = 5;

/// Asking more than once an hour is pestering; a month apart is as good as
/// never, and anything further out may as well be off.
const MIN_CHECK_HOURS: u32 = 1;
const MAX_CHECK_HOURS: u32 = 720;

impl Config {
    /// Whether this configuration wants caret to look for a newer release.
    pub fn check_updates(&self) -> bool {
        self.update.check
    }

    pub(crate) fn apply(&mut self, raw: RawConfig, warn: &mut Vec<String>) {
        if let Some(theme) = raw.theme {
            self.theme = theme;
        }

        if let Some(f) = raw.font {
            set(&mut self.font.family, f.family);
            set(&mut self.font.fallback, f.fallback);
            if let Some(size) = f.size {
                if size > 0.0 {
                    self.font.size = size.clamp(4.0, 200.0);
                } else {
                    warn.push("font.size must be positive".into());
                }
            }
            if let Some(lh) = f.line_height {
                self.font.line_height = lh.clamp(0.5, 4.0);
            }
            set(&mut self.font.bold_is_bright, f.bold_is_bright);
            set(&mut self.font.ligatures, f.ligatures);
            set(&mut self.font.italics, f.italics);
            set(&mut self.font.builtin_box_drawing, f.builtin_box_drawing);
            if let Some(spacing) = f.letter_spacing {
                self.font.letter_spacing = spacing.clamp(-4.0, 16.0);
            }
        }

        if let Some(w) = raw.window {
            if let Some(o) = w.opacity {
                self.window.opacity = o.clamp(0.1, 1.0);
            }
            set(&mut self.window.padding, w.padding);
            set(&mut self.window.decorations, w.decorations);
            if let Some(c) = w.cols {
                self.window.cols = c.max(MIN_COLS);
            }
            if let Some(r) = w.rows {
                self.window.rows = r.max(MIN_ROWS);
            }
        }

        if let Some(t) = raw.terminal {
            set(&mut self.terminal.scrollback, t.scrollback);
            if let Some(shell) = t.shell {
                self.terminal.shell = Some(shell).filter(|s| !s.is_empty());
            }
            set(&mut self.terminal.shell_args, t.shell_args);
            if let Some(name) = t.cursor {
                match CursorShape::parse(&name) {
                    Some(shape) => self.terminal.cursor = shape,
                    None => warn.push(format!(
                        "terminal.cursor {name:?} is not one of {}",
                        CursorShape::accepted().join(", ")
                    )),
                }
            }
            // The older boolean still works; the named form wins when both
            // are present, because it can say more.
            if let Some(blink) = t.cursor_blink {
                self.terminal.cursor_fill = match blink {
                    true => CursorFill::Blink,
                    false => CursorFill::Solid,
                };
            }
            if let Some(name) = t.cursor_fill {
                match CursorFill::parse(&name) {
                    Some(fill) => self.terminal.cursor_fill = fill,
                    None => warn.push(format!(
                        "terminal.cursor_fill {name:?} is not one of {}",
                        CursorFill::accepted().join(", ")
                    )),
                }
            }
            if let Some(value) = t.cursor_color {
                match crate::impls::hex::parse_hex(&value) {
                    Some(rgb) => self.terminal.cursor_color = Some(rgb),
                    None => warn.push(format!(
                        "terminal.cursor_color {value:?} is not a hex colour"
                    )),
                }
            }
            if let Some(name) = t.bell {
                match BellStyle::parse(&name) {
                    Some(style) => self.terminal.bell = style,
                    None => warn.push(format!(
                        "terminal.bell {name:?} is not one of {}",
                        BellStyle::accepted().join(", ")
                    )),
                }
            }
            set(&mut self.terminal.word_separators, t.word_separators);
            set(&mut self.terminal.confirm_close, t.confirm_close);
            set(&mut self.terminal.highlight_input, t.highlight_input);
            set(&mut self.terminal.clipboard_write, t.clipboard_write);
            set(&mut self.terminal.mouse, t.mouse);
        }

        if let Some(c) = raw.chrome {
            set(&mut self.chrome.tab_bar, c.tab_bar);
            set(&mut self.chrome.status, c.status);
            set(
                &mut self.chrome.hide_tab_bar_when_single,
                c.hide_tab_bar_when_single,
            );
            set(&mut self.chrome.show_memory, c.show_memory);
            if let Some(padding) = c.padding {
                self.chrome.padding = padding.min(8);
            }
        }

        if let Some(u) = raw.update {
            set(&mut self.update.check, u.check);
            if let Some(hours) = u.interval_hours {
                self.update.interval_hours = hours.clamp(MIN_CHECK_HOURS, MAX_CHECK_HOURS);
            }
        }

        if let Some(s) = raw.sidebar {
            if let Some(e) = s.explorer {
                set(&mut self.sidebars.explorer.visible, e.visible);
                set(&mut self.sidebars.explorer.show_hidden, e.show_hidden);
                set(&mut self.sidebars.explorer.follow_cwd, e.follow_cwd);
                if let Some(w) = e.width {
                    self.sidebars.explorer.width = w.clamp(Dock::MIN_CELLS, Dock::MAX_CELLS);
                }
            }
            if let Some(a) = s.agent {
                set(&mut self.sidebars.agent.visible, a.visible);
                set(&mut self.sidebars.agent.args, a.args);
                if let Some(cmd) = a.command {
                    if cmd.trim().is_empty() {
                        warn.push("sidebar.agent.command must not be empty".into());
                    } else {
                        self.sidebars.agent.command = cmd;
                    }
                }
                if let Some(w) = a.width {
                    self.sidebars.agent.width = w.clamp(Dock::MIN_CELLS, Dock::MAX_CELLS);
                }
            }
        }

        if let Some(keys) = raw.keys {
            self.apply_keys(keys, warn);
        }
        if let Some(layouts) = raw.layouts {
            for (name, raw) in layouts {
                self.apply_layout(name, raw, warn);
            }
        }
    }

    /// Adds or replaces a saved layout.
    ///
    /// A layout is either a preset with an optional pane count, or an explicit
    /// shape; asking for both is ambiguous, so it is reported rather than
    /// silently resolved.
    fn apply_layout(&mut self, name: String, raw: RawLayout, warn: &mut Vec<String>) {
        let description = raw.description.unwrap_or_default();

        let shape = match (raw.preset, raw.split) {
            (Some(_), Some(_)) => {
                warn.push(format!(
                    "layouts.{name}: set either preset or split, not both"
                ));
                return;
            }
            (Some(preset), None) => match LayoutPreset::parse(&preset) {
                Some(preset) => LayoutShape::Preset {
                    preset,
                    panes: raw.panes,
                    main: raw.main.unwrap_or(50.0),
                },
                None => {
                    warn.push(format!(
                        "layouts.{name}: preset {preset:?} is not one of {}",
                        LayoutPreset::accepted().join(", ")
                    ));
                    return;
                }
            },
            (None, Some(split)) => match LayoutNode::parse(&split) {
                Ok(node) => LayoutShape::Explicit(node),
                Err(e) => {
                    warn.push(format!("layouts.{name}: {e}"));
                    return;
                }
            },
            (None, None) => {
                warn.push(format!("layouts.{name}: needs either preset or split"));
                return;
            }
        };

        let template = LayoutTemplate::new(name.clone(), description, shape);
        match self.layouts.iter_mut().find(|t| t.name == name) {
            Some(existing) => *existing = template,
            None => self.layouts.push(template),
        }
    }

    fn apply_keys(&mut self, raw: RawKeys, warn: &mut Vec<String>) {
        if raw.defaults == Some(false) {
            self.keymap = Keymap::empty();
        }

        if let Some(prefix) = raw.prefix {
            let trimmed = prefix.trim();
            if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("none") {
                self.keymap.set_prefix(None);
            } else {
                match KeyBinding::parse(trimmed) {
                    Ok(binding) => self.keymap.set_prefix(Some(binding)),
                    Err(e) => warn.push(format!("keys.prefix: {e}")),
                }
            }
        }

        for spec in raw.unbind.into_iter().flatten() {
            match KeyBinding::parse(&spec) {
                Ok(binding) => {
                    self.keymap.unbind(binding, false);
                    self.keymap.unbind(binding, true);
                }
                Err(e) => warn.push(format!("keys.unbind: {e}")),
            }
        }

        for (table, prefixed) in [(raw.direct, false), (raw.prefixed, true)] {
            for (key, action) in table.into_iter().flatten() {
                self.bind_from_config(&key, &action, prefixed, warn);
            }
        }
    }

    fn bind_from_config(
        &mut self,
        key: &str,
        action: &str,
        prefixed: bool,
        warn: &mut Vec<String>,
    ) {
        let table = if prefixed {
            "keys.prefixed"
        } else {
            "keys.direct"
        };
        let binding = match KeyBinding::parse(key) {
            Ok(b) => b,
            Err(e) => {
                warn.push(format!("{table}: {e}"));
                return;
            }
        };

        let name = action.trim();
        if name.is_empty() || name.eq_ignore_ascii_case("none") {
            self.keymap.unbind(binding, prefixed);
            return;
        }

        match Action::from_name(name) {
            Some(a) => self.keymap.bind(binding, a, prefixed),
            None => warn.push(format!("{table}: unknown action {name:?} for {key:?}")),
        }
    }
}

fn set<T>(slot: &mut T, value: Option<T>) {
    if let Some(v) = value {
        *slot = v;
    }
}
