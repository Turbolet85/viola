//! The distro side of `pre-push`: the WSL2 `Ubuntu` clone synced from the working tree, its tool
//! pins, its build cache, and the ubuntu test job run inside it.

use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

use serde_json::{Value, json};

use super::super::Workspace;
use super::super::run::{Runner, fuzz_channel};
use super::{CACHE_CAP_BYTES, DISTRO, Doc, Stop, WSL, ok, out};

const CLONE_DIR: &str = "viola-pre-push";

/// Where `scripts/wsl-provision.sh` installs ci.yml's pinned Node, under the distro user's home.
const NODE_BIN: &str = ".local/viola-node/bin";

/// The distro side: its user's home and the clone under it. Never printed.
pub(super) struct Linux {
    home: String,
    pub(super) clone: String,
}

impl Linux {
    pub(super) fn new(home: &str) -> Self {
        Self {
            home: home.to_owned(),
            clone: format!("{home}/{CLONE_DIR}"),
        }
    }

    /// `--exec` passes argv verbatim (the `--` form re-parses it through the distro shell), and
    /// `env -i` hands the child only HOME and a Linux PATH: the distro PATH carries the Windows one,
    /// and nothing of this process's environment crosses. Every PATH component is a constant under
    /// the distro home or a system dir.
    fn cmd(&self, cd: Option<&str>, argv: &[&str]) -> Command {
        let mut cmd = Command::new(WSL);
        cmd.args(["-d", DISTRO]);
        if let Some(dir) = cd {
            cmd.args(["--cd", dir]);
        }
        cmd.args(["--exec", "/usr/bin/env", "-i"])
            .arg(format!("HOME={}", self.home))
            .arg(format!(
                "PATH={home}/.cargo/bin:{home}/{NODE_BIN}:/usr/local/bin:/usr/bin:/bin",
                home = self.home
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

/// The workflow-level `NODE_PIN_VERSION` in ci.yml: the one version source of Node, which
/// `scripts/install-node.sh` parses the same way.
pub fn node_pin(ci_yml: &str) -> Option<String> {
    ci_yml
        .lines()
        .filter_map(|l| l.strip_prefix("  NODE_PIN_VERSION:"))
        .map(|v| v.trim().trim_matches('"').to_owned())
        .find(|v| !v.is_empty())
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

/// The distro answers with its user's home, which doubles as its presence probe.
pub(super) fn distro_home(runner: &mut Runner<'_>) -> Result<String, Stop> {
    let mut cmd = Command::new(WSL);
    cmd.args(["-d", DISTRO, "--exec", "/usr/bin/printenv", "HOME"]);
    out(runner, cmd)
        .map(|home| home.trim().to_owned())
        .filter(|home| home.starts_with('/'))
        .ok_or_else(|| Stop::new("tool-missing", "wsl-distro-ubuntu"))
}

/// The C linker, then every tool at its pin: `rust-toolchain.toml`'s channel, ci.yml's test-job
/// tool line and its Node pin, never a version written here.
pub(super) fn tools(ws: &Workspace, linux: &Linux, runner: &mut Runner<'_>) -> Result<(), Stop> {
    if out(runner, linux.cmd(None, &["cc", "--version"])).is_none() {
        return Err(Stop::new("tool-missing", "cc"));
    }
    let unreadable = || Stop::new("tool-pin-mismatch", "pins-unreadable");
    let channel = fs::read_to_string(ws.root.join("rust-toolchain.toml"))
        .ok()
        .and_then(|toml| fuzz_channel(&toml))
        .ok_or_else(unreadable)?;
    let yml = fs::read_to_string(ws.root.join(".github").join("workflows").join("ci.yml"))
        .map_err(|_| unreadable())?;
    let pins = ci_pins(&yml).ok_or_else(unreadable)?;
    let node = node_pin(&yml).ok_or_else(unreadable)?;
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
    let Some(text) = out(runner, linux.cmd(None, &["node", "--version"])) else {
        return Err(Stop::new("tool-missing", "node"));
    };
    if text.trim() != format!("v{node}") {
        return Err(Stop::new("tool-pin-mismatch", "node"));
    }
    Ok(())
}

/// The working tree as one binary patch through a temporary index (the real one is never touched),
/// applied to the clone at the same HEAD; the clone's tree id must then equal the working tree's.
pub(super) fn sync(
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

/// `du -sb` of a distro path; 0 when it is absent or unreadable.
fn dir_bytes(linux: &Linux, runner: &mut Runner<'_>, path: &str) -> u64 {
    out(runner, linux.cmd(None, &["du", "-sb", path]))
        .and_then(|text| text.split_whitespace().next()?.parse().ok())
        .unwrap_or(0)
}

pub(super) fn target_bytes(linux: &Linux, runner: &mut Runner<'_>) -> u64 {
    dir_bytes(linux, runner, &format!("{}/target", linux.clone))
}

/// The clone's `target/` against its cap.
pub(super) fn cache(linux: &Linux, runner: &mut Runner<'_>) -> Result<Value, Stop> {
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
pub(super) fn linux_harness(
    linux: &Linux,
    args: &[&str],
    runner: &mut Runner<'_>,
) -> Result<Value, Stop> {
    let mut argv = vec!["bash", "scripts/agent-run.sh"];
    argv.extend_from_slice(args);
    let (_, stdout) = runner(&mut linux.cmd(Some(&linux.clone), &argv));
    last_document(&stdout).ok_or_else(|| Stop::new("linux-document-unreadable", args[0]))
}

/// A Linux run or gate document reduced to codes and counts: the suites' `artifact` paths are the
/// clone's absolute paths, and the user's home never reaches this document.
pub(super) fn summary(doc: &Value) -> Value {
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

/// `run --coverage`, `run --browser`, then `gate --require coverage,doctest,playwright`: the ubuntu
/// test job's own suites.
pub(super) fn linux_tests(
    linux: &Linux,
    runner: &mut Runner<'_>,
    doc: &mut Doc,
) -> Result<bool, Stop> {
    let run = linux_harness(linux, &["run", "--coverage"], runner)?;
    doc.linux.insert("run".to_owned(), summary(&run));
    if !ok(&run) {
        return Ok(false);
    }
    let browser = linux_harness(linux, &["run", "--browser"], runner)?;
    doc.linux.insert("browser".to_owned(), summary(&browser));
    if !ok(&browser) {
        return Ok(false);
    }
    let gate = linux_harness(
        linux,
        &["gate", "--require", "coverage,doctest,playwright"],
        runner,
    )?;
    doc.linux.insert("gate".to_owned(), summary(&gate));
    Ok(ok(&gate))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::super::tests::{Fake, HOME, drive, git_in, pinned, red_tests, same, stopped};
    use super::*;

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
    fn pre_push_node_pin_reads_the_workflow_env_line() {
        let root = Workspace::from_build().root;
        let yml = fs::read_to_string(root.join(".github/workflows/ci.yml")).expect("ci.yml");
        assert_eq!(node_pin(&yml).as_deref(), Some("24.21.0"));
        assert_eq!(
            node_pin("env:\n  NODE_PIN_VERSION: 1.2.3\n").as_deref(),
            Some("1.2.3")
        );
        assert_eq!(
            node_pin("    NODE_PIN_VERSION: \"1.2.3\"\n"),
            None,
            "job level"
        );
        assert_eq!(node_pin("  NODE_PIN_VERSION: \"\"\n"), None);
        assert_eq!(node_pin("  NODE_PIN_SHA256_WIN_X64: \"ab\"\n"), None);
    }

    #[test]
    fn pre_push_node_missing_or_off_pin_is_named() {
        let absent = Fake::new().on(&["node --version"], 127, "");
        let absent = stopped(absent, "tool-missing", Some("node"), "tools");
        assert!(
            absent
                .calls
                .last()
                .is_some_and(|c| c.ends_with("node --version"))
        );
        let other = Fake::new().on(&["node --version"], 0, "v24.20.0\n");
        stopped(other, "tool-pin-mismatch", Some("node"), "tools");
        let (_tmp, ws) = pinned();
        fs::write(
            ws.root.join(".github/workflows/ci.yml"),
            super::super::tests::CI_LINE.replace("  NODE_PIN_VERSION", "    NODE_PIN_VERSION"),
        )
        .expect("ci");
        let mut fake = Fake::new().green();
        let out = drive(&ws, &mut fake);
        assert_eq!(out.doc["reason"], "tool-pin-mismatch");
        assert_eq!(out.doc["detail"], "pins-unreadable");
    }

    #[test]
    fn pre_push_linux_browser_red_stops_before_the_gate() {
        let (_tmp, ws) = pinned();
        let red = "{\"v\":1,\"cmd\":\"run\",\"ok\":false,\"reason\":\"browser-missing\",\
                   \"suites\":[{\"suite\":\"playwright\",\"passed\":0,\"failed\":1,\"skipped\":0,\
                   \"survived\":0,\"artifact\":null,\"failures\":[\"chromium-missing\"]}]}\n";
        let mut fake = Fake::new().on(&["run --browser"], 1, red).green();
        let out = drive(&ws, &mut fake);
        assert_eq!(out.doc["stage"], "linux-tests");
        assert_eq!(out.doc["linux"]["browser"]["reason"], "browser-missing");
        assert_eq!(
            out.doc["linux"]["browser"]["suites"][0]["failures"][0],
            "chromium-missing"
        );
        assert!(out.doc["linux"].get("gate").is_none());
        assert!(!fake.calls.iter().any(|c| c.contains("gate --require")));
        assert!(!fake.calls.iter().any(|c| c.contains("--mutants")));
    }

    #[test]
    fn pre_push_linux_tests_run_coverage_browser_then_the_playwright_gate() {
        let (_tmp, ws) = pinned();
        let mut fake = Fake::new().green();
        let out = drive(&ws, &mut fake);
        let at = |needle: &str| {
            fake.calls
                .iter()
                .position(|c| c.ends_with(needle))
                .unwrap_or_else(|| panic!("{needle} in {:?}", fake.calls))
        };
        let coverage = at("bash scripts/agent-run.sh run --coverage");
        let browser = at("bash scripts/agent-run.sh run --browser");
        let gate = at("bash scripts/agent-run.sh gate --require coverage,doctest,playwright");
        assert!(coverage < browser && browser < gate, "{:?}", fake.calls);
        assert_eq!(
            out.doc["linux"]["browser"]["suites"][0]["suite"],
            "playwright"
        );
        assert_eq!(out.doc["linux"]["gate"]["ok"], true, "{}", out.doc);
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
        let mut fake = Fake::new().green();
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
        let mut fake = red_tests().on(&["du -sb"], 0, &over).green();
        let out = drive(&ws, &mut fake);
        assert_eq!(out.doc["cache"]["cleaned"], true, "{}", out.doc);
        assert_eq!(out.doc["cache"]["bytes"], CACHE_CAP_BYTES + 1);
        assert_eq!(out.doc["cache"]["cap"], 42_949_672_960_u64, "40 GiB");
        let rm = format!("rm -rf {HOME}/{CLONE_DIR}/target");
        assert!(fake.calls.iter().any(|c| c.ends_with(&rm)));

        let at = format!("{CACHE_CAP_BYTES}\t/x\n");
        let mut fake = red_tests().on(&["du -sb"], 0, &at).green();
        let out = drive(&ws, &mut fake);
        assert_eq!(out.doc["cache"]["cleaned"], false);
        assert_eq!(out.doc["cache"]["bytes_after"], CACHE_CAP_BYTES);
        assert!(!fake.calls.iter().any(|c| c.ends_with(&rm)));
    }

    #[test]
    fn pre_push_linux_test_red_stops_at_linux_tests() {
        let (_tmp, ws) = pinned();
        let mut fake = red_tests().green();
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
    fn pre_push_linux_gate_red_stops_at_linux_tests() {
        let (_tmp, ws) = pinned();
        let red =
            "{\"v\":1,\"cmd\":\"gate\",\"ok\":false,\"breaches\":[{\"gate\":\"coverage\"}]}\n";
        let mut fake = Fake::new()
            .on(&["gate --require coverage,doctest"], 1, red)
            .green();
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
    fn pre_push_every_wsl_call_uses_exec_and_a_clean_env() {
        let (_tmp, ws) = pinned();
        let mut fake = red_tests().green();
        drive(&ws, &mut fake);
        let clean = format!("--exec /usr/bin/env -i HOME={HOME} PATH={HOME}/.cargo/bin:");
        // The one call that runs nothing inside the distro: stopping it (`release_vm`).
        let terminate = "wsl.exe --terminate Ubuntu";
        let wsl: Vec<&String> = fake
            .calls
            .iter()
            .filter(|c| c.starts_with(WSL) && *c != terminate)
            .collect();
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
