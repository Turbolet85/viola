//! `viola-fake-agent` as a process (test-plan §7): receipts, bracketed paste, hook invocation from
//! the plugin folder, the prompt-submit modes, gated script steps, `--exit-no-eof`, and the root
//! fixture chain it runs under. Ordering comes from control appends and receipt lines only.

#[allow(dead_code)]
mod support;

use std::io::{Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, ExitStatus, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use rstest::rstest;
use serde_json::{Value, json};
use support::fake::{self, FAKE, of_kind, unhex};
use support::home::{
    StampedHome, TestHome, VIOLA, Wrapper, home, keep_decision, stamped_home, workspace_path,
};

const SENTINEL: &str = "sentinel-value-7f3a-never-logged";
const GATED_TURN: &str = "fixtures/fake-scripts/gated-turn.json";
const HOLD_WINDOW: Duration = Duration::from_millis(300);

/// The fake agent spawned directly, its receipt and control files in the test's scratch dir.
struct Direct {
    child: Child,
    stdin: Option<ChildStdin>,
    receipt: PathBuf,
    control: PathBuf,
}

impl Direct {
    fn spawn(tmp: &TestHome, args: &[&str], env: &[(&str, &str)]) -> Self {
        let receipt = tmp.scratch().join("agent.receipt.ndjson");
        let control = tmp.scratch().join("agent.control");
        let mut cmd = Command::new(FAKE);
        cmd.arg("--receipt")
            .arg(&receipt)
            .arg("--control")
            .arg(&control)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        for (k, v) in env {
            cmd.env(k, v);
        }
        let mut child = cmd.spawn().expect("fake agent");
        let stdin = child.stdin.take();
        let d = Self {
            child,
            stdin,
            receipt,
            control,
        };
        fake::wait_for(&d.receipt, "start line", |l| {
            !of_kind(l, "start").is_empty()
        });
        d
    }

    fn send(&mut self, bytes: &[u8]) {
        let stdin = self.stdin.as_mut().expect("stdin");
        stdin.write_all(bytes).expect("write");
        stdin.flush().expect("flush");
    }

    fn finish(mut self) -> Vec<Value> {
        self.send(b"\x03");
        drop(self.stdin.take());
        assert_eq!(self.child.wait().expect("exit").code(), Some(0));
        fake::receipt(&self.receipt)
    }
}

fn paste(text: &str) -> Vec<u8> {
    [b"\x1b[200~".as_slice(), text.as_bytes(), b"\x1b[201~\r"].concat()
}

/// A plugin folder whose `event` hook is the fake agent itself, receipting its stdin as keys.
fn echo_plugin(tmp: &TestHome, event: &str) -> (PathBuf, PathBuf) {
    let plugin = tmp.scratch().join("plugin");
    let echo = tmp.scratch().join("hook.receipt.ndjson");
    let echo_arg = echo.to_str().expect("utf-8");
    fake::write_plugin(&plugin, event, FAKE, &["--receipt", echo_arg]);
    (plugin, echo)
}

fn fixtures(tmp: &TestHome, event: &str, variant: &str) -> PathBuf {
    let dir = tmp.scratch().join("fixtures");
    let body =
        json!({"hook_event_name": event, "prompt": "placeholder", "session_id": "synthetic"});
    fake::write_fixture(&dir, "2.1.0", event, variant, &body);
    dir
}

fn path_str(p: &Path) -> &str {
    p.to_str().expect("utf-8 path")
}

const PROMPT_FIXTURE: (&str, &str) = ("UserPromptSubmit", "default");

/// The fake agent with `plugin` as its plugin folder and one `(event, variant)` fixture.
fn hooked(
    tmp: &TestHome,
    plugin: &Path,
    (event, variant): (&str, &str),
    extra: &[&str],
    env: &[(&str, &str)],
) -> Direct {
    let fx = fixtures(tmp, event, variant);
    let mut args = vec![
        "--plugin-dir",
        path_str(plugin),
        "--fixtures",
        path_str(&fx),
    ];
    args.extend_from_slice(extra);
    Direct::spawn(tmp, &args, env)
}

fn prompts(lines: &[Value]) -> Vec<&Value> {
    of_kind(lines, "prompt")
}

fn keys(lines: &[Value]) -> Vec<&str> {
    of_kind(lines, "key")
        .iter()
        .filter_map(|l| l["hex"].as_str())
        .collect()
}

fn echoed_payload(echo: &Path) -> Value {
    let bytes: Vec<u8> = keys(&fake::receipt(echo))
        .iter()
        .flat_map(|h| unhex(h))
        .collect();
    serde_json::from_slice(&bytes).expect("payload JSON")
}

#[test]
fn fake_agent_report_version_changes_only_the_version_answer() {
    let out = Command::new(FAKE)
        .args([
            "--report-version",
            "9.0.0",
            "--cli-version",
            "2.1.0",
            "--version",
        ])
        .output()
        .expect("fake agent");
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "9.0.0 (Claude Code)\n"
    );
    let tmp = TestHome::new();
    let (plugin, _) = echo_plugin(&tmp, "UserPromptSubmit");
    let mut agent = hooked(
        &tmp,
        &plugin,
        PROMPT_FIXTURE,
        &["--report-version", "9.0.0", "--cli-version", "2.1.0"],
        &[],
    );
    agent.send(&paste("replay"));
    let lines = agent.finish();
    assert_eq!(of_kind(&lines, "start")[0]["cli_version"], "2.1.0");
    assert_eq!(prompts(&lines)[0]["submit"], "fired");
}

