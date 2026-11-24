use caret_config::Keymap;

/// Renders the shortcut list, grouped, with every key that runs each action.
pub fn render(keymap: &Keymap) -> String {
    let groups = keymap.shortcuts();
    let width = groups
        .iter()
        .flat_map(|(_, rows)| rows)
        .map(|s| s.keys_display().chars().count())
        .max()
        .unwrap_or(0);

    let mut out = String::from("Shortcuts");
    if let Some(prefix) = keymap.prefix() {
        out.push_str(&format!("   (prefix: {prefix})"));
    }
    out.push('\n');

    for (group, rows) in groups {
        out.push_str(&format!("\n{group}\n"));
        for s in rows {
            out.push_str(&format!(
                "  {:<width$}   {}\n",
                s.keys_display(),
                s.description,
                width = width
            ));
        }
    }
    out
}
