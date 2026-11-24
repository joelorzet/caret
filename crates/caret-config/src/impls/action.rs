#[allow(unused_imports)]
use crate::prelude::*;

fn side(kind: DockKind) -> &'static str {
    match kind {
        DockKind::Explorer => "file explorer (left)",
        DockKind::Agent => "agent panel (right)",
    }
}

fn dir_name(dir: Direction) -> &'static str {
    match dir {
        Direction::Left => "left",
        Direction::Right => "right",
        Direction::Up => "up",
        Direction::Down => "down",
    }
}

fn kind_name(kind: DockKind) -> &'static str {
    match kind {
        DockKind::Explorer => "explorer",
        DockKind::Agent => "agent",
    }
}

impl Action {
    pub fn group(&self) -> ActionGroup {
        match self {
            Action::SplitPane(_)
            | Action::ClosePane
            | Action::FocusPane(_)
            | Action::ResizePane(..)
            | Action::ToggleZoom
            | Action::EqualizePanes
            | Action::FlipSplit
            | Action::JoinPane(_)
            | Action::Rearrange
            | Action::RotatePanes(_)
            | Action::FocusPaneIndex(_)
            | Action::NextPane
            | Action::PrevPane
            | Action::LastPane => ActionGroup::Panes,

            Action::ApplyLayout(_) | Action::NextLayout => ActionGroup::Layouts,

            Action::NewTab
            | Action::CloseTab
            | Action::NextTab
            | Action::PrevTab
            | Action::RenameTab
            | Action::SelectTab(_) => ActionGroup::Tabs,

            Action::ToggleSidebar(_) | Action::FocusSidebar(_) | Action::ResizeSidebar(..) => {
                ActionGroup::Sidebars
            }

            Action::Search
            | Action::ScrollLines(_)
            | Action::ScrollPage(_)
            | Action::ScrollToBottom => ActionGroup::Scrollback,

            Action::Copy | Action::Paste => ActionGroup::Clipboard,

            Action::ShowShortcuts
            | Action::ShowThemes
            | Action::ShowSettings
            | Action::EditConfig
            | Action::ReloadConfig
            | Action::Quit => ActionGroup::General,
        }
    }

    pub fn description(&self) -> String {
        match self {
            Action::FocusPaneIndex(n) => format!("Focus pane {n}"),
            Action::NextPane => "Focus the next pane".into(),
            Action::PrevPane => "Focus the previous pane".into(),
            Action::LastPane => "Focus the pane you came from".into(),
            Action::ApplyLayout(name) => format!("Use the {name} layout"),
            Action::NextLayout => "Cycle through the layout presets".into(),
            Action::SplitPane(Axis::Horizontal) => "Split the pane left and right".into(),
            Action::SplitPane(Axis::Vertical) => "Split the pane top and bottom".into(),
            Action::ClosePane => "Close the focused pane".into(),
            Action::FocusPane(d) => format!("Move focus to the pane {}", dir_name(*d)),
            Action::ResizePane(d, n) => format!("Move the pane border {} by {n}", dir_name(*d)),
            Action::ToggleZoom => "Expand the focused pane to fill the tab".into(),
            Action::EqualizePanes => "Give every pane an equal share".into(),
            Action::FlipSplit => "Turn this split the other way".into(),
            Action::JoinPane(Direction::Left) => "Stack this pane under the one to its left".into(),
            Action::JoinPane(Direction::Right) => {
                "Stack this pane over the one to its right".into()
            }
            Action::JoinPane(Direction::Up) => "Put this pane beside the one above".into(),
            Action::JoinPane(Direction::Down) => "Put this pane beside the one below".into(),
            Action::Rearrange => "Arrange the panes by hand".into(),
            Action::RotatePanes(true) => "Move the panes round".into(),
            Action::RotatePanes(false) => "Move the panes round the other way".into(),

            Action::NewTab => "Open a new tab".into(),
            Action::RenameTab => "Name this tab".into(),
            Action::CloseTab => "Close the current tab".into(),
            Action::NextTab => "Go to the next tab".into(),
            Action::PrevTab => "Go to the previous tab".into(),
            Action::SelectTab(n) => format!("Go to tab {n}"),

            Action::ToggleSidebar(k) => format!("Show or hide the {}", side(*k)),
            Action::FocusSidebar(k) => format!("Move focus into the {}", side(*k)),
            Action::ResizeSidebar(k, d) if *d < 0 => format!("Narrow the {}", side(*k)),
            Action::ResizeSidebar(k, _) => format!("Widen the {}", side(*k)),

            Action::ScrollLines(n) => {
                let (dir, count) = if *n < 0 {
                    ("back", -*n)
                } else {
                    ("forward", *n)
                };
                let unit = if count == 1 { "line" } else { "lines" };
                format!("Scroll {dir} {count} {unit}")
            }
            Action::ScrollPage(n) if *n < 0 => "Scroll back one page".into(),
            Action::ScrollPage(_) => "Scroll forward one page".into(),
            Action::ScrollToBottom => "Jump back to the live output".into(),
            Action::Search => "Search the scrollback".into(),

            Action::Copy => "Copy the selection".into(),
            Action::Paste => "Paste the clipboard".into(),

            Action::ShowShortcuts => "Show the command palette".into(),
            Action::ShowThemes => "Pick a theme".into(),
            Action::ShowSettings => "Show every setting and what it is set to".into(),
            Action::EditConfig => "Open the config file in your editor".into(),
            Action::ReloadConfig => "Reload the configuration file".into(),
            Action::Quit => "Quit".into(),
        }
    }

