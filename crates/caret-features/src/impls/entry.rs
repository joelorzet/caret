#[allow(unused_imports)]
use crate::prelude::*;

use std::path::Path;

impl Entry {
    pub fn is_directory(&self) -> bool {
        self.kind == EntryKind::Directory
    }

    /// Reads one directory entry, following symlinks only to decide whether
    /// the target is a directory.
    pub(crate) fn read(path: &Path) -> Option<Entry> {
        let name = path.file_name()?.to_string_lossy().into_owned();
        let link = path.symlink_metadata().ok()?;
        let kind = if link.file_type().is_symlink() {
            match path.metadata() {
                Ok(target) if target.is_dir() => EntryKind::Directory,
                _ => EntryKind::Symlink,
            }
        } else if link.is_dir() {
            EntryKind::Directory
        } else if is_executable(&link) {
            EntryKind::Executable
        } else {
            EntryKind::File
        };

        Some(Entry {
            name,
            path: path.to_path_buf(),
            kind,
        })
    }

    /// Directories first, then by name ignoring case, so the listing is
    /// stable and reads the way a file manager does.
    pub(crate) fn order(&self) -> (u8, String) {
        let group = if self.is_directory() { 0 } else { 1 };
        (group, self.name.to_lowercase())
    }
}

#[cfg(unix)]
fn is_executable(meta: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    meta.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn is_executable(_meta: &std::fs::Metadata) -> bool {
    false
}
