//! `pre-push`: the local Linux gate an operator pass runs before its push (test-plan §3 Internal
//! harness subcommands). On this Linux host it runs the ubuntu test job's suites in the working tree
//! itself, each child through `env -i` with only HOME and a constant PATH: `tools → linux-tests`.
//! Stages run in order and stop at the first red; one document names the stage it reached. The
//! other OSes are CI's alone, and so are runner-speed timeouts: no local stage can reproduce them.

use std::process::Command;

use serde_json::{Map, Value, json};

use super::run::Runner;
use super::{Outcome, Workspace};

mod linux;

use linux::{Linux, linux_tests, passwd_home, tools};
pub use linux::{ci_pins, last_document};

/// The native gate is a Linux host's. A const, not a fn: a fn body equal to one OS's answer is an
/// unkillable mutant on that OS's leg.
pub const PRE_PUSH_HOST_SUPPORTED: bool = cfg!(target_os = "linux");

/// Why a stage stopped: a closed `reason` and an optional closed `detail`.
#[derive(Debug, PartialEq, Eq)]
struct Stop {
    reason: &'static str,
    detail: Option<String>,
}

impl Stop {
    fn new(reason: &'static str, detail: &str) -> Self {
        Self {
            reason,
            detail: Some(detail.to_owned()),
        }
    }
}

fn out(runner: &mut Runner<'_>, mut cmd: Command) -> Option<String> {
    match runner(&mut cmd) {
        (Some(0), text) => Some(text),
        _ => None,
    }
}

fn ok(doc: &Value) -> bool {
    doc["ok"] == true
}

pub fn pre_push(ws: &Workspace, supported: bool, runner: &mut Runner<'_>) -> Outcome {
    let mut doc = Doc::default();
    if !supported {
        doc.stage = "host";
        return doc.finish(Err(Stop::new("pre-push-linux-only", "")), 2);
    }
    let verdict = stages(ws, runner, &mut doc);
    doc.finish(verdict, 1)
}

/// The document as the stages fill it; key order is `v, cmd, ok, reason, detail, stage, linux`.
#[derive(Default)]
struct Doc {
    stage: &'static str,
    linux: Map<String, Value>,
}

impl Doc {
    fn finish(self, verdict: Result<bool, Stop>, refused: u8) -> Outcome {
        let ok = verdict == Ok(true);
        let mut doc = json!({"v": 1, "cmd": "pre-push", "ok": ok});
        let mut code = u8::from(!ok);
        if let Err(stop) = verdict {
            doc["reason"] = json!(stop.reason);
            if let Some(detail) = stop.detail.filter(|d| !d.is_empty()) {
                doc["detail"] = json!(detail);
            }
            code = refused;
        }
        doc["stage"] = json!(self.stage);
        if !self.linux.is_empty() {
            doc["linux"] = Value::Object(self.linux);
        }
        Outcome { doc, code }
    }
}

