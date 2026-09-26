//! `secret-scan`: the CI scan behind obs-plan §9 step 3 (test-plan §6 Error sanitization and secret
//! scan). It runs before any upload, over the home diagnostics files, the harness capture and the
//! nextest JUnit report. A hit names file, line, byte offset and class — never the matched bytes —
//! and the report goes to `target/secret-scan/hits.json` for the `secret-scan-<os>` upload.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use super::schema_check::{DiagKind, diag_files, relative};
use super::{Outcome, Workspace, write_json};

/// NEVER-log values: forbidden in every scanned file, detail files included (obs-plan §8). The
/// stripped `CLAUDE*` ones are the fixed canaries the root tests plant through the environment.
pub const CRITICAL: [(&str, &str); 13] = [
    ("claude-stripped", "canary-token-value-7f3a"),
    ("claude-stripped", "canary-socket-value-2b9d"),
    ("claude-stripped", "canary-entrypoint-value-8e41"),
    ("claude-stripped", "canary-claudecode-value-3c07"),
    ("claude-stripped", "canary-bridge-value-91d2"),
    ("claude-stripped", "canary-child-value-4e6b"),
    ("claude-stripped", "canary-execpath-value-a813"),
    ("claude-stripped", "canary-attended-value-5f29"),
    ("claude-stripped", "canary-session-value-d4a0"),
    ("claude-stripped", "canary-effort-value-0b7e"),
    ("claude-stripped", "canary-pid-value-6c35"),
    ("claude-stripped", "canary-newname-value-e2f8"),
    ("token-query", "?t="),
];

/// Content canaries: allowed in detail files and the harness capture, never in a home-level role
/// file (obs-plan §8 item 6).
pub const CONTENT: [(&str, &str); 1] = [("content-canary", "canary-chain-value-5c1e")];

/// The scanned roots; each may be absent (`agent_run` and `junit` exist only on some runs).
#[derive(Debug, Clone)]
pub struct Roots {
    pub e2e_home: PathBuf,
    pub agent_run: PathBuf,
    pub junit: PathBuf,
    /// The prefix a report path is made relative to (`target/`).
    pub base: PathBuf,
}

impl Roots {
    pub fn of(ws: &Workspace) -> Self {
        let target = ws.root.join("target");
        Self {
            e2e_home: ws.e2e_home(),
            agent_run: ws.agent_run(),
            junit: target.join("nextest").join("ci").join("junit.xml"),
            base: target,
        }
    }
}

pub fn secret_scan(ws: &Workspace) -> Outcome {
    scan(&Roots::of(ws), &ws.root.join("target").join("secret-scan"))
}

pub fn scan(roots: &Roots, report_dir: &Path) -> Outcome {
    let _ = fs::remove_file(report_dir.join("hits.json"));
    let diag = diag_files(&roots.e2e_home);
    let tokens = gui_tokens(&roots.e2e_home);
    let mut hits = Vec::new();
    let mut files = 0usize;

    for (path, kind) in &diag.files {
        files += 1;
        let role = *kind == DiagKind::Role;
        scan_file(roots, path, role, &tokens, &mut hits);
        if let Some(hit) = mode_hit(roots, path, file_mode(path)) {
            hits.push(hit);
        }
    }
    let mut capture = Vec::new();
    collect_files(&roots.agent_run, &mut capture);
    // The mutation leg's `chunk.diff` is repository source by construction (this file's own patterns
    // included), not runtime capture, and the `harness-<os>` upload excludes it.
    let mutation_diff = roots.agent_run.join("chunk.diff");
    capture.retain(|path| *path != mutation_diff);
    if roots.junit.is_file() {
        capture.push(roots.junit.clone());
    }
    for path in &capture {
        files += 1;
        scan_file(roots, path, false, &tokens, &mut hits);
    }

    let empty = diag.files.is_empty();
    let ok = !empty && hits.is_empty();
    let mut doc = json!({
        "v": 1,
        "cmd": "secret-scan",
        "ok": ok,
        "files": files,
        "skipped_non_file": diag.skipped_non_file,
        "mode_check": if cfg!(unix) { "unix" } else { "skipped-windows" },
        "hits": hits,
    });
    if empty {
        doc["reason"] = json!("empty-scope");
    }
    if !hits.is_empty() {
        let written = fs::create_dir_all(report_dir)
            .is_ok_and(|()| write_json(&report_dir.join("hits.json"), &doc).is_ok());
        if !written {
            doc["report"] = json!("unwritten");
        }
    }
    Outcome::new(doc, ok)
}

