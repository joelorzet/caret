use caret_features::Explorer;
use std::fs;
use std::path::PathBuf;

/// A directory tree:
///   root/.hidden  root/apple.txt  root/Banana.txt  root/run.sh*  root/sub/  root/sub/deep/
pub fn tree(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("caret-exp-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("sub/deep")).unwrap();
    fs::write(root.join("apple.txt"), b"a").unwrap();
    fs::write(root.join("Banana.txt"), b"b").unwrap();
    fs::write(root.join(".hidden"), b"h").unwrap();
    let script = root.join("run.sh");
    fs::write(&script, b"#!/bin/sh\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    }
    root
}

pub fn names(explorer: &Explorer) -> Vec<&str> {
    explorer.entries().iter().map(|e| e.name.as_str()).collect()
}
