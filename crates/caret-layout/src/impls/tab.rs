#[allow(unused_imports)]
use crate::prelude::*;

impl Tab {
    pub fn new(id: TabId, pane: PaneId) -> Tab {
        Tab {
            id,
            title: String::new(),
            root: Node::leaf(pane),
            focus: pane,
            zoomed: None,
            last_focus: None,
        }
    }

    pub fn id(&self) -> TabId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn focus(&self) -> PaneId {
        self.focus
    }

    pub fn set_focus(&mut self, pane: PaneId) -> bool {
        if !self.root.contains(pane) || pane == self.focus {
            return self.root.contains(pane);
        }
        self.last_focus = Some(self.focus);
        self.focus = pane;
        true
    }

    pub fn last_focus(&self) -> Option<PaneId> {
        self.last_focus
    }

    /// Index of the focused pane in layout order, counting from 1.
    pub fn focus_index(&self) -> usize {
        self.panes()
            .iter()
            .position(|p| *p == self.focus)
            .map_or(1, |i| i + 1)
    }

    /// Focuses the nth pane in layout order, counting from 1.
    pub fn focus_nth(&mut self, index: usize) -> bool {
        let panes = self.panes();
        match index.checked_sub(1).and_then(|i| panes.get(i)) {
            Some(pane) => self.set_focus(*pane),
            None => false,
        }
    }

    pub fn focus_next(&mut self) -> bool {
        self.cycle_focus(1)
    }

    pub fn focus_prev(&mut self) -> bool {
        self.cycle_focus(-1)
    }

    fn cycle_focus(&mut self, step: isize) -> bool {
        let panes = self.panes();
        if panes.len() < 2 {
            return false;
        }
        let at = panes.iter().position(|p| *p == self.focus).unwrap_or(0) as isize;
        let next = (at + step).rem_euclid(panes.len() as isize) as usize;
        self.set_focus(panes[next])
    }

    /// Returns to the previously focused pane.
    pub fn focus_last(&mut self) -> bool {
        match self.last_focus {
            Some(pane) => self.set_focus(pane),
            None => false,
        }
    }

    /// Rebuilds the tab to match `template`.
    ///
    /// Existing panes are reused in layout order, so rearranging never
    /// restarts a shell. Ids are drawn from `next_id` only when the template
    /// needs more panes than the tab has.
    pub fn apply_layout(
        &mut self,
        template: &LayoutTemplate,
        next_id: &mut impl FnMut() -> PaneId,
    ) -> LayoutChange {
        let existing = self.panes();
        let want = template.pane_count(existing.len()).max(1);

        let mut ids: Vec<PaneId> = existing.iter().copied().take(want).collect();
        let mut created = Vec::new();
        while ids.len() < want {
            let id = next_id();
            created.push(id);
            ids.push(id);
        }
        let removed: Vec<PaneId> = existing.iter().copied().skip(want).collect();

        let shape = template.shape_for(existing.len());
        let mut supply = ids.iter().copied();
        self.root = shape.build(&mut || supply.next().expect("ids cover every pane"));

        if !ids.contains(&self.focus) {
            self.focus = ids[0];
        }
        self.last_focus = self.last_focus.filter(|p| ids.contains(p));
        self.zoomed = None;

        LayoutChange { created, removed }
    }

    pub fn panes(&self) -> Vec<PaneId> {
        self.root.panes()
    }

    pub fn pane_count(&self) -> usize {
        self.root.pane_count()
    }

    pub fn zoomed(&self) -> Option<PaneId> {
        self.zoomed
    }

    pub fn toggle_zoom(&mut self) -> bool {
        if self.zoomed.is_some() {
            self.zoomed = None;
            false
        } else if self.root.pane_count() > 1 {
            self.zoomed = Some(self.focus);
            true
        } else {
            false
        }
    }

    /// Geometry for the current area. A zoomed pane takes the whole area and
    /// the rest are not reported, so the renderer skips them entirely.
    pub fn resolve(&self, area: Rect) -> Resolved {
        match self.zoomed {
            Some(pane) if self.root.contains(pane) => Resolved {
                panes: vec![(pane, area)],
                dividers: Vec::new(),
            },
            _ => self.root.resolve(area),
        }
    }

