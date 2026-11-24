#[allow(unused_imports)]
use crate::prelude::*;

/// What a second window takes from the first rather than reading again.
pub(crate) struct Inherited {
    pub(crate) config: Config,
    pub(crate) themes: ThemeRegistry,
    pub(crate) commands: CommandIndex,
}
