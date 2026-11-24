#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ExplorerConfig {
    pub visible: bool,
    pub width: u16,
    pub show_hidden: bool,
    /// Follow the focused pane's working directory.
    pub follow_cwd: bool,
}
