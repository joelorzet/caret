#[allow(unused_imports)]
use crate::prelude::*;

/// Commands kept before the oldest are dropped.
const DEFAULT_LIMIT: usize = 5_000;

impl Autocomplete {
    pub fn new(history: Vec<String>) -> Autocomplete {
        Autocomplete {
            history,
            input: String::new(),
            matches: Vec::new(),
            selected: 0,
            limit: DEFAULT_LIMIT,
        }
    }

    pub fn history(&self) -> &[String] {
        &self.history
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    /// Remembers a command that was run.
    ///
    /// An earlier identical entry is removed rather than kept, so repeating a
    /// command promotes it instead of filling the history with duplicates.
    pub fn record(&mut self, command: &str) {
        let command = command.trim();
        if command.is_empty() {
            return;
        }
        self.history.retain(|entry| entry != command);
        self.history.push(command.to_string());

        if self.history.len() > self.limit {
            let excess = self.history.len() - self.limit;
            self.history.drain(..excess);
        }
        self.refresh();
    }

    /// Sets the text being typed. Returns whether the suggestions changed.
    pub fn set_input(&mut self, input: &str) -> bool {
        if self.input == input {
            return false;
        }
        self.input = input.to_string();
        self.refresh();
        true
    }

    pub fn clear(&mut self) {
        self.input.clear();
        self.matches.clear();
        self.selected = 0;
    }

    /// True when there is something to show.
    pub fn is_active(&self) -> bool {
        !self.input.trim().is_empty() && !self.matches.is_empty()
    }

    /// The whole command being suggested.
    pub fn suggestion(&self) -> Option<&str> {
        let index = *self.matches.get(self.selected)?;
        Some(self.history[index].as_str())
    }

    /// Just the part that has not been typed yet, for the ghost text.
    pub fn remainder(&self) -> Option<&str> {
        let suggestion = self.suggestion()?;
        suggestion
            .get(self.input.len()..)
            .filter(|rest| !rest.is_empty())
    }

    pub fn matches(&self) -> impl Iterator<Item = &str> {
        self.matches
            .iter()
            .map(|index| self.history[*index].as_str())
    }

    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn select_next(&mut self) -> bool {
        self.step(1)
    }

    pub fn select_previous(&mut self) -> bool {
        self.step(-1)
    }

    fn step(&mut self, delta: isize) -> bool {
        if self.matches.len() < 2 {
            return false;
        }
        let count = self.matches.len() as isize;
        self.selected = (self.selected as isize + delta).rem_euclid(count) as usize;
        true
    }

    /// Re-ranks the history against the current input.
    ///
    /// Prefix matching rather than fuzzy: a suggestion is accepted by typing
    /// the rest of it, so anything that is not a continuation of what is
    /// already there would be wrong to offer.
    fn refresh(&mut self) {
        self.selected = 0;
        self.matches.clear();

        let input = self.input.trim_start();
        if input.is_empty() {
            return;
        }
        let needle = input.to_lowercase();

        // Newest first: the last thing you ran is the likeliest thing to want.
        for (index, entry) in self.history.iter().enumerate().rev() {
            if entry.len() <= input.len() {
                continue;
            }
            if entry.to_lowercase().starts_with(&needle) {
                self.matches.push(index);
            }
        }
    }
}
