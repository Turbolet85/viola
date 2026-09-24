//! `run`: the built suites (unit · integration · doctest · coverage · fuzz-replay) and the per-chunk
//! mutation gate, one JSON summary, merged by suite into
//! `target/agent-run/artifacts/run-summary.json` (test-plan §3 `run`).

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::{Outcome, Workspace, read_json, write_json};

const UNIT: &str = "kind(lib) | kind(bin)";
// test-plan §3 also excludes the E2E binaries (`path_`, `tui_`, …); nextest rejects a `binary()`
// operator that matches no binary, so that exclusion arrives with the first E2E binary and `--e2e`.
const INTEGRATION: &str = "kind(test)";

/// test-plan §10's excluded trees, with a separator class: llvm-cov reports Windows paths with `\`.
pub const COVERAGE_IGNORE: &str =
    r"(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])";

/// test-plan §10 Comprehensive: line, function and region floors, per OS.
pub const COVERAGE_FLOORS: [(&str, f64); 3] =
    [("lines", 85.0), ("functions", 95.0), ("regions", 80.0)];

/// Runs one tool invocation: its exit code and its stdout.
pub type Runner<'r> = dyn FnMut(&mut Command) -> (Option<i32>, String) + 'r;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Selection {
    pub unit: bool,
    pub integration: bool,
    pub mutants: bool,
    pub coverage: bool,
    pub fuzz_replay: bool,
}

