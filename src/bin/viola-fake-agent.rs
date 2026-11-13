//! Test-only stand-in for the `claude` CLI (feature `fake-agent`, never in a release build).
//! Behaviour contract: test-plan §7 "Fake agent". Everything it observes goes to the `--receipt`
//! file; stdout carries only the `--version` answer and print mode's one reply line, so wrapped and
//! unwrapped runs stay comparable.

// It emulates the claude CLI on stdout/stderr; the workspace print bans are for product code.
#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::fs::{self, File, OpenOptions};
use std::io::{Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};
use std::time::Duration;

use serde_json::{Value, json};
use viola_agent_claude::ledger::{
    PROBE_COMPACT_COMMAND, PROBE_LOCAL_COMMAND, PROBE_LONG_PASTE, PROBE_TAG_PASTE,
    STATUSLINE_DEFAULT, STATUSLINE_RATE_LIMITS, dialog_stem, framing_stem,
};
use viola_agent_claude::statusline::BIN_SH;
use viola_core::MAX_FRAME;
use viola_state::liveness::{process_start_time, start_times};

#[path = "viola-fake-agent/agents.rs"]
mod agents;
#[path = "viola-fake-agent/input.rs"]
mod input;
#[path = "viola-fake-agent/statusline.rs"]
mod statusline;
#[cfg(test)]
#[path = "viola-fake-agent/test_cases.rs"]
mod tests;

use agents::{Registry, agents, exe_dir};
use input::{Action, Input};
use statusline::{shell_expanded, statusline_echo, statusline_words};

const DEFAULT_CLI_VERSION: &str = "2.1.287";
const PASTE_START: &[u8] = b"\x1b[200~";
const PASTE_END: &[u8] = b"\x1b[201~";
const WIDE_CHAR: &[u8] = b"\xe4\xb8\xad";
const HARNESS_TURN: &str = "<task-notification>synthetic harness turn</task-notification>";
/// The internal re-exec behind `--exit-no-eof`: a grandchild that holds the inherited stdout.
const HOLD_STDOUT: &str = "--hold-stdout-internal";
const HOLD_FOR: Duration = Duration::from_secs(10);
const CONTROL_POLL: Duration = Duration::from_millis(10);
/// `--stop-receipt-hold-ms` is capped: a test-only hold that forces a window open, never a delay.
const STOP_RECEIPT_HOLD_CAP_MS: u64 = 1000;
/// `--paste-hint-ms` is capped: a test-only hold that forces the window after a long paste open.
/// The cap sits above the gate's 8.5 s maximum wait, which `viola verify`'s hint case holds past.
const PASTE_HINT_CAP_MS: u64 = 10_000;
/// The session a `--resume` with `--fork-session` reports: an id no recorded fixture holds.
const FORK_SESSION_ID: &str = "0f0e0d0c-0b0a-4908-8706-050403020100";
/// The recorded SessionStart a `--resume` launch fires, where the set holds it.
const RESUME_VARIANT: &str = "resume-1";
/// The word of a statusline command the real CLI's shell expands to the shell's own name.
const SHELL_NAME_WORD: &str = "\"$0\"";
/// The mode word, as the first argument: a user's statusline command for tests.
const STATUSLINE_ECHO: &str = "statusline-echo";
/// What `statusline-echo` prints, whatever its stdin holds.
const STATUSLINE_ECHO_OUTPUT: &[u8] = b"viola-fake-statusline\n";
/// The mode word, as the first argument: `agents --json`, answered from the files beside the
/// agent's own executable (no env var selects it).
const AGENTS: &str = "agents";
/// Beside the executable: `recorded` (the default, also for an absent file), `oversize` or
/// `malformed`.
const AGENTS_MODE: &str = "agents-mode";
/// Beside the executable: rows `recorded` lists ahead of the live sessions, as one JSON array.
const AGENTS_ROWS: &str = "agents-rows.json";
/// Beside the executable: one `<pid>.json` row per live interactive fake agent.
const SESSIONS: &str = "sessions";
/// The title a registry row carries: synthetic, never user text.
const SESSION_NAME: &str = "viola fake agent session";
const REGISTERED_EVENTS: [&str; 9] = [
    "SessionStart",
    "UserPromptSubmit",
    "PreToolUse",
    "PermissionRequest",
    "Stop",
    "SessionEnd",
    "Notification",
    "PostToolUse",
    "PostToolUseFailure",
];

