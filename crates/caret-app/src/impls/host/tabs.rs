#[allow(unused_imports)]
use crate::prelude::*;

use caret_ui::{StripHit, TabItem};

impl Session {
    /// The tab strip as data, for a backend that draws its own chrome.
    pub fn tab_items(&self) -> Vec<TabItem> {
        (0..self.workspace.tabs().len())
            .map(|index| {
                let tab = &self.workspace.tabs()[index];
                TabItem {
                    title: self.tab_label(index),
                    panes: tab.pane_count() as u16,
                    zoomed: tab.zoomed().is_some(),
                }
            })
            .collect()
    }

    pub fn active_tab_index(&self) -> usize {
        self.workspace.active_index()
    }

    /// Hands the tab bar to the backend, which is only worth doing when it can
    /// draw in pixels; the row the grid would have spent on it goes back to
    /// the panes.
    pub fn take_over_tab_bar(&mut self) {
        self.pixel_chrome = true;
        self.chrome.tab_bar = 0;
    }

    /// Closing and opening go through the same actions a key binding uses, so
    /// a click cannot take a different path than ctrl+b c.
    pub fn on_strip_hit(&mut self, hit: StripHit) -> std::io::Result<()> {
        match hit {
            StripHit::Tab(index) => {
                self.workspace.select_tab(index);
                self.focus = FocusTarget::Panes;
            }
            StripHit::Close(index) => {
                if !self.workspace.select_tab(index) {
                    return Ok(());
                }
                self.perform(Action::CloseTab)?;
            }
            StripHit::NewTab => self.perform(Action::NewTab)?,
            StripHit::Background => return Ok(()),
        }
        self.needs_draw = true;
        Ok(())
    }
}