#[test]
fn fake_agent_receipt_env_holds_names_never_values() {
    let tmp = TestHome::new();
    let agent = Direct::spawn(&tmp, &[], &[("CLAUDE_CODE_MESSAGING_TOKEN", SENTINEL)]);
    let lines = agent.finish();
    let text =
        std::fs::read_to_string(tmp.scratch().join("agent.receipt.ndjson")).expect("receipt");
    assert!(!text.contains(SENTINEL));
    let env = of_kind(&lines, "env");
    let names = env[0]["names"].as_array().expect("names");
    assert!(names.iter().any(|n| n == "CLAUDE_CODE_MESSAGING_TOKEN"));
    let mut sorted = names.clone();
    sorted.sort_by(|a, b| a.as_str().cmp(&b.as_str()));
    assert_eq!(&sorted, names);
    assert_eq!(
        of_kind(&lines, "start")[0],
        &json!({"v": 1, "kind": "start", "cli_version": "2.1.0"})
    );
    assert!(lines.iter().all(|l| l["v"] == 1));
}

#[cfg(unix)]
#[test]
fn fake_agent_receipt_lists_open_fds() {
    let tmp = TestHome::new();
    let lines = Direct::spawn(&tmp, &[], &[]).finish();
    let fds = of_kind(&lines, "fds")[0]["fds"]
        .as_array()
        .expect("fds")
        .clone();
    for fd in [0, 1, 2] {
        assert!(fds.contains(&json!(fd)), "fd {fd} in {fds:?}");
    }
}

#[test]
fn fake_agent_bracketed_paste_and_cr_is_one_prompt() {
    let tmp = TestHome::new();
    let mut agent = Direct::spawn(&tmp, &[], &[]);
    agent.send(&paste("one\ntwo"));
    let lines = agent.finish();
    let p = prompts(&lines);
    assert_eq!(p.len(), 1);
    assert_eq!(p[0]["text"], "one\ntwo");
    assert_eq!(p[0]["hex"], "6f6e650a74776f");
    assert_eq!(p[0]["bare_esc"], false);
    assert_eq!(p[0]["origin"], "human");
    assert_eq!(p[0]["submit"], "no-hooks");
    assert_eq!(keys(&lines), ["0d"]);
}