#[derive(Debug, Default)]
struct Opts {
    version: bool,
    print: Option<String>,
    cli_version: Option<String>,
    report_version: Option<String>,
    script: Option<PathBuf>,
    control: Option<PathBuf>,
    receipt: Option<PathBuf>,
    fixtures: Option<PathBuf>,
    plugin_dir: Option<PathBuf>,
    suppress_prompt_submit: bool,
    local_command_mode: bool,
    inject_harness_turn: bool,
    exit_no_eof: bool,
    vt100_panic_bytes: bool,
    hold_stdout: bool,
    trusted_root: Option<PathBuf>,
    screens: bool,
    turn_stop: bool,
    dialogs: bool,
    framing: bool,
    stop_receipt_hold: Option<Duration>,
    paste_hint: Option<Duration>,
    tag_turn_screen: Option<String>,
    long_turn_screen: Option<String>,
    resume: Option<String>,
    fork_session: bool,
    settings: Option<PathBuf>,
    statusline_stdin: Option<PathBuf>,
}

impl Opts {
    /// Unknown arguments are ignored, as the real CLI's other flags would be.
    fn parse(args: &[String]) -> Self {
        let mut o = Self::default();
        let mut it = args.iter();
        while let Some(a) = it.next() {
            let mut value = || it.next().cloned();
            match a.as_str() {
                "--version" => o.version = true,
                "-p" | "--print" => o.print = value(),
                "--cli-version" => o.cli_version = value(),
                "--report-version" => o.report_version = value(),
                "--script" => o.script = value().map(PathBuf::from),
                "--control" => o.control = value().map(PathBuf::from),
                "--receipt" => o.receipt = value().map(PathBuf::from),
                "--fixtures" => o.fixtures = value().map(PathBuf::from),
                "--plugin-dir" => o.plugin_dir = value().map(PathBuf::from),
                "--suppress-prompt-submit" => o.suppress_prompt_submit = true,
                "--local-command-mode" => o.local_command_mode = true,
                "--inject-harness-turn" => o.inject_harness_turn = true,
                "--exit-no-eof" => o.exit_no_eof = true,
                "--vt100-panic-bytes" => o.vt100_panic_bytes = true,
                "--trusted-root" => o.trusted_root = value().map(PathBuf::from),
                "--screens" => o.screens = true,
                "--turn-stop" => o.turn_stop = true,
                "--dialogs" => o.dialogs = true,
                "--framing" => o.framing = true,
                "--stop-receipt-hold-ms" => {
                    o.stop_receipt_hold = value()
                        .and_then(|v| v.parse::<u64>().ok())
                        .map(|ms| Duration::from_millis(ms.min(STOP_RECEIPT_HOLD_CAP_MS)));
                }
                "--paste-hint-ms" => {
                    o.paste_hint = value()
                        .and_then(|v| v.parse::<u64>().ok())
                        .map(|ms| Duration::from_millis(ms.min(PASTE_HINT_CAP_MS)));
                }
                "--tag-turn-screen" => o.tag_turn_screen = value(),
                "--long-turn-screen" => o.long_turn_screen = value(),
                "--resume" => o.resume = value(),
                "--fork-session" => o.fork_session = true,
                "--settings" => o.settings = value().map(PathBuf::from),
                "--statusline-stdin" => o.statusline_stdin = value().map(PathBuf::from),
                HOLD_STDOUT => o.hold_stdout = true,
                _ => {}
            }
        }
        o
    }

    fn cli_version(&self) -> &str {
        self.cli_version.as_deref().unwrap_or(DEFAULT_CLI_VERSION)
    }

    fn version_answer(&self) -> String {
        let v = self.report_version.as_deref().unwrap_or(self.cli_version());
        format!("{v} (Claude Code)")
    }

    /// The cwd is trusted when it or an ancestor is `--trusted-root`, the way the real CLI walks up
    /// to a trusted project; without the option every cwd is untrusted.
    fn trusted(&self) -> bool {
        let cwd = std::env::current_dir().unwrap_or_default();
        self.trusted_root
            .as_deref()
            .is_some_and(|root| under_root(&cwd, root))
    }

    /// What `--resume <id>` sets on the SessionStart fired at launch (the set's recorded `resume-1`,
    /// else its `default`): the source, and the id it was given, or with `--fork-session` beside it
    /// the compiled fork id. Nothing without the option.
    fn resume_fields(&self) -> Vec<(&'static str, &str)> {
        let Some(id) = self.resume.as_deref() else {
            return Vec::new();
        };
        let id = if self.fork_session {
            FORK_SESSION_ID
        } else {
            id
        };
        vec![("source", "resume"), ("session_id", id)]
    }

    /// `<fixtures>/<cli version>/Screen.<phase>.json`'s rows, when it exists and parses.
    fn screen(&self, phase: &str) -> Option<Vec<String>> {
        let path = self
            .fixtures
            .as_ref()?
            .join(self.cli_version())
            .join(format!("Screen.{phase}.json"));
        let doc: Value = serde_json::from_slice(&fs::read(path).ok()?).ok()?;
        doc["rows"]
            .as_array()?
            .iter()
            .map(|r| r.as_str().map(str::to_owned))
            .collect()
    }
}

