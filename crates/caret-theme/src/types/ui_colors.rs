#[allow(unused_imports)]
use crate::prelude::*;

/// Colors for the window furniture, derived from the terminal colors so a
/// theme only has to specify the palette and an accent.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UiColors {
    pub chrome_bg: Rgb,
    pub chrome_fg: Rgb,
    pub tab_active_bg: Rgb,
    pub tab_active_fg: Rgb,
    pub tab_inactive_fg: Rgb,
    pub border: Rgb,
    pub border_focused: Rgb,
    pub status_bg: Rgb,
    pub status_fg: Rgb,
    pub panel_bg: Rgb,
    pub panel_fg: Rgb,
    pub selection_ui: Rgb,
    pub accent: Rgb,
}