/// Every hit of `needles` in one file's bytes, located but never quoted.
fn scan_file(roots: &Roots, path: &Path, role: bool, tokens: &[String], hits: &mut Vec<Value>) {
    let Ok(bytes) = fs::read(path) else {
        hits.push(hit(roots, path, 0, 0, "unreadable"));
        return;
    };
    let lower = bytes.to_ascii_lowercase();
    let mut report = |offset: usize, class: &str| {
        let line = bytes[..offset].iter().filter(|&&b| b == b'\n').count() + 1;
        hits.push(hit(roots, path, line, offset, class));
    };
    for (class, needle) in CRITICAL {
        for offset in find_all(&bytes, needle.as_bytes()) {
            report(offset, class);
        }
    }
    for token in tokens {
        for offset in find_all(&bytes, token.as_bytes()) {
            report(offset, "gui-token");
        }
    }
    for offset in find_all(&lower, b"cookie:") {
        report(offset, "cookie");
    }
    for offset in cookie_pairs(&bytes) {
        report(offset, "cookie");
    }
    if role {
        for (class, needle) in CONTENT {
            for offset in find_all(&bytes, needle.as_bytes()) {
                report(offset, class);
            }
        }
    }
}

fn hit(roots: &Roots, path: &Path, line: usize, offset: usize, class: &str) -> Value {
    json!({"file": relative(&roots.base, path), "line": line, "offset": offset, "class": class})
}

fn find_all(haystack: &[u8], needle: &[u8]) -> Vec<usize> {
    // `windows(0)` panics; a haystack shorter than the needle yields no window at all.
    if needle.is_empty() {
        return Vec::new();
    }
    haystack
        .windows(needle.len())
        .enumerate()
        .filter(|(_, w)| *w == needle)
        .map(|(i, _)| i)
        .collect()
}

/// `viola_<port>=`: the GUI cookie as it would appear in a `Cookie` or `Set-Cookie` value.
fn cookie_pairs(bytes: &[u8]) -> Vec<usize> {
    find_all(bytes, b"viola_")
        .into_iter()
        .filter(|&at| {
            let rest = &bytes[at + b"viola_".len()..];
            let digits = rest.iter().take_while(|b| b.is_ascii_digit()).count();
            digits > 0 && rest.get(digits) == Some(&b'=')
        })
        .collect()
}

/// The per-launch token of every home that ran `viola ui`: the `t` value in `ui/<port>.url`.
fn gui_tokens(e2e_home: &Path) -> Vec<String> {
    let mut all = Vec::new();
    collect_files(e2e_home, &mut all);
    let mut tokens: Vec<String> = all
        .iter()
        .filter(|p| {
            p.extension().is_some_and(|e| e == "url")
                && p.parent()
                    .and_then(Path::file_name)
                    .is_some_and(|d| d == "ui")
        })
        .filter_map(|p| fs::read_to_string(p).ok())
        .filter_map(|text| url_token(&text))
        .collect();
    tokens.sort();
    tokens.dedup();
    tokens
}

fn url_token(text: &str) -> Option<String> {
    let query = text.trim().split_once('?')?.1;
    query
        .split('&')
        .find_map(|pair| pair.strip_prefix("t="))
        .map(|t| t.trim().to_owned())
        .filter(|t| !t.is_empty())
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        match fs::symlink_metadata(&path) {
            Ok(meta) if meta.is_file() => out.push(path),
            // Anything else is descended into; `read_dir` refuses whatever is not a directory.
            Ok(_) => collect_files(&path, out),
            Err(_) => {}
        }
    }
}

/// A diagnostics file must be owner-only (obs-plan §10). Windows access is the home DACL, which the
/// strict-modes check owns; there is no mode to read and the document says the check was skipped.
fn mode_hit(roots: &Roots, path: &Path, mode: Option<u32>) -> Option<Value> {
    mode.filter(|&m| !owner_only(m))
        .map(|_| hit(roots, path, 0, 0, "mode"))
}

fn owner_only(mode: u32) -> bool {
    mode & 0o777 == 0o600
}

