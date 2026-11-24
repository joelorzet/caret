#[allow(unused_imports)]
use crate::prelude::*;

use std::io::{self, Write};

use crate::impls::host::draw::{agent_rect, dock_body, DOCK_HEADER};

/// How still a drag has to be before the children are told the new size.
const RESIZE_SETTLE: std::time::Duration = std::time::Duration::from_millis(80);

/// How long a message stays on the status line before it gives the line back.
const MESSAGE_LIFE: std::time::Duration = std::time::Duration::from_secs(6);

/// Editors that take `+n` to open on a line.
const OPENS_AT_LINE: [&str; 6] = ["vi", "vim", "nvim", "nano", "pico", "helix"];

/// Room for the two bracketed-paste markers, so the buffer is sized once.
const PASTE_MARKERS: usize = 12;

/// How long to wait for activity before looking round again, in
/// milliseconds. Long enough to be idle-cheap, short enough to notice a
/// child that exited without closing its terminal.
const POLL_INTERVAL: i32 = 100;

impl Session {
    pub fn new() -> io::Result<Session> {
        let loaded = Config::load();
        let (themes, theme_warnings) = ThemeRegistry::load();
        let theme = themes.resolve(&loaded.config.theme).clone();

        let size = caret_pty::terminal_size()
            .unwrap_or((loaded.config.window.cols, loaded.config.window.rows));
        let chrome = loaded.config.chrome();

        let explorer_root = std::env::current_dir().unwrap_or_else(|_| "/".into());
        let explorer = Explorer::new(explorer_root, loaded.config.sidebars.explorer.show_hidden);

        let mut session = Session {
            config: loaded.config,
            syntax: SyntaxPalette::derive(&theme),
            pixel_chrome: false,
            theme,
            themes,
            workspace: Workspace::new(),
            panes: Vec::new(),
            ptys: PtyPool::new(),
            chrome,
            explorer,
            agent: None,
            commands: CommandIndex::from_path(),
            pane_numbers: Vec::new(),
            editing: Vec::new(),
            resize_settles: None,
            agent_alert: false,
            focus: FocusTarget::Panes,
            selection: None,
            dragging: false,
            drag: None,
            last_click: None,
            search: None,
            renaming: None,
            preview: None,
            autocomplete: Autocomplete::new(
                Session::history_path()
                    .map(|path| caret_features::history::load(&path, 5_000))
                    .unwrap_or_default(),
            ),
            input_anchor: None,
            pending_record: None,
            native_window: false,
            layout_cycle: 0,
            size,
            prefix_pending: false,
            palette: None,
            rearrange: None,
            update_check: None,
            message: String::new(),
            message_shown: String::new(),
            message_until: None,
            decoder: InputDecoder::default(),
            needs_draw: true,
            running: true,
        };

        let warnings = loaded.warnings.len() + theme_warnings.len();
        if warnings > 0 {
            session.message = format!(" {warnings} config problem(s): run `caret config`");
        }

        session.begin_update_check(false);

        let first = session.workspace.active_pane();
        let area = session.center_rect();
        session.spawn_pane(first, area.w, area.h)?;
        Ok(session)
    }

    // ---- panes ------------------------------------------------------------

    pub(crate) fn pane(&self, id: PaneId) -> Option<&Pane> {
        self.panes
            .iter()
            .find(|(other, _)| *other == id)
            .map(|(_, p)| p)
    }

    pub(crate) fn pane_mut(&mut self, id: PaneId) -> Option<&mut Pane> {
        self.panes
            .iter_mut()
            .find(|(other, _)| *other == id)
            .map(|(_, p)| p)
    }

    pub(crate) fn memory_bytes(&self) -> usize {
        self.panes
            .iter()
            .map(|(_, p)| p.terminal.memory_bytes())
            .sum()
    }

    /// Where a new pane should start: where the focused one is now.
    ///
    /// Read before the new pane exists, because creating it makes it the
    /// focused one and there would be nothing left to inherit from.
    pub(crate) fn inherited_cwd(&self) -> Option<std::path::PathBuf> {
        let from = self.workspace.active_pane();
        self.pane(from)
            .and_then(|pane| pane.terminal.cwd())
            .map(std::path::PathBuf::from)
            .or_else(|| self.ptys.working_dir(PtyId(from.0)))
            .filter(|path| path.is_dir())
    }

    /// Starts a shell for `id`, inheriting the focused pane's directory so a
    /// new split opens where you were working.
    pub(crate) fn spawn_pane(&mut self, id: PaneId, cols: u16, rows: u16) -> io::Result<()> {
        self.spawn_shell(id, None, cols, rows)
    }

    /// Starts a shell for `id` in `cwd`, or wherever the focused pane is.
    pub(crate) fn spawn_shell(
        &mut self,
        id: PaneId,
        cwd: Option<std::path::PathBuf>,
        cols: u16,
        rows: u16,
    ) -> io::Result<()> {
        let program = self.config.shell_program();
        let args = self.config.terminal.shell_args.clone();
        self.spawn_with(id, program, args, cwd, cols, rows)
    }

