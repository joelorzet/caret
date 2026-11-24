#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct SidebarsConfig {
    pub explorer: ExplorerConfig,
    pub agent: AgentConfig,
}
