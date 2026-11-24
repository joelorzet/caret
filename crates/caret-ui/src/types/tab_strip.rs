#[allow(unused_imports)]
use crate::prelude::*;

/// The tab bar, as geometry rather than as cells.
pub struct TabStrip {
    pub tabs: Vec<TabItem>,
    pub active: usize,
    /// Space on the left that something else owns: the window controls.
    pub inset_left: u32,
    pub height: u32,
    /// One character of the terminal's font, which the chrome shares.
    pub cell_width: u32,
    /// Device pixels per logical pixel, so the metrics below hold on any
    /// display.
    pub scale: f32,
}
