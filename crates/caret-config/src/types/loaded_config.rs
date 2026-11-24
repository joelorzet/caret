#[allow(unused_imports)]
use crate::prelude::*;

use std::path::PathBuf;

/// A configuration plus everything that went wrong loading it.
///
/// A bad config must never stop the terminal from starting, so problems are
/// collected as warnings and the affected settings keep their defaults.
#[derive(Clone, PartialEq, Debug)]
pub struct LoadedConfig {
    pub config: Config,
    pub path: Option<PathBuf>,
    pub warnings: Vec<String>,
}
