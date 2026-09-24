//! `gate`: the one verdict that closes every CI job (test-plan §3 Internal harness subcommands,
//! §10). It reads the artifacts `run` left in `target/agent-run/artifacts/` and names every breach;
//! a missing artifact is a breach, never a pass. A breach carries a fixed code and never file content.

use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use super::run::{COVERAGE_FLOORS, leg_verdict_path};
use super::{Outcome, read_json, valid_session_id};

/// The closed `suite` values (test-plan §3 Closed enums).
pub const SUITES: [&str; 9] = [
    "nextest-unit",
    "nextest-integration",
    "doctest",
    "nextest-e2e",
    "playwright",
    "mutants",
    "coverage",
    "perf",
    "fuzz-replay",
];

/// The suites whose run leaves a `junit-<suite>.xml` beside the summary.
const JUNIT: [&str; 5] = [
    "nextest-unit",
    "nextest-integration",
    "nextest-e2e",
    "coverage",
    "playwright",
];

/// test-plan §10: the provisional spine-hook deadline until architecture names the constant.
pub const SPINE_DEADLINE_S: f64 = 1.0;

/// A comma list of closed suite values; `None` when it is empty or names anything else.
pub fn parse_require(list: &str) -> Option<Vec<String>> {
    let suites: Vec<String> = list.split(',').map(|s| s.trim().to_owned()).collect();
    suites
        .iter()
        .all(|s| SUITES.contains(&s.as_str()))
        .then_some(suites)
}

/// A comma list of leg names, each held to the session-id charset (it names a file).
pub fn parse_legs(list: &str) -> Option<Vec<String>> {
    let legs: Vec<String> = list.split(',').map(|s| s.trim().to_owned()).collect();
    legs.iter().all(|l| valid_session_id(l)).then_some(legs)
}

fn breach(gate: &str, suite: &str, detail: String) -> Value {
    json!({"gate": gate, "suite": suite, "detail": detail})
}

pub fn gate(artifacts: &Path, require: &[String], legs: Option<&[String]>) -> Outcome {
    let summary = read_json::<Value>(&artifacts.join("run-summary.json")).ok();
    let mut breaches = Vec::new();
    for suite in require {
        let suite = suite.as_str();
        match (suite, legs) {
            ("mutants", Some(legs)) => union(artifacts, legs, &mut breaches),
            _ => summary_checks(summary.as_ref(), suite, &mut breaches),
        }
        if JUNIT.contains(&suite) && !artifacts.join(format!("junit-{suite}.xml")).is_file() {
            breaches.push(breach(
                "artifact-missing",
                suite,
                format!("junit-{suite}.xml"),
            ));
        }
        if suite == "coverage" {
            coverage(artifacts, &mut breaches);
        }
        if suite == "perf" {
            perf(artifacts, &mut breaches);
        }
    }
    let ok = breaches.is_empty();
    Outcome::new(
        json!({"v": 1, "cmd": "gate", "ok": ok, "breaches": breaches}),
        ok,
    )
}

fn count(entry: &Value, key: &str) -> u64 {
    entry[key].as_u64().unwrap_or(0)
}

fn summary_checks(summary: Option<&Value>, suite: &str, breaches: &mut Vec<Value>) {
    let Some(summary) = summary else {
        breaches.push(breach("suite-missing", suite, "no-run-summary".to_owned()));
        return;
    };
    let entry = summary["suites"]
        .as_array()
        .and_then(|s| s.iter().find(|e| e["suite"] == suite));
    let Some(entry) = entry else {
        breaches.push(breach("suite-missing", suite, "absent".to_owned()));
        return;
    };
    for (gate, key) in [("suite-failed", "failed"), ("suite-skipped", "skipped")] {
        let n = count(entry, key);
        if n > 0 {
            breaches.push(breach(gate, suite, format!("{key} {n}")));
        }
    }
    let survived = count(entry, "survived");
    if suite == "mutants" && survived > 0 {
        breaches.push(breach("mutants", suite, format!("survived {survived}")));
    }
}

