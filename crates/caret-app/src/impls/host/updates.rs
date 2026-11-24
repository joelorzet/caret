#[allow(unused_imports)]
use crate::prelude::*;

use std::sync::mpsc;

use caret_update::{check, fetch_latest, install, CheckState, Outcome, Version, RELEASES};

pub fn current() -> Version {
    Version::parse(env!("CARGO_PKG_VERSION")).unwrap_or(Version {
        major: 0,
        minor: 0,
        patch: 0,
        pre: String::new(),
    })
}

impl Session {
    /// `asked` separates the palette command, which reports either way, from
    /// the daily check, which speaks only when there is something newer.
    pub(crate) fn begin_update_check(&mut self, asked: bool) {
        if self.update_check.is_some() {
            return;
        }
        if !asked && !self.config.check_updates() {
            return;
        }
        if !asked && self.report_what_the_last_check_found() {
            return;
        }

        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(check(&current(), fetch_latest));
        });
        self.update_check = Some((rx, asked));
    }

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
        remember(&outcome);

        match outcome {
            Outcome::Newer(release) => self.announce_update(&release.version.to_string()),
            Outcome::Current if asked => {
                self.message = format!(" caret {} is the latest", current())
            }
            Outcome::Unknown if asked => self.message = format!(" could not reach {RELEASES}"),
            _ => {}
        }
        self.needs_draw = true;
    }

    pub(crate) fn begin_update_install(&mut self) {
        if self.update_install.is_some() {
            return;
        }
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(match check(&current(), fetch_latest) {
                Outcome::Newer(release) => install(&release),
                Outcome::Current => Err(format!("caret {} is already the latest", current())),
                Outcome::Unknown => Err(format!("could not reach {RELEASES}")),
            });
        });
        self.update_install = Some(rx);
    }

    pub(crate) fn settle_update_install(&mut self) {
        let Some(rx) = &self.update_install else {
            return;
        };
        let done = match rx.try_recv() {
            Ok(done) => done,
            Err(mpsc::TryRecvError::Empty) => return,
            Err(mpsc::TryRecvError::Disconnected) => Err("the install stopped early".into()),
        };
        self.update_install = None;
        self.message = match done {
            Ok(said) => format!(" {said}"),
            Err(why) => format!(" {why}"),
        };
        self.needs_draw = true;
    }

    /// True when the answer is already known and does not need asking for.
    fn report_what_the_last_check_found(&mut self) -> bool {
        let state = last_check();
        if let Some(seen) = Version::parse(&state.seen) {
            if seen.is_newer_than(&current()) {
                self.announce_update(&seen.to_string());
                return true;
            }
        }
        !state.due(caret_update::now(), self.config.update.interval_hours)
    }

    fn announce_update(&mut self, version: &str) {
        self.message = format!(" caret {version} is out: `caret update --install`, or the palette");
        self.needs_draw = true;
    }
}

fn last_check() -> CheckState {
    caret_config::paths::config_dir()
        .as_deref()
        .map(CheckState::read)
        .unwrap_or_default()
}

fn remember(outcome: &Outcome) {
    let Some(dir) = caret_config::paths::config_dir() else {
        return;
    };
    let seen = match outcome {
        Outcome::Newer(release) => release.version.to_string(),
        _ => String::new(),
    };
    CheckState {
        checked_at: caret_update::now(),
        seen,
    }
    .write(&dir);
}

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
        Outcome::Newer(release) if !install_it => {
            println!("caret {} is out", release.version);
            println!("run `caret update --install` to install it,");
            println!("or see {RELEASES}/tag/{}", release.tag);
            0
        }
        Outcome::Newer(release) => {
            println!("caret {} is out", release.version);
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
