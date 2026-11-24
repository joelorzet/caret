#[allow(unused_imports)]
use crate::prelude::*;

impl Node {
    /// Splits `target` along `axis`, placing `new` after it unless `before`.
    ///
    /// `new` takes half of `target`'s space. When the enclosing split already
    /// runs along `axis` it joins that split as a sibling rather than nesting
    /// a new one.
    pub fn split_pane(&mut self, target: PaneId, axis: Axis, new: PaneId, before: bool) -> bool {
        match self {
            Node::Leaf(id) => {
                if *id != target {
                    return false;
                }
                let existing = Node::Leaf(*id);
                let added = Node::Leaf(new);
                let children = if before {
                    vec![added, existing]
                } else {
                    vec![existing, added]
                };
                *self = Node::Split(Split::new(axis, children));
                true
            }
            Node::Split(sp) => {
                if sp.axis == axis {
                    let direct = sp
                        .children
                        .iter()
                        .position(|c| matches!(c, Node::Leaf(id) if *id == target));
                    if let Some(i) = direct {
                        let half = sp.weights[i] / 2.0;
                        sp.weights[i] = half;
                        let at = if before { i } else { i + 1 };
                        sp.weights.insert(at, half);
                        sp.children.insert(at, Node::Leaf(new));
                        return true;
                    }
                }
                sp.children
                    .iter_mut()
                    .any(|c| c.split_pane(target, axis, new, before))
            }
        }
    }

    /// Removes `target`. Returns true when this node itself became empty and
    /// the caller should drop it.
    pub(crate) fn remove_pane(&mut self, target: PaneId) -> bool {
        let Node::Split(sp) = self else {
            return matches!(self, Node::Leaf(id) if *id == target);
        };

        let hit = sp.children.iter_mut().position(|c| c.remove_pane(target));
        let Some(i) = hit else { return false };

        sp.children.remove(i);
        let freed = sp.weights.remove(i);

        if sp.children.is_empty() {
            return true;
        }
        if sp.children.len() == 1 {
            let only = sp.children.pop().expect("length checked");
            *self = only;
            return false;
        }

        let total: f32 = sp.weights.iter().sum();
        if total > 0.0 {
            for w in sp.weights.iter_mut() {
                *w += freed * (*w / total);
            }
        }
        sp.normalize();
        false
    }

    /// Gives every child of the split directly holding `pane` an equal share.
    ///
    /// A pane that moves lands in a split rather than carving one up, so it
    /// shares that split instead of taking half of whatever it landed on.
    /// Taking half compounded: moving a pane about a few times left a row of
    /// slivers beside one pane holding almost everything.
    pub fn even_split_holding(&mut self, pane: PaneId) -> bool {
        let Node::Split(sp) = self else {
            return false;
        };
        let direct = sp
            .children
            .iter()
            .any(|child| matches!(child, Node::Leaf(id) if *id == pane));
        if direct {
            let share = 1.0 / sp.children.len() as f32;
            for weight in sp.weights.iter_mut() {
                *weight = share;
            }
            return true;
        }
        sp.children
            .iter_mut()
            .any(|child| child.even_split_holding(pane))
    }

    /// Puts `pane` at one end of this node along `axis`, joining the split
    /// already there or wrapping the node in a new one.
    ///
    /// For a pane pushed at the edge of the layout with nothing beyond it:
    /// it leaves whatever it was nested in and becomes a column or row of
    /// the whole tab.
    pub fn place_at_edge(&mut self, axis: Axis, pane: PaneId, before: bool) {
        if let Node::Split(sp) = self {
            if sp.axis == axis {
                let at = if before { 0 } else { sp.children.len() };
                let share = 1.0 / (sp.children.len() + 1) as f32;
                sp.weights.insert(at, share);
                sp.children.insert(at, Node::Leaf(pane));
                sp.normalize();
                return;
            }
        }
        let existing = core::mem::replace(self, Node::Leaf(pane));
        let added = Node::Leaf(pane);
        let children = if before {
            vec![added, existing]
        } else {
            vec![existing, added]
        };
        *self = Node::Split(Split::new(axis, children));
    }

    /// True when `pane` is already the outermost child at `before`'s end of a
    /// split along `axis`, so pushing it that way would change nothing.
    pub fn is_at_edge(&self, axis: Axis, pane: PaneId, before: bool) -> bool {
        let Node::Split(sp) = self else {
            return true;
        };
        if sp.axis != axis {
            return false;
        }
        let end = if before {
            sp.children.first()
        } else {
            sp.children.last()
        };
        matches!(end, Some(Node::Leaf(id)) if *id == pane)
    }

    /// Turns the split holding `target` the other way: a row of panes becomes
    /// a column and back.
    ///
    /// Changing one split in place is what you usually want, and no preset
    /// does it: `main-vertical` and friends rebuild the whole tab.
    pub fn flip_split(&mut self, target: PaneId) -> bool {
        let Node::Split(split) = self else {
            return false;
        };

        let direct = split
            .children
            .iter()
            .any(|child| matches!(child, Node::Leaf(id) if *id == target));
        if direct {
            split.axis = match split.axis {
                Axis::Horizontal => Axis::Vertical,
                Axis::Vertical => Axis::Horizontal,
            };
            return true;
        }

        split
            .children
            .iter_mut()
            .any(|child| child.flip_split(target))
    }

    /// Moves every pane one position along, keeping the shape.
    ///
    /// The same as tmux's rotate-window: the arrangement stays put and the
    /// terminals move through it.
    pub fn rotate_panes(&mut self, forward: bool) -> bool {
        let mut order = self.panes();
        if order.len() < 2 {
            return false;
        }
        if forward {
            order.rotate_right(1);
        } else {
            order.rotate_left(1);
        }

        let mut next = order.into_iter();
        self.relabel_in_order(&mut next);
        true
    }

    fn relabel_in_order(&mut self, next: &mut impl Iterator<Item = PaneId>) {
        match self {
            Node::Leaf(id) => {
                if let Some(replacement) = next.next() {
                    *id = replacement;
                }
            }
            Node::Split(split) => {
                for child in split.children.iter_mut() {
                    child.relabel_in_order(next);
                }
            }
        }
    }

    /// Resets every split to equal shares, like tmux's even-* layouts.
    pub fn equalize(&mut self) {
        if let Node::Split(sp) = self {
            let w = 1.0 / sp.children.len().max(1) as f32;
            sp.weights.fill(w);
            for c in sp.children.iter_mut() {
                c.equalize();
            }
        }
    }

    pub fn swap_panes(&mut self, a: PaneId, b: PaneId) -> bool {
        if !self.contains(a) || !self.contains(b) {
            return false;
        }
        self.relabel(a, b);
        true
    }

    fn relabel(&mut self, a: PaneId, b: PaneId) {
        match self {
            Node::Leaf(id) => {
                if *id == a {
                    *id = b;
                } else if *id == b {
                    *id = a;
                }
            }
            Node::Split(sp) => {
                for c in sp.children.iter_mut() {
                    c.relabel(a, b);
                }
            }
        }
    }
}
