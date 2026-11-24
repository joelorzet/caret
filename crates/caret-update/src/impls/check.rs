#[allow(unused_imports)]
use crate::prelude::*;

use std::path::Path;
use std::process::Command;

/// Taken from the manifest, so a fork updates from its own repository.
/// `CARET_RELEASES` overrides it at build time.
pub const RELEASES: &str = match option_env!("CARET_RELEASES") {
    Some(url) => url,
    None => concat!(env!("CARGO_PKG_REPOSITORY"), "/releases"),
};

const TIMEOUT: &str = "10";
const DOWNLOAD_TIMEOUT: &str = "300";

/// What a fetched address means. The fetch is an argument so the deciding
/// can be tested without a network.
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
/// curl rather than an HTTP client: this crate is not growing a TLS stack
/// to ask one question a day. The request carries the program's name and
/// version and nothing else, and anything unanswerable is unknown rather
/// than an error.
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

pub fn download(url: &str, to: &Path) -> bool {
    Command::new("curl")
        .args([
            "--silent",
            "--show-error",
            "--location",
            "--fail",
            "--max-time",
            DOWNLOAD_TIMEOUT,
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
