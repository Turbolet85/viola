//! The rig's own prompt-submit capture, the hidden `capture --out <file>` verb a wake start's
//! private config registers as its UserPromptSubmit hook: one payload read from stdin, one line
//! appended holding only key names, a closed `source` word and the capture's own clock. No value of
//! any key, no prompt text and no path is written. A StopFailure payload (the `SF` arm's hook)
//! reads to its key names, its `error` word only when it is one of the CLI's 13 (else `other`) and
//! the byte lengths of its two texts.

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

/// The CLI's 13 StopFailure error words, as `viola_agent_claude::hook::api_error_cause` maps them.
pub const ERROR_WORDS: [&str; 13] = [
    "rate_limit",
    "overloaded",
    "authentication_failed",
    "oauth_org_not_allowed",
    "account_on_hold",
    "verification_required",
    "cloud_credential_error",
    "billing_error",
    "invalid_request",
    "model_not_found",
    "server_error",
    "max_output_tokens",
    "unknown",
];

/// The payload's `error` as one closed word: itself when one of [`ERROR_WORDS`], `absent` when
/// missing, else `other`.
fn error_word(payload: &Map<String, Value>) -> &'static str {
    match payload.get("error") {
        None => "absent",
        Some(Value::String(word)) => ERROR_WORDS
            .iter()
            .find(|w| **w == word.as_str())
            .copied()
            .unwrap_or("other"),
        Some(_) => "other",
    }
}

/// A text key's byte length; `null` when absent or not a string.
fn text_len(payload: &Map<String, Value>, key: &str) -> Value {
    payload
        .get(key)
        .and_then(Value::as_str)
        .map_or(Value::Null, |text| json!(text.len()))
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
    if payload.get("hook_event_name").and_then(Value::as_str) == Some("StopFailure") {
        return json!({
            "event": "stop-failure",
            "keys": keys,
            "other_keys": other_keys,
            "error": error_word(&payload),
            "error_details_len": text_len(&payload, "error_details"),
            "last_assistant_message_len": text_len(&payload, "last_assistant_message"),
            "ms": ms,
        });
    }
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
    fn the_error_words_are_the_thirteen_the_adapter_maps_unmarked() {
        assert_eq!(ERROR_WORDS.len(), 13);
        for word in ERROR_WORDS {
            let (_, marked) = viola_agent_claude::hook::api_error_cause(Some(word));
            assert!(!marked, "{word}");
        }
        let (_, marked) = viola_agent_claude::hook::api_error_cause(Some("other"));
        assert!(marked);
    }

    #[test]
    fn a_stop_failure_line_keeps_a_known_word_and_lengths_only() {
        let dir = tempfile::tempdir().expect("tempdir");
        let out = dir.path().join("capture.ndjson");
        let needle_word = "needle_word_zq";
        let detail = "needle detail text zq";
        let message = "needle message text zq";
        let payload = |error: Value| {
            json!({
                "hook_event_name": "StopFailure",
                "session_id": "abc",
                "error": error,
                "error_details": detail,
                "last_assistant_message": message,
            })
            .to_string()
        };
        for (label, error, word) in [
            ("one of the 13", json!("server_error"), "server_error"),
            ("unknown word", json!(needle_word), "other"),
            ("a number", json!(7), "other"),
        ] {
            let body = payload(error);
            let line = line_of(body.as_bytes(), 3);
            assert_eq!(line["event"], "stop-failure", "{label}");
            assert_eq!(line["error"], word, "{label}");
            assert_eq!(line["error_details_len"], detail.len(), "{label}");
            assert_eq!(line["last_assistant_message_len"], message.len(), "{label}");
            assert_eq!(
                line["keys"],
                json!([
                    "error",
                    "error_details",
                    "hook_event_name",
                    "last_assistant_message",
                    "session_id"
                ]),
                "{label}"
            );
            run(body.as_bytes(), &out);
        }
        run(&b"[\"StopFailure\"]"[..], &out);
        let text = std::fs::read_to_string(&out).expect("capture");
        assert_eq!(text.lines().count(), 4);
        assert!(
            text.lines()
                .last()
                .is_some_and(|l| l.contains("\"unread\":true"))
        );
        for needle in [needle_word, detail, message, "abc"] {
            assert!(!text.contains(needle), "{needle} kept");
        }
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
