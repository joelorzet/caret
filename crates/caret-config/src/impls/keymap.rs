#[allow(unused_imports)]
use crate::prelude::*;

/// tmux-shaped prefix bindings. Pressing the prefix arms the table below.
const PREFIX: &str = "ctrl+b";

const PREFIXED: &[(&str, &str)] = &[
    ("%", "split_horizontal"),
    ("\"", "split_vertical"),
    ("x", "close_pane"),
    ("z", "toggle_zoom"),
    ("E", "equalize_panes"),
    ("t", "flip_split"),
    ("shift+left", "join_left"),
    ("shift+right", "join_right"),
    ("shift+up", "join_up"),
    ("shift+down", "join_down"),
    ("ctrl+o", "rotate_panes"),
    ("m", "rearrange"),
    ("left", "focus_left"),
    ("right", "focus_right"),
    ("up", "focus_up"),
    ("down", "focus_down"),
    ("h", "focus_left"),
    ("l", "focus_right"),
    ("k", "focus_up"),
    ("j", "focus_down"),
    ("ctrl+left", "resize_left"),
    ("ctrl+right", "resize_right"),
    ("ctrl+up", "resize_up"),
    ("ctrl+down", "resize_down"),
    ("c", "new_tab"),
    ("R", "rename_tab"),
    ("&", "close_tab"),
    ("o", "next_pane"),
    (";", "last_pane"),
    ("space", "next_layout"),
    ("alt+1", "apply_layout:even-horizontal"),
    ("alt+2", "apply_layout:even-vertical"),
    ("alt+3", "apply_layout:main-vertical"),
    ("alt+4", "apply_layout:main-horizontal"),
    ("alt+5", "apply_layout:tiled"),
    ("1", "select_tab:1"),
    ("2", "select_tab:2"),
    ("3", "select_tab:3"),
    ("4", "select_tab:4"),
    ("5", "select_tab:5"),
    ("n", "next_tab"),
    ("p", "prev_tab"),
    ("e", "toggle_explorer"),
    ("a", "toggle_agent"),
    ("<", "resize_explorer:-4"),
    (">", "resize_explorer:4"),
    (",", "resize_agent:-4"),
    (".", "resize_agent:4"),
    ("lbracket", "scroll_page:-1"),
    ("Q", "quit"),
    ("y", "copy"),
    ("/", "search"),
    ("?", "show_shortcuts"),
    ("T", "show_themes"),
    ("s", "show_settings"),
    ("S", "edit_config"),
    ("r", "reload_config"),
];

/// Direct bindings, no prefix. Command-based to match the platform.
const DIRECT: &[(&str, &str)] = &[
    ("cmd+d", "split_horizontal"),
    ("cmd+D", "split_vertical"),
    ("cmd+w", "close_pane"),
    ("cmd+enter", "toggle_zoom"),
    ("cmd+alt+m", "rearrange"),
    ("cmd+alt+shift+left", "join_left"),
    ("cmd+alt+shift+right", "join_right"),
    ("cmd+alt+shift+up", "join_up"),
    ("cmd+alt+shift+down", "join_down"),
    ("cmd+alt+left", "focus_left"),
    ("cmd+alt+right", "focus_right"),
    ("cmd+alt+up", "focus_up"),
    ("cmd+alt+down", "focus_down"),
    ("cmd+ctrl+left", "resize_left"),
    ("cmd+ctrl+right", "resize_right"),
    ("cmd+ctrl+up", "resize_up"),
    ("cmd+ctrl+down", "resize_down"),
    ("cmd+t", "new_tab"),
    ("cmd+R", "rename_tab"),
    ("cmd+1", "select_tab:1"),
    ("cmd+2", "select_tab:2"),
    ("cmd+3", "select_tab:3"),
    ("cmd+4", "select_tab:4"),
    ("cmd+5", "select_tab:5"),
    ("cmd+W", "close_tab"),
    ("ctrl+tab", "next_tab"),
    ("ctrl+shift+tab", "prev_tab"),
    ("cmd+b", "toggle_explorer"),
    ("cmd+j", "toggle_agent"),
    ("cmd+alt+b", "focus_explorer"),
    ("cmd+alt+j", "focus_agent"),
    ("shift+pageup", "scroll_page:-1"),
    ("shift+pagedown", "scroll_page:1"),
    ("shift+up", "scroll_lines:-1"),
    ("shift+down", "scroll_lines:1"),
    ("cmd+end", "scroll_to_bottom"),
    ("cmd+c", "copy"),
    ("cmd+v", "paste"),
    ("cmd+slash", "show_shortcuts"),
    ("cmd+P", "show_shortcuts"),
    ("cmd+T", "show_themes"),
    ("cmd+comma", "show_settings"),
    ("cmd+alt+comma", "edit_config"),
    ("cmd+q", "quit"),
];