/// `cwd` or one of its ancestors is `root`, both canonicalized; an unreadable path is never under.
fn under_root(cwd: &Path, root: &Path) -> bool {
    match (fs::canonicalize(cwd), fs::canonicalize(root)) {
        (Ok(cwd), Ok(root)) => cwd.ancestors().any(|a| a == root),
        _ => false,
    }
}

/// A recorded screen as a terminal shows it: a clear, then the rows joined by CRLF.
fn render_screen(rows: &[String]) -> Vec<u8> {
    let mut bytes = b"\x1b[2J\x1b[H".to_vec();
    bytes.extend_from_slice(rows.join("\r\n").as_bytes());
    bytes
}

fn draw(rows: &[String]) {
    let mut out = std::io::stdout().lock();
    let _ = out
        .write_all(&render_screen(rows))
        .and_then(|()| out.flush());
}

/// The recorded screen for `phase` on the terminal; a missing fixture writes nothing.
fn write_screen(opts: &Opts, phase: &str) {
    if let Some(rows) = opts.screen(phase) {
        draw(&rows);
    }
}

/// One JSON object + `\n` per `write_all`; the stdin and script threads share it.
struct Receipt(Option<Mutex<File>>);

impl Receipt {
    fn open(path: Option<&Path>) -> Self {
        let file = path.and_then(|p| {
            if let Some(dir) = p.parent() {
                let _ = fs::create_dir_all(dir);
            }
            OpenOptions::new().create(true).append(true).open(p).ok()
        });
        Self(file.map(Mutex::new))
    }

    fn write(&self, kind: &str, fields: Value) {
        let Some(file) = &self.0 else { return };
        let mut line = json!({"v": 1, "kind": kind});
        if let (Some(line), Value::Object(fields)) = (line.as_object_mut(), fields) {
            line.extend(fields);
        }
        let mut bytes = line.to_string().into_bytes();
        bytes.push(b'\n');
        if let Ok(mut f) = file.lock() {
            let _ = f.write_all(&bytes);
        }
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Step {
    event: String,
    variant: String,
    gate: bool,
    /// The `--inject-harness-turn` step: a CLI-originated `UserPromptSubmit`, not a script step.
    harness: bool,
}

fn parse_script(text: &str) -> Option<Vec<Step>> {
    let doc: Value = serde_json::from_str(text).ok()?;
    if doc["v"] != 1 {
        return None;
    }
    doc["steps"]
        .as_array()?
        .iter()
        .map(|s| {
            let event = s["event"].as_str()?;
            REGISTERED_EVENTS.contains(&event).then(|| Step {
                event: event.to_owned(),
                variant: s["variant"].as_str().unwrap_or("default").to_owned(),
                gate: s["gate"].as_bool().unwrap_or(false),
                harness: false,
            })
        })
        .collect()
}

/// Whether a group's `matcher` admits a payload's `tool_name`: an absent or empty matcher admits
/// everything, a `|`-separated one only the names it lists. A payload with no `tool_name` (no
/// recorded fixture of a non-tool event carries one) is never matched against.
fn matcher_admits(matcher: Option<&str>, tool: Option<&str>) -> bool {
    match (matcher.filter(|m| !m.is_empty()), tool) {
        (Some(matcher), Some(tool)) => matcher.split('|').any(|name| name == tool),
        _ => true,
    }
}

/// The `command` + `args` of every `type: "command"` hook registered for `event` in a group whose
/// matcher admits `tool`.
fn hook_commands(hooks_json: &str, event: &str, tool: Option<&str>) -> Vec<(String, Vec<String>)> {
    let Ok(doc) = serde_json::from_str::<Value>(hooks_json) else {
        return Vec::new();
    };
    let Some(groups) = doc["hooks"][event].as_array() else {
        return Vec::new();
    };
    groups
        .iter()
        .filter(|g| matcher_admits(g["matcher"].as_str(), tool))
        .filter_map(|g| g["hooks"].as_array())
        .flatten()
        .filter(|h| h["type"] == "command")
        .filter_map(|h| {
            let command = h["command"].as_str()?.to_owned();
            let args = h["args"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(Value::as_str)
                        .map(str::to_owned)
                        .collect()
                })
                .unwrap_or_default();
            Some((command, args))
        })
        .collect()
}

/// The fixture with the fields its caller names set: the documented `prompt` for
/// `UserPromptSubmit`, `source` and `session_id` for a resumed SessionStart. Nothing else is built,
/// and with no field named the fixture is returned as recorded. The fixture's trailing newline
/// stays, so the recorded prompt yields the recorded bytes.
fn payload(fixture: Vec<u8>, set: &[(&str, &str)]) -> Vec<u8> {
    if set.is_empty() {
        return fixture;
    }
    match serde_json::from_slice::<Value>(&fixture) {
        Ok(Value::Object(mut obj)) => {
            for (key, value) in set {
                obj.insert((*key).to_owned(), json!(value));
            }
            let mut bytes = Value::Object(obj).to_string().into_bytes();
            if fixture.ends_with(b"\n") {
                bytes.push(b'\n');
            }
            bytes
        }
        _ => fixture,
    }
}

/// What firing one event did.
enum Fired {
    NoHooks,
    NoFixture,
    Ran { answered: bool },
}

struct Agent {
    opts: Opts,
    receipt: Receipt,
    /// Held for each hook's whole run; `true` once the agent is exiting.
    hooks: Mutex<bool>,
    /// Set by the first turn's Stop: the status line's `rate_limits` replay follows it once.
    stopped_once: AtomicBool,
    /// This agent's row in the registry beside its executable, once its launch SessionStart fired.
    registry: OnceLock<Registry>,
    /// The `session_id` of the last SessionStart fired: the launch one, or a replayed clear's.
    session: Mutex<Option<String>>,
}

impl Agent {
    fn new(opts: Opts) -> Self {
        Self {
            receipt: Receipt::open(opts.receipt.as_deref()),
            opts,
            hooks: Mutex::new(false),
            stopped_once: AtomicBool::new(false),
            registry: OnceLock::new(),
            session: Mutex::new(None),
        }
    }

