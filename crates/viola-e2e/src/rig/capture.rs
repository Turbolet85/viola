//! The rig's own prompt-submit capture, the hidden `capture --out <file>` verb a wake start's
//! private config registers as its UserPromptSubmit hook: one payload read from stdin, one line
//! appended holding only key names, a closed `source` word and the capture's own clock. No value of
//! any key, no prompt text and no path is written.

use std::io::{Read, Write as _};
use std::path::Path;

use serde_json::{Map, Value, json};
use viola_core::MAX_FRAME;
use viola_state::fs::open_private_append;

/// The longest key name a capture line lists; any other name is only counted.
const KEY_MAX: usize = 40;

/// Whether a key name is listed: `[a-z_]{1,40}`.
fn listed(name: &str) -> bool {
    (1..=KEY_MAX).contains(&name.len()) && name.bytes().all(|b| b.is_ascii_lowercase() || b == b'_')
}

/// The payload's `source` as one closed word.
fn source_word(payload: &Map<String, Value>) -> &'static str {
    match payload.get("source").and_then(Value::as_str) {
        Some("schedule_wakeup") => "schedule_wakeup",
        Some("loop_wakeup") => "loop_wakeup",
        Some(_) => "other",
        None => "absent",
    }
}

/// The one line a payload's bytes read to, stamped `ms`.
pub fn line_of(bytes: &[u8], ms: u128) -> Value {
    if u64::try_from(bytes.len()).map_or(true, |n| n > MAX_FRAME) {
        return json!({"unread": true, "oversize": true, "ms": ms});
    }
    let Ok(Value::Object(payload)) = serde_json::from_slice::<Value>(bytes) else {
        return json!({"unread": true, "ms": ms});
    };
    let mut keys: Vec<&str> = payload
        .keys()
        .map(String::as_str)
        .filter(|k| listed(k))
        .collect();
    keys.sort_unstable();
    let other_keys = payload.len() - keys.len();
    json!({
        "keys": keys,
        "other_keys": other_keys,
        "source": source_word(&payload),
        "ms": ms,
    })
}

/// Reads stdin through `MAX_FRAME + 1` and appends its line to `out`; every failure is silent.
pub fn run(input: impl Read, out: &Path) {
    let mut bytes = Vec::new();
    let _ = input.take(MAX_FRAME + 1).read_to_end(&mut bytes);
    let mut line = line_of(&bytes, super::isolate::now_ms()).to_string();
    line.push('\n');
    if let Ok(mut file) = open_private_append(out) {
        let _ = file.write_all(line.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_source_reads_its_closed_word() {
        let cases = [
            (
                "schedule",
                r#"{"source":"schedule_wakeup"}"#,
                "schedule_wakeup",
            ),
            ("loop", r#"{"source":"loop_wakeup"}"#, "loop_wakeup"),
            ("user", r#"{"source":"user"}"#, "other"),
            ("number", r#"{"source":7}"#, "absent"),
            ("missing", r#"{"prompt":"hi"}"#, "absent"),
        ];
        for (label, payload, want) in cases {
            assert_eq!(line_of(payload.as_bytes(), 1)["source"], want, "{label}");
        }
    }

    #[test]
    fn a_line_lists_only_patterned_key_names_and_no_value() {
        let payload = br#"{"prompt":"secret text","session_id":"abc","Weird-Key":1,"x9":2}"#;
        let line = line_of(payload, 42);
        assert_eq!(line["keys"], json!(["prompt", "session_id"]));
        assert_eq!(line["other_keys"], 2);
        assert_eq!(line["ms"], 42);
        let text = line.to_string();
        for value in ["secret text", "abc", "Weird-Key", "x9"] {
            assert!(!text.contains(value), "{value} leaked");
        }
    }

    #[test]
    fn a_key_name_past_forty_chars_is_counted_not_listed() {
        let long = "a".repeat(41);
        let payload = format!(r#"{{"{long}":1,"{}":2}}"#, "b".repeat(40));
        let line = line_of(payload.as_bytes(), 0);
        assert_eq!(line["keys"], json!(["b".repeat(40)]));
        assert_eq!(line["other_keys"], 1);
    }

    #[test]
    fn a_non_object_or_oversize_input_reads_unread() {
        assert_eq!(line_of(b"[1,2]", 5), json!({"unread": true, "ms": 5}));
        assert_eq!(line_of(b"not json", 5), json!({"unread": true, "ms": 5}));
        let big = vec![b' '; usize::try_from(MAX_FRAME).expect("fits") + 1];
        assert_eq!(
            line_of(&big, 5),
            json!({"unread": true, "oversize": true, "ms": 5})
        );
    }

    #[test]
    fn run_appends_one_private_line_per_payload() {
        use std::os::unix::fs::PermissionsExt as _;
        let dir = tempfile::tempdir().expect("tempdir");
        let out = dir.path().join("capture.ndjson");
        run(&br#"{"source":"loop_wakeup"}"#[..], &out);
        run(&b"[]"[..], &out);
        let text = std::fs::read_to_string(&out).expect("capture");
        let lines: Vec<Value> = text
            .lines()
            .map(|l| serde_json::from_str(l).expect("json"))
            .collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0]["source"], "loop_wakeup");
        assert_eq!(lines[1]["unread"], true);
        let mode = std::fs::metadata(&out).expect("meta").permissions().mode();
        assert_eq!(mode & 0o777, 0o600);
    }
}
