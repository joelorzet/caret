#[allow(unused_imports)]
use crate::prelude::*;

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::impls::check::download;

/// The checksums published beside the files.
const SUMS: &str = "SHA256SUMS";

impl Release {
    /// The file to fetch for the machine this is running on.
    pub fn asset(&self) -> Option<String> {
        asset_for(&self.version, std::env::consts::OS, std::env::consts::ARCH)
    }
}

/// What a release calls its download for one platform.
///
/// Taken apart from the machine it runs on so the naming can be tested; the
/// names are the ones the release workflow builds.
pub fn asset_for(version: &Version, os: &str, arch: &str) -> Option<String> {
    let arch = match arch {
        "aarch64" => "arm64",
        "x86_64" => "x86_64",
        _ => return None,
    };
    match os {
        "macos" => Some(format!("caret-{version}-macos-{arch}.dmg")),
        "linux" if arch == "x86_64" => Some(format!("caret-{version}-linux-x86_64.tar.gz")),
        _ => None,
    }
}

/// The hash `SHA256SUMS` records for one file.
pub fn hash_of(sums: &str, file: &str) -> Option<String> {
    sums.lines().find_map(|line| {
        let (hash, name) = line.split_once(char::is_whitespace)?;
        (name.trim().trim_start_matches('*') == file).then(|| hash.trim().to_lowercase())
    })
}

/// Downloads a release and puts it where the running copy is.
///
/// Every step can fail in a way the person needs to hear about, so this
/// reports in words rather than swallowing anything. Nothing is installed
/// that does not match the published checksum: the download comes over the
/// network, and a terminal is not a thing to take on trust.
pub fn install(release: &Release) -> Result<String, String> {
    let file = release
        .asset()
        .ok_or("there is no published build for this platform")?;

    let dir = std::env::temp_dir().join(format!("caret-update-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).map_err(|e| format!("no room to download into: {e}"))?;

    let archive = dir.join(&file);
    if !download(&release.download_url(&file), &archive) {
        return Err(format!("could not download {file}"));
    }
    let sums = dir.join(SUMS);
    if !download(&release.download_url(SUMS), &sums) {
        return Err("could not download the checksums".into());
    }

    let published = std::fs::read_to_string(&sums)
        .ok()
        .and_then(|text| hash_of(&text, &file))
        .ok_or_else(|| format!("{SUMS} does not list {file}"))?;
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
