//! The one instrumented run that replaces the unit and integration nextest runs (test-plan §3
//! `--coverage`).

use std::fs;
use std::process::Command;

use super::nextest::{junit_source, junit_suite};
use super::{Runner, Suite, Workspace};

/// test-plan §10's excluded trees, with a separator class: llvm-cov reports Windows paths with `\`.
pub const COVERAGE_IGNORE: &str =
    r"(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])";

/// test-plan §10 Comprehensive: line, function and region floors, per OS.
pub const COVERAGE_FLOORS: [(&str, f64); 3] =
    [("lines", 85.0), ("functions", 95.0), ("regions", 80.0)];

/// llvm-cov builds in its own target dir, so the running harness is never relinked.
pub(super) fn coverage(ws: &Workspace, filter: Option<&str>, runner: &mut Runner<'_>) -> Suite {
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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;

    use super::super::test_support::{Calls, args_of, has, scratch, suite};
    use super::super::{Outcome, Selection, Workspace, run_with};
    use super::COVERAGE_IGNORE;

    const GREEN_JUNIT: &str = "<testsuites><testcase name=\"a\" classname=\"c\"/>\
        <testcase name=\"b\" classname=\"c\"/></testsuites>";
    const DOCTEST_OK: &str = "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured\n";

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
}