impl Selection {
    /// No selector, or `--all`, selects unit, integration and mutants. `--coverage` replaces the
    /// nextest runs and `--fuzz-replay` is Linux-only, so neither joins the default.
    pub fn from_flags(named: Selection, all: bool) -> Self {
        let none = named == Selection::default();
        Self {
            unit: named.unit || all || none,
            integration: named.integration || all || none,
            mutants: named.mutants || all || none,
            ..named
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
    run_with(ws, sel, filter, chunk_base, None, &mut run_forwarding)
}

/// A tool arm that tested nothing: the document's `reason` and an optional `detail`.
#[derive(Debug, PartialEq, Eq)]
pub struct Refusal {
    pub reason: String,
    pub detail: Option<String>,
}

impl Refusal {
    fn new(reason: &str, detail: Option<&str>) -> Self {
        Self {
            reason: reason.to_owned(),
            detail: detail.map(str::to_owned),
        }
    }
}

/// Cargo-fuzz's libFuzzer runs on the Linux runner only (test-plan §3 `--fuzz-replay`).
pub fn fuzz_host_supported() -> bool {
    cfg!(target_os = "linux")
}

/// `leg` names a mutation leg whose verdict the `gate` union decides (`--mutants --leg`).
pub fn run_with(
    ws: &Workspace,
    sel: Selection,
    filter: Option<&str>,
    chunk_base: Option<String>,
    leg: Option<&str>,
    runner: &mut Runner<'_>,
) -> Outcome {
    if sel.fuzz_replay && !fuzz_host_supported() {
        let doc = json!({"v": 1, "cmd": "run", "ok": false, "reason": "fuzz-linux-only"});
        return Outcome { doc, code: 2 };
    }
    let mut suites = Vec::new();
    if sel.coverage {
        suites.push(coverage(ws, filter, runner));
    } else {
        if sel.unit {
            suites.push(nextest(ws, "nextest-unit", UNIT, filter, runner));
        }
        if sel.integration {
            suites.push(nextest(
                ws,
                "nextest-integration",
                INTEGRATION,
                filter,
                runner,
            ));
        }
    }
    if sel.unit || sel.integration || sel.coverage {
        suites.push(doctest(ws, runner));
    }
    let mut refusal = None;
    if sel.fuzz_replay {
        match fuzz_replay(ws, runner) {
            Ok(suite) => suites.push(suite),
            Err(r) => refusal = Some(r),
        }
    }
    let mut mutants_doc = None;
    if sel.mutants && refusal.is_none() {
        match mutants(ws, chunk_base, leg, runner) {
            Ok((suite, doc)) => {
                suites.push(suite);
                mutants_doc = Some(doc);
            }
            Err(reason) => refusal = Some(Refusal::new(&reason, None)),
        }
    }
    let deferred = |s: &Suite| leg.is_some() && s.suite == "mutants" && s.failed == s.survived;
    let ok = refusal.is_none() && suites.iter().all(|s| s.green() || deferred(s));
    let _ = merge_summary(&ws.artifacts().join("run-summary.json"), &suites);
    let mut doc = json!({"v": 1, "cmd": "run", "ok": ok});
    if let Some(refusal) = refusal {
        doc["reason"] = json!(refusal.reason);
        if let Some(detail) = refusal.detail {
            doc["detail"] = json!(detail);
        }
    }
    doc["suites"] = json!(suites);
    if let Some(mutants_doc) = mutants_doc {
        doc["mutants"] = mutants_doc;
    }
    Outcome::new(doc, ok)
}

/// Runs a cargo tool with its stdout moved to our stderr: the harness's stdout carries only its
/// document.
pub fn run_forwarding(cmd: &mut Command) -> (Option<i32>, String) {
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

/// nextest keeps its store under `<workspace root>/target/nextest` whatever the target dir, and
/// under `cargo llvm-cov nextest` too (measured: llvm-cov 0.9.1, nextest 0.9.133).
fn junit_source(ws: &Workspace) -> PathBuf {
    ws.root
        .join("target")
        .join("nextest")
        .join("ci")
        .join("junit.xml")
}

/// The suite a finished JUnit-writing run reports; the report is copied to `junit-<suite>.xml`.
fn junit_suite(ws: &Workspace, suite: &str, code: Option<i32>, tool: &str) -> Suite {
    let artifact = ws.artifacts().join(format!("junit-{suite}.xml"));
    let Ok(xml) = fs::read_to_string(junit_source(ws)) else {
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
    exit_must_agree(&mut result, code, tool);
    result
}

fn nextest(
    ws: &Workspace,
    suite: &str,
    layer: &str,
    filter: Option<&str>,
    runner: &mut Runner<'_>,
) -> Suite {
    let _ = fs::remove_file(junit_source(ws));
    let expr = match filter {
        Some(f) => format!("({layer}) & ({f})"),
        None => layer.to_owned(),
    };
    let (code, _) = runner(
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
    junit_suite(ws, suite, code, "nextest")
}

/// The instrumented run that replaces the unit and integration nextest runs (test-plan §3
/// `--coverage`). llvm-cov builds in its own target dir, so the running harness is never relinked.
fn coverage(ws: &Workspace, filter: Option<&str>, runner: &mut Runner<'_>) -> Suite {
    let summary = ws.artifacts().join("llvm-cov-summary.json");
    let _ = fs::remove_file(junit_source(ws));
    let _ = fs::remove_file(&summary);
    let mut cmd = Command::new("cargo");
    cmd.args([
        "llvm-cov",
        "nextest",
        "--workspace",
        "--features",
        "viola/fake-agent",
        "--profile",
        "ci",
        "--lcov",
        "--output-path",
        "target/lcov.info",
        "--ignore-filename-regex",
        COVERAGE_IGNORE,
    ]);
    for (metric, floor) in COVERAGE_FLOORS {
        cmd.arg(format!("--fail-under-{metric}"))
            .arg(floor.to_string());
    }
    if let Some(f) = filter {
        cmd.args(["-E", f]);
    }
    let (code, _) = runner(cmd.current_dir(&ws.root));
    let mut suite = junit_suite(ws, "coverage", code, "llvm-cov");
    if suite.failures.iter().any(|f| f == "artifact-missing") {
        return suite;
    }
    let _ = fs::create_dir_all(ws.artifacts());
    let _ = runner(
        Command::new("cargo")
            .args(["llvm-cov", "report", "--json", "--summary-only"])
            .args(["--ignore-filename-regex", COVERAGE_IGNORE, "--output-path"])
            .arg(&summary)
            .current_dir(&ws.root),
    );
    if !summary.is_file() {
        suite.failed += 1;
        suite.failures.push("llvm-cov-summary-missing".to_owned());
    }
    suite
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

fn doctest(ws: &Workspace, runner: &mut Runner<'_>) -> Suite {
    let (code, out) = runner(
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

// ---- fuzz corpus replay (test-plan §3 `--fuzz-replay`) ----

/// The `channel` of `fuzz/rust-toolchain.toml`: the one source of the fuzz toolchain.
pub fn fuzz_channel(toml: &str) -> Option<String> {
    toml.lines()
        .filter_map(|l| l.trim().strip_prefix("channel"))
        .filter_map(|rest| rest.trim_start().strip_prefix('='))
        .map(|v| v.trim().trim_matches('"').to_owned())
        .find(|v| !v.is_empty())
}

/// Every `fuzz/fuzz_targets/<target>.rs`, by name, sorted.
fn fuzz_targets(fuzz: &Path) -> Vec<String> {
    let mut targets: Vec<String> = fs::read_dir(fuzz.join("fuzz_targets"))
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "rs"))
        .filter_map(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .collect();
    targets.sort();
    targets
}

fn has_entries(dir: &Path) -> bool {
    fs::read_dir(dir).is_ok_and(|mut entries| entries.next().is_some())
}

/// Replays each target's committed corpus once (`-runs=0`); an absent or empty corpus fails its
/// target, never passes it.
fn fuzz_replay(ws: &Workspace, runner: &mut Runner<'_>) -> Result<Suite, Refusal> {
    let (probe, _) = runner(
        Command::new("cargo")
            .args(["fuzz", "--version"])
            .current_dir(&ws.root),
    );
    if probe != Some(0) {
        return Err(Refusal::new("tool-missing", Some("cargo-fuzz")));
    }
    let fuzz = ws.root.join("fuzz");
    let channel = fs::read_to_string(fuzz.join("rust-toolchain.toml"))
        .ok()
        .and_then(|t| fuzz_channel(&t))
        .ok_or_else(|| Refusal::new("tool-missing", Some("fuzz/rust-toolchain.toml")))?;
    let targets = fuzz_targets(&fuzz);
    if targets.is_empty() {
        return Err(Refusal::new("corpus-empty", None));
    }
    let mut suite = Suite {
        artifact: Some("fuzz/corpus".to_owned()),
        ..Suite::named("fuzz-replay")
    };
    for target in targets {
        let corpus = fuzz.join("corpus").join(&target);
        if !has_entries(&corpus) {
            suite.failed += 1;
            suite.failures.push(format!("{target}: corpus-empty"));
            continue;
        }
        let (code, _) = runner(
            Command::new("cargo")
                .arg(format!("+{channel}"))
                .args(["fuzz", "run", "--fuzz-dir", "fuzz", &target])
                .arg(format!("fuzz/corpus/{target}"))
                .args(["--", "-runs=0"])
                .current_dir(&ws.root),
        );
        if code == Some(0) {
            suite.passed += 1;
        } else {
            suite.failed += 1;
            suite.failures.push(target);
        }
    }
    Ok(suite)
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

/// Every path a `diff --git a/<old> b/<new>` header names, both sides (a rename away from `.rs`
/// still removes Rust source).
pub fn diff_paths(diff: &str) -> Vec<&str> {
    diff.lines()
        .filter_map(|l| l.strip_prefix("diff --git a/"))
        .filter_map(|rest| rest.rsplit_once(" b/"))
        .flat_map(|(old, new)| [old, new])
        .collect()
}

/// The number of files the diff changes (one `diff --git` header each).
pub fn diff_files(diff: &str) -> usize {
    diff.lines()
        .filter(|l| l.starts_with("diff --git "))
        .count()
}

pub fn rust_delta(diff: &str) -> bool {
    diff_paths(diff).iter().any(|p| p.ends_with(".rs"))
}

/// A diff with no Rust source never reaches cargo-mutants: its silent exit 0 writes no
/// `outcomes.json`, and an earlier run's file would otherwise be read as this one's.
fn no_rust_delta(diff: &str) -> (Suite, Value) {
    let suite = Suite {
        artifact: Some("target/agent-run/chunk.diff".to_owned()),
        ..Suite::named("mutants")
    };
    let doc = json!({
        "tested": 0,
        "verdict": "no-rust-delta",
        "diff": "target/agent-run/chunk.diff",
        "files": diff_files(diff),
    });
    (suite, doc)
}

/// One outcome as the `gate` union reads it; `None` for the baseline and anything else unscored.
fn leg_outcome(summary: &str) -> Option<&'static str> {
    match summary {
        "CaughtMutant" => Some("caught"),
        "MissedMutant" => Some("missed"),
        "Timeout" => Some("timeout"),
        "Unviable" => Some("unviable"),
        _ => None,
    }
}

/// The reduced per-leg verdict: each mutant's name and outcome, never an argv, a log path or test
/// output (`outcomes.json` carries all three).
pub fn leg_verdict(leg: &str, verdict: &str, outcomes: &Value) -> Value {
    let mutants: Vec<Value> = outcomes["outcomes"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|o| {
            let name = o["scenario"]["Mutant"]["name"].as_str()?;
            let outcome = leg_outcome(o["summary"].as_str()?)?;
            Some(json!({"name": name, "outcome": outcome}))
        })
        .collect();
    json!({"v": 1, "leg": leg, "verdict": verdict, "mutants": mutants})
}

pub fn leg_verdict_path(artifacts: &Path, leg: &str) -> PathBuf {
    artifacts.join(format!("mutants-verdict-{leg}.json"))
}

fn write_leg_verdict(ws: &Workspace, leg: Option<&str>, verdict: &str, outcomes: &Value) {
    if let Some(leg) = leg {
        let _ = fs::create_dir_all(ws.artifacts());
        let _ = write_json(
            &leg_verdict_path(&ws.artifacts(), leg),
            &leg_verdict(leg, verdict, outcomes),
        );
    }
}

fn mutants(
    ws: &Workspace,
    chunk_base: Option<String>,
    leg: Option<&str>,
    runner: &mut Runner<'_>,
) -> Result<(Suite, Value), String> {
    let diff_path = ws.agent_run().join("chunk.diff");
    let _ = fs::remove_file(&diff_path);
    if let Some(leg) = leg {
        let _ = fs::remove_file(leg_verdict_path(&ws.artifacts(), leg));
    }
    let missing = || "base-missing".to_owned();
    let base = resolve_base(&ws.root, chunk_base).ok_or_else(missing)?;
    let diff = chunk_diff(&ws.root, &base).ok_or_else(missing)?;
    fs::create_dir_all(ws.agent_run()).map_err(|_| missing())?;
    fs::write(&diff_path, &diff).map_err(|_| missing())?;
    if !rust_delta(&diff) {
        write_leg_verdict(ws, leg, "no-rust-delta", &Value::Null);
        let (suite, mut doc) = no_rust_delta(&diff);
        if let Some(leg) = leg {
            doc["leg"] = json!(leg);
        }
        return Ok((suite, doc));
    }
    let out_dir = ws.root.join("mutants.out");
    let _ = fs::remove_file(out_dir.join("outcomes.json"));

    // cargo-mutants builds only the packages a diff touches, but the harness tests spawn the root
    // package's `viola` and `viola-fake-agent`: build them (root package only, so the running
    // `viola-harness` is never relinked) and copy `target/` into the scratch tree.
    let (built, _) = runner(
        Command::new("cargo")
            .args(["build", "--package", "viola", "--features", "fake-agent"])
            .env_remove("CARGO_TARGET_DIR")
            .current_dir(&ws.root),
    );
    if built != Some(0) {
        return Err("build-failed".to_owned());
    }
    let (code, _) = runner(
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
    let (suite, tested) = match read_json::<Value>(&out_dir.join("outcomes.json")) {
        Ok(outcomes) => {
            write_leg_verdict(ws, leg, "counted", &outcomes);
            let mut failures = listed(&out_dir.join("missed.txt"));
            failures.extend(listed(&out_dir.join("timeout.txt")));
            mutants_suite(&outcomes, failures)
        }
        Err(_) => (
            Suite {
                failed: 1,
                failures: vec!["outcomes-missing".to_owned()],
                ..Suite::named("mutants")
            },
            0,
        ),
    };
    let mut doc = json!({"tested": tested, "verdict": "counted"});
    if let Some(leg) = leg {
        doc["leg"] = json!(leg);
    }
    Ok((suite, doc))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flags(unit: bool, integration: bool, mutants: bool, all: bool) -> Selection {
        let named = Selection {
            unit,
            integration,
            mutants,
            ..Selection::default()
        };
        Selection::from_flags(named, all)
    }

    #[test]
    fn selection_defaults_to_unit_integration_and_mutants() {
        let all = Selection {
            unit: true,
            integration: true,
            mutants: true,
            ..Selection::default()
        };
        assert_eq!(flags(false, false, false, false), all);
        assert_eq!(flags(false, false, false, true), all);
        assert_eq!(flags(true, false, true, true), all);
    }

    #[test]
    fn selection_picks_only_the_named_suites() {
        let unit = flags(true, false, false, false);
        assert!(unit.unit && !unit.integration && !unit.mutants);
        let integ = flags(false, true, false, false);
        assert!(!integ.unit && integ.integration && !integ.mutants);
        let muts = flags(false, false, true, false);
        assert!(!muts.unit && !muts.integration && muts.mutants);
    }

    #[test]
    fn selection_coverage_or_fuzz_alone_selects_nothing_else() {
        for named in [
            Selection {
                coverage: true,
                ..Selection::default()
            },
            Selection {
                fuzz_replay: true,
                ..Selection::default()
            },
        ] {
            assert_eq!(Selection::from_flags(named, false), named);
        }
        let with_all = Selection::from_flags(
            Selection {
                coverage: true,
                ..Selection::default()
            },
            true,
        );
        assert!(with_all.unit && with_all.integration && with_all.mutants && with_all.coverage);
        assert!(!with_all.fuzz_replay);
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
        let sel = flags(true, true, false, false);
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
        let out = run(&ws, flags(true, false, false, false), None, None);
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
            flags(false, true, false, false),
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
        let out = run(&ws, flags(true, false, false, false), None, None);
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
            flags(false, false, true, false),
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
        let out = run(&ws, flags(false, false, true, false), None, Some(base));
        assert_eq!(out.code, 1);
        assert_eq!(out.doc["reason"], "build-failed");
    }

    #[test]
    fn run_mutants_reports_survivors_of_an_untested_change() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        let lib = format!("{GOOD_LIB}pub fn three() -> u32 {{ 3 }}\n");
        fs::write(ws.root.join("src").join("lib.rs"), lib).expect("write");
        let out = run(&ws, flags(false, false, true, false), None, Some(base));
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
        let out = run(&ws, flags(false, false, true, false), None, Some(base));
        assert_eq!(out.code, 0, "{}", out.doc);
        let m = suite(&out.doc, "mutants");
        assert_eq!(m["survived"], 0);
        assert!(m["passed"].as_u64().is_some_and(|n| n >= 1));
        assert_eq!(out.doc["mutants"]["verdict"], "counted");
    }

    const RUST_DIFF: &str =
        "diff --git a/src/lib.rs b/src/lib.rs\n--- a/src/lib.rs\n+++ b/src/lib.rs\n";
    const DOCS_DIFF: &str = "diff --git a/README.md b/README.md\n--- a/README.md\n+++ b/README.md\n\
        diff --git a/docs/x y.md b/docs/x y.md\n";
    const RENAME_DIFF: &str =
        "diff --git a/src/old.rs b/notes/old.md\nsimilarity index 100%\nrename from src/old.rs\n";

    #[test]
    fn rust_delta_classifies_diff_headers() {
        assert!(rust_delta(RUST_DIFF));
        assert!(!rust_delta(DOCS_DIFF));
        assert!(!rust_delta(""));
        assert!(rust_delta(RENAME_DIFF));
        assert!(!rust_delta("+++ b/src/lib.rs\n--- a/src/lib.rs\n"));
        assert_eq!(
            diff_paths(DOCS_DIFF),
            vec!["README.md", "README.md", "docs/x y.md", "docs/x y.md"]
        );
        assert_eq!(diff_paths(RENAME_DIFF), vec!["src/old.rs", "notes/old.md"]);
        assert_eq!(diff_files(DOCS_DIFF), 2);
        assert_eq!(diff_files(RUST_DIFF), 1);
        assert_eq!(diff_files(""), 0);
    }

    fn plant_stale_outcomes(ws: &Workspace) {
        let out = ws.root.join("mutants.out");
        fs::create_dir_all(&out).expect("mkdir");
        fs::write(
            out.join("outcomes.json"),
            r#"{"caught": 777, "missed": 5, "timeout": 0, "unviable": 0}"#,
        )
        .expect("write");
    }

    #[test]
    fn run_mutants_no_rust_delta_passes_by_name_and_never_reads_stale_outcomes() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        plant_stale_outcomes(&ws);
        fs::write(ws.root.join("README.md"), "docs only\n").expect("write");
        let out = run(&ws, flags(false, false, true, false), None, Some(base));
        assert_eq!(out.code, 0, "{}", out.doc);
        assert_eq!(
            out.doc["mutants"],
            json!({"tested": 0, "verdict": "no-rust-delta", "diff": "target/agent-run/chunk.diff", "files": 1})
        );
        let m = suite(&out.doc, "mutants");
        assert_eq!(
            (m["passed"].as_u64(), m["survived"].as_u64()),
            (Some(0), Some(0))
        );
        assert_eq!(m["artifact"], "target/agent-run/chunk.diff");
        assert!(!out.doc.to_string().contains("777"));
        assert!(ws.agent_run().join("chunk.diff").is_file());
    }

    #[test]
    fn run_mutants_rust_delta_without_fresh_outcomes_is_outcomes_missing() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        plant_stale_outcomes(&ws);
        // A Rust file no package builds: a Rust delta by path, yet cargo-mutants finds no source.
        fs::create_dir_all(ws.root.join("scripts")).expect("mkdir");
        fs::write(ws.root.join("scripts").join("tool.rs"), "fn main() {}\n").expect("write");
        let out = run(&ws, flags(false, false, true, false), None, Some(base));
        assert_eq!(out.code, 1, "{}", out.doc);
        let m = suite(&out.doc, "mutants");
        assert_eq!(m["failures"][0], "outcomes-missing");
        assert_eq!(
            out.doc["mutants"],
            json!({"tested": 0, "verdict": "counted"})
        );
        assert!(!out.doc.to_string().contains("777"));
    }

    // ---- the tool seam: coverage, fuzz replay and the mutants leg against a stand-in runner ----

    type Calls = Vec<Vec<String>>;

    fn args_of(cmd: &Command) -> Vec<String> {
        cmd.get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect()
    }

    fn has(call: &[String], words: &[&str]) -> bool {
        call.windows(words.len())
            .any(|w| w.iter().zip(words).all(|(a, b)| a == b))
    }

    const GREEN_JUNIT: &str = "<testsuites><testcase name=\"a\" classname=\"c\"/>\
        <testcase name=\"b\" classname=\"c\"/></testsuites>";
    const DOCTEST_OK: &str = "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured\n";

    fn scratch() -> (tempfile::TempDir, Workspace) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ws = Workspace {
            root: tmp.path().to_path_buf(),
        };
        (tmp, ws)
    }

    /// The stand-in for `cargo llvm-cov` + doctest: `junit` and `summary` say what it writes,
    /// `code` is the instrumented run's exit.
    fn run_coverage(
        ws: &Workspace,
        sel: Selection,
        filter: Option<&str>,
        junit: bool,
        summary: bool,
        code: i32,
    ) -> (Outcome, Calls) {
        let mut calls = Vec::new();
        let root = ws.root.clone();
        let out = {
            let mut runner = |cmd: &mut Command| {
                let args = args_of(cmd);
                let answer = if has(&args, &["llvm-cov", "nextest"]) {
                    if junit {
                        let path = root.join("target/nextest/ci/junit.xml");
                        fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
                        fs::write(path, GREEN_JUNIT).expect("junit");
                    }
                    (Some(code), String::new())
                } else if has(&args, &["llvm-cov", "report"]) {
                    if summary {
                        fs::write(args.last().expect("output path"), "{}").expect("summary");
                    }
                    (Some(0), String::new())
                } else {
                    (Some(0), DOCTEST_OK.to_owned())
                };
                calls.push(args);
                answer
            };
            run_with(ws, sel, filter, None, None, &mut runner)
        };
        (out, calls)
    }

    /// `run --coverage` as the CI step calls it: no other selector.
    fn coverage_only() -> Selection {
        Selection::from_flags(
            Selection {
                coverage: true,
                ..Selection::default()
            },
            false,
        )
    }

    #[test]
    fn coverage_alone_still_runs_the_doctests() {
        let (_tmp, ws) = scratch();
        let (out, calls) = run_coverage(&ws, coverage_only(), None, true, true, 0);
        assert_eq!(out.code, 0, "{}", out.doc);
        assert_eq!(suite(&out.doc, "doctest")["passed"], 1);
        assert!(
            calls
                .iter()
                .any(|c| has(c, &["test", "--workspace", "--doc"]))
        );
    }

    #[test]
    fn coverage_replaces_the_nextest_runs_with_one_instrumented_run() {
        let (_tmp, ws) = scratch();
        let with_nextest_layers = Selection {
            unit: true,
            integration: true,
            coverage: true,
            ..Selection::default()
        };
        let (out, calls) = run_coverage(&ws, with_nextest_layers, None, true, true, 0);
        assert_eq!(out.code, 0, "{}", out.doc);
        assert_eq!(suite(&out.doc, "coverage")["passed"], 2);
        assert_eq!(suite(&out.doc, "doctest")["passed"], 1);
        assert_eq!(out.doc["suites"].as_array().map(Vec::len), Some(2));
        assert!(ws.artifacts().join("junit-coverage.xml").is_file());
        assert!(ws.artifacts().join("llvm-cov-summary.json").is_file());
        assert!(
            !calls
                .iter()
                .any(|c| c.first().is_some_and(|a| a == "nextest"))
        );
        let instrumented = &calls[0];
        for words in [
            &["--features", "viola/fake-agent"][..],
            &["--profile", "ci"],
            &["--ignore-filename-regex", COVERAGE_IGNORE],
            &["--fail-under-lines", "85"],
            &["--fail-under-functions", "95"],
            &["--fail-under-regions", "80"],
            &["--output-path", "target/lcov.info"],
        ] {
            assert!(has(instrumented, words), "{words:?} in {instrumented:?}");
        }
        assert!(!instrumented.contains(&"-E".to_owned()));
        assert!(has(
            &calls[1],
            &["--ignore-filename-regex", COVERAGE_IGNORE]
        ));
    }

    #[test]
    fn coverage_passes_the_filter_as_its_expression() {
        let (_tmp, ws) = scratch();
        let (_, calls) = run_coverage(&ws, coverage_only(), Some("test(x)"), true, true, 0);
        assert!(has(&calls[0], &["-E", "test(x)"]));
    }

    #[test]
    fn coverage_ignore_regex_matches_either_separator() {
        let excluded = [
            "D:\\v\\crates\\viola-e2e\\src\\a.rs",
            "/v/crates/viola-e2e/src/a.rs",
            "/v/tests/support/home.rs",
            "D:\\v\\tests\\support\\home.rs",
            "/v/fuzz/fuzz_targets/x.rs",
            "D:\\v\\src\\bin\\viola-fake-agent.rs",
        ];
        let body = COVERAGE_IGNORE
            .trim_start_matches('(')
            .trim_end_matches(')');
        let alternatives: Vec<String> = body.split('|').map(|a| a.replace(r"[/\\]", "/")).collect();
        for path in excluded {
            let normal = path.replace('\\', "/");
            assert!(
                alternatives.iter().any(|a| normal.contains(a.as_str())),
                "{path}"
            );
        }
        assert!(
            !alternatives
                .iter()
                .any(|a| "/v/src/main.rs".contains(a.as_str()))
        );
    }

    #[test]
    fn coverage_without_junit_is_artifact_missing_and_skips_the_report() {
        let (_tmp, ws) = scratch();
        let (out, calls) = run_coverage(&ws, coverage_only(), None, false, true, 1);
        assert_eq!(out.code, 1);
        assert_eq!(
            suite(&out.doc, "coverage")["failures"][0],
            "artifact-missing"
        );
        assert!(!calls.iter().any(|c| has(c, &["llvm-cov", "report"])));
    }

    #[test]
    fn coverage_without_a_fresh_summary_is_red() {
        let (_tmp, ws) = scratch();
        fs::create_dir_all(ws.artifacts()).expect("mkdir");
        fs::write(ws.artifacts().join("llvm-cov-summary.json"), "stale").expect("stale");
        let (out, _) = run_coverage(&ws, coverage_only(), None, true, false, 0);
        assert_eq!(out.code, 1);
        let cov = suite(&out.doc, "coverage");
        assert_eq!(cov["failed"], 1);
        assert_eq!(cov["failures"][0], "llvm-cov-summary-missing");
    }

    #[test]
    fn coverage_red_exit_over_green_tests_is_red() {
        let (_tmp, ws) = scratch();
        let (out, _) = run_coverage(&ws, coverage_only(), None, true, true, 1);
        assert_eq!(out.code, 1);
        assert_eq!(
            suite(&out.doc, "coverage")["failures"][0],
            "llvm-cov-exit-1"
        );
    }

    #[test]
    fn fuzz_channel_reads_the_toolchain_file() {
        let toml = "[toolchain]\nchannel = \"nightly-2026-09-20\"\ncomponents = [\"rust-src\"]\n";
        assert_eq!(fuzz_channel(toml).as_deref(), Some("nightly-2026-09-20"));
        assert_eq!(fuzz_channel("channel=\"x\"").as_deref(), Some("x"));
        assert_eq!(fuzz_channel("channels = \"x\"\n"), None);
        assert_eq!(fuzz_channel("channel = \"\"\n"), None);
        assert_eq!(fuzz_channel("[toolchain]\n"), None);
    }

    fn plant_fuzz(ws: &Workspace, targets: &[&str], corpora: &[(&str, bool)]) {
        let fuzz = ws.root.join("fuzz");
        fs::create_dir_all(fuzz.join("fuzz_targets")).expect("mkdir");
        fs::write(
            fuzz.join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"nightly-2026-09-20\"\n",
        )
        .expect("toolchain");
        for t in targets {
            fs::write(fuzz.join("fuzz_targets").join(format!("{t}.rs")), "").expect("target");
        }
        fs::write(fuzz.join("fuzz_targets").join("README.md"), "").expect("not a target");
        for (t, seeded) in corpora {
            let dir = fuzz.join("corpus").join(t);
            fs::create_dir_all(&dir).expect("mkdir");
            if *seeded {
                fs::write(dir.join("seed"), "builder").expect("seed");
            }
        }
    }

    /// `version` is `cargo fuzz --version`'s exit; `red` names the targets whose replay fails.
    fn replay(ws: &Workspace, version: i32, red: &[&str]) -> (Result<Suite, Refusal>, Calls) {
        let mut calls = Vec::new();
        let result = {
            let mut runner = |cmd: &mut Command| {
                let args = args_of(cmd);
                let code = if has(&args, &["fuzz", "--version"]) {
                    version
                } else {
                    i32::from(red.iter().any(|t| args.contains(&(*t).to_owned())))
                };
                calls.push(args);
                (Some(code), String::new())
            };
            fuzz_replay(ws, &mut runner)
        };
        (result, calls)
    }

    #[test]
    fn fuzz_replay_without_cargo_fuzz_is_tool_missing() {
        let (_tmp, ws) = scratch();
        plant_fuzz(&ws, &["a"], &[("a", true)]);
        let (result, calls) = replay(&ws, 101, &[]);
        assert_eq!(
            result.expect_err("refused"),
            Refusal::new("tool-missing", Some("cargo-fuzz"))
        );
        assert_eq!(calls.len(), 1);
    }

    #[test]
    fn fuzz_replay_without_its_toolchain_file_is_tool_missing() {
        let (_tmp, ws) = scratch();
        let (result, _) = replay(&ws, 0, &[]);
        assert_eq!(
            result.expect_err("refused"),
            Refusal::new("tool-missing", Some("fuzz/rust-toolchain.toml"))
        );
    }

    #[test]
    fn fuzz_replay_without_targets_is_corpus_empty() {
        let (_tmp, ws) = scratch();
        plant_fuzz(&ws, &[], &[]);
        let (result, _) = replay(&ws, 0, &[]);
        assert_eq!(
            result.expect_err("refused"),
            Refusal::new("corpus-empty", None)
        );
    }

    #[test]
    fn fuzz_replay_counts_each_target_and_never_passes_an_empty_corpus() {
        let (_tmp, ws) = scratch();
        plant_fuzz(
            &ws,
            &["d", "c", "b", "a"],
            &[("a", true), ("b", false), ("d", true)],
        );
        let (result, calls) = replay(&ws, 0, &["d"]);
        let suite = result.expect("replayed");
        assert_eq!(suite.suite, "fuzz-replay");
        assert_eq!(suite.artifact.as_deref(), Some("fuzz/corpus"));
        assert_eq!((suite.passed, suite.failed), (1, 3));
        assert_eq!(suite.failures, ["b: corpus-empty", "c: corpus-empty", "d"]);
        let expected: Vec<String> = [
            "+nightly-2026-09-20",
            "fuzz",
            "run",
            "--fuzz-dir",
            "fuzz",
            "a",
            "fuzz/corpus/a",
            "--",
            "-runs=0",
        ]
        .map(str::to_owned)
        .to_vec();
        assert_eq!(calls[1], expected);
        assert_eq!(calls.len(), 3, "version probe, then a and d only");
    }

    #[test]
    fn fuzz_host_is_linux_only() {
        assert_eq!(fuzz_host_supported(), cfg!(target_os = "linux"));
    }

    #[test]
    fn run_fuzz_replay_refusals_reach_the_document() {
        let (_tmp, ws) = scratch();
        let mut ran = 0;
        let sel = Selection {
            fuzz_replay: true,
            mutants: true,
            ..Selection::default()
        };
        let out = run_with(&ws, sel, None, None, None, &mut |_: &mut Command| {
            ran += 1;
            (Some(101), String::new())
        });
        if cfg!(target_os = "linux") {
            assert_eq!(out.code, 1);
            assert_eq!(out.doc["reason"], "tool-missing");
            assert_eq!(out.doc["detail"], "cargo-fuzz");
            assert!(
                out.doc.get("mutants").is_none(),
                "mutants skipped after a refusal"
            );
            assert_eq!(ran, 1);
        } else {
            assert_eq!(out.code, 2);
            assert_eq!(out.doc["reason"], "fuzz-linux-only");
            assert_eq!(ran, 0);
        }
    }

    const OUTCOMES: &str = r#"{"outcomes": [
        {"scenario": "Baseline", "summary": "Success", "log_path": "log/baseline.log"},
        {"scenario": {"Mutant": {"name": "src/a.rs:1:5: replace a with 0"}}, "summary": "CaughtMutant",
         "phase_results": [{"argv": ["C:\\cargo.exe"]}]},
        {"scenario": {"Mutant": {"name": "src/a.rs:2:5: replace b with 1"}}, "summary": "MissedMutant"},
        {"scenario": {"Mutant": {"name": "src/a.rs:3:5: replace c with 2"}}, "summary": "Timeout"},
        {"scenario": {"Mutant": {"name": "src/a.rs:4:5: replace d with 3"}}, "summary": "Unviable"},
        {"scenario": {"Mutant": {"name": "src/a.rs:5:5: replace e with 4"}}, "summary": "Failure"}
    ], "caught": 1, "missed": 1, "timeout": 1, "unviable": 1}"#;

    #[test]
    fn leg_verdict_keeps_names_and_outcomes_only() {
        let outcomes: Value = serde_json::from_str(OUTCOMES).expect("json");
        let v = leg_verdict("ubuntu-latest", "counted", &outcomes);
        assert_eq!(
            v,
            json!({"v": 1, "leg": "ubuntu-latest", "verdict": "counted", "mutants": [
                {"name": "src/a.rs:1:5: replace a with 0", "outcome": "caught"},
                {"name": "src/a.rs:2:5: replace b with 1", "outcome": "missed"},
                {"name": "src/a.rs:3:5: replace c with 2", "outcome": "timeout"},
                {"name": "src/a.rs:4:5: replace d with 3", "outcome": "unviable"},
            ]})
        );
        assert_eq!(
            leg_verdict("w", "no-rust-delta", &Value::Null)["mutants"],
            json!([])
        );
        assert_eq!(
            leg_verdict_path(Path::new("a"), "w"),
            Path::new("a").join("mutants-verdict-w.json")
        );
    }

    /// A Rust delta in `mini`, then a stand-in `cargo mutants` that writes `outcomes` (or nothing).
    fn run_leg(
        leg: Option<&str>,
        outcomes: Option<&str>,
    ) -> (tempfile::TempDir, Workspace, Outcome) {
        let (tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        fs::write(ws.root.join("src").join("b.rs"), "fn b() {}\n").expect("write");
        let out_dir = ws.root.join("mutants.out");
        let out = run_with(
            &ws,
            flags(false, false, true, false),
            None,
            Some(base),
            leg,
            &mut |cmd: &mut Command| {
                if has(&args_of(cmd), &["mutants"]) {
                    if let Some(text) = outcomes {
                        fs::create_dir_all(&out_dir).expect("mkdir");
                        fs::write(out_dir.join("outcomes.json"), text).expect("outcomes");
                        fs::write(
                            out_dir.join("missed.txt"),
                            "src/a.rs:2:5: replace b with 1\n",
                        )
                        .expect("missed");
                    }
                    return (Some(2), String::new());
                }
                (Some(0), String::new())
            },
        );
        (tmp, ws, out)
    }

    #[test]
    fn run_mutants_leg_defers_survivors_to_the_gate_union() {
        let (_tmp, ws, out) = run_leg(Some("l1"), Some(OUTCOMES));
        assert_eq!(out.code, 0, "{}", out.doc);
        assert_eq!(out.doc["mutants"]["leg"], "l1");
        assert_eq!(suite(&out.doc, "mutants")["survived"], 2);
        let v: Value = read_json(&leg_verdict_path(&ws.artifacts(), "l1")).expect("verdict");
        assert_eq!(v["verdict"], "counted");
        assert_eq!(v["mutants"].as_array().map(Vec::len), Some(4));
        assert!(!v.to_string().contains("argv"));
    }

    #[test]
    fn run_mutants_without_a_leg_keeps_survivors_red() {
        let (_tmp, _ws, out) = run_leg(None, Some(OUTCOMES));
        assert_eq!(out.code, 1);
        assert!(out.doc["mutants"].get("leg").is_none());
    }

    #[test]
    fn run_mutants_leg_without_outcomes_stays_red_and_writes_no_verdict() {
        let (_tmp, ws, out) = run_leg(Some("l1"), None);
        assert_eq!(out.code, 1);
        assert_eq!(
            suite(&out.doc, "mutants")["failures"][0],
            "outcomes-missing"
        );
        assert!(!leg_verdict_path(&ws.artifacts(), "l1").exists());
    }

    #[test]
    fn run_mutants_leg_no_rust_delta_writes_its_verdict() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        fs::write(ws.root.join("README.md"), "docs only\n").expect("write");
        let out = run_with(
            &ws,
            flags(false, false, true, false),
            None,
            Some(base),
            Some("l2"),
            &mut run_forwarding,
        );
        assert_eq!(out.code, 0, "{}", out.doc);
        assert_eq!(out.doc["mutants"]["leg"], "l2");
        let v: Value = read_json(&leg_verdict_path(&ws.artifacts(), "l2")).expect("verdict");
        assert_eq!(v["verdict"], "no-rust-delta");
    }

    #[test]
    fn run_mutants_leg_refused_clears_a_stale_verdict() {
        let (_tmp, ws) = mini(GOOD_LIB);
        fs::create_dir_all(ws.artifacts()).expect("mkdir");
        let stale = leg_verdict_path(&ws.artifacts(), "l3");
        fs::write(&stale, "{}").expect("stale");
        let out = run_with(
            &ws,
            flags(false, false, true, false),
            None,
            Some("0".repeat(40)),
            Some("l3"),
            &mut run_forwarding,
        );
        assert_eq!(out.doc["reason"], "base-missing");
        assert!(!stale.exists());
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
