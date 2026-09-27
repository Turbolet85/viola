//! `pre-push`: the local Linux gate an operator pass runs before its push (test-plan §3 Internal
//! harness subcommands). From this Windows host it syncs a history-carrying clone in WSL2 `Ubuntu`
//! from the working tree, runs the ubuntu test job's suites and the `ubuntu-latest` mutation leg
//! there, the windows test job's coverage suites and the `windows-2025` leg here, and judges both
//! legs with CI's own union. Stages run in order and stop at the first red; one document names the
//! stage it reached. Runner-speed timeouts are CI's alone: no local stage can reproduce them.

use std::path::Path;
use std::process::Command;

use serde_json::{Map, Value, json};

use super::gate::gate;
use super::run::{Runner, Selection, host_scratch_bytes, run_with};
use super::{Outcome, Workspace};

mod linux;

use linux::{
    Linux, cache, dir_bytes, distro_home, leg, linux_leg, linux_tests, summary, sync, target_bytes,
    tools,
};
pub use linux::{ci_pins, last_document, wsl_path};

/// WSL2 is a Windows host's. A const, not a fn: a fn body equal to one OS's answer is an
/// unkillable mutant on that OS's leg.
pub const PRE_PUSH_HOST_SUPPORTED: bool = cfg!(windows);

/// The clone's `target/` above this is removed before a run: the distro's vhdx never shrinks, so the
/// cap bounds its peak (a mutation run copies `target/` once more).
pub const CACHE_CAP_BYTES: u64 = 40 * 1024 * 1024 * 1024;

