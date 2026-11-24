#[allow(unused_imports)]
use crate::prelude::*;

use crate::impls::host::draw::{MAX_PALETTE_WIDTH, MIN_PALETTE_WIDTH, PALETTE_CHROME};

/// Fuzzy-match scoring: a match that continues a run counts for more, and a
/// first match counts for less the further into the name it is.
const RUN_BONUS: i32 = 6;
const FIRST_MATCH_BONUS: i32 = 10;

impl Session {
    pub(crate) fn open_palette(&mut self, mode: PaletteMode) {
        let items = match mode {
            PaletteMode::Commands => self.command_items(),
            PaletteMode::Themes => self.theme_items(),
            PaletteMode::Settings => self.setting_items(),
        };
        let restore = match mode {
            PaletteMode::Themes => Some(self.theme.name.to_string()),
            PaletteMode::Commands | PaletteMode::Settings => None,
        };

        let mut palette = Palette {
            mode,
            query: String::new(),
            items,
            rows: Vec::new(),
            capturing: None,
            selected: 0,
            offset: 0,
            height: 10,
            restore,
        };
        palette.refilter();

        // Open the theme picker on the theme in use, so the list starts where
        // the eye expects rather than at the top.
        if mode == PaletteMode::Themes {
            if let Some(at) = palette.position_of(&self.theme.name) {
                palette.selected = at;
                palette.scroll_into_view();
            }
        }
        self.palette = Some(palette);
        self.needs_draw = true;
    }

    fn command_items(&self) -> Vec<PaletteItem> {
        self.config
            .keymap
            .shortcuts()
            .into_iter()
            // The group the action belongs to is kept rather than flattened
            // away: a list of every binding tells you what exists, and the
            // topic beside each one tells you what it is for.
            .flat_map(|(group, rows)| rows.into_iter().map(move |shortcut| (group, shortcut)))
            .map(|(group, shortcut)| {
                let keys = shortcut
                    .keys
                    .iter()
                    .find(|key| self.reachable(key))
                    .cloned()
                    .unwrap_or_default();
                PaletteItem {
                    section: group.title().to_string(),
                    label: shortcut.description,
                    hint: keys,
                    kind: PaletteKind::Run(shortcut.action),
                }
            })
            .collect()
    }

    fn setting_items(&self) -> Vec<PaletteItem> {
        // The file comes first, because it is where a change is actually
        // made and nobody should have to guess that enter does it.
        let mut items = vec![
            PaletteItem {
                section: "config".into(),
                label: "Open the config file".into(),
                hint: "edit anything by hand".into(),
                kind: PaletteKind::Run(Action::EditConfig),
            },
            PaletteItem {
                section: "config".into(),
                label: "Reload the config file".into(),
                hint: "apply what you changed".into(),
                kind: PaletteKind::Run(Action::ReloadConfig),
            },
        ];
        items.extend(
            self.config
                .settings()
                .into_iter()
                .map(|setting| PaletteItem {
                    section: setting.section.to_string(),
                    label: format!("{} = {}", setting.key, setting.value),
                    hint: setting.about.to_string(),
                    kind: PaletteKind::Setting {
                        table: setting.table.to_string(),
                        key: setting.key.to_string(),
                        value: quoted(&setting.value, setting.key),
                    },
                }),
        );
        items
    }

    fn theme_items(&self) -> Vec<PaletteItem> {
        self.themes
            .themes()
            .iter()
            .map(|theme| PaletteItem {
                section: theme.appearance.name().to_string(),
                label: theme.name.to_string(),
                hint: theme.description.to_string(),
                kind: PaletteKind::Theme(theme.name.to_string()),
            })
            .collect()
    }

    /// Where the palette sits.
    ///
    /// Shared with drawing and with hit testing, because a second copy of this
    /// arithmetic is how the preview ended up scrolling past its own frame.
    pub(crate) fn palette_rect(&self) -> Option<Rect> {
        let palette = self.palette.as_ref()?;
        let rows = palette.rows.len().min(palette.height) as u16;
        Some(palette_rect(self.size, rows))
    }

    /// The row the list starts on, inside the frame.
    pub(crate) const LIST_TOP: u16 = 3;

