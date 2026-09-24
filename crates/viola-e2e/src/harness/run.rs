//! `run`: the built suites (unit · integration · doctest) and the per-chunk mutation gate, one JSON
//! summary, merged by suite into `target/agent-run/artifacts/run-summary.json` (test-plan §3 `run`).

use std::fs;
use std::io::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::{Outcome, Workspace, read_json, write_json};

const UNIT: &str = "kind(lib) | kind(bin)";
// test-plan §3 also excludes the E2E binaries (`path_`, `tui_`, …); nextest rejects a `binary()`
// operator that matches no binary, so that exclusion arrives with the first E2E binary and `--e2e`.
const INTEGRATION: &str = "kind(test)";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub unit: bool,
    pub integration: bool,
    pub mutants: bool,
}

impl Selection {
    /// No selector, or `--all`, selects every built suite.
    pub fn from_flags(unit: bool, integration: bool, mutants: bool, all: bool) -> Self {
        let none = !(unit || integration || mutants);
        Self {
            unit: unit || all || none,
            integration: integration || all || none,
            mutants: mutants || all || none,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Suite {
    pub suite: String,
    pub passed: u64,
    pub failed: u64,
    pub skipped: u64,
    pub survived: u64,
    pub artifact: Option<String>,
    pub failures: Vec<String>,
}

impl Suite {
    fn named(suite: &str) -> Self {
        Self {
            suite: suite.to_owned(),
            ..Self::default()
        }
    }

    fn green(&self) -> bool {
        self.failed == 0 && self.survived == 0
    }
}

/// `chunk_base` is `AGENT_RUN_CHUNK_BASE` as the caller read it.
pub fn run(
    ws: &Workspace,
    sel: Selection,
    filter: Option<&str>,
    chunk_base: Option<String>,
) -> Outcome {
    let mut suites = Vec::new();
    if sel.unit {
        suites.push(nextest(ws, "nextest-unit", UNIT, filter));
    }
    if sel.integration {
        suites.push(nextest(ws, "nextest-integration", INTEGRATION, filter));
    }
    if sel.unit || sel.integration {
        suites.push(doctest(ws));
    }
    let mut reason = None;
    let mut tested = None;
    if sel.mutants {
        match mutants(ws, chunk_base) {
            Ok((suite, n)) => {
                suites.push(suite);
                tested = Some(n);
            }
            Err(r) => reason = Some(r),
        }
    }
    let ok = reason.is_none() && suites.iter().all(Suite::green);
    let _ = merge_summary(&ws.artifacts().join("run-summary.json"), &suites);
    let mut doc = json!({"v": 1, "cmd": "run", "ok": ok});
    if let Some(reason) = reason {
        doc["reason"] = json!(reason);
    }
    doc["suites"] = json!(suites);
    if let Some(tested) = tested {
        doc["mutants"] = json!({"tested": tested});
    }
    Outcome::new(doc, ok)
}

/// Runs a cargo tool with its stdout moved to our stderr: the harness's stdout carries only its
/// document.
fn run_forwarding(cmd: &mut Command) -> (Option<i32>, String) {
    let output = cmd.stdin(Stdio::null()).stderr(Stdio::inherit()).output();
    match output {
        Ok(out) => {
            let _ = std::io::stderr().write_all(&out.stdout);
            (
                out.status.code(),
                String::from_utf8_lossy(&out.stdout).into_owned(),
            )
        }
        Err(_) => (None, String::new()),
    }
}

fn nextest(ws: &Workspace, suite: &str, layer: &str, filter: Option<&str>) -> Suite {
    // nextest keeps its store under `<workspace root>/target/nextest` whatever the target dir.
    let junit = ws
        .root
        .join("target")
        .join("nextest")
        .join("ci")
        .join("junit.xml");
    let _ = fs::remove_file(&junit);
    let expr = match filter {
        Some(f) => format!("({layer}) & ({f})"),
        None => layer.to_owned(),
    };
    let (code, _) = run_forwarding(
        Command::new("cargo")
            .args([
                "nextest",
                "run",
                "--workspace",
                "--features",
                "viola/fake-agent",
            ])
            .args(["--profile", "ci", "-E", &expr])
            .env("CARGO_TARGET_DIR", ws.cargo_target())
            .current_dir(&ws.root),
    );
    let artifact = ws.artifacts().join(format!("junit-{suite}.xml"));
    let Ok(xml) = fs::read_to_string(&junit) else {
        return Suite {
            failed: 1,
            failures: vec!["artifact-missing".to_owned()],
            ..Suite::named(suite)
        };
    };
    let _ = fs::create_dir_all(ws.artifacts());
    let _ = fs::write(&artifact, &xml);
    let mut result = parse_junit(&xml);
    result.suite = suite.to_owned();
    result.artifact = Some(artifact.to_string_lossy().into_owned());
    exit_must_agree(&mut result, code, "nextest");
    result
}

/// A red tool exit with nothing red in its report is still red (a build failure, no tests run).
fn exit_must_agree(suite: &mut Suite, code: Option<i32>, tool: &str) {
    if code != Some(0) && suite.failed == 0 {
        suite.failed = 1;
        let code = code.map_or("signal".to_owned(), |c| c.to_string());
        suite.failures.push(format!("{tool}-exit-{code}"));
    }
}

fn attr<'a>(head: &'a str, key: &str) -> &'a str {
    let needle = format!(" {key}=\"");
    head.find(&needle)
        .map(|at| &head[at + needle.len()..])
        .and_then(|rest| rest.split('"').next())
        .unwrap_or("")
}

/// Counts from nextest's JUnit: `skipped` is only a selected test that did not run (`<skipped/>`).
pub fn parse_junit(xml: &str) -> Suite {
    let mut suite = Suite::default();
    for piece in xml.split("<testcase").skip(1) {
        let head_end = piece.find('>').unwrap_or(piece.len());
        let head = &piece[..head_end];
        let body = if head.ends_with('/') {
            ""
        } else {
            let end = piece.find("</testcase>").unwrap_or(piece.len());
            &piece[head_end.min(end)..end]
        };
        if body.contains("<failure") || body.contains("<error") {
            suite.failed += 1;
            suite.failures.push(format!(
                "{} {}",
                attr(head, "classname"),
                attr(head, "name")
            ));
        } else if body.contains("<skipped") {
            suite.skipped += 1;
        } else {
            suite.passed += 1;
        }
    }
    suite
}

fn doctest(ws: &Workspace) -> Suite {
    let (code, out) = run_forwarding(
        Command::new("cargo")
            .args([
                "test",
                "--workspace",
                "--doc",
                "--features",
                "viola/fake-agent",
            ])
            .env("CARGO_TARGET_DIR", ws.cargo_target())
            .current_dir(&ws.root),
    );
    let mut suite = parse_doctest(&out);
    suite.suite = "doctest".to_owned();
    exit_must_agree(&mut suite, code, "doctest");
    suite
}

fn count_before(line: &str, word: &str) -> u64 {
    line.split(';')
        .find_map(|part| part.trim().strip_suffix(word))
        .and_then(|n| n.trim().rsplit(' ').next())
        .and_then(|n| n.parse().ok())
        .unwrap_or(0)
}

/// Sums every `test result: …` line; `ignored` counts as skipped.
pub fn parse_doctest(out: &str) -> Suite {
    let mut suite = Suite::default();
    for line in out.lines().filter(|l| l.contains("test result:")) {
        suite.passed += count_before(line, "passed");
        suite.failed += count_before(line, "failed");
        suite.skipped += count_before(line, "ignored");
    }
    suite
}

fn merge_summary(path: &Path, suites: &[Suite]) -> Result<(), super::HarnessError> {
    let mut merged: Vec<Suite> = read_json::<Value>(path)
        .ok()
        .and_then(|doc| serde_json::from_value(doc["suites"].clone()).ok())
        .unwrap_or_default();
    for s in suites {
        merged.retain(|m| m.suite != s.suite);
        merged.push(s.clone());
    }
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let ok = merged.iter().all(Suite::green);
    write_json(
        path,
        &json!({"v": 1, "cmd": "run", "ok": ok, "suites": merged}),
    )
}

// ---- mutation gate (test-plan §3 `run` step 4, §10) ----

fn git(repo: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

pub fn commit_exists(repo: &Path, rev: &str) -> bool {
    git(repo, &["cat-file", "-e", &format!("{rev}^{{commit}}")]).is_some()
}

/// `AGENT_RUN_CHUNK_BASE`, else the merge-base with `origin/main`; `None` when neither is a commit.
pub fn resolve_base(repo: &Path, from_env: Option<String>) -> Option<String> {
    let base = from_env
        .map(|b| b.trim().to_owned())
        .filter(|b| !b.is_empty())
        .or_else(|| {
            git(repo, &["merge-base", "HEAD", "origin/main"]).map(|b| b.trim().to_owned())
        })?;
    commit_exists(repo, &base).then_some(base)
}

/// The chunk as the working tree holds it: tracked changes since the merge-base plus every
/// untracked, non-ignored file (equal to `<base>...HEAD` once the chunk is committed).
pub fn chunk_diff(repo: &Path, base: &str) -> Option<String> {
    let merge_base = git(repo, &["merge-base", base, "HEAD"])?;
    let mut diff = git(
        repo,
        &["diff", "--no-color", "--no-ext-diff", merge_base.trim()],
    )?;
    let untracked = git(repo, &["ls-files", "--others", "--exclude-standard", "-z"])?;
    for file in untracked.split('\0').filter(|f| !f.is_empty()) {
        let out = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args([
                "diff",
                "--no-color",
                "--no-ext-diff",
                "--no-index",
                "--",
                "/dev/null",
                file,
            ])
            .stderr(Stdio::null())
            .output()
            .ok()?;
        diff.push_str(&String::from_utf8_lossy(&out.stdout));
    }
    Some(diff)
}

/// Only 0, 2 or 3 leave the verdict to the counts; any other exit tested no mutants.
pub fn mutants_exit_reason(code: Option<i32>) -> Option<String> {
    match code {
        Some(0 | 2 | 3) => None,
        Some(c) => Some(format!("mutants-exit-{c}")),
        None => Some("mutants-exit-signal".to_owned()),
    }
}

fn count(outcomes: &Value, key: &str) -> u64 {
    outcomes[key].as_u64().unwrap_or(0)
}

/// `passed` = caught; `failed` = `survived` = missed + timeout; unviable is reported, never red.
pub fn mutants_suite(outcomes: &Value, failures: Vec<String>) -> (Suite, u64) {
    let survived = count(outcomes, "missed") + count(outcomes, "timeout");
    let tested = count(outcomes, "caught") + survived + count(outcomes, "unviable");
    let suite = Suite {
        suite: "mutants".to_owned(),
        passed: count(outcomes, "caught"),
        failed: survived,
        skipped: 0,
        survived,
        artifact: Some("mutants.out/outcomes.json".to_owned()),
        failures,
    };
    (suite, tested)
}

fn listed(path: &Path) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(str::to_owned)
        .collect()
}

fn mutants(ws: &Workspace, chunk_base: Option<String>) -> Result<(Suite, u64), String> {
    let diff_path = ws.agent_run().join("chunk.diff");
    let _ = fs::remove_file(&diff_path);
    let missing = || "base-missing".to_owned();
    let base = resolve_base(&ws.root, chunk_base).ok_or_else(missing)?;
    let diff = chunk_diff(&ws.root, &base).ok_or_else(missing)?;
    fs::create_dir_all(ws.agent_run()).map_err(|_| missing())?;
    fs::write(&diff_path, &diff).map_err(|_| missing())?;

    // cargo-mutants builds only the packages a diff touches, but the harness tests spawn the root
    // package's `viola` and `viola-fake-agent`: build them (root package only, so the running
    // `viola-harness` is never relinked) and copy `target/` into the scratch tree.
    let (built, _) = run_forwarding(
        Command::new("cargo")
            .args(["build", "--package", "viola", "--features", "fake-agent"])
            .env_remove("CARGO_TARGET_DIR")
            .current_dir(&ws.root),
    );
    if built != Some(0) {
        return Err("build-failed".to_owned());
    }
    let (code, _) = run_forwarding(
        Command::new("cargo")
            .args([
                "mutants",
                "--workspace",
                "--features",
                "fake-agent",
                "--in-diff",
            ])
            .arg(&diff_path)
            .args(["--test-tool=nextest", "--copy-target=true"])
            .env("NEXTEST_PROFILE", "mutants")
            .env_remove("CARGO_TARGET_DIR")
            .current_dir(&ws.root),
    );
    if let Some(reason) = mutants_exit_reason(code) {
        return Err(reason);
    }
    let out_dir = ws.root.join("mutants.out");
    match read_json::<Value>(&out_dir.join("outcomes.json")) {
        Ok(outcomes) => {
            let mut failures = listed(&out_dir.join("missed.txt"));
            failures.extend(listed(&out_dir.join("timeout.txt")));
            Ok(mutants_suite(&outcomes, failures))
        }
        Err(_) if diff.trim().is_empty() => Ok(mutants_suite(&json!({}), Vec::new())),
        Err(_) => Ok((
            Suite {
                failed: 1,
                failures: vec!["outcomes-missing".to_owned()],
                ..Suite::named("mutants")
            },
            0,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_defaults_to_everything() {
        let all = Selection {
            unit: true,
            integration: true,
            mutants: true,
        };
        assert_eq!(Selection::from_flags(false, false, false, false), all);
        assert_eq!(Selection::from_flags(false, false, false, true), all);
        assert_eq!(Selection::from_flags(true, false, true, true), all);
    }

    #[test]
    fn selection_picks_only_the_named_suites() {
        let unit = Selection::from_flags(true, false, false, false);
        assert!(unit.unit && !unit.integration && !unit.mutants);
        let integ = Selection::from_flags(false, true, false, false);
        assert!(!integ.unit && integ.integration && !integ.mutants);
        let muts = Selection::from_flags(false, false, true, false);
        assert!(!muts.unit && !muts.integration && muts.mutants);
    }

    #[test]
    fn parse_junit_counts_pass_fail_skip() {
        let xml = r#"<testsuites><testsuite name="x">
<testcase name="a::ok" classname="viola-core" time="0.1"></testcase>
<testcase name="a::quick" classname="viola-core"/>
<testcase name="a::bad" classname="viola-e2e"><failure message="boom">x</failure></testcase>
<testcase name="a::err" classname="viola-e2e"><error message="e"/></testcase>
<testcase name="a::skip" classname="viola"><skipped/></testcase>
</testsuite></testsuites>"#;
        let s = parse_junit(xml);
        assert_eq!((s.passed, s.failed, s.skipped), (2, 2, 1));
        assert_eq!(s.failures, vec!["viola-e2e a::bad", "viola-e2e a::err"]);
        assert_eq!(parse_junit("<testsuites/>"), Suite::default());
    }

    #[test]
    fn attr_reads_one_attribute_or_nothing() {
        assert_eq!(attr(r#" name="n" classname="c""#, "name"), "n");
        assert_eq!(attr(r#" name="n" classname="c""#, "classname"), "c");
        assert_eq!(attr(r#" name="n""#, "time"), "");
    }

    #[test]
    fn parse_doctest_sums_result_lines() {
        let out = "running 2 tests\n\
test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.1s\n\
test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out\n";
        let s = parse_doctest(out);
        assert_eq!((s.passed, s.failed, s.skipped), (5, 1, 1));
        assert_eq!(parse_doctest("nothing here"), Suite::default());
    }

    #[test]
    fn exit_must_agree_turns_a_silent_red_exit_red() {
        let mut s = Suite::named("nextest-unit");
        exit_must_agree(&mut s, Some(0), "nextest");
        assert!(s.green());
        exit_must_agree(&mut s, Some(4), "nextest");
        assert_eq!(s.failed, 1);
        assert_eq!(s.failures, vec!["nextest-exit-4"]);
        let mut already = Suite {
            failed: 2,
            ..Suite::named("x")
        };
        exit_must_agree(&mut already, Some(100), "nextest");
        assert_eq!(already.failed, 2);
        let mut signalled = Suite::named("x");
        exit_must_agree(&mut signalled, None, "doctest");
        assert_eq!(signalled.failures, vec!["doctest-exit-signal"]);
    }

    #[test]
    fn suite_is_green_only_without_failures_or_survivors() {
        assert!(Suite::named("a").green());
        assert!(
            !Suite {
                failed: 1,
                ..Suite::named("a")
            }
            .green()
        );
        assert!(
            !Suite {
                survived: 1,
                ..Suite::named("a")
            }
            .green()
        );
    }

    #[test]
    fn mutants_exit_codes_map_to_reasons() {
        for pass in [0, 2, 3] {
            assert_eq!(mutants_exit_reason(Some(pass)), None);
        }
        for fail in [1, 4, 5, 6, 70] {
            assert_eq!(
                mutants_exit_reason(Some(fail)),
                Some(format!("mutants-exit-{fail}"))
            );
        }
        assert_eq!(
            mutants_exit_reason(None).as_deref(),
            Some("mutants-exit-signal")
        );
    }

    #[test]
    fn mutants_suite_counts_missed_and_timeout_as_survivors() {
        let outcomes = json!({"caught": 7, "missed": 2, "timeout": 1, "unviable": 3});
        let (s, tested) = mutants_suite(&outcomes, vec!["m".to_owned()]);
        assert_eq!((s.passed, s.failed, s.survived, tested), (7, 3, 3, 13));
        assert!(!s.green());
        let (clean, n) = mutants_suite(&json!({"caught": 4, "unviable": 2}), vec![]);
        assert!(clean.green());
        assert_eq!(n, 6);
        assert_eq!(clean.artifact.as_deref(), Some("mutants.out/outcomes.json"));
    }

    #[test]
    fn merge_summary_replaces_by_suite() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("a").join("run-summary.json");
        merge_summary(
            &path,
            &[Suite::named("nextest-unit"), Suite::named("doctest")],
        )
        .expect("write");
        let red = Suite {
            failed: 1,
            ..Suite::named("doctest")
        };
        merge_summary(&path, &[red]).expect("merge");
        let doc: Value = read_json(&path).expect("read");
        let suites = doc["suites"].as_array().expect("suites");
        assert_eq!(suites.len(), 2);
        assert_eq!(suites[1]["suite"], "doctest");
        assert_eq!(suites[1]["failed"], 1);
        assert_eq!(doc["ok"], false);
    }

    fn git_repo() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path();
        let g = |args: &[&str]| {
            let ok = Command::new("git")
                .arg("-C")
                .arg(dir)
                .args(["-c", "user.name=t", "-c", "user.email=t@example.com"])
                .args(args)
                .output()
                .expect("git")
                .status
                .success();
            assert!(ok, "git {args:?}");
        };
        g(&["init", "-q"]);
        fs::write(dir.join("a.rs"), "fn a() {}\n").expect("write");
        g(&["add", "a.rs"]);
        g(&["commit", "-q", "-m", "base"]);
        tmp
    }

    #[test]
    fn resolve_base_needs_a_real_commit() {
        let repo = git_repo();
        let head = git(repo.path(), &["rev-parse", "HEAD"]).expect("head");
        assert_eq!(
            resolve_base(repo.path(), Some(format!(" {} ", head.trim()))),
            Some(head.trim().to_owned())
        );
        let zeros = "0".repeat(40);
        assert_eq!(resolve_base(repo.path(), Some(zeros)), None);
        assert_eq!(resolve_base(repo.path(), None), None);
        assert_eq!(resolve_base(repo.path(), Some("  ".to_owned())), None);
        assert!(commit_exists(repo.path(), "HEAD"));
    }

    #[test]
    fn chunk_diff_holds_tracked_edits_and_untracked_files() {
        let repo = git_repo();
        fs::write(repo.path().join("a.rs"), "fn a() { let _x = 1; }\n").expect("write");
        fs::write(repo.path().join("b.rs"), "fn b() {}\n").expect("write");
        let diff = chunk_diff(repo.path(), "HEAD").expect("diff");
        assert!(diff.contains("+fn a() { let _x = 1; }"));
        assert!(diff.contains("+fn b() {}"));
        assert!(diff.contains("b.rs"));
        assert_eq!(chunk_diff(repo.path(), &"0".repeat(40)), None);
    }

    const GOOD_LIB: &str = "/// ```\n/// assert_eq!(viola::two(), 2);\n/// ```\n\
        pub fn two() -> u32 { 2 }\n\
        #[cfg(test)]\nmod tests {\n    #[test]\n    fn two_is_two() { assert_eq!(super::two(), 2); }\n}\n";

    /// A throwaway one-crate workspace in its own git repo: never this workspace, never nested in it.
    fn mini(lib: &str) -> (tempfile::TempDir, Workspace) {
        let tmp = git_repo();
        let root = tmp.path();
        fs::create_dir_all(root.join("src")).expect("mkdir");
        fs::create_dir_all(root.join("tests")).expect("mkdir");
        fs::create_dir_all(root.join(".config")).expect("mkdir");
        fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = \"viola\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n\
             [features]\nfake-agent = []\n\n[workspace]\n",
        )
        .expect("write");
        fs::write(root.join("src").join("lib.rs"), lib).expect("write");
        fs::write(
            root.join("tests").join("it.rs"),
            "#[test]\nfn it_passes() {}\n",
        )
        .expect("write");
        fs::write(
            root.join(".config").join("nextest.toml"),
            "[profile.ci]\nfail-fast = false\n\n[profile.ci.junit]\npath = \"junit.xml\"\n\n\
             [profile.mutants]\nfail-fast = true\n",
        )
        .expect("write");
        fs::write(
            root.join(".gitignore"),
            "target/\nmutants.out*/\nCargo.lock\n",
        )
        .expect("write");
        let ok = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["-c", "user.name=t", "-c", "user.email=t@example.com"])
            .args(["add", "-A"])
            .status()
            .expect("git add")
            .success()
            && Command::new("git")
                .arg("-C")
                .arg(root)
                .args(["-c", "user.name=t", "-c", "user.email=t@example.com"])
                .args(["commit", "-q", "-m", "mini"])
                .status()
                .expect("git commit")
                .success();
        assert!(ok);
        let ws = Workspace {
            root: root.to_path_buf(),
        };
        (tmp, ws)
    }

    fn head(ws: &Workspace) -> String {
        git(&ws.root, &["rev-parse", "HEAD"])
            .expect("head")
            .trim()
            .to_owned()
    }

    fn suite<'a>(doc: &'a Value, name: &str) -> &'a Value {
        doc["suites"]
            .as_array()
            .and_then(|s| s.iter().find(|x| x["suite"] == name))
            .unwrap_or_else(|| panic!("suite {name} in {doc}"))
    }

    #[test]
    fn run_reports_unit_integration_and_doctest_suites() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let sel = Selection::from_flags(true, true, false, false);
        let out = run(&ws, sel, None, None);
        assert_eq!(out.code, 0, "{}", out.doc);
        assert_eq!(suite(&out.doc, "nextest-unit")["passed"], 1);
        assert_eq!(suite(&out.doc, "nextest-integration")["passed"], 1);
        assert_eq!(suite(&out.doc, "doctest")["passed"], 1);
        assert!(ws.artifacts().join("junit-nextest-unit.xml").is_file());
        let summary: Value = read_json(&ws.artifacts().join("run-summary.json")).expect("summary");
        assert_eq!(summary["suites"].as_array().map(Vec::len), Some(3));
        assert!(out.doc.get("mutants").is_none());
    }

    #[test]
    fn run_with_a_failing_test_is_red_and_names_it() {
        let lib = "pub fn two() -> u32 { 3 }\n#[cfg(test)]\nmod tests {\n    #[test]\n    \
                   fn two_is_two() { assert_eq!(super::two(), 2); }\n}\n";
        let (_tmp, ws) = mini(lib);
        let out = run(
            &ws,
            Selection::from_flags(true, false, false, false),
            None,
            None,
        );
        assert_eq!(out.code, 1);
        let unit = suite(&out.doc, "nextest-unit");
        assert_eq!(unit["failed"], 1);
        assert!(
            unit["failures"][0]
                .as_str()
                .is_some_and(|f| f.contains("two_is_two"))
        );
    }

    #[test]
    fn run_with_a_filter_matching_nothing_is_red() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let out = run(
            &ws,
            Selection::from_flags(false, true, false, false),
            Some("test(=no_such_test)"),
            None,
        );
        assert_eq!(out.code, 1);
        let integ = suite(&out.doc, "nextest-integration");
        assert_eq!(integ["failed"], 1);
        assert_eq!(integ["passed"], 0);
    }

    #[test]
    fn run_without_a_workspace_is_artifact_missing() {
        let empty = tempfile::tempdir().expect("tempdir");
        let ws = Workspace {
            root: empty.path().to_path_buf(),
        };
        let out = run(
            &ws,
            Selection::from_flags(true, false, false, false),
            None,
            None,
        );
        assert_eq!(out.code, 1);
        assert_eq!(
            suite(&out.doc, "nextest-unit")["failures"][0],
            "artifact-missing"
        );
        assert_eq!(suite(&out.doc, "doctest")["failed"], 1);
    }

    #[test]
    fn run_mutants_with_an_unreachable_base_is_base_missing_and_writes_no_diff() {
        let (_tmp, ws) = mini(GOOD_LIB);
        fs::create_dir_all(ws.agent_run()).expect("mkdir");
        fs::write(ws.agent_run().join("chunk.diff"), "stale").expect("write");
        let out = run(
            &ws,
            Selection::from_flags(false, false, true, false),
            None,
            Some("0".repeat(40)),
        );
        assert_eq!(out.code, 1);
        assert_eq!(out.doc["reason"], "base-missing");
        assert!(!ws.agent_run().join("chunk.diff").exists());
    }

    #[test]
    fn run_mutants_with_an_unbuildable_root_package_is_build_failed() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        fs::write(ws.root.join("src").join("lib.rs"), "pub fn broken( {}\n").expect("write");
        let out = run(
            &ws,
            Selection::from_flags(false, false, true, false),
            None,
            Some(base),
        );
        assert_eq!(out.code, 1);
        assert_eq!(out.doc["reason"], "build-failed");
    }

    #[test]
    fn run_mutants_reports_survivors_of_an_untested_change() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        let lib = format!("{GOOD_LIB}pub fn three() -> u32 {{ 3 }}\n");
        fs::write(ws.root.join("src").join("lib.rs"), lib).expect("write");
        let out = run(
            &ws,
            Selection::from_flags(false, false, true, false),
            None,
            Some(base),
        );
        assert_eq!(out.code, 1, "{}", out.doc);
        let m = suite(&out.doc, "mutants");
        assert!(m["survived"].as_u64().is_some_and(|n| n >= 1));
        assert!(
            m["failures"][0]
                .as_str()
                .is_some_and(|f| f.contains("three"))
        );
        assert!(
            out.doc["mutants"]["tested"]
                .as_u64()
                .is_some_and(|n| n >= 1)
        );
        assert!(ws.agent_run().join("chunk.diff").is_file());
    }

    #[test]
    fn run_mutants_passes_when_the_change_is_tested() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        let lib = format!(
            "{GOOD_LIB}pub fn three() -> u32 {{ 3 }}\n#[cfg(test)]\nmod t3 {{\n    #[test]\n    \
             fn three_is_three() {{ assert_eq!(super::three(), 3); }}\n}}\n"
        );
        fs::write(ws.root.join("src").join("lib.rs"), lib).expect("write");
        let out = run(
            &ws,
            Selection::from_flags(false, false, true, false),
            None,
            Some(base),
        );
        assert_eq!(out.code, 0, "{}", out.doc);
        let m = suite(&out.doc, "mutants");
        assert_eq!(m["survived"], 0);
        assert!(m["passed"].as_u64().is_some_and(|n| n >= 1));
    }

    #[test]
    fn listed_reads_non_empty_lines() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("missed.txt");
        fs::write(&path, "src/a.rs:1: x\n\n  \nsrc/b.rs:2: y\n").expect("write");
        assert_eq!(listed(&path), vec!["src/a.rs:1: x", "src/b.rs:2: y"]);
        assert!(listed(&tmp.path().join("none.txt")).is_empty());
    }
}
