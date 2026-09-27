//! `pre-push`: the local Linux gate an operator pass runs before its push (test-plan §3 Internal
//! harness subcommands). From this Windows host it syncs a history-carrying clone in WSL2 `Ubuntu`
//! from the working tree, runs the ubuntu test job's suites and the `ubuntu-latest` mutation leg
//! there, the `windows-2025` leg here, and judges both legs with CI's own union. Stages run in order
//! and stop at the first red; one document names the stage it reached.

use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

use serde_json::{Map, Value, json};

use super::gate::gate;
use super::run::{Runner, Selection, fuzz_channel, leg_verdict_path, run_with};
use super::{Outcome, Workspace, write_json};

/// WSL2 is a Windows host's. A const, not a fn: a fn body equal to one OS's answer is an
/// unkillable mutant on that OS's leg.
pub const PRE_PUSH_HOST_SUPPORTED: bool = cfg!(windows);

/// The clone's `target/` above this is removed before a run: the distro's vhdx never shrinks, so the
/// cap bounds its peak (a mutation run copies `target/` once more).
pub const CACHE_CAP_BYTES: u64 = 40 * 1024 * 1024 * 1024;

const WSL: &str = "wsl.exe";
const DISTRO: &str = "Ubuntu";
const CLONE_DIR: &str = "viola-pre-push";
const LINUX_LEG: &str = "ubuntu-latest";
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

/// The distro side: its user's home and the clone under it. Never printed.
struct Linux {
    home: String,
    clone: String,
}

impl Linux {
    fn new(home: &str) -> Self {
        Self {
            home: home.to_owned(),
            clone: format!("{home}/{CLONE_DIR}"),
        }
    }

    /// `--exec` passes argv verbatim (the `--` form re-parses it through the distro shell), and
    /// `env -i` hands the child only HOME and a Linux PATH: the distro PATH carries the Windows one,
    /// and nothing of this process's environment crosses.
    fn cmd(&self, cd: Option<&str>, argv: &[&str]) -> Command {
        let mut cmd = Command::new(WSL);
        cmd.args(["-d", DISTRO]);
        if let Some(dir) = cd {
            cmd.args(["--cd", dir]);
        }
        cmd.args(["--exec", "/usr/bin/env", "-i"])
            .arg(format!("HOME={}", self.home))
            .arg(format!(
                "PATH={}/.cargo/bin:/usr/local/bin:/usr/bin:/bin",
                self.home
            ))
            .args(argv);
        cmd
    }

    fn git(&self, args: &[&str]) -> Command {
        let mut argv = vec!["git", "-C", self.clone.as_str()];
        argv.extend_from_slice(args);
        self.cmd(None, &argv)
    }
}

