#[allow(unused_imports)]
use crate::prelude::*;

/// Something a keypress can do.
///
/// Not `Copy`: a layout is referred to by name, and names come from the
/// config file rather than from a fixed set.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Action {
    SplitPane(Axis),
    ClosePane,
    FocusPane(Direction),
    /// Moves the focused pane's border, in cells.
    ResizePane(Direction, u16),
    ToggleZoom,
    EqualizePanes,
    /// Turn the split holding the focused pane the other way.
    FlipSplit,
    /// Pair the focused pane with its neighbour in this direction, stacking
    /// them if they sit side by side and vice versa. The program keeps
    /// running: the pane moves, it is not reopened.
    JoinPane(Direction),
    /// Move every pane one position along, keeping the arrangement.
    RotatePanes(bool),
    /// Arrange the panes by hand, with the layout itself as the preview.
    Rearrange,
    FocusPaneIndex(u8),
    NextPane,
    PrevPane,
    /// Back to the pane focused before this one.
    LastPane,

    /// Rearrange the tab to a saved layout.
    ApplyLayout(Box<str>),
    /// Step through the built-in layout presets.
    NextLayout,

    NewTab,
    /// Give the current tab a name of your own.
    RenameTab,
    CloseTab,
    NextTab,
    PrevTab,
    SelectTab(u8),

    ToggleSidebar(DockKind),
    FocusSidebar(DockKind),
    /// Widens (positive) or narrows (negative) a sidebar, in cells.
    ResizeSidebar(DockKind, i16),

    ScrollLines(i32),
    ScrollPage(i32),
    ScrollToBottom,
    /// Open the scrollback search prompt.
    Search,

    Copy,
    Paste,

    ShowShortcuts,
    /// Open the theme picker.
    ShowThemes,
    /// Open the settings list: every setting and what it is set to.
    ShowSettings,
    /// Open the config file itself, which is where a change is made.
    EditConfig,
    ReloadConfig,
    /// Scroll to the prompt before this one.
    PreviousCommand,
    /// Scroll to the prompt after this one.
    NextCommand,
    /// Ask whether a newer release has been published.
    CheckUpdates,
    /// Fetch and install it.
    InstallUpdate,
    Quit,
}
