#[allow(unused_imports)]
use crate::prelude::*;

/// Ready-made arrangements, named after tmux's.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LayoutPreset {
    /// Panes side by side in equal columns.
    EvenHorizontal,
    /// Panes stacked in equal rows.
    EvenVertical,
    /// One large pane on the left, the rest stacked on the right.
    MainVertical,
    /// One large pane on top, the rest side by side below.
    MainHorizontal,
    /// A grid, as square as the count allows.
    Tiled,
}