/// A drive path as the distro mounts it (`D:\a\b` → `/mnt/d/a/b`); `None` for anything else.
pub fn wsl_path(path: &Path) -> Option<String> {
    let text = path.to_string_lossy();
    let text = text.strip_prefix(r"\\?\").unwrap_or(&text);
    let mut chars = text.chars();
    let drive = chars.next().filter(char::is_ascii_alphabetic)?;
    let rest = chars.as_str().strip_prefix(':')?;
    if !(rest.is_empty() || rest.starts_with(['\\', '/'])) {
        return None;
    }
    Some(format!(
        "/mnt/{}{}",
        drive.to_ascii_lowercase(),
        rest.replace('\\', "/")
    ))
}

/// The `name@version` pins on ci.yml's `test`-job tool line (the one that also installs
/// cargo-llvm-cov): the one version source of the Linux cargo tools.
pub fn ci_pins(ci_yml: &str) -> Option<Vec<(String, String)>> {
    let line = ci_yml.lines().find_map(|l| {
        l.trim()
            .strip_prefix("tool:")
            .filter(|tools| tools.contains("cargo-llvm-cov@"))
    })?;
    line.split(',')
        .map(|pin| {
            let (name, version) = pin.trim().split_once('@')?;
            Some((name.to_owned(), version.to_owned()))
        })
        .collect()
}

/// The last non-empty stdout line, when it is a harness document.
pub fn last_document(stdout: &str) -> Option<Value> {
    stdout
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .and_then(|l| serde_json::from_str::<Value>(l).ok())
        .filter(|doc| doc["cmd"].is_string())
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
    doc.cache = Some(cache(&linux, runner)?);
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
    if let Some(cache) = doc.cache.as_mut() {
        cache["bytes_after"] = json!(bytes_after);
    }
    if !tests? {
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

/// The distro answers with its user's home, which doubles as its presence probe.
fn distro_home(runner: &mut Runner<'_>) -> Result<String, Stop> {
    let mut cmd = Command::new(WSL);
    cmd.args(["-d", DISTRO, "--exec", "/usr/bin/printenv", "HOME"]);
    out(runner, cmd)
        .map(|home| home.trim().to_owned())
        .filter(|home| home.starts_with('/'))
        .ok_or_else(|| Stop::new("tool-missing", "wsl-distro-ubuntu"))
}

/// The C linker, then every tool at its pin: `rust-toolchain.toml`'s channel and ci.yml's test-job
/// tool line, never a version written here.
fn tools(ws: &Workspace, linux: &Linux, runner: &mut Runner<'_>) -> Result<(), Stop> {
    if out(runner, linux.cmd(None, &["cc", "--version"])).is_none() {
        return Err(Stop::new("tool-missing", "cc"));
    }
    let unreadable = || Stop::new("tool-pin-mismatch", "pins-unreadable");
    let channel = fs::read_to_string(ws.root.join("rust-toolchain.toml"))
        .ok()
        .and_then(|toml| fuzz_channel(&toml))
        .ok_or_else(unreadable)?;
    let pins = fs::read_to_string(ws.root.join(".github").join("workflows").join("ci.yml"))
        .ok()
        .and_then(|yml| ci_pins(&yml))
        .ok_or_else(unreadable)?;
    let toolchain = format!("+{channel}");
    let mut wanted = vec![("rustc".to_owned(), channel.clone())];
    wanted.extend(pins);
    for (name, pin) in &wanted {
        let argv: Vec<&str> = match name.strip_prefix("cargo-") {
            Some(sub) => vec!["cargo", &toolchain, sub, "--version"],
            None => vec![name.as_str(), &toolchain, "--version"],
        };
        let Some(text) = out(runner, linux.cmd(None, &argv)) else {
            return Err(Stop::new("tool-missing", name));
        };
        let version = text
            .lines()
            .next()
            .and_then(|l| l.split_whitespace().nth(1));
        if version != Some(pin.as_str()) {
            return Err(Stop::new("tool-pin-mismatch", name));
        }
    }
    Ok(())
}

/// The working tree as one binary patch through a temporary index (the real one is never touched),
/// applied to the clone at the same HEAD; the clone's tree id must then equal the working tree's.
fn sync(
    ws: &Workspace,
    linux: &Linux,
    to_linux: fn(&Path) -> Option<String>,
    runner: &mut Runner<'_>,
) -> Result<Value, Stop> {
    let started = Instant::now();
    let failed = |step: &str| Stop::new("sync-failed", step);
    let src = to_linux(&ws.root).ok_or_else(|| failed("source-path"))?;
    let dir = ws.root.join("target").join("pre-push");
    fs::create_dir_all(&dir).map_err(|_| failed("patch"))?;
    let index = dir.join("index");
    let patch = dir.join("tree.patch");
    let _ = fs::remove_file(&index);
    let host = |args: &[&str]| {
        let mut cmd = Command::new("git");
        cmd.arg("-C")
            .arg(&ws.root)
            .args(["-c", "core.safecrlf=false"])
            .args(args)
            .env("GIT_INDEX_FILE", &index);
        cmd
    };
    let head = out(runner, host(&["rev-parse", "HEAD"])).ok_or_else(|| failed("patch"))?;
    let head = head.trim().to_owned();
    let output = format!("--output={}", patch.display());
    let steps: [&[&str]; 3] = [&["read-tree", "HEAD"], &["add", "-A"], &["write-tree"]];
    let mut tree = String::new();
    for args in steps {
        tree = out(runner, host(args)).ok_or_else(|| failed("patch"))?;
    }
    let tree = tree.trim().to_owned();
    let names = out(runner, host(&["diff", "--cached", "--name-only", "HEAD"]))
        .ok_or_else(|| failed("patch"))?;
    let files = names.lines().filter(|l| !l.trim().is_empty()).count();
    let diff = [
        "diff",
        "--cached",
        "--binary",
        "--no-color",
        "--no-ext-diff",
        &output,
        "HEAD",
    ];
    out(runner, host(&diff)).ok_or_else(|| failed("patch"))?;

    if out(runner, linux.git(&["rev-parse", "--git-dir"])).is_none() {
        let mut clone = linux.cmd(None, &["git", "clone", "-q", "--no-hardlinks", &src]);
        clone.arg(&linux.clone);
        out(runner, clone).ok_or_else(|| failed("clone"))?;
    }
    out(
        runner,
        linux.git(&["fetch", "-q", "--no-tags", &src, "HEAD"]),
    )
    .ok_or_else(|| failed("fetch"))?;
    let fetched = out(runner, linux.git(&["rev-parse", "FETCH_HEAD"]));
    if fetched.as_deref().map(str::trim) != Some(head.as_str()) {
        return Err(failed("fetch"));
    }
    out(runner, linux.git(&["reset", "-q", "--hard", &head])).ok_or_else(|| failed("reset"))?;
    out(runner, linux.git(&["clean", "-fdq"])).ok_or_else(|| failed("clean"))?;
    if files > 0 {
        let patch = to_linux(&patch).ok_or_else(|| failed("apply"))?;
        out(runner, linux.git(&["apply", "--binary", &patch])).ok_or_else(|| failed("apply"))?;
    }
    out(runner, linux.git(&["add", "-A"])).ok_or_else(|| failed("apply"))?;
    let synced = out(runner, linux.git(&["write-tree"])).ok_or_else(|| failed("apply"))?;
    if synced.trim() != tree {
        return Err(Stop::new("sync-mismatch", ""));
    }
    let ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
    Ok(json!({"head": head, "tree": tree, "files": files, "ms": ms}))
}

fn target_bytes(linux: &Linux, runner: &mut Runner<'_>) -> u64 {
    let target = format!("{}/target", linux.clone);
    out(runner, linux.cmd(None, &["du", "-sb", &target]))
        .and_then(|text| text.split_whitespace().next()?.parse().ok())
        .unwrap_or(0)
}

fn cache(linux: &Linux, runner: &mut Runner<'_>) -> Result<Value, Stop> {
    let bytes = target_bytes(linux, runner);
    let cleaned = bytes > CACHE_CAP_BYTES;
    if cleaned {
        let target = format!("{}/target", linux.clone);
        out(runner, linux.cmd(None, &["rm", "-rf", &target]))
            .ok_or_else(|| Stop::new("sync-failed", "cache"))?;
    }
    Ok(json!({"bytes": bytes, "cap": CACHE_CAP_BYTES, "cleaned": cleaned}))
}

/// One harness command inside the clone, read from its own stdout document.
fn linux_harness(linux: &Linux, args: &[&str], runner: &mut Runner<'_>) -> Result<Value, Stop> {
    let mut argv = vec!["bash", "scripts/agent-run.sh"];
    argv.extend_from_slice(args);
    let (_, stdout) = runner(&mut linux.cmd(Some(&linux.clone), &argv));
    last_document(&stdout).ok_or_else(|| Stop::new("linux-document-unreadable", args[0]))
}

/// A Linux run or gate document reduced to codes and counts: the suites' `artifact` paths are the
/// clone's absolute paths, and the user's home never reaches this document.
fn summary(doc: &Value) -> Value {
    let mut out = json!({"ok": doc["ok"]});
    if let Some(reason) = doc["reason"].as_str() {
        out["reason"] = json!(reason);
    }
    if let Some(suites) = doc["suites"].as_array() {
        let suites: Vec<Value> = suites
            .iter()
            .map(|s| {
                json!({
                    "suite": s["suite"], "passed": s["passed"], "failed": s["failed"],
                    "skipped": s["skipped"], "failures": s["failures"],
                })
            })
            .collect();
        out["suites"] = json!(suites);
    }
    if doc.get("breaches").is_some() {
        out["breaches"] = doc["breaches"].clone();
    }
    out
}

fn leg(doc: &Value) -> Value {
    let mut out = json!({"ok": doc["ok"]});
    if let Some(reason) = doc["reason"].as_str() {
        out["reason"] = json!(reason);
    }
    for key in ["base", "verdict", "tested"] {
        out[key] = doc["mutants"][key].clone();
    }
    out
}

/// `run --coverage` then `gate --require coverage,doctest`: the ubuntu test job's own suites.
fn linux_tests(linux: &Linux, runner: &mut Runner<'_>, doc: &mut Doc) -> Result<bool, Stop> {
    let run = linux_harness(linux, &["run", "--coverage"], runner)?;
    doc.linux.insert("run".to_owned(), summary(&run));
    if !ok(&run) {
        return Ok(false);
    }
    let gate = linux_harness(linux, &["gate", "--require", "coverage,doctest"], runner)?;
    doc.linux.insert("gate".to_owned(), summary(&gate));
    Ok(ok(&gate))
}

/// The ubuntu leg in the clone, its reduced verdict copied beside the host's for the union.
fn linux_leg(
    ws: &Workspace,
    linux: &Linux,
    runner: &mut Runner<'_>,
    doc: &mut Doc,
) -> Result<bool, Stop> {
    let host_copy = leg_verdict_path(&ws.artifacts(), LINUX_LEG);
    let _ = fs::remove_file(&host_copy);
    let run = linux_harness(linux, &["run", "--mutants", "--leg", LINUX_LEG], runner)?;
    doc.legs.insert(LINUX_LEG.to_owned(), leg(&run));
    if !ok(&run) {
        return Ok(false);
    }
    let missing = || Stop::new("verdict-missing", LINUX_LEG);
    let path = format!(
        "{}/target/agent-run/artifacts/mutants-verdict-{LINUX_LEG}.json",
        linux.clone
    );
    let verdict: Value = out(runner, linux.cmd(None, &["cat", &path]))
        .and_then(|text| serde_json::from_str(&text).ok())
        .filter(|v: &Value| v["leg"] == LINUX_LEG)
        .ok_or_else(missing)?;
    fs::create_dir_all(ws.artifacts()).map_err(|_| missing())?;
    write_json(&host_copy, &verdict).map_err(|_| missing())?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::harness::read_json;

    const HOME: &str = "/home/tester";
    const CI_LINE: &str = "      - uses: x\n        with:\n          tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0,cargo-llvm-cov@0.9.1\n";

    fn same(path: &Path) -> Option<String> {
        Some(path.to_string_lossy().into_owned())
    }

    fn git_in(dir: &Path, args: &[&str]) -> String {
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
    fn pinned() -> (tempfile::TempDir, Workspace) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let root = tmp.path();
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
    struct Fake {
        answers: Vec<(Vec<String>, i32, String)>,
        calls: Vec<String>,
        claude_envs: usize,
    }

    impl Fake {
        fn new() -> Self {
            Self {
                answers: Vec::new(),
                calls: Vec::new(),
                claude_envs: 0,
            }
        }

        fn on(mut self, patterns: &[&str], code: i32, out: &str) -> Self {
            let patterns = patterns.iter().map(|p| (*p).to_owned()).collect();
            self.answers.push((patterns, code, out.to_owned()));
            self
        }

        /// Every stage green up to and including the Linux leg.
        fn green(self, base: &str, linux_outcome: &str) -> Self {
            let leg = json!({"v": 1, "cmd": "run", "ok": true, "suites": [],
                "mutants": {"tested": 1, "verdict": "counted", "base": base, "leg": LINUX_LEG}});
            let verdict = json!({"v": 1, "leg": LINUX_LEG, "verdict": "counted",
                "mutants": [{"name": MUTANT, "outcome": linux_outcome}]});
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
                .on(&["rev-parse HEAD"], 0, "h1\n")
                .on(&["rev-parse FETCH_HEAD"], 0, "h1\n")
                .on(&["write-tree"], 0, "t1\n")
                .on(&["du -sb"], 0, "10\t/x\n")
                .on(&["run --coverage"], 0, &green_doc("coverage"))
                .on(
                    &["gate --require coverage,doctest"],
                    0,
                    "{\"v\":1,\"cmd\":\"gate\",\"ok\":true,\"breaches\":[]}\n",
                )
                .on(&["--mutants --leg ubuntu-latest"], 0, &format!("{leg}\n"))
                .on(&["cat "], 0, &verdict.to_string())
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

    const MUTANT: &str = "src/lib.rs:1:26: replace a -> u32 with 0";

    fn green_doc(suite: &str) -> String {
        format!(
            "{{\"v\":1,\"cmd\":\"run\",\"ok\":true,\"suites\":[{{\"suite\":\"{suite}\",\"passed\":3,\
             \"failed\":0,\"skipped\":0,\"survived\":0,\"artifact\":\"{HOME}/viola-pre-push/target/x.xml\",\
             \"failures\":[]}}]}}\n"
        )
    }

    fn red_tests() -> Fake {
        let red = format!(
            "noise\n{{\"v\":1,\"cmd\":\"run\",\"ok\":false,\"suites\":[{{\"suite\":\"coverage\",\"passed\":2,\
             \"failed\":1,\"skipped\":0,\"survived\":0,\"artifact\":\"{HOME}/viola-pre-push/target/x.xml\",\
             \"failures\":[\"pre_push_planted_unix_red\"]}}]}}\n"
        );
        Fake::new().on(&["run --coverage"], 1, &red)
    }

    fn drive(ws: &Workspace, fake: &mut Fake) -> Outcome {
        pre_push_with(ws, true, same, &mut |c| fake.answer(c))
    }

    #[test]
    fn pre_push_host_gate_is_a_windows_const() {
        assert_eq!(PRE_PUSH_HOST_SUPPORTED, std::env::consts::OS == "windows");
    }

    #[test]
    fn pre_push_ci_pins_parse_the_test_job_tool_line() {
        let root = Workspace::from_build().root;
        let yml = fs::read_to_string(root.join(".github/workflows/ci.yml")).expect("ci.yml");
        let pins = ci_pins(&yml).expect("pins");
        let want = [
            ("cargo-nextest", "0.9.146"),
            ("cargo-mutants", "27.1.0"),
            ("cargo-llvm-cov", "0.9.1"),
        ]
        .map(|(n, v)| (n.to_owned(), v.to_owned()));
        assert_eq!(pins, want.to_vec());
        assert_eq!(ci_pins("tool: cargo-nextest@1,cargo-mutants@2\n"), None);
        assert_eq!(ci_pins("tool: cargo-nextest@1,cargo-llvm-cov\n"), None);
    }

    #[test]
    fn pre_push_toolchain_channel_reads_the_pin() {
        let root = Workspace::from_build().root;
        let toml = fs::read_to_string(root.join("rust-toolchain.toml")).expect("toml");
        assert_eq!(fuzz_channel(&toml).as_deref(), Some("1.98.1"));
    }

    #[test]
    fn pre_push_wsl_path_maps_drive_paths() {
        let cases = [
            (r"D:\dev\projects\viola", Some("/mnt/d/dev/projects/viola")),
            ("c:/a/b", Some("/mnt/c/a/b")),
            (r"\\?\E:\x", Some("/mnt/e/x")),
            ("F:", Some("/mnt/f")),
            ("D:x", None),
            (r"\\server\share", None),
            (r"rel\x", None),
            ("/home/x", None),
            ("", None),
        ];
        for (input, want) in cases {
            assert_eq!(wsl_path(Path::new(input)).as_deref(), want, "{input}");
        }
    }

    #[test]
    fn pre_push_last_document_takes_the_final_json_line() {
        let doc = last_document("warn\n{\"cmd\":\"run\",\"ok\":true}\n\n").expect("doc");
        assert_eq!(doc["ok"], true);
        assert_eq!(last_document("{\"cmd\":\"run\"}\nnot json\n"), None);
        assert_eq!(last_document("{\"ok\":true}\n"), None);
        assert_eq!(last_document(""), None);
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

    fn stopped(fake: Fake, reason: &str, detail: Option<&str>, stage: &str) -> Fake {
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

    #[test]
    fn pre_push_distro_missing_is_tool_missing() {
        let fake = Fake::new().on(&["printenv HOME"], 1, "");
        let fake = stopped(fake, "tool-missing", Some("wsl-distro-ubuntu"), "tools");
        assert_eq!(fake.calls.len(), 1);
        let relative = Fake::new().on(&["printenv HOME"], 0, "home\n");
        stopped(relative, "tool-missing", Some("wsl-distro-ubuntu"), "tools");
    }

    #[test]
    fn pre_push_cc_missing_is_tool_missing_cc() {
        let fake = Fake::new().on(&["cc --version"], 1, "");
        stopped(fake, "tool-missing", Some("cc"), "tools");
    }

    #[test]
    fn pre_push_pin_mismatch_names_the_tool() {
        let fake = Fake::new().on(&["mutants --version"], 0, "cargo-mutants 27.0.9\n");
        stopped(fake, "tool-pin-mismatch", Some("cargo-mutants"), "tools");
        let rustc = Fake::new().on(&["rustc +1.98.1"], 0, "rustc 1.98.0 (x)\n");
        stopped(rustc, "tool-pin-mismatch", Some("rustc"), "tools");
        let absent = Fake::new().on(&["llvm-cov --version"], 101, "");
        stopped(absent, "tool-missing", Some("cargo-llvm-cov"), "tools");
    }

    #[test]
    fn pre_push_unreadable_pins_stop_at_tools() {
        let (_tmp, ws) = pinned();
        fs::write(ws.root.join(".github/workflows/ci.yml"), "jobs: {}\n").expect("ci");
        let mut fake = Fake::new().green("b", "caught");
        let out = drive(&ws, &mut fake);
        assert_eq!(out.doc["reason"], "tool-pin-mismatch");
        assert_eq!(out.doc["detail"], "pins-unreadable");
    }

    #[test]
    fn pre_push_sync_mismatch_is_red() {
        let fake = Fake::new().on(&["wsl.exe", "write-tree"], 0, "t2\n");
        let fake = stopped(fake, "sync-mismatch", None, "sync");
        assert!(!fake.calls.iter().any(|c| c.contains("du -sb")));
    }

    #[test]
    fn pre_push_sync_fetch_of_another_head_is_red() {
        let fake = Fake::new().on(&["rev-parse FETCH_HEAD"], 0, "h2\n");
        stopped(fake, "sync-failed", Some("fetch"), "sync");
    }

    #[test]
    fn pre_push_cache_over_cap_is_cleaned_and_reported() {
        let over = format!("{}\t/x\n", CACHE_CAP_BYTES + 1);
        let (_tmp, ws) = pinned();
        let mut fake = red_tests().on(&["du -sb"], 0, &over).green("b", "caught");
        let out = drive(&ws, &mut fake);
        assert_eq!(out.doc["cache"]["cleaned"], true, "{}", out.doc);
        assert_eq!(out.doc["cache"]["bytes"], CACHE_CAP_BYTES + 1);
        assert_eq!(out.doc["cache"]["cap"], 42_949_672_960_u64, "40 GiB");
        let rm = format!("rm -rf {HOME}/{CLONE_DIR}/target");
        assert!(fake.calls.iter().any(|c| c.ends_with(&rm)));

        let at = format!("{CACHE_CAP_BYTES}\t/x\n");
        let mut fake = red_tests().on(&["du -sb"], 0, &at).green("b", "caught");
        let out = drive(&ws, &mut fake);
        assert_eq!(out.doc["cache"]["cleaned"], false);
        assert_eq!(out.doc["cache"]["bytes_after"], CACHE_CAP_BYTES);
        assert!(!fake.calls.iter().any(|c| c.contains("rm -rf")));
    }

    #[test]
    fn pre_push_linux_test_red_stops_before_the_legs() {
        let (_tmp, ws) = pinned();
        let mut fake = red_tests().green("b", "caught");
        let out = drive(&ws, &mut fake);
        assert_eq!(out.code, 1);
        assert_eq!(out.doc["ok"], false);
        assert_eq!(out.doc["stage"], "linux-tests");
        assert!(out.doc.get("reason").is_none());
        assert_eq!(
            out.doc["linux"]["run"]["suites"][0]["failures"][0],
            "pre_push_planted_unix_red"
        );
        assert!(out.doc["linux"].get("gate").is_none());
        assert!(!fake.calls.iter().any(|c| c.contains("--mutants")));
        // The scripted working tree changes nothing, so there is no patch to apply.
        assert_eq!(out.doc["sync"]["files"], 0);
        assert!(!fake.calls.iter().any(|c| c.contains(" apply ")));
        assert!(fake.calls.iter().any(|c| c.contains(" write-tree")));
    }

    #[test]
    fn pre_push_linux_gate_red_stops_before_the_legs() {
        let (_tmp, ws) = pinned();
        let red =
            "{\"v\":1,\"cmd\":\"gate\",\"ok\":false,\"breaches\":[{\"gate\":\"coverage\"}]}\n";
        let mut fake = Fake::new()
            .on(&["gate --require coverage,doctest"], 1, red)
            .green("b", "caught");
        let out = drive(&ws, &mut fake);
        assert_eq!(out.doc["stage"], "linux-tests");
        assert_eq!(out.doc["linux"]["gate"]["breaches"][0]["gate"], "coverage");
        assert!(!fake.calls.iter().any(|c| c.contains("--mutants")));
    }

    #[test]
    fn pre_push_unreadable_linux_document_is_named() {
        let fake = Fake::new().on(&["run --coverage"], 101, "error: could not compile\n");
        stopped(
            fake,
            "linux-document-unreadable",
            Some("run"),
            "linux-tests",
        );
    }

    #[test]
    fn pre_push_document_carries_no_absolute_path() {
        let (_tmp, ws) = pinned();
        let mut fake = red_tests().green("b", "caught");
        let text = drive(&ws, &mut fake).doc.to_string();
        assert!(!text.contains(HOME), "{text}");
        assert!(!text.contains(&*ws.root.to_string_lossy()), "{text}");
        assert!(!text.contains("x.xml"), "{text}");
    }

    #[test]
    fn pre_push_every_wsl_call_uses_exec_and_a_clean_env() {
        let (_tmp, ws) = pinned();
        let mut fake = red_tests().green("b", "caught");
        drive(&ws, &mut fake);
        let clean = format!("--exec /usr/bin/env -i HOME={HOME} PATH={HOME}/.cargo/bin:");
        let wsl: Vec<&String> = fake.calls.iter().filter(|c| c.starts_with(WSL)).collect();
        assert!(wsl.len() > 10, "{wsl:?}");
        for call in wsl {
            assert!(call.starts_with("wsl.exe -d Ubuntu "), "{call}");
            assert!(!call.split(' ').any(|t| t == "--"), "{call}");
            if !call.ends_with("printenv HOME") {
                assert!(call.contains(&clean), "{call}");
            }
        }
        assert_eq!(fake.claude_envs, 0);
    }

    #[test]
    fn pre_push_verdict_missing_is_red() {
        let fake = Fake::new().on(&["cat "], 1, "");
        stopped(fake, "verdict-missing", Some(LINUX_LEG), "linux-leg");
        let other = Fake::new().on(&["cat "], 0, "{\"v\":1,\"leg\":\"windows-2025\"}");
        stopped(other, "verdict-missing", Some(LINUX_LEG), "linux-leg");
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
    /// given outcomes, as the existing `run_mutants_leg_*` tests do.
    fn with_host_leg(ws: &Workspace, fake: &mut Fake, windows: &str) -> Outcome {
        let root: PathBuf = ws.root.clone();
        let outcomes = format!(
            "{{\"outcomes\":[{{\"scenario\":{{\"Mutant\":{{\"name\":\"{MUTANT}\"}}}},\"summary\":\"{windows}\"}}],\
             \"caught\":0,\"missed\":1,\"timeout\":0,\"unviable\":0}}"
        );
        pre_push_with(ws, true, same, &mut |c| {
            let args: Vec<String> = c
                .get_args()
                .map(|a| a.to_string_lossy().into_owned())
                .collect();
            if c.get_program() == "cargo" && args.first().is_some_and(|a| a == "mutants") {
                fs::create_dir_all(root.join("mutants.out")).expect("mkdir");
                fs::write(root.join("mutants.out/outcomes.json"), &outcomes).expect("outcomes");
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

    /// Runs the call natively: a WSL call loses its `wsl.exe -d Ubuntu [--cd D] --exec /usr/bin/env
    /// -i HOME=… PATH=…` prefix and runs in D, so the sync's real git sequence runs on this host.
    fn native(cmd: &mut Command) -> (Option<i32>, String) {
        let program = cmd.get_program().to_string_lossy().into_owned();
        let args: Vec<String> = cmd
            .get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();
        let mut real = if program == WSL {
            let mut at = 2;
            let mut cwd = None;
            if args[at] == "--cd" {
                cwd = Some(args[at + 1].clone());
                at += 2;
            }
            assert_eq!(args[at..at + 3], ["--exec", "/usr/bin/env", "-i"]);
            at += 5;
            let mut real = Command::new(&args[at]);
            real.args(&args[at + 1..]);
            if let Some(dir) = cwd {
                real.current_dir(dir);
            }
            real
        } else {
            let mut real = Command::new(&program);
            real.args(&args);
            for (key, value) in cmd.get_envs() {
                if let Some(value) = value {
                    real.env(key, value);
                }
            }
            real
        };
        let out = real.output().expect("spawn");
        (
            out.status.code(),
            String::from_utf8_lossy(&out.stdout).into_owned(),
        )
    }

    #[test]
    fn pre_push_sync_reproduces_a_dirty_tree() {
        let (_tmp, ws) = pinned();
        let src = ws.root.clone();
        for (name, text) in [("a.rs", "a\n"), ("b.rs", "b\n"), ("c.rs", "c\n")] {
            fs::write(src.join(name), text).expect("write");
        }
        fs::write(src.join(".gitignore"), "target/\nignored.txt\n").expect("ignore");
        fs::write(src.join(".gitattributes"), "* text=auto eol=lf\n").expect("attributes");
        git_in(&src, &["init", "-q"]);
        git_in(&src, &["add", "-A"]);
        git_in(&src, &["commit", "-q", "-m", "base"]);
        fs::write(src.join("a.rs"), "a edited\n").expect("edit");
        fs::write(src.join("new.rs"), "new\n").expect("new");
        fs::remove_file(src.join("c.rs")).expect("delete");
        fs::write(src.join("ignored.txt"), "local\n").expect("ignored");

        let distro = tempfile::tempdir().expect("home");
        let linux = Linux::new(&distro.path().to_string_lossy());
        let clone = PathBuf::from(&linux.clone);
        let synced = sync(&ws, &linux, same, &mut native).expect("first sync");
        assert_eq!(synced["files"], 3);
        assert_eq!(
            synced["head"],
            git_in(&src, &["rev-parse", "HEAD"]).as_str()
        );
        assert_eq!(
            fs::read_to_string(clone.join("a.rs")).expect("a"),
            "a edited\n"
        );
        assert!(clone.join("new.rs").is_file());
        assert!(!clone.join("c.rs").exists());
        assert!(!clone.join("ignored.txt").exists());
        git_in(&src, &["diff", "--cached", "--quiet"]);
        let status = git_in(&src, &["status", "--porcelain"]);
        assert!(
            status.contains("M a.rs") && status.contains("?? new.rs"),
            "{status}"
        );

        // A file the clone gained outside a sync (a test run's leftover) is never in its index, so
        // only `clean` removes it; a synced file is dropped by `reset --hard` already.
        fs::write(clone.join("stray.rs"), "stray\n").expect("stray");
        fs::remove_file(src.join("new.rs")).expect("drop");
        fs::write(src.join("b.rs"), "b edited\n").expect("edit");
        let again = sync(&ws, &linux, same, &mut native).expect("second sync");
        assert!(!clone.join("stray.rs").exists());
        assert_eq!(again["files"], 3);
        assert!(!clone.join("new.rs").exists());
        assert_eq!(
            fs::read_to_string(clone.join("b.rs")).expect("b"),
            "b edited\n"
        );
    }
}
