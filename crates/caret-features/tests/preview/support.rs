use std::fs;
use std::path::PathBuf;

pub fn write(tag: &str, name: &str, body: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("caret-prev-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    fs::write(&path, body).unwrap();
    path
}

pub const CODE: &str = "pub fn main() {\n    let x = 1;\n    // a note\n    println!(\"hi\");\n}\n";
