#[allow(unused_imports)]
use crate::prelude::*;

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::impls::check::download;

const LISTING: &str = "SHA256SUMS";

struct Flavour {
    rust_os: &'static str,
    word_in_name: &'static str,
    endings_best_first: &'static [&'static str],
}

const FLAVOURS: &[Flavour] = &[
    Flavour {
        rust_os: "macos",
        word_in_name: "macos",
        endings_best_first: &[".dmg", ".app.zip"],
    },
    Flavour {
        rust_os: "linux",
        word_in_name: "linux",
        endings_best_first: &[".tar.gz"],
    },
];

const ARCHITECTURE_WORDS: &[(&str, &str)] = &[("aarch64", "arm64"), ("x86_64", "x86_64")];

impl Release {
    pub fn asset(&self, listing: &str) -> Option<String> {
        pick_asset(listing, std::env::consts::OS, std::env::consts::ARCH)
    }
}

/// Picks a published file out of the names the release lists, rather than
/// building a name the release workflow would also have to agree with.
///
/// A name that says the architecture wins. One that says nothing about any
/// architecture is taken as covering every architecture, which is what a
/// universal build is.
pub fn pick_asset(listing: &str, os: &str, arch: &str) -> Option<String> {
    let flavour = FLAVOURS.iter().find(|f| f.rust_os == os)?;
    let (_, wanted) = ARCHITECTURE_WORDS.iter().find(|(rust, _)| *rust == arch)?;
    let names: Vec<&str> = listing.lines().filter_map(file_named).collect();

    for ending in flavour.endings_best_first {
        let fitting = |exact: bool| {
            names
                .iter()
                .find(|name| {
                    name.ends_with(ending)
                        && name.contains(flavour.word_in_name)
                        && if exact {
                            name.contains(wanted)
                        } else {
                            !mentions_an_architecture(name)
                        }
                })
                .map(|name| name.to_string())
        };
        if let Some(name) = fitting(true).or_else(|| fitting(false)) {
            return Some(name);
        }
    }
    None
}

fn mentions_an_architecture(name: &str) -> bool {
    ARCHITECTURE_WORDS
        .iter()
        .any(|(rust, word)| name.contains(rust) || name.contains(word))
}

pub fn hash_of(listing: &str, file: &str) -> Option<String> {
    listing.lines().find_map(|line| {
        let (hash, _) = line.split_once(char::is_whitespace)?;
        (file_named(line)? == file).then(|| hash.trim().to_lowercase())
    })
}

fn file_named(line: &str) -> Option<&str> {
    let (_, name) = line.split_once(char::is_whitespace)?;
    let name = name.trim().trim_start_matches('*');
    (!name.is_empty()).then_some(name)
}

/// Downloads a release, refuses it unless it matches its published
/// checksum, and puts it where the running copy is.
pub fn install(release: &Release) -> Result<String, String> {
    let work = work_dir()?;
    let listing = fetch_listing(release, &work)?;

    let file = release
        .asset(&listing)
        .ok_or("this release has no build for this platform")?;
    let published = hash_of(&listing, &file).ok_or(format!("{LISTING} does not list {file}"))?;

    let archive = work.join(&file);
    if !download(&release.download_url(&file), &archive) {
        return Err(format!("could not download {file}"));
    }
    let downloaded = sha256(&archive).ok_or("no sha256 command to check the download with")?;
    if downloaded != published {
        return Err(format!("{file} does not match its published checksum"));
    }

    let placed = if cfg!(target_os = "macos") {
        place_bundle(&archive)
    } else {
        place_binary(&archive, &work)
    };
    let _ = std::fs::remove_dir_all(&work);
    placed
}

