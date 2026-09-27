//! `run`: the built suites (unit · integration · doctest · coverage · browser · fuzz-replay) and the
//! per-chunk mutation gate, one JSON summary, merged by suite into
//! `target/agent-run/artifacts/run-summary.json` (test-plan §3 `run`).

mod browser;
mod coverage;
mod doctest;
mod fuzz;
mod mutants;
mod nextest;

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::{Outcome, Workspace, read_json, write_json};

pub use coverage::{COVERAGE_FLOORS, COVERAGE_IGNORE};
pub use doctest::parse_doctest;
pub use fuzz::{FUZZ_HOST_SUPPORTED, fuzz_channel};
pub(super) use mutants::host_scratch_bytes;
pub use mutants::{
    chunk_diff, diff_files, diff_paths, leg_verdict, leg_verdict_path, mutants_exit_reason,
    mutants_suite, resolve_base, rust_delta,
};
pub use nextest::parse_junit;

/// Runs one tool invocation: its exit code and its stdout.
pub type Runner<'r> = dyn FnMut(&mut Command) -> (Option<i32>, String) + 'r;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Selection {
    pub unit: bool,
    pub integration: bool,
    pub mutants: bool,
    pub coverage: bool,
    pub fuzz_replay: bool,
    pub browser: bool,
    /// `--file`: the mutation run scoped to these sources (the inner loop, never a verdict).
    pub files: Vec<String>,
}

impl Selection {
    /// No selector, or `--all`, selects unit, integration and mutants. `--coverage` replaces the
    /// nextest runs, `--fuzz-replay` is Linux-only and `--browser` needs Node and Chromium, so none
    /// of them joins the default.
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

/// `chunk_base` is `AGENT_RUN_CHUNK_BASE` as the caller read it: an explicit override of the base
/// `resolve_base` derives from the master route's history.
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

/// `leg` names a mutation leg whose verdict the `gate` union decides (`--mutants --leg`).
pub fn run_with(
    ws: &Workspace,
    sel: Selection,
    filter: Option<&str>,
    chunk_base: Option<String>,
    leg: Option<&str>,
    runner: &mut Runner<'_>,
) -> Outcome {
    if sel.fuzz_replay && !FUZZ_HOST_SUPPORTED {
        let doc = json!({"v": 1, "cmd": "run", "ok": false, "reason": "fuzz-linux-only"});
        return Outcome { doc, code: 2 };
    }
    // A scoped run is never a leg: the union must never read an inner loop's verdict.
    if !sel.files.is_empty() && leg.is_some() {
        return Outcome::usage(Some("run"), "scoped-leg");
    }
    let (mut suites, browser_refusal) = test_suites(ws, &sel, filter, runner);
    let (tool_refusal, mutants_doc, outcomes) =
        tool_arms(ws, &sel, chunk_base, leg, runner, &mut suites);
    let refusal = browser_refusal.or(tool_refusal);
    let deferred = |s: &Suite| leg.is_some() && s.suite == "mutants" && s.failed == s.survived;
    let ok = refusal.is_none() && suites.iter().all(|s| s.green() || deferred(s));
    let _ = merge_summary(&ws.artifacts().join("run-summary.json"), &suites);
    let archived = archive(ws, &archive_sources(&suites, outcomes));
    let mut doc = document(ok, refusal, &suites, mutants_doc);
    if let Some((dir, skipped)) = archived {
        for name in skipped {
            eprintln!("run-archive: skipped {name} (absent)");
        }
        doc["archived"] = json!(dir);
    }
    Outcome::new(doc, ok)
}

/// The JUnit reports this run's nextest, coverage or browser suites wrote (a suite that wrote none
/// names its file with no source), then the `outcomes.json` the mutation arm read. The browser
/// suite's artifact is Playwright's `pw.json`; its JUnit is the sibling `pw-junit.xml`.
fn archive_sources(suites: &[Suite], outcomes: Option<PathBuf>) -> Vec<(String, Option<PathBuf>)> {
    let mut sources: Vec<(String, Option<PathBuf>)> = suites
        .iter()
        .filter(|s| {
            [
                "nextest-unit",
                "nextest-integration",
                "coverage",
                "playwright",
            ]
            .contains(&s.suite.as_str())
        })
        .map(|s| {
            let name = format!("junit-{}.xml", s.suite);
            let source = s.artifact.as_ref().map(PathBuf::from);
            let source = if s.suite == "playwright" {
                source.map(|p| p.with_file_name(browser::JUNIT))
            } else {
                source
            };
            (name, source)
        })
        .collect();
    if let Some(path) = outcomes {
        sources.push(("outcomes.json".to_owned(), Some(path)));
    }
    sources
}

const ARCHIVE: &str = "target/run-archive";
const ARCHIVE_KEEP: u64 = 10;

/// The next archive slot: one more than the highest numbered one.
fn next_slot(existing: &[u64]) -> u64 {
    existing.iter().max().map_or(1, |n| n + 1)
}

/// The slots past the newest `ARCHIVE_KEEP`, oldest first.
fn stale_slots(existing: &[u64]) -> Vec<u64> {
    let mut slots = existing.to_vec();
    slots.sort_unstable();
    let keep_from = slots
        .len()
        .saturating_sub(usize::try_from(ARCHIVE_KEEP).unwrap_or(usize::MAX));
    slots.truncate(keep_from);
    slots
}

/// Copies the sources into `target/run-archive/<n>/` before the next run overwrites them, keeps the
/// newest ten, and returns the repo-relative dir with the names of the sources it skipped (absent),
/// which the caller names on stderr. It sits outside `target/agent-run/`, which CI uploads, because
/// `outcomes.json` carries absolute argv paths.
fn archive(ws: &Workspace, sources: &[(String, Option<PathBuf>)]) -> Option<(String, Vec<String>)> {
    if sources.is_empty() {
        return None;
    }
    let root = ws.root.join(ARCHIVE);
    let existing: Vec<u64> = fs::read_dir(&root)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| e.file_name().to_str()?.parse().ok())
        .collect();
    let slot = next_slot(&existing);
    let dir = root.join(slot.to_string());
    fs::create_dir_all(&dir).ok()?;
    let skipped = sources
        .iter()
        .filter(|(name, source)| {
            !source
                .as_ref()
                .is_some_and(|s| fs::copy(s, dir.join(name)).is_ok())
        })
        .map(|(name, _)| name.clone())
        .collect();
    let mut all = existing;
    all.push(slot);
    for stale in stale_slots(&all) {
        let _ = fs::remove_dir_all(root.join(stale.to_string()));
    }
    Some((format!("{ARCHIVE}/{slot}"), skipped))
}