impl Keymap {
    pub fn empty() -> Keymap {
        Keymap {
            prefix: None,
            direct: Vec::new(),
            prefixed: Vec::new(),
        }
    }

    pub fn prefix(&self) -> Option<KeyBinding> {
        self.prefix
    }

    pub fn set_prefix(&mut self, prefix: Option<KeyBinding>) {
        self.prefix = prefix;
    }

    /// Adds a binding, replacing any existing one for the same key.
    pub fn bind(&mut self, key: KeyBinding, action: Action, prefixed: bool) {
        let table = if prefixed {
            &mut self.prefixed
        } else {
            &mut self.direct
        };
        match table.iter_mut().find(|(k, _)| *k == key) {
            Some(entry) => entry.1 = action,
            None => table.push((key, action)),
        }
    }

    pub fn unbind(&mut self, key: KeyBinding, prefixed: bool) {
        let table = if prefixed {
            &mut self.prefixed
        } else {
            &mut self.direct
        };
        table.retain(|(k, _)| *k != key);
    }

    /// Looks up a keypress. `prefix_pending` is true when the previous
    /// keypress was the prefix; the caller clears it on any outcome.
    pub fn resolve(&self, pressed: KeyBinding, prefix_pending: bool) -> KeyResolution {
        let table = if prefix_pending {
            &self.prefixed
        } else {
            &self.direct
        };

        if !prefix_pending {
            if let Some(prefix) = self.prefix {
                if prefix.matches(&pressed) {
                    return KeyResolution::PrefixPending;
                }
            }
        }

        table
            .iter()
            .find(|(k, _)| k.matches(&pressed))
            .map(|(_, action)| KeyResolution::Run(action.clone()))
            .unwrap_or(KeyResolution::Unbound)
    }

    pub fn bindings_for(&self, action: &Action) -> Vec<String> {
        self.shortcuts()
            .into_iter()
            .flat_map(|(_, rows)| rows)
            .find(|s| &s.action == action)
            .map(|s| s.keys)
            .unwrap_or_default()
    }

    /// Every binding, grouped for display. Actions bound more than once list
    /// all their keys together.
    pub fn shortcuts(&self) -> Vec<(ActionGroup, Vec<Shortcut>)> {
        let mut rows: Vec<Shortcut> = Vec::new();

        let mut add =
            |keys: String, action: &Action| match rows.iter_mut().find(|s| &s.action == action) {
                Some(existing) => existing.keys.push(keys),
                None => rows.push(Shortcut {
                    keys: vec![keys],
                    action: action.clone(),
                    description: action.description(),
                }),
            };

        for (key, action) in &self.direct {
            add(key.to_string(), action);
        }
        for (key, action) in &self.prefixed {
            let rendered = match self.prefix {
                Some(prefix) => format!("{prefix} {key}"),
                None => key.to_string(),
            };
            add(rendered, action);
        }

        ActionGroup::ALL
            .into_iter()
            .filter_map(|group| {
                let items: Vec<Shortcut> = rows
                    .iter()
                    .filter(|s| s.action.group() == group)
                    .cloned()
                    .collect();
                (!items.is_empty()).then_some((group, items))
            })
            .collect()
    }
}

impl Default for Keymap {
    fn default() -> Keymap {
        let mut map = Keymap::empty();
        map.prefix = Some(
            KeyBinding::parse(PREFIX).unwrap_or_else(|e| panic!("default prefix {PREFIX:?}: {e}")),
        );

        for (spec, action) in DIRECT {
            map.bind(parse_default(spec), parse_action(action), false);
        }
        for (spec, action) in PREFIXED {
            map.bind(parse_default(spec), parse_action(action), true);
        }
        map
    }
}

fn parse_default(spec: &str) -> KeyBinding {
    KeyBinding::parse(spec).unwrap_or_else(|e| panic!("default binding {spec:?}: {e}"))
}

fn parse_action(name: &str) -> Action {
    Action::from_name(name).unwrap_or_else(|| panic!("default action {name:?} is not known"))
}
