//! The per-chunk mutation gate (test-plan §3 `run` step 4, §10).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::{Value, json};

use super::{Runner, Suite, Workspace, read_json, write_json};

/// Every outcome printed as it lands, and each mutant's build bounded (cargo-mutants sets no build
/// timeout by default), so a stalled leg names the mutant it stalled on.
const MUTANTS_PROGRESS: [&str; 3] = ["--caught", "--unviable", "--build-timeout-multiplier=5"];

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

pub(super) fn mutants(
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
            .args(MUTANTS_PROGRESS)
            // Live to our stderr: a leg that stalls or is cancelled still shows its last outcome.
            .stdout(Stdio::from(std::io::stderr()))
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
    use super::super::test_support::{GOOD_LIB, args_of, flags, git_repo, has, mini, suite};
    use super::super::{Outcome, run, run_forwarding, run_with};
    use super::*;

    fn head(ws: &Workspace) -> String {
        git(&ws.root, &["rev-parse", "HEAD"])
            .expect("head")
            .trim()
            .to_owned()
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
    fn run_mutants_prints_every_outcome_and_bounds_each_build() {
        let (_tmp, ws) = mini(GOOD_LIB);
        let base = head(&ws);
        fs::write(ws.root.join("src").join("b.rs"), "fn b() {}\n").expect("write");
        let mut calls = Vec::new();
        let _ = run_with(
            &ws,
            flags(false, false, true, false),
            None,
            Some(base),
            None,
            &mut |cmd: &mut Command| {
                calls.push(args_of(cmd));
                (Some(0), String::new())
            },
        );
        let mutants = calls
            .iter()
            .find(|c| has(c, &["mutants", "--workspace"]))
            .expect("cargo mutants ran");
        for flag in ["--caught", "--unviable", "--build-timeout-multiplier=5"] {
            assert!(mutants.contains(&flag.to_owned()), "{flag} in {mutants:?}");
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