/// The nextest runs (or the one instrumented run that replaces them), the doctests, then the
/// browser suite with its `browser-missing` refusal.
fn test_suites(
    ws: &Workspace,
    sel: &Selection,
    filter: Option<&str>,
    runner: &mut Runner<'_>,
) -> (Vec<Suite>, Option<Refusal>) {
    let mut suites = Vec::new();
    if sel.coverage {
        suites.push(coverage::coverage(ws, filter, runner));
    } else {
        if sel.unit {
            suites.push(nextest::nextest(
                ws,
                "nextest-unit",
                nextest::UNIT,
                filter,
                runner,
            ));
        }
        if sel.integration {
            suites.push(nextest::nextest(
                ws,
                "nextest-integration",
                nextest::INTEGRATION,
                filter,
                runner,
            ));
        }
    }
    if sel.unit || sel.integration || sel.coverage {
        suites.push(doctest::doctest(ws, runner));
    }
    let mut refusal = None;
    if sel.browser {
        let (suite, missing) = browser::browser(ws, runner);
        suites.push(suite);
        refusal = missing;
    }
    (suites, refusal)
}

/// Fuzz replay, then the mutation gate unless a refusal came first.
fn tool_arms(
    ws: &Workspace,
    sel: &Selection,
    chunk_base: Option<String>,
    leg: Option<&str>,
    runner: &mut Runner<'_>,
    suites: &mut Vec<Suite>,
) -> (Option<Refusal>, Option<Value>, Option<PathBuf>) {
    let mut refusal = None;
    if sel.fuzz_replay {
        match fuzz::fuzz_replay(ws, runner) {
            Ok(suite) => suites.push(suite),
            Err(r) => refusal = Some(r),
        }
    }
    let mut mutants_doc = None;
    let mut outcomes = None;
    if sel.mutants && refusal.is_none() {
        match mutants::mutants(ws, chunk_base, leg, &sel.files, runner) {
            Ok((suite, doc, read)) => {
                suites.push(suite);
                mutants_doc = Some(doc);
                outcomes = read;
            }
            Err(reason) => refusal = Some(Refusal::new(&reason, None)),
        }
    }
    (refusal, mutants_doc, outcomes)
}