#[test]
fn fake_agent_typed_bytes_are_keys_byte_exact() {
    let tmp = TestHome::new();
    let mut agent = Direct::spawn(&tmp, &[], &[]);
    agent.send(b"hi\x1b[I\x1b[O\r");
    agent.send(b"\x1b[2x\r");
    let lines = agent.finish();
    assert_eq!(
        keys(&lines),
        [
            "68", "69", "1b", "5b", "49", "1b", "5b", "4f", "0d", "1b", "5b", "32", "78", "0d"
        ]
    );
    let p = prompts(&lines);
    assert_eq!(p.len(), 2);
    assert_eq!(p[0]["hex"], "68691b5b491b5b4f");
    assert_eq!(p[0]["bare_esc"], true);
    assert_eq!(p[1]["hex"], "1b5b3278");
}

#[test]
fn fake_agent_cr_with_nothing_typed_submits_nothing() {
    let tmp = TestHome::new();
    let mut agent = Direct::spawn(&tmp, &[], &[]);
    agent.send(b"\r\r");
    let lines = agent.finish();
    assert!(prompts(&lines).is_empty());
    assert_eq!(keys(&lines), ["0d", "0d"]);
}

#[test]
fn fake_agent_fires_the_plugin_hook_with_the_prompt_in_the_payload() {
    let tmp = TestHome::new();
    let (plugin, echo) = echo_plugin(&tmp, "UserPromptSubmit");
    let mut agent = hooked(&tmp, &plugin, PROMPT_FIXTURE, &[], &[]);
    agent.send(&paste("hello there"));
    let lines = agent.finish();
    let hooks = of_kind(&lines, "hook");
    assert_eq!(hooks.len(), 1);
    assert_eq!(
        hooks[0],
        &json!({"v": 1, "kind": "hook", "event": "UserPromptSubmit", "command_absolute": true,
                "ran": true, "exit_code": 0, "stderr_len": 0, "stdout_hex": ""})
    );
    assert_eq!(prompts(&lines)[0]["submit"], "fired");
    let payload = echoed_payload(&echo);
    assert_eq!(payload["prompt"], "hello there");
    assert_eq!(payload["session_id"], "synthetic");
}

#[test]
fn fake_agent_hook_stdout_and_exit_are_receipted() {
    let tmp = TestHome::new();
    let plugin = tmp.scratch().join("plugin");
    fake::write_plugin(
        &plugin,
        "UserPromptSubmit",
        FAKE,
        &["--version", "--cli-version", "4.5.6"],
    );
    let mut agent = hooked(&tmp, &plugin, PROMPT_FIXTURE, &[], &[]);
    agent.send(&paste("x"));
    let lines = agent.finish();
    let hook = of_kind(&lines, "hook")[0];
    assert_eq!(
        unhex(hook["stdout_hex"].as_str().expect("hex")),
        b"4.5.6 (Claude Code)\n"
    );
    assert_eq!(hook["exit_code"], 0);
}

#[test]
fn fake_agent_without_a_fixture_or_plugin_spawns_nothing() {
    let tmp = TestHome::new();
    let (plugin, echo) = echo_plugin(&tmp, "UserPromptSubmit");
    let mut agent = Direct::spawn(&tmp, &["--plugin-dir", path_str(&plugin)], &[]);
    agent.send(&paste("a"));
    let lines = agent.finish();
    assert_eq!(prompts(&lines)[0]["submit"], "no-fixture");
    assert!(of_kind(&lines, "hook").is_empty());
    assert!(!echo.exists());
    let fx = fixtures(&tmp, "UserPromptSubmit", "default");
    std::fs::remove_file(tmp.scratch().join("agent.receipt.ndjson")).expect("reset");
    let mut agent = Direct::spawn(&tmp, &["--fixtures", path_str(&fx)], &[]);
    agent.send(&paste("b"));
    assert_eq!(prompts(&agent.finish())[0]["submit"], "no-hooks");
}