    /// The name used in the config file.
    pub fn name(&self) -> String {
        match self {
            Action::FocusPaneIndex(n) => format!("focus_pane:{n}"),
            Action::NextPane => "next_pane".into(),
            Action::PrevPane => "prev_pane".into(),
            Action::LastPane => "last_pane".into(),
            Action::ApplyLayout(name) => format!("apply_layout:{name}"),
            Action::NextLayout => "next_layout".into(),
            Action::SplitPane(Axis::Horizontal) => "split_horizontal".into(),
            Action::SplitPane(Axis::Vertical) => "split_vertical".into(),
            Action::ClosePane => "close_pane".into(),
            Action::FocusPane(d) => format!("focus_{}", dir_name(*d)),
            Action::ResizePane(d, n) => format!("resize_{}:{n}", dir_name(*d)),
            Action::ToggleZoom => "toggle_zoom".into(),
            Action::EqualizePanes => "equalize_panes".into(),
            Action::FlipSplit => "flip_split".into(),
            Action::JoinPane(d) => format!("join_{}", dir_name(*d)),
            Action::Rearrange => "rearrange".into(),
            Action::RotatePanes(true) => "rotate_panes".into(),
            Action::RotatePanes(false) => "rotate_panes_back".into(),

            Action::NewTab => "new_tab".into(),
            Action::RenameTab => "rename_tab".into(),
            Action::CloseTab => "close_tab".into(),
            Action::NextTab => "next_tab".into(),
            Action::PrevTab => "prev_tab".into(),
            Action::SelectTab(n) => format!("select_tab:{n}"),

            Action::ToggleSidebar(k) => format!("toggle_{}", kind_name(*k)),
            Action::FocusSidebar(k) => format!("focus_{}", kind_name(*k)),
            Action::ResizeSidebar(k, d) => format!("resize_{}:{d}", kind_name(*k)),

            Action::ScrollLines(n) => format!("scroll_lines:{n}"),
            Action::ScrollPage(n) => format!("scroll_page:{n}"),
            Action::ScrollToBottom => "scroll_to_bottom".into(),
            Action::Search => "search".into(),

            Action::Copy => "copy".into(),
            Action::Paste => "paste".into(),

            Action::ShowShortcuts => "show_shortcuts".into(),
            Action::ShowThemes => "show_themes".into(),
            Action::ShowSettings => "show_settings".into(),
            Action::EditConfig => "edit_config".into(),
            Action::ReloadConfig => "reload_config".into(),
            Action::Quit => "quit".into(),
        }
    }