    /// Starts `program` for `id`, inheriting the focused pane's directory so a
    /// new split or the agent opens where you were working.
    /// Starts `program` for `id`.
    ///
    /// `cwd` of `None` inherits the focused pane's directory, so a new split
    /// opens where you were working; the explorer passes a folder instead.
    pub(crate) fn spawn_with(
        &mut self,
        id: PaneId,
        program: String,
        args: Vec<String>,
        cwd: Option<std::path::PathBuf>,
        cols: u16,
        rows: u16,
    ) -> io::Result<()> {
        let cols = cols.max(1);
        let rows = rows.max(1);

        let from = self.workspace.active_pane();
        let cwd = cwd
            // What the shell said, if it says anything at all.
            .or_else(|| {
                self.pane(from)
                    .and_then(|pane| pane.terminal.cwd())
                    .map(std::path::PathBuf::from)
            })
            // Otherwise ask the kernel where that pane's shell actually is.
            // Most shells never send OSC 7, so without this a new pane opens
            // wherever the window was started rather than where you are.
            .or_else(|| self.ptys.working_dir(PtyId(from.0)))
            // Nothing else to go on means the very first pane. Launched from
            // the command line that is wherever you were, but launched from
            // Finder or Spotlight it is `/`, which is nobody's idea of where
            // they wanted to start.
            .or_else(|| {
                let here = std::env::current_dir().ok();
                match here {
                    Some(dir) if dir != std::path::Path::new("/") => Some(dir),
                    _ => std::env::var_os("HOME").map(std::path::PathBuf::from),
                }
            })
            .filter(|path| path.is_dir());

        let spec = PtySpec {
            program,
            args,
            cwd,
            env: Vec::new(),
            cols,
            rows,
        };
        self.ptys.spawn(PtyId(id.0), &spec)?;
        self.panes.push((
            id,
            Pane::new(
                cols,
                rows,
                self.config.terminal.scrollback,
                self.config.terminal.cursor,
            ),
        ));
        self.needs_draw = true;
        Ok(())
    }

    /// Splits the pane area and starts a shell in `dir`.
    pub(crate) fn open_pane_in(&mut self, dir: std::path::PathBuf) -> io::Result<()> {
        let area = self.center_rect();
        let Some(pane) = self.workspace.split(Axis::Horizontal, false) else {
            self.message = " cannot split any further".into();
            return Ok(());
        };
        let rect = self
            .workspace
            .active_tab()
            .resolve(area)
            .panes
            .iter()
            .find(|(id, _)| *id == pane)
            .map(|(_, rect)| *rect)
            .unwrap_or(area);

        let program = self.config.shell_program();
        let args = self.config.terminal.shell_args.clone();
        self.spawn_with(pane, program, args, Some(dir), rect.w, rect.h)
    }

    /// Opens the file in `$EDITOR` in a new pane.
    ///
    /// Running the editor in a pane rather than replacing the preview keeps
    /// it a terminal: vim or nvim gets a real pty and behaves exactly as it
    /// would anywhere else.
    pub(crate) fn open_editor(&mut self, path: std::path::PathBuf) -> io::Result<()> {
        self.open_editor_at(path, None)
    }

    /// Opens `path`, on `line` where the editor knows how to be told.
    pub(crate) fn open_editor_at(
        &mut self,
        path: std::path::PathBuf,
        line: Option<usize>,
    ) -> io::Result<()> {
        let editor = std::env::var("VISUAL")
            .or_else(|_| std::env::var("EDITOR"))
            .unwrap_or_else(|_| "vi".to_string());

        // `$EDITOR` may carry arguments, as in "code -w".
        let mut parts = editor.split_whitespace().map(str::to_string);
        let Some(program) = parts.next() else {
            return Ok(());
        };
        let mut args: Vec<String> = parts.collect();

        // `+n` is the old convention and the editors people set `$EDITOR` to
        // in a terminal all take it. Anything else just opens the file.
        let name = std::path::Path::new(&program)
            .file_stem()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if let Some(line) = line.filter(|_| OPENS_AT_LINE.contains(&name.as_str())) {
            args.push(format!("+{line}"));
        }
        args.push(path.to_string_lossy().into_owned());

        let dir = path.parent().map(|p| p.to_path_buf());
        let area = self.center_rect();
        let Some(pane) = self.workspace.split(Axis::Horizontal, false) else {
            self.message = " cannot split any further".into();
            return Ok(());
        };
        let rect = self
            .workspace
            .active_tab()
            .resolve(area)
            .panes
            .iter()
            .find(|(id, _)| *id == pane)
            .map(|(_, rect)| *rect)
            .unwrap_or(area);

        self.preview = None;
        self.focus = FocusTarget::Panes;
        self.message = format!(" {program} {}", path.display());
        self.editing.push((pane, path));
        self.spawn_with(pane, program, args, dir, rect.w, rect.h)
    }

