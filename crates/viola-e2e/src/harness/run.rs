//! `run`: the built suites (unit · integration · doctest · coverage · fuzz-replay) and the per-chunk
//! mutation gate, one JSON summary, merged by suite into
//! `target/agent-run/artifacts/run-summary.json` (test-plan §3 `run`).

mod coverage;
mod doctest;
mod fuzz;
mod mutants;
mod nextest;

use std::fs;
use std::io::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::{Outcome, Workspace, read_json, write_json};

pub use coverage::{COVERAGE_FLOORS, COVERAGE_IGNORE};
pub use doctest::parse_doctest;
pub use fuzz::{FUZZ_HOST_SUPPORTED, fuzz_channel};
pub use mutants::{
    chunk_diff, commit_exists, diff_files, diff_paths, leg_verdict, leg_verdict_path,
    mutants_exit_reason, mutants_suite, resolve_base, rust_delta,
};
pub use nextest::parse_junit;

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
    let mut suites = test_suites(ws, sel, filter, runner);
    let (refusal, mutants_doc) = tool_arms(ws, sel, chunk_base, leg, runner, &mut suites);
    let deferred = |s: &Suite| leg.is_some() && s.suite == "mutants" && s.failed == s.survived;
    let ok = refusal.is_none() && suites.iter().all(|s| s.green() || deferred(s));
    let _ = merge_summary(&ws.artifacts().join("run-summary.json"), &suites);
    Outcome::new(document(ok, refusal, &suites, mutants_doc), ok)
}

/// The nextest runs (or the one instrumented run that replaces them), then the doctests.
fn test_suites(
    ws: &Workspace,
    sel: Selection,
    filter: Option<&str>,
    runner: &mut Runner<'_>,
) -> Vec<Suite> {
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
    suites
}

/// Fuzz replay, then the mutation gate unless a refusal came first.
fn tool_arms(
    ws: &Workspace,
    sel: Selection,
    chunk_base: Option<String>,
    leg: Option<&str>,
    runner: &mut Runner<'_>,
    suites: &mut Vec<Suite>,
) -> (Option<Refusal>, Option<Value>) {
    let mut refusal = None;
    if sel.fuzz_replay {
        match fuzz::fuzz_replay(ws, runner) {
            Ok(suite) => suites.push(suite),
            Err(r) => refusal = Some(r),
        }
    }
    let mut mutants_doc = None;
    if sel.mutants && refusal.is_none() {
        match mutants::mutants(ws, chunk_base, leg, runner) {
            Ok((suite, doc)) => {
                suites.push(suite);
                mutants_doc = Some(doc);
            }
            Err(reason) => refusal = Some(Refusal::new(&reason, None)),
        }
    }
    (refusal, mutants_doc)
}

/// Key order is the contract: `v, cmd, ok`, then `reason`, `detail`, `suites`, `mutants`.
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

    /// A throwaway one-crate workspace in its own git repo: never this workspace, never nested in it.
    pub(super) fn mini(lib: &str) -> (tempfile::TempDir, Workspace) {
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
    use super::test_support::{GOOD_LIB, flags, mini, suite};
    use super::*;

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
