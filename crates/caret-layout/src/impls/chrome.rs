#[allow(unused_imports)]
use crate::prelude::*;

impl Chrome {
    /// How tall the status band is: a rule, then the line itself.
    ///
    /// One row put the text hard against the bottom of the panes with nothing
    /// between them, so it read as another line of terminal output.
    pub const STATUS_ROWS: u16 = 2;

    /// Carves the window into tab bar, docks, pane area and status line.
    ///
    /// Each region is only granted space when what remains still leaves a
    /// usable pane area, so a narrow window drops the docks rather than
    /// squeezing the terminal to nothing.
    pub fn layout(&self, area: Rect) -> Frame {
        let mut y = area.y;
        let mut h = area.h;

        let tab_bar = if self.tab_bar > 0 && h > self.tab_bar {
            let r = Rect::new(area.x, y, area.w, self.tab_bar);
            y += self.tab_bar;
            h -= self.tab_bar;
            Some(r)
        } else {
            None
        };

        let status = if self.status > 0 && h > self.status {
            h -= self.status;
            Some(Rect::new(area.x, y + h, area.w, self.status))
        } else {
            None
        };

        let mut x = area.x;
        let mut w = area.w;

        let left =
            if self.left.visible && self.left.cells > 0 && w > self.left.cells + Self::MIN_CENTER {
                let r = Rect::new(x, y, self.left.cells, h);
                x += self.left.cells;
                w -= self.left.cells;
                Some(r)
            } else {
                None
            };

        let right = if self.right.visible
            && self.right.cells > 0
            && w > self.right.cells + Self::MIN_CENTER
        {
            w -= self.right.cells;
            Some(Rect::new(x + w, y, self.right.cells, h))
        } else {
            None
        };

        // Padding comes out of the pane area only, and is dropped entirely
        // rather than squeezing the panes below a usable width.
        let pad = self.padding;
        let (x, w) = if w > pad * 2 + Self::MIN_CENTER {
            (x + pad, w - pad * 2)
        } else {
            (x, w)
        };

        Frame {
            tab_bar,
            left,
            center: Rect::new(x, y, w, h),
            right,
            status,
        }
    }
}

impl Chrome {
    /// Columns the pane area must keep before a dock is allowed to appear.
    pub const MIN_CENTER: u16 = 20;
}

impl Default for Chrome {
    fn default() -> Self {
        Chrome {
            tab_bar: 1,
            padding: 1,
            status: Chrome::STATUS_ROWS,
            left: Dock::new(DockKind::Explorer, 28),
            right: Dock::new(DockKind::Agent, 44),
        }
    }
}
