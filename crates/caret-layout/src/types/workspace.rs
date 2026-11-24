#[allow(unused_imports)]
use crate::prelude::*;

pub struct Workspace {
    pub(crate) tabs: Vec<Tab>,
    pub(crate) active: usize,
    pub(crate) next_pane: u32,
    pub(crate) next_tab: u32,
}
