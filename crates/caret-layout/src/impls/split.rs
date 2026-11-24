#[allow(unused_imports)]
use crate::prelude::*;

impl Split {
    pub fn new(axis: Axis, children: Vec<Node>) -> Split {
        let n = children.len().max(1);
        let w = 1.0 / n as f32;
        Split {
            axis,
            weights: vec![w; children.len()],
            children,
        }
    }

    /// Normalises `weights` against `children`, padding or truncating to fit.
    pub fn with_weights(axis: Axis, children: Vec<Node>, weights: Vec<f32>) -> Split {
        let mut weights = weights;
        weights.resize(children.len(), 0.0);
        let mut split = Split {
            axis,
            children,
            weights,
        };
        split.normalize();
        split
    }

    pub fn axis(&self) -> Axis {
        self.axis
    }

    pub fn children(&self) -> &[Node] {
        &self.children
    }

    pub fn weights(&self) -> &[f32] {
        &self.weights
    }

    pub(crate) fn normalize(&mut self) {
        let sum: f32 = self.weights.iter().sum();
        if sum <= 0.0 {
            let w = 1.0 / self.weights.len().max(1) as f32;
            self.weights.fill(w);
        } else {
            for x in self.weights.iter_mut() {
                *x /= sum;
            }
        }
    }

    /// Cells available to children once dividers are deducted.
    pub(crate) fn usable(&self, area: Rect) -> u16 {
        let dividers = (self.children.len().saturating_sub(1)) as u16;
        let extent = match self.axis {
            Axis::Horizontal => area.w,
            Axis::Vertical => area.h,
        };
        extent.saturating_sub(dividers)
    }

    /// Rect for each child, with one cell of divider between neighbours.
    pub(crate) fn child_rects(&self, area: Rect) -> Vec<Rect> {
        let sizes = distribute(self.usable(area), &self.weights, 1);
        let mut out = Vec::with_capacity(sizes.len());
        let mut pos = match self.axis {
            Axis::Horizontal => area.x,
            Axis::Vertical => area.y,
        };
        for size in sizes {
            out.push(match self.axis {
                Axis::Horizontal => Rect::new(pos, area.y, size, area.h),
                Axis::Vertical => Rect::new(area.x, pos, area.w, size),
            });
            pos += size + 1;
        }
        out
    }
}

/// Splits `total` cells across `weights`, keeping the sum exact.
///
/// Flooring each share then handing the remainder to the largest fractional
/// parts is what stops a column of panes from losing a cell per divider and
/// drifting away from the window edge.
pub fn distribute(total: u16, weights: &[f32], min: u16) -> Vec<u16> {
    let n = weights.len();
    if n == 0 {
        return Vec::new();
    }
    // Clamping negatives away can leave nothing to divide by; fall back to
    // equal shares so the remainder pass has a full set of weights to spread
    // across rather than handing out one cell per slot.
    let mut share: Vec<f32> = weights.iter().map(|w| w.max(0.0)).collect();
    let mut sum: f32 = share.iter().sum();
    if sum <= 0.0 {
        share.fill(1.0);
        sum = n as f32;
    }

    let mut sizes = Vec::with_capacity(n);
    let mut frac = Vec::with_capacity(n);
    let mut used: u32 = 0;
    for &w in &share {
        let ideal = total as f32 * (w / sum);
        let floor = ideal.floor();
        sizes.push(floor as u16);
        frac.push(ideal - floor);
        used += floor as u32;
    }

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| {
        frac[b]
            .partial_cmp(&frac[a])
            .unwrap_or(core::cmp::Ordering::Equal)
            .then(a.cmp(&b))
    });
    let mut rem = (total as u32).saturating_sub(used);
    for &i in &order {
        if rem == 0 {
            break;
        }
        sizes[i] += 1;
        rem -= 1;
    }

    enforce_min(&mut sizes, total, min);
    sizes
}

fn enforce_min(sizes: &mut [u16], total: u16, min: u16) {
    let n = sizes.len();
    if min == 0 || n == 0 {
        return;
    }
    if (total as u32) < n as u32 * min as u32 {
        let base = total / n as u16;
        let mut extra = total % n as u16;
        for s in sizes.iter_mut() {
            *s = base
                + if extra > 0 {
                    extra -= 1;
                    1
                } else {
                    0
                };
        }
        return;
    }
    while let Some(i) = sizes.iter().position(|&s| s < min) {
        let j = sizes
            .iter()
            .enumerate()
            .max_by_key(|&(_, &s)| s)
            .map(|(j, _)| j)
            .unwrap_or(i);
        if j == i || sizes[j] <= min {
            break;
        }
        sizes[j] -= 1;
        sizes[i] += 1;
    }
}
