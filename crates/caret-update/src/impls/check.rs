#[allow(unused_imports)]
use crate::prelude::*;

use std::path::Path;
use std::process::Command;

/// Where releases live.
///
/// Taken from the manifest rather than written out here, so a fork that
/// publishes its own builds updates from its own repository by changing the
/// field Cargo already has for it. `CARET_RELEASES` at build time overrides
/// both, for anybody hosting somewhere that is not GitHub. Nothing secret:
/// it is a public address, and a secret would be baked into the binary for
/// anyone to read anyway.
pub const RELEASES: &str = match option_env!("CARET_RELEASES") {
    Some(url) => url,
    None => concat!(env!("CARGO_PKG_REPOSITORY"), "/releases"),
};

/// How long any one request may take.
const TIMEOUT: &str = "10";

/// Decides what a fetched address means.
///
/// The fetch is passed in so that the deciding can be tested without a
/// network: everything here is a string in and an answer out.
pub fn check(current: &Version, fetch: impl FnOnce() -> Option<String>) -> Outcome {
    let Some(url) = fetch() else {
        return Outcome::Unknown;
    };
    let Some(release) = Release::from_redirect(&url) else {
        return Outcome::Unknown;
    };
    if release.version.is_newer_than(current) {
        Outcome::Newer(release)
    } else {
        Outcome::Current
    }
}

/// Asks where `releases/latest` redirects to.
///
/// curl rather than an HTTP client: a terminal with no dependencies worth
/// speaking of is not going to carry a TLS stack to ask one question a day,
/// and anything that can run this has curl. The request carries the
/// program's name and version and nothing else: no token, no cookies,
/// nothing about the machine or the person at it. If curl is missing or the
/// network is down the answer is simply unknown.
pub fn fetch_latest() -> Option<String> {
    let out = Command::new("curl")
        .args([
            "--silent",
            "--show-error",
            "--location",
            "--head",
            "--output",
            "/dev/null",
            "--max-time",
            TIMEOUT,
            "--user-agent",
            &user_agent(),
            "--write-out",
            "%{url_effective}",
            &format!("{RELEASES}/latest"),
        ])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let url = String::from_utf8_lossy(&out.stdout).trim().to_string();
    (!url.is_empty()).then_some(url)
}

/// Downloads a file, returning false rather than explaining itself.
pub fn download(url: &str, to: &Path) -> bool {
    Command::new("curl")
        .args([
            "--silent",
            "--show-error",
            "--location",
            "--fail",
            "--max-time",
            "300",
            "--user-agent",
            &user_agent(),
            "--output",
        ])
        .arg(to)
        .arg(url)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn user_agent() -> String {
    format!("caret/{}", env!("CARGO_PKG_VERSION"))
}