#[test]
fn fake_agent_never_runs_a_non_absolute_hook_command() {
    let tmp = TestHome::new();
    let bin = tmp.scratch().join("bin");
    std::fs::create_dir_all(&bin).expect("bin");
    let decoy = bin.join(format!("decoy-hook{}", std::env::consts::EXE_SUFFIX));
    std::fs::copy(FAKE, &decoy).expect("decoy");
    let marker = tmp.scratch().join("decoy-ran.ndjson");
    let plugin = tmp.scratch().join("plugin");
    fake::write_plugin(
        &plugin,
        "UserPromptSubmit",
        "decoy-hook",
        &["--receipt", path_str(&marker)],
    );
    let mut agent = hooked(
        &tmp,
        &plugin,
        PROMPT_FIXTURE,
        &[],
        &[("PATH", path_str(&bin))],
    );
    agent.send(&paste("a"));
    let lines = agent.finish();
    assert_eq!(
        of_kind(&lines, "hook")[0],
        &json!({"v": 1, "kind": "hook", "event": "UserPromptSubmit", "command_absolute": false, "ran": false})
    );
    assert!(!marker.exists());
}

#[test]
fn fake_agent_suppress_prompt_submit_fires_nothing() {
    let tmp = TestHome::new();
    let (plugin, echo) = echo_plugin(&tmp, "UserPromptSubmit");
    let mut agent = hooked(
        &tmp,
        &plugin,
        PROMPT_FIXTURE,
        &["--suppress-prompt-submit"],
        &[],
    );
    agent.send(&paste("a"));
    let lines = agent.finish();
    assert_eq!(prompts(&lines)[0]["submit"], "suppressed");
    assert!(of_kind(&lines, "hook").is_empty());
    assert!(!echo.exists());
}

#[test]
fn fake_agent_local_command_mode_skips_only_slash_prompts() {
    let tmp = TestHome::new();
    let (plugin, _) = echo_plugin(&tmp, "UserPromptSubmit");
    let mut agent = hooked(
        &tmp,
        &plugin,
        PROMPT_FIXTURE,
        &["--local-command-mode"],
        &[],
    );
    agent.send(&paste("/clear"));
    agent.send(&paste("hello"));
    let lines = agent.finish();
    let p = prompts(&lines);
    assert_eq!(
        (p[0]["submit"].as_str(), p[1]["submit"].as_str()),
        (Some("local-command"), Some("fired"))
    );
    assert_eq!(of_kind(&lines, "hook").len(), 1);
}

#[test]
fn fake_agent_slash_prompt_fires_without_local_command_mode() {
    let tmp = TestHome::new();
    let (plugin, _) = echo_plugin(&tmp, "UserPromptSubmit");
    let mut agent = hooked(&tmp, &plugin, PROMPT_FIXTURE, &[], &[]);
    agent.send(&paste("/clear"));
    assert_eq!(prompts(&agent.finish())[0]["submit"], "fired");
}

fn steps(lines: &[Value]) -> Vec<(u64, String)> {
    of_kind(lines, "step")
        .iter()
        .map(|s| {
            (
                s["index"].as_u64().unwrap_or(99),
                s["event"].as_str().unwrap_or("").to_owned(),
            )
        })
        .collect()
}

#[test]
fn fake_agent_gated_steps_wait_for_control_lines() {
    let tmp = TestHome::new();
    let (plugin, _) = echo_plugin(&tmp, "PreToolUse");
    let script = workspace_path(GATED_TURN);
    let agent = hooked(
        &tmp,
        &plugin,
        ("PreToolUse", "ask"),
        &["--script", path_str(&script)],
        &[],
    );
    fake::stays_false(&agent.receipt, HOLD_WINDOW, |l| !steps(l).is_empty());
    fake::release(&agent.control);
    fake::wait_for(&agent.receipt, "step 0", |l| !of_kind(l, "hook").is_empty());
    fake::stays_false(&agent.receipt, HOLD_WINDOW, |l| steps(l).len() > 1);
    fake::release(&agent.control);
    fake::wait_for(&agent.receipt, "step 1", |l| steps(l).len() == 2);
    let lines = agent.finish();
    assert_eq!(
        steps(&lines),
        [(0, "PreToolUse".to_owned()), (1, "Stop".to_owned())]
    );
    let hooks = of_kind(&lines, "hook");
    assert_eq!(hooks.len(), 1);
    assert_eq!(hooks[0]["event"], "PreToolUse");
}

