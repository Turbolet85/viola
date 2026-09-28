//! Hook perf rows (test-plan §3 `--perf`, §10 Performance budgets; obs-plan §10): the
//! non-instrumented `target/perf` release build, one booted perf session, then hyperfine over the
//! real `viola hook` binary per row. Each row's export is `artifacts/perf-<hook>.json`; the deadline
//! verdict is `gate --require perf`, which reads them once — nothing here trims or re-runs a row.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{Value, json};

use super::super::boot::{BootOptions, DEFAULT_CLI_VERSION, InstanceSpec, boot, instance_dir};
use super::super::cleanup::{Target, cleanup};
use super::super::{Outcome, Workspace};
use super::{Refusal, Runner, Suite};

/// The four test-plan §10 rows that exist today; `pre-tool-use` joins with the dialog tier.
pub const ROWS: [&str; 4] = ["session-start", "user-prompt-submit", "stop", "session-end"];

/// The perf build's own target dir (architecture §Occupied Resources).
const TARGET_DIR: &str = "target/perf";
const INSTANCE: &str = "builder";
const CANARY: &str = "canary-chain-value-5c1e";

/// The session a perf run times against. Boot and cleanup go through this seam, so the harness's
/// own tests stand in for both and never boot a session or nest a build.
pub(super) trait PerfSession {
    fn boot(&mut self, opts: &BootOptions) -> Outcome;
    fn cleanup(&mut self, ws: &Workspace, session: &str, keep_homes: bool) -> Outcome;
}

/// The real session: `boot` and `cleanup` as the agent commands run them.
pub(super) struct Live;

impl PerfSession for Live {
    fn boot(&mut self, opts: &BootOptions) -> Outcome {
        boot(opts)
    }

    fn cleanup(&mut self, ws: &Workspace, session: &str, keep_homes: bool) -> Outcome {
        cleanup(ws, Target::Session(session), keep_homes)
    }
}

/// `AGENT_RUN_KEEP_HOMES` keeps the perf home only when it is exactly `1`, as `cleanup` reads it.
pub(super) fn keep_homes(value: Option<&str>) -> bool {
    value == Some("1")
}

/// The home a successful boot reports, else its `reason`.
fn booted_home(doc: &Value) -> Result<PathBuf, String> {
    match (doc["ok"].as_bool(), doc["home"].as_str()) {
        (Some(true), Some(home)) => Ok(PathBuf::from(home)),
        _ => Err(doc["reason"].as_str().unwrap_or("unknown").to_owned()),
    }
}

/// A synthetic payload per row: string fields only, so every sample takes the channel path and
/// never a drift report; the tests-owned canary rides the content fields.
fn payload(hook: &str) -> Value {
    match hook {
        "session-start" => json!({"hook_event_name": "SessionStart", "session_id": CANARY,
                                  "source": "startup"}),
        "user-prompt-submit" => json!({"hook_event_name": "UserPromptSubmit",
                                       "session_id": CANARY, "prompt": CANARY}),
        "stop" => json!({"hook_event_name": "Stop", "last_assistant_message": CANARY}),
        _ => json!({"hook_event_name": "SessionEnd", "reason": "exit"}),
    }
}

/// `event:"panic"` lines in the home's codes-only role files (`detail-*` excluded). No seam
/// exemption: a perf session never sets the panic seam.
fn role_panics(home: &Path) -> usize {
    fs::read_dir(home.join("diagnostics"))
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "ndjson"))
        .filter(|p| {
            p.file_name()
                .is_some_and(|n| !n.to_string_lossy().starts_with("detail-"))
        })
        .filter_map(|p| fs::read_to_string(p).ok())
        .map(|text| {
            text.lines()
                .filter_map(|l| serde_json::from_str::<Value>(l).ok())
                .filter(|v| v["event"] == "panic")
                .count()
        })
        .sum()
}

