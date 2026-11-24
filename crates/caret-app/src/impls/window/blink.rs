#[allow(unused_imports)]
use crate::prelude::*;

use std::time::{Duration, Instant};

impl Blink {
    /// Half a cycle. xterm's default, and what every terminal since has used.
    const PHASE: Duration = Duration::from_millis(530);

    pub fn new() -> Blink {
        Blink {
            since: Instant::now(),
            last: true,
        }
    }

    pub fn visible(&self) -> bool {
        // `is_multiple_of` would read better and is newer than the minimum
        // Rust this builds on.
        (self.since.elapsed().as_millis() / Self::PHASE.as_millis()) % 2 == 0
    }

    /// Whether the phase flipped since the last call, so the window redraws
    /// on the beat rather than on every frame.
    pub fn flipped(&mut self) -> bool {
        let now = self.visible();
        let changed = now != self.last;
        self.last = now;
        changed
    }

    /// How long until the phase flips, so the loop can sleep until then
    /// rather than waking to ask whether it has.
    pub fn until_flip(&self) -> Duration {
        let phase = Self::PHASE.as_millis();
        let into = self.since.elapsed().as_millis() % phase;
        Duration::from_millis((phase - into) as u64)
    }

    pub fn restart(&mut self) {
        self.since = Instant::now();
        self.last = true;
    }
}

impl Default for Blink {
    fn default() -> Blink {
        Blink::new()
    }
}
