#[allow(unused_imports)]
use crate::prelude::*;

/// Everything one window owns.
pub struct Session {
    pub(crate) config: Config,
    pub(crate) theme: Theme,
    /// Derived from the theme; kept so it is not recomputed every frame.
    pub(crate) syntax: SyntaxPalette,
    /// The backend draws the tab bar itself, in pixels.
    pub(crate) pixel_chrome: bool,
    pub(crate) themes: ThemeRegistry,
    pub(crate) workspace: Workspace,
    /// Parallel to the workspace's panes; a pane without an entry here has
    /// lost its child.
    pub(crate) panes: Vec<(PaneId, Pane)>,
    pub(crate) ptys: PtyPool,
    pub(crate) chrome: Chrome,
    pub(crate) explorer: Explorer,
    /// The agent dock is an ordinary pane running a command; `None` until the
    /// dock is first shown.
    pub(crate) agent: Option<PaneId>,
    /// The agent rang its bell while you were looking elsewhere.
    ///
    /// Hiding the dock does not stop the agent, so without this the only way
    /// to learn it had finished would be to open the panel and check.
    pub(crate) agent_alert: bool,
    pub(crate) focus: FocusTarget,
    /// The pane a selection belongs to, and the selection itself.
    pub(crate) selection: Option<(PaneId, Selection)>,
    /// True while the button is still down.
    pub(crate) dragging: bool,
    /// A pane border or dock edge being dragged with the mouse.
    pub(crate) drag: Option<DividerDrag>,
    /// Time, place and count of the last click, for double and triple clicks.
    pub(crate) last_click: Option<(std::time::Instant, u16, u16, u8)>,
    pub(crate) search: Option<SearchState>,
    /// The tab name being typed, if one is.
    pub(crate) renaming: Option<String>,
    /// A paste waiting to be confirmed.
    pub(crate) pending_paste: Option<PendingPaste>,
    /// The link under the pointer while the modifier is held: the pane,
    /// the logical row, and the span on it. Drawn underlined, so there is
    /// something to see before there is something to click.
    pub(crate) hover_link: Option<(PaneId, u32, usize, usize)>,
    pub(crate) preview: Option<Preview>,
    pub(crate) autocomplete: Autocomplete,
    /// What names run something, for colouring only lines that would.
    pub(crate) commands: CommandIndex,
    /// When the children should be told their new size.
    ///
    /// A drag changes the size a column at a time, and each one a child hears
    /// about is a SIGWINCH it redraws its prompt for. Telling them once the
    /// dragging stops turns a screenful of repeated prompts into one.
    pub(crate) resize_settles: Option<std::time::Instant>,
    /// Panes that were opened to edit a file, and which file.
    ///
    /// A second editor on a file the first one still has open is refused by
    /// the editor itself, with a swap-file warning nobody asked for. The
    /// pane that is already on it gets the focus instead.
    pub(crate) editing: Vec<(PaneId, std::path::PathBuf)>,
    /// Each pane's number, by the slot it holds: index 0 is pane 1.
    ///
    /// A number belongs to the terminal, not to where it currently sits, so
    /// moving a pane about in the arrange panel does not renumber it and the
    /// panel shows the same numbers the next time it is opened. A closed
    /// pane frees its slot for the next one.
    pub(crate) pane_numbers: Vec<Option<PaneId>>,
    /// Where the current command line began: pane, logical row, column.
    ///
    /// Reading the line back off the grid rather than accumulating keystrokes
    /// means the shell's own editing (backspace, ^U, history recall) is
    /// reflected for free.
    pub(crate) input_anchor: Option<(PaneId, u32, u16)>,
    /// A submitted line waiting to be read back once the shell has echoed all
    /// of it.
    pub(crate) pending_record: Option<(PaneId, u32, u16)>,
    /// False while running inside another terminal, which keeps Cmd for
    /// itself. The shortcut panel hides bindings that cannot fire.
    pub(crate) native_window: bool,
    /// Position in the preset cycle, for the next-layout command.
    pub(crate) layout_cycle: usize,
    pub(crate) size: (u16, u16),
    /// The prefix key was pressed and the next key is looked up prefixed.
    pub(crate) prefix_pending: bool,
    pub(crate) palette: Option<Palette>,
    /// Set while the panes are being arranged by hand.
    pub(crate) rearrange: Option<Rearrange>,
    /// An update check running in the background, and whether somebody
    /// asked for it rather than it being the daily one.
    pub(crate) update_check: Option<(std::sync::mpsc::Receiver<Outcome>, bool)>,
    /// An install running in the background, and what it has to say.
    pub(crate) update_install: Option<std::sync::mpsc::Receiver<Result<String, String>>>,
    /// The shell being asked what names it knows.
    pub(crate) shell_names: Option<std::sync::mpsc::Receiver<Vec<String>>>,
    pub(crate) message: String,
    /// The message as it was last seen, and when it stops being news.
    ///
    /// Watched rather than set alongside every message: messages are raised
    /// from two dozen places, and one that forgot to start the clock would
    /// sit on the status line for the rest of the session.
    pub(crate) message_shown: String,
    pub(crate) message_until: Option<std::time::Instant>,
    pub(crate) decoder: InputDecoder,
    pub(crate) needs_draw: bool,
    pub(crate) running: bool,
}