    /// Turns the split holding the focused pane the other way.
    pub fn flip_split(&mut self) -> bool {
        let focus = self.focus;
        let flipped = self.root.flip_split(focus);
        if flipped {
            self.zoomed = None;
        }
        flipped
    }

    /// Moves every pane one position along, keeping the arrangement.
    pub fn rotate_panes(&mut self, forward: bool) -> bool {
        let rotated = self.root.rotate_panes(forward);
        if rotated {
            self.zoomed = None;
            // Focus follows the pane, not the position it used to hold.
            if !self.root.contains(self.focus) {
                self.focus = self.root.first_pane();
            }
        }
        rotated
    }

    /// Pairs the focused pane with its neighbour in `dir`, arranging the two
    /// the other way round from how they sit now.
    ///
    /// Side by side becomes stacked, stacked becomes side by side. That is the
    /// only thing joining two panes that are already adjacent can mean, and it
    /// is what `flip_split` cannot do: flipping turns a whole row at once,
    /// where this peels one pane off and pairs it with a single neighbour.
    ///
    /// The terminal moves with the pane, so a long-running command keeps
    /// going; nothing is closed and nothing is started.
    pub fn join_pane(&mut self, area: Rect, dir: Direction) -> bool {
        let moving = self.focus;
        let Some(target) = self.neighbour(area, dir) else {
            // Nothing that way, so there is nothing to pair with. Push the
            // pane out instead: it leaves whatever it is nested in and
            // becomes a column or row of the whole tab, which is how a
            // stacked pair goes back to sitting beside everything else.
            return self.push_to_edge(dir);
        };

        // Out before back in, so the pane is never in the tree twice and the
        // split it leaves behind collapses exactly as a close would.
        if self.root.remove_pane(moving) {
            return false;
        }

        let axis = match dir {
            Direction::Left | Direction::Right => Axis::Vertical,
            Direction::Up | Direction::Down => Axis::Horizontal,
        };
        // Whichever was ahead stays ahead: moving onto the pane at your left
        // puts you below it, not above.
        let before = matches!(dir, Direction::Right | Direction::Down);

        if !self.root.split_pane(target, axis, moving, before) {
            // The pane is out of the tree at this point, and a pane that
            // failed to land would take its running program with it.
            let anywhere = self.root.first_pane();
            self.root.split_pane(anywhere, axis, moving, false);
        }
        self.root.even_split_holding(moving);
        self.focus = moving;
        self.zoomed = None;
        true
    }

    /// Takes the focused pane out of its split and puts it at the far edge
    /// of the tab, along `dir`.
    fn push_to_edge(&mut self, dir: Direction) -> bool {
        let moving = self.focus;
        if self.pane_count() < 2 {
            return false;
        }
        let axis = dir.axis();
        let before = !dir.is_forward();
        if self.root.is_at_edge(axis, moving, before) {
            return false;
        }
        if self.root.remove_pane(moving) {
            return false;
        }
        self.root.place_at_edge(axis, moving, before);
        self.root.even_split_holding(moving);
        self.focus = moving;
        self.zoomed = None;
        true
    }

    pub fn swap(&mut self, a: PaneId, b: PaneId) -> bool {
        self.root.swap_panes(a, b)
    }

    /// The arrangement as it stands, so a rearrangement can be abandoned and
    /// the tab put back exactly as it was. Only the shape is copied; the
    /// panes and the terminals behind them are untouched either way.
    pub fn layout(&self) -> Node {
        self.root.clone()
    }

    pub fn set_layout(&mut self, root: Node, focus: PaneId) {
        self.root = root;
        if !self.set_focus(focus) {
            self.focus = self.root.first_pane();
        }
        self.zoomed = None;
    }

    pub fn equalize(&mut self) {
        self.root.equalize();
    }

    pub fn split(&mut self, axis: Axis, new: PaneId, before: bool) -> bool {
        let ok = self.root.split_pane(self.focus, axis, new, before);
        if ok {
            self.focus = new;
            self.zoomed = None;
        }
        ok
    }