/// Key order is the contract: `v, cmd, ok`, then `reason`, `detail`, `suites`, `mutants`, and
/// `archived` after them.
fn document(
    ok: bool,
    refusal: Option<Refusal>,
    suites: &[Suite],
    mutants_doc: Option<Value>,
) -> Value {
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
    doc
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

/// Fixtures shared by the `run` submodules' tests.
#[cfg(test)]
mod test_support {
    use std::fs;
    use std::process::Command;

    use serde_json::Value;

    use super::{Selection, Workspace};

    pub(super) type Calls = Vec<Vec<String>>;

    pub(super) const GOOD_LIB: &str = "/// ```\n/// assert_eq!(viola::two(), 2);\n/// ```\n\
        pub fn two() -> u32 { 2 }\n\
        #[cfg(test)]\nmod tests {\n    #[test]\n    fn two_is_two() { assert_eq!(super::two(), 2); }\n}\n";

    pub(super) fn flags(unit: bool, integration: bool, mutants: bool, all: bool) -> Selection {
        let named = Selection {
            unit,
            integration,
            mutants,
            ..Selection::default()
        };
        Selection::from_flags(named, all)
    }

    pub(super) fn git_repo() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().expect("tempdir");
        init_repo(tmp.path());
        tmp
    }

    /// A one-commit repo at `dir`.
    fn init_repo(dir: &std::path::Path) {
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
    }

    /// A throwaway one-crate workspace in its own git repo: never this workspace, never nested in it.
    /// The repo sits one level down, so its host mutation scratch (the repo's sibling) is its own.
    pub(super) fn mini(lib: &str) -> (tempfile::TempDir, Workspace) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let root = &tmp.path().join("ws");
        fs::create_dir_all(root).expect("mkdir");
        init_repo(root);
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

    pub(super) fn suite<'a>(doc: &'a Value, name: &str) -> &'a Value {
        doc["suites"]
            .as_array()
            .and_then(|s| s.iter().find(|x| x["suite"] == name))
            .unwrap_or_else(|| panic!("suite {name} in {doc}"))
    }

    pub(super) fn args_of(cmd: &Command) -> Vec<String> {
        cmd.get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect()
    }

    pub(super) fn has(call: &[String], words: &[&str]) -> bool {
        call.windows(words.len())
            .any(|w| w.iter().zip(words).all(|(a, b)| a == b))
    }

    pub(super) fn scratch() -> (tempfile::TempDir, Workspace) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ws = Workspace {
            root: tmp.path().to_path_buf(),
        };
        (tmp, ws)
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::{GOOD_LIB, flags, mini, scratch, suite};
    use super::*;

    #[test]
    fn archive_slots_number_up_and_keep_the_newest_ten() {
        assert_eq!(next_slot(&[]), 1);
        assert_eq!(next_slot(&[3, 12, 7]), 13);
        let eleven: Vec<u64> = (1..=11).collect();
        assert_eq!(stale_slots(&eleven), vec![1]);
        assert!(stale_slots(&(1..=10).collect::<Vec<u64>>()).is_empty());
        assert_eq!(
            stale_slots(&[14, 2, 9, 30, 1, 5, 6, 7, 8, 10, 11, 12]),
            vec![1, 2]
        );
    }

    /// Each run fills the next slot; the oldest past ten go; an absent source is skipped, and a
    /// run with nothing to archive makes no slot.
    #[test]
    fn archive_copies_the_sources_into_the_next_slot_and_prunes() {
        let (tmp, ws) = scratch();
        let junit = tmp.path().join("junit.xml");
        fs::write(&junit, "<testsuites/>").expect("write");
        let root = ws.root.join(ARCHIVE);
        for n in [1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
            fs::create_dir_all(root.join(n.to_string())).expect("mkdir");
        }
        fs::create_dir_all(root.join("notes")).expect("mkdir");
        let sources = [
            ("junit-nextest-unit.xml".to_owned(), Some(junit)),
            (
                "outcomes.json".to_owned(),
                Some(tmp.path().join("absent.json")),
            ),
            ("junit-coverage.xml".to_owned(), None),
        ];
        assert_eq!(
            archive(&ws, &sources),
            Some((
                "target/run-archive/11".to_owned(),
                vec!["outcomes.json".to_owned(), "junit-coverage.xml".to_owned()]
            ))
        );
        let slot = root.join("11");
        assert_eq!(
            fs::read_to_string(slot.join("junit-nextest-unit.xml")).expect("copied"),
            "<testsuites/>"
        );
        assert!(!slot.join("outcomes.json").exists());
        assert!(!slot.join("junit-coverage.xml").exists());
        assert!(!root.join("1").exists(), "the oldest was pruned");
        assert!(root.join("2").is_dir() && root.join("notes").is_dir());
        assert_eq!(archive(&ws, &[]), None);
        assert!(!root.join("12").exists());
    }

    #[test]
    fn archive_sources_are_the_junit_suites_and_the_read_outcomes() {
        let written = Suite {
            artifact: Some("a/junit-nextest-unit.xml".to_owned()),
            ..Suite::named("nextest-unit")
        };
        let suites = [
            written,
            Suite::named("coverage"),
            Suite::named("doctest"),
            Suite::named("mutants"),
        ];
        assert_eq!(
            archive_sources(&suites, Some(PathBuf::from("o/outcomes.json"))),
            vec![
                (
                    "junit-nextest-unit.xml".to_owned(),
                    Some(PathBuf::from("a/junit-nextest-unit.xml"))
                ),
                ("junit-coverage.xml".to_owned(), None),
                (
                    "outcomes.json".to_owned(),
                    Some(PathBuf::from("o/outcomes.json"))
                ),
            ]
        );
        assert!(archive_sources(&[Suite::named("nextest-integration")], None).len() == 1);
        assert!(archive_sources(&[Suite::named("doctest")], None).is_empty());
        let browser = Suite {
            artifact: Some("w/pw.json".to_owned()),
            ..Suite::named("playwright")
        };
        assert_eq!(
            archive_sources(&[browser, Suite::named("playwright")], None),
            vec![
                (
                    "junit-playwright.xml".to_owned(),
                    Some(PathBuf::from("w/pw-junit.xml"))
                ),
                ("junit-playwright.xml".to_owned(), None),
            ]
        );
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
            assert_eq!(Selection::from_flags(named.clone(), false), named);
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
}
