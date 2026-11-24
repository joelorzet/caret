#[allow(unused_imports)]
use crate::prelude::*;

impl LayoutNode {
    pub fn pane() -> LayoutNode {
        LayoutNode {
            weight: 0.0,
            kind: LayoutKind::Pane,
        }
    }

    pub fn split(axis: Axis, children: Vec<LayoutNode>) -> LayoutNode {
        LayoutNode {
            weight: 0.0,
            kind: LayoutKind::Split { axis, children },
        }
    }

    pub fn weighted(mut self, weight: f32) -> LayoutNode {
        self.weight = weight;
        self
    }

    pub fn pane_count(&self) -> usize {
        match &self.kind {
            LayoutKind::Pane => 1,
            LayoutKind::Split { children, .. } => children.iter().map(|c| c.pane_count()).sum(),
        }
    }

    /// Instantiates the shape, taking pane ids from `next` in layout order.
    pub fn build(&self, next: &mut impl FnMut() -> PaneId) -> Node {
        match &self.kind {
            LayoutKind::Pane => Node::Leaf(next()),
            LayoutKind::Split { axis, children } => {
                if children.len() == 1 {
                    return children[0].build(next);
                }
                let built: Vec<Node> = children.iter().map(|c| c.build(next)).collect();
                let weights = shares(children);
                Node::Split(Split::with_weights(*axis, built, weights))
            }
        }
    }
}

/// Explicit weights where given, an equal share of what is left otherwise.
fn shares(children: &[LayoutNode]) -> Vec<f32> {
    let given: f32 = children.iter().map(|c| c.weight.max(0.0)).sum();
    let unset = children.iter().filter(|c| c.weight <= 0.0).count();

    if unset == 0 {
        return children.iter().map(|c| c.weight).collect();
    }

    // Spread whatever the explicit weights left over, or divide evenly when
    // nothing was specified.
    let spare = (100.0 - given).max(0.0);
    let each = if spare > 0.0 {
        spare / unset as f32
    } else {
        100.0 / children.len() as f32
    };
    children
        .iter()
        .map(|c| if c.weight > 0.0 { c.weight } else { each })
        .collect()
}
