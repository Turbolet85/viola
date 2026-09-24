//! The unit and integration nextest runs and the JUnit report they write.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use super::{Runner, Suite, Workspace};

pub(super) const UNIT: &str = "kind(lib) | kind(bin)";
// test-plan §3 also excludes the E2E binaries (`path_`, `tui_`, …); nextest rejects a `binary()`
// operator that matches no binary, so that exclusion arrives with the first E2E binary and `--e2e`.
pub(super) const INTEGRATION: &str = "kind(test)";

/// nextest keeps its store under `<workspace root>/target/nextest` whatever the target dir, and
/// under `cargo llvm-cov nextest` too (measured: llvm-cov 0.9.1, nextest 0.9.133).
pub(super) fn junit_source(ws: &Workspace) -> PathBuf {
    ws.root
        .join("target")
        .join("nextest")
        .join("ci")
        .join("junit.xml")
}

/// The suite a finished JUnit-writing run reports; the report is copied to `junit-<suite>.xml`.
pub(super) fn junit_suite(ws: &Workspace, suite: &str, code: Option<i32>, tool: &str) -> Suite {
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

pub(super) fn nextest(
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

/// A red tool exit with nothing red in its report is still red (a build failure, no tests run).
pub(super) fn exit_must_agree(suite: &mut Suite, code: Option<i32>, tool: &str) {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
