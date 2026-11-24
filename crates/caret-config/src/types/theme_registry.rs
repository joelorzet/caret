#[allow(unused_imports)]
use crate::prelude::*;

/// Built-in themes plus any found in the config directory, which override a
/// built-in of the same name.
#[derive(Clone, Debug)]
pub struct ThemeRegistry {
    pub(crate) themes: Vec<Theme>,
}
