#[allow(unused_imports)]
use crate::prelude::*;

impl ActionGroup {
    pub const ALL: [ActionGroup; 7] = [
        ActionGroup::Panes,
        ActionGroup::Layouts,
        ActionGroup::Tabs,
        ActionGroup::Sidebars,
        ActionGroup::Scrollback,
        ActionGroup::Clipboard,
        ActionGroup::General,
    ];

    pub const fn title(self) -> &'static str {
        match self {
            ActionGroup::Panes => "Panes",
            ActionGroup::Layouts => "Layouts",
            ActionGroup::Tabs => "Tabs",
            ActionGroup::Sidebars => "Sidebars",
            ActionGroup::Scrollback => "Scrollback",
            ActionGroup::Clipboard => "Clipboard",
            ActionGroup::General => "General",
        }
    }
}

impl core::fmt::Display for ActionGroup {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.title())
    }
}
