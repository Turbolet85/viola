//! `logs`: every `<home>/diagnostics/*.ndjson` line as `{"src":"diag","file","record"}`, then
//! every `<home>/instances/<name>/diagnostics/detail-*.ndjson` line with its `instance`, then every
//! `<home>/instances/<name>/events.ndjson` line as `{"src":"events","instance","offset","record"}`
//! (`offset` = the line's first byte); a torn or unparseable line keeps its source and offset with
//! `"torn":true` and is never dropped (test-plan §3 `logs`). `--kind` matches an event record's
//! `kind`; a diagnostics line never matches it.

use std::fs;
use std::io::Read as _;
use std::path::Path;

use serde_json::{Map, Value};
use viola_core::ViolaName;

use super::{Outcome, Workspace, load_session};

/// A harness-side read cap per role file.
const MAX_FILE: u64 = 64 * 1024 * 1024;

pub struct Filter<'a> {
    pub instance: Option<&'a str>,
    pub process: Option<&'a str>,
}

impl<'a> Filter<'a> {
    pub fn with_kind(&self, kind: Option<&'a str>) -> Query<'a> {
        Query {
            instance: self.instance,
            process: self.process,
            kind,
        }
    }
}

/// A `Filter` plus `--kind`, which only an event record can match.
pub struct Query<'a> {
    pub instance: Option<&'a str>,
    pub process: Option<&'a str>,
    pub kind: Option<&'a str>,
}

pub fn logs(ws: &Workspace, session: &str, filter: &Filter<'_>) -> Result<Vec<Value>, Outcome> {
    query(ws, session, &filter.with_kind(None))
}

pub fn query(ws: &Workspace, session: &str, filter: &Query<'_>) -> Result<Vec<Value>, Outcome> {
    let record = load_session(ws, "logs", session)?;
    let mut out = diag_lines(&record.home.join("diagnostics"), filter);
    out.extend(detail_lines(&record.home.join("instances"), filter));
    out.extend(event_lines(&record.home.join("instances"), filter));
    Ok(out)
}

/// Entries of `dir` that are real (non-symlink) files named by `keep`, in name order.
fn files_in(dir: &Path, keep: impl Fn(&str) -> bool) -> Vec<(String, Vec<u8>)> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.file_type().is_ok_and(|t| t.is_file()))
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| keep(n))
        .collect();
    names.sort();
    names
        .into_iter()
        .map(|name| {
            let mut bytes = Vec::new();
            if let Ok(f) = fs::File::open(dir.join(&name)) {
                let _ = f.take(MAX_FILE).read_to_end(&mut bytes);
            }
            (name, bytes)
        })
        .collect()
}

pub fn diag_lines(dir: &Path, filter: &Query<'_>) -> Vec<Value> {
    files_in(dir, |n| n.ends_with(".ndjson"))
        .into_iter()
        .flat_map(|(file, bytes)| wrap_lines(&file, &bytes, filter))
        .collect()
}

/// Instance dirs, named only through `ViolaName::try_new`, in name order; symlinks are skipped.
fn instance_names(instances: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(instances)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| ViolaName::try_new(n.clone()).is_ok())
        .collect();
    names.sort();
    names
}

/// Symlinked dirs and files are skipped.
pub fn detail_lines(instances: &Path, filter: &Query<'_>) -> Vec<Value> {
    let mut out = Vec::new();
    for name in instance_names(instances) {
        let dir = instances.join(&name).join("diagnostics");
        if !fs::symlink_metadata(&dir).is_ok_and(|m| m.is_dir()) {
            continue;
        }
        let files = files_in(&dir, |n| n.starts_with("detail-") && n.ends_with(".ndjson"));
        for (file, bytes) in files {
            out.extend(wrap_in(&file, Some(&name), &bytes, filter));
        }
    }
    out
}

pub fn wrap_lines(file: &str, bytes: &[u8], filter: &Query<'_>) -> Vec<Value> {
    wrap_in(file, None, bytes, filter)
}