    /// The `session_id` the launch SessionStart carries (the recorded variant's, or the resumed id)
    /// and its recorded `cwd`, `~` when it holds none: the registry row replays recorded, scrubbed
    /// values, never this host's directory, which a `--record` would carry into a fixture.
    fn launch_session(&self) -> Option<(String, String)> {
        let fixture = self.fixture("SessionStart", self.launch_variant())?;
        let fired = payload(fixture, &self.opts.resume_fields());
        let doc: Value = serde_json::from_slice(&fired).ok()?;
        let id = doc["session_id"].as_str()?.to_owned();
        let cwd = doc["cwd"].as_str().unwrap_or("~").to_owned();
        Some((id, cwd))
    }

    /// The registry row's status: `busy` once a prompt is submitted, `idle` again at its Stop.
    fn mark(&self, event: &str) {
        let status = match event {
            "UserPromptSubmit" => "busy",
            "Stop" => "idle",
            _ => return,
        };
        if let Some(registry) = self.registry.get() {
            registry.set_status(status);
        }
    }

    /// Before the agent exits: waits for a hook still running and starts no other. A hook is in
    /// the terminal's foreground group, and the agent's exit as its session leader hangs that group
    /// up, which would cut a hook off mid-exit (its coverage profile half written).
    fn close_hooks(&self) {
        *self.hooks.lock().unwrap_or_else(PoisonError::into_inner) = true;
    }

    /// Fires `event`; the returned word is the prompt receipt's `submit` value. The registry row
    /// moves first, so a line the hook leads to is never read ahead of the status it implies.
    fn fire(&self, event: &str, variant: &str, set: &[(&str, &str)]) -> &'static str {
        self.mark(event);
        // A SessionStart with no fixture fires nothing, and leaves the session as it was.
        let start = (event == "SessionStart").then(|| self.fixture(event, variant));
        if let Some(fixture) = start.flatten() {
            let id = serde_json::from_slice::<Value>(&payload(fixture, set))
                .ok()
                .and_then(|doc| doc["session_id"].as_str().map(str::to_owned));
            *self.session.lock().unwrap_or_else(PoisonError::into_inner) = id;
        }
        match self.fire_answered(event, variant, set) {
            Fired::NoHooks => "no-hooks",
            Fired::NoFixture => "no-fixture",
            Fired::Ran { .. } => "fired",
        }
    }

