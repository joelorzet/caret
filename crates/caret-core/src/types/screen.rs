#[allow(unused_imports)]
use crate::prelude::*;

/// Viewport plus scrollback in one flat allocation.
///
/// `cells` is a single `Box<[Cell]>` of `cap * cols`. `slots` maps a logical
/// row to a physical slot and is itself a ring addressed through `head`, so
/// scrolling permutes 4-byte indices instead of moving cell data: full-screen
/// scroll is O(1) (bump `head`, clear the recycled row), region scroll is
/// O(region height). The allocation is sized once from the scrollback limit,
/// so steady-state output never allocates.
///
/// Three coordinate systems:
///   logical  `0..len`   oldest retained row to newest
///   viewport `0..rows`  what escape sequences address; last `rows` logical
///   display  `0..rows`  what is drawn, offset back by `scroll_offset`
pub struct Screen {
    pub(crate) cols: u16,
    pub(crate) rows: u16,

    pub(crate) cap: u32,
    pub(crate) cells: Box<[Cell]>,
    pub(crate) meta: Box<[RowMeta]>,
    pub(crate) slots: Box<[u32]>,
    pub(crate) head: u32,
    pub(crate) len: u32,

    /// Rows that have left the buffer entirely, so a row can be named by
    /// a number that does not change when older ones are dropped.
    pub(crate) dropped: u64,
    /// Where each command's prompt began, counted from the very first row
    /// the pane ever had. Oldest first.
    pub(crate) prompts: Vec<u64>,
    /// What the cells holding more than one character show. Empty for text
    /// that is one character to a cell, which is most of it.
    pub(crate) clusters: ClusterTable,
    /// Where the links a program marked actually point. Empty until one
    /// does, and never consulted when drawing: the cells carry the id.
    pub(crate) links: LinkTable,
    pub(crate) scroll_offset: u32,
    pub(crate) scrollback_limit: u32,

    pub(crate) cursor: Cursor,
    pub(crate) saved_cursor: Cursor,
    pub(crate) pen: Cell,
    pub(crate) region: ScrollRegion,

    /// Whether a resize may lay the content out again. The alternate
    /// screen belongs to a program that redraws itself, and rewrapping what
    /// it drew would only scramble it.
    pub(crate) reflow: bool,
    pub(crate) all_dirty: bool,
    pub(crate) scratch: Vec<u32>,
}