/// One body for every OS: cargo-mutants mutates `#[cfg]`-gated items it cannot compile on the
/// runner, so a per-OS pair of functions leaves the other OS's body unkillable in CI.
fn file_mode(path: &Path) -> Option<u32> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::metadata(path).ok().map(|m| m.permissions().mode())
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Tree {
        _tmp: tempfile::TempDir,
        roots: Roots,
        report: PathBuf,
    }

    fn tree() -> Tree {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = tmp.path().to_path_buf();
        let roots = Roots {
            e2e_home: base.join("e2e-home"),
            agent_run: base.join("agent-run"),
            junit: base.join("nextest").join("ci").join("junit.xml"),
            base: base.clone(),
        };
        let t = Tree {
            report: base.join("secret-scan"),
            roots,
            _tmp: tmp,
        };
        plant(
            &t.roots.e2e_home,
            "h/home/diagnostics/run-builder.ndjson",
            "{\"event\":\"x\"}\n",
        );
        t
    }

    fn plant(root: &Path, rel: &str, text: &str) -> PathBuf {
        let path = root.join(rel);
        fs::create_dir_all(path.parent().expect("parent")).expect("dirs");
        fs::write(&path, text).expect("write");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).expect("mode");
        }
        path
    }

    fn classes(out: &Outcome) -> Vec<String> {
        out.doc["hits"]
            .as_array()
            .expect("hits")
            .iter()
            .map(|h| h["class"].as_str().expect("class").to_owned())
            .collect()
    }

    #[test]
    fn secret_scan_passes_a_clean_tree() {
        let t = tree();
        let out = scan(&t.roots, &t.report);
        assert_eq!(out.code, 0, "{}", out.doc);
        assert!(!t.report.join("hits.json").exists());
    }

    #[test]
    fn secret_scan_flags_every_critical_class_even_in_a_detail_file() {
        let t = tree();
        let detail = "h/home/instances/builder/diagnostics/detail-run.ndjson";
        for (class, text) in [
            ("claude-stripped", "{\"v\":\"canary-socket-value-2b9d\"}\n"),
            ("token-query", "{\"u\":\"/?t=abc\"}\n"),
            ("cookie", "{\"h\":\"COOKIE: x\"}\n"),
            ("cookie", "{\"h\":\"viola_47319=abc\"}\n"),
        ] {
            plant(&t.roots.e2e_home, detail, text);
            let out = scan(&t.roots, &t.report);
            assert_eq!(out.code, 1, "{class}: {}", out.doc);
            assert_eq!(classes(&out), [class]);
            assert!(t.report.join("hits.json").is_file());
        }
    }

    #[test]
    fn secret_scan_reads_the_gui_token_from_the_url_file() {
        let t = tree();
        plant(
            &t.roots.e2e_home,
            "h/home/ui/47319.url",
            "http://127.0.0.1:47319/?t=tok9z\n",
        );
        // A `.url` outside a `ui/` dir is not a launch file.
        plant(
            &t.roots.e2e_home,
            "h/home/notes/x.url",
            "http://h/?t=other1\n",
        );
        plant(
            &t.roots.agent_run,
            "logs.ndjson",
            "{\"line\":\"tok9z\",\"other\":\"other1\"}\n",
        );
        let out = scan(&t.roots, &t.report);
        assert_eq!(classes(&out), ["gui-token"]);
        assert_eq!(out.doc["hits"][0]["file"], "agent-run/logs.ndjson");
        // The launch token `tok9z` (offset 9), not `other1` from the stray `.url` (offset 26).
        assert_eq!(out.doc["hits"][0]["offset"], 9);
        assert_eq!(out.doc["files"], 2, "one role file and one capture file");
    }

    #[test]
    fn secret_scan_descends_into_the_capture_tree() {
        let t = tree();
        plant(
            &t.roots.agent_run,
            "ci-smoke/nested/status.json",
            "{\"u\":\"?t=x\"}\n",
        );
        let out = scan(&t.roots, &t.report);
        assert_eq!(classes(&out), ["token-query"]);
        assert_eq!(
            out.doc["hits"][0]["file"],
            "agent-run/ci-smoke/nested/status.json"
        );
    }

    const DIFF_LINE: &str = "+    (\"token-query\", \"?t=\"),\n";

    #[test]
    fn secret_scan_skips_the_mutation_diff() {
        let t = tree();
        plant(&t.roots.agent_run, "chunk.diff", DIFF_LINE);
        let out = scan(&t.roots, &t.report);
        assert_eq!(out.code, 0, "{}", out.doc);
        assert_eq!(out.doc["files"], 1, "the role file only");
    }

    #[test]
    fn secret_scan_skips_the_mutation_diff_only_at_its_path() {
        let t = tree();
        plant(&t.roots.agent_run, "chunk.diff", DIFF_LINE);
        plant(&t.roots.agent_run, "logs.ndjson", DIFF_LINE);
        plant(&t.roots.agent_run, "x/chunk.diff", DIFF_LINE);
        let out = scan(&t.roots, &t.report);
        assert_eq!(classes(&out), ["token-query", "token-query"]);
        assert_eq!(out.doc["files"], 3);
    }

    #[test]
    fn secret_scan_marks_a_report_it_could_not_write() {
        let t = tree();
        plant(&t.roots.agent_run, "logs.ndjson", "?t=x\n");
        let blocker = plant(&t.roots.base, "blocker", "a file, not a directory\n");
        let out = scan(&t.roots, &blocker.join("secret-scan"));
        assert_eq!(out.code, 1);
        assert_eq!(out.doc["report"], "unwritten");
        let written = scan(&t.roots, &t.report);
        assert!(written.doc.get("report").is_none());
    }

    #[test]
    fn mode_hit_flags_only_a_mode_that_is_not_owner_only() {
        let t = tree();
        let path = t
            .roots
            .e2e_home
            .join("h/home/diagnostics/run-builder.ndjson");
        assert!(mode_hit(&t.roots, &path, None).is_none());
        assert!(mode_hit(&t.roots, &path, Some(0o100_600)).is_none());
        let hit = mode_hit(&t.roots, &path, Some(0o100_644)).expect("a hit");
        assert_eq!(hit["class"], "mode");
        assert!(owner_only(0o600));
        assert!(owner_only(0o100_600));
        assert!(!owner_only(0o644));
        assert!(!owner_only(0o700));
    }

    #[test]
    fn secret_scan_content_canary_fails_a_role_file_only() {
        let t = tree();
        plant(
            &t.roots.e2e_home,
            "h/home/instances/builder/diagnostics/detail-run.ndjson",
            "{\"chain\":\"canary-chain-value-5c1e\"}\n",
        );
        plant(
            &t.roots.agent_run,
            "logs.ndjson",
            "canary-chain-value-5c1e\n",
        );
        assert_eq!(scan(&t.roots, &t.report).code, 0);

        plant(
            &t.roots.e2e_home,
            "h/home/diagnostics/run-builder.ndjson",
            "{\"event\":\"x\"}\n{\"m\":\"canary-chain-value-5c1e\"}\n",
        );
        let out = scan(&t.roots, &t.report);
        assert_eq!(classes(&out), ["content-canary"]);
    }

    #[test]
    fn secret_scan_hit_locates_without_the_matched_bytes() {
        let t = tree();
        plant(
            &t.roots.e2e_home,
            "h/home/diagnostics/run-builder.ndjson",
            "{\"event\":\"x\"}\n{\"t\":\"canary-token-value-7f3a\"}\n",
        );
        let out = scan(&t.roots, &t.report);
        let h = &out.doc["hits"][0];
        assert_eq!(h["file"], "e2e-home/h/home/diagnostics/run-builder.ndjson");
        assert_eq!(h["line"], 2);
        // `{"event":"x"}\n` is 14 bytes, then `{"t":"` is 6.
        assert_eq!(h["offset"], 20);
        let report = fs::read_to_string(t.report.join("hits.json")).expect("report");
        for text in [out.doc.to_string(), report] {
            assert!(!text.contains("canary-token-value-7f3a"));
        }
    }

    #[test]
    fn secret_scan_scans_the_junit_report() {
        let t = tree();
        plant(
            &t.roots.base,
            "nextest/ci/junit.xml",
            "<system-out>?t=x</system-out>\n",
        );
        assert_eq!(classes(&scan(&t.roots, &t.report)), ["token-query"]);
    }

    #[test]
    fn secret_scan_on_an_empty_scope_is_empty_scope() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let roots = Roots {
            e2e_home: tmp.path().join("e2e-home"),
            agent_run: tmp.path().join("agent-run"),
            junit: tmp.path().join("junit.xml"),
            base: tmp.path().to_path_buf(),
        };
        let out = scan(&roots, &tmp.path().join("secret-scan"));
        assert_eq!(out.code, 1);
        assert_eq!(out.doc["reason"], "empty-scope");
    }

    #[test]
    fn secret_scan_clears_a_stale_report_on_a_clean_run() {
        let t = tree();
        plant(&t.report, "hits.json", "{}\n");
        assert_eq!(scan(&t.roots, &t.report).code, 0);
        assert!(!t.report.join("hits.json").exists());
    }

    #[cfg(unix)]
    #[test]
    fn secret_scan_flags_a_diagnostics_file_that_is_not_owner_only() {
        use std::os::unix::fs::PermissionsExt as _;
        let t = tree();
        let path = t
            .roots
            .e2e_home
            .join("h/home/diagnostics/run-builder.ndjson");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).expect("mode");
        let out = scan(&t.roots, &t.report);
        assert_eq!(classes(&out), ["mode"]);
        assert_eq!(out.doc["mode_check"], "unix");
    }

    #[test]
    fn url_token_takes_the_t_parameter() {
        assert_eq!(url_token("http://h/?a=1&t=abc\n"), Some("abc".to_owned()));
        assert_eq!(url_token("http://h/?t="), None);
        assert_eq!(url_token("http://h/"), None);
    }

    #[test]
    fn cookie_pairs_need_digits_then_equals() {
        assert_eq!(cookie_pairs(b"viola_47319=x"), vec![0]);
        assert!(cookie_pairs(b"viola_=x viola_12 viola_ab=").is_empty());
    }

    #[test]
    fn find_all_handles_short_and_empty_inputs() {
        assert!(find_all(b"ab", b"abc").is_empty());
        assert_eq!(find_all(b"abc", b"abc"), vec![0]);
        assert!(find_all(b"abc", b"").is_empty());
        assert_eq!(find_all(b"aXaXa", b"a"), vec![0, 2, 4]);
    }
}