    /// Parses a config-file action name, with an optional `:amount` suffix.
    pub fn from_name(spec: &str) -> Option<Action> {
        let (name, arg) = match spec.split_once(':') {
            Some((n, a)) => (n.trim(), Some(a.trim())),
            None => (spec.trim(), None),
        };
        let num = |default: i32| -> i32 { arg.and_then(|a| a.parse().ok()).unwrap_or(default) };

        // A layout argument is a name, not a number, so it is read before the
        // numeric actions below.
        if name == "apply_layout" {
            let layout = arg.filter(|a| !a.is_empty())?;
            return Some(Action::ApplyLayout(layout.into()));
        }

        Some(match name {
            "focus_pane" => Action::FocusPaneIndex(num(1).clamp(1, 255) as u8),
            "next_pane" => Action::NextPane,
            "prev_pane" => Action::PrevPane,
            "last_pane" => Action::LastPane,
            "next_layout" => Action::NextLayout,
            "split_horizontal" => Action::SplitPane(Axis::Horizontal),
            "split_vertical" => Action::SplitPane(Axis::Vertical),
            "close_pane" => Action::ClosePane,
            "focus_left" => Action::FocusPane(Direction::Left),
            "focus_right" => Action::FocusPane(Direction::Right),
            "focus_up" => Action::FocusPane(Direction::Up),
            "focus_down" => Action::FocusPane(Direction::Down),
            "resize_left" => Action::ResizePane(Direction::Left, num(5) as u16),
            "resize_right" => Action::ResizePane(Direction::Right, num(5) as u16),
            "resize_up" => Action::ResizePane(Direction::Up, num(5) as u16),
            "resize_down" => Action::ResizePane(Direction::Down, num(5) as u16),
            "toggle_zoom" => Action::ToggleZoom,
            "equalize_panes" => Action::EqualizePanes,
            "flip_split" => Action::FlipSplit,
            "join_left" => Action::JoinPane(Direction::Left),
            "join_right" => Action::JoinPane(Direction::Right),
            "join_up" => Action::JoinPane(Direction::Up),
            "join_down" => Action::JoinPane(Direction::Down),
            "rearrange" => Action::Rearrange,
            "rotate_panes" => Action::RotatePanes(true),
            "rotate_panes_back" => Action::RotatePanes(false),

            "new_tab" => Action::NewTab,
            "rename_tab" => Action::RenameTab,
            "close_tab" => Action::CloseTab,
            "next_tab" => Action::NextTab,
            "prev_tab" => Action::PrevTab,
            "select_tab" => Action::SelectTab(num(1).clamp(1, 255) as u8),

            "toggle_explorer" => Action::ToggleSidebar(DockKind::Explorer),
            "toggle_agent" => Action::ToggleSidebar(DockKind::Agent),
            "focus_explorer" => Action::FocusSidebar(DockKind::Explorer),
            "focus_agent" => Action::FocusSidebar(DockKind::Agent),
            "resize_explorer" => Action::ResizeSidebar(DockKind::Explorer, num(4) as i16),
            "resize_agent" => Action::ResizeSidebar(DockKind::Agent, num(4) as i16),

            "scroll_lines" => Action::ScrollLines(num(-3)),
            "scroll_page" => Action::ScrollPage(num(-1)),
            "scroll_to_bottom" => Action::ScrollToBottom,
            "search" => Action::Search,

            "copy" => Action::Copy,
            "paste" => Action::Paste,

            "show_shortcuts" => Action::ShowShortcuts,
            "show_themes" => Action::ShowThemes,
            "show_settings" => Action::ShowSettings,
            "edit_config" => Action::EditConfig,
            "reload_config" => Action::ReloadConfig,
            "quit" => Action::Quit,
            _ => return None,
        })
    }
}
