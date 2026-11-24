//! The status line, and the key hints other panels share.
//!
//! Part of the drawing layer: one region of the window per file, so a
//! change to one of them is a change to one file.

#[allow(unused_imports)]
use super::*;

impl Session {
    pub(super) fn draw_status(&self, surface: &mut Surface, rect: Rect) {
        let ui = &self.theme.ui;
        surface.fill(rect, ui.status_bg);

        // A dashed rule rather than the solid one the panes are divided by:
        // this separates furniture from content, which is a quieter job than
        // separating one terminal from another.
        let (rect, rule) = match rect.h {
            0 => return,
            1 => (rect, None),
            _ => (
                Rect::new(rect.x, rect.y + 1, rect.w, rect.h - 1),
                Some(Rect::new(rect.x, rect.y, rect.w, 1)),
            ),
        };
        if let Some(rule) = rule {
            // Unbroken, and a third of the way from the background to the
            // border: present enough to separate the line from the panes,
            // quiet enough not to be read as another divider.
            let colour = self.theme.bg.blend(ui.border, RULE_STRENGTH);
            surface.hline(
                Rect::new(
                    rule.x + GUTTER,
                    rule.y,
                    rule.w.saturating_sub(GUTTER * 2),
                    1,
                ),
                '\u{2500}',
                colour,
                ui.status_bg,
            );
        }

        let (left, left_accent) = self.status_left();
        let right = self.status_right();

        let right_width: u16 = right
            .iter()
            .map(|(text, _)| text.chars().count() as u16 + 2)
            .sum();

        let room = rect.w.saturating_sub(right_width + GUTTER * 2);
        let left_style = if left_accent {
            Style::new(ui.accent, ui.status_bg)
        } else {
            Style::new(ui.status_fg, ui.status_bg)
        };
        surface.text(
            rect.x + GUTTER,
            rect.y,
            room,
            &truncate(&left, room as usize),
            left_style,
        );

        let mut x = rect.right().saturating_sub(right_width + GUTTER);
        for (text, accent) in right {
            let style = if accent {
                Style::new(ui.accent, ui.status_bg).bold()
            } else {
                Style::new(ui.status_fg, ui.status_bg)
            };
            surface.text(x, rect.y, rect.right() - x, &text, style);
            x += text.chars().count() as u16 + 2;
        }
    }

    /// The left of the status line: whatever needs saying, else where you are.
    fn status_left(&self) -> (String, bool) {
        if let Some(prompt) = self.rename_prompt() {
            return (prompt, true);
        }
        if let Some(prompt) = self.search_prompt() {
            return (prompt, true);
        }
        if self.prefix_pending {
            return ("PREFIX".to_string(), true);
        }
        match self.focus {
            FocusTarget::Explorer => {
                return (
                    "explorer    p preview    n pane    N tab    . hidden".into(),
                    false,
                )
            }
            FocusTarget::Agent => {
                return (
                    format!("agent    {}", self.config.sidebars.agent.command),
                    false,
                )
            }
            FocusTarget::Panes => {}
        }
        if !self.message.is_empty() {
            return (self.message.trim().to_string(), true);
        }
        (self.working_directory(), false)
    }

    /// The right of the status line: only what is true right now.
    fn status_right(&self) -> Vec<(String, bool)> {
        let tab = self.workspace.active_tab();
        let mut out = Vec::new();

        // The agent runs whether or not its dock is open, so the one place
        // that is always on screen has to be where it says it wants you.
        if self.agent_alert {
            out.push(("\u{25cf} agent".to_string(), true));
        }
        if let Some(text) = self.selection_text() {
            let count = text.chars().count();
            if count > 0 {
                out.push((format!("{count} selected"), true));
            }
        }
        let scrolled = self
            .pane(tab.focus())
            .map(|p| p.terminal.screen().scroll_offset())
            .unwrap_or(0);
        if scrolled > 0 {
            out.push((format!("scrolled back {scrolled}"), true));
        }
        if tab.zoomed().is_some() {
            out.push((format!("zoom 1/{}", tab.pane_count()), true));
        }
        if tab.pane_count() > 1 {
            out.push((format!("{}/{}", tab.focus_index(), tab.pane_count()), false));
        }
        if self.config.chrome.show_memory {
            out.push((format_bytes(self.memory_bytes()), false));
        }
        out
    }

    fn working_directory(&self) -> String {
        let cwd = self
            .pane(self.workspace.active_pane())
            .and_then(|p| p.terminal.cwd())
            .map(str::to_string);
        match cwd {
            Some(path) => shorten_home(&path),
            None => String::new(),
        }
    }

    /// Key hints, as `key label` pairs.
    ///
    /// Reads better than a sentence strung together with separators: the key
    /// stands out in the accent colour and the word next to it says what it
    /// does.
    pub(super) fn draw_hints(
        &self,
        surface: &mut Surface,
        at: (u16, u16),
        width: u16,
        hints: &[(&str, &str)],
        bg: Rgb,
    ) {
        let ui = &self.theme.ui;
        let (mut x, y) = at;
        let limit = at.0 + width;

        // The hints sit on the panel's bottom edge, so the row is cleared
        // first: otherwise the border shows through the gaps between them
        // and reads as dashes strung between the words.
        surface.fill(Rect::new(at.0, y, width, 1), bg);

        for (key, label) in hints {
            let needed = (key.chars().count() + label.chars().count() + 3) as u16;
            if x + needed > limit {
                break;
            }
            surface.text(
                x,
                y,
                width,
                key,
                Style {
                    fg: ui.accent,
                    bg,
                    flags: Flags::BOLD,
                },
            );
            x += key.chars().count() as u16 + 1;
            surface.text(
                x,
                y,
                width,
                label,
                Style {
                    fg: ui.tab_inactive_fg,
                    bg,
                    flags: Flags::NONE,
                },
            );
            x += label.chars().count() as u16 + 2;
        }
    }
}

/// How far the status rule is drawn from the background toward the border.
const RULE_STRENGTH: f32 = 0.3;

fn shorten_home(path: &str) -> String {
    match std::env::var("HOME") {
        Ok(home) if !home.is_empty() && path.starts_with(&home) => {
            format!("~{}", &path[home.len()..])
        }
        _ => path.to_string(),
    }
}

fn format_bytes(bytes: usize) -> String {
    const MIB: f64 = 1024.0 * 1024.0;
    if bytes as f64 >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB)
    } else {
        format!("{} KiB", bytes / 1024)
    }
}
