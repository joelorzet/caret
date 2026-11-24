#[allow(unused_imports)]
use crate::prelude::*;

use serde::Deserialize;
use std::collections::BTreeMap;

/// Serde mirror of the config file. Every field is optional so a partial file
/// only overrides what it mentions.
///
/// Unknown keys are deliberately tolerated here and reported by
/// `schema::unknown_keys` instead: rejecting the document would throw away
/// every valid setting alongside the typo.
#[derive(Deserialize, Default)]
pub(crate) struct RawConfig {
    pub theme: Option<String>,
    pub font: Option<RawFont>,
    pub window: Option<RawWindow>,
    pub terminal: Option<RawTerminal>,
    pub chrome: Option<RawChrome>,
    pub update: Option<RawUpdate>,
    pub sidebar: Option<RawSidebars>,
    pub keys: Option<RawKeys>,
    pub layouts: Option<BTreeMap<String, RawLayout>>,
}

/// A saved layout: either a preset with a pane count, or an explicit shape in
/// the layout spec language.
#[derive(Deserialize, Default)]
pub(crate) struct RawLayout {
    pub description: Option<String>,
    pub preset: Option<String>,
    pub panes: Option<usize>,
    pub main: Option<f32>,
    pub split: Option<String>,
}

#[derive(Deserialize, Default)]
pub(crate) struct RawFont {
    pub family: Option<String>,
    pub fallback: Option<Vec<String>>,
    pub size: Option<f32>,
    pub line_height: Option<f32>,
    pub bold_is_bright: Option<bool>,
    pub ligatures: Option<bool>,
    pub letter_spacing: Option<f32>,
    pub italics: Option<bool>,
    pub builtin_box_drawing: Option<bool>,
}

#[derive(Deserialize, Default)]
pub(crate) struct RawWindow {
    pub opacity: Option<f32>,
    pub padding: Option<u16>,
    pub decorations: Option<bool>,
    pub cols: Option<u16>,
    pub rows: Option<u16>,
}

#[derive(Deserialize, Default)]
pub(crate) struct RawTerminal {
    pub scrollback: Option<u32>,
    pub shell: Option<String>,
    pub shell_args: Option<Vec<String>>,
    pub cursor: Option<String>,
    pub cursor_blink: Option<bool>,
    pub cursor_fill: Option<String>,
    pub cursor_color: Option<String>,
    pub bell: Option<String>,
    pub word_separators: Option<String>,
    pub confirm_close: Option<bool>,
    pub highlight_input: Option<bool>,
    pub clipboard_write: Option<bool>,
    pub confirm_paste: Option<String>,
    pub notify_finish: Option<String>,
    pub notify_after: Option<u32>,
    pub mouse: Option<bool>,
}

#[derive(Deserialize, Default)]
pub(crate) struct RawUpdate {
    pub check: Option<bool>,
    pub interval_hours: Option<u32>,
}

#[derive(Deserialize, Default)]
pub(crate) struct RawChrome {
    pub tab_bar: Option<bool>,
    pub status: Option<bool>,
    pub hide_tab_bar_when_single: Option<bool>,
    pub show_memory: Option<bool>,
    pub padding: Option<u16>,
}

#[derive(Deserialize, Default)]
pub(crate) struct RawSidebars {
    pub explorer: Option<RawExplorer>,
    pub agent: Option<RawAgent>,
}

#[derive(Deserialize, Default)]
pub(crate) struct RawExplorer {
    pub visible: Option<bool>,
    pub width: Option<u16>,
    pub show_hidden: Option<bool>,
    pub follow_cwd: Option<bool>,
}

#[derive(Deserialize, Default)]
pub(crate) struct RawAgent {
    pub visible: Option<bool>,
    pub width: Option<u16>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
}

#[derive(Deserialize, Default)]
pub(crate) struct RawKeys {
    pub prefix: Option<String>,
    /// `false` starts from an empty keymap instead of the built-in one.
    pub defaults: Option<bool>,
    pub unbind: Option<Vec<String>>,
    pub direct: Option<BTreeMap<String, String>>,
    pub prefixed: Option<BTreeMap<String, String>>,
}

/// Serde mirror of a theme file in `themes/`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawTheme {
    pub name: Option<String>,
    pub description: Option<String>,
    pub appearance: Option<String>,
    pub background: String,
    pub foreground: String,
    pub cursor: Option<String>,
    pub selection: Option<String>,
    pub accent: Option<String>,
    pub normal: Vec<String>,
    pub bright: Vec<String>,
}
