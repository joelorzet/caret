#[allow(unused_imports)]
use crate::prelude::*;

impl Workspace {
    /// Starts with one tab holding one pane.
    pub fn new() -> Workspace {
        let mut ws = Workspace {
            tabs: Vec::new(),
            active: 0,
            next_pane: 0,
            next_tab: 0,
        };
        ws.new_tab();
        ws
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    pub fn active_index(&self) -> usize {
        self.active
    }

    pub fn active_tab(&self) -> &Tab {
        &self.tabs[self.active]
    }

    pub fn active_tab_mut(&mut self) -> &mut Tab {
        let i = self.active;
        &mut self.tabs[i]
    }

    pub fn active_pane(&self) -> PaneId {
        self.active_tab().focus()
    }

    pub fn panes(&self) -> Vec<PaneId> {
        self.tabs.iter().flat_map(|t| t.panes()).collect()
    }

    pub fn tab_of(&self, pane: PaneId) -> Option<TabId> {
        self.tabs
            .iter()
            .find(|t| t.root.contains(pane))
            .map(|t| t.id)
    }

    pub fn new_tab(&mut self) -> (TabId, PaneId) {
        let tab = TabId(self.next_tab);
        let pane = PaneId(self.next_pane);
        self.next_tab += 1;
        self.next_pane += 1;
        self.tabs.push(Tab::new(tab, pane));
        self.active = self.tabs.len() - 1;
        (tab, pane)
    }

    /// Splits the focused pane and focuses the new one.
    pub fn split(&mut self, axis: Axis, before: bool) -> Option<PaneId> {
        let pane = PaneId(self.next_pane);
        if self.active_tab_mut().split(axis, pane, before) {
            self.next_pane += 1;
            Some(pane)
        } else {
            None
        }
    }

    /// Applies a layout to the active tab, allocating ids as needed.
    pub fn apply_layout(&mut self, template: &LayoutTemplate) -> LayoutChange {
        let mut counter = self.next_pane;
        let change = {
            let tab = self.active_tab_mut();
            tab.apply_layout(template, &mut || {
                let id = PaneId(counter);
                counter += 1;
                id
            })
        };
        self.next_pane = counter;
        change
    }

    pub fn close_pane(&mut self, pane: PaneId) -> CloseOutcome {
        let Some(i) = self.tabs.iter().position(|t| t.root.contains(pane)) else {
            return CloseOutcome::NotFound;
        };

        if let Some(next) = self.tabs[i].close(pane) {
            return CloseOutcome::Pane(next);
        }

        let id = self.tabs[i].id();
        self.tabs.remove(i);
        if self.tabs.is_empty() {
            return CloseOutcome::Empty;
        }
        self.active = self.active.min(self.tabs.len() - 1);
        CloseOutcome::Tab(id)
    }

    pub fn close_tab(&mut self, id: TabId) -> CloseOutcome {
        let Some(i) = self.tabs.iter().position(|t| t.id() == id) else {
            return CloseOutcome::NotFound;
        };
        self.tabs.remove(i);
        if self.tabs.is_empty() {
            return CloseOutcome::Empty;
        }
        self.active = self.active.min(self.tabs.len() - 1);
        CloseOutcome::Tab(id)
    }

    pub fn select_tab(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.active = index;
            true
        } else {
            false
        }
    }

    pub fn next_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active = (self.active + 1) % self.tabs.len();
        }
    }

    pub fn prev_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active = (self.active + self.tabs.len() - 1) % self.tabs.len();
        }
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Workspace::new()
    }
}