    /// Clicks, the wheel and plain movement over the palette.
    ///
    /// Movement is deliberately inert. The palette used to close on any mouse
    /// event at all, so brushing the mouse while reading the list dismissed
    /// it.
    pub(crate) fn palette_mouse(&mut self, event: MouseEvent) -> std::io::Result<bool> {
        let Some(rect) = self.palette_rect() else {
            return Ok(false);
        };

        if event.is_wheel() {
            let delta = if event.button == MouseButton::WheelUp {
                -3
            } else {
                3
            };
            if let Some(palette) = self.palette.as_mut() {
                palette.step(delta);
            }
            self.preview();
            self.needs_draw = true;
            return Ok(true);
        }

        if event.kind != MouseKind::Press {
            return Ok(true);
        }

        // Clicking away is how a menu is dismissed everywhere else.
        if !rect.contains(event.col, event.row) {
            self.close_palette(true);
            self.needs_draw = true;
            return Ok(true);
        }

        let Some(palette) = self.palette.as_ref() else {
            return Ok(true);
        };
        let first = rect.y + Self::LIST_TOP;
        if event.row < first {
            return Ok(true);
        }
        let row = palette.offset + (event.row - first) as usize;
        // A heading is a label, not a choice.
        if palette.rows.get(row).and_then(PaletteRow::entry).is_none() {
            return Ok(true);
        }

        if let Some(palette) = self.palette.as_mut() {
            palette.selected = row;
        }
        self.needs_draw = true;
        self.accept_palette()?;
        Ok(true)
    }

    pub(crate) fn palette_key(&mut self, press: &KeyPress) -> std::io::Result<()> {
        let Some(palette) = self.palette.as_mut() else {
            return Ok(());
        };
        self.needs_draw = true;

        // While a shortcut is being captured every key is the answer, so
        // nothing else can be typed until it is given or escaped.
        if palette.capturing.is_some() {
            return self.capture_binding(press);
        }

        match press.binding.key {
            Key::Escape => self.close_palette(true),
            // Rebinding, in the list where you found the command. The file
            // is still the record: this writes to it.
            Key::Char('e') if press.binding.mods.contains(Modifiers::CTRL) => {
                let target = palette.current().and_then(|item| match &item.kind {
                    PaletteKind::Run(action) => Some((action.clone(), item.label.clone())),
                    _ => None,
                });
                palette.capturing = target;
            }
            Key::Enter => return self.accept_palette(),
            Key::Up => {
                palette.step(-1);
                self.preview();
            }
            Key::Down => {
                palette.step(1);
                self.preview();
            }
            Key::PageUp => {
                let page = palette.height as isize;
                palette.step(-page);
                self.preview();
            }
            Key::PageDown => {
                let page = palette.height as isize;
                palette.step(page);
                self.preview();
            }
            Key::Backspace => {
                palette.query.pop();
                palette.refilter();
                self.preview();
            }
            Key::Char(c) if press.binding.mods.without_shift() == Modifiers::NONE => {
                palette.query.push(c);
                palette.refilter();
                self.preview();
            }
            _ => {}
        }
        Ok(())
    }

    /// Takes the key pressed and makes it the shortcut.
    fn capture_binding(&mut self, press: &KeyPress) -> std::io::Result<()> {
        let Some(palette) = self.palette.as_mut() else {
            return Ok(());
        };
        if press.binding.key == Key::Escape {
            palette.capturing = None;
            return Ok(());
        }
        // A bare modifier is the start of a chord, not the chord.
        if matches!(press.binding.key, Key::Char(c) if c == '\0') {
            return Ok(());
        }

        let Some((action, label)) = palette.capturing.take() else {
            return Ok(());
        };
        let keys = press.binding.to_string();
        let name = action.name();
        self.close_palette(false);

        self.message = match Config::save_binding(&keys, &name) {
            Ok(path) => {
                self.reload_config();
                format!(" {keys} now runs {label}, saved to {}", path.display())
            }
            Err(e) => format!(" cannot save {keys}: {e}"),
        };
        Ok(())
    }

    /// Applies the highlighted theme straight away, so the list is a preview
    /// rather than a guess.
    pub(crate) fn preview(&mut self) {
        let Some(palette) = self.palette.as_ref() else {
            return;
        };
        if palette.mode != PaletteMode::Themes {
            return;
        }
        let Some(PaletteKind::Theme(name)) = palette.current().map(|item| item.kind.clone()) else {
            return;
        };
        self.apply_theme(self.themes.resolve(&name).clone());
    }

