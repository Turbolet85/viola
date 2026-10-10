//! `viola-fake-agent` as a process (test-plan §7): receipts, bracketed paste, hook invocation from
//! the plugin folder, the prompt-submit modes, gated script steps, `--exit-no-eof`, and the root
//! fixture chain it runs under. Ordering comes from control appends and receipt lines only.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, ExitStatus, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use rstest::rstest;
use serde_json::{Value, json};
use support::fake::{self, FAKE, of_kind, unhex};
use support::home::{
    STATUSLINE_ECHO_OUTPUT, StampedHome, TestHome, VIOLA, Wrapper, home, keep_decision,
    stamped_home, statusline_echo_command, workspace_path,
};
use support::outer_pty::{EXIT_WITHIN, OuterPty};
use support::piped::Piped;

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
    fake::write_fixture(&dir, fake::RECORDED_CLI_VERSION, event, variant, &body);
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
            fake::RECORDED_CLI_VERSION,
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
        &[
            "--report-version",
            "9.0.0",
            "--cli-version",
            fake::RECORDED_CLI_VERSION,
        ],
        &[],
    );
    agent.send(&paste("replay"));
    let lines = agent.finish();
    assert_eq!(
        of_kind(&lines, "start")[0]["cli_version"],
        fake::RECORDED_CLI_VERSION
    );
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
    let start = of_kind(&lines, "start")[0];
    let keys: Vec<&str> = start
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        keys,
        ["v", "kind", "cli_version", "started_at", "plugin_dir"]
    );
    assert_eq!(start["cli_version"], fake::RECORDED_CLI_VERSION);
    assert!(
        start["started_at"]
            .as_str()
            .is_some_and(|t| t.ends_with('Z') && t.len() == 24)
    );
    assert_eq!(start["plugin_dir"], json!(null));
    assert!(lines.iter().all(|l| l["v"] == 1));
}