    /// The variant of the SessionStart fired at launch: under `--resume`, the set's recorded
    /// `resume-1` when it holds one, else `default`.
    fn launch_variant(&self) -> &'static str {
        if self.opts.resume.is_some() && self.fixture("SessionStart", RESUME_VARIANT).is_some() {
            RESUME_VARIANT
        } else {
            "default"
        }
    }

    /// `<fixtures>/<cli version>/<event>.<variant>.json`, when it exists.
    fn fixture(&self, event: &str, variant: &str) -> Option<Vec<u8>> {
        self.set_file(&format!("{event}.{variant}.json"))
    }

    /// `<fixtures>/<cli version>/<name>`, when it exists.
    fn set_file(&self, name: &str) -> Option<Vec<u8>> {
        let dir = self.opts.fixtures.as_ref()?.join(self.opts.cli_version());
        fs::read(dir.join(name)).ok()
    }

    /// Fires `event` and reports whether any hook it ran printed something: a dialog's decision.
    fn fire_answered(&self, event: &str, variant: &str, set: &[(&str, &str)]) -> Fired {
        let hooks = self
            .opts
            .plugin_dir
            .as_ref()
            .and_then(|d| fs::read_to_string(d.join("hooks").join("hooks.json")).ok());
        let Some(hooks) = hooks.filter(|h| !hook_commands(h, event, None).is_empty()) else {
            return Fired::NoHooks;
        };
        let Some(fixture) = self.fixture(event, variant) else {
            return Fired::NoFixture;
        };
        let tool = serde_json::from_slice::<Value>(&fixture)
            .ok()
            .and_then(|v| v["tool_name"].as_str().map(str::to_owned));
        let commands = hook_commands(&hooks, event, tool.as_deref());
        let body = payload(fixture, set);
        let mut answered = false;
        for (command, args) in commands {
            answered |= self.run_hook(event, &command, &args, &body);
        }
        Fired::Ran { answered }
    }

    /// `--dialogs`: each recorded call of the prompt's `<stem>-<n>` variants in claim order. Its
    /// PreToolUse fires first; with no decision, its PermissionRequest; a decision from either fires
    /// its PostToolUse. A call with neither a PreToolUse nor a PermissionRequest fixture ends the turn.
    fn replay_dialogs(&self, stem: &str) {
        for n in 1.. {
            let variant = format!("{stem}-{n}");
            let has = |event: &str| self.fixture(event, &variant).is_some();
            if !has("PreToolUse") && !has("PermissionRequest") {
                break;
            }
            let decided = |event: &str| {
                has(event)
                    && matches!(
                        self.fire_answered(event, &variant, &[]),
                        Fired::Ran { answered: true }
                    )
            };
            if decided("PreToolUse") || decided("PermissionRequest") {
                self.fire("PostToolUse", &variant, &[]);
            }
        }
    }

    /// Runs one hook; `true` when it printed anything on stdout.
    fn run_hook(&self, event: &str, command: &str, args: &[String], body: &[u8]) -> bool {
        self.run_receipted("hook", Some(event), command, args, body)
    }

    /// The status line the `--settings` override names, run once with `--statusline-stdin`'s bytes
    /// on its stdin: the command split on ASCII white space and spawned directly, never through a
    /// shell. Without the stdin file nothing is run and nothing is receipted: the agent invents no
    /// payload. Nothing of the run reaches the screen.
    fn run_statusline(&self) {
        let (Some(settings), Some(stdin)) = (&self.opts.settings, &self.opts.statusline_stdin)
        else {
            return;
        };
        let Ok(body) = fs::read(stdin) else {
            return;
        };
        let words = fs::read(settings)
            .ok()
            .and_then(|bytes| statusline_words(&bytes));
        if let Some((command, args)) = words {
            self.run_receipted("statusline", None, &command, &args, &body);
        }
    }

    /// With `--framing`, the status line the `--settings` file names, run once with the set's
    /// recorded `name` payload on its stdin: the command split on ASCII white space, its `"$0"` word
    /// handed over as `/bin/sh`, spawned directly, never through a shell, and receipted as
    /// `statusline`. A set without the variant runs nothing and receipts nothing.
    fn replay_statusline(&self, name: &str) {
        let Some(settings) = self.opts.settings.as_ref().filter(|_| self.opts.framing) else {
            return;
        };
        let Some(body) = self.set_file(name) else {
            return;
        };
        let words = fs::read(settings)
            .ok()
            .and_then(|bytes| statusline_words(&bytes));
        if let Some((command, args)) = words {
            self.run_receipted("statusline", None, &command, &shell_expanded(args), &body);
        }
    }

    /// One command with `body` on its stdin, receipted as `kind` (a hook's line names its `event`);
    /// `true` when it printed anything on stdout. Only an absolute command is spawned, directly.
    fn run_receipted(
        &self,
        kind: &str,
        event: Option<&str>,
        command: &str,
        args: &[String],
        body: &[u8],
    ) -> bool {
        let mut fields = serde_json::Map::new();
        if let Some(event) = event {
            fields.insert("event".to_owned(), json!(event));
        }
        let absolute = Path::new(command).is_absolute();
        fields.insert("command_absolute".to_owned(), json!(absolute));
        if !absolute {
            fields.insert("ran".to_owned(), json!(false));
            self.receipt.write(kind, Value::Object(fields));
            return false;
        }
        let closed = self.hooks.lock().unwrap_or_else(PoisonError::into_inner);
        if *closed {
            return false;
        }
        let mut cmd = Command::new(command);
        cmd.args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        // On Unix the command leaves the foreground process group, as the stdout holder does: a
        // hangup of this terminal (a killed wrapper closing its PTY) reaches that group, and a
        // `viola hook` it reaches mid its exit-time coverage write leaves a cut-short profile.
        #[cfg(unix)]
        std::os::unix::process::CommandExt::process_group(&mut cmd, 0);
        let child = cmd.spawn();
        if let Err(e) = &child {
            fields.insert("spawn_os_error".to_owned(), json!(e.raw_os_error()));
        }
        let output = child.and_then(|mut c| {
            if let Some(mut stdin) = c.stdin.take() {
                let _ = stdin.write_all(body);
            }
            c.wait_with_output()
        });
        fields.insert("ran".to_owned(), json!(output.is_ok()));
        let printed = output.is_ok_and(|out| {
            fields.insert("exit_code".to_owned(), json!(out.status.code()));
            fields.insert("stderr_len".to_owned(), json!(out.stderr.len()));
            fields.insert("stdout_hex".to_owned(), json!(hex(&out.stdout)));
            fields.insert("stdin_hex".to_owned(), json!(hex(body)));
            !out.stdout.is_empty()
        });
        if let Some(hold) = self
            .opts
            .stop_receipt_hold
            .filter(|_| event == Some("Stop"))
        {
            std::thread::sleep(hold);
        }
        self.receipt.write(kind, Value::Object(fields));
        printed
    }

    /// With `--framing`, a text that is one of the verify probe's compiled pastes replays its recorded
    /// variant: a paste text fires that variant's UserPromptSubmit with its bytes unchanged (the
    /// recorded prompt, not the typed text), `/compact` fires its recorded SessionStart with
    /// `session_id` set to the last SessionStart's (the recorded id belongs to the recorded session),
    /// and the probed local command fires its recorded SessionEnd and SessionStart; neither command
    /// fires a UserPromptSubmit. A set without a variant fires nothing for it. With `--paste-hint-ms` beside it, the long text's turn ends on a cleared screen and the
    /// `turn` screen is drawn only after the hold: the real CLI shows a paste hint in the input-box
    /// literal's place for seconds after a long paste. With `--tag-turn-screen <phase>` beside it,
    /// the tag-like text's turn, and no other, ends on that phase's screen in place of `turn`, and
    /// with `--long-turn-screen <phase>` the long text's turn alone does.
    fn submit(&self, bytes: &[u8], origin: &str) {
        let text = String::from_utf8_lossy(bytes);
        let framing = framing_stem(&text).filter(|_| self.opts.framing);
        let local = framing.filter(|stem| framing_stem(PROBE_LOCAL_COMMAND) == Some(*stem));
        let compact = framing.filter(|stem| framing_stem(PROBE_COMPACT_COMMAND) == Some(*stem));
        let submit = if self.opts.suppress_prompt_submit {
            "suppressed"
        } else if self.opts.local_command_mode && text.starts_with('/') {
            "local-command"
        } else if let Some(stem) = compact {
            let session = self
                .session
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .clone();
            let set: Vec<(&str, &str)> = session
                .as_deref()
                .map(|id| ("session_id", id))
                .into_iter()
                .collect();
            self.fire("SessionStart", stem, &set)
        } else if let Some(stem) = local {
            self.fire("SessionEnd", stem, &[]);
            self.fire("SessionStart", stem, &[])
        } else {
            let fired = match framing {
                Some(stem) => self.fire("UserPromptSubmit", stem, &[]),
                None => self.fire("UserPromptSubmit", "default", &[("prompt", &text)]),
            };
            if let Some(stem) = dialog_stem(&text).filter(|_| self.opts.dialogs) {
                self.replay_dialogs(stem);
            }
            if self.opts.turn_stop {
                self.end_turn(framing);
            }
            fired
        };
        self.receipt.write(
            "prompt",
            json!({
                "text": text,
                "hex": hex(bytes),
                "bare_esc": bytes.contains(&0x1b),
                "origin": origin,
                "submit": submit,
            }),
        );
    }

    /// The end of a submitted turn under `--turn-stop`: the Stop hook, the `--paste-hint-ms` hold on
    /// a cleared screen after the long paste, then the turn's screen (`--long-turn-screen`'s after the
    /// long paste alone, `--tag-turn-screen`'s after the tag-like paste alone), and after the first
    /// Stop alone the rate-limits status line. The status line runs last: run before the clear, it
    /// left the earlier input box on screen for a gate wait to read as settled. `framing` is the turn's framing stem, when it replayed one.
    fn end_turn(&self, framing: Option<&str>) {
        self.fire("Stop", "default", &[]);
        // `framing_stem(PROBE_LONG_PASTE)` is a compiled text's stem, always `Some`, so the equality
        // alone says this turn replayed the long paste.
        let long = framing == framing_stem(PROBE_LONG_PASTE);
        if let Some(hold) = self.opts.paste_hint.filter(|_| long) {
            draw(&[]);
            std::thread::sleep(hold);
        }
        let tag = framing.is_some() && framing == framing_stem(PROBE_TAG_PASTE);
        let named = self
            .opts
            .long_turn_screen
            .as_deref()
            .filter(|_| long)
            .or_else(|| self.opts.tag_turn_screen.as_deref().filter(|_| tag));
        write_screen(&self.opts, named.unwrap_or("turn"));
        if !self.stopped_once.swap(true, Ordering::SeqCst) {
            self.replay_statusline(STATUSLINE_RATE_LIMITS);
        }
    }

    /// Blocks until one more complete line exists in the control file past `offset`.
    fn wait_control(&self, offset: &mut u64) {
        loop {
            let released = self.opts.control.as_ref().and_then(|p| {
                let mut f = File::open(p).ok()?;
                let mut buf = Vec::new();
                f.read_to_end(&mut buf).ok()?;
                let start = usize::try_from(*offset).ok()?;
                let nl = buf.get(start..)?.iter().position(|&b| b == b'\n')?;
                Some(start + nl + 1)
            });
            if let Some(next) = released {
                *offset = next as u64;
                return;
            }
            std::thread::sleep(CONTROL_POLL);
        }
    }

    fn run_steps(&self, steps: &[Step]) {
        let mut offset = 0;
        for (index, step) in steps.iter().enumerate() {
            if step.gate {
                self.wait_control(&mut offset);
            }
            self.receipt
                .write("step", json!({"index": index, "event": step.event}));
            if step.harness {
                self.submit(HARNESS_TURN.as_bytes(), "harness");
            } else {
                self.fire(&step.event, &step.variant, &[]);
            }
        }
    }
}

