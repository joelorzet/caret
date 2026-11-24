//! The tab bar, for the backend that draws it in cells.
//!
//! Part of the drawing layer: one region of the window per file, so a
//! change to one of them is a change to one file.

#[allow(unused_imports)]
use super::*;

impl Session {
    /// Where each tab sits in the bar. Shared with hit testing so a click
    /// cannot disagree with what was drawn.
    pub(crate) fn tab_bar_slots(&self, rect: Rect) -> Vec<(usize, Rect)> {
        let mut out = Vec::new();
        let mut x = rect.x + GUTTER;
        for index in 0..self.workspace.tabs().len() {
            if x >= rect.right() {
                break;
            }
            let width = (self.tab_width(index) as u16).min(rect.right() - x);
            out.push((index, Rect::new(x, rect.y, width, rect.h)));
            x += width;
        }
        out
    }

    fn tab_width(&self, index: usize) -> usize {
        let count = self
            .tab_count_text(index)
            .map_or(0, |text| text.chars().count() + 1);
        TAB_PAD * 2
            + (index + 1).to_string().chars().count()
            + 1
            + self.tab_label(index).chars().count()
            + count
    }

    /// How many terminals a tab holds, when it holds more than one.
    ///
    /// This matters most while a pane is zoomed: the others are hidden, and
    /// without a count there is nothing to say they are still there.
    fn tab_count_text(&self, index: usize) -> Option<String> {
        let tab = self.workspace.tabs().get(index)?;
        let panes = tab.pane_count();
        if panes < 2 {
            return None;
        }
        Some(match tab.zoomed() {
            Some(_) => format!("1/{panes}"),
            None => panes.to_string(),
        })
    }

    /// What a tab is called.
    ///
    /// The working directory, because that is what distinguishes one tab from
    /// another. A shell commonly sets the title to `user@host`, which is
    /// identical in every tab and says nothing, so that form is rejected.
    pub(crate) fn tab_label(&self, index: usize) -> String {
        let fallback = self.shell_name();
        let Some(tab) = self.workspace.tabs().get(index) else {
            return fallback;
        };
        if !tab.title().is_empty() {
            return truncate(tab.title(), MAX_LABEL);
        }
        let Some(pane) = self.pane(tab.focus()) else {
            return fallback;
        };

        if let Some(name) = pane.terminal.cwd().and_then(directory_label) {
            return name;
        }
        let title = pane.terminal.title();
        if !title.is_empty() && !looks_like_host(title) {
            return truncate(title, MAX_LABEL);
        }
        fallback
    }

    pub(super) fn draw_tab_bar(&self, surface: &mut Surface, rect: Rect) {
        let ui = &self.theme.ui;
        surface.fill(rect, ui.chrome_bg);

        let mut end = rect.x;
        for (index, slot) in self.tab_bar_slots(rect) {
            let active = index == self.workspace.active_index();
            let number = (index + 1).to_string();
            let label = self.tab_label(index);

            // The active tab is a quiet chip, not a slab: a heavy block fights
            // the theme and reads as a different application.
            let bg = if active {
                ui.tab_active_bg
            } else {
                ui.chrome_bg
            };
            if active {
                surface.fill(slot, bg);
            }

            let (number_fg, label_fg, flags) = if active {
                (ui.accent, ui.tab_active_fg, Flags::BOLD)
            } else {
                (ui.tab_inactive_fg, ui.tab_inactive_fg, Flags::NONE)
            };

            let mut x = slot.x + TAB_PAD as u16;
            surface.text(
                x,
                slot.y,
                slot.right() - x,
                &number,
                Style {
                    fg: number_fg,
                    bg,
                    flags,
                },
            );
            x += number.chars().count() as u16 + 1;
            surface.text(
                x,
                slot.y,
                slot.right().saturating_sub(x),
                &label,
                Style {
                    fg: label_fg,
                    bg,
                    flags,
                },
            );
            if let Some(count) = self.tab_count_text(index) {
                let zoomed = self.workspace.tabs()[index].zoomed().is_some();
                let count_fg = if zoomed {
                    ui.accent
                } else {
                    ui.tab_inactive_fg
                };
                let at = x + label.chars().count() as u16 + 1;
                surface.text(
                    at,
                    slot.y,
                    slot.right().saturating_sub(at),
                    &count,
                    Style {
                        fg: count_fg,
                        bg,
                        flags: Flags::BOLD,
                    },
                );
            }
            end = slot.right();
        }

        let hint = if self.native_window {
            "cmd+/"
        } else {
            "ctrl+b ?"
        };
        let width = hint.chars().count() as u16 + GUTTER;
        if rect.right() > end + width {
            surface.text(
                rect.right() - width,
                rect.y,
                width,
                hint,
                Style::new(ui.tab_inactive_fg, ui.chrome_bg),
            );
        }
    }

    fn shell_name(&self) -> String {
        let program = self.config.shell_program();
        program.rsplit('/').next().unwrap_or(&program).to_string()
    }
}

/// Padding either side of a tab label.
const TAB_PAD: usize = 2;

/// Longest tab label before it is cut short.
const MAX_LABEL: usize = 18;

/// The last component of a path, or `~` for the home directory itself.
fn directory_label(cwd: &str) -> Option<String> {
    let path = std::path::Path::new(cwd);
    if let Some(home) = std::env::var_os("HOME") {
        if path == std::path::Path::new(&home) {
            return Some("~".to_string());
        }
    }
    match path.file_name() {
        Some(name) => Some(truncate(&name.to_string_lossy(), MAX_LABEL)),
        None => Some("/".to_string()),
    }
}

/// `user@host`, which shells set as the title and which is the same in every
/// tab.
fn looks_like_host(title: &str) -> bool {
    title.contains('@') && !title.contains(' ') && !title.contains('/')
}
