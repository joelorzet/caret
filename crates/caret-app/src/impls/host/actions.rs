#[allow(unused_imports)]
use crate::prelude::*;

use std::io;

impl Session {
    pub(crate) fn perform(&mut self, action: Action) -> io::Result<()> {
        match action {
            Action::SplitPane(axis) => self.split(axis)?,
            Action::ClosePane => {
                let focus = self.workspace.active_pane();
                self.drop_pane(focus);
                self.close_pane(focus);
            }
            Action::FocusPane(direction) => {
                // Directional focus also leaves a sidebar, which is the
                // natural way back to the terminal.
                if self.focus != FocusTarget::Panes {
                    self.focus = FocusTarget::Panes;
                    return Ok(());
                }
                let area = self.center_rect();
                self.workspace.active_tab_mut().focus_dir(area, direction);
            }
            Action::ResizePane(direction, cells) => {
                let area = self.center_rect();
                self.workspace
                    .active_tab_mut()
                    .resize(area, direction, cells);
            }
            Action::ToggleZoom => {
                self.workspace.active_tab_mut().toggle_zoom();
            }
            Action::EqualizePanes => self.workspace.active_tab_mut().equalize(),
            Action::FlipSplit => {
                if !self.workspace.active_tab_mut().flip_split() {
                    self.message = " nothing to flip; split the pane first".into();
                }
            }
            Action::JoinPane(direction) => {
                let area = self.center_rect();
                if !self.workspace.active_tab_mut().join_pane(area, direction) {
                    self.message = format!(" no pane to the {} to join", dir_word(direction));
                }
            }
            Action::Rearrange => self.open_rearrange(),
            Action::RotatePanes(forward) => {
                self.workspace.active_tab_mut().rotate_panes(forward);
            }
            Action::NextPane => {
                self.workspace.active_tab_mut().focus_next();
            }
            Action::PrevPane => {
                self.workspace.active_tab_mut().focus_prev();
            }
            Action::LastPane => {
                self.workspace.active_tab_mut().focus_last();
            }
            Action::FocusPaneIndex(n) => {
                // The number the arrange panel shows, so naming a pane means
                // the same thing wherever you name it from.
                let target = self
                    .pane_numbered(n as usize)
                    .filter(|id| self.workspace.active_tab().panes().contains(id));
                match target {
                    Some(id) => {
                        self.workspace.active_tab_mut().set_focus(id);
                    }
                    None => self.message = format!(" no pane {n}"),
                }
            }

            Action::ApplyLayout(name) => match self.config.layout(&name).cloned() {
                Some(template) => {
                    self.message = format!(" layout {}", template.name);
                    self.apply_layout(&template)?;
                }
                None => self.message = format!(" no layout {name:?}"),
            },
            Action::NextLayout => {
                self.layout_cycle = (self.layout_cycle + 1) % LayoutPreset::ALL.len();
                let preset = LayoutPreset::ALL[self.layout_cycle];
                let template = LayoutTemplate::new(
                    preset.name(),
                    preset.description(),
                    LayoutShape::Preset {
                        preset,
                        panes: None,
                        main: 50.0,
                    },
                );
                self.message = format!(" layout {}", preset.name());
                self.apply_layout(&template)?;
            }

            Action::RenameTab => self.open_rename(),
            Action::NewTab => {
                // Read before the tab exists: opening one moves the focus to
                // its pane, and then there is nothing left to inherit from.
                let cwd = self.inherited_cwd();
                let (_, pane) = self.workspace.new_tab();
                let area = self.center_rect();
                self.spawn_shell(pane, cwd, area.w, area.h)?;
            }
            Action::CloseTab => {
                let panes = self.workspace.active_tab().panes();
                for pane in &panes {
                    self.drop_pane(*pane);
                }
                let id = self.workspace.active_tab().id();
                if self.workspace.close_tab(id) == CloseOutcome::Empty {
                    self.running = false;
                }
            }
            Action::NextTab => self.workspace.next_tab(),
            Action::PrevTab => self.workspace.prev_tab(),
            Action::SelectTab(n) => {
                if !self.workspace.select_tab(n.saturating_sub(1) as usize) {
                    self.message = format!(" no tab {n}");
                }
            }

            Action::ToggleSidebar(kind) => self.toggle_sidebar(kind)?,
            Action::FocusSidebar(kind) => self.focus_sidebar(kind)?,
            Action::ResizeSidebar(kind, delta) => {
                let dock = self.dock_mut(kind);
                let next = dock.cells as i32 + delta as i32;
                dock.resize(next.clamp(0, u16::MAX as i32) as u16);
            }

            Action::ScrollLines(lines) => self.scroll(lines),
            Action::ScrollPage(pages) => {
                let rows = self
                    .pane(self.workspace.active_pane())
                    .map(|p| p.terminal.rows().saturating_sub(1))
                    .unwrap_or(1) as i32;
                self.scroll(pages * rows.max(1));
            }
            Action::ScrollToBottom => {
                if let Some(pane) = self.focused_pane_mut() {
                    pane.terminal.screen_mut().scroll_to_bottom();
                }
            }

            Action::Copy => match self.selection_text() {
                Some(text) if !text.is_empty() => {
                    let count = text.chars().count();
                    self.message = match clipboard::copy(&text, self.native_window) {
                        Ok(()) => format!(" copied {count} characters"),
                        Err(e) => format!(" copy failed: {e}"),
                    };
                }
                _ => self.message = " nothing selected".into(),
            },
            // In a window of our own the clipboard is ours to read. Inside
            // another terminal it is not: that terminal's own paste shortcut
            // sends the text here as input, and is handled on arrival.
            Action::Paste => match clipboard::paste(self.native_window) {
                Some(text) if !text.is_empty() => self.on_paste(text.into_bytes()),
                Some(_) => self.message = " clipboard is empty".into(),
                None => self.message = " paste with your terminal's own shortcut".into(),
            },

            Action::Search => {
                let previous = self.search.as_ref().map(|s| s.query.clone());
                self.open_search(previous);
            }
            Action::ShowShortcuts => self.open_palette(PaletteMode::Commands),
            Action::ShowThemes => self.open_palette(PaletteMode::Themes),
            Action::ShowSettings => self.open_palette(PaletteMode::Settings),
            Action::EditConfig => self.edit_config()?,
            Action::ReloadConfig => self.reload_config(),
            Action::CheckUpdates => {
                self.message = " looking for a newer release...".into();
                self.begin_update_check(true);
            }
            Action::Quit => self.running = false,
        }
        Ok(())
    }