    /// Removes `pane`. Returns the new focus, or `None` when the tab is now
    /// empty and should be closed.
    pub fn close(&mut self, pane: PaneId) -> Option<PaneId> {
        let order = self.root.panes();
        let idx = order.iter().position(|p| *p == pane)?;

        if self.root.remove_pane(pane) {
            return None;
        }

        let remaining = self.root.panes();
        let next = remaining[idx.min(remaining.len() - 1)];
        if self.focus == pane {
            self.focus = next;
        }
        if self.zoomed == Some(pane) {
            self.zoomed = None;
        }
        Some(self.focus)
    }

    /// The pane adjacent to the focused one in `dir`, chosen the way tmux
    /// does: prefer panes whose edge actually faces the current one, then the
    /// nearest, then the one sharing the most border.
    pub fn neighbour(&self, area: Rect, dir: Direction) -> Option<PaneId> {
        let resolved = self.root.resolve(area);
        let current = resolved.panes.iter().find(|(id, _)| *id == self.focus)?.1;

        resolved
            .panes
            .iter()
            .filter(|(id, _)| *id != self.focus)
            .filter_map(|(id, r)| {
                let (gap, overlap) = match dir {
                    Direction::Left => (current.x.checked_sub(r.right())?, current.overlap_y(r)),
                    Direction::Right => (r.x.checked_sub(current.right())?, current.overlap_y(r)),
                    Direction::Up => (current.y.checked_sub(r.bottom())?, current.overlap_x(r)),
                    Direction::Down => (r.y.checked_sub(current.bottom())?, current.overlap_x(r)),
                };
                Some((*id, gap, overlap))
            })
            .min_by_key(|&(id, gap, overlap)| {
                (u8::from(overlap == 0), gap, u16::MAX - overlap, id.0)
            })
            .map(|(id, _, _)| id)
    }

    pub fn focus_dir(&mut self, area: Rect, dir: Direction) -> bool {
        match self.neighbour(area, dir) {
            Some(next) => {
                self.focus = next;
                true
            }
            None => false,
        }
    }

    /// Moves the focused pane's border `cells` in `dir`.
    ///
    /// Walks the ancestors of the focused pane and uses the deepest split that
    /// runs along `dir`, so the border that moves is the one the user sees
    /// next to the pane rather than an outer one.
    pub fn resize(&mut self, area: Rect, dir: Direction, cells: u16) -> bool {
        if cells == 0 || self.zoomed.is_some() {
            return false;
        }
        let axis = dir.axis();
        let path = match self.root.path_to(self.focus) {
            Some(p) => p,
            None => return false,
        };

        let mut target: Option<(usize, Rect)> = None;
        let mut node = &self.root;
        let mut rect = area;
        for (depth, &child) in path.iter().enumerate() {
            let Node::Split(sp) = node else { break };
            if sp.axis == axis && sp.children.len() > 1 {
                target = Some((depth, rect));
            }
            let rects = sp.child_rects(rect);
            rect = rects[child];
            node = &sp.children[child];
        }

        let Some((depth, split_rect)) = target else {
            return false;
        };

        let mut node = &mut self.root;
        for &child in &path[..depth] {
            let Node::Split(sp) = node else { return false };
            node = &mut sp.children[child];
        }
        let Node::Split(sp) = node else { return false };

        let index = path[depth];
        let count = sp.children.len();
        // Move the border that faces `dir`, preferring the one after the pane.
        // Rightmost pane resized left grows (its left border moves left);
        // otherwise it shrinks (its right border moves left). This is what
        // tmux's resize-pane does.
        let after = index + 1 < count;
        let before = index > 0;
        let (grow, shrink) = match (dir.is_forward(), after, before) {
            (true, true, _) => (index, index + 1),
            (true, false, true) => (index - 1, index),
            (false, true, _) => (index + 1, index),
            (false, false, true) => (index, index - 1),
            _ => return false,
        };

        let usable = sp.usable(split_rect).max(1) as f32;
        let floor = 1.0 / usable;
        let wanted = cells as f32 / usable;
        let moved = wanted.min(sp.weights[shrink] - floor).max(0.0);
        if moved <= 0.0 {
            return false;
        }
        sp.weights[grow] += moved;
        sp.weights[shrink] -= moved;
        true
    }
}