/// A receipt read while the agent appends can end in a line not yet complete (a reader can see an
/// append part-way): only newline-terminated lines are read, and the rest is read once it lands.
#[test]
fn receipt_reader_leaves_a_torn_final_line_unread() {
    let tmp = TestHome::new();
    std::fs::create_dir_all(tmp.scratch()).expect("scratch");
    let path = tmp.scratch().join("torn.receipt.ndjson");
    std::fs::write(
        &path,
        "{\"v\":1,\"kind\":\"start\"}\n{\"v\":1,\"kind\":\"ke",
    )
    .expect("torn");
    assert_eq!(fake::receipt(&path), [json!({"v": 1, "kind": "start"})]);
    let mut rest = std::fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .expect("append");
    rest.write_all(b"y\"}\n").expect("the rest of the line");
    assert_eq!(
        fake::receipt(&path),
        [
            json!({"v": 1, "kind": "start"}),
            json!({"v": 1, "kind": "key"})
        ]
    );
    assert!(fake::receipt(&tmp.scratch().join("absent.ndjson")).is_empty());
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
    let sent =
        br#"{"hook_event_name":"UserPromptSubmit","prompt":"hello there","session_id":"synthetic"}"#;
    let stdin_hex: String = sent.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(
        hooks[0],
        &json!({"v": 1, "kind": "hook", "event": "UserPromptSubmit", "command_absolute": true,
                "ran": true, "exit_code": 0, "stderr_len": 0, "stdout_hex": "",
                "stdin_hex": stdin_hex})
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

/// `<scratch>/spine-plugin` registering the fake agent's `--version` (it exits 0) for each spine
/// event.
fn spine_plugin(tmp: &TestHome) -> PathBuf {
    let plugin = tmp.scratch().join("spine-plugin");
    let hooks = plugin.join("hooks");
    std::fs::create_dir_all(&hooks).expect("hooks dir");
    let mut registered = serde_json::Map::new();
    for event in support::verify::SPINE {
        registered.insert(
            event.to_owned(),
            json!([{"hooks": [{"type": "command", "command": FAKE, "args": ["--version"]}]}]),
        );
    }
    std::fs::write(
        hooks.join("hooks.json"),
        json!({"hooks": registered}).to_string(),
    )
    .expect("hooks.json");
    plugin
}

/// Each `hook` line's event and the bytes its hook was offered, in receipt order.
fn offered(lines: &[Value]) -> Vec<(String, Vec<u8>)> {
    of_kind(lines, "hook")
        .iter()
        .map(|h| {
            (
                h["event"].as_str().expect("event").to_owned(),
                h["stdin_hex"].as_str().map(unhex).expect("stdin_hex"),
            )
        })
        .collect()
}

/// The SessionStart bytes the fake agent offers its hook at launch over the committed set, and that
/// set's recorded payload.
fn launch_session_start(extra: &[&str]) -> (Vec<u8>, Vec<u8>) {
    let tmp = TestHome::new();
    let plugin = spine_plugin(&tmp);
    let committed = workspace_path("fixtures/claude");
    let mut args = vec![
        "--plugin-dir",
        path_str(&plugin),
        "--fixtures",
        path_str(&committed),
    ];
    args.extend_from_slice(extra);
    let lines = Direct::spawn(&tmp, &args, &[]).finish();
    let (event, offered) = offered(&lines).into_iter().next().expect("a hook ran");
    assert_eq!(event, "SessionStart");
    let recorded = std::fs::read(
        committed
            .join(fake::RECORDED_CLI_VERSION)
            .join("SessionStart.default.json"),
    )
    .expect("the recorded payload");
    (offered, recorded)
}

/// The recorded payload with its `source` and `session_id` set, its trailing newline kept.
fn resumed(recorded: &[u8], session_id: &str) -> Vec<u8> {
    let mut payload: Value = serde_json::from_slice(recorded).expect("json");
    assert_eq!(payload["source"], "startup");
    payload["source"] = json!("resume");
    payload["session_id"] = json!(session_id);
    let mut bytes = payload.to_string().into_bytes();
    bytes.push(b'\n');
    bytes
}

/// `--resume <id>`: the SessionStart fired at launch is the recorded payload with `source`
/// `resume` and the given id, every other byte as recorded. Without the option it is the recorded
/// bytes.
#[test]
fn fake_agent_resume_reports_the_given_session_on_the_recorded_session_start() {
    let id = "11111111-2222-4333-8444-555555555555";
    let (offered, recorded) = launch_session_start(&["--resume", id]);
    assert_eq!(offered, resumed(&recorded, id));
    assert!(recorded.ends_with(b"\n") && !recorded.windows(id.len()).any(|w| w == id.as_bytes()));
    let (plain, recorded) = launch_session_start(&[]);
    assert_eq!(plain, recorded);
}

/// `--fork-session` beside `--resume`: the session reported is the compiled fork id, which no
/// committed fixture holds; alone it changes nothing.
#[test]
fn fake_agent_fork_session_reports_the_compiled_fork_id() {
    let fork = "0f0e0d0c-0b0a-4908-8706-050403020100";
    let asked = "11111111-2222-4333-8444-555555555555";
    let (offered, recorded) = launch_session_start(&["--resume", asked, "--fork-session"]);
    assert_eq!(offered, resumed(&recorded, fork));
    let (alone, recorded) = launch_session_start(&["--fork-session"]);
    assert_eq!(alone, recorded);
}

/// With `--framing` a compiled paste text replays its recorded variant with the bytes unchanged,
/// and the local command fires its recorded SessionEnd and SessionStart and no UserPromptSubmit;
/// any other text is echoed as before. Without the option every text is echoed as typed, and a set
/// that lacks a variant fires nothing for it.
#[test]
fn fake_agent_framing_replays_the_recorded_shapes() {
    let tmp = TestHome::new();
    let plugin = spine_plugin(&tmp);
    let fx = tmp.scratch().join("fixtures");
    let version = fake::RECORDED_CLI_VERSION;
    support::verify::write_spine_set(&fx, version, None);
    let read = |name: &str| std::fs::read(fx.join(version).join(name)).expect("fixture");
    let long = support::verify::long_paste();
    let base = [
        "--plugin-dir",
        path_str(&plugin),
        "--fixtures",
        path_str(&fx),
    ];

    let mut args = base.to_vec();
    args.extend(["--framing", "--turn-stop"]);
    let mut agent = Direct::spawn(&tmp, &args, &[]);
    agent.send(&paste(&long));
    agent.send(&paste(support::verify::TAG_PASTE));
    agent.send(&paste("/clear"));
    agent.send(&paste("hello"));
    let lines = agent.finish();
    let mut echoed: Value =
        serde_json::from_slice(&read("UserPromptSubmit.default.json")).expect("json");
    echoed["prompt"] = json!("hello");
    assert_eq!(
        offered(&lines),
        [
            ("SessionStart".to_owned(), read("SessionStart.default.json")),
            (
                "UserPromptSubmit".to_owned(),
                read("UserPromptSubmit.paste-1.json")
            ),
            ("Stop".to_owned(), read("Stop.default.json")),
            (
                "UserPromptSubmit".to_owned(),
                read("UserPromptSubmit.paste-2.json")
            ),
            ("Stop".to_owned(), read("Stop.default.json")),
            ("SessionEnd".to_owned(), read("SessionEnd.clear-1.json")),
            ("SessionStart".to_owned(), read("SessionStart.clear-1.json")),
            (
                "UserPromptSubmit".to_owned(),
                echoed.to_string().into_bytes()
            ),
            ("Stop".to_owned(), read("Stop.default.json")),
        ]
    );
    let submits: Vec<&str> = prompts(&lines)
        .iter()
        .filter_map(|p| p["submit"].as_str())
        .collect();
    assert_eq!(submits, ["fired", "fired", "fired", "fired"]);
    assert_eq!(prompts(&lines)[0]["text"], long.as_str());

    let plain = TestHome::new();
    let mut agent = Direct::spawn(&plain, &base, &[]);
    agent.send(&paste(&long));
    agent.send(&paste("/clear"));
    let lines = agent.finish();
    let sent: Vec<(String, Value)> = offered(&lines)
        .into_iter()
        .map(|(event, bytes)| (event, serde_json::from_slice(&bytes).expect("json")))
        .collect();
    let events: Vec<&str> = sent.iter().map(|(e, _)| e.as_str()).collect();
    assert_eq!(
        events,
        ["SessionStart", "UserPromptSubmit", "UserPromptSubmit"]
    );
    assert_eq!(sent[1].1["prompt"], long.as_str());
    assert_eq!(sent[2].1["prompt"], "/clear");

    let bare = TestHome::new();
    let spine_only = bare.scratch().join("fixtures");
    for event in support::verify::SPINE {
        fake::write_fixture(
            &spine_only,
            version,
            event,
            "default",
            &support::verify::spine_payload(event),
        );
    }
    let lacking = [
        "--plugin-dir",
        path_str(&plugin),
        "--fixtures",
        path_str(&spine_only),
        "--framing",
    ];
    let mut agent = Direct::spawn(&bare, &lacking, &[]);
    agent.send(&paste(&long));
    agent.send(&paste("/clear"));
    let lines = agent.finish();
    let events: Vec<String> = offered(&lines).into_iter().map(|(e, _)| e).collect();
    assert_eq!(events, ["SessionStart"]);
    let submits: Vec<&str> = prompts(&lines)
        .iter()
        .filter_map(|p| p["submit"].as_str())
        .collect();
    assert_eq!(submits, ["no-fixture", "no-fixture"]);
}

/// `--tag-turn-screen <phase>`: under `--framing --turn-stop` the tag-like text's turn ends on the
/// named screen of the set, and every other turn on `turn`. Read from the receipt's prompts and
/// from what the agent drew on this test's own terminal, where each screen is one marker row.
#[test]
fn fake_agent_tag_turn_screen_draws_the_named_screen_after_the_tag_turn_alone() {
    const TURN: &str = "turn-screen-5c1e";
    const TAGGED: &str = "tagged-screen-5c1e";
    let tmp = TestHome::new();
    let fx = tmp.scratch().join("fixtures");
    let version = fake::RECORDED_CLI_VERSION;
    support::verify::write_screen(&fx, version, "turn", &[TURN.to_owned()]);
    support::verify::write_screen(&fx, version, "tagged", &[TAGGED.to_owned()]);
    let receipt = tmp.scratch().join("agent.receipt.ndjson");
    let args = [
        "--receipt",
        path_str(&receipt),
        "--fixtures",
        path_str(&fx),
        "--framing",
        "--turn-stop",
        "--tag-turn-screen",
        "tagged",
    ]
    .map(OsString::from);
    let mut pty = OuterPty::spawn(Path::new(FAKE), &args, &[]);
    fake::wait_for(&receipt, "start line", |l| !of_kind(l, "start").is_empty());
    let long = support::verify::long_paste();
    let texts = [long.as_str(), support::verify::TAG_PASTE, "hello"];
    for (n, text) in texts.iter().enumerate() {
        pty.write(&paste(text));
        // A prompt is receipted after its turn's screen is drawn.
        fake::wait_for(&receipt, "the prompt's receipt", |l| prompts(l).len() > n);
    }
    pty.write(b"\x03");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    let lines = fake::receipt(&receipt);
    let typed: Vec<&str> = prompts(&lines)
        .iter()
        .filter_map(|p| p["text"].as_str())
        .collect();
    assert_eq!(typed, texts);
    let drawn = String::from_utf8_lossy(&pty.finish()).into_owned();
    let mut screens: Vec<(usize, &str)> = [TURN, TAGGED]
        .iter()
        .flat_map(|marker| drawn.match_indices(marker))
        .collect();
    screens.sort_unstable();
    let order: Vec<&str> = screens.into_iter().map(|(_, marker)| marker).collect();
    assert_eq!(order, [TURN, TAGGED, TURN], "{drawn:?}");
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
    support::home::seed_conpty(tmp.path());
    let mut piped = Piped::spawn(
        Command::new(VIOLA)
            .arg("--home")
            .arg(tmp.path())
            .args(["run", "builder", "--", FAKE])
            .args(fake_args)
            .arg("--receipt")
            .arg(&receipt),
    );
    let eof = piped.stdout_eof();
    fake::wait_for(&receipt, "start", |l| !of_kind(l, "start").is_empty());
    piped.write(b"\x03");
    let status = piped.wait();
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
    let exit =
        support::ndjson::read_lines(&tmp.path().join("diagnostics").join("run-builder.ndjson"))
            .into_iter()
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
    assert!(stamped_home.stamped);
    let e2e_home = workspace_path("target/e2e-home");
    assert!(stamped_home.home.path().starts_with(&e2e_home));
    assert!(
        stamped_home.home.path().exists(),
        "viola verify created the home"
    );
    let stamps = std::fs::read(stamped_home.home.path().join("ledger").join("stamps.json"))
        .expect("stamps written by verify");
    let stamps: Value = serde_json::from_slice(&stamps).expect("stamps JSON");
    assert_eq!(stamps["writer"], "verify");
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
/// A settings override whose status line is the fake agent's own `statusline-echo`, a payload
/// file, and the marker file the echo appends to, all in the test's scratch dir.
fn statusline_files(tmp: &TestHome, payload: &[u8]) -> (PathBuf, PathBuf, PathBuf) {
    let marker = tmp.scratch().join("statusline.marker");
    let command = statusline_echo_command(&marker, &[]);
    let settings = tmp.scratch().join("settings.json");
    let doc = json!({"statusLine": {"type": "command", "command": command}});
    std::fs::write(&settings, doc.to_string()).expect("settings");
    let stdin = tmp.scratch().join("statusline.stdin");
    std::fs::write(&stdin, payload).expect("payload");
    (settings, stdin, marker)
}

fn hex_of(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// `--settings` alone names a status line and runs nothing: the agent invents no payload.
#[test]
fn fake_statusline_settings_without_a_stdin_file_runs_nothing() {
    let tmp = TestHome::new();
    let (settings, _, marker) = statusline_files(&tmp, b"{}");
    let agent = Direct::spawn(&tmp, &["--settings", path_str(&settings)], &[]);
    let lines = agent.finish();
    assert!(of_kind(&lines, "statusline").is_empty());
    assert!(!marker.exists());
}

/// `--statusline-stdin` beside it runs the override's status line once at launch, directly, with
/// exactly the file's bytes, and receipts the run in the hook receipt's form.
#[test]
fn fake_statusline_stdin_runs_the_settings_command_with_the_file_s_bytes() {
    let tmp = TestHome::new();
    let payload = br#"{"session_id":"canary-chain-value-5c1e","rate_limits":{}}"#;
    let (settings, stdin, marker) = statusline_files(&tmp, payload);
    let agent = Direct::spawn(
        &tmp,
        &[
            "--settings",
            path_str(&settings),
            "--statusline-stdin",
            path_str(&stdin),
        ],
        &[],
    );
    let ran = fake::wait_statusline(&agent.receipt);
    assert_eq!(
        ran,
        json!({"v": 1, "kind": "statusline", "command_absolute": true, "ran": true,
               "exit_code": 0, "stderr_len": 0,
               "stdout_hex": hex_of(STATUSLINE_ECHO_OUTPUT.as_bytes()),
               "stdin_hex": hex_of(payload)})
    );
    let lines = agent.finish();
    assert_eq!(of_kind(&lines, "statusline").len(), 1);
    assert_eq!(
        std::fs::read_to_string(&marker).expect("the marker file"),
        hex_of(payload) + "\n"
    );
}

/// The `statusline-echo` mode: the fixed output whatever its stdin, one line of that stdin's hex
/// appended to the marker file, and the exit code `--exit` names.
#[test]
fn fake_statusline_echo_mode_prints_its_output_and_files_its_stdin() {
    let tmp = TestHome::new();
    let marker = tmp.scratch().join("statusline.marker");
    let echo = |stdin: &[u8], extra: &[&str]| {
        let mut child = Command::new(FAKE)
            .arg("statusline-echo")
            .arg(&marker)
            .args(extra)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("fake agent");
        child
            .stdin
            .take()
            .expect("stdin")
            .write_all(stdin)
            .expect("write");
        child.wait_with_output().expect("exit")
    };
    let first = echo(b"\x1b[0m one", &[]);
    assert_eq!(first.status.code(), Some(0));
    assert_eq!(first.stdout, STATUSLINE_ECHO_OUTPUT.as_bytes());
    assert!(first.stderr.is_empty());
    let second = echo(b"", &["--exit", "3"]);
    assert_eq!(second.status.code(), Some(3));
    assert_eq!(second.stdout, STATUSLINE_ECHO_OUTPUT.as_bytes());
    assert_eq!(
        std::fs::read_to_string(&marker).expect("the marker file"),
        hex_of(b"\x1b[0m one") + "\n\n"
    );
}

/// readiness timeout: under a mutant that makes `viola` exit silently, the fixture must fail fast.
#[rstest]
#[should_panic(expected = "exited before ready")]
fn wrapper_boot_exiting_before_ready_fails_as_exited(home: TestHome) {
    let missing = home.scratch().join("no-such-program");
    let stamped = StampedHome {
        fake: missing,
        ..StampedHome::unstamped(home)
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
