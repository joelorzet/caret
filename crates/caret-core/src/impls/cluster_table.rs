#[allow(unused_imports)]
use crate::prelude::*;

/// Where handles start: the first plane of private use, which no font maps
/// and no program prints on purpose.
const HANDLE_BASE: u32 = 0xF_0000;

/// The character a cell carries in place of the cluster at `index`.
///
/// Anything that holds clusters beside its cells uses these two, so a
/// handle means the same thing wherever one turns up.
pub fn cluster_handle(index: usize) -> Option<char> {
    char::from_u32(HANDLE_BASE + u32::try_from(index).ok()?)
}

pub fn cluster_index(handle: char) -> Option<usize> {
    (handle as u32).checked_sub(HANDLE_BASE).map(|i| i as usize)
}

/// How many clusters a screen holds the text of at once.
///
/// Only cells that need one take an entry, and the table is compacted
/// against the grid before it is allowed to fill, so this is reached only
/// when the grid itself genuinely holds this many different clusters. Past
/// it a cell keeps its base character and loses what combined with it,
/// which is what it did before any of this existed.
const REMEMBERED: usize = 32_768;

/// The longest cluster worth keeping. A base character and a handful of
/// marks is what text is made of; past this something is being built out of
/// escapes rather than written.
const MAX_CHARS: usize = 16;

impl ClusterTable {
    pub(crate) fn new() -> ClusterTable {
        ClusterTable { text: Vec::new() }
    }

    /// Stores `text` and returns the handle a cell carries for it.
    pub(crate) fn add(&mut self, text: &str) -> Option<char> {
        if text.chars().count() > MAX_CHARS || self.text.len() >= REMEMBERED {
            return None;
        }
        self.text.push(text.into());
        cluster_handle(self.text.len() - 1)
    }

    pub(crate) fn text_of(&self, handle: char) -> Option<&str> {
        self.text.get(cluster_index(handle)?).map(|t| &**t)
    }

    pub(crate) fn is_full(&self) -> bool {
        self.text.len() >= REMEMBERED
    }

    /// Replaces the table with only the clusters `keep` names, in that
    /// order, and says what each old handle became.
    ///
    /// The caller walks the grid to collect `keep`, so what survives is
    /// what is still on screen or in scrollback; everything that scrolled
    /// away or was written over goes.
    pub(crate) fn retain(&mut self, keep: &[char]) -> Vec<(char, char)> {
        let mut kept: Vec<Box<str>> = Vec::with_capacity(keep.len());
        let mut moved = Vec::with_capacity(keep.len());
        for old in keep {
            let Some(text) = self.text_of(*old) else {
                continue;
            };
            let text = Box::from(text);
            let Some(new) = cluster_handle(kept.len()) else {
                break;
            };
            kept.push(text);
            moved.push((*old, new));
        }
        self.text = kept;
        moved
    }

    pub(crate) fn bytes(&self) -> usize {
        self.text.capacity() * core::mem::size_of::<Box<str>>()
            + self.text.iter().map(|t| t.len()).sum::<usize>()
    }
}
