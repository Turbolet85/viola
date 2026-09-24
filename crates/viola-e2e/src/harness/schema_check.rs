//! `schema-check`: the check body behind obs-plan gate G4 (test-plan §6 Schema conformance). Every
//! home-level role line under `target/e2e-home/**` against `schemas/diag-line.v1.json`, every
//! instance detail line against `schemas/diag-detail.v1.json`. A failure names file, line and schema
//! keyword — never the line's content, which may carry the canary.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use super::{Outcome, Workspace, read_json};

/// Which schema a diagnostics file answers to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagKind {
    Role,
    Detail,
}

/// The files the gates read under a home tree: `*.ndjson` directly inside a `diagnostics/` dir,
/// `detail-*` ones being instance detail files. A directory at such a path is counted, never read.
#[derive(Debug, Default)]
pub struct DiagFiles {
    pub files: Vec<(PathBuf, DiagKind)>,
    pub skipped_non_file: usize,
}

pub fn diag_files(root: &Path) -> DiagFiles {
    let mut out = DiagFiles::default();
    walk(root, &mut out);
    out.files.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn walk(dir: &Path, out: &mut DiagFiles) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(meta) = fs::symlink_metadata(&path) else {
            continue;
        };
        let kind = diag_kind(&path);
        if meta.is_dir() {
            if kind.is_some() {
                out.skipped_non_file += 1;
            } else {
                walk(&path, out);
            }
        } else if let (true, Some(kind)) = (meta.is_file(), kind) {
            out.files.push((path, kind));
        }
    }
}

fn diag_kind(path: &Path) -> Option<DiagKind> {
    let name = path.file_name()?.to_str()?;
    let in_diagnostics = path
        .parent()
        .and_then(Path::file_name)
        .is_some_and(|d| d == "diagnostics");
    if !in_diagnostics || !name.ends_with(".ndjson") {
        return None;
    }
    Some(if name.starts_with("detail-") {
        DiagKind::Detail
    } else {
        DiagKind::Role
    })
}

