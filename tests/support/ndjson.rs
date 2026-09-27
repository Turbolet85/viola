//! Reading an ndjson file another process may still be appending to: a role file,
//! `events.ndjson`, a fake-agent receipt. The writer appends one line per write, but a reader can
//! see an append part-way, so only newline-terminated lines are read.

use std::fs;
use std::path::Path;

use serde_json::Value;

/// The complete lines of `bytes`, each one JSON object. The bytes after the last `\n` are a line
/// the writer has not finished: never parsed, read on a later call once its `\n` lands. A complete
/// line that is not one JSON object still fails the test.
pub fn complete_lines(bytes: &[u8]) -> Vec<Value> {
    let Some(end) = bytes.iter().rposition(|b| *b == b'\n') else {
        return Vec::new();
    };
    bytes[..end]
        .split(|b| *b == b'\n')
        .map(|l| serde_json::from_slice(l).expect("one JSON object per complete line"))
        .collect()
}

/// `complete_lines` of the file at `path`; a file not written yet has none.
pub fn read_lines(path: &Path) -> Vec<Value> {
    complete_lines(&fs::read(path).unwrap_or_default())
}