    fn toggle_sidebar(&mut self, kind: DockKind) -> io::Result<()> {
        let shown = match kind {
            DockKind::Explorer => self.chrome.left.toggle(),
            DockKind::Agent => self.chrome.right.toggle(),
        };

        if !shown {
            if self.focus == focus_for(kind) {
                self.focus = FocusTarget::Panes;
            }
            return Ok(());
        }

        // The layout drops a dock it cannot fit, so say so instead of
        // appearing to do nothing.
        if self.chrome_hidden(kind) {
            self.dock_mut(kind).visible = false;
            self.message = " window too narrow for that sidebar".into();
            return Ok(());
        }

        if kind == DockKind::Agent {
            self.ensure_agent()?;
        }
        self.focus = focus_for(kind);
        Ok(())
    }

    /// Moves focus into a sidebar, revealing it first if it is hidden.
    fn focus_sidebar(&mut self, kind: DockKind) -> io::Result<()> {
        if !self.dock_mut(kind).visible {
            return self.toggle_sidebar(kind);
        }
        if self.chrome_hidden(kind) {
            self.message = " window too narrow for that sidebar".into();
            return Ok(());
        }
        if kind == DockKind::Agent {
            self.ensure_agent()?;
        }
        self.focus = focus_for(kind);
        Ok(())
    }

