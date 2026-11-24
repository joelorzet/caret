#[allow(unused_imports)]
use crate::prelude::*;

impl Session {
    /// Opens the prompt, keeping the previous query so the same search can be
    /// repeated without retyping it.
    pub(crate) fn open_search(&mut self, previous: Option<String>) {
        let pane = match self.focus {
            FocusTarget::Agent => self.agent.unwrap_or_else(|| self.workspace.active_pane()),
            _ => self.workspace.active_pane(),
        };
        self.search = Some(SearchState {
            pane,
            query: previous.unwrap_or_default(),
            matches: Vec::new(),
            current: 0,
        });
        self.refresh_search();
        self.needs_draw = true;
    }

    pub(crate) fn on_search_key(&mut self, press: &KeyPress) {
        let Some(state) = self.search.as_mut() else {
            return;
        };

        match press.binding.key {
            Key::Escape => {
                self.search = None;
                self.selection = None;
            }
            // Enter keeps the match highlighted and closes the prompt, so it
            // can be copied straight away.
            Key::Enter => self.search = None,
            Key::Backspace => {
                state.query.pop();
                self.refresh_search();
            }
            Key::Up => self.step_search(-1),
            Key::Down => self.step_search(1),
            Key::Char(c) if press.binding.mods.is_empty() => {
                state.query.push(c);
                self.refresh_search();
            }
            _ => {}
        }
        self.needs_draw = true;
    }

    /// Recomputes the matches and jumps to the most recent one, which is what
    /// you want when searching output that has scrolled past.
    fn refresh_search(&mut self) {
        let Some(state) = self.search.as_ref() else {
            return;
        };
        let (pane, query) = (state.pane, state.query.clone());

        let matches = match self.pane(pane) {
            // A lower-case query matches either case; adding a capital makes
            // the search case-sensitive, as in vim and less.
            Some(p) => {
                let sensitive = query.chars().any(char::is_uppercase);
                p.terminal.screen().search_all(&query, sensitive)
            }
            None => Vec::new(),
        };

        let last = matches.len().saturating_sub(1);
        if let Some(state) = self.search.as_mut() {
            state.matches = matches;
            state.current = last;
        }
        self.show_current_match();
    }

    fn step_search(&mut self, delta: isize) {
        let Some(state) = self.search.as_mut() else {
            return;
        };
        if state.matches.is_empty() {
            return;
        }
        let count = state.matches.len() as isize;
        state.current = (state.current as isize + delta).rem_euclid(count) as usize;
        self.show_current_match();
    }

    /// Scrolls the match into view and selects it, which reuses the selection
    /// highlight and makes the copy command work on a search hit.
    fn show_current_match(&mut self) {
        let Some(state) = self.search.as_ref() else {
            return;
        };
        let pane = state.pane;
        let Some((start, end)) = state.matches.get(state.current).copied() else {
            self.selection = None;
            return;
        };

        if let Some(p) = self.pane_mut(pane) {
            p.terminal.screen_mut().scroll_to_row(start.row);
        }
        self.selection = Some((pane, Selection::spanning(start, end, SelectionMode::Cell)));
    }

    /// `/query  3/17`, or a note that nothing matched.
    pub(crate) fn search_prompt(&self) -> Option<String> {
        let state = self.search.as_ref()?;
        Some(if state.matches.is_empty() {
            let suffix = if state.query.is_empty() {
                ""
            } else {
                "  no matches"
            };
            format!(" /{}{suffix}", state.query)
        } else {
            format!(
                " /{}  {}/{}  up and down to move, enter to keep, esc to cancel",
                state.query,
                state.current + 1,
                state.matches.len()
            )
        })
    }
}
