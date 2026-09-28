//! The per-chunk mutation gate (test-plan §3 `run` step 4, §10).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

use serde_json::{Value, json};

use super::{Runner, Suite, Workspace, read_json};

mod base;
mod leg;
mod scratch;

pub use base::{
    chunk_diff, diff_files, diff_paths, resolve_base, rust_delta, rust_paths, test_target,
};
use leg::write_leg_verdict;
pub use leg::{leg_verdict, leg_verdict_path};
pub(in crate::harness) use scratch::host_scratch_bytes;

/// Every outcome printed as it lands, and each mutant's build bounded (cargo-mutants sets no build
/// timeout by default), so a stalled leg names the mutant it stalled on.
const MUTANTS_PROGRESS: [&str; 3] = ["--caught", "--unviable", "--build-timeout-multiplier=5"];

/// The mutation run's own target dir, relative to the tree cargo runs in: the root binaries are
/// prebuilt into the repository's copy, and cargo-mutants' copied tree builds every test binary in
/// its own. A root test binary a local `cargo test` left in `target/debug` still names the
/// repository's unmutated `viola` (`CARGO_BIN_EXE_*` is fixed at compile time, and cargo reads the
/// copied binary as fresh), so a mutant only a root integration test kills would survive.
const MUTANTS_TARGET: &str = "target/mutants";

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