    pub(crate) fn dock_mut(&mut self, kind: DockKind) -> &mut Dock {
        match kind {
            DockKind::Explorer => &mut self.chrome.left,
            DockKind::Agent => &mut self.chrome.right,
        }
    }

    /// Rearranges the tab, opening shells for panes the layout adds and
    /// closing those it drops.
    fn apply_layout(&mut self, template: &LayoutTemplate) -> io::Result<()> {
        let change = self.workspace.apply_layout(template);
        for id in &change.removed {
            self.drop_pane(*id);
        }

        let area = self.center_rect();
        let resolved = self.workspace.active_tab().resolve(area);
        for id in &change.created {
            let rect = resolved
                .panes
                .iter()
                .find(|(pane, _)| pane == id)
                .map(|(_, rect)| *rect)
                .unwrap_or(area);
            self.spawn_pane(*id, rect.w, rect.h)?;
        }
        Ok(())
    }

    fn chrome_hidden(&self, kind: DockKind) -> bool {
        let frame = self.chrome.layout(self.window_rect());
        match kind {
            DockKind::Explorer => frame.left.is_none(),
            DockKind::Agent => frame.right.is_none(),
        }
    }

    fn focused_pane_mut(&mut self) -> Option<&mut Pane> {
        let focus = self.workspace.active_pane();
        self.panes
            .iter_mut()
            .find(|(id, _)| *id == focus)
            .map(|(_, pane)| pane)
    }

    fn scroll(&mut self, lines: i32) {
        if let Some(pane) = self.focused_pane_mut() {
            // Positive scrolls back into history, matching the wheel.
            pane.terminal.screen_mut().scroll_display(-lines);
        }
    }

    fn split(&mut self, axis: Axis) -> io::Result<()> {
        let area = self.center_rect();
        let cwd = self.inherited_cwd();
        match self.workspace.split(axis, false) {
            Some(new_pane) => {
                let rect = self
                    .workspace
                    .active_tab()
                    .resolve(area)
                    .panes
                    .iter()
                    .find(|(id, _)| *id == new_pane)
                    .map(|(_, r)| *r)
                    .unwrap_or(area);
                trace(&format!("spawning pane {} at {rect:?}", new_pane.0));
                self.spawn_shell(new_pane, cwd, rect.w, rect.h)?;
            }
            None => self.message = " cannot split any further".into(),
        }
        Ok(())
    }

    /// Removes a pane from the layout and shuts the window down when the last
    /// one goes.
    pub(crate) fn close_pane(&mut self, pane: PaneId) {
        match self.workspace.close_pane(pane) {
            CloseOutcome::Empty => self.running = false,
            CloseOutcome::Tab(_) | CloseOutcome::Pane(_) | CloseOutcome::NotFound => {}
        }
        self.needs_draw = true;
    }

    pub(crate) fn reload_config(&mut self) {
        let loaded = Config::load();
        let (themes, theme_warnings) = ThemeRegistry::load();
        self.themes = themes;
        self.apply_theme(self.themes.resolve(&loaded.config.theme).clone());
        self.chrome = loaded.config.chrome();
        if self.pixel_chrome {
            self.chrome.tab_bar = 0;
        }
        self.config = loaded.config;

        for (_, pane) in self.panes.iter_mut() {
            pane.terminal.screen_mut().damage_all();
        }

        let problems = loaded.warnings.len() + theme_warnings.len();
        self.message = if problems == 0 {
            " config reloaded".into()
        } else {
            format!(" config reloaded with {problems} problem(s)")
        };
    }
}

fn focus_for(kind: DockKind) -> FocusTarget {
    match kind {
        DockKind::Explorer => FocusTarget::Explorer,
        DockKind::Agent => FocusTarget::Agent,
    }
}

/// The direction as the status line says it.
fn dir_word(dir: Direction) -> &'static str {
    match dir {
        Direction::Left => "left",
        Direction::Right => "right",
        Direction::Up => "up",
        Direction::Down => "down",
    }
}