#[test]
fn fake_agent_ungated_steps_run_at_start() {
    let tmp = TestHome::new();
    let script = tmp.scratch().join("ungated.json");
    std::fs::write(
        &script,
        r#"{"v":1,"steps":[{"event":"Stop"},{"event":"SessionEnd"}]}"#,
    )
    .expect("script");
    let agent = Direct::spawn(&tmp, &["--script", path_str(&script)], &[]);
    fake::wait_for(&agent.receipt, "both steps", |l| steps(l).len() == 2);
    assert_eq!(
        steps(&agent.finish()),
        [(0, "Stop".to_owned()), (1, "SessionEnd".to_owned())]
    );
}

#[test]
fn fake_agent_injected_harness_turn_is_a_gated_harness_prompt() {
    let tmp = TestHome::new();
    let agent = Direct::spawn(&tmp, &["--inject-harness-turn"], &[]);
    fake::stays_false(&agent.receipt, HOLD_WINDOW, |l| !prompts(l).is_empty());
    fake::release(&agent.control);
    fake::wait_for(&agent.receipt, "harness prompt", |l| !prompts(l).is_empty());
    let lines = agent.finish();
    let p = prompts(&lines)[0];
    assert_eq!(p["origin"], "harness");
    assert!(
        p["text"]
            .as_str()
            .is_some_and(|t| t.starts_with("<task-notification>"))
    );
    assert_eq!(steps(&lines), [(0, "UserPromptSubmit".to_owned())]);
}

#[rstest]
#[case::wrong_version(r#"{"v":2,"steps":[]}"#)]
#[case::unknown_event(r#"{"v":1,"steps":[{"event":"Nope"}]}"#)]
#[case::not_json("steps")]
fn fake_agent_rejects_an_unreadable_script(#[case] body: &str) {
    let tmp = TestHome::new();
    let script = tmp.scratch().join("bad.json");
    std::fs::write(&script, body).expect("script");
    let status = Command::new(FAKE)
        .args(["--script", path_str(&script)])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("fake agent");
    assert_eq!(status.code(), Some(2));
}

/// Runs `viola run` with a PIPED stdout, Ctrl-C once the fake agent's terminal is raw (its `start`
/// receipt), and reports (viola's status, EOF seen at exit).
fn run_and_watch_stdout(tmp: &TestHome, fake_args: &[&str]) -> (ExitStatus, bool, Arc<AtomicBool>) {
    let receipt = tmp.scratch().join("watch.receipt.ndjson");
    let mut child = Command::new(VIOLA)
        .arg("--home")
        .arg(tmp.path())
        .args(["run", "builder", "--", FAKE])
        .args(fake_args)
        .arg("--receipt")
        .arg(&receipt)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("viola run");
    let eof = Arc::new(AtomicBool::new(false));
    let mut out = child.stdout.take().expect("stdout");
    let seen = Arc::clone(&eof);
    std::thread::spawn(move || {
        let mut sink = Vec::new();
        let _ = out.read_to_end(&mut sink);
        seen.store(true, Ordering::SeqCst);
    });
    fake::wait_for(&receipt, "start", |l| !of_kind(l, "start").is_empty());
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(b"\x03")
        .expect("ctrl-c");
    let status = child.wait().expect("viola exits");
    let at_exit = eof.load(Ordering::SeqCst);
    (status, at_exit, eof)
}

/// Under the PTY the grandchild holds the child's console, not viola's stdout: the wrapper still
/// exits on the child's process handle (the PTY-level no-EOF witness is `tui_pty_seam`).
#[test]
fn fake_agent_exit_no_eof_exits_while_stdout_is_held() {
    let tmp = TestHome::new();
    let (status, _, _) = run_and_watch_stdout(&tmp, &["--exit-no-eof"]);
    assert_eq!(status.code(), Some(0));
    let role = std::fs::read_to_string(tmp.path().join("diagnostics").join("run-builder.ndjson"))
        .expect("role");
    let exit: Value = role
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).expect("line"))
        .find(|l| l["event"] == "process-exit" && l["subject"] == "claude-child")
        .expect("child exit line");
    assert_eq!(exit["exit_source"], "handle-wait");
    assert_eq!(exit["child_exit_status"], 0);
}

