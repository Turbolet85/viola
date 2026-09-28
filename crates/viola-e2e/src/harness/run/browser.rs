//! The browser suite (test-plan §3 `run` step 3): the committed `e2e-web/` package under its locked
//! Playwright, counted from Playwright's JSON report, its JUnit copied to `junit-playwright.xml`.

use std::fs;
use std::path::Path;
use std::process::Command;

use serde_json::Value;

use super::nextest::{exit_must_agree, parse_junit};
use super::{Refusal, Runner, Suite, Workspace};

/// npm's Windows entry point is a `.cmd` shim, which `Command` never finds from the bare name. A const,
/// not a fn: a fn body equal to one OS's answer is an unkillable mutant on that OS's leg.
const NPM: &str = if cfg!(windows) { "npm.cmd" } else { "npm" };

const SUITE: &str = "playwright";
const REPORT: &str = "pw.json";
pub(super) const JUNIT: &str = "pw-junit.xml";

/// Exit 0 only when the locked Playwright's Chromium is installed.
const CHROMIUM_PROBE: &str = "process.exit(require('fs').existsSync(\
    require('playwright-core').chromium.executablePath()) ? 0 : 1)";

/// `npm ci`, the Chromium probe, then the Playwright CLI run directly by `node` (no shell, no npx).
/// A failed install or an absent Chromium is a red suite and the `browser-missing` refusal, never a
/// skip; Playwright never runs after either.
pub(super) fn browser(ws: &Workspace, runner: &mut Runner<'_>) -> (Suite, Option<Refusal>) {
    let web = ws.root.join("e2e-web");
    let copy = ws.artifacts().join(format!("junit-{SUITE}.xml"));
    for stale in [web.join(REPORT), web.join(JUNIT), copy] {
        let _ = fs::remove_file(stale);
    }
    let (code, _) = runner(Command::new(NPM).arg("ci").current_dir(&web));
    if code != Some(0) {
        let code = code.map_or("signal".to_owned(), |c| c.to_string());
        return missing(format!("npm-ci-exit-{code}"));
    }
    let (probe, _) = runner(
        Command::new("node")
            .args(["-e", CHROMIUM_PROBE])
            .current_dir(&web),
    );
    if probe != Some(0) {
        return missing("chromium-missing".to_owned());
    }
    let (code, _) = runner(
        Command::new("node")
            .args(["node_modules/@playwright/test/cli.js", "test"])
            .current_dir(&web),
    );
    (playwright_suite(ws, &web, code), None)
}

fn missing(failure: String) -> (Suite, Option<Refusal>) {
    let suite = Suite {
        failed: 1,
        failures: vec![failure],
        ..Suite::named(SUITE)
    };
    (suite, Some(Refusal::new("browser-missing", None)))
}