    pub(crate) fn accept_palette(&mut self) -> std::io::Result<()> {
        let Some(palette) = self.palette.as_ref() else {
            return Ok(());
        };
        let chosen = palette.current().map(|item| item.kind.clone());
        self.close_palette(false);

        match chosen {
            Some(PaletteKind::Run(action)) => self.perform(action)?,
            Some(PaletteKind::Theme(name)) => {
                self.apply_theme(self.themes.resolve(&name).clone());
                self.message = match Config::save_theme(&name) {
                    Ok(path) => format!(" theme {name}, saved to {}", path.display()),
                    Err(e) => format!(" theme {name} for this session only: {e}"),
                };
            }
            // A setting is read here and changed in the file, which is the
            // one place a change survives, so this opens the file on it.
            Some(PaletteKind::Setting { table, key, value }) => {
                self.edit_setting(Some((&table, &key, &value)))?
            }
            None => {}
        }
        Ok(())
    }

    fn close_palette(&mut self, cancelled: bool) {
        let Some(palette) = self.palette.take() else {
            return;
        };
        if cancelled {
            if let Some(name) = palette.restore {
                self.apply_theme(self.themes.resolve(&name).clone());
            }
        }
        for (_, pane) in self.panes.iter_mut() {
            pane.terminal.screen_mut().damage_all();
        }
        self.needs_draw = true;
    }
}

impl Palette {
    pub(crate) fn current(&self) -> Option<&PaletteItem> {
        self.items.get(self.rows.get(self.selected)?.entry()?)
    }

    pub(crate) fn visible(&self) -> impl Iterator<Item = (usize, &PaletteRow)> {
        self.rows
            .iter()
            .enumerate()
            .skip(self.offset)
            .take(self.height)
    }

    pub(crate) fn item(&self, row: &PaletteRow) -> Option<&PaletteItem> {
        self.items.get(row.entry()?)
    }

    pub(crate) fn title(&self) -> &'static str {
        match self.mode {
            PaletteMode::Commands => " Commands ",
            PaletteMode::Themes => " Themes ",
            PaletteMode::Settings => " Settings ",
        }
    }

    /// What typing here does, shown while the line is empty.
    pub(crate) fn placeholder(&self) -> &'static str {
        match self.mode {
            PaletteMode::Commands => "type to find a command, or a topic",
            PaletteMode::Themes => "type to find a theme",
            PaletteMode::Settings => "type to find a setting, or a section",
        }
    }

    /// Key hints for the footer, as `key label` pairs.
    pub(crate) fn hints(&self) -> &'static [(&'static str, &'static str)] {
        match self.mode {
            PaletteMode::Commands => &[
                ("enter", "run"),
                ("ctrl+e", "rebind"),
                ("arrows", "choose"),
                ("esc", "close"),
            ],
            PaletteMode::Themes => &[
                ("enter", "keep and save"),
                ("arrows", "preview"),
                ("esc", "restore"),
            ],
            PaletteMode::Settings => &[
                ("enter", "open the file here"),
                ("type", "filter"),
                ("esc", "close"),
            ],
        }
    }

    pub(crate) fn set_height(&mut self, height: usize) {
        self.height = height.max(1);
        self.scroll_into_view();
    }

    fn position_of(&self, label: &str) -> Option<usize> {
        self.rows.iter().position(|row| {
            row.entry()
                .and_then(|index| self.items.get(index))
                .is_some_and(|item| item.label == label)
        })
    }

    /// Moves the selection by `delta` entries, stepping over the headings:
    /// a heading names what follows it, it is not something to choose.
    pub(crate) fn step(&mut self, delta: isize) {
        let direction = if delta < 0 { -1 } else { 1 };
        let mut at = self.selected as isize;
        for _ in 0..delta.abs() {
            let Some(next) = self.next_entry(at, direction) else {
                break;
            };
            at = next;
        }
        self.selected = at.max(0) as usize;
        self.scroll_into_view();
    }

    fn next_entry(&self, from: isize, direction: isize) -> Option<isize> {
        let mut at = from + direction;
        while (0..self.rows.len() as isize).contains(&at) {
            if self.rows[at as usize].entry().is_some() {
                return Some(at);
            }
            at += direction;
        }
        None
    }

    fn scroll_into_view(&mut self) {
        // A heading immediately above the selection is brought along with it,
        // so the first entry of a group never appears without its name.
        let anchor = match self.selected.checked_sub(1) {
            Some(above) if self.rows.get(above).is_some_and(|r| r.entry().is_none()) => above,
            _ => self.selected,
        };
        if anchor < self.offset {
            self.offset = anchor;
        } else if self.selected >= self.offset + self.height {
            self.offset = self.selected + 1 - self.height;
        }
        self.offset = self.offset.min(self.rows.len().saturating_sub(self.height));
    }

    /// Re-ranks the entries against the query, best first.
    pub(crate) fn refilter(&mut self) {
        let scored: Vec<(i32, usize)> = self
            .items
            .iter()
            .enumerate()
            .filter_map(|(index, item)| {
                let label = fuzzy(&item.label, &self.query);
                // Matching the keys as well lets you type the binding you
                // half-remember instead of the wording.
                let hint = fuzzy(&item.hint, &self.query).map(|score| score - 10);
                // The topic matches too, so typing "pane" or "font" narrows a
                // long list to one subject without knowing any wording.
                let section = fuzzy(&item.section, &self.query).map(|score| score - 5);
                label
                    .into_iter()
                    .chain(hint)
                    .chain(section)
                    .max()
                    .map(|s| (s, index))
            })
            .collect();

        self.rows = group_rows(&self.items, &scored);
        self.selected = self
            .rows
            .iter()
            .position(|row| row.entry().is_some())
            .unwrap_or(0);
        self.offset = 0;
    }
}