fn start_receipts(agent: &Agent) {
    let started_at = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let plugin_dir = agent
        .opts
        .plugin_dir
        .as_ref()
        .map(|d| d.to_string_lossy().into_owned());
    agent.receipt.write(
        "start",
        json!({
            "cli_version": agent.opts.cli_version(),
            "started_at": started_at,
            "plugin_dir": plugin_dir,
        }),
    );
    let cwd = std::env::current_dir().map(|d| d.to_string_lossy().into_owned());
    agent.receipt.write("cwd", json!({"cwd": cwd.ok()}));
    let mut names: Vec<String> = std::env::vars_os()
        .map(|(k, _)| k.to_string_lossy().into_owned())
        .collect();
    names.sort();
    agent.receipt.write("env", json!({"names": names}));
    #[cfg(unix)]
    {
        let mut fds: Vec<u32> = fs::read_dir("/dev/fd")
            .map(|d| {
                d.filter_map(|e| e.ok()?.file_name().to_str()?.parse().ok())
                    .collect()
            })
            .unwrap_or_default();
        fds.sort_unstable();
        agent.receipt.write("fds", json!({"fds": fds}));
    }
}

/// On Unix the holder leaves the foreground process group: the fake agent is its terminal's session
/// leader, and its exit sends that group SIGHUP, which would end the holder with it.
fn hold_inherited_stdout(receipt: Option<&Path>) {
    let mut cmd = Command::new(std::env::current_exe().unwrap_or_default());
    cmd.arg(HOLD_STDOUT)
        .stdin(Stdio::null())
        .stderr(Stdio::null());
    if let Some(receipt) = receipt {
        cmd.arg("--receipt").arg(receipt);
    }
    #[cfg(unix)]
    std::os::unix::process::CommandExt::process_group(&mut cmd, 0);
    let _ = cmd.spawn();
}

