//! The native side of `pre-push`: the tool pins and the ubuntu test job, run in the working tree on
//! this Linux host through `env -i`, so only HOME and a constant PATH reach a child.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{Value, json};

use super::super::Workspace;
use super::super::run::{Runner, fuzz_channel};
use super::{Doc, Stop, ok, out};

const ENV: &str = "/usr/bin/env";

/// Where `scripts/install-node.sh` puts ci.yml's pinned Node, under the user's home.
const NODE_BIN: &str = ".local/viola-node/bin";

/// The system dirs the passwd probes search: the home is what they read, so it is not yet known.
const SYSTEM_PATH: &str = "PATH=/usr/bin:/bin";

/// The user's home (from the passwd entry) and the repository root the children run in. Never
/// printed.
pub(super) struct Linux {
    home: String,
    root: PathBuf,
}

impl Linux {
    pub(super) fn new(home: &str, root: &Path) -> Self {
        Self {
            home: home.to_owned(),
            root: root.to_path_buf(),
        }
    }

    /// `env -i` hands the child only HOME and PATH, so nothing of this process's environment
    /// crosses (no `CLAUDE*`, no `CARGO_*`, no `LLVM_PROFILE_FILE`). Every PATH component is a
    /// constant under the passwd home or a system dir.
    pub(super) fn cmd(&self, argv: &[&str]) -> Command {
        let mut cmd = Command::new(ENV);
        cmd.arg("-i")
            .arg(format!("HOME={}", self.home))
            .arg(format!(
                "PATH={home}/.cargo/bin:{home}/{NODE_BIN}:/usr/local/bin:/usr/bin:/bin",
                home = self.home
            ))
            .args(argv)
            .current_dir(&self.root);
        cmd
    }
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

/// The passwd entry's home for this process's uid (`id -u`, then `getent passwd <uid>`, field 6),
/// read with only a system PATH. The harness's own `$HOME` is never read for it: that would carry
/// a host value across `env -i`.
pub(super) fn passwd_home(runner: &mut Runner<'_>) -> Result<String, Stop> {
    let missing = || Stop::new("tool-missing", "passwd-home");
    let probe = |argv: &[&str]| {
        let mut cmd = Command::new(ENV);
        cmd.args(["-i", SYSTEM_PATH]).args(argv);
        cmd
    };
    let uid = out(runner, probe(&["id", "-u"])).ok_or_else(missing)?;
    let uid = uid.trim();
    if uid.is_empty() || !uid.bytes().all(|b| b.is_ascii_digit()) {
        return Err(missing());
    }
    let entry = out(runner, probe(&["getent", "passwd", uid])).ok_or_else(missing)?;
    entry
        .lines()
        .next()
        .and_then(|line| line.split(':').nth(5))
        .filter(|home| home.starts_with('/'))
        .map(str::to_owned)
        .ok_or_else(missing)
}

/// The C linker, then every tool at its pin: `rust-toolchain.toml`'s channel, ci.yml's test-job
/// tool line and its Node pin, never a version written here.
pub(super) fn tools(ws: &Workspace, linux: &Linux, runner: &mut Runner<'_>) -> Result<(), Stop> {
    if out(runner, linux.cmd(&["cc", "--version"])).is_none() {
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
        let Some(text) = out(runner, linux.cmd(&argv)) else {
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
    let Some(text) = out(runner, linux.cmd(&["node", "--version"])) else {
        return Err(Stop::new("tool-missing", "node"));
    };
    if text.trim() != format!("v{node}") {
        return Err(Stop::new("tool-pin-mismatch", "node"));
    }
    Ok(())
}

/// One harness command in the working tree, read from its own stdout document.
pub(super) fn linux_harness(
    linux: &Linux,
    args: &[&str],
    runner: &mut Runner<'_>,
) -> Result<Value, Stop> {
    let mut argv = vec!["bash", "scripts/agent-run.sh"];
    argv.extend_from_slice(args);
    let (_, stdout) = runner(&mut linux.cmd(&argv));
    last_document(&stdout).ok_or_else(|| Stop::new("linux-document-unreadable", args[0]))
}

/// A run or gate document reduced to codes and counts: the suites' `artifact` paths are absolute,
/// and neither the home nor the repository path reaches this document.
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
    use super::super::tests::{CI_LINE, Fake, HOME, drive, pinned, red_tests, stopped};
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
            CI_LINE.replace("  NODE_PIN_VERSION", "    NODE_PIN_VERSION"),
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
    fn pre_push_last_document_takes_the_final_json_line() {
        let doc = last_document("warn\n{\"cmd\":\"run\",\"ok\":true}\n\n").expect("doc");
        assert_eq!(doc["ok"], true);
        assert_eq!(last_document("{\"cmd\":\"run\"}\nnot json\n"), None);
        assert_eq!(last_document("{\"ok\":true}\n"), None);
        assert_eq!(last_document(""), None);
    }

    /// The home every launcher call carries is the passwd entry's field 6, read through `id -u`
    /// and `getent passwd <uid>`; an unreadable entry stops at `tools`.
    #[test]
    fn pre_push_home_reads_the_passwd_entry() {
        let (_tmp, ws) = pinned();
        let mut fake = Fake::new().green();
        drive(&ws, &mut fake);
        assert_eq!(fake.calls[0], format!("{ENV} -i {SYSTEM_PATH} id -u"));
        assert_eq!(
            fake.calls[1],
            format!("{ENV} -i {SYSTEM_PATH} getent passwd 1000")
        );
        let home = format!(" HOME={HOME} ");
        assert!(
            fake.calls[2..].iter().all(|c| c.contains(&home)),
            "{:?}",
            fake.calls
        );

        for (id, entry, calls) in [
            ((1, "1000\n"), (0, "x\n"), 1),
            ((0, "\n"), (0, "x\n"), 1),
            ((0, "10x0\n"), (0, "x\n"), 1),
            ((0, "1000\n"), (2, ""), 2),
            (
                (0, "1000\n"),
                (0, "tester:x:1000:1000::home/tester:/bin/bash\n"),
                2,
            ),
            ((0, "1000\n"), (0, "tester:x:1000:1000:\n"), 2),
        ] {
            let fake =
                Fake::new()
                    .on(&["id -u"], id.0, id.1)
                    .on(&["getent passwd"], entry.0, entry.1);
            let fake = stopped(fake, "tool-missing", Some("passwd-home"), "tools");
            assert_eq!(fake.calls.len(), calls, "{:?}", fake.calls);
        }
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
        assert!(out.doc["linux"].get("browser").is_none());
        assert!(out.doc["linux"].get("gate").is_none());
        assert!(!fake.calls.iter().any(|c| c.contains("--mutants")));
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
        assert_eq!(out.code, 1);
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

    /// Every call is `/usr/bin/env -i` followed only by `HOME=` and `PATH=` assignments: HOME is the
    /// passwd home and every PATH component a constant under it or a system dir. The launcher's PATH
    /// is exactly the former WSL shape (no venv, nothing else).
    #[test]
    fn pre_push_every_native_call_carries_only_home_and_path() {
        let (_tmp, ws) = pinned();
        let mut fake = Fake::new().green();
        let out = drive(&ws, &mut fake);
        assert_eq!(out.code, 0, "{}", out.doc);
        assert!(fake.calls.len() > 10, "{:?}", fake.calls);
        let system = ["/usr/local/bin", "/usr/bin", "/bin"];
        let under_home = format!("{HOME}/");
        for call in &fake.calls {
            let rest = call
                .strip_prefix(&format!("{ENV} -i "))
                .unwrap_or_else(|| panic!("not an env -i launch: {call}"));
            let assignments: Vec<(&str, &str)> = rest
                .split(' ')
                .map_while(|token| token.split_once('='))
                .collect();
            assert!(!assignments.is_empty(), "{call}");
            for (name, value) in assignments {
                match name {
                    "HOME" => assert_eq!(value, HOME, "{call}"),
                    "PATH" => {
                        for dir in value.split(':') {
                            assert!(
                                dir.starts_with(&under_home) || system.contains(&dir),
                                "{dir} in {call}"
                            );
                        }
                    }
                    other => panic!("{other} crosses env -i in {call}"),
                }
            }
        }
        let launcher = format!(
            "{ENV} -i HOME={HOME} PATH={HOME}/.cargo/bin:{HOME}/.local/viola-node/bin:\
             /usr/local/bin:/usr/bin:/bin bash scripts/agent-run.sh run --coverage"
        );
        assert!(fake.calls.contains(&launcher), "{:?}", fake.calls);
        assert_eq!(fake.claude_envs, 0);
    }

    /// Two-sided, a real launch: `/usr/bin/env` through the launcher, with a canary on the outer
    /// `Command`, sees only HOME and PATH; the same launch without `-i` forwards the canary.
    #[cfg(target_os = "linux")]
    #[test]
    fn pre_push_native_child_sees_only_home_and_path() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let linux = Linux::new(HOME, tmp.path());
        let names = |cmd: &mut Command| -> Vec<String> {
            let out = cmd
                .env("CLAUDE_CANARY", "pre-push-canary")
                .output()
                .expect("env");
            assert!(out.status.success());
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter_map(|l| l.split_once('=').map(|(name, _)| name.to_owned()))
                .collect()
        };
        let mut subject = linux.cmd(&[ENV]);
        let seen = names(&mut subject);
        assert_eq!(seen, ["HOME", "PATH"]);
        assert_eq!(seen.iter().filter(|n| n.starts_with("CLAUDE")).count(), 0);

        let mut control = Command::new(subject.get_program());
        control
            .args(subject.get_args().filter(|a| a.to_str() != Some("-i")))
            .current_dir(tmp.path());
        let forwarded = names(&mut control);
        assert_eq!(
            forwarded.iter().filter(|n| *n == "CLAUDE_CANARY").count(),
            1,
            "{forwarded:?}"
        );
    }
}
