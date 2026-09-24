//! `logs`: every `<home>/diagnostics/*.ndjson` line as `{"src":"diag","file","record"}`; a torn or
//! unparseable line becomes `{"src":"diag","file","torn":true,"offset":n}` and is never dropped
//! (test-plan §3 `logs`). The events and detail sources arrive with their producers.

use std::fs;
use std::io::Read as _;
use std::path::Path;

use serde_json::{Value, json};

use super::{Outcome, Workspace, load_session};

/// A harness-side read cap per role file.
const MAX_FILE: u64 = 64 * 1024 * 1024;

pub struct Filter<'a> {
    pub instance: Option<&'a str>,
    pub process: Option<&'a str>,
}

pub fn logs(ws: &Workspace, session: &str, filter: &Filter<'_>) -> Result<Vec<Value>, Outcome> {
    let record = load_session(ws, "logs", session)?;
    Ok(diag_lines(&record.home.join("diagnostics"), filter))
}

pub fn diag_lines(dir: &Path, filter: &Filter<'_>) -> Vec<Value> {
    let mut files: Vec<_> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "ndjson"))
        .collect();
    files.sort();
    let mut out = Vec::new();
    for path in files {
        let file = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let mut bytes = Vec::new();
        if let Ok(f) = fs::File::open(&path) {
            let _ = f.take(MAX_FILE).read_to_end(&mut bytes);
        }
        out.extend(wrap_lines(&file, &bytes, filter));
    }
    out
}

pub fn wrap_lines(file: &str, bytes: &[u8], filter: &Filter<'_>) -> Vec<Value> {
    let mut out = Vec::new();
    let mut offset = 0;
    for chunk in bytes.split_inclusive(|b| *b == b'\n') {
        let at = offset;
        offset += chunk.len();
        let Some(line) = chunk.strip_suffix(b"\n") else {
            out.push(json!({"src": "diag", "file": file, "torn": true, "offset": at}));
            continue;
        };
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        match serde_json::from_slice::<Value>(line) {
            Ok(record) if record.is_object() => {
                if matches(&record, filter) {
                    out.push(json!({"src": "diag", "file": file, "record": record}));
                }
            }
            _ => out.push(json!({"src": "diag", "file": file, "torn": true, "offset": at})),
        }
    }
    out
}

/// An absent key and `null` are the same value (obs-plan null-as-absence), so neither matches a name.
fn matches(record: &Value, filter: &Filter<'_>) -> bool {
    let field_is =
        |key: &str, want: Option<&str>| want.is_none_or(|w| record[key].as_str() == Some(w));
    field_is("instance", filter.instance) && field_is("process", filter.process)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NONE: Filter<'static> = Filter {
        instance: None,
        process: None,
    };

    #[test]
    fn wrap_lines_wraps_records_and_marks_torn_lines() {
        let bytes =
            b"{\"event\":\"a\",\"process\":\"run\"}\nnot json\n[1]\n{\"event\":\"b\"}\n{\"partial";
        let out = wrap_lines("run-b.ndjson", bytes, &NONE);
        assert_eq!(out.len(), 5);
        assert_eq!(out[0]["src"], "diag");
        assert_eq!(out[0]["file"], "run-b.ndjson");
        assert_eq!(out[0]["record"]["event"], "a");
        let second = bytes.iter().position(|b| *b == b'\n').expect("newline") + 1;
        assert_eq!(
            out[1],
            json!({"src": "diag", "file": "run-b.ndjson", "torn": true, "offset": second})
        );
        assert_eq!(out[2]["torn"], true);
        assert_eq!(out[3]["record"]["event"], "b");
        assert_eq!(out[4]["torn"], true);
        assert_eq!(out[4]["offset"], bytes.len() - "{\"partial".len());
    }

    #[test]
    fn wrap_lines_accepts_crlf_terminated_lines() {
        let out = wrap_lines("f", b"{\"event\":\"a\"}\r\n", &NONE);
        assert_eq!(out[0]["record"]["event"], "a");
    }

    #[test]
    fn filters_match_named_values_and_treat_absent_as_null() {
        let bytes = b"{\"instance\":\"builder\",\"process\":\"run\"}\n{\"process\":\"run\"}\n{\"instance\":null,\"process\":\"ui\"}\n";
        let by_instance = Filter {
            instance: Some("builder"),
            process: None,
        };
        assert_eq!(wrap_lines("f", bytes, &by_instance).len(), 1);
        let by_process = Filter {
            instance: None,
            process: Some("run"),
        };
        assert_eq!(wrap_lines("f", bytes, &by_process).len(), 2);
        let both = Filter {
            instance: Some("builder"),
            process: Some("ui"),
        };
        assert!(wrap_lines("f", bytes, &both).is_empty());
        assert_eq!(wrap_lines("f", bytes, &NONE).len(), 3);
    }

    #[test]
    fn torn_lines_survive_any_filter() {
        let only = Filter {
            instance: Some("x"),
            process: Some("y"),
        };
        assert_eq!(wrap_lines("f", b"garbage\n", &only).len(), 1);
    }

    #[test]
    fn diag_lines_reads_only_ndjson_files_in_name_order() {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::write(tmp.path().join("run-b.ndjson"), "{\"n\":2}\n").expect("write");
        fs::write(tmp.path().join("run-a.ndjson"), "{\"n\":1}\n").expect("write");
        fs::write(tmp.path().join("notes.txt"), "{\"n\":3}\n").expect("write");
        let out = diag_lines(tmp.path(), &NONE);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0]["file"], "run-a.ndjson");
        assert_eq!(out[1]["record"]["n"], 2);
        assert!(diag_lines(&tmp.path().join("missing"), &NONE).is_empty());
    }

    #[test]
    fn logs_for_an_unknown_session_is_exit_2() {
        let out = logs(&Workspace::from_build(), "no-such-session-x9", &NONE).expect_err("unknown");
        assert_eq!(out.code, 2);
    }
}
