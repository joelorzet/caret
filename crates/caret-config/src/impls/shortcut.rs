#[allow(unused_imports)]
use crate::prelude::*;

impl Shortcut {
    /// `cmd+d` or `ctrl+b % / cmd+d` when an action has several bindings.
    pub fn keys_display(&self) -> String {
        self.keys.join(" / ")
    }
}