/// Every leg's verdict must exist; a mutant is red only when no leg caught it and some leg missed
/// it or timed out. One leg cannot kill a body `#[cfg]`-gated to another OS.
fn union(artifacts: &Path, legs: &[String], breaches: &mut Vec<Value>) {
    let mut verdicts = Vec::new();
    for leg in legs {
        let path = leg_verdict_path(artifacts, leg);
        match read_json::<Value>(&path) {
            Ok(v) => verdicts.push(v),
            Err(_) => breaches.push(breach(
                "artifact-missing",
                "mutants",
                format!("mutants-verdict-{leg}.json"),
            )),
        }
    }
    if verdicts.len() != legs.len() {
        return;
    }
    let mut names: Vec<&str> = verdicts
        .iter()
        .flat_map(|v| v["mutants"].as_array().into_iter().flatten())
        .filter_map(|m| m["name"].as_str())
        .collect();
    names.sort_unstable();
    names.dedup();
    for name in names {
        let outcomes: Vec<&str> = verdicts
            .iter()
            .flat_map(|v| v["mutants"].as_array().into_iter().flatten())
            .filter(|m| m["name"] == name)
            .filter_map(|m| m["outcome"].as_str())
            .collect();
        let caught = outcomes.contains(&"caught");
        let survived = outcomes.iter().any(|o| matches!(*o, "missed" | "timeout"));
        if survived && !caught {
            breaches.push(breach("mutants", "mutants", name.to_owned()));
        }
    }
}

fn coverage(artifacts: &Path, breaches: &mut Vec<Value>) {
    let Ok(summary) = read_json::<Value>(&artifacts.join("llvm-cov-summary.json")) else {
        breaches.push(breach(
            "artifact-missing",
            "coverage",
            "llvm-cov-summary.json".to_owned(),
        ));
        return;
    };
    let totals = &summary["data"][0]["totals"];
    for (metric, floor) in COVERAGE_FLOORS {
        match totals[metric]["percent"].as_f64() {
            Some(pct) if pct >= floor => {}
            Some(pct) => breaches.push(breach(
                "coverage",
                "coverage",
                format!("{metric} {pct:.2} < {floor}"),
            )),
            None => breaches.push(breach(
                "coverage",
                "coverage",
                format!("{metric} unreadable"),
            )),
        }
    }
}