/// Every instance's `events.ndjson`, in instance-name order.
pub fn event_lines(instances: &Path, filter: &Query<'_>) -> Vec<Value> {
    let mut out = Vec::new();
    for name in instance_names(instances) {
        let dir = instances.join(&name);
        for (_, bytes) in files_in(&dir, |n| n == "events.ndjson") {
            out.extend(wrap_events(&name, &bytes, filter));
        }
    }
    out
}

pub fn wrap_events(instance: &str, bytes: &[u8], filter: &Query<'_>) -> Vec<Value> {
    let head = |at: usize| {
        let mut head = Map::new();
        head.insert("src".to_owned(), "events".into());
        head.insert("instance".to_owned(), instance.into());
        head.insert("offset".to_owned(), at.into());
        head
    };
    let mut out = Vec::new();
    for (at, line) in split_lines(bytes) {
        match line.and_then(|l| serde_json::from_slice::<Value>(l).ok()) {
            Some(record) if record.is_object() => {
                if event_matches(&record, filter) {
                    let mut wrapped = head(at);
                    wrapped.insert("record".to_owned(), record);
                    out.push(Value::Object(wrapped));
                }
            }
            _ => {
                let mut torn = head(at);
                torn.insert("torn".to_owned(), true.into());
                out.push(Value::Object(torn));
            }
        }
    }
    out
}

/// Each line's starting offset and its bytes without the terminator; `None` for an unterminated
/// tail.
fn split_lines(bytes: &[u8]) -> Vec<(usize, Option<&[u8]>)> {
    let mut out = Vec::new();
    let mut offset = 0;
    for chunk in bytes.split_inclusive(|b| *b == b'\n') {
        let at = offset;
        offset += chunk.len();
        let line = chunk
            .strip_suffix(b"\n")
            .map(|l| l.strip_suffix(b"\r").unwrap_or(l));
        out.push((at, line));
    }
    out
}

fn wrap_in(file: &str, instance: Option<&str>, bytes: &[u8], filter: &Query<'_>) -> Vec<Value> {
    let head = || {
        let mut head = Map::new();
        head.insert("src".to_owned(), "diag".into());
        head.insert("file".to_owned(), file.into());
        if let Some(instance) = instance {
            head.insert("instance".to_owned(), instance.into());
        }
        head
    };
    let torn = |at: usize| {
        let mut out = head();
        out.insert("torn".to_owned(), true.into());
        out.insert("offset".to_owned(), at.into());
        Value::Object(out)
    };
    let mut out = Vec::new();
    for (at, line) in split_lines(bytes) {
        let Some(line) = line else {
            out.push(torn(at));
            continue;
        };
        match serde_json::from_slice::<Value>(line) {
            Ok(record) if record.is_object() => {
                if filter.kind.is_none() && matches(&record, filter) {
                    let mut wrapped = head();
                    wrapped.insert("record".to_owned(), record);
                    out.push(Value::Object(wrapped));
                }
            }
            _ => out.push(torn(at)),
        }
    }
    out
}

/// An absent key and `null` are the same value (obs-plan null-as-absence), so neither matches a name.
fn field_is(record: &Value, key: &str, want: Option<&str>) -> bool {
    want.is_none_or(|w| record[key].as_str() == Some(w))
}

fn matches(record: &Value, filter: &Query<'_>) -> bool {
    field_is(record, "instance", filter.instance) && field_is(record, "process", filter.process)
}

