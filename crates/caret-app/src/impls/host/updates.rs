#[allow(unused_imports)]
use crate::prelude::*;

use std::sync::mpsc;

use caret_update::{check, fetch_latest, install, CheckState, Outcome, Version, RELEASES};

/// This build's version, which is the thing a release is compared against.
pub fn current() -> Version {
    Version::parse(env!("CARGO_PKG_VERSION")).unwrap_or(Version {
        major: 0,
        minor: 0,
        patch: 0,
        pre: String::new(),
    })
}

impl Session {
    /// Starts looking for a newer release, if looking is wanted.
    ///
    /// `asked` is true when somebody chose it from the palette, which makes
    /// it worth saying "you are on the latest" as well; the daily one speaks
    /// only when there is something to say.
    ///
    /// A thread rather than the poll loop: the request takes as long as the
    /// network takes, and a terminal that waits for a web server before
    /// drawing its first prompt is a broken terminal. It is one thread, it
    /// lives for one request, and the answer comes back through a channel
    /// the loop already visits.
    pub(crate) fn begin_update_check(&mut self, asked: bool) {
        if self.update_check.is_some() {
            return;
        }
        if !asked && !self.config.check_updates() {
            return;
        }

        let dir = caret_config::paths::config_dir();
        let state = dir.as_deref().map(CheckState::read).unwrap_or_default();

        // What the last check found still counts: mentioning it again costs
        // nothing, where asking again would cost a request.
        if !asked {
            if let Some(seen) = Version::parse(&state.seen) {
                if seen.is_newer_than(&current()) {
                    self.announce_update(&seen.to_string());
                    return;
                }
            }
            if !state.due(caret_update::now(), self.config.update.interval_hours) {
                return;
            }
        }

        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(check(&current(), fetch_latest));
        });
        self.update_check = Some((rx, asked));
    }

    /// Picks up the answer, if one has arrived.
    pub(crate) fn settle_update_check(&mut self) {
        let Some((rx, asked)) = &self.update_check else {
            return;
        };
        let asked = *asked;
        let outcome = match rx.try_recv() {
            Ok(outcome) => outcome,
            Err(mpsc::TryRecvError::Empty) => return,
            Err(mpsc::TryRecvError::Disconnected) => Outcome::Unknown,
        };
        self.update_check = None;

        if let Some(dir) = caret_config::paths::config_dir() {
            let seen = match &outcome {
                Outcome::Newer(release) => release.version.to_string(),
                _ => String::new(),
            };
            CheckState {
                checked_at: caret_update::now(),
                seen,
            }
            .write(&dir);
        }

        match outcome {
            Outcome::Newer(release) => self.announce_update(&release.version.to_string()),
            Outcome::Current if asked => {
                self.message = format!(" caret {} is the latest", current());
            }
            Outcome::Unknown if asked => {
                self.message = format!(" could not reach {RELEASES}");
            }
            _ => {}
        }
        self.needs_draw = true;
    }

    fn announce_update(&mut self, version: &str) {
        self.message = format!(" caret {version} is out: run `caret update --install`");
        self.needs_draw = true;
    }
}

/// `caret update`, and `caret update --install`.
///
/// Prints rather than returns: this is the command line, and the answer is
/// the output.
pub fn command(install_it: bool) -> i32 {
    let current = current();
    println!("caret {current}");

    match check(&current, fetch_latest) {
        Outcome::Current => {
            println!("the latest release, nothing to do");
            0
        }
        Outcome::Unknown => {
            eprintln!("could not ask {RELEASES} what the latest release is");
            eprintln!("check the network, or look yourself: {RELEASES}/latest");
            1
        }
        Outcome::Newer(release) => {
            println!("caret {} is out", release.version);
            if !install_it {
                println!("run `caret update --install` to install it,");
                println!("or see {RELEASES}/tag/{}", release.tag);
                return 0;
            }
            println!("downloading and checking it...");
            match install(&release) {
                Ok(done) => {
                    println!("{done}");
                    0
                }
                Err(why) => {
                    eprintln!("{why}");
                    eprintln!("the release page is {RELEASES}/tag/{}", release.tag);
                    1
                }
            }
        }
    }
}
