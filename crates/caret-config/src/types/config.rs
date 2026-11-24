#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Config {
    pub theme: String,
    pub font: FontConfig,
    pub window: WindowConfig,
    pub terminal: TerminalConfig,
    pub chrome: ChromeConfig,
    pub sidebars: SidebarsConfig,
    pub keymap: Keymap,
    /// Presets first, then anything defined in the config file.
    pub layouts: Vec<LayoutTemplate>,
}