fn work_dir() -> Result<PathBuf, String> {
    let dir = std::env::temp_dir().join(format!("caret-update-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).map_err(|e| format!("no room to download into: {e}"))?;
    Ok(dir)
}

fn fetch_listing(release: &Release, work: &Path) -> Result<String, String> {
    let path = work.join(LISTING);
    if !download(&release.download_url(LISTING), &path) {
        return Err(format!("could not reach the release to read its {LISTING}"));
    }
    std::fs::read_to_string(&path).map_err(|e| format!("unreadable {LISTING}: {e}"))
}

fn sha256(file: &Path) -> Option<String> {
    for (program, args) in [("shasum", ["-a", "256"].as_slice()), ("sha256sum", &[])] {
        let Ok(out) = Command::new(program).args(args).arg(file).output() else {
            continue;
        };
        if !out.status.success() {
            continue;
        }
        let text = String::from_utf8_lossy(&out.stdout);
        if let Some(hash) = text.split_whitespace().next() {
            return Some(hash.to_lowercase());
        }
    }
    None
}

/// Replacing a bundle under a running copy is allowed: the copy that is
/// running stays as it was until it is quit.
fn place_bundle(dmg: &Path) -> Result<String, String> {
    let mount = std::env::temp_dir().join(format!("caret-mount-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&mount);
    if !attach(dmg, &mount) {
        return Err("could not open the disk image".into());
    }

    let placed = copy_bundle_from(&mount);
    let _ = Command::new("hdiutil")
        .args(["detach", "-quiet"])
        .arg(&mount)
        .status();
    let _ = std::fs::remove_dir_all(&mount);
    placed
}

fn attach(dmg: &Path, mount: &Path) -> bool {
    Command::new("hdiutil")
        .args(["attach", "-nobrowse", "-quiet", "-mountpoint"])
        .arg(mount)
        .arg(dmg)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn copy_bundle_from(mount: &Path) -> Result<String, String> {
    let app = mount.join("caret.app");
    if !app.is_dir() {
        return Err("the disk image holds no caret.app".into());
    }

    let target = bundle_dir();
    std::fs::create_dir_all(&target).map_err(|e| format!("cannot write to {target:?}: {e}"))?;
    let installed = target.join("caret.app");
    let _ = std::fs::remove_dir_all(&installed);

    let copied = Command::new("cp")
        .arg("-R")
        .arg(&app)
        .arg(&target)
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    if !copied {
        return Err(format!("could not copy caret.app into {target:?}"));
    }

    let _ = Command::new("mdimport").arg(&installed).status();
    Ok(format!(
        "installed to {}. Quit caret and open it again to run it.",
        installed.display()
    ))
}

fn bundle_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    let usual = PathBuf::from(home).join("Applications");
    let Ok(exe) = std::env::current_exe() else {
        return usual;
    };
    let app = exe
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .filter(|dir| dir.extension().is_some_and(|kind| kind == "app"));
    match app {
        Some(app) => app.parent().map(Path::to_path_buf).unwrap_or(usual),
        None => usual,
    }
}

fn place_binary(archive: &Path, work: &Path) -> Result<String, String> {
    if !unpack(archive, work) {
        return Err("could not unpack the download".into());
    }
    let new_binary = std::fs::read_dir(work)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .map(|entry| entry.path().join("caret"))
        .find(|path| path.is_file())
        .ok_or("the download holds no caret binary")?;

    let Some((exe, beside)) = current_exe_and_dir() else {
        return Ok(format!("the new caret is at {}", new_binary.display()));
    };

    // Copied beside the old one and moved over it, so a half-written
    // terminal is never left behind.
    let staged = beside.join("caret.new");
    if std::fs::copy(&new_binary, &staged).is_err() {
        return Ok(format!(
            "{} is not writable, so the new caret is at {}",
            beside.display(),
            new_binary.display()
        ));
    }
    std::fs::rename(&staged, &exe).map_err(|e| format!("could not replace {exe:?}: {e}"))?;
    Ok(format!(
        "installed to {}. Restart caret to run it.",
        exe.display()
    ))
}

fn unpack(archive: &Path, into: &Path) -> bool {
    Command::new("tar")
        .arg("-xzf")
        .arg(archive)
        .arg("-C")
        .arg(into)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn current_exe_and_dir() -> Option<(PathBuf, PathBuf)> {
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?.to_path_buf();
    Some((exe, dir))
}