fn event_matches(record: &Value, filter: &Query<'_>) -> bool {
    matches(record, filter) && field_is(record, "kind", filter.kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const NONE: Query<'static> = Query {
        instance: None,
        process: None,
        kind: None,
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
        let by_instance = Query {
            instance: Some("builder"),
            process: None,
            kind: None,
        };
        assert_eq!(wrap_lines("f", bytes, &by_instance).len(), 1);
        let by_process = Query {
            instance: None,
            process: Some("run"),
            kind: None,
        };
        assert_eq!(wrap_lines("f", bytes, &by_process).len(), 2);
        let both = Query {
            instance: Some("builder"),
            process: Some("ui"),
            kind: None,
        };
        assert!(wrap_lines("f", bytes, &both).is_empty());
        assert_eq!(wrap_lines("f", bytes, &NONE).len(), 3);
    }

    #[test]
    fn torn_lines_survive_any_filter() {
        let only = Query {
            instance: Some("x"),
            process: Some("y"),
            kind: Some("z"),
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

    /// A role file well past 1 MiB is read whole: the cap is 64 MiB, written as one literal here.
    #[test]
    fn files_in_reads_a_role_file_whole_below_the_cap() {
        assert_eq!(MAX_FILE, 67_108_864);
        let tmp = tempfile::tempdir().expect("tempdir");
        let line = format!("{{\"pad\":\"{}\"}}\n", "x".repeat(1000));
        let text = line.repeat(2100);
        fs::write(tmp.path().join("run-a.ndjson"), &text).expect("write");
        let files = files_in(tmp.path(), |n| n.ends_with(".ndjson"));
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].1.len(), text.len());
        assert_eq!(diag_lines(tmp.path(), &NONE).len(), 2100);
    }

    #[test]
    fn logs_for_an_unknown_session_is_exit_2() {
        let filter = Filter {
            instance: None,
            process: None,
        };
        let out =
            logs(&Workspace::from_build(), "no-such-session-x9", &filter).expect_err("unknown");
        assert_eq!(out.code, 2);
        let out =
            query(&Workspace::from_build(), "no-such-session-x9", &NONE).expect_err("unknown");
        assert_eq!(out.code, 2);
    }

    #[test]
    fn filter_with_kind_keeps_instance_and_process() {
        let filter = Filter {
            instance: Some("builder"),
            process: Some("run"),
        };
        let q = filter.with_kind(Some("wheel"));
        assert_eq!(
            (q.instance, q.process, q.kind),
            (Some("builder"), Some("run"), Some("wheel"))
        );
    }

    fn plant(instances: &Path, dir: &str, file: &str, text: &str) {
        let diag = instances.join(dir).join("diagnostics");
        fs::create_dir_all(&diag).expect("dirs");
        fs::write(diag.join(file), text).expect("write");
    }

    #[test]
    fn detail_lines_carry_instance_and_skip_invalid_names() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instances = tmp.path();
        let line = "{\"event\":\"panic\",\"process\":\"run\",\"instance\":\"builder\"}\n";
        plant(instances, "builder", "detail-run.ndjson", line);
        plant(instances, "builder", "run-builder.ndjson", line);
        plant(
            instances,
            "overseer",
            "detail-cli.ndjson",
            "{\"event\":\"panic\",\"process\":\"cli\",\"instance\":\"overseer\"}\n",
        );
        plant(instances, "bad_name", "detail-run.ndjson", line);
        plant(instances, "1abc", "detail-run.ndjson", line);
        fs::write(instances.join("stray.ndjson"), line).expect("write");

        let out = detail_lines(instances, &NONE);
        assert_eq!(out.len(), 2);
        assert_eq!(
            out[0],
            json!({"src": "diag", "file": "detail-run.ndjson", "instance": "builder",
                   "record": {"event": "panic", "process": "run", "instance": "builder"}})
        );
        assert_eq!(out[1]["instance"], "overseer");
        assert_eq!(out[1]["file"], "detail-cli.ndjson");

        let only_cli = Query {
            instance: None,
            process: Some("cli"),
            kind: None,
        };
        let filtered = detail_lines(instances, &only_cli);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0]["instance"], "overseer");
        assert!(detail_lines(&instances.join("missing"), &NONE).is_empty());
    }

    #[test]
    fn detail_lines_mark_torn_lines() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let whole = "{\"event\":\"panic\"}\n";
        plant(
            tmp.path(),
            "builder",
            "detail-run.ndjson",
            &format!("{whole}{{\"par"),
        );
        let out = detail_lines(tmp.path(), &NONE);
        assert_eq!(out.len(), 2);
        assert_eq!(
            out[1],
            json!({"src": "diag", "file": "detail-run.ndjson", "instance": "builder",
                   "torn": true, "offset": whole.len()})
        );
    }

    const WHEEL: &str = "{\"v\":1,\"instance\":\"builder\",\"kind\":\"wheel\",\"source\":\"wrapper\",\"data\":{\"cause\":\"start\"}}\n";
    const GATE: &str = "{\"v\":1,\"instance\":\"builder\",\"kind\":\"budget-gate\",\"source\":\"wrapper\",\"data\":{}}\n";

    #[test]
    fn wrap_events_carry_instance_and_line_offsets() {
        let bytes = format!("{WHEEL}not json\n{GATE}{{\"par");
        let out = wrap_events("builder", bytes.as_bytes(), &NONE);
        assert_eq!(out.len(), 4);
        assert_eq!(
            out[0],
            json!({"src": "events", "instance": "builder", "offset": 0,
                   "record": serde_json::from_str::<Value>(WHEEL).expect("json")})
        );
        assert_eq!(
            out[1],
            json!({"src": "events", "instance": "builder", "offset": WHEEL.len(), "torn": true})
        );
        let gate_at = WHEEL.len() + "not json\n".len();
        assert_eq!(out[2]["offset"], gate_at);
        assert_eq!(out[2]["record"]["kind"], "budget-gate");
        assert_eq!(
            out[3],
            json!({"src": "events", "instance": "builder", "offset": gate_at + GATE.len(), "torn": true})
        );
    }

    #[test]
    fn wrap_events_mark_json_that_is_not_an_object_torn() {
        let out = wrap_events("builder", b"[1]\n\"text\"\n", &NONE);
        assert_eq!(
            out,
            [
                json!({"src": "events", "instance": "builder", "offset": 0, "torn": true}),
                json!({"src": "events", "instance": "builder", "offset": 4, "torn": true}),
            ]
        );
    }

    #[test]
    fn kind_filter_matches_event_kinds_and_never_a_diag_line() {
        let wheel = Query {
            instance: None,
            process: None,
            kind: Some("wheel"),
        };
        let bytes = format!("{WHEEL}{GATE}");
        let out = wrap_events("builder", bytes.as_bytes(), &wheel);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0]["record"]["kind"], "wheel");
        assert_eq!(wrap_events("builder", bytes.as_bytes(), &NONE).len(), 2);
        let diag = b"{\"event\":\"process-start\",\"kind\":\"wheel\"}\n";
        assert!(wrap_lines("run-builder.ndjson", diag, &wheel).is_empty());
        assert_eq!(wrap_lines("run-builder.ndjson", diag, &NONE).len(), 1);
        let other = Query {
            instance: Some("overseer"),
            process: None,
            kind: Some("wheel"),
        };
        assert!(wrap_events("builder", bytes.as_bytes(), &other).is_empty());
    }

    #[test]
    fn event_lines_read_each_valid_instance_in_name_order() {
        let tmp = tempfile::tempdir().expect("tempdir");
        for dir in ["overseer", "builder", "bad_name"] {
            fs::create_dir_all(tmp.path().join(dir)).expect("dir");
            fs::write(tmp.path().join(dir).join("events.ndjson"), WHEEL).expect("write");
        }
        fs::write(tmp.path().join("builder").join("other.ndjson"), GATE).expect("write");
        let out = event_lines(tmp.path(), &NONE);
        let names: Vec<&Value> = out.iter().map(|l| &l["instance"]).collect();
        assert_eq!(names, [&json!("builder"), &json!("overseer")]);
        assert!(event_lines(&tmp.path().join("missing"), &NONE).is_empty());
    }

    #[test]
    fn detail_lines_skip_a_diagnostics_path_that_is_not_a_dir() {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::create_dir(tmp.path().join("builder")).expect("instance dir");
        fs::write(tmp.path().join("builder").join("diagnostics"), "{}\n").expect("file");
        assert!(detail_lines(tmp.path(), &NONE).is_empty());
    }
}