/// Repo-relative, so hyperfine's export carries no host path; `-N` splits the command on
/// whitespace, and this path holds none.
fn viola_exe() -> String {
    format!("{TARGET_DIR}/release/viola{}", std::env::consts::EXE_SUFFIX)
}

/// One hyperfine run of `viola hook <hook>` against the booted instance.
fn row_command(ws: &Workspace, session: &str, hook: &str, dir: &Path) -> Command {
    let mut cmd = Command::new("hyperfine");
    cmd.args(["-N", "--warmup", "3", "--runs", "30", "--input"])
        .arg(format!("target/agent-run/{session}/payload-{hook}.json"))
        .arg("--export-json")
        .arg(format!("target/agent-run/artifacts/perf-{hook}.json"))
        .arg(format!("{} hook {hook}", viola_exe()))
        .env("VIOLA_NAME", INSTANCE)
        .env("VIOLA_DIR", dir)
        .current_dir(&ws.root);
    cmd
}

fn fail(suite: &mut Suite, name: &str) {
    suite.failed += 1;
    suite.failures.push(name.to_owned());
}

fn check(suite: &mut Suite, ok: bool, name: &str) {
    if ok {
        suite.passed += 1;
    } else {
        fail(suite, name);
    }
}

pub(super) fn perf(
    ws: &Workspace,
    runner: &mut Runner<'_>,
    session: &mut dyn PerfSession,
    keep_homes: bool,
) -> Result<Suite, Refusal> {
    let (probe, _) = runner(
        Command::new("hyperfine")
            .arg("--version")
            .current_dir(&ws.root),
    );
    if probe != Some(0) {
        return Err(Refusal::new("tool-missing", Some("hyperfine")));
    }
    let mut suite = Suite {
        artifact: Some("target/agent-run/artifacts".to_owned()),
        ..Suite::named("perf")
    };
    let (built, _) = runner(
        Command::new("cargo")
            .args(["build", "--release", "--workspace", "--features"])
            .args(["viola/fake-agent", "--target-dir", TARGET_DIR])
            .current_dir(&ws.root),
    );
    if built != Some(0) {
        fail(&mut suite, "build");
        return Ok(suite);
    }
    let id = format!("perf-{}", std::process::id());
    let booted = session.boot(&BootOptions {
        ws: ws.clone(),
        bin_dir: ws.root.join(TARGET_DIR).join("release"),
        session: id.clone(),
        instances: vec![InstanceSpec {
            name: INSTANCE.to_owned(),
            fake_args: Vec::new(),
        }],
        cli_version: DEFAULT_CLI_VERSION.to_owned(),
        build: false,
    });
    let home = match booted_home(&booted.doc) {
        Ok(home) => home,
        Err(reason) => {
            fail(&mut suite, &format!("boot: {reason}"));
            return Ok(suite);
        }
    };
    time_rows(ws, runner, &id, &home, &mut suite);
    check(&mut suite, role_panics(&home) == 0, "panic");
    let cleaned = session.cleanup(ws, &id, keep_homes);
    check(&mut suite, cleaned.doc["ok"] == true, "cleanup");
    Ok(suite)
}