#[test]
fn fake_agent_without_exit_no_eof_releases_stdout_at_exit() {
    let tmp = TestHome::new();
    let (status, _, eof) = run_and_watch_stdout(&tmp, &[]);
    assert_eq!(status.code(), Some(0));
    let deadline = Instant::now() + Duration::from_secs(5);
    while !eof.load(Ordering::SeqCst) {
        assert!(Instant::now() < deadline, "stdout never reached EOF");
        std::thread::yield_now();
    }
}

#[rstest]
fn chain_boots_the_fake_agent_under_viola_with_a_gated_script(stamped_home: StampedHome) {
    assert!(!stamped_home.stamped);
    let e2e_home = workspace_path("target/e2e-home");
    assert!(stamped_home.home.path().starts_with(&e2e_home));
    assert!(
        !stamped_home.home.path().exists(),
        "the chain never pre-creates the home"
    );
    let mut wrapper = Wrapper::boot(stamped_home, "builder", Some(GATED_TURN), &[]);
    let receipt = wrapper.receipt();
    assert!(receipt.starts_with(wrapper.home().join("fake")));
    fake::wait_for(&receipt, "start", |l| !of_kind(l, "start").is_empty());
    wrapper.release();
    fake::wait_for(&receipt, "step 0", |l| steps(l).len() == 1);
    wrapper.release();
    fake::wait_for(&receipt, "step 1", |l| steps(l).len() == 2);
    wrapper.send(&paste("through viola"));
    fake::wait_for(&receipt, "prompt", |l| !prompts(l).is_empty());
    assert!(!wrapper.home().join("ledger").join("stamps.json").exists());
    assert_eq!(wrapper.stop().code(), Some(0));
}

#[rstest]
fn booted_wrapper_fixture_is_ready_and_receipting(
    #[from(support::home::booted_wrapper)] wrapper: Wrapper,
) {
    assert_eq!(wrapper.name, "builder");
    let lines = fake::wait_for(&wrapper.receipt(), "env", |l| !of_kind(l, "env").is_empty());
    assert_eq!(of_kind(&lines, "start").len(), 1);
    assert_eq!(wrapper.stop().code(), Some(0));
}

/// A wrapper that exits before `claude-child` starts is reported as exited at once, not as a
/// readiness timeout: under a mutant that makes `viola` exit silently, the fixture must fail fast.
#[rstest]
#[should_panic(expected = "exited before ready")]
fn wrapper_boot_exiting_before_ready_fails_as_exited(home: TestHome) {
    let missing = home.scratch().join("no-such-program");
    let stamped = StampedHome {
        home,
        fake: missing,
        stamped: false,
    };
    Wrapper::boot(stamped, "builder", None, &[]);
}

#[rstest]
#[case::nothing_set(false, false, false, false)]
#[case::failed_flag_but_passing(false, true, false, false)]
#[case::panicking_without_flag(false, false, true, false)]
#[case::failed_and_panicking(false, true, true, true)]
#[case::keep_homes(true, false, false, true)]
#[case::keep_homes_panicking(true, false, true, true)]
#[case::both_flags_passing(true, true, false, true)]
#[case::both_flags_panicking(true, true, true, true)]
fn keep_decision_keeps_for_ci_or_failed_tests(
    #[case] keep_homes: bool,
    #[case] keep_failed: bool,
    #[case] panicking: bool,
    #[case] kept: bool,
) {
    assert_eq!(keep_decision(keep_homes, keep_failed, panicking), kept);
}

#[rstest]
fn test_home_drop_follows_the_keep_decision(home: TestHome) {
    let scratch = home.scratch().to_path_buf();
    assert!(scratch.is_dir());
    drop(home);
    let flag = |k: &str| std::env::var(k).is_ok_and(|v| v == "1");
    let expected = keep_decision(
        flag("AGENT_RUN_KEEP_HOMES"),
        flag("AGENT_RUN_KEEP_FAILED"),
        false,
    );
    assert_eq!(scratch.exists(), expected);
}
