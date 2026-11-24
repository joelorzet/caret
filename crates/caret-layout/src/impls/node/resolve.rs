#[allow(unused_imports)]
use crate::prelude::*;

impl Node {
    pub fn resolve(&self, area: Rect) -> Resolved {
        let mut out = Resolved::default();
        self.resolve_into(area, &mut out);
        out
    }

    pub(crate) fn resolve_into(&self, area: Rect, out: &mut Resolved) {
        match self {
            Node::Leaf(id) => out.panes.push((*id, area)),
            Node::Split(sp) => {
                let rects = sp.child_rects(area);
                let last = rects.len().saturating_sub(1);
                for (i, (child, rect)) in sp.children.iter().zip(&rects).enumerate() {
                    child.resolve_into(*rect, out);
                    if i < last {
                        out.dividers.push(Divider {
                            rect: match sp.axis {
                                Axis::Horizontal => Rect::new(rect.right(), rect.y, 1, rect.h),
                                Axis::Vertical => Rect::new(rect.x, rect.bottom(), rect.w, 1),
                            },
                            axis: sp.axis,
                        });
                    }
                }
            }
        }
    }
}
