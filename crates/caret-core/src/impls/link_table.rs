#[allow(unused_imports)]
use crate::prelude::*;

/// How many marked links a pane remembers the targets of.
///
/// A listing that marks every filename is the demanding case, and a few
/// thousand of those is already more than a screen and its scrollback show
/// at once. Each target is the URL itself, so this is tens of kilobytes
/// rather than megabytes.
const REMEMBERED: usize = 4_096;

/// The longest target worth keeping. Past this a program is using the
/// escape as storage rather than as a link.
const MAX_TARGET: usize = 2_048;

impl LinkTable {
    pub(crate) fn new() -> LinkTable {
        LinkTable {
            targets: Vec::new(),
            generation: 0,
        }
    }

    /// Names `target`, reusing the id when it is the one just named.
    ///
    /// Returns `None` when the table is full, which asks the caller to
    /// forget what it has and start again.
    pub(crate) fn intern(&mut self, target: &str) -> Option<LinkId> {
        if target.is_empty() || target.len() > MAX_TARGET {
            return None;
        }
        // A program marks the link once and then prints the whole run, so
        // the id it wants is almost always the last one made.
        if self.targets.last().is_some_and(|last| &**last == target) {
            return Some(LinkId(self.targets.len() as u16));
        }
        if self.targets.len() >= REMEMBERED {
            return None;
        }
        self.targets.push(target.into());
        Some(LinkId(self.targets.len() as u16))
    }

    pub(crate) fn target(&self, id: LinkId) -> Option<&str> {
        if id.is_none() {
            return None;
        }
        self.targets.get(id.0 as usize - 1).map(|t| &**t)
    }

    pub(crate) fn forget_all(&mut self) {
        self.targets.clear();
        self.generation = self.generation.wrapping_add(1);
    }

    pub(crate) fn bytes(&self) -> usize {
        self.targets.capacity() * core::mem::size_of::<Box<str>>()
            + self.targets.iter().map(|t| t.len()).sum::<usize>()
    }
}