/// Each row's stale export is removed first, so `gate` never reads an earlier run's file.
fn time_rows(ws: &Workspace, runner: &mut Runner<'_>, id: &str, home: &Path, suite: &mut Suite) {
    let artifacts = ws.artifacts();
    let session_dir = ws.session_dir(id);
    let written =
        fs::create_dir_all(&artifacts).is_ok() && fs::create_dir_all(&session_dir).is_ok();
    let dir = instance_dir(home, INSTANCE);
    for hook in ROWS {
        let _ = fs::remove_file(artifacts.join(format!("perf-{hook}.json")));
        let staged = written
            && fs::write(
                session_dir.join(format!("payload-{hook}.json")),
                payload(hook).to_string(),
            )
            .is_ok();
        let (code, _) = runner(&mut row_command(ws, id, hook, &dir));
        check(suite, staged && code == Some(0), hook);
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::super::test_support::{Calls, args_of, has, scratch};
    use super::super::{Selection, run_with};
    use super::*;

    /// Records what `perf` asked of the session; `boot` answers with `home` or a failure.
    struct StandIn {
        home: PathBuf,
        boot_ok: bool,
        cleanup_ok: bool,
        booted: Vec<BootOptions>,
        cleaned: Vec<(String, bool)>,
    }

    impl StandIn {
        fn new(home: PathBuf) -> Self {
            Self {
                home,
                boot_ok: true,
                cleanup_ok: true,
                booted: Vec::new(),
                cleaned: Vec::new(),
            }
        }
    }

    impl PerfSession for StandIn {
        fn boot(&mut self, opts: &BootOptions) -> Outcome {
            self.booted.push(opts.clone());
            let doc = if self.boot_ok {
                json!({"v": 1, "cmd": "boot", "ok": true, "home": self.home})
            } else {
                json!({"v": 1, "cmd": "boot", "ok": false, "reason": "instance-exited"})
            };
            Outcome::new(doc, self.boot_ok)
        }

        fn cleanup(&mut self, _ws: &Workspace, session: &str, keep_homes: bool) -> Outcome {
            self.cleaned.push((session.to_owned(), keep_homes));
            Outcome::new(
                json!({"v": 1, "cmd": "cleanup", "ok": self.cleanup_ok}),
                self.cleanup_ok,
            )
        }
    }

    /// Exit codes by call: the version probe, the build, then a row whose event `red_row` names.
    struct Codes {
        probe: i32,
        build: i32,
        red_row: Option<&'static str>,
    }

    const GREEN: Codes = Codes {
        probe: 0,
        build: 0,
        red_row: None,
    };

    /// Each call's environment settings, by name.
    type Envs = Vec<Vec<(String, String)>>;

    fn timed(
        ws: &Workspace,
        codes: &Codes,
        session: &mut StandIn,
    ) -> (Result<Suite, Refusal>, Calls, Envs) {
        let mut calls = Vec::new();
        let mut envs = Vec::new();
        let result = {
            let mut runner = |cmd: &mut Command| {
                let args = args_of(cmd);
                let code = if has(&args, &["--version"]) {
                    codes.probe
                } else if has(&args, &["build", "--release"]) {
                    codes.build
                } else {
                    let row = args.last().cloned().unwrap_or_default();
                    i32::from(
                        codes
                            .red_row
                            .is_some_and(|r| row.ends_with(&format!(" {r}"))),
                    )
                };
                envs.push(
                    cmd.get_envs()
                        .map(|(k, v)| {
                            (
                                k.to_string_lossy().into_owned(),
                                v.map(|v| v.to_string_lossy().into_owned())
                                    .unwrap_or_default(),
                            )
                        })
                        .collect(),
                );
                calls.push(args);
                (Some(code), String::new())
            };
            perf(ws, &mut runner, session, true)
        };
        (result, calls, envs)
    }

    fn home_in(ws: &Workspace) -> PathBuf {
        let home = ws.root.join("home");
        fs::create_dir_all(home.join("diagnostics")).expect("home");
        home
    }

    #[test]
    fn perf_without_hyperfine_is_tool_missing() {
        let (_tmp, ws) = scratch();
        let mut session = StandIn::new(home_in(&ws));
        let codes = Codes {
            probe: 127,
            ..GREEN
        };
        let (result, calls, _) = timed(&ws, &codes, &mut session);
        assert_eq!(
            result.expect_err("refused"),
            Refusal::new("tool-missing", Some("hyperfine"))
        );
        assert_eq!(calls, [["--version"]]);
        assert!(session.booted.is_empty());
    }

    #[test]
    fn perf_times_every_row_on_the_perf_build_and_cleans_up() {
        let (_tmp, ws) = scratch();
        let home = home_in(&ws);
        let mut session = StandIn::new(home.clone());
        fs::create_dir_all(ws.artifacts()).expect("artifacts");
        fs::write(ws.artifacts().join("perf-stop.json"), "{}").expect("stale");
        let (result, calls, envs) = timed(&ws, &GREEN, &mut session);
        let suite = result.expect("timed");
        assert_eq!(suite.suite, "perf");
        assert_eq!(
            suite.artifact.as_deref(),
            Some("target/agent-run/artifacts")
        );
        assert_eq!((suite.passed, suite.failed), (6, 0), "{:?}", suite.failures);
        assert!(
            !ws.artifacts().join("perf-stop.json").exists(),
            "stale export removed"
        );

        let build: Vec<String> = [
            "build",
            "--release",
            "--workspace",
            "--features",
            "viola/fake-agent",
            "--target-dir",
            "target/perf",
        ]
        .map(str::to_owned)
        .to_vec();
        assert_eq!(calls[1], build);
        let opts = &session.booted[0];
        assert_eq!(
            opts.bin_dir,
            ws.root.join("target").join("perf").join("release")
        );
        assert!(!opts.build);
        assert_eq!(opts.instances.len(), 1);
        assert_eq!(opts.instances[0].name, "builder");
        assert_eq!(opts.session, format!("perf-{}", std::process::id()));

        let exe = format!("target/perf/release/viola{}", std::env::consts::EXE_SUFFIX);
        assert_eq!(calls.len(), 2 + ROWS.len());
        for (n, hook) in ROWS.iter().enumerate() {
            let expected: Vec<String> = [
                "-N".to_owned(),
                "--warmup".to_owned(),
                "3".to_owned(),
                "--runs".to_owned(),
                "30".to_owned(),
                "--input".to_owned(),
                format!("target/agent-run/{}/payload-{hook}.json", opts.session),
                "--export-json".to_owned(),
                format!("target/agent-run/artifacts/perf-{hook}.json"),
                format!("{exe} hook {hook}"),
            ]
            .to_vec();
            assert_eq!(calls[2 + n], expected);
            let env = &envs[2 + n];
            let dir = instance_dir(&home, "builder");
            assert!(env.contains(&("VIOLA_NAME".to_owned(), "builder".to_owned())));
            assert!(env.contains(&("VIOLA_DIR".to_owned(), dir.to_string_lossy().into_owned())));
            let staged = ws
                .session_dir(&opts.session)
                .join(format!("payload-{hook}.json"));
            let body: Value =
                serde_json::from_slice(&fs::read(staged).expect("payload")).expect("json");
            assert_eq!(body, payload(hook));
        }
        assert_eq!(session.cleaned, [(opts.session.clone(), true)]);
    }

    #[test]
    fn perf_payloads_carry_only_strings_and_the_canary() {
        for hook in ROWS {
            let body = payload(hook);
            let fields = body.as_object().expect("object");
            assert!(fields.values().all(Value::is_string), "{hook}");
        }
        assert_eq!(payload("session-start")["source"], "startup");
        assert_eq!(payload("user-prompt-submit")["prompt"], CANARY);
        assert_eq!(payload("stop")["last_assistant_message"], CANARY);
        assert_eq!(payload("session-end")["hook_event_name"], "SessionEnd");
    }

    #[test]
    fn perf_build_failure_is_a_red_suite_that_boots_nothing() {
        let (_tmp, ws) = scratch();
        let mut session = StandIn::new(home_in(&ws));
        let codes = Codes {
            build: 101,
            ..GREEN
        };
        let (result, calls, _) = timed(&ws, &codes, &mut session);
        let suite = result.expect("a suite");
        assert_eq!((suite.passed, suite.failed), (0, 1));
        assert_eq!(suite.failures, ["build"]);
        assert_eq!(calls.len(), 2);
        assert!(session.booted.is_empty());
    }

    #[test]
    fn perf_boot_failure_names_its_reason_and_times_nothing() {
        let (_tmp, ws) = scratch();
        let mut session = StandIn::new(home_in(&ws));
        session.boot_ok = false;
        let (result, calls, _) = timed(&ws, &GREEN, &mut session);
        let suite = result.expect("a suite");
        assert_eq!(suite.failures, ["boot: instance-exited"]);
        assert_eq!((suite.passed, suite.failed), (0, 1));
        assert_eq!(calls.len(), 2);
        assert!(session.cleaned.is_empty());
    }

    #[test]
    fn perf_red_row_panic_line_and_failed_cleanup_are_each_named() {
        let (_tmp, ws) = scratch();
        let home = home_in(&ws);
        let diag = home.join("diagnostics");
        fs::write(
            diag.join("hook-builder.ndjson"),
            "{\"event\":\"hook-invoked\"}\nnot json\n{\"event\":\"panic\"}\n",
        )
        .expect("role");
        fs::write(diag.join("detail-hook.ndjson"), "{\"event\":\"panic\"}\n").expect("detail");
        let mut session = StandIn::new(home);
        session.cleanup_ok = false;
        let codes = Codes {
            red_row: Some("stop"),
            ..GREEN
        };
        let (result, _, _) = timed(&ws, &codes, &mut session);
        let suite = result.expect("a suite");
        assert_eq!(suite.failures, ["stop", "panic", "cleanup"]);
        assert_eq!((suite.passed, suite.failed), (3, 3));
    }

    #[test]
    fn role_panics_counts_role_files_only() {
        let (_tmp, ws) = scratch();
        let home = home_in(&ws);
        assert_eq!(role_panics(&home), 0);
        let diag = home.join("diagnostics");
        let panic = "{\"event\":\"panic\"}\n";
        fs::write(diag.join("run-builder.ndjson"), panic.repeat(2)).expect("role");
        fs::write(diag.join("hook-builder.ndjson"), panic).expect("role");
        fs::write(diag.join("detail-run.ndjson"), panic).expect("detail");
        fs::write(diag.join("notes.txt"), panic).expect("other");
        assert_eq!(role_panics(&home), 3);
    }

    #[test]
    fn booted_home_reads_ok_and_home_else_the_reason() {
        let ok = json!({"ok": true, "home": "h"});
        assert_eq!(booted_home(&ok), Ok(PathBuf::from("h")));
        let failed = json!({"ok": false, "reason": "build-failed"});
        assert_eq!(booted_home(&failed), Err("build-failed".to_owned()));
        assert_eq!(booted_home(&json!({"ok": true})), Err("unknown".to_owned()));
        assert_eq!(
            booted_home(&json!({"ok": false, "home": "h"})),
            Err("unknown".to_owned())
        );
    }

    #[test]
    fn keep_homes_only_for_exactly_one() {
        assert!(keep_homes(Some("1")));
        for value in [None, Some("0"), Some(""), Some("true"), Some("11")] {
            assert!(!keep_homes(value), "{value:?}");
        }
    }

    #[test]
    fn run_perf_refusal_reaches_the_document_and_skips_mutants() {
        let (_tmp, ws) = scratch();
        let mut ran = 0;
        let sel = Selection {
            perf: true,
            mutants: true,
            ..Selection::default()
        };
        let out = run_with(&ws, sel, None, None, None, &mut |_: &mut Command| {
            ran += 1;
            (Some(127), String::new())
        });
        assert_eq!(out.code, 1);
        assert_eq!(out.doc["reason"], "tool-missing");
        assert_eq!(out.doc["detail"], "hyperfine");
        assert!(out.doc.get("mutants").is_none());
        assert_eq!(ran, 1);
    }

    #[test]
    fn perf_is_named_only() {
        let named = Selection {
            perf: true,
            ..Selection::default()
        };
        assert_eq!(Selection::from_flags(named.clone(), false), named);
        assert!(!Selection::from_flags(Selection::default(), true).perf);
        assert!(!Selection::from_flags(Selection::default(), false).perf);
    }
}
