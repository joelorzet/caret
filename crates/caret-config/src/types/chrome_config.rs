#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ChromeConfig {
    pub tab_bar: bool,
    pub status: bool,
    /// Hide the tab bar while only one tab is open.
    pub hide_tab_bar_when_single: bool,
    /// Show the grid's memory use in the status line. Diagnostics, off by
    /// default.
    pub show_memory: bool,
    /// Columns either side of the pane area.
    pub padding: u16,
}
