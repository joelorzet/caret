#[allow(unused_imports)]
use crate::prelude::*;

impl Screen {
    /// What a cell shows, which is its character unless something combined
    /// with it.
    pub fn grapheme(&self, cell: &Cell) -> Grapheme<'_> {
        if cell.flags.contains(Flags::CLUSTER) {
            if let Some(text) = self.clusters.text_of(cell.ch) {
                return Grapheme::Many(text);
            }
        }
        Grapheme::One(cell.ch)
    }

    /// The grapheme at a place in a row, for reading text back out.
    pub fn grapheme_at(&self, l: u32, col: u16) -> Option<Grapheme<'_>> {
        let row = self.logical_line(l);
        row.get(col as usize).map(|cell| self.grapheme(cell))
    }

    /// Adds `ch` to the cell the cursor last wrote, so an accent lands on
    /// the letter it belongs to instead of a cell of its own.
    ///
    /// Nothing moves: a mark has no width, and the cursor stays where it
    /// was. A mark arriving with nothing before it has nothing to attach to
    /// and is dropped, which is what the standard asks for.
    pub fn attach(&mut self, ch: char) -> bool {
        let Some((y, x)) = self.attach_point() else {
            return false;
        };

        let existing = {
            let cell = &self.line(y)[x as usize];
            match self.grapheme(cell) {
                Grapheme::One(base) => {
                    let mut text = String::with_capacity(8);
                    text.push(base);
                    text
                }
                Grapheme::Many(text) => text.to_owned(),
            }
        };

        let mut text = existing;
        text.push(ch);
        self.hold(y, x, &text)
    }

    /// Replaces what the cell the cursor last wrote shows, which is how a
    /// base and its mark become the single character that means both.
    pub fn recompose(&mut self, ch: char) -> bool {
        let Some((y, x)) = self.attach_point() else {
            return false;
        };
        let row = self.line_mut(y);
        row[x as usize].ch = ch;
        row[x as usize].flags.remove(Flags::CLUSTER);
        true
    }

    /// The character the cell the cursor last wrote is built on, which is
    /// what a mark would combine with.
    pub fn last_base(&self) -> Option<char> {
        let (y, x) = self.attach_point()?;
        Some(self.grapheme(&self.line(y)[x as usize]).base())
    }

    /// Everything the cell the cursor last wrote shows.
    pub fn last_grapheme(&self) -> Option<Grapheme<'_>> {
        let (y, x) = self.attach_point()?;
        Some(self.grapheme(&self.line(y)[x as usize]))
    }

    /// Where the character a mark would join sits.
    ///
    /// The cursor has already moved past what was printed, and it stops on
    /// the last column rather than past it when a wrap is pending. The
    /// right half of a wide character is not a character of its own, so the
    /// left half is the one that owns the mark.
    fn attach_point(&self) -> Option<(u16, u16)> {
        let y = self.cursor.y;
        let x = match self.cursor.pending_wrap {
            true => self.cursor.x,
            false => self.cursor.x.checked_sub(1)?,
        };
        let row = self.line(y);
        let cell = row.get(x as usize)?;
        let x = match cell.flags.contains(Flags::WIDE_TAIL) {
            true => x.checked_sub(1)?,
            false => x,
        };
        // A blank cell has had nothing printed on it, and a mark on nothing
        // belongs nowhere.
        let written = row.get(x as usize)?;
        (written.ch != '\0').then_some((y, x))
    }

    /// Points a cell at `text`, compacting the table first if it has to.
    fn hold(&mut self, y: u16, x: u16, text: &str) -> bool {
        if self.clusters.is_full() {
            self.compact_clusters();
        }
        let Some(handle) = self.clusters.add(text) else {
            return false;
        };
        let row = self.line_mut(y);
        row[x as usize].ch = handle;
        row[x as usize].flags.insert(Flags::CLUSTER);
        true
    }

    /// Throws away the text of clusters no cell points at any more.
    ///
    /// Rows scroll away and are written over, and their clusters go with
    /// them; this is where that is noticed. Only what the grid still shows
    /// is kept, and the handles in the cells are moved to match.
    fn compact_clusters(&mut self) {
        let mut keep: Vec<char> = self
            .cells
            .iter()
            .filter(|cell| cell.flags.contains(Flags::CLUSTER))
            .map(|cell| cell.ch)
            .collect();
        keep.sort_unstable();
        keep.dedup();

        let moved = self.clusters.retain(&keep);
        for cell in self.cells.iter_mut() {
            if !cell.flags.contains(Flags::CLUSTER) {
                continue;
            }
            match moved.binary_search_by_key(&cell.ch, |(old, _)| *old) {
                Ok(at) => cell.ch = moved[at].1,
                // Its text was already gone, so the cell keeps nothing but
                // a handle nobody can resolve. Blank it rather than draw it.
                Err(_) => {
                    cell.ch = ' ';
                    cell.flags.remove(Flags::CLUSTER);
                }
            }
        }
        self.all_dirty = true;
    }
}
