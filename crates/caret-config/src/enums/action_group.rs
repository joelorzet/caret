#[allow(unused_imports)]
use crate::prelude::*;

/// How actions are bucketed in the shortcut list.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum ActionGroup {
    Panes,
    Layouts,
    Tabs,
    Sidebars,
    Scrollback,
    Clipboard,
    General,
}
