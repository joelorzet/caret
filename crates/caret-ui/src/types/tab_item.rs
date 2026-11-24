#[allow(unused_imports)]
use crate::prelude::*;

/// What the strip needs to know about one tab.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TabItem {
    pub title: String,
    pub panes: u16,
    pub zoomed: bool,
}