/// `passed` = caught; `survived` = missed + timeout. Unviable alone is never red, but a leg with more
/// unviable than caught mutants tested almost nothing: its builds failed (measured on the windows CI
/// leg of run 36118112104, a leaked process holding a binary: 8 caught, 135 unviable), so it is red.
pub fn mutants_suite(outcomes: &Value, mut failures: Vec<String>) -> (Suite, u64) {
    let caught = count(outcomes, "caught");
    let unviable = count(outcomes, "unviable");
    let survived = count(outcomes, "missed") + count(outcomes, "timeout");
    let tested = caught + survived + unviable;
    let swamped = unviable > caught;
    if swamped {
        failures.push("unviable-exceeds-caught".to_owned());
    }
    let suite = Suite {
        suite: "mutants".to_owned(),
        passed: caught,
        failed: survived + u64::from(swamped),
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

/// A diff cargo-mutants can find no mutant in never reaches it: its silent exit writes no
/// `outcomes.json`, and an earlier run's file would otherwise be read as this one's. The verdict
/// names why it is empty, never a bare pass.
fn unmutated(
    ws: &Workspace,
    leg: Option<&str>,
    verdict: &str,
    diff: &str,
    base: &str,
) -> (Suite, Value) {
    write_leg_verdict(ws, leg, verdict, &Value::Null);
    let suite = Suite {
        artifact: Some("target/agent-run/chunk.diff".to_owned()),
        ..Suite::named("mutants")
    };
    let mut doc = json!({
        "tested": 0,
        "verdict": verdict,
        "base": base,
        "diff": "target/agent-run/chunk.diff",
        "files": diff_files(diff),
    });
    if let Some(leg) = leg {
        doc["leg"] = json!(leg);
    }
    (suite, doc)
}

/// One phase of the arm through `runner`, bracketed by `mutants-phase` lines on stderr. The start
/// line survives a kill, so a phase that stalls is still named and the run's phase times stay
/// readable in a killed test's captured output.
fn timed(runner: &mut Runner<'_>, name: &str, cmd: &mut Command) -> (Option<i32>, String) {
    let started = Instant::now();
    eprintln!("mutants-phase {name} start");
    let out = runner(cmd);
    eprintln!(
        "mutants-phase {name} done {} ms",
        started.elapsed().as_millis()
    );
    out
}

/// The mutation arm's result: its suite, its document part, and the `outcomes.json` it read (for
/// the run archive), when cargo-mutants ran.
pub(super) type Mutated = (Suite, Value, Option<PathBuf>);

/// `files` scopes the run to those sources (`--file`): an inner-loop filter whose `scoped` verdict
/// is never a leg verdict, so no union ever reads it.
pub(super) fn mutants(
    ws: &Workspace,
    chunk_base: Option<String>,
    leg: Option<&str>,
    files: &[String],
    runner: &mut Runner<'_>,
) -> Result<Mutated, String> {
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
        let (suite, doc) = unmutated(ws, leg, "no-rust-delta", &diff, &base);
        return Ok((suite, doc, None));
    }
    let rust_files = rust_paths(&diff);
    if rust_files.iter().all(|p| test_target(p)) {
        let (suite, mut doc) = unmutated(ws, leg, "test-only-rust-delta", &diff, &base);
        doc["rust_files"] = json!(rust_files);
        return Ok((suite, doc, None));
    }
    let scratch = scratch::prepare(&ws.root)?;
    let out_dir = match &scratch {
        Some((dir, _)) => dir.join("mutants.out"),
        None => ws.root.join("mutants.out"),
    };
    let _ = fs::remove_file(out_dir.join("outcomes.json"));

    // cargo-mutants builds only the packages a diff touches, but the harness tests spawn the root
    // package's `viola` and `viola-fake-agent`: build them (root package only, so the running
    // `viola-harness` is never relinked) and copy `target/` into the scratch tree.
    let (built, _) = timed(
        runner,
        "prebuild",
        Command::new("cargo")
            .args(["build", "--package", "viola", "--features", "fake-agent"])
            .env("CARGO_TARGET_DIR", ws.root.join(MUTANTS_TARGET))
            .current_dir(&ws.root),
    );
    if built != Some(0) {
        return Err("build-failed".to_owned());
    }
    let mut cargo_mutants = Command::new("cargo");
    cargo_mutants
        .args([
            "mutants",
            "--workspace",
            "--features",
            "fake-agent",
            "--in-diff",
        ])
        .arg(&diff_path);
    for file in files {
        cargo_mutants.arg("--file").arg(file);
    }
    cargo_mutants
        .args(["--test-tool=nextest", "--copy-target=true"])
        .args(MUTANTS_PROGRESS)
        // MEASUREMENT ONLY (macOS 120 s kill): cargo-mutants' own phase trace; removed after it.
        .args(["-L", "debug", "--all-logs"])
        // Live to our stderr: a leg that stalls or is cancelled still shows its last outcome.
        .stdout(Stdio::from(std::io::stderr()))
        .env("NEXTEST_PROFILE", "mutants")
        // No home outlives a mutant's test run: nothing reads them afterwards (the kept-home
        // gates belong to the test job), and each carries a pinned copy of the viola binary.
        .env("AGENT_RUN_KEEP_HOMES", "0")
        .env("AGENT_RUN_KEEP_FAILED", "0")
        .env("CARGO_TARGET_DIR", MUTANTS_TARGET)
        .current_dir(&ws.root);
    if let Some((dir, _)) = &scratch {
        // cargo-mutants copies the tree into `std::env::temp_dir()`, which Windows reads from
        // `TMP`, then `TEMP`.
        cargo_mutants
            .env("TMP", dir)
            .env("TEMP", dir)
            .arg("--output")
            .arg(dir);
    }
    let (code, _) = timed(runner, "cargo-mutants", &mut cargo_mutants);
    if let Some(reason) = mutants_exit_reason(code) {
        return Err(reason);
    }
    let verdict = if files.is_empty() {
        "counted"
    } else {
        "scoped"
    };
    let outcomes_path = out_dir.join("outcomes.json");
    let (suite, tested) = match read_json::<Value>(&outcomes_path) {
        Ok(outcomes) => {
            write_leg_verdict(ws, leg, verdict, &outcomes);
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
    let mut doc = json!({"tested": tested, "verdict": verdict, "base": base});
    if !files.is_empty() {
        doc["files"] = json!(files);
    }
    if let Some((_, bytes)) = scratch {
        doc["scratch_bytes"] = json!(bytes);
    }
    if let Some(leg) = leg {
        doc["leg"] = json!(leg);
    }
    Ok((suite, doc, Some(outcomes_path)))
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{GOOD_LIB, args_of, flags, has, mini, suite};
    use super::super::{Outcome, Selection, run, run_forwarding, run_with};
    use super::base::git;
    use super::*;

    fn head(ws: &Workspace) -> String {
        git(&ws.root, &["rev-parse", "HEAD"])
            .expect("head")
            .trim()
            .to_owned()
    }

    /// `run --mutants` over the throwaway workspace with a private `CARGO_HOME` beside it (in the
    /// test's tempdir, outside the diffed tree): the workspace has no dependencies, so its nested
    /// cargo never needs a registry, and it never waits on the package-cache lock of the cargo
    /// running this suite (measured on the macOS runner: 96 s of a 101 s run blocked on it).
    fn run_private(ws: &Workspace, base: String) -> Outcome {
        let cargo_home = ws.root.parent().unwrap_or(&ws.root).join("cargo-home");
        let mut runner = |cmd: &mut Command| {
            cmd.env("CARGO_HOME", &cargo_home);
            run_forwarding(cmd)
        };
        run_with(
            ws,
            flags(false, false, true, false),
            None,
            Some(base),
            None,
            &mut runner,
        )
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
    fn mutants_suite_is_red_when_unviable_outnumbers_caught() {
        let swamp = json!({"caught": 8, "missed": 0, "timeout": 0, "unviable": 135});
        let (s, tested) = mutants_suite(&swamp, vec![]);
        assert_eq!((s.passed, s.failed, s.survived, tested), (8, 1, 0, 143));
        assert_eq!(s.failures, vec!["unviable-exceeds-caught"]);
        assert!(!s.green());
        let even = json!({"caught": 3, "unviable": 3});
        let (s, _) = mutants_suite(&even, vec![]);
        assert!(s.green(), "equal counts are not a swamp");
        assert!(s.failures.is_empty());
    }

    #[test]
    fn run_mutants_leg_with_an_unviable_swamp_is_red_not_deferred() {
        let swamp = r#"{"outcomes": [
            {"scenario": {"Mutant": {"name": "src/a.rs:1:5: replace a with 0"}}, "summary": "CaughtMutant"},
            {"scenario": {"Mutant": {"name": "src/a.rs:2:5: replace b with 1"}}, "summary": "Unviable"},
            {"scenario": {"Mutant": {"name": "src/a.rs:3:5: replace c with 2"}}, "summary": "Unviable"}
        ], "caught": 1, "missed": 0, "timeout": 0, "unviable": 2}"#;
        let (_tmp, _ws, out) = run_leg(Some("l4"), Some(swamp));
        assert_eq!(out.code, 1, "{}", out.doc);
        let m = suite(&out.doc, "mutants");
        assert!(
            m["failures"]
                .as_array()
                .is_some_and(|f| f.iter().any(|x| x == "unviable-exceeds-caught"))
        );
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
        let out = run_private(&ws, base);
        assert_eq!(out.code, 1);
        assert_eq!(out.doc["reason"], "build-failed");
    }

    #[test]
    fn run_mutants_reports_survivors_of_an_untested_change() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        let lib = format!("{GOOD_LIB}pub fn three() -> u32 {{ 3 }}\n");
        fs::write(ws.root.join("src").join("lib.rs"), lib).expect("write");
        let out = run_private(&ws, base);
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
        let out = run_private(&ws, base);
        assert_eq!(out.code, 0, "{}", out.doc);
        let m = suite(&out.doc, "mutants");
        assert_eq!(m["survived"], 0);
        assert!(m["passed"].as_u64().is_some_and(|n| n >= 1));
        assert_eq!(out.doc["mutants"]["verdict"], "counted");
    }

    /// Where this host's run keeps cargo-mutants' output: the repo's sibling scratch on Windows.
    pub(super) fn scratch_or_root(ws: &Workspace) -> PathBuf {
        let scratch = ws.root.parent().map(|p| p.join("viola-mutants-scratch"));
        scratch.filter(|_| cfg!(windows)).unwrap_or(ws.root.clone())
    }

    const STALE_OUTCOMES: &str = r#"{"caught": 777, "missed": 5, "timeout": 0, "unviable": 0}"#;

    fn plant_stale_outcomes(ws: &Workspace) {
        let out = scratch_or_root(ws).join("mutants.out");
        fs::create_dir_all(&out).expect("mkdir");
        fs::write(out.join("outcomes.json"), STALE_OUTCOMES).expect("write");
    }

    /// Whether any NUMBER in `doc` equals `n`: stale outcomes would surface as a count, while a
    /// `base` sha or a path may contain the digits by chance (a substring check once read `777`
    /// inside a sha and failed four tests at once).
    fn carries_number(doc: &Value, n: u64) -> bool {
        match doc {
            Value::Number(v) => v.as_u64() == Some(n),
            Value::Array(items) => items.iter().any(|v| carries_number(v, n)),
            Value::Object(fields) => fields.values().any(|v| carries_number(v, n)),
            _ => false,
        }
    }

    #[test]
    fn carries_number_reads_counts_never_digits_in_strings() {
        let sha = json!({"mutants": {"base": "0a1777bc", "diff": "x/777/y"}});
        assert!(
            sha.to_string().contains("777"),
            "the old substring check would flag it"
        );
        assert!(!carries_number(&sha, 777));
        let stale = json!({"suites": [{"suite": "mutants", "caught": 777}]});
        assert!(carries_number(&stale, 777));
        assert!(!carries_number(&json!({"caught": 7770}), 777));
    }

    #[test]
    fn run_mutants_no_rust_delta_passes_by_name_and_never_reads_stale_outcomes() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        plant_stale_outcomes(&ws);
        fs::write(ws.root.join("README.md"), "docs only\n").expect("write");
        let out = run(
            &ws,
            flags(false, false, true, false),
            None,
            Some(base.clone()),
        );
        assert_eq!(out.code, 0, "{}", out.doc);
        assert_eq!(
            out.doc["mutants"],
            json!({"tested": 0, "verdict": "no-rust-delta", "base": base,
                   "diff": "target/agent-run/chunk.diff", "files": 1})
        );
        let m = suite(&out.doc, "mutants");
        assert_eq!(
            (m["passed"].as_u64(), m["survived"].as_u64()),
            (Some(0), Some(0))
        );
        assert_eq!(m["artifact"], "target/agent-run/chunk.diff");
        assert!(!carries_number(&out.doc, 777), "{}", out.doc);
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
        let out = run(
            &ws,
            flags(false, false, true, false),
            None,
            Some(base.clone()),
        );
        assert_eq!(out.code, 1, "{}", out.doc);
        let m = suite(&out.doc, "mutants");
        assert_eq!(m["failures"][0], "outcomes-missing");
        assert_eq!(
            out.doc["mutants"],
            wiped_stale(json!({"tested": 0, "verdict": "counted", "base": base}))
        );
        assert!(!carries_number(&out.doc, 777), "{}", out.doc);
    }

    /// A counted run's expected `mutants` part after `plant_stale_outcomes`: on a Windows host the
    /// scratch held the stale file, so the run reports its bytes as it wipes it.
    fn wiped_stale(mut doc: Value) -> Value {
        if cfg!(windows) {
            doc["scratch_bytes"] = json!(STALE_OUTCOMES.len());
        }
        doc
    }

    fn write_test_target(ws: &Workspace) {
        fs::create_dir_all(ws.root.join("tests")).expect("mkdir");
        fs::write(ws.root.join("tests").join("it.rs"), "#[test]\nfn it() {}\n").expect("write");
    }

    #[test]
    fn run_mutants_test_only_delta_passes_by_name_without_running_cargo() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        plant_stale_outcomes(&ws);
        write_test_target(&ws);
        fs::write(ws.root.join("README.md"), "docs\n").expect("write");
        let mut calls = Vec::new();
        let out = run_with(
            &ws,
            flags(false, false, true, false),
            None,
            Some(base.clone()),
            Some("l5"),
            &mut |cmd: &mut Command| {
                calls.push(args_of(cmd));
                (Some(0), String::new())
            },
        );
        assert_eq!(out.code, 0, "{}", out.doc);
        assert_eq!(
            out.doc["mutants"],
            json!({"tested": 0, "verdict": "test-only-rust-delta", "base": base,
                   "diff": "target/agent-run/chunk.diff",
                   "files": 2, "rust_files": ["tests/it.rs"], "leg": "l5"})
        );
        assert!(calls.is_empty(), "{calls:?}");
        let v: Value = read_json(&leg_verdict_path(&ws.artifacts(), "l5")).expect("verdict");
        assert_eq!(v["verdict"], "test-only-rust-delta");
        assert!(!carries_number(&out.doc, 777), "{}", out.doc);
    }

    #[test]
    fn run_mutants_mixed_src_and_test_delta_without_outcomes_stays_outcomes_missing() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        plant_stale_outcomes(&ws);
        write_test_target(&ws);
        fs::write(ws.root.join("src").join("b.rs"), "fn b() {}\n").expect("write");
        let out = run_with(
            &ws,
            flags(false, false, true, false),
            None,
            Some(base.clone()),
            Some("l6"),
            &mut |_: &mut Command| (Some(0), String::new()),
        );
        assert_eq!(out.code, 1, "{}", out.doc);
        assert_eq!(
            suite(&out.doc, "mutants")["failures"][0],
            "outcomes-missing"
        );
        assert_eq!(
            out.doc["mutants"],
            wiped_stale(json!({"tested": 0, "verdict": "counted", "base": base, "leg": "l6"}))
        );
        assert!(!leg_verdict_path(&ws.artifacts(), "l6").exists());
        assert!(!carries_number(&out.doc, 777), "{}", out.doc);
    }

    pub(super) const OUTCOMES: &str = r#"{"outcomes": [
        {"scenario": "Baseline", "summary": "Success", "log_path": "log/baseline.log"},
        {"scenario": {"Mutant": {"name": "src/a.rs:1:5: replace a with 0"}}, "summary": "CaughtMutant",
         "phase_results": [{"argv": ["C:\\cargo.exe"]}]},
        {"scenario": {"Mutant": {"name": "src/a.rs:2:5: replace b with 1"}}, "summary": "MissedMutant"},
        {"scenario": {"Mutant": {"name": "src/a.rs:3:5: replace c with 2"}}, "summary": "Timeout"},
        {"scenario": {"Mutant": {"name": "src/a.rs:4:5: replace d with 3"}}, "summary": "Unviable"},
        {"scenario": {"Mutant": {"name": "src/a.rs:5:5: replace e with 4"}}, "summary": "Failure"}
    ], "caught": 1, "missed": 1, "timeout": 1, "unviable": 1}"#;

    /// A Rust delta in `mini`, then a stand-in `cargo mutants` that writes `outcomes` (or nothing).
    fn run_leg(
        leg: Option<&str>,
        outcomes: Option<&str>,
    ) -> (tempfile::TempDir, Workspace, Outcome) {
        let s = stub_run(&[], leg, outcomes, |_| {});
        (s.tmp, s.ws, s.out)
    }

    pub(super) type Env = Vec<(String, Option<String>)>;

    fn env_of(cmd: &Command) -> Env {
        cmd.get_envs()
            .map(|(k, v)| {
                let v = v.map(|v| v.to_string_lossy().into_owned());
                (k.to_string_lossy().into_owned(), v)
            })
            .collect()
    }

    pub(super) struct Stubbed {
        tmp: tempfile::TempDir,
        pub(super) ws: Workspace,
        pub(super) out: Outcome,
        /// The `cargo mutants` call's args and env, when it ran.
        pub(super) mutants: Option<(Vec<String>, Env)>,
    }

    /// `run_leg` scoped to `files`, with `before` run on the workspace first; the stand-in writes
    /// where this host's run reads.
    pub(super) fn stub_run(
        files: &[&str],
        leg: Option<&str>,
        outcomes: Option<&str>,
        before: impl FnOnce(&Workspace),
    ) -> Stubbed {
        let (tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        fs::write(ws.root.join("src").join("b.rs"), "fn b() {}\n").expect("write");
        before(&ws);
        let out_dir = scratch_or_root(&ws).join("mutants.out");
        let mut mutants = None;
        let sel = Selection {
            files: files.iter().map(|f| (*f).to_owned()).collect(),
            ..flags(false, false, true, false)
        };
        let out = run_with(&ws, sel, None, Some(base), leg, &mut |cmd: &mut Command| {
            if has(&args_of(cmd), &["mutants"]) {
                mutants = Some((args_of(cmd), env_of(cmd)));
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
        });
        Stubbed {
            tmp,
            ws,
            out,
            mutants,
        }
    }

    pub(super) const CAUGHT: &str = r#"{"outcomes": [
        {"scenario": {"Mutant": {"name": "src/a.rs:1:5: replace a with 0"}}, "summary": "CaughtMutant"}
    ], "caught": 1, "missed": 0, "timeout": 0, "unviable": 0}"#;

    /// `--file` scopes cargo-mutants beside `--in-diff`; the verdict is `scoped` with the files, and
    /// no leg verdict is written. Survivors stay red, as in a counted run.
    #[test]
    fn run_mutants_scoped_passes_each_file_and_reads_as_scoped() {
        let s = stub_run(&["src/b.rs", "src/c.rs"], None, Some(OUTCOMES), |_| {});
        assert_eq!(s.out.code, 1, "{}", s.out.doc);
        assert_eq!(s.out.doc["mutants"]["verdict"], "scoped");
        assert_eq!(
            s.out.doc["mutants"]["files"],
            json!(["src/b.rs", "src/c.rs"])
        );
        let (args, _) = s.mutants.expect("cargo mutants ran");
        let at = args
            .iter()
            .position(|a| a == "--in-diff")
            .expect("--in-diff");
        assert_eq!(
            args[at + 2..at + 6],
            ["--file", "src/b.rs", "--file", "src/c.rs"]
        );
        let written = fs::read_dir(s.ws.artifacts())
            .into_iter()
            .flatten()
            .flatten();
        assert!(
            !written
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .any(|n| n.starts_with("mutants-verdict-")),
            "a scoped run wrote a leg verdict"
        );

        let clean = stub_run(&["src/b.rs"], None, Some(CAUGHT), |_| {});
        assert_eq!(clean.out.code, 0, "{}", clean.out.doc);
        assert_eq!(clean.out.doc["mutants"]["verdict"], "scoped");
        let unscoped = stub_run(&[], None, Some(CAUGHT), |_| {});
        assert_eq!(unscoped.out.doc["mutants"]["verdict"], "counted");
        assert!(unscoped.out.doc["mutants"].get("files").is_none());
        let (args, _) = unscoped.mutants.expect("ran");
        assert!(!args.contains(&"--file".to_owned()), "{args:?}");
    }

    #[test]
    fn run_mutants_scoped_with_a_leg_is_a_usage_refusal() {
        let s = stub_run(&["src/b.rs"], Some("l7"), Some(CAUGHT), |_| {});
        assert_eq!(s.out.code, 2, "{}", s.out.doc);
        assert_eq!(
            s.out.doc,
            json!({"v": 1, "cmd": "run", "ok": false, "reason": "usage", "detail": "scoped-leg"})
        );
        assert!(s.mutants.is_none());
        assert!(!leg_verdict_path(&s.ws.artifacts(), "l7").exists());
    }

    /// The run's `outcomes.json` is copied into the run archive, outside `target/agent-run/`.
    #[test]
    fn run_mutants_archives_the_outcomes_it_read() {
        let s = stub_run(&[], None, Some(CAUGHT), |_| {});
        assert_eq!(
            s.out.doc["archived"], "target/run-archive/1",
            "{}",
            s.out.doc
        );
        let archived = s.ws.root.join("target/run-archive/1/outcomes.json");
        assert_eq!(fs::read_to_string(archived).expect("archived"), CAUGHT);
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
    fn run_mutants_prints_every_outcome_and_bounds_each_build() {
        let (mutants, _) = stub_run(&[], None, None, |_| {}).mutants.expect("ran");
        for flag in ["--caught", "--unviable", "--build-timeout-multiplier=5"] {
            assert!(mutants.contains(&flag.to_owned()), "{flag} in {mutants:?}");
        }
    }

    /// Both the root prebuild and cargo-mutants build in `target/mutants`: the prebuild in the
    /// repository's, cargo-mutants (a relative path) in its copied tree's.
    #[test]
    fn run_mutants_builds_in_its_own_target_dir() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        fs::write(ws.root.join("src").join("b.rs"), "fn b() {}\n").expect("write");
        let (mut prebuild, mut mutants) = (None, None);
        let _ = run_with(
            &ws,
            flags(false, false, true, false),
            None,
            Some(base),
            None,
            &mut |cmd: &mut Command| {
                let args = args_of(cmd);
                if has(&args, &["build", "--package", "viola"]) {
                    prebuild = Some(env_of(cmd));
                } else if has(&args, &["mutants", "--workspace"]) {
                    mutants = Some(env_of(cmd));
                }
                (Some(0), String::new())
            },
        );
        let target = |env: Option<Env>| {
            env.expect("ran")
                .into_iter()
                .find(|(k, _)| k == "CARGO_TARGET_DIR")
                .and_then(|(_, v)| v)
        };
        let seeded = ws.root.join("target/mutants");
        assert_eq!(
            target(prebuild),
            Some(seeded.to_string_lossy().into_owned())
        );
        assert_eq!(target(mutants).as_deref(), Some("target/mutants"));
    }

    #[test]
    fn run_mutants_never_keeps_test_homes() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        fs::write(ws.root.join("src").join("b.rs"), "fn b() {}\n").expect("write");
        let mut env = None;
        let _ = run_with(
            &ws,
            flags(false, false, true, false),
            None,
            Some(base),
            Some("l1"),
            &mut |cmd: &mut Command| {
                if has(&args_of(cmd), &["mutants", "--workspace"]) {
                    env = Some(env_of(cmd));
                }
                (Some(0), String::new())
            },
        );
        let env = env.expect("cargo mutants ran");
        for name in ["AGENT_RUN_KEEP_HOMES", "AGENT_RUN_KEEP_FAILED"] {
            assert!(
                env.contains(&(name.to_owned(), Some("0".to_owned()))),
                "{name} in {env:?}"
            );
        }
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
