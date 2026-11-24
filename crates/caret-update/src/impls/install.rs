#[allow(unused_imports)]
use crate::prelude::*;

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::impls::check::download;

/// The checksums published beside the files.
const SUMS: &str = "SHA256SUMS";

/// How a published file says which machine it is for.
///
/// Selection criteria, not a name to build: the release workflow decides
/// what its files are called, and the only thing agreed between the two is
/// that the name carries these words. Rebuilding the name here would be a
/// second copy of a rule nothing checks.
struct Flavour {
    /// What Rust calls the platform.
    os: &'static str,
    /// The word a file name carries for it.
    word: &'static str,
    /// Endings worth taking, the best first. macOS publishes an installable
    /// image and a zipped bundle; the image is the one to hand somebody.
    endings: &'static [&'static str],
}

const FLAVOURS: &[Flavour] = &[
    Flavour {
        os: "macos",
        word: "macos",
        endings: &[".dmg", ".app.zip"],
    },
    Flavour {
        os: "linux",
        word: "linux",
        endings: &[".tar.gz"],
    },
];

/// What published names call each architecture Rust has its own word for.
const ARCHITECTURES: &[(&str, &str)] = &[("aarch64", "arm64"), ("x86_64", "x86_64")];

impl Release {
    /// The file to fetch for the machine this is running on.
    pub fn asset(&self, listing: &str) -> Option<String> {
        pick_asset(listing, std::env::consts::OS, std::env::consts::ARCH)
    }
}

/// Chooses a published file for one platform, out of the names the release
/// itself lists.
///
/// `SHA256SUMS` names every file in the release and is downloaded anyway to
/// check what arrives, so it serves as the release's own list of what it
/// published. Nothing here invents a name: it recognises one.
pub fn pick_asset(listing: &str, os: &str, arch: &str) -> Option<String> {
    let flavour = FLAVOURS.iter().find(|f| f.os == os)?;
    let (_, word) = ARCHITECTURES.iter().find(|(rust, _)| *rust == arch)?;

    let names: Vec<&str> = listing.lines().filter_map(file_named).collect();
    flavour.endings.iter().find_map(|ending| {
        names
            .iter()
            .find(|name| {
                name.ends_with(ending) && name.contains(flavour.word) && name.contains(word)
            })
            .map(|name| name.to_string())
    })
}

/// The hash the listing records for one file.
pub fn hash_of(listing: &str, file: &str) -> Option<String> {
    listing.lines().find_map(|line| {
        let (hash, _) = line.split_once(char::is_whitespace)?;
        (file_named(line)? == file).then(|| hash.trim().to_lowercase())
    })
}

/// The file a checksum line is about.
fn file_named(line: &str) -> Option<&str> {
    let (_, name) = line.split_once(char::is_whitespace)?;
    let name = name.trim().trim_start_matches('*');
    (!name.is_empty()).then_some(name)
}