    /// The pane already editing `path`, if one still is.
    ///
    /// Panes close by several routes, so the list is pruned here rather than
    /// at each of them: an entry pointing at a pane that has gone would send
    /// the focus somewhere that no longer exists.
    fn editor_on(&mut self, path: &std::path::Path) -> Option<PaneId> {
        self.editing
            .retain(|(pane, _)| self.panes.iter().any(|(id, _)| id == pane));
        // Only this tab: focus cannot reach into another one, and a pane
        // somewhere else is no answer to "show me the file".
        let here = self.workspace.active_tab().panes();
        self.editing
            .iter()
            .find(|(pane, open)| open == path && here.contains(pane))
            .map(|(pane, _)| *pane)
    }

    /// Opens the config file in a pane, writing the example first if there
    /// is nothing there yet.
    ///
    /// An empty file would be a worse answer than no file: the example lists
    /// every key with what it does, so there is something to edit rather
    /// than something to remember.
    pub(crate) fn edit_config(&mut self) -> io::Result<()> {
        self.edit_setting(None)
    }

    /// Opens the config file, on the setting named if there is one.
    ///
    /// The setting is written in first with the value it already has, so a
    /// file that is only a line long still opens on something to change.
    pub(crate) fn edit_setting(&mut self, at: Option<(&str, &str, &str)>) -> io::Result<()> {
        let Some(path) = caret_config::paths::config_path() else {
            self.message = " no config directory on this system".into();
            return Ok(());
        };
        if !path.exists() {
            if let Err(e) = Config::write_example(&path) {
                self.message = format!(" cannot write {}: {e}", path.display());
                return Ok(());
            }
        }

        // An editor is already on it. Opening a second one gets a swap-file
        // warning from the editor rather than a file to edit.
        if let Some(pane) = self.editor_on(&path) {
            if self.workspace.active_tab_mut().set_focus(pane) {
                self.focus = FocusTarget::Panes;
                self.preview = None;
                self.needs_draw = true;
                self.message = " already open, here it is".into();
                return Ok(());
            }
        }

        // A stub is filled out with the commented example first, so there
        // is something to read and edit rather than one line and a screen of
        // tildes.
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        let filled = Config::filled_out(&text);
        if filled != text {
            if let Err(e) = std::fs::write(&path, &filled) {
                self.message = format!(" cannot write {}: {e}", path.display());
                return Ok(());
            }
        }

        let line = match at {
            None => None,
            Some((table, key, value)) => {
                let (updated, line) = Config::with_setting(&filled, table, key, value);
                if updated != filled {
                    if let Err(e) = std::fs::write(&path, &updated) {
                        self.message = format!(" cannot write {}: {e}", path.display());
                        return Ok(());
                    }
                }
                Some(line)
            }
        };
        self.open_editor_at(path, line)
    }

    /// Opens a tab with a shell in `dir`.
    pub(crate) fn open_tab_in(&mut self, dir: std::path::PathBuf) -> io::Result<()> {
        let (_, pane) = self.workspace.new_tab();
        let area = self.center_rect();
        let program = self.config.shell_program();
        let args = self.config.terminal.shell_args.clone();
        self.spawn_with(pane, program, args, Some(dir), area.w, area.h)
    }

    /// The agent dock's pane. It sits outside the pane tree, so it takes an id
    /// the workspace will never allocate.
    pub(crate) const AGENT_PANE: PaneId = PaneId(u32::MAX);

    /// Starts the agent if it is not already running.
    pub(crate) fn ensure_agent(&mut self) -> io::Result<()> {
        if self.agent.is_some() {
            return Ok(());
        }
        // Born at the size it will be drawn at. Spawning it to the dock's
        // whole rectangle and resizing a frame later made it redraw itself
        // the moment it appeared, which is the flicker on opening the panel.
        let rect = match self.chrome.layout(self.window_rect()).right {
            Some(dock) => agent_rect(dock),
            None => self.center_rect(),
        };

        let command = self.config.sidebars.agent.command.clone();
        let args = self.config.sidebars.agent.args.clone();
        match self.spawn_with(
            Session::AGENT_PANE,
            command.clone(),
            args,
            None,
            rect.w,
            rect.h,
        ) {
            Ok(()) => {
                self.agent = Some(Session::AGENT_PANE);
                self.message = format!(" agent: {command}");
            }
            Err(e) => {
                // A missing agent binary must not take the dock down with it.
                self.chrome.right.visible = false;
                self.focus = FocusTarget::Panes;
                self.message = format!(" cannot start {command:?}: {e}");
            }
        }
        Ok(())
    }

    pub(crate) fn drop_pane(&mut self, id: PaneId) {
        self.ptys.close(PtyId(id.0));
        self.panes.retain(|(other, _)| *other != id);
    }

    // ---- geometry ---------------------------------------------------------

    pub(crate) fn window_rect(&self) -> Rect {
        Rect::new(0, 0, self.size.0, self.size.1)
    }