/// Counts from `pw.json` `stats`: a flaky test is a failure (retries are 0, so none should exist).
fn playwright_suite(ws: &Workspace, web: &Path, code: Option<i32>) -> Suite {
    let report = web.join(REPORT);
    let stats = fs::read_to_string(&report)
        .ok()
        .and_then(|text| serde_json::from_str::<Value>(&text).ok())
        .map(|doc| doc["stats"].clone())
        .filter(Value::is_object);
    let Some(stats) = stats else {
        return Suite {
            failed: 1,
            failures: vec!["artifact-missing".to_owned()],
            ..Suite::named(SUITE)
        };
    };
    let count = |key: &str| stats[key].as_u64().unwrap_or(0);
    let mut suite = Suite {
        passed: count("expected"),
        failed: count("unexpected") + count("flaky"),
        skipped: count("skipped"),
        artifact: Some(report.to_string_lossy().into_owned()),
        ..Suite::named(SUITE)
    };
    if let Ok(xml) = fs::read_to_string(web.join(JUNIT)) {
        let _ = fs::create_dir_all(ws.artifacts());
        let _ = fs::write(ws.artifacts().join(format!("junit-{SUITE}.xml")), &xml);
        suite.failures = parse_junit(&xml).failures;
    }
    exit_must_agree(&mut suite, code, SUITE);
    suite
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    use super::super::test_support::{Calls, args_of, scratch, suite};
    use super::super::{Selection, run_with};
    use super::*;

    const RED_JUNIT: &str = r#"<testsuites><testsuite name="pipe-reachability.spec.ts">
<testcase name="pipe: the stub page shows its heading" classname="pipe-reachability.spec.ts"><failure message="x"/></testcase>
</testsuite></testsuites>"#;

    /// A stand-in for npm, node and Playwright: `ci` and `probe` are their exits; `report` is what
    /// the Playwright run writes (`pw.json`, `pw-junit.xml`) before exiting `test`.
    struct Tools<'a> {
        ci: Option<i32>,
        probe: i32,
        test: i32,
        report: Option<(&'a str, Option<&'a str>)>,
    }

    fn drive(ws: &Workspace, tools: &Tools<'_>) -> ((Suite, Option<Refusal>), Calls) {
        let web = ws.root.join("e2e-web");
        fs::create_dir_all(&web).expect("mkdir");
        let mut calls = Vec::new();
        let result = {
            let mut runner = |cmd: &mut Command| {
                let mut call = vec![cmd.get_program().to_string_lossy().into_owned()];
                call.extend(args_of(cmd));
                assert_eq!(cmd.get_current_dir(), Some(web.as_path()));
                let code = match call.get(1).map(String::as_str) {
                    Some("ci") => tools.ci,
                    Some("-e") => Some(tools.probe),
                    _ => {
                        if let Some((json, junit)) = tools.report {
                            fs::write(web.join(REPORT), json).expect("pw.json");
                            if let Some(junit) = junit {
                                fs::write(web.join(JUNIT), junit).expect("pw-junit.xml");
                            }
                        }
                        Some(tools.test)
                    }
                };
                calls.push(call);
                (code, String::new())
            };
            browser(ws, &mut runner)
        };
        (result, calls)
    }

    fn stats(expected: u64, unexpected: u64, flaky: u64, skipped: u64) -> String {
        serde_json::json!({"stats": {"expected": expected, "unexpected": unexpected,
            "flaky": flaky, "skipped": skipped}, "suites": []})
        .to_string()
    }

    fn green_tools(report: &str) -> Tools<'_> {
        Tools {
            ci: Some(0),
            probe: 0,
            test: 0,
            report: Some((
                report,
                Some("<testsuites><testcase name=\"a\"/></testsuites>"),
            )),
        }
    }

    #[test]
    fn npm_is_the_cmd_shim_only_on_windows() {
        let want = if std::env::consts::OS == "windows" {
            "npm.cmd"
        } else {
            "npm"
        };
        assert_eq!(NPM, want);
    }

    #[test]
    fn browser_runs_npm_ci_the_chromium_probe_then_the_playwright_cli() {
        let (_tmp, ws) = scratch();
        let report = stats(1, 0, 0, 0);
        let ((suite, refusal), calls) = drive(&ws, &green_tools(&report));
        assert_eq!(refusal, None);
        assert!(suite.green(), "{suite:?}");
        assert_eq!((suite.passed, suite.failed, suite.skipped), (1, 0, 0));
        let expected: Vec<Vec<String>> = vec![
            vec![NPM.to_owned(), "ci".to_owned()],
            vec![
                "node".to_owned(),
                "-e".to_owned(),
                CHROMIUM_PROBE.to_owned(),
            ],
            vec![
                "node".to_owned(),
                "node_modules/@playwright/test/cli.js".to_owned(),
                "test".to_owned(),
            ],
        ];
        assert_eq!(calls, expected);
        let web = ws.root.join("e2e-web");
        assert_eq!(
            suite.artifact.map(PathBuf::from),
            Some(web.join(REPORT)),
            "the suite's artifact is pw.json"
        );
        let copy = ws.artifacts().join("junit-playwright.xml");
        assert_eq!(
            fs::read_to_string(copy).expect("copied"),
            "<testsuites><testcase name=\"a\"/></testsuites>"
        );
    }

    #[test]
    fn browser_with_a_failed_npm_ci_is_browser_missing_and_never_runs_playwright() {
        let (_tmp, ws) = scratch();
        let report = stats(1, 0, 0, 0);
        for (ci, failure) in [(Some(1), "npm-ci-exit-1"), (None, "npm-ci-exit-signal")] {
            let tools = Tools {
                ci,
                ..green_tools(&report)
            };
            let ((suite, refusal), calls) = drive(&ws, &tools);
            assert_eq!(refusal, Some(Refusal::new("browser-missing", None)));
            assert_eq!(suite.suite, "playwright");
            assert_eq!(suite.failed, 1);
            assert_eq!(suite.failures, vec![failure]);
            assert_eq!(calls.len(), 1, "{calls:?}");
        }
    }

    #[test]
    fn browser_without_chromium_is_browser_missing_and_never_runs_playwright() {
        let (_tmp, ws) = scratch();
        let report = stats(1, 0, 0, 0);
        let tools = Tools {
            probe: 1,
            ..green_tools(&report)
        };
        let ((suite, refusal), calls) = drive(&ws, &tools);
        assert_eq!(refusal, Some(Refusal::new("browser-missing", None)));
        assert_eq!(suite.failed, 1);
        assert_eq!(suite.failures, vec!["chromium-missing"]);
        assert_eq!(calls.len(), 2, "{calls:?}");
    }

    /// Reports an earlier run left are deleted before the invocation, so a Playwright run that
    /// writes nothing reads `artifact-missing`, never the stale counts.
    #[test]
    fn browser_deletes_stale_reports_and_a_missing_report_is_artifact_missing() {
        let (_tmp, ws) = scratch();
        let web = ws.root.join("e2e-web");
        fs::create_dir_all(&web).expect("mkdir");
        fs::create_dir_all(ws.artifacts()).expect("mkdir");
        fs::write(web.join(REPORT), stats(5, 0, 0, 0)).expect("stale pw.json");
        fs::write(web.join(JUNIT), "<testsuites/>").expect("stale junit");
        let copy = ws.artifacts().join("junit-playwright.xml");
        fs::write(&copy, "<testsuites/>").expect("stale copy");
        let tools = Tools {
            report: None,
            ..green_tools("")
        };
        let ((suite, refusal), _) = drive(&ws, &tools);
        assert_eq!(refusal, None);
        assert_eq!(suite.failed, 1);
        assert_eq!(suite.passed, 0);
        assert_eq!(suite.failures, vec!["artifact-missing"]);
        assert!(!web.join(REPORT).exists() && !web.join(JUNIT).exists() && !copy.exists());
    }

    #[test]
    fn browser_counts_flaky_as_failed_and_names_failures_from_the_junit() {
        let (_tmp, ws) = scratch();
        let report = stats(2, 1, 2, 3);
        let tools = Tools {
            test: 1,
            report: Some((&report, Some(RED_JUNIT))),
            ..green_tools(&report)
        };
        let ((suite, refusal), _) = drive(&ws, &tools);
        assert_eq!(refusal, None);
        assert_eq!((suite.passed, suite.failed, suite.skipped), (2, 3, 3));
        assert_eq!(
            suite.failures,
            vec!["pipe-reachability.spec.ts pipe: the stub page shows its heading"]
        );
    }

    #[test]
    fn browser_red_exit_with_a_clean_report_is_red() {
        let (_tmp, ws) = scratch();
        let report = stats(1, 0, 0, 0);
        let tools = Tools {
            test: 1,
            ..green_tools(&report)
        };
        let ((suite, _), _) = drive(&ws, &tools);
        assert_eq!(suite.failed, 1);
        assert_eq!(suite.failures, vec!["playwright-exit-1"]);
    }

    #[test]
    fn browser_report_without_stats_is_artifact_missing() {
        let (_tmp, ws) = scratch();
        let tools = green_tools("{\"suites\": []}");
        let ((suite, _), _) = drive(&ws, &tools);
        assert_eq!(suite.failures, vec!["artifact-missing"]);
        assert!(!ws.artifacts().join("junit-playwright.xml").exists());
    }

    #[test]
    fn run_browser_refusal_reaches_the_document() {
        let (_tmp, ws) = scratch();
        fs::create_dir_all(ws.root.join("e2e-web")).expect("mkdir");
        let sel = Selection {
            browser: true,
            ..Selection::default()
        };
        let out = run_with(&ws, sel, None, None, &mut |_: &mut Command| {
            (Some(1), String::new())
        });
        assert_eq!(out.code, 1);
        assert_eq!(out.doc["ok"], false);
        assert_eq!(out.doc["reason"], "browser-missing");
        assert!(out.doc.get("detail").is_none());
        assert_eq!(suite(&out.doc, "playwright")["failed"], 1);
    }
}
