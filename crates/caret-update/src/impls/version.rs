#[allow(unused_imports)]
use crate::prelude::*;

use std::cmp::Ordering;

impl Version {
    /// Reads `0.1.1`, `v0.1.1` or `0.2.0-rc1`, and nothing else.
    pub fn parse(text: &str) -> Option<Version> {
        let text = text.trim().trim_start_matches('v');
        let (numbers, pre) = match text.split_once(['-', '+']) {
            Some((numbers, pre)) => (numbers, pre.to_string()),
            None => (text, String::new()),
        };

        let mut parts = numbers.split('.');
        let mut number = || parts.next()?.parse::<u32>().ok();
        let major = number()?;
        let minor = number()?;
        let patch = number()?;
        if parts.next().is_some() {
            return None;
        }

        Some(Version {
            major,
            minor,
            patch,
            pre,
        })
    }

    pub fn is_newer_than(&self, other: &Version) -> bool {
        self.cmp(other) == Ordering::Greater
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if !self.pre.is_empty() {
            write!(f, "-{}", self.pre)?;
        }
        Ok(())
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        (self.major, self.minor, self.patch)
            .cmp(&(other.major, other.minor, other.patch))
            // A candidate comes before the release it is for, so no suffix
            // wins; between two suffixes, alphabetical at least is stable.
            .then_with(|| match (self.pre.is_empty(), other.pre.is_empty()) {
                (true, true) => Ordering::Equal,
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                (false, false) => self.pre.cmp(&other.pre),
            })
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
