#[allow(unused_imports)]
use crate::prelude::*;

impl Session {
    pub(crate) fn open_rearrange(&mut self) {
        if self.workspace.active_tab().pane_count() < 2 {
            self.message = " only one pane to arrange".into();
            return;
        }
        let mut draft = self.workspace.active_tab().clone();
        // Zoom hides every pane but one, and the panel is about the others.
        if draft.zoomed().is_some() {
            draft.toggle_zoom();
        }
        self.rearrange = Some(Rearrange { draft });
        self.needs_draw = true;
    }

    /// Keys while the arrange panel is open. It claims all of them, the way
    /// the command palette does.
    pub(crate) fn rearrange_key(&mut self, press: &KeyPress) {
        self.needs_draw = true;

        match press.binding.key {
            Key::Escape => {
                self.rearrange = None;
                return;
            }
            Key::Enter => {
                self.apply_rearrange();
                return;
            }
            _ => {}
        }

        // Which pane is left of which is decided against the area the tab
        // will really occupy, not against the panel's diagram: what a move
        // does should not depend on how big the panel happens to be, and a
        // window too small to draw a diagram in would otherwise answer the
        // question from boxes of no size.
        let area = self.center_rect();
        // A digit picks the pane wearing that number, which is not where it
        // sits: the whole point of a stable number is that it survives a move.
        let mut picked: Option<usize> = None;

        let Some(state) = self.rearrange.as_mut() else {
            return;
        };

        let dir = match press.binding.key {
            Key::Left | Key::Char('h') | Key::Char('H') => Some(Direction::Left),
            Key::Right | Key::Char('l') | Key::Char('L') => Some(Direction::Right),
            Key::Up | Key::Char('k') | Key::Char('K') => Some(Direction::Up),
            Key::Down | Key::Char('j') | Key::Char('J') => Some(Direction::Down),
            _ => None,
        };
        if let Some(dir) = dir {
            // A capital carries the pane, for keyboards that report shift and
            // an arrow as the bare arrow.
            let carrying = press.binding.mods.contains(Modifiers::SHIFT)
                || matches!(press.binding.key, Key::Char('H' | 'J' | 'K' | 'L'));
            if carrying {
                state.draft.join_pane(area, dir);
            } else {
                state.draft.focus_dir(area, dir);
            }
            return;
        }

        match press.binding.key {
            Key::Char('t') => {
                state.draft.flip_split();
            }
            Key::Char('r') => {
                state.draft.rotate_panes(true);
            }
            Key::Char('R') => {
                state.draft.rotate_panes(false);
            }
            Key::Char('=') => state.draft.equalize(),
            Key::Char(c) if c.is_ascii_digit() && c != '0' => {
                picked = c.to_digit(10).map(|d| d as usize);
            }
            _ => {}
        }

        if let Some(id) = picked.and_then(|number| self.pane_numbered(number)) {
            if let Some(state) = self.rearrange.as_mut() {
                state.draft.set_focus(id);
            }
        }
    }

    fn apply_rearrange(&mut self) {
        let Some(state) = self.rearrange.take() else {
            return;
        };
        let focus = state.draft.focus();
        self.workspace
            .active_tab_mut()
            .set_layout(state.draft.layout(), focus);
    }

    /// Picks the pane under the pointer out of the panel's diagram.
    pub(crate) fn rearrange_mouse(&mut self, event: MouseEvent) {
        if event.kind != MouseKind::Press {
            return;
        }
        self.needs_draw = true;
        let canvas = self.arrange_canvas();
        if !canvas.contains(event.col, event.row) {
            // Clicking away is the same as walking away, as with the palette.
            if !self.arrange_rect().contains(event.col, event.row) {
                self.rearrange = None;
            }
            return;
        }
        let Some(state) = self.rearrange.as_mut() else {
            return;
        };
        let hit = state
            .draft
            .resolve(canvas)
            .panes
            .into_iter()
            .find(|(_, rect)| rect.contains(event.col, event.row));
        if let Some((id, _)) = hit {
            state.draft.set_focus(id);
        }
    }

    pub(crate) fn is_rearranging(&self) -> bool {
        self.rearrange.is_some()
    }
}