/// `Ok(false)`: a stage ran and read red; `Err`: a stage could not run.
fn stages(ws: &Workspace, runner: &mut Runner<'_>, doc: &mut Doc) -> Result<bool, Stop> {
    doc.stage = "tools";
    let linux = Linux::new(&passwd_home(runner)?, &ws.root);
    tools(ws, &linux, runner)?;
    doc.stage = "linux-tests";
    linux_tests(&linux, runner, doc)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    pub(super) const HOME: &str = "/srv/tester";
    pub(super) const CI_LINE: &str = "env:\n  NODE_PIN_VERSION: \"24.21.0\"\n      - uses: x\n        with:\n          tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0,cargo-llvm-cov@0.9.1\n";

    /// A scratch root holding the two pin sources the tools stage reads.
    pub(super) fn pinned() -> (tempfile::TempDir, Workspace) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let root = &tmp.path().join("ws");
        fs::create_dir_all(root).expect("mkdir");
        fs::write(
            root.join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"1.98.1\"\n",
        )
        .expect("toml");
        fs::create_dir_all(root.join(".github").join("workflows")).expect("mkdir");
        fs::write(
            root.join(".github").join("workflows").join("ci.yml"),
            CI_LINE,
        )
        .expect("ci");
        let ws = Workspace {
            root: root.to_path_buf(),
        };
        (tmp, ws)
    }

    /// Scripted answers by substring (every pattern must occur in the call line); the first match
    /// wins, anything else exits 0 silently. Records every call and every `CLAUDE*` env it was handed.
    pub(super) struct Fake {
        answers: Vec<(Vec<String>, i32, String)>,
        pub(super) calls: Vec<String>,
        pub(super) claude_envs: usize,
    }

    impl Fake {
        pub(super) fn new() -> Self {
            Self {
                answers: Vec::new(),
                calls: Vec::new(),
                claude_envs: 0,
            }
        }

        pub(super) fn on(mut self, patterns: &[&str], code: i32, out: &str) -> Self {
            let patterns = patterns.iter().map(|p| (*p).to_owned()).collect();
            self.answers.push((patterns, code, out.to_owned()));
            self
        }

        /// Every stage green.
        pub(super) fn green(self) -> Self {
            self.on(&["id -u"], 0, "1000\n")
                .on(
                    &["getent passwd 1000"],
                    0,
                    &format!("tester:x:1000:1000:Tester:{HOME}:/bin/bash\n"),
                )
                .on(&["cc --version"], 0, "cc 13\n")
                .on(
                    &["rustc +1.98.1 --version"],
                    0,
                    "rustc 1.98.1 (x 2026-09-01)\n",
                )
                .on(&["nextest --version"], 0, "cargo-nextest 0.9.146 (x)\n")
                .on(&["mutants --version"], 0, "cargo-mutants 27.1.0\n")
                .on(&["llvm-cov --version"], 0, "cargo-llvm-cov 0.9.1\n")
                .on(&["node --version"], 0, "v24.21.0\n")
                .on(&["run --coverage"], 0, &green_doc("coverage"))
                .on(&["run --browser"], 0, &green_doc("playwright"))
                .on(
                    &["gate --require coverage,doctest,playwright"],
                    0,
                    "{\"v\":1,\"cmd\":\"gate\",\"ok\":true,\"breaches\":[]}\n",
                )
        }

        fn answer(&mut self, cmd: &mut Command) -> (Option<i32>, String) {
            let line: Vec<String> = std::iter::once(cmd.get_program())
                .chain(cmd.get_args())
                .map(|a| a.to_string_lossy().into_owned())
                .collect();
            let line = line.join(" ");
            self.claude_envs += cmd
                .get_envs()
                .filter(|(k, _)| k.to_string_lossy().starts_with("CLAUDE"))
                .count();
            self.calls.push(line.clone());
            self.answers
                .iter()
                .find(|(pats, _, _)| pats.iter().all(|p| line.contains(p.as_str())))
                .map_or((Some(0), String::new()), |(_, code, out)| {
                    (Some(*code), out.clone())
                })
        }
    }

    fn green_doc(suite: &str) -> String {
        format!(
            "{{\"v\":1,\"cmd\":\"run\",\"ok\":true,\"suites\":[{{\"suite\":\"{suite}\",\"passed\":3,\
             \"failed\":0,\"skipped\":0,\"survived\":0,\"artifact\":\"{HOME}/viola/target/x.xml\",\
             \"failures\":[]}}]}}\n"
        )
    }

    pub(super) fn red_tests() -> Fake {
        let red = format!(
            "noise\n{{\"v\":1,\"cmd\":\"run\",\"ok\":false,\"suites\":[{{\"suite\":\"coverage\",\"passed\":2,\
             \"failed\":1,\"skipped\":0,\"survived\":0,\"artifact\":\"{HOME}/viola/target/x.xml\",\
             \"failures\":[\"pre_push_planted_unix_red\"]}}]}}\n"
        );
        Fake::new().on(&["run --coverage"], 1, &red)
    }

    pub(super) fn drive(ws: &Workspace, fake: &mut Fake) -> Outcome {
        pre_push(ws, true, &mut |c| fake.answer(c))
    }

    /// The pass ends green at `linux-tests` with the three reduced documents under `linux`, and no
    /// mutation run is made.
    #[test]
    fn pre_push_native_stages_run_tools_then_linux_tests_and_end_the_pass() {
        let (_tmp, ws) = pinned();
        let mut fake = Fake::new().green();
        let out = drive(&ws, &mut fake);
        assert_eq!(out.code, 0, "{}", out.doc);
        assert_eq!(out.doc["ok"], true, "{}", out.doc);
        assert_eq!(out.doc["stage"], "linux-tests", "{}", out.doc);
        for section in ["run", "browser", "gate"] {
            assert_eq!(out.doc["linux"][section]["ok"], true, "{}", out.doc);
        }
        let at = |needle: &str| {
            fake.calls
                .iter()
                .position(|c| c.contains(needle))
                .unwrap_or_else(|| panic!("{needle} in {:?}", fake.calls))
        };
        assert!(at("node --version") < at("run --coverage"));
        assert!(!fake.calls.iter().any(|c| c.contains("--mutants")));
        let keys: Vec<&String> = out.doc.as_object().expect("doc").keys().collect();
        assert_eq!(keys, ["v", "cmd", "ok", "stage", "linux"], "{keys:?}");
    }

    #[test]
    fn pre_push_host_gate_is_a_linux_const() {
        assert_eq!(PRE_PUSH_HOST_SUPPORTED, std::env::consts::OS == "linux");
    }

    #[test]
    fn pre_push_refuses_off_linux_host() {
        let (_tmp, ws) = pinned();
        let mut fake = Fake::new();
        let out = pre_push(&ws, false, &mut |c| fake.answer(c));
        assert_eq!(out.code, 2);
        assert_eq!(
            out.doc,
            json!({"v": 1, "cmd": "pre-push", "ok": false,
                   "reason": "pre-push-linux-only", "stage": "host"})
        );
        assert!(fake.calls.is_empty());
    }

    pub(super) fn stopped(fake: Fake, reason: &str, detail: Option<&str>, stage: &str) -> Fake {
        let (_tmp, ws) = pinned();
        let mut fake = fake.green();
        let out = drive(&ws, &mut fake);
        assert_eq!(out.code, 1, "{}", out.doc);
        assert_eq!(out.doc["ok"], false);
        assert_eq!(out.doc["reason"], reason, "{}", out.doc);
        assert_eq!(out.doc["detail"].as_str(), detail, "{}", out.doc);
        assert_eq!(out.doc["stage"], stage);
        fake
    }

    /// Neither the home nor the repo reaches the document, red at the Linux tests or green.
    #[test]
    fn pre_push_document_carries_no_absolute_path() {
        for fake in [red_tests(), Fake::new()] {
            let (_tmp, ws) = pinned();
            let mut fake = fake.green();
            let text = drive(&ws, &mut fake).doc.to_string();
            assert!(text.contains("\"stage\":\"linux-tests\""), "{text}");
            assert!(!text.contains(HOME), "{text}");
            assert!(!text.contains(&*ws.root.to_string_lossy()), "{text}");
            assert!(!text.contains("x.xml"), "{text}");
        }
    }
}
