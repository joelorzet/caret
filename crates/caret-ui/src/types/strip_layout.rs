#[allow(unused_imports)]
use crate::prelude::*;

/// Where every piece of the strip ended up.
#[derive(Clone, Debug, Default)]
pub struct StripLayout {
    pub tabs: Vec<PxRect>,
    /// Empty when the tabs are too narrow to carry a close button.
    pub closes: Vec<PxRect>,
    pub new_tab: PxRect,
}
