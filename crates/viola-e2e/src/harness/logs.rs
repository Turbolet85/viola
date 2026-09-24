//! `logs`: every `<home>/diagnostics/*.ndjson` line as `{"src":"diag","file","record"}`, then
//! every `<home>/instances/<name>/diagnostics/detail-*.ndjson` line with its `instance`; a torn or
//! unparseable line becomes `{"src":"diag","file",…,"torn":true,"offset":n}` and is never dropped
//! (test-plan §3 `logs`). The events source arrives with its producer.

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

pub fn logs(ws: &Workspace, session: &str, filter: &Filter<'_>) -> Result<Vec<Value>, Outcome> {
    let record = load_session(ws, "logs", session)?;
    let mut out = diag_lines(&record.home.join("diagnostics"), filter);
    out.extend(detail_lines(&record.home.join("instances"), filter));
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

pub fn diag_lines(dir: &Path, filter: &Filter<'_>) -> Vec<Value> {
    files_in(dir, |n| n.ends_with(".ndjson"))
        .into_iter()
        .flat_map(|(file, bytes)| wrap_lines(&file, &bytes, filter))
        .collect()
}

/// Instance dirs are named only through `ViolaName::try_new`; symlinked dirs and files are skipped.
pub fn detail_lines(instances: &Path, filter: &Filter<'_>) -> Vec<Value> {
    let mut names: Vec<String> = fs::read_dir(instances)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| ViolaName::try_new(n.clone()).is_ok())
        .collect();
    names.sort();
    let mut out = Vec::new();
    for name in names {
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

pub fn wrap_lines(file: &str, bytes: &[u8], filter: &Filter<'_>) -> Vec<Value> {
    wrap_in(file, None, bytes, filter)
}

fn wrap_in(file: &str, instance: Option<&str>, bytes: &[u8], filter: &Filter<'_>) -> Vec<Value> {
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
    let mut offset = 0;
    for chunk in bytes.split_inclusive(|b| *b == b'\n') {
        let at = offset;
        offset += chunk.len();
        let Some(line) = chunk.strip_suffix(b"\n") else {
            out.push(torn(at));
            continue;
        };
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        match serde_json::from_slice::<Value>(line) {
            Ok(record) if record.is_object() => {
                if matches(&record, filter) {
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
fn matches(record: &Value, filter: &Filter<'_>) -> bool {
    let field_is =
        |key: &str, want: Option<&str>| want.is_none_or(|w| record[key].as_str() == Some(w));
    field_is("instance", filter.instance) && field_is("process", filter.process)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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

        let only_cli = Filter {
            instance: None,
            process: Some("cli"),
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

    #[test]
    fn detail_lines_skip_a_diagnostics_path_that_is_not_a_dir() {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::create_dir(tmp.path().join("builder")).expect("instance dir");
        fs::write(tmp.path().join("builder").join("diagnostics"), "{}\n").expect("file");
        assert!(detail_lines(tmp.path(), &NONE).is_empty());
    }
}
