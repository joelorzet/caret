#[allow(unused_imports)]
use crate::prelude::*;

impl TabItem {
    /// How many terminals this tab holds, when it holds more than one.
    ///
    /// It matters most while a pane is zoomed: the others are hidden, and
    /// without a count there is nothing to say they are still there.
    pub fn badge(&self) -> Option<String> {
        if self.panes < 2 {
            return None;
        }
        Some(match self.zoomed {
            true => format!("1/{}", self.panes),
            false => self.panes.to_string(),
        })
    }

    /// Characters the tab has to show: its name, its badge, and its number
    /// when the strip is showing numbers at all.
    pub fn columns(&self, index: usize, numbered: bool) -> u32 {
        let number = if numbered {
            (index + 1).to_string().chars().count() + 1
        } else {
            0
        };
        let badge = self.badge().map_or(0, |b| b.chars().count() + 2);
        (number + self.title.chars().count() + badge) as u32
    }
}
