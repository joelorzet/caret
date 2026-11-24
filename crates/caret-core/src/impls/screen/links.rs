#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    /// Starts marking printed cells as part of a link to `target`.
    ///
    /// Everything printed until the link is closed belongs to it, however
    /// the colours change in between, and whatever the text says: a program
    /// may show a test name and mean a URL.
    pub fn open_link(&mut self, target: &str) {
        match self.links.intern(target) {
            Some(id) => self.pen.link = id,
            None => {
                // Full, so the oldest targets go and with them the ids in
                // the cells that named them. Those runs read as plain text
                // from here on, which is what they would have done had the
                // program never marked them.
                self.forget_links();
                self.pen.link = self.links.intern(target).unwrap_or(LinkId::NONE);
            }
        }
    }

    pub fn close_link(&mut self) {
        self.pen.link = LinkId::NONE;
    }

    /// The marked link at a logical row and column: where its run begins and
    /// ends, and where it points.
    ///
    /// The run is the neighbouring cells carrying the same id, so a link
    /// underlines and opens as one thing however it was printed.
    pub fn marked_link(&self, l: u32, col: u16) -> Option<(usize, usize, &str)> {
        if l >= self.len {
            return None;
        }
        let row = self.logical_line(l);
        let id = row.get(col as usize).map(|c| c.link)?;
        let target = self.links.target(id)?;

        let start = row[..col as usize]
            .iter()
            .rposition(|c| c.link != id)
            .map_or(0, |i| i + 1);
        let end = row[col as usize..]
            .iter()
            .position(|c| c.link != id)
            .map_or(row.len(), |i| col as usize + i);
        Some((start, end, target))
    }

    fn forget_links(&mut self) {
        self.links.forget_all();
        for cell in self.cells.iter_mut() {
            cell.link = LinkId::NONE;
        }
        self.pen.link = LinkId::NONE;
        self.all_dirty = true;
    }
}