    pub(crate) fn center_rect(&self) -> Rect {
        self.chrome.layout(self.window_rect()).center
    }

    /// Recomputes the layout and tells every pane and pty its new size.
    /// Notes that the children are owed a new size, once the dragging stops.
    ///
    /// Our own grid is resized straight away, so the window keeps drawing
    /// correctly throughout; it is only the children that wait. A shell
    /// redraws its prompt for every SIGWINCH, so one per pixel of a drag
    /// fills the pane with prompts.
    fn defer_pty_resize(&mut self) {
        self.resize_settles = Some(std::time::Instant::now() + RESIZE_SETTLE);
    }

    /// Tells every child the size its pane now is, if the dragging has
    /// stopped long enough to be sure it has.
    fn settle_pty_sizes(&mut self) {
        let due = self
            .resize_settles
            .is_some_and(|at| std::time::Instant::now() >= at);
        if !due {
            return;
        }
        self.resize_settles = None;

        let sizes: Vec<(PaneId, u16, u16)> = self
            .panes
            .iter()
            .map(|(id, pane)| (*id, pane.terminal.cols(), pane.terminal.rows()))
            .collect();
        for (id, cols, rows) in sizes {
            let _ = self.ptys.resize(PtyId(id.0), cols, rows);
        }
    }

    /// Hands out and takes back pane numbers.
    ///
    /// Done in one place every frame rather than at each spawn and close:
    /// panes leave by several routes, and a number left pointing at a pane
    /// that is gone would be handed to the next one.
    fn sync_pane_numbers(&mut self) {
        let live: Vec<PaneId> = self
            .panes
            .iter()
            .map(|(id, _)| *id)
            .filter(|id| *id != Session::AGENT_PANE)
            .collect();
        assign_pane_numbers(&mut self.pane_numbers, &live);
    }

    /// The number shown on a pane, counting from one.
    pub(crate) fn pane_number(&self, id: PaneId) -> Option<usize> {
        let slot = self
            .pane_numbers
            .iter()
            .position(|held| *held == Some(id))?;
        Some(slot + 1)
    }

    /// The pane a number refers to.
    pub(crate) fn pane_numbered(&self, number: usize) -> Option<PaneId> {
        *self.pane_numbers.get(number.checked_sub(1)?)?
    }

    fn relayout(&mut self) -> Frame {
        self.sync_pane_numbers();
        // Looking at the agent is what clears its alert, and this is the one
        // place every frame passes through regardless of how focus moved.
        if self.focus == FocusTarget::Agent {
            self.agent_alert = false;
        }
        let frame = self.chrome.layout(self.window_rect());
        let resolved = self.workspace.active_tab().resolve(frame.center);

        let preview_rows = self.preview_rows();
        if let Some(preview) = self.preview.as_mut() {
            preview.set_height(preview_rows);
        }
        if let Some(palette) = self.palette.as_mut() {
            palette.set_height((self.size.1.saturating_sub(8)).max(3) as usize);
        }

        // The docks are not part of the pane tree, so they are sized here.
        // They are sized to the card they are drawn into, not to the space the
        // dock was given: the difference is the margin around the card, and a
        // terminal told it has columns it is never drawn into leaves the
        // leftovers of its last frame on screen.
        if let Some(rect) = frame.left {
            let body = dock_body(rect, true);
            self.explorer
                .set_height(body.h.saturating_sub(DOCK_HEADER) as usize);
        }
        if let Some((rect, agent)) = frame.right.zip(self.agent) {
            let body = agent_rect(rect);
            let changed = self
                .pane(agent)
                .is_some_and(|p| p.terminal.cols() != body.w || p.terminal.rows() != body.h);
            if changed {
                if let Some(pane) = self.pane_mut(agent) {
                    pane.terminal.resize(body.w, body.h);
                }
                self.defer_pty_resize();
            }
        }

        for (id, rect) in resolved.panes {
            let changed = match self.pane(id) {
                Some(pane) => pane.terminal.cols() != rect.w || pane.terminal.rows() != rect.h,
                None => false,
            };
            if changed {
                if let Some(pane) = self.pane_mut(id) {
                    pane.terminal.resize(rect.w, rect.h);
                }
                self.defer_pty_resize();
            }
        }
        frame
    }

    /// Switches theme, keeping the derived syntax colours in step.
    pub(crate) fn apply_theme(&mut self, theme: Theme) {
        self.syntax = SyntaxPalette::derive(&theme);
        self.theme = theme;
        for (_, pane) in self.panes.iter_mut() {
            pane.terminal.screen_mut().damage_all();
        }
        self.needs_draw = true;
    }

    /// Declares that we own a window rather than running inside another
    /// terminal.
    ///
    /// Three things follow from it: Cmd reaches us, the chrome can be drawn in
    /// pixels, and the system clipboard is ours to read and write instead of
    /// being asked for over OSC 52.
    pub fn set_native_window(&mut self, native: bool) {
        self.native_window = native;
    }

