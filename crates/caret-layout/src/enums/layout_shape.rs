#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum LayoutShape {
    /// `panes: None` means "however many the tab already has", which makes a
    /// preset rearrange what is open instead of creating panes.
    Preset {
        preset: LayoutPreset,
        panes: Option<usize>,
        /// Percentage given to the main pane.
        main: f32,
    },
    Explicit(LayoutNode),
}