/// The `--inject-harness-turn` step, then the `--script` steps; `None` for an unreadable script.
fn script_steps(opts: &Opts) -> Option<Vec<Step>> {
    let mut steps = Vec::new();
    if opts.inject_harness_turn {
        steps.push(Step {
            event: "UserPromptSubmit".to_owned(),
            variant: "default".to_owned(),
            gate: true,
            harness: true,
        });
    }
    if let Some(path) = &opts.script {
        steps.extend(
            fs::read_to_string(path)
                .ok()
                .and_then(|t| parse_script(&t))?,
        );
    }
    Some(steps)
}

/// The terminal size as a `size` receipt when it differs from `last`: the watcher's one step.
/// Nothing when stdin/stdout is not a terminal.
fn receipt_size(agent: &Agent, last: &mut Option<viola_pty::Size>) {
    let now = viola_pty::host_size();
    if let Some(size) = now.filter(|s| Some(*s) != *last) {
        agent
            .receipt
            .write("size", json!({"cols": size.cols, "rows": size.rows}));
        *last = now;
    }
}

/// The resize oracle: the size at once, then again on every change, polled on the control cadence
/// for the life of the process, so a resize reaches the receipt with no key after it.
fn watch_size(agent: Arc<Agent>) {
    std::thread::spawn(move || {
        let mut last = None;
        loop {
            receipt_size(&agent, &mut last);
            std::thread::sleep(CONTROL_POLL);
        }
    });
}