/// Downloads a release and puts it where the running copy is.
///
/// Every step can fail in a way the person needs to hear about, so this
/// reports in words rather than swallowing anything. Nothing is installed
/// that does not match the published checksum: the download comes over the
/// network, and a terminal is not a thing to take on trust.
pub fn install(release: &Release) -> Result<String, String> {
    let dir = std::env::temp_dir().join(format!("caret-update-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).map_err(|e| format!("no room to download into: {e}"))?;

    // The checksums first: they are the release's own list of what it
    // published, so they say both what to fetch and what it should be.
    let sums = dir.join(SUMS);
    if !download(&release.download_url(SUMS), &sums) {
        return Err(format!("could not reach the release to read its {SUMS}"));
    }
    let listing = std::fs::read_to_string(&sums).map_err(|e| format!("unreadable {SUMS}: {e}"))?;

    let file = release
        .asset(&listing)
        .ok_or("this release has no build for this platform")?;
    let published =
        hash_of(&listing, &file).ok_or_else(|| format!("{SUMS} does not list {file}"))?;

    let archive = dir.join(&file);
    if !download(&release.download_url(&file), &archive) {
        return Err(format!("could not download {file}"));
    }
    let got = sha256(&archive).ok_or("no sha256 command to check the download with")?;
    if got != published {
        return Err(format!("{file} does not match its published checksum"));
    }

    let done = if cfg!(target_os = "macos") {
        install_bundle(&archive)
    } else {
        install_binary(&archive, &dir)
    };
    let _ = std::fs::remove_dir_all(&dir);
    done
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

/// macOS: mount the disk image and replace the app bundle in place.
///
/// Replacing a bundle underneath a running copy is allowed, and the copy
/// that is running carries on as it was until it is quit, so the only thing
/// left to do is say so.
fn install_bundle(dmg: &Path) -> Result<String, String> {
    let mount = std::env::temp_dir().join(format!("caret-mount-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&mount);
    let attached = Command::new("hdiutil")
        .args(["attach", "-nobrowse", "-quiet", "-mountpoint"])
        .arg(&mount)
        .arg(dmg)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !attached {
        return Err("could not open the disk image".into());
    }

    let target = bundle_dir();
    let result = (|| {
        let app = mount.join("caret.app");
        if !app.is_dir() {
            return Err("the disk image holds no caret.app".to_string());
        }
        std::fs::create_dir_all(&target).map_err(|e| format!("cannot write to {target:?}: {e}"))?;
        let installed = target.join("caret.app");
        let _ = std::fs::remove_dir_all(&installed);
        let copied = Command::new("cp")
            .arg("-R")
            .arg(&app)
            .arg(&target)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !copied {
            return Err(format!("could not copy caret.app into {target:?}"));
        }
        // Tell Spotlight now rather than whenever it next looks.
        let _ = Command::new("mdimport").arg(&installed).status();
        Ok(format!(
            "installed to {}. Quit caret and open it again to run it.",
            installed.display()
        ))
    })();

    let _ = Command::new("hdiutil")
        .args(["detach", "-quiet"])
        .arg(&mount)
        .status();
    let _ = std::fs::remove_dir_all(&mount);
    result
}

/// Where the app bundle lives: the one this is running from, or the usual
/// place if it is not running from a bundle at all.
fn bundle_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    let usual = PathBuf::from(home).join("Applications");
    let Ok(exe) = std::env::current_exe() else {
        return usual;
    };
    // .../caret.app/Contents/MacOS/caret
    let inside = exe
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent());
    match inside {
        Some(app) if app.extension().is_some_and(|e| e == "app") => {
            app.parent().map(Path::to_path_buf).unwrap_or(usual)
        }
        _ => usual,
    }
}

/// Elsewhere: unpack the tarball and replace the running binary, if its
/// directory is ours to write to. If it is not, say where the new one is
/// rather than guessing at a destination.
fn install_binary(archive: &Path, dir: &Path) -> Result<String, String> {
    let unpacked = Command::new("tar")
        .arg("-xzf")
        .arg(archive)
        .arg("-C")
        .arg(dir)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !unpacked {
        return Err("could not unpack the download".into());
    }

    let binary = std::fs::read_dir(dir)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .map(|entry| entry.path().join("caret"))
        .find(|path| path.is_file())
        .ok_or("the download holds no caret binary")?;

    let Ok(exe) = std::env::current_exe() else {
        return Ok(format!("the new caret is at {}", binary.display()));
    };
    let Some(parent) = exe.parent() else {
        return Ok(format!("the new caret is at {}", binary.display()));
    };

    // Written beside the old one and moved over it: a rename is atomic, and
    // a half-copied terminal is a terminal nobody can start.
    let staged = parent.join("caret.new");
    if std::fs::copy(&binary, &staged).is_err() {
        return Ok(format!(
            "{} is not writable, so the new caret is at {}",
            parent.display(),
            binary.display()
        ));
    }
    std::fs::rename(&staged, &exe).map_err(|e| format!("could not replace {exe:?}: {e}"))?;
    Ok(format!(
        "installed to {}. Restart caret to run it.",
        exe.display()
    ))
}
