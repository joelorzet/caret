#[allow(unused_imports)]
use crate::prelude::*;

impl Session {
    /// Starts naming the current tab, with whatever it is called now as the
    /// text to edit, so a small correction does not mean retyping it.
    pub(crate) fn open_rename(&mut self) {
        let index = self.workspace.active_index();
        let current = self.workspace.active_tab().title().to_string();
        let start = if current.is_empty() {
            self.tab_label(index)
        } else {
            current
        };
        self.renaming = Some(start);
        self.needs_draw = true;
    }

    /// Keys while the name is being typed. It takes all of them, so a name
    /// can contain the characters the bindings use.
    pub(crate) fn rename_key(&mut self, press: &KeyPress) {
        let Some(name) = self.renaming.as_mut() else {
            return;
        };
        self.needs_draw = true;

        match press.binding.key {
            Key::Escape => self.renaming = None,
            Key::Enter => {
                let name = self.renaming.take().unwrap_or_default();
                // An empty name gives the tab back to whatever it would have
                // been called, rather than leaving it blank.
                self.workspace.active_tab_mut().set_title(name.trim());
            }
            Key::Backspace => {
                name.pop();
            }
            Key::Char(c) if !press.binding.mods.contains(Modifiers::CTRL) => name.push(c),
            _ => {}
        }
    }

    /// The prompt, for the status line.
    pub(crate) fn rename_prompt(&self) -> Option<String> {
        let name = self.renaming.as_ref()?;
        Some(format!(
            " name this tab: {name}_   enter to keep, esc to cancel, empty to reset"
        ))
    }

    pub(crate) fn is_renaming(&self) -> bool {
        self.renaming.is_some()
    }
}