/// hyperfine's `max` sample is the gated statistic (test-plan §10; obs-plan §10).
fn perf(artifacts: &Path, breaches: &mut Vec<Value>) {
    let mut files: Vec<String> = fs::read_dir(artifacts)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with("perf-") && n.ends_with(".json"))
        .collect();
    files.sort();
    if files.is_empty() {
        breaches.push(breach("artifact-missing", "perf", "perf-*.json".to_owned()));
    }
    for file in files {
        let max = read_json::<Value>(&artifacts.join(&file))
            .ok()
            .and_then(|v| v["results"][0]["max"].as_f64());
        match max {
            Some(max) if max < SPINE_DEADLINE_S => {}
            Some(max) => breaches.push(breach(
                "perf",
                "perf",
                format!("{file} max {max} >= {SPINE_DEADLINE_S}"),
            )),
            None => breaches.push(breach("perf", "perf", format!("{file} unreadable"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::harness::write_json;

    fn dir() -> tempfile::TempDir {
        tempfile::tempdir().expect("tempdir")
    }

    fn summary(dir: &Path, suites: Value) {
        write_json(
            &dir.join("run-summary.json"),
            &json!({"v": 1, "cmd": "run", "ok": true, "suites": suites}),
        )
        .expect("summary");
    }

    fn entry(suite: &str, failed: u64, skipped: u64, survived: u64) -> Value {
        json!({"suite": suite, "passed": 3, "failed": failed, "skipped": skipped,
               "survived": survived, "artifact": null, "failures": []})
    }

    fn junit(dir: &Path, suite: &str) {
        fs::write(dir.join(format!("junit-{suite}.xml")), "<testsuites/>").expect("junit");
    }

    fn cov(dir: &Path, lines: f64, functions: f64, regions: f64) {
        write_json(
            &dir.join("llvm-cov-summary.json"),
            &json!({"data": [{"totals": {
                "lines": {"percent": lines},
                "functions": {"percent": functions},
                "regions": {"percent": regions},
            }}]}),
        )
        .expect("cov");
    }

    fn req(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| (*s).to_owned()).collect()
    }

    fn gates(out: &Outcome) -> Vec<(String, String)> {
        out.doc["breaches"]
            .as_array()
            .expect("breaches")
            .iter()
            .map(|b| {
                (
                    b["gate"].as_str().expect("gate").to_owned(),
                    b["detail"].as_str().expect("detail").to_owned(),
                )
            })
            .collect()
    }

    fn pair(gate: &str, detail: &str) -> (String, String) {
        (gate.to_owned(), detail.to_owned())
    }

    #[test]
    fn gate_passes_a_clean_coverage_and_doctest_job() {
        let d = dir();
        summary(
            d.path(),
            json!([entry("coverage", 0, 0, 0), entry("doctest", 0, 0, 0)]),
        );
        junit(d.path(), "coverage");
        cov(d.path(), 85.0, 95.0, 80.0);
        let out = gate(d.path(), &req(&["coverage", "doctest"]), None);
        assert_eq!(out.code, 0, "{}", out.doc);
        assert_eq!(
            out.doc,
            json!({"v": 1, "cmd": "gate", "ok": true, "breaches": []})
        );
    }

    #[test]
    fn gate_without_a_run_summary_is_suite_missing_for_each_suite() {
        let d = dir();
        let out = gate(d.path(), &req(&["doctest", "fuzz-replay"]), None);
        assert_eq!(out.code, 1);
        assert_eq!(
            gates(&out),
            [
                pair("suite-missing", "no-run-summary"),
                pair("suite-missing", "no-run-summary")
            ]
        );
        assert_eq!(out.doc["breaches"][1]["suite"], "fuzz-replay");
    }

    #[test]
    fn gate_names_an_absent_suite_and_its_missing_junit() {
        let d = dir();
        summary(d.path(), json!([entry("doctest", 0, 0, 0)]));
        let out = gate(d.path(), &req(&["nextest-unit"]), None);
        assert_eq!(
            gates(&out),
            [
                pair("suite-missing", "absent"),
                pair("artifact-missing", "junit-nextest-unit.xml")
            ]
        );
    }

    #[test]
    fn gate_fails_failed_and_skipped_suites_playwright_included() {
        let d = dir();
        summary(
            d.path(),
            json!([
                entry("nextest-integration", 2, 0, 0),
                entry("playwright", 0, 1, 0)
            ]),
        );
        junit(d.path(), "nextest-integration");
        junit(d.path(), "playwright");
        let out = gate(d.path(), &req(&["nextest-integration", "playwright"]), None);
        assert_eq!(
            gates(&out),
            [
                pair("suite-failed", "failed 2"),
                pair("suite-skipped", "skipped 1")
            ]
        );
    }

    #[test]
    fn gate_checks_each_coverage_floor_on_its_own() {
        let d = dir();
        summary(d.path(), json!([entry("coverage", 0, 0, 0)]));
        junit(d.path(), "coverage");
        cov(d.path(), 84.99, 94.5, 79.0);
        let out = gate(d.path(), &req(&["coverage"]), None);
        assert_eq!(
            gates(&out),
            [
                pair("coverage", "lines 84.99 < 85"),
                pair("coverage", "functions 94.50 < 95"),
                pair("coverage", "regions 79.00 < 80")
            ]
        );
        write_json(
            &d.path().join("llvm-cov-summary.json"),
            &json!({"data": [{"totals": {"lines": {"percent": 90.0}}}]}),
        )
        .expect("partial");
        let partial = gate(d.path(), &req(&["coverage"]), None);
        assert_eq!(
            gates(&partial),
            [
                pair("coverage", "functions unreadable"),
                pair("coverage", "regions unreadable")
            ]
        );
    }

    #[test]
    fn gate_coverage_without_its_summary_is_artifact_missing() {
        let d = dir();
        summary(d.path(), json!([entry("coverage", 0, 0, 0)]));
        junit(d.path(), "coverage");
        let out = gate(d.path(), &req(&["coverage"]), None);
        assert_eq!(
            gates(&out),
            [pair("artifact-missing", "llvm-cov-summary.json")]
        );
    }

    #[test]
    fn gate_single_leg_mutants_read_the_summary_survivors() {
        let d = dir();
        summary(d.path(), json!([entry("mutants", 2, 0, 2)]));
        let out = gate(d.path(), &req(&["mutants"]), None);
        assert_eq!(
            gates(&out),
            [
                pair("suite-failed", "failed 2"),
                pair("mutants", "survived 2")
            ]
        );
        summary(d.path(), json!([entry("mutants", 0, 0, 0)]));
        assert_eq!(gate(d.path(), &req(&["mutants"]), None).code, 0);
    }

    fn leg(dir: &Path, name: &str, mutants: &[(&str, &str)]) {
        let list: Vec<Value> = mutants
            .iter()
            .map(|(n, o)| json!({"name": n, "outcome": o}))
            .collect();
        write_json(
            &leg_verdict_path(dir, name),
            &json!({"v": 1, "leg": name, "verdict": "counted", "mutants": list}),
        )
        .expect("leg");
    }

    #[test]
    fn gate_union_is_red_only_where_no_leg_caught_the_mutant() {
        let d = dir();
        leg(
            d.path(),
            "ubuntu",
            &[
                ("cfg-windows body", "missed"),
                ("shared", "caught"),
                ("both missed", "missed"),
                ("timed out", "timeout"),
                ("never builds", "unviable"),
            ],
        );
        leg(
            d.path(),
            "windows",
            &[
                ("cfg-windows body", "caught"),
                ("shared", "caught"),
                ("both missed", "missed"),
                ("timed out", "unviable"),
                ("never builds", "unviable"),
            ],
        );
        let legs = req(&["ubuntu", "windows"]);
        let out = gate(d.path(), &req(&["mutants"]), Some(&legs));
        assert_eq!(
            gates(&out),
            [pair("mutants", "both missed"), pair("mutants", "timed out")]
        );
        assert!(out.doc.to_string().find("run-summary").is_none());
    }

    #[test]
    fn gate_union_needs_every_named_leg() {
        let d = dir();
        leg(d.path(), "ubuntu", &[("m", "missed")]);
        let legs = req(&["ubuntu", "windows"]);
        let out = gate(d.path(), &req(&["mutants"]), Some(&legs));
        assert_eq!(
            gates(&out),
            [pair("artifact-missing", "mutants-verdict-windows.json")],
            "a partial union judges no mutant"
        );
        leg(d.path(), "windows", &[("m", "caught")]);
        assert_eq!(gate(d.path(), &req(&["mutants"]), Some(&legs)).code, 0);
    }

    #[test]
    fn gate_perf_gates_the_max_sample_against_the_spine_deadline() {
        let d = dir();
        summary(d.path(), json!([entry("perf", 0, 0, 0)]));
        let none = gate(d.path(), &req(&["perf"]), None);
        assert_eq!(gates(&none), [pair("artifact-missing", "perf-*.json")]);
        let hook = |name: &str, max: f64| {
            write_json(
                &d.path().join(name),
                &json!({"results": [{"mean": 0.1, "max": max}]}),
            )
            .expect("perf");
        };
        hook("perf-stop.json", 0.99);
        assert_eq!(gate(d.path(), &req(&["perf"]), None).code, 0);
        hook("perf-session-end.json", 1.0);
        fs::write(d.path().join("perf-zz.json"), "{}").expect("bad");
        fs::write(d.path().join("perf.txt"), "not a perf file").expect("other");
        let out = gate(d.path(), &req(&["perf"]), None);
        assert_eq!(
            gates(&out),
            [
                pair("perf", "perf-session-end.json max 1 >= 1"),
                pair("perf", "perf-zz.json unreadable")
            ]
        );
    }

    #[test]
    fn parse_require_accepts_only_closed_suites() {
        assert_eq!(
            parse_require("coverage, doctest"),
            Some(req(&["coverage", "doctest"]))
        );
        for s in SUITES {
            assert!(parse_require(s).is_some(), "{s}");
        }
        for bad in ["", "a11y", "coverage,", "unit", "coverage,bogus"] {
            assert_eq!(parse_require(bad), None, "{bad:?}");
        }
    }

    #[test]
    fn parse_legs_holds_names_to_a_file_safe_charset() {
        assert_eq!(
            parse_legs("ubuntu-latest,windows-2025"),
            Some(req(&["ubuntu-latest", "windows-2025"]))
        );
        for bad in ["", "a/b", "..", "ok,", "a b"] {
            assert_eq!(parse_legs(bad), None, "{bad:?}");
        }
    }
}