const WSL: &str = "wsl.exe";
const DISTRO: &str = "Ubuntu";
const LINUX_LEG: &str = "ubuntu-latest";
/// The host stages' `CARGO_BUILD_JOBS` (half the host's 32 threads): fewer parallel rustc and link
/// processes lower the memory peak — a hypothesis, measured per run.
const HOST_BUILD_JOBS: &str = "16";
const HOST_LEG: &str = "windows-2025";

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
    legs: Map<String, Value>,
    gate: Option<Value>,
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
        if !self.legs.is_empty() {
            doc["legs"] = Value::Object(self.legs);
        }
        if let Some(gate) = self.gate {
            doc["gate"] = gate;
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
    let mut cached = cache(&linux, runner)?;
    // What an earlier windows leg left in the host scratch; the leg itself wipes it.
    cached["windows_scratch_bytes"] = json!(host_scratch_bytes(&ws.root));
    doc.cache = Some(cached);
    doc.stage = "linux-tests";
    let tests = linux_tests(&linux, runner, doc);
    let tests = tests.and_then(|green| {
        if !green {
            return Ok(false);
        }
        doc.stage = "linux-leg";
        linux_leg(ws, &linux, runner, doc)
    });
    let bytes_after = target_bytes(&linux, runner);
    let scratch_after = dir_bytes(&linux, runner, &linux.scratch);
    if let Some(cache) = doc.cache.as_mut() {
        cache["bytes_after"] = json!(bytes_after);
        cache["scratch_bytes_after"] = json!(scratch_after);
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
    if !windows_tests(ws, runner, doc) {
        return Ok(false);
    }
    doc.stage = "windows-leg";
    let host = run_with(
        ws,
        Selection {
            mutants: true,
            ..Selection::default()
        },
        None,
        None,
        Some(HOST_LEG),
        runner,
    );
    doc.legs.insert(HOST_LEG.to_owned(), leg(&host.doc));
    if let Some(cache) = doc.cache.as_mut() {
        cache["windows_scratch_bytes_after"] = json!(host_scratch_bytes(&ws.root));
    }
    if host.code != 0 {
        return Ok(false);
    }
    doc.stage = "union";
    let bases = [LINUX_LEG, HOST_LEG].map(|l| doc.legs[l]["base"].as_str().map(str::to_owned));
    if bases[0].is_none() || bases[0] != bases[1] {
        return Err(Stop::new("base-mismatch", ""));
    }
    let union = gate(
        &ws.artifacts(),
        &ws.root,
        &["mutants".to_owned()],
        Some(&[LINUX_LEG.to_owned(), HOST_LEG.to_owned()]),
    );
    doc.gate = Some(union.doc);
    Ok(union.code == 0)
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
/// suites and floors, before its mutation leg.
fn windows_tests(ws: &Workspace, runner: &mut Runner<'_>, doc: &mut Doc) -> bool {
    let coverage = Selection {
        coverage: true,
        ..Selection::default()
    };
    let run = run_with(ws, coverage, None, None, None, runner);
    doc.windows.insert("run".to_owned(), summary(&run.doc));
    if run.code != 0 {
        return false;
    }
    let required = ["coverage".to_owned(), "doctest".to_owned()];
    let floors = gate(&ws.artifacts(), &ws.root, &required, None);
    doc.windows.insert("gate".to_owned(), summary(&floors.doc));
    floors.code == 0
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::super::run::leg_verdict_path;
    use super::*;
    use crate::harness::read_json;

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

    /// A scratch root holding the two pin sources the tools stage reads. It sits one level down, so
    /// its host mutation scratch (the repo's sibling) is this test's own.
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

        /// Every stage green up to and including the Linux leg.
        pub(super) fn green(self, base: &str, linux_outcome: &str) -> Self {
            let leg = json!({"v": 1, "cmd": "run", "ok": true, "suites": [],
                "mutants": {"tested": 1, "verdict": "counted", "base": base, "leg": LINUX_LEG}});
            let verdict = json!({"v": 1, "leg": LINUX_LEG, "verdict": "counted",
                "mutants": [{"name": MUTANT, "outcome": linux_outcome}]});
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
                .on(&["--mutants --leg ubuntu-latest"], 0, &format!("{leg}\n"))
                .on(&["cat "], 0, &verdict.to_string())
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

    const MUTANT: &str = "src/lib.rs:1:26: replace a -> u32 with 0";

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

    /// The windows test job's suites run on the host after the Linux leg and before the host leg,
    /// and their reduced documents sit under `windows`.
    /// The VM is stopped only after the ubuntu verdict is on the host, and before any host stage;
    /// the host's free memory is recorded on both sides of it.
    #[test]
    fn pre_push_stops_the_vm_after_the_copy_back_and_before_the_host_stages() {
        let (_tmp, ws) = pinned();
        let mut fake = Fake::new().green("b", "caught");
        let out = drive(&ws, &mut fake);
        let at = |needle: &str| {
            fake.calls
                .iter()
                .position(|c| c.contains(needle))
                .unwrap_or_else(|| panic!("{needle} in {:?}", fake.calls))
        };
        let copy_back = at("mutants-verdict-ubuntu-latest.json");
        let terminate = at("wsl.exe --terminate Ubuntu");
        let host = at("cargo llvm-cov nextest");
        assert!(
            copy_back < terminate && terminate < host,
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
        let mut fake = Fake::new().green("b", "caught");
        drive(&ws, &mut fake);
        assert!(!fake.cargo_jobs.is_empty());
        assert!(
            fake.cargo_jobs.iter().all(|j| j.as_deref() == Some("16")),
            "{:?}",
            fake.cargo_jobs
        );
    }

    #[test]
    fn pre_push_windows_tests_run_between_the_linux_leg_and_the_windows_leg() {
        let (_tmp, ws) = pinned();
        let mut fake = Fake::new().green("b", "caught");
        let out = drive(&ws, &mut fake);
        assert_eq!(out.doc["stage"], "windows-leg", "{}", out.doc);
        assert_eq!(out.doc["windows"]["run"]["ok"], true, "{}", out.doc);
        assert_eq!(out.doc["windows"]["run"]["suites"][0]["suite"], "coverage");
        assert_eq!(out.doc["windows"]["gate"]["ok"], true, "{}", out.doc);
        let at = |needle: &str| {
            fake.calls
                .iter()
                .position(|c| c.contains(needle))
                .unwrap_or_else(|| panic!("{needle} in {:?}", fake.calls))
        };
        assert!(at("--mutants --leg ubuntu-latest") < at("cargo llvm-cov nextest"));
        assert!(!fake.calls[at("cargo llvm-cov nextest")].starts_with(WSL));
        let keys: Vec<&String> = out.doc.as_object().expect("doc").keys().collect();
        let (linux, windows, legs) = (
            keys.iter().position(|k| *k == "linux"),
            keys.iter().position(|k| *k == "windows"),
            keys.iter().position(|k| *k == "legs"),
        );
        assert!(linux < windows && windows < legs, "{keys:?}");
    }

    /// A red windows test stage stops the pass before the host mutation leg.
    #[test]
    fn pre_push_windows_tests_red_stops_before_the_windows_leg() {
        for (coverage, section) in [(Coverage::Failing, "run"), (Coverage::BelowFloor, "gate")] {
            let (_tmp, ws) = pinned();
            let mut fake = Fake::new().host(coverage).green("b", "caught");
            let out = drive(&ws, &mut fake);
            assert_eq!(out.code, 1, "{}", out.doc);
            assert_eq!(out.doc["stage"], "windows-tests", "{}", out.doc);
            assert_eq!(out.doc["windows"][section]["ok"], false, "{}", out.doc);
            assert!(out.doc.get("reason").is_none());
            assert!(out.doc["legs"].get(HOST_LEG).is_none(), "{}", out.doc);
            assert!(!fake.calls.iter().any(|c| c.starts_with("cargo mutants")));
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
        let mut fake = fake.green("b", "caught");
        let out = drive(&ws, &mut fake);
        assert_eq!(out.code, 1, "{}", out.doc);
        assert_eq!(out.doc["ok"], false);
        assert_eq!(out.doc["reason"], reason, "{}", out.doc);
        assert_eq!(out.doc["detail"].as_str(), detail, "{}", out.doc);
        assert_eq!(out.doc["stage"], stage);
        fake
    }

    /// Neither the distro home, the repo, nor the host scratch reaches the document: the scratch
    /// fields are byte counts.
    #[test]
    fn pre_push_document_carries_no_absolute_path() {
        let (_tmp, ws) = pinned();
        let mut fake = red_tests().green("b", "caught");
        let text = drive(&ws, &mut fake).doc.to_string();
        assert!(!text.contains(HOME), "{text}");
        assert!(!text.contains(&*ws.root.to_string_lossy()), "{text}");
        assert!(!text.contains("x.xml"), "{text}");

        let (_tmp, ws, flip) = leg_repo();
        let mut fake = Fake::new().green(&flip, "caught");
        let doc = with_host_leg(&ws, &mut fake, "CaughtMutant").doc;
        let text = doc.to_string();
        let parent = ws
            .root
            .parent()
            .expect("parent")
            .to_string_lossy()
            .into_owned();
        assert!(!text.contains(&parent), "{text}");
        assert!(!text.contains("viola-mutants-scratch"), "{text}");
        assert!(doc["cache"]["windows_scratch_bytes"].is_u64(), "{text}");
        assert!(
            doc["cache"]["windows_scratch_bytes_after"].is_u64(),
            "{text}"
        );
    }

    /// The host scratch is read at the cache stage (what an earlier leg left) and again after the
    /// windows leg, which wipes it on a Windows host before cargo-mutants writes into it.
    #[test]
    fn pre_push_cache_reports_the_host_scratch_before_and_after_the_windows_leg() {
        let (_tmp, ws, flip) = leg_repo();
        let scratch = ws
            .root
            .parent()
            .expect("parent")
            .join("viola-mutants-scratch");
        fs::create_dir_all(&scratch).expect("mkdir");
        fs::write(scratch.join("left"), [0u8; 9]).expect("write");
        let mut fake = Fake::new().green(&flip, "caught");
        let doc = with_host_leg(&ws, &mut fake, "CaughtMutant").doc;
        assert_eq!(doc["cache"]["windows_scratch_bytes"], 9, "{doc}");
        let after = if cfg!(windows) {
            fs::metadata(scratch.join("mutants.out").join("outcomes.json"))
                .expect("outcomes in the scratch")
                .len()
        } else {
            9
        };
        assert_eq!(doc["cache"]["windows_scratch_bytes_after"], after, "{doc}");
        assert_eq!(scratch.join("left").exists(), !cfg!(windows));
    }

    /// A git repo with a flip at HEAD and a promoted, uncommitted chunk changing Rust source: the
    /// state an operator pass runs `pre-push` in. Returns the flip.
    fn leg_repo() -> (tempfile::TempDir, Workspace, String) {
        let (tmp, ws) = pinned();
        let root = ws.root.clone();
        let master = root.join(".andromeda").join("master-route.md");
        fs::create_dir_all(master.parent().expect("dir")).expect("mkdir");
        fs::write(&master, "## p\na · complete · first · → x\n").expect("master");
        fs::create_dir_all(root.join("src")).expect("mkdir");
        fs::write(root.join("src/lib.rs"), "pub fn a() -> u32 { 1 }\n").expect("lib");
        fs::write(root.join(".gitignore"), "target/\nmutants.out*/\n").expect("ignore");
        git_in(&root, &["init", "-q"]);
        git_in(&root, &["add", "-A"]);
        git_in(&root, &["commit", "-q", "-m", "feat(a): wrap a"]);
        let flip = git_in(&root, &["rev-parse", "HEAD"]);
        fs::write(
            &master,
            "## p\na · complete · first · → x\nm · pending · second · → y\n",
        )
        .expect("promote");
        fs::write(root.join("src/lib.rs"), "pub fn a() -> u32 { 2 }\n").expect("lib");
        (tmp, ws, flip)
    }

    /// The host leg's cargo calls answered in place: the build passes and cargo-mutants leaves the
    /// given outcomes where the leg reads them (the host scratch on Windows, the repo elsewhere), as
    /// the existing `run_mutants_leg_*` tests do.
    fn with_host_leg(ws: &Workspace, fake: &mut Fake, windows: &str) -> Outcome {
        let root: PathBuf = ws.root.clone();
        let out_dir = if cfg!(windows) {
            root.parent().expect("parent").join("viola-mutants-scratch")
        } else {
            root.clone()
        }
        .join("mutants.out");
        let outcomes = format!(
            "{{\"outcomes\":[{{\"scenario\":{{\"Mutant\":{{\"name\":\"{MUTANT}\"}}}},\"summary\":\"{windows}\"}}],\
             \"caught\":0,\"missed\":1,\"timeout\":0,\"unviable\":0}}"
        );
        fake.root = Some(root.clone());
        pre_push_with(ws, true, same, &mut |c| {
            let args: Vec<String> = c
                .get_args()
                .map(|a| a.to_string_lossy().into_owned())
                .collect();
            if c.get_program() == "cargo" && args.first().is_some_and(|a| a == "mutants") {
                fs::create_dir_all(&out_dir).expect("mkdir");
                fs::write(out_dir.join("outcomes.json"), &outcomes).expect("outcomes");
                return (Some(2), String::new());
            }
            fake.answer(c)
        })
    }

    #[test]
    fn pre_push_union_reads_both_legs() {
        let (_tmp, ws, flip) = leg_repo();
        let mut fake = Fake::new().green(&flip, "missed");
        let out = with_host_leg(&ws, &mut fake, "MissedMutant");
        assert_eq!(out.code, 1, "{}", out.doc);
        assert_eq!(out.doc["stage"], "union");
        assert!(out.doc.get("reason").is_none());
        assert_eq!(out.doc["gate"]["breaches"][0]["detail"], MUTANT);
        assert_eq!(out.doc["legs"][HOST_LEG]["base"], flip.as_str());
        assert_eq!(out.doc["legs"][LINUX_LEG]["base"], flip.as_str());
        let copied: Value =
            read_json(&leg_verdict_path(&ws.artifacts(), LINUX_LEG)).expect("copied verdict");
        assert_eq!(copied["leg"], LINUX_LEG);

        let (_tmp, ws, flip) = leg_repo();
        let mut fake = Fake::new().green(&flip, "caught");
        let out = with_host_leg(&ws, &mut fake, "MissedMutant");
        assert_eq!(out.code, 0, "{}", out.doc);
        assert_eq!(out.doc["ok"], true);
        assert_eq!(out.doc["stage"], "union");
        assert_eq!(out.doc["gate"]["ok"], true);
    }

    #[test]
    fn pre_push_base_mismatch_is_red() {
        let (_tmp, ws, _flip) = leg_repo();
        let mut fake = Fake::new().green(&"0".repeat(40), "caught");
        let out = with_host_leg(&ws, &mut fake, "CaughtMutant");
        assert_eq!(out.code, 1, "{}", out.doc);
        assert_eq!(out.doc["reason"], "base-mismatch");
        assert_eq!(out.doc["stage"], "union");
        assert!(out.doc.get("gate").is_none());
    }

    #[test]
    fn pre_push_windows_leg_red_stops_before_the_union() {
        let (_tmp, ws) = pinned();
        let mut fake = Fake::new().green("b", "caught");
        let out = drive(&ws, &mut fake);
        assert_eq!(out.code, 1, "{}", out.doc);
        assert_eq!(out.doc["stage"], "windows-leg");
        assert_eq!(out.doc["legs"][HOST_LEG]["reason"], "base-missing");
        assert!(out.doc.get("gate").is_none());
    }
}
