//! `pre-push`: the local Linux gate an operator pass runs before its push (test-plan §3 Internal
//! harness subcommands). From this Windows host it syncs a history-carrying clone in WSL2 `Ubuntu`
//! from the working tree, runs the ubuntu test job's suites there and the windows test job's
//! coverage suites here: `tools → sync → cache → linux-tests → vm-release → windows-tests`. Stages
//! run in order and stop at the first red; one document names the stage it reached. Runner-speed
//! timeouts are CI's alone: no local stage can reproduce them.

use std::path::Path;
use std::process::Command;

use serde_json::{Map, Value, json};

use super::gate::gate;
use super::run::{Runner, Selection, run_with};
use super::{Outcome, Workspace};

mod linux;

use linux::{Linux, cache, distro_home, linux_tests, summary, sync, target_bytes, tools};
pub use linux::{ci_pins, last_document, wsl_path};

/// WSL2 is a Windows host's. A const, not a fn: a fn body equal to one OS's answer is an
/// unkillable mutant on that OS's leg.
pub const PRE_PUSH_HOST_SUPPORTED: bool = cfg!(windows);

/// The clone's `target/` above this is removed before a run: the distro's vhdx never shrinks, so the
/// cap bounds its peak.
pub const CACHE_CAP_BYTES: u64 = 40 * 1024 * 1024 * 1024;

const WSL: &str = "wsl.exe";
const DISTRO: &str = "Ubuntu";
/// The host stages' `CARGO_BUILD_JOBS` (half the host's 32 threads): fewer parallel rustc and link
/// processes lower the memory peak — a hypothesis, measured per run.
const HOST_BUILD_JOBS: &str = "16";

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
    pre_push_with(ws, supported, wsl_path, runner)
}

pub fn pre_push_with(
    ws: &Workspace,
    supported: bool,
    to_linux: fn(&Path) -> Option<String>,
    runner: &mut Runner<'_>,
) -> Outcome {
    let mut doc = Doc::default();
    if !supported {
        doc.stage = "host";
        return doc.finish(Err(Stop::new("pre-push-windows-only", "")), 2);
    }
    let verdict = stages(ws, to_linux, runner, &mut doc);
    doc.finish(verdict, 1)
}

/// The document as the stages fill it; key order is `v, cmd, ok, reason, detail, stage`, then the
/// sections in stage order.
#[derive(Default)]
struct Doc {
    stage: &'static str,
    sync: Option<Value>,
    cache: Option<Value>,
    linux: Map<String, Value>,
    vm: Option<Value>,
    windows: Map<String, Value>,
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
        for (key, section) in [("sync", self.sync), ("cache", self.cache)] {
            if let Some(section) = section {
                doc[key] = section;
            }
        }
        if !self.linux.is_empty() {
            doc["linux"] = Value::Object(self.linux);
        }
        if let Some(vm) = self.vm {
            doc["vm"] = vm;
        }
        if !self.windows.is_empty() {
            doc["windows"] = Value::Object(self.windows);
        }
        Outcome { doc, code }
    }
}

/// `Ok(false)`: a stage ran and read red; `Err`: a stage could not run.
fn stages(
    ws: &Workspace,
    to_linux: fn(&Path) -> Option<String>,
    runner: &mut Runner<'_>,
    doc: &mut Doc,
) -> Result<bool, Stop> {
    doc.stage = "tools";
    let linux = Linux::new(&distro_home(runner)?);
    tools(ws, &linux, runner)?;
    doc.stage = "sync";
    doc.sync = Some(sync(ws, &linux, to_linux, runner)?);
    doc.stage = "cache";
    doc.cache = Some(cache(&linux, runner)?);
    doc.stage = "linux-tests";
    let tests = linux_tests(&linux, runner, doc);
    let bytes_after = target_bytes(&linux, runner);
    if let Some(cache) = doc.cache.as_mut() {
        cache["bytes_after"] = json!(bytes_after);
    }
    if !tests? {
        return Ok(false);
    }
    doc.stage = "vm-release";
    doc.vm = Some(release_vm(runner));
    let mut capped = |cmd: &mut Command| {
        cmd.env("CARGO_BUILD_JOBS", HOST_BUILD_JOBS);
        runner(cmd)
    };
    let runner: &mut Runner<'_> = &mut capped;
    doc.stage = "windows-tests";
    Ok(windows_tests(ws, runner, doc))
}

/// The host's free physical memory in KiB, read from the OS; `None` when unreadable.
fn host_free_kib(runner: &mut Runner<'_>) -> Option<u64> {
    let mut cmd = Command::new("powershell.exe");
    cmd.args([
        "-NoProfile",
        "-Command",
        "(Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory",
    ]);
    out(runner, cmd).and_then(|text| text.trim().parse().ok())
}