/// `path` relative to `base`, with forward slashes, for a report that carries no host prefix.
pub fn relative(base: &Path, path: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

pub fn schema_check(ws: &Workspace) -> Outcome {
    check_tree(&ws.e2e_home(), &ws.root.join("schemas"))
}

pub fn check_tree(e2e_home: &Path, schemas: &Path) -> Outcome {
    let validator = |name: &str| {
        read_json::<Value>(&schemas.join(name))
            .ok()
            .and_then(|s| jsonschema::validator_for(&s).ok())
    };
    let (Some(line_schema), Some(detail_schema)) = (
        validator("diag-line.v1.json"),
        validator("diag-detail.v1.json"),
    ) else {
        return Outcome::new(
            json!({"v": 1, "cmd": "schema-check", "ok": false, "reason": "schema-unreadable"}),
            false,
        );
    };

    let found = diag_files(e2e_home);
    let (mut lines, mut torn) = (0usize, 0usize);
    let mut failures = Vec::new();
    for (path, kind) in &found.files {
        let file = relative(e2e_home, path);
        let Ok(text) = fs::read_to_string(path) else {
            failures.push(json!({"file": file, "line": 0, "keyword": "unreadable"}));
            continue;
        };
        let schema = match kind {
            DiagKind::Role => &line_schema,
            DiagKind::Detail => &detail_schema,
        };
        for (n, raw) in text.lines().enumerate() {
            let Ok(line) = serde_json::from_str::<Value>(raw) else {
                torn += 1;
                continue;
            };
            lines += 1;
            let mut keywords: Vec<String> = schema
                .iter_errors(&line)
                .map(|e| e.kind().keyword().to_owned())
                .collect();
            keywords.dedup();
            for keyword in keywords {
                failures.push(json!({"file": file, "line": n + 1, "keyword": keyword}));
            }
        }
    }

    let empty = found.files.is_empty();
    let ok = !empty && failures.is_empty();
    let mut doc = json!({
        "v": 1,
        "cmd": "schema-check",
        "ok": ok,
        "files": found.files.len(),
        "lines": lines,
        "torn": torn,
        "skipped_non_file": found.skipped_non_file,
        "failures": failures,
    });
    if empty {
        doc["reason"] = json!("empty-scope");
    }
    Outcome::new(doc, ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A line a real `viola run` wrote (kept-homes integration pass, 2026-09-24).
    const ROLE_LINE: &str = r#"{"timestamp":"2026-09-24T12:16:20.894Z","level":"INFO","message":"process-start","event":"process-start","process":"run","instance":"builder","subject":"self","service_name":"viola","version":"0.1.0","os":"windows","pid":7,"target":"viola::run"}"#;

    fn schemas() -> PathBuf {
        Workspace::from_build().root.join("schemas")
    }

    fn plant(root: &Path, rel: &str, text: &str) -> PathBuf {
        let path = root.join(rel);
        fs::create_dir_all(path.parent().expect("parent")).expect("dirs");
        fs::write(&path, text).expect("write");
        path
    }

    #[test]
    fn schema_check_passes_a_conforming_role_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        plant(
            tmp.path(),
            "viola-test-a/home/diagnostics/run-builder.ndjson",
            &format!("{ROLE_LINE}\n"),
        );
        let out = check_tree(tmp.path(), &schemas());
        assert_eq!(out.code, 0, "{}", out.doc);
        assert_eq!(out.doc["files"], 1);
        assert_eq!(out.doc["lines"], 1);
    }

    #[test]
    fn schema_check_names_file_line_and_keyword_never_the_content() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let secret = "canary-chain-value-5c1e";
        let bad = ROLE_LINE.replace(r#""pid":7"#, &format!(r#""pid":7,"leak":"{secret}""#));
        plant(
            tmp.path(),
            "viola-test-a/home/diagnostics/run-builder.ndjson",
            &format!("{ROLE_LINE}\n{bad}\n"),
        );
        let out = check_tree(tmp.path(), &schemas());
        assert_eq!(out.code, 1);
        let failure = &out.doc["failures"][0];
        assert_eq!(
            failure["file"],
            "viola-test-a/home/diagnostics/run-builder.ndjson"
        );
        assert_eq!(failure["line"], 2);
        assert_eq!(failure["keyword"], "unevaluatedProperties");
        assert!(!out.doc.to_string().contains(secret));
    }

    #[test]
    fn schema_check_counts_a_torn_line_and_skips_a_directory() {
        let tmp = tempfile::tempdir().expect("tempdir");
        plant(
            tmp.path(),
            "h/home/diagnostics/run-builder.ndjson",
            &format!("{ROLE_LINE}\n{{\"timestamp\":\"2026-09\n"),
        );
        fs::create_dir_all(tmp.path().join("g/home/diagnostics/run-builder.ndjson")).expect("dir");
        let out = check_tree(tmp.path(), &schemas());
        assert_eq!(out.code, 0, "{}", out.doc);
        assert_eq!(out.doc["torn"], 1);
        assert_eq!(out.doc["skipped_non_file"], 1);
    }

    #[test]
    fn schema_check_validates_detail_files_against_the_detail_schema() {
        let tmp = tempfile::tempdir().expect("tempdir");
        plant(
            tmp.path(),
            "h/home/instances/builder/diagnostics/detail-run.ndjson",
            &format!("{ROLE_LINE}\n"),
        );
        let out = check_tree(tmp.path(), &schemas());
        assert_eq!(out.doc["files"], 1);
        assert_eq!(out.code, 1, "a role line is not a detail line: {}", out.doc);
    }

    #[test]
    fn schema_check_on_an_empty_scope_is_empty_scope() {
        let tmp = tempfile::tempdir().expect("tempdir");
        plant(tmp.path(), "h/home/fake/receipt.ndjson", "{}\n");
        let out = check_tree(tmp.path(), &schemas());
        assert_eq!(out.code, 1);
        assert_eq!(out.doc["reason"], "empty-scope");
    }

    #[test]
    fn schema_check_with_unreadable_schemas_is_schema_unreadable() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let out = check_tree(tmp.path(), &tmp.path().join("no-schemas"));
        assert_eq!(out.code, 1);
        assert_eq!(out.doc["reason"], "schema-unreadable");
    }

    #[test]
    fn diag_kind_classifies_role_and_detail_files_only_inside_diagnostics() {
        let p = Path::new;
        assert_eq!(
            diag_kind(p("h/diagnostics/run-a.ndjson")),
            Some(DiagKind::Role)
        );
        assert_eq!(
            diag_kind(p("h/instances/a/diagnostics/detail-run.ndjson")),
            Some(DiagKind::Detail)
        );
        assert_eq!(diag_kind(p("h/events.ndjson")), None);
        assert_eq!(diag_kind(p("h/diagnostics/notes.txt")), None);
    }

    #[test]
    fn relative_uses_forward_slashes() {
        let base = Path::new("root");
        assert_eq!(
            relative(base, &base.join("a").join("b.ndjson")),
            "a/b.ndjson"
        );
    }
}