/// Print mode (`-p <prompt>`): one turn's spine hooks from the fixture set, the prompt as sent, then
/// one reply line. It never enters raw mode and never reads stdin.
fn print_turn(agent: &Agent, prompt: &str) {
    start_receipts(agent);
    agent.fire("SessionStart", "default", &[]);
    agent.fire("UserPromptSubmit", "default", &[("prompt", prompt)]);
    agent.fire("Stop", "default", &[]);
    agent.fire("SessionEnd", "default", &[]);
    println!("ok");
}

/// U+4E2D, a wide character, once on the terminal: at one column vt100 panics on it (the measured
/// bytes behind the forced feed panic). Nothing new is receipted.
fn write_panic_bytes() {
    let mut out = std::io::stdout().lock();
    let _ = out.write_all(WIDE_CHAR).and_then(|()| out.flush());
}

/// On Unix the exit ignores SIGHUP from its first step. The agent is its terminal's session
/// leader, and a leader cannot leave its process group (`setpgid` refuses it), so a hangup of that
/// terminal (a killed wrapper closing its PTY, or portable-pty's SIGHUP kill) still reaches it; one
/// that lands mid its exit-time coverage write leaves a cut-short profile. The `--exit-no-eof`
/// holder, spawned after this, inherits the disposition outside the group.
fn ignore_hangup() {
    #[cfg(unix)]
    // SAFETY: SIG_IGN installs no handler, and the agent sets no SIGHUP action of its own.
    let _ = unsafe { libc::signal(libc::SIGHUP, libc::SIG_IGN) };
}

/// Reads stdin byte by byte until EOF or `\x03`.
fn read_stdin(agent: &Agent) -> ExitCode {
    let mut input = Input::new();
    let mut stdin = viola_pty::host_stdin();
    let mut byte = [0u8; 1];
    while let Ok(1) = stdin.read(&mut byte) {
        for action in input.feed(byte[0]) {
            match action {
                Action::Key(b) => agent.receipt.write("key", json!({"hex": hex(&[b])})),
                Action::Submit(bytes) => agent.submit(&bytes, "human"),
                Action::Exit => {
                    ignore_hangup();
                    agent.close_hooks();
                    if agent.opts.exit_no_eof {
                        hold_inherited_stdout(agent.opts.receipt.as_deref());
                    }
                    return ExitCode::SUCCESS;
                }
            }
        }
    }
    ignore_hangup();
    agent.close_hooks();
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some((mode, rest)) = args.split_first()
        && mode == STATUSLINE_ECHO
    {
        return statusline_echo(rest);
    }
    if args.first().is_some_and(|mode| mode == AGENTS) {
        return agents();
    }
    let opts = Opts::parse(&args);
    if opts.hold_stdout {
        Receipt::open(opts.receipt.as_deref()).write("hold", json!({"pid": std::process::id()}));
        std::thread::sleep(HOLD_FOR);
        return ExitCode::SUCCESS;
    }
    if opts.version {
        println!("{}", opts.version_answer());
        return ExitCode::SUCCESS;
    }
    if let Some(prompt) = opts.print.clone() {
        let agent = Agent::new(opts);
        print_turn(&agent, &prompt);
        return ExitCode::SUCCESS;
    }
    let Some(steps) = script_steps(&opts) else {
        eprintln!("fake agent: unreadable script");
        return ExitCode::from(2);
    };
    let agent = Arc::new(Agent::new(opts));
    // Raw like the real CLI: on a cooked terminal keys wait for Enter and `\x03` never arrives.
    // The `start` receipt below is written only once the mode is set.
    let _terminal = viola_pty::HostTerminal::enter();
    start_receipts(&agent);
    if agent.opts.vt100_panic_bytes {
        write_panic_bytes();
    }
    watch_size(Arc::clone(&agent));
    // With `--screens` an untrusted cwd shows the trust dialog, and the real CLI fires no hook
    // before trust.
    let trusted = !agent.opts.screens || agent.opts.trusted();
    if agent.opts.screens {
        write_screen(&agent.opts, if trusted { "ready" } else { "modal" });
    }
    // The real CLI fires SessionStart at launch; with no registered hook or no fixture, nothing runs.
    // An untrusted folder has no status line either.
    if trusted {
        // The row first: whoever reads the session-start line already finds the session listed.
        let registered = exe_dir()
            .zip(agent.launch_session())
            .and_then(|(dir, (id, cwd))| Registry::open(&dir, &id, &cwd));
        if let Some(registry) = registered {
            let _ = agent.registry.set(registry);
        }
        agent.fire(
            "SessionStart",
            agent.launch_variant(),
            &agent.opts.resume_fields(),
        );
        agent.run_statusline();
        agent.replay_statusline(STATUSLINE_DEFAULT);
    }
    if !steps.is_empty() {
        let runner = Arc::clone(&agent);
        std::thread::spawn(move || runner.run_steps(&steps));
    }
    let code = read_stdin(&agent);
    if let Some(registry) = agent.registry.get() {
        registry.remove();
    }
    code
}