/// Subsequence match: every character of `needle` in order, not necessarily
/// adjacent. Runs and early matches score higher, so "spl" finds "Split the
/// pane..." ahead of "Scroll back one page".
fn fuzzy(haystack: &str, needle: &str) -> Option<i32> {
    if needle.is_empty() {
        return Some(0);
    }
    let hay: Vec<char> = haystack.to_lowercase().chars().collect();
    let mut score = 0;
    let mut at = 0;
    let mut previous: Option<usize> = None;

    for wanted in needle.to_lowercase().chars() {
        if wanted == ' ' {
            continue;
        }
        let found = hay[at..].iter().position(|c| *c == wanted)? + at;
        match previous {
            Some(prev) if found == prev + 1 => score += RUN_BONUS,
            Some(prev) => score -= (found - prev) as i32,
            None => score += FIRST_MATCH_BONUS - (found as i32).min(FIRST_MATCH_BONUS),
        }
        previous = Some(found);
        at = found + 1;
    }
    Some(score)
}

/// The palette's frame for a window of this size holding this many rows.
pub fn palette_rect(size: (u16, u16), rows: u16) -> Rect {
    // Three quarters rather than three fifths: each row now carries its
    // topic as well as its description, and a description cut in half says
    // less than no description at all.
    let width =
        (size.0 * 3 / 4).clamp(MIN_PALETTE_WIDTH.min(size.0), MAX_PALETTE_WIDTH.min(size.0));
    let height = (rows + PALETTE_CHROME).min(size.1);
    Rect::new(
        size.0.saturating_sub(width) / 2,
        size.1.saturating_sub(height) / 3,
        width,
        height,
    )
}

/// Lays the matches out as headings and entries.
///
/// Grouped by topic, in the order the topics were built, and best match first
/// within each. A flat ranking put related entries far apart and gave the
/// list no shape at all; a topic nothing matched is left out entirely.
pub fn group_rows(items: &[PaletteItem], scored: &[(i32, usize)]) -> Vec<PaletteRow> {
    let mut topics: Vec<&str> = Vec::new();
    for item in items {
        if !topics.contains(&item.section.as_str()) {
            topics.push(&item.section);
        }
    }

    let mut rows = Vec::new();
    for topic in topics {
        let mut group: Vec<(i32, usize)> = scored
            .iter()
            .copied()
            .filter(|(_, index)| items[*index].section == topic)
            .collect();
        if group.is_empty() {
            continue;
        }
        group.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        rows.push(PaletteRow::Heading(topic.to_string(), group.len()));
        rows.extend(group.into_iter().map(|(_, index)| PaletteRow::Entry(index)));
    }
    rows
}

/// A value as the file would spell it.
///
/// The settings list prints values for people; a file needs them as TOML, so
/// anything that is not plainly a number, a boolean or already a list goes
/// back in quotes.
fn quoted(value: &str, key: &str) -> String {
    let plain = value == "true"
        || value == "false"
        || value.starts_with('[')
        || (value.parse::<f64>().is_ok() && key != "family");
    if plain {
        value.to_string()
    } else {
        format!("{value:?}")
    }
}