/// The Linux side is done and its verdict is on the host: the VM is stopped so the host stages link
/// with its memory back (measured: an idle VM kept ~21 GB while the host's links failed with
/// 0xc0000142). Its ext4 disk, and the clone's build cache on it, survive.
fn release_vm(runner: &mut Runner<'_>) -> Value {
    let free_kib_before = host_free_kib(runner);
    let mut terminate = Command::new(WSL);
    terminate.args(["--terminate", DISTRO]);
    let terminated = out(runner, terminate).is_some();
    let free_kib_after = host_free_kib(runner);
    json!({
        "terminated": terminated,
        "free_kib_before": free_kib_before,
        "free_kib_after": free_kib_after,
    })
}

/// `run --coverage` then `gate --require coverage,doctest` on this host: the windows test job's own
/// suites and floors.
fn windows_tests(ws: &Workspace, runner: &mut Runner<'_>, doc: &mut Doc) -> bool {
    let coverage = Selection {
        coverage: true,
        ..Selection::default()
    };
    let run = run_with(ws, coverage, None, None, false, runner);
    doc.windows.insert("run".to_owned(), summary(&run.doc));
    if run.code != 0 {
        return false;
    }
    let required = ["coverage".to_owned(), "doctest".to_owned()];
    let floors = gate(&ws.artifacts(), &required);
    doc.windows.insert("gate".to_owned(), summary(&floors.doc));
    floors.code == 0
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;

    pub(super) const HOME: &str = "/home/tester";
    pub(super) const CI_LINE: &str = "env:\n  NODE_PIN_VERSION: \"24.21.0\"\n      - uses: x\n        with:\n          tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0,cargo-llvm-cov@0.9.1\n";

    pub(super) fn same(path: &Path) -> Option<String> {
        Some(path.to_string_lossy().into_owned())
    }

    pub(super) fn git_in(dir: &Path, args: &[&str]) -> String {
        let out = Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(["-c", "user.name=t", "-c", "user.email=t@example.com"])
            .args(args)
            .output()
            .expect("git");
        assert!(out.status.success(), "git {args:?}");
        String::from_utf8_lossy(&out.stdout).trim().to_owned()
    }

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
        /// Each host `cargo` call's `CARGO_BUILD_JOBS`, in call order.
        cargo_jobs: Vec<Option<String>>,
        /// The host coverage run's stand-in, once `root` is known: what `cargo llvm-cov` leaves.
        host_coverage: Option<Coverage>,
        root: Option<PathBuf>,
    }

    #[derive(Clone, Copy)]
    enum Coverage {
        Green,
        Failing,
        BelowFloor,
    }

    const DOCTEST_OK: &str = "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured\n";

    impl Fake {
        pub(super) fn new() -> Self {
            Self {
                answers: Vec::new(),
                calls: Vec::new(),
                claude_envs: 0,
                cargo_jobs: Vec::new(),
                host_coverage: None,
                root: None,
            }
        }

        fn host(mut self, coverage: Coverage) -> Self {
            self.host_coverage = Some(coverage);
            self
        }

        /// `cargo llvm-cov nextest` writes its JUnit report, `cargo llvm-cov report` its summary.
        fn coverage(&self, line: &str, last_arg: Option<&str>) -> Option<(Option<i32>, String)> {
            let (Some(coverage), Some(root)) = (self.host_coverage, &self.root) else {
                return None;
            };
            if line.starts_with("cargo llvm-cov nextest") {
                let failing = matches!(coverage, Coverage::Failing);
                let case = if failing {
                    "<testcase name=\"a\" classname=\"c\"><failure message=\"x\"/></testcase>"
                } else {
                    "<testcase name=\"a\" classname=\"c\"/>"
                };
                let junit = root.join("target/nextest/ci/junit.xml");
                fs::create_dir_all(junit.parent().expect("parent")).expect("mkdir");
                fs::write(junit, format!("<testsuites>{case}</testsuites>")).expect("junit");
                return Some((Some(if failing { 100 } else { 0 }), String::new()));
            }
            if line.starts_with("cargo llvm-cov report") {
                let pct = if matches!(coverage, Coverage::BelowFloor) {
                    50.0
                } else {
                    100.0
                };
                let totals = json!({"data": [{"totals": {
                    "lines": {"percent": pct}, "functions": {"percent": pct},
                    "regions": {"percent": pct},
                }}]});
                let path = last_arg.expect("output path");
                fs::write(path, totals.to_string()).expect("summary");
                return Some((Some(0), String::new()));
            }
            None
        }

        pub(super) fn on(mut self, patterns: &[&str], code: i32, out: &str) -> Self {
            let patterns = patterns.iter().map(|p| (*p).to_owned()).collect();
            self.answers.push((patterns, code, out.to_owned()));
            self
        }

        /// Every stage green.
        pub(super) fn green(self) -> Self {
            let coverage = self.host_coverage.unwrap_or(Coverage::Green);
            self.on(&["printenv HOME"], 0, &format!("{HOME}\n"))
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
                .on(&["rev-parse HEAD"], 0, "h1\n")
                .on(&["rev-parse FETCH_HEAD"], 0, "h1\n")
                .on(&["write-tree"], 0, "t1\n")
                .on(&["du -sb"], 0, "10\t/x\n")
                .on(&["run --coverage"], 0, &green_doc("coverage"))
                .on(&["run --browser"], 0, &green_doc("playwright"))
                .on(
                    &["gate --require coverage,doctest"],
                    0,
                    "{\"v\":1,\"cmd\":\"gate\",\"ok\":true,\"breaches\":[]}\n",
                )
                .on(&["cargo test --workspace --doc"], 0, DOCTEST_OK)
                .on(&["FreePhysicalMemory"], 0, "1048576\n")
                .host(coverage)
        }

        fn answer(&mut self, cmd: &mut Command) -> (Option<i32>, String) {
            let line: Vec<String> = std::iter::once(cmd.get_program())
                .chain(cmd.get_args())
                .map(|a| a.to_string_lossy().into_owned())
                .collect();
            let last_arg = line.last().cloned();
            let line = line.join(" ");
            self.claude_envs += cmd
                .get_envs()
                .filter(|(k, _)| k.to_string_lossy().starts_with("CLAUDE"))
                .count();
            if cmd.get_program() == "cargo" {
                let jobs = cmd
                    .get_envs()
                    .find(|(k, _)| *k == "CARGO_BUILD_JOBS")
                    .and_then(|(_, v)| v.map(|v| v.to_string_lossy().into_owned()));
                self.cargo_jobs.push(jobs);
            }
            self.calls.push(line.clone());
            if let Some(answer) = self.coverage(&line, last_arg.as_deref()) {
                return answer;
            }
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
             \"failed\":0,\"skipped\":0,\"survived\":0,\"artifact\":\"{HOME}/viola-pre-push/target/x.xml\",\
             \"failures\":[]}}]}}\n"
        )
    }

    pub(super) fn red_tests() -> Fake {
        let red = format!(
            "noise\n{{\"v\":1,\"cmd\":\"run\",\"ok\":false,\"suites\":[{{\"suite\":\"coverage\",\"passed\":2,\
             \"failed\":1,\"skipped\":0,\"survived\":0,\"artifact\":\"{HOME}/viola-pre-push/target/x.xml\",\
             \"failures\":[\"pre_push_planted_unix_red\"]}}]}}\n"
        );
        Fake::new().on(&["run --coverage"], 1, &red)
    }

    pub(super) fn drive(ws: &Workspace, fake: &mut Fake) -> Outcome {
        fake.root = Some(ws.root.clone());
        pre_push_with(ws, true, same, &mut |c| fake.answer(c))
    }

    /// The VM is stopped only after the ubuntu gate's verdict is on the host, and before any host
    /// stage; the host's free memory is recorded on both sides of it.
    #[test]
    fn pre_push_stops_the_vm_after_the_linux_tests_and_before_the_host_stages() {
        let (_tmp, ws) = pinned();
        let mut fake = Fake::new().green();
        let out = drive(&ws, &mut fake);
        let at = |needle: &str| {
            fake.calls
                .iter()
                .position(|c| c.contains(needle))
                .unwrap_or_else(|| panic!("{needle} in {:?}", fake.calls))
        };
        let linux_gate = at("gate --require coverage,doctest,playwright");
        let terminate = at("wsl.exe --terminate Ubuntu");
        let host = at("cargo llvm-cov nextest");
        assert!(
            linux_gate < terminate && terminate < host,
            "{:?}",
            fake.calls
        );
        assert!(
            !fake.calls[terminate + 1..]
                .iter()
                .any(|c| c.starts_with("wsl.exe -d")),
            "the distro was used again after the stop"
        );
        assert_eq!(
            out.doc["vm"],
            json!({"terminated": true, "free_kib_before": 1_048_576, "free_kib_after": 1_048_576})
        );
    }

    /// Every host cargo call of the host stages runs capped; the Linux side's harness calls are
    /// WSL calls and carry no host cargo at all.
    #[test]
    fn pre_push_caps_the_host_stages_build_jobs() {
        let (_tmp, ws) = pinned();
        let mut fake = Fake::new().green();
        drive(&ws, &mut fake);
        assert!(!fake.cargo_jobs.is_empty());
        assert!(
            fake.cargo_jobs.iter().all(|j| j.as_deref() == Some("16")),
            "{:?}",
            fake.cargo_jobs
        );
    }

    /// The windows test job's suites run on the host after the Linux tests and end the pass: its
    /// reduced documents sit under `windows`, the last section, and no mutation run is made.
    #[test]
    fn pre_push_windows_tests_run_after_the_linux_tests_and_end_the_pass() {
        let (_tmp, ws) = pinned();
        let mut fake = Fake::new().green();
        let out = drive(&ws, &mut fake);
        assert_eq!(out.code, 0, "{}", out.doc);
        assert_eq!(out.doc["ok"], true, "{}", out.doc);
        assert_eq!(out.doc["stage"], "windows-tests", "{}", out.doc);
        assert_eq!(out.doc["windows"]["run"]["ok"], true, "{}", out.doc);
        assert_eq!(out.doc["windows"]["run"]["suites"][0]["suite"], "coverage");
        assert_eq!(out.doc["windows"]["gate"]["ok"], true, "{}", out.doc);
        let at = |needle: &str| {
            fake.calls
                .iter()
                .position(|c| c.contains(needle))
                .unwrap_or_else(|| panic!("{needle} in {:?}", fake.calls))
        };
        assert!(at("gate --require coverage,doctest,playwright") < at("cargo llvm-cov nextest"));
        assert!(!fake.calls[at("cargo llvm-cov nextest")].starts_with(WSL));
        assert!(!fake.calls.iter().any(|c| c.contains("--mutants")));
        let keys: Vec<&String> = out.doc.as_object().expect("doc").keys().collect();
        assert_eq!(
            keys,
            [
                "v", "cmd", "ok", "stage", "sync", "cache", "linux", "vm", "windows"
            ],
            "{keys:?}"
        );
    }

    /// A red windows test stage ends the pass red at `windows-tests`, with no refusal reason.
    #[test]
    fn pre_push_windows_tests_red_is_red_at_windows_tests() {
        for (coverage, section) in [(Coverage::Failing, "run"), (Coverage::BelowFloor, "gate")] {
            let (_tmp, ws) = pinned();
            let mut fake = Fake::new().host(coverage).green();
            let out = drive(&ws, &mut fake);
            assert_eq!(out.code, 1, "{}", out.doc);
            assert_eq!(out.doc["ok"], false, "{}", out.doc);
            assert_eq!(out.doc["stage"], "windows-tests", "{}", out.doc);
            assert_eq!(out.doc["windows"][section]["ok"], false, "{}", out.doc);
            assert!(out.doc.get("reason").is_none());
        }
    }

    #[test]
    fn pre_push_host_gate_is_a_windows_const() {
        assert_eq!(PRE_PUSH_HOST_SUPPORTED, std::env::consts::OS == "windows");
    }

    #[test]
    fn pre_push_refuses_off_windows_host() {
        let (_tmp, ws) = pinned();
        let mut fake = Fake::new();
        let out = pre_push_with(&ws, false, same, &mut |c| fake.answer(c));
        assert_eq!(out.code, 2);
        assert_eq!(out.doc["reason"], "pre-push-windows-only");
        assert_eq!(out.doc["stage"], "host");
        assert!(out.doc.get("detail").is_none());
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

    /// Neither the distro home nor the repo reaches the document, red at the Linux tests or green
    /// through the host stages.
    #[test]
    fn pre_push_document_carries_no_absolute_path() {
        let (_tmp, ws) = pinned();
        let mut fake = red_tests().green();
        let text = drive(&ws, &mut fake).doc.to_string();
        assert!(!text.contains(HOME), "{text}");
        assert!(!text.contains(&*ws.root.to_string_lossy()), "{text}");
        assert!(!text.contains("x.xml"), "{text}");

        let (_tmp, ws) = pinned();
        let mut fake = Fake::new().green();
        let doc = drive(&ws, &mut fake).doc;
        assert_eq!(doc["stage"], "windows-tests", "{doc}");
        let text = doc.to_string();
        let parent = ws
            .root
            .parent()
            .expect("parent")
            .to_string_lossy()
            .into_owned();
        assert!(!text.contains(HOME), "{text}");
        assert!(!text.contains(&parent), "{text}");
        assert!(!text.contains("x.xml"), "{text}");
    }
}
