use std::io::Write;

/// Appends a line to the file named by `CARET_TRACE`, if it is set.
///
/// A terminal cannot log to its own stdout, so diagnostics go to a file and
/// only when asked for.
pub fn trace(line: &str) {
    let Ok(path) = std::env::var("CARET_TRACE") else {
        return;
    };
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = writeln!(file, "{line}");
    }
}
