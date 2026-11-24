#[allow(unused_imports)]
use crate::prelude::*;

impl Node {
    pub fn panes(&self) -> Vec<PaneId> {
        let mut out = Vec::new();
        self.collect_panes(&mut out);
        out
    }

    pub(crate) fn collect_panes(&self, out: &mut Vec<PaneId>) {
        match self {
            Node::Leaf(id) => out.push(*id),
            Node::Split(sp) => {
                for c in &sp.children {
                    c.collect_panes(out);
                }
            }
        }
    }

    pub fn pane_count(&self) -> usize {
        match self {
            Node::Leaf(_) => 1,
            Node::Split(sp) => sp.children.iter().map(|c| c.pane_count()).sum(),
        }
    }

    pub fn contains(&self, id: PaneId) -> bool {
        match self {
            Node::Leaf(x) => *x == id,
            Node::Split(sp) => sp.children.iter().any(|c| c.contains(id)),
        }
    }

    pub fn first_pane(&self) -> PaneId {
        match self {
            Node::Leaf(id) => *id,
            Node::Split(sp) => sp.children[0].first_pane(),
        }
    }

    /// Child indices from the root down to `id`.
    pub fn path_to(&self, id: PaneId) -> Option<Vec<usize>> {
        match self {
            Node::Leaf(x) => (*x == id).then(Vec::new),
            Node::Split(sp) => sp.children.iter().enumerate().find_map(|(i, c)| {
                c.path_to(id).map(|mut p| {
                    p.insert(0, i);
                    p
                })
            }),
        }
    }
}