    /// A short name for the window title.
    pub fn window_title(&self) -> String {
        let tab = self.workspace.active_tab();
        let label = self
            .pane(tab.focus())
            .and_then(|p| p.terminal.cwd())
            .map(|cwd| {
                std::path::Path::new(cwd)
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| cwd.to_string())
            });
        match label {
            Some(name) => format!("{name} \u{2014} caret"),
            None => "caret".to_string(),
        }
    }

    /// Sets the grid size from a backend that owns the window.
    pub fn set_size(&mut self, cols: u16, rows: u16) {
        let size = (cols.max(1), rows.max(1));
        if size != self.size {
            self.size = size;
            self.needs_draw = true;
        }
    }

    pub fn size(&self) -> (u16, u16) {
        self.size
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn needs_draw(&self) -> bool {
        self.needs_draw
    }

    pub fn mark_drawn(&mut self) {
        self.needs_draw = false;
    }

    /// Recomputes the layout and returns the frame, for a backend that draws
    /// on its own schedule.
    pub fn frame(&mut self) -> Frame {
        self.relayout()
    }

    /// Services the pseudoterminals without blocking, returning whether
    /// anything happened.
    /// Retires a message once it has been up long enough.
    ///
    /// Long enough to read, then the status line goes back to saying where
    /// you are, which is what it is for the rest of the time.
    fn age_message(&mut self) {
        if self.message != self.message_shown {
            self.message_shown = self.message.clone();
            self.message_until =
                (!self.message.is_empty()).then(|| std::time::Instant::now() + MESSAGE_LIFE);
        }
        if self
            .message_until
            .is_some_and(|until| std::time::Instant::now() >= until)
        {
            self.message.clear();
            self.message_shown.clear();
            self.message_until = None;
            self.needs_draw = true;
        }
    }

    pub fn pump(&mut self, timeout_ms: i32) -> std::io::Result<bool> {
        self.age_message();
        self.settle_update_check();
        self.settle_pty_sizes();
        let mut events = Vec::new();
        let mut ready = Vec::new();
        self.ptys.poll(timeout_ms, &[], &mut events, &mut ready)?;
        let had = !events.is_empty();
        for event in events {
            self.on_pty_event(event);
        }
        if self.workspace.tabs().is_empty() {
            self.running = false;
        }
        Ok(had)
    }

    /// Delivers a key from a backend that decodes its own input.
    pub fn handle_key(&mut self, press: KeyPress) -> std::io::Result<()> {
        self.on_key(press)
    }

    pub fn handle_mouse(&mut self, event: MouseEvent) {
        self.on_mouse(event);
    }

    pub fn handle_paste(&mut self, text: Vec<u8>) {
        self.on_paste(text);
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn font_config(&self) -> &caret_config::FontConfig {
        &self.config.font
    }

    pub fn window_config(&self) -> &caret_config::WindowConfig {
        &self.config.window
    }

    /// The frame and the cursor, for a windowed backend.
    pub fn paint(&mut self) -> (Surface, Option<CursorPaint>) {
        let frame = self.relayout();
        let (surface, target) = self.compose(&frame);
        let cursor = self.cursor_paint(target);
        (surface, cursor)
    }

    fn adopt_terminal_size(&mut self) {
        if let Some(size) = caret_pty::terminal_size() {
            if size != self.size {
                self.size = size;
                self.needs_draw = true;
            }
        }
    }

    // ---- the loop ---------------------------------------------------------

    pub fn run(&mut self) -> io::Result<()> {
        let _raw = RawMode::enable()?;
        signals::watch_resize();
        signals::watch_termination();

        let mut out = io::stdout();
        // Alternate screen, so the user's scrollback is untouched on exit.
        out.write_all(b"\x1b[?1049h\x1b[2J")?;
        // Ask the surrounding terminal for button and motion reports in the
        // SGR encoding, which is the only one that can address a wide window.
        if self.config.terminal.mouse {
            out.write_all(b"\x1b[?1002h\x1b[?1006h")?;
        }
        out.write_all(b"\x1b[?2004h")?;
        {}
        out.flush()?;

        let result = self.event_loop(&mut out);

        if self.config.terminal.mouse {
            out.write_all(b"\x1b[?1002l\x1b[?1006l")?;
        }
        out.write_all(b"\x1b[?2004l")?;
        {}
        out.write_all(b"\x1b[0m\x1b[?25h\x1b[?1049l")?;
        out.flush()?;
        result
    }

    fn event_loop(&mut self, out: &mut impl Write) -> io::Result<()> {
        let mut events = Vec::new();
        let mut ready = Vec::new();
        let mut keys: Vec<InputEvent> = Vec::new();
        let mut buffer = [0u8; 8192];

        while self.running && !signals::ending() {
            if signals::take_resize() {
                self.adopt_terminal_size();
            }

            let frame = self.relayout();
            if self.needs_draw {
                let painted = self.draw(&frame);
                out.write_all(painted.as_bytes())?;
                out.flush()?;
                self.needs_draw = false;
            }

            events.clear();
            self.ptys.poll(
                POLL_INTERVAL,
                &[libc::STDIN_FILENO],
                &mut events,
                &mut ready,
            )?;

            if ready.contains(&libc::STDIN_FILENO) {
                match read_stdin(&mut buffer)? {
                    0 => self.running = false,
                    n => {
                        keys.clear();
                        self.decoder.feed(&buffer[..n], &mut keys);
                        trace(&format!("stdin {n} bytes -> {} keys", keys.len()));
                        for event in core::mem::take(&mut keys) {
                            match event {
                                InputEvent::Key(press) => self.on_key(press)?,
                                InputEvent::Mouse(mouse) => self.on_mouse(mouse),
                                InputEvent::Paste(text) => self.on_paste(text),
                            }
                        }
                    }
                }
            }

            for event in core::mem::take(&mut events) {
                self.on_pty_event(event);
            }

            if self.workspace.tabs().is_empty() {
                self.running = false;
            }
        }
        Ok(())
    }

    fn on_pty_event(&mut self, event: PtyEvent) {
        match event {
            PtyEvent::Output { id, bytes } => {
                dump(id, &bytes);
                let reply = match self.pane_mut(PaneId(id.0)) {
                    Some(pane) => pane.consume(&bytes),
                    None => return,
                };
                if !reply.is_empty() {
                    let _ = self.ptys.write(id, &reply);
                }
                self.drain_terminal_events(PaneId(id.0));
                self.sync_suggestions();
                // A hidden agent keeps running, it just has nowhere to show.
                // Repainting the window for output no one can see is the one
                // cost of leaving it open, and it is avoidable; the bell still
                // asks for a frame of its own.
                let unseen = Some(PaneId(id.0)) == self.agent && !self.chrome.right.visible;
                if !unseen {
                    self.needs_draw = true;
                }
            }
            PtyEvent::Exited { id, .. } => {
                let pane = PaneId(id.0);
                self.panes.retain(|(other, _)| *other != pane);
                // The agent is not in the pane tree; losing it just closes the
                // dock rather than touching the layout.
                if self.agent == Some(pane) {
                    self.agent = None;
                    self.chrome.right.visible = false;
                    if self.focus == FocusTarget::Agent {
                        self.focus = FocusTarget::Panes;
                    }
                    self.message = " agent exited".into();
                    self.needs_draw = true;
                    return;
                }
                self.close_pane(pane);
            }
        }
    }

    fn drain_terminal_events(&mut self, id: PaneId) {
        let Some(pane) = self.pane_mut(id) else {
            return;
        };
        let events: Vec<TerminalEvent> = pane.terminal.drain_events().collect();
        for event in events {
            match event {
                TerminalEvent::Bell => {
                    // The agent rings when it has finished or wants an answer.
                    // If you are not looking at it, that has to be said
                    // somewhere you are.
                    if Some(id) == self.agent && self.focus != FocusTarget::Agent {
                        self.agent_alert = true;
                        self.needs_draw = true;
                    }
                    if self.config.terminal.bell == BellStyle::Audible {
                        let _ = io::stdout().write_all(b"\x07");
                    }
                }
                TerminalEvent::CwdChanged => {
                    if self.config.sidebars.explorer.follow_cwd
                        && id == self.workspace.active_pane()
                    {
                        if let Some(cwd) = self.pane(id).and_then(|p| p.terminal.cwd()) {
                            let cwd = cwd.to_string();
                            self.explorer.set_root(cwd);
                        }
                    }
                }
                TerminalEvent::TitleChanged => {}
                TerminalEvent::PromptMarked => {}
            }
        }
    }

    // ---- input ------------------------------------------------------------

    fn on_key(&mut self, press: KeyPress) -> io::Result<()> {
        // The palette takes every key while it is open, so a query can
        // contain the characters the bindings use.
        if self.palette.is_some() {
            self.prefix_pending = false;
            return self.palette_key(&press);
        }

        // Naming a tab takes every key, so a name can hold the characters
        // the bindings use.
        if self.is_renaming() {
            self.rename_key(&press);
            return Ok(());
        }

        // While the search prompt is open it takes every key, so a query can
        // contain the same characters the bindings use. Escape closes it.
        if self.search.is_some() {
            self.on_search_key(&press);
            return Ok(());
        }

        // Rearranging is modal in the strongest sense: every key is claimed,
        // so none reaches a shell and runs something while the panes are
        // being shuffled about.
        if self.is_rearranging() {
            self.rearrange_key(&press);
            return Ok(());
        }

        // A preview is modal, so it takes the keys ahead of whatever has
        // focus underneath. The prefix still gets through, so the window can
        // be driven without closing it first.
        if self.preview.is_some() && !self.prefix_pending {
            let is_prefix = self
                .config
                .keymap
                .prefix()
                .is_some_and(|prefix| prefix.matches(&press.binding));
            if is_prefix {
                self.prefix_pending = true;
                self.needs_draw = true;
                return Ok(());
            }
            return self.preview_key(&press);
        }

        let resolution = self
            .config
            .keymap
            .resolve(press.binding, self.prefix_pending);
        let was_pending = self.prefix_pending;
        self.prefix_pending = false;

        // A key the keymap does not claim belongs to whatever has focus.
        if let (KeyResolution::Unbound, false) = (&resolution, was_pending) {
            match self.focus {
                FocusTarget::Explorer => return self.route_to_explorer(&press),
                FocusTarget::Agent => {
                    self.forward_to(self.agent, press);
                    return Ok(());
                }
                FocusTarget::Panes => {}
            }
        }

        // Suggestions claim the arrows only while they are showing, so the
        // shell keeps them the rest of the time.
        if !was_pending && self.autocomplete.is_active() {
            match press.binding.key {
                Key::Down => {
                    self.autocomplete.select_next();
                    self.needs_draw = true;
                    return Ok(());
                }
                Key::Up => {
                    self.autocomplete.select_previous();
                    self.needs_draw = true;
                    return Ok(());
                }
                // Right accepts only at the end of the line; mid-line it is
                // still a cursor move.
                Key::Right if self.at_input_end() && self.accept_suggestion() => {
                    return Ok(());
                }
                _ => {}
            }
        }

        match resolution {
            KeyResolution::PrefixPending => {
                self.prefix_pending = true;
                self.needs_draw = true;
            }
            KeyResolution::Run(action) => {
                trace(&format!("action {action:?}"));
                self.message.clear();
                self.perform(action)?;
                self.needs_draw = true;
            }
            // An unbound key after the prefix is swallowed, as in tmux;
            // otherwise it belongs to the child.
            KeyResolution::Unbound if was_pending => {
                self.needs_draw = true;
            }
            KeyResolution::Unbound => self.forward(press),
        }
        Ok(())
    }

    /// Routes a key into the explorer, acting on what it returns.
    fn route_to_explorer(&mut self, press: &KeyPress) -> io::Result<()> {
        let Some(key) = panel_key(&press.binding) else {
            return Ok(());
        };
        match self.explorer.handle(key) {
            PanelOutcome::Redraw => self.needs_draw = true,
            PanelOutcome::Ignored => {}
            PanelOutcome::Dismiss => {
                self.focus = FocusTarget::Panes;
                self.needs_draw = true;
            }
            PanelOutcome::Preview(path) => {
                let mut preview = Preview::open(&path);
                preview.set_height(self.preview_rows());
                self.preview = Some(preview);
                self.needs_draw = true;
            }
            PanelOutcome::Copy(text) => self.copy_text(&text),
            PanelOutcome::OpenEditor(path) => self.open_editor(path)?,
            PanelOutcome::OpenPane(dir) => {
                self.focus = FocusTarget::Panes;
                self.open_pane_in(dir)?;
            }
            PanelOutcome::OpenTab(dir) => {
                self.focus = FocusTarget::Panes;
                self.open_tab_in(dir)?;
            }
            PanelOutcome::Insert(path) => {
                // Typing the path into the shell is more useful than guessing
                // which program should open it.
                let quoted = shell_quote(&path.to_string_lossy());
                let focus = self.workspace.active_pane();
                let _ = self.ptys.write(PtyId(focus.0), quoted.as_bytes());
                self.focus = FocusTarget::Panes;
                self.needs_draw = true;
            }
        }
        Ok(())
    }

    fn forward(&mut self, press: KeyPress) {
        let target = match self.focus {
            FocusTarget::Agent => self.agent,
            _ => Some(self.workspace.active_pane()),
        };
        self.forward_to(target, press);
    }

    fn forward_to(&mut self, target: Option<PaneId>, press: KeyPress) {
        let Some(focus) = target else { return };

        // Typing returns the view to the live output, which is what every
        // terminal does and what the user expects after scrolling back.
        if self.selection.is_some() {
            self.selection = None;
            self.needs_draw = true;
        }

        let (returned, app_cursor) = match self.pane_mut(focus) {
            Some(pane) => (
                pane.terminal.screen_mut().scroll_to_bottom(),
                pane.terminal.modes().contains(Modes::APP_CURSOR),
            ),
            None => return,
        };
        if returned {
            self.needs_draw = true;
        }

        // A submitted line is worth remembering; anything else just moves
        // the cursor and the line is re-read afterwards.
        match press.binding.key {
            Key::Enter => self.record_command(),
            Key::Char(_) => self.anchor_input(),
            _ => {}
        }

        let bytes = encode_for_child(&press.binding, app_cursor).unwrap_or(press.bytes);
        let _ = self.ptys.write(PtyId(focus.0), &bytes);
        self.sync_suggestions();
    }
}

impl Session {
    /// Sends pasted text to the focused pane.
    ///
    /// Newlines become carriage returns, which is what Enter sends, and the
    /// text is bracketed when the application asked for mode 2004 so an editor
    /// can tell a paste from typing and skip its auto-indent.
    pub(crate) fn on_paste(&mut self, text: Vec<u8>) {
        let target = match self.focus {
            FocusTarget::Agent => self.agent,
            _ => Some(self.workspace.active_pane()),
        };
        let Some(id) = target else { return };

        let bracketed = self
            .pane(id)
            .is_some_and(|p| p.terminal.modes().contains(Modes::BRACKETED_PASTE));

        let mut body: Vec<u8> = Vec::with_capacity(text.len() + PASTE_MARKERS);
        let mut previous = 0u8;
        for byte in text {
            match byte {
                b'\n' if previous == b'\r' => {}
                b'\n' => body.push(b'\r'),
                other => body.push(other),
            }
            previous = byte;
        }

        let mut out = Vec::with_capacity(body.len() + PASTE_MARKERS);
        if bracketed {
            out.extend_from_slice(b"\x1b[200~");
        }
        out.extend_from_slice(&body);
        if bracketed {
            out.extend_from_slice(b"\x1b[201~");
        }
        let _ = self.ptys.write(PtyId(id.0), &out);
        self.needs_draw = true;
    }

    /// The selected text, if there is a selection.
    pub(crate) fn selection_text(&self) -> Option<String> {
        let (id, selection) = self.selection.as_ref()?;
        let pane = self.pane(*id)?;
        Some(pane.terminal.screen().selection_text(selection))
    }
}

/// Maps a keypress onto the vocabulary panels understand.
pub(crate) fn panel_key(binding: &KeyBinding) -> Option<PanelKey> {
    let shift = binding.mods.contains(Modifiers::SHIFT);
    let ctrl = binding.mods.contains(Modifiers::CTRL);

    Some(match binding.key {
        // Shift and an arrow extends a selection; the panels decide what that
        // means for them.
        Key::Up if shift => PanelKey::SelectUp,
        Key::Down if shift => PanelKey::SelectDown,
        Key::Up => PanelKey::Up,
        Key::Down => PanelKey::Down,
        Key::Left => PanelKey::Left,
        Key::Right => PanelKey::Right,
        Key::Enter => PanelKey::Enter,
        Key::Home => PanelKey::Home,
        Key::End => PanelKey::End,
        Key::PageUp => PanelKey::PageUp,
        Key::PageDown => PanelKey::PageDown,
        Key::Escape => PanelKey::Escape,
        Key::Char('d') if ctrl => PanelKey::HalfPageDown,
        Key::Char('u') if ctrl => PanelKey::HalfPageUp,
        // Any other control combination belongs to the application, not here.
        Key::Char(_) if ctrl => return None,
        Key::Char(c) => PanelKey::Char(c),
        _ => return None,
    })
}

/// Single-quotes a path for the shell, which is safe for every character
/// except the quote itself.
fn shell_quote(path: &str) -> String {
    format!("'{}' ", path.replace('\'', "'\\''"))
}

fn read_stdin(buffer: &mut [u8]) -> io::Result<usize> {
    let n = unsafe {
        libc::read(
            libc::STDIN_FILENO,
            buffer.as_mut_ptr() as *mut libc::c_void,
            buffer.len(),
        )
    };
    if n < 0 {
        let err = io::Error::last_os_error();
        if err.kind() == io::ErrorKind::Interrupted {
            return Ok(0);
        }
        return Err(err);
    }
    Ok(n as usize)
}

/// Appends a pane's raw output to `$CARET_DUMP/pane-<id>.bin`, when asked for.
///
/// What a program writes is the only way to tell our handling of a sequence
/// from the program not sending one; `CARET_TRACE` records what we concluded,
/// this records what we were told.
fn dump(id: PtyId, bytes: &[u8]) {
    use std::io::Write;
    let Ok(dir) = std::env::var("CARET_DUMP") else {
        return;
    };
    let _ = std::fs::create_dir_all(&dir);
    let path = std::path::Path::new(&dir).join(format!("pane-{}.bin", id.0));
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = file.write_all(bytes);
    }
}

/// Keeps `slots` holding exactly the live panes, each in the slot it already
/// had, with new panes taking the lowest free one.
///
/// Separate from the session because this is the whole of the rule: a number
/// follows its pane through any rearrangement, and only a pane that has gone
/// gives its number up.
pub fn assign_pane_numbers(slots: &mut Vec<Option<PaneId>>, live: &[PaneId]) {
    for slot in slots.iter_mut() {
        if slot.is_some_and(|id| !live.contains(&id)) {
            *slot = None;
        }
    }
    let waiting: Vec<PaneId> = live
        .iter()
        .copied()
        .filter(|id| !slots.contains(&Some(*id)))
        .collect();
    for id in waiting {
        match slots.iter().position(Option::is_none) {
            Some(free) => slots[free] = Some(id),
            None => slots.push(Some(id)),
        }
    }
    while slots.last() == Some(&None) {
        slots.pop();
    }
}
