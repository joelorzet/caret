#[allow(unused_imports)]
use crate::prelude::*;

/// Known tables and their keys, used to warn about typos.
///
/// A config that silently ignores a misspelled setting is worse than one that
/// complains, so unknown keys are reported even though they are not fatal.
const SCHEMA: &[(&str, &[&str])] = &[
    (
        "",
        &[
            "theme", "font", "window", "terminal", "chrome", "update", "sidebar", "keys", "layouts",
        ],
    ),
    (
        "font",
        &[
            "family",
            "fallback",
            "size",
            "line_height",
            "bold_is_bright",
            "ligatures",
            "letter_spacing",
            "italics",
            "builtin_box_drawing",
        ],
    ),
    (
        "window",
        &["opacity", "padding", "decorations", "cols", "rows"],
    ),
    (
        "terminal",
        &[
            "scrollback",
            "shell",
            "shell_args",
            "cursor",
            "cursor_blink",
            "cursor_fill",
            "cursor_color",
            "bell",
            "word_separators",
            "confirm_close",
            "highlight_input",
            "clipboard_write",
            "confirm_paste",
            "mouse",
        ],
    ),
    (
        "chrome",
        &[
            "tab_bar",
            "status",
            "hide_tab_bar_when_single",
            "show_memory",
            "padding",
        ],
    ),
    ("update", &["check", "interval_hours"]),
    ("sidebar", &["explorer", "agent"]),
    (
        "sidebar.explorer",
        &["visible", "width", "show_hidden", "follow_cwd"],
    ),
    ("sidebar.agent", &["visible", "width", "command", "args"]),
    (
        "keys",
        &["prefix", "defaults", "unbind", "direct", "prefixed"],
    ),
    // Layout names are the user's, so every `[layouts.<name>]` table is
    // checked against this one wildcard entry.
    (
        "layouts.*",
        &["description", "preset", "panes", "main", "split"],
    ),
];

/// Tables whose keys are user-chosen and so cannot be validated.
const FREEFORM: &[&str] = &["keys.direct", "keys.prefixed"];

pub(crate) fn unknown_keys(text: &str) -> Vec<String> {
    let Ok(value) = toml::from_str::<toml::Value>(text) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    walk(&value, "", "", &mut out);
    out
}

/// `schema` selects the table's rules; `label` is what the user sees, so a
/// wildcard table still reports the name they actually wrote.
fn walk(value: &toml::Value, schema: &str, label: &str, out: &mut Vec<String>) {
    if FREEFORM.contains(&schema) {
        return;
    }
    let Some(table) = value.as_table() else {
        return;
    };

    // Layout names belong to the user, so each one is checked against a
    // single wildcard entry instead of being treated as a typo itself.
    if schema == "layouts" {
        for (name, child) in table {
            walk(child, "layouts.*", &format!("layouts.{name}"), out);
        }
        return;
    }

    let Some((_, allowed)) = SCHEMA.iter().find(|(p, _)| *p == schema) else {
        return;
    };

    for (key, child) in table {
        let full = if label.is_empty() {
            key.clone()
        } else {
            format!("{label}.{key}")
        };
        if !allowed.contains(&key.as_str()) {
            out.push(full);
            continue;
        }
        let child_schema = if schema.is_empty() {
            key.clone()
        } else {
            format!("{schema}.{key}")
        };
        walk(child, &child_schema, &full, out);
    }
}
