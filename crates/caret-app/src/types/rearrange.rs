#[allow(unused_imports)]
use crate::prelude::*;

/// The arrange panel: a layout being worked on in its own window.
///
/// The draft is a copy of the tab, so every move is made to a diagram rather
/// than to the panes you are using. Nothing moves, resizes or redraws behind
/// the panel until you accept it.
pub struct Rearrange {
    pub(crate) draft: Tab,
}
