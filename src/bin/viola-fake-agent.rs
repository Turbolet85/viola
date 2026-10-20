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

/// `statusLine.command` of a settings document, split on ASCII white space into the program and its
/// arguments.
fn statusline_words(settings: &[u8]) -> Option<(String, Vec<String>)> {
    let doc: Value = serde_json::from_slice(settings).ok()?;
    let mut words = doc["statusLine"]["command"]
        .as_str()?
        .split_ascii_whitespace()
        .map(str::to_owned);
    let command = words.next()?;
    Some((command, words.collect()))
}

/// A statusline command's arguments as the real CLI's `/bin/sh` hands them over (2.1.287, read
/// live): the word `"$0"` becomes `/bin/sh`; every other word is unchanged. No shell is run.
fn shell_expanded(args: Vec<String>) -> Vec<String> {
    args.into_iter()
        .map(|word| {
            if word == SHELL_NAME_WORD {
                BIN_SH.to_owned()
            } else {
                word
            }
        })
        .collect()
}

/// The `statusline-echo <marker file> [--exit <code>]` mode: stdin read to its end, the fixed
/// output printed, one line holding the hex of that stdin appended to the marker file, then the
/// exit code asked for (0 without one).
fn statusline_echo(args: &[String]) -> ExitCode {
    let mut stdin = Vec::new();
    let _ = std::io::stdin().lock().read_to_end(&mut stdin);
    if let Some(marker) = args.first() {
        let line = hex(&stdin) + "\n";
        let file = OpenOptions::new().create(true).append(true).open(marker);
        let _ = file.and_then(|mut f| f.write_all(line.as_bytes()));
    }
    let mut out = std::io::stdout().lock();
    let _ = out
        .write_all(STATUSLINE_ECHO_OUTPUT)
        .and_then(|()| out.flush());
    ExitCode::from(statusline_echo_exit(args))
}

/// The code after `--exit`, when that word follows the marker path; 0 otherwise.
fn statusline_echo_exit(args: &[String]) -> u8 {
    match args {
        [_, flag, code, ..] if flag == "--exit" => code.parse().unwrap_or(0),
        _ => 0,
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
        let child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();
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

/// One live interactive fake agent's row beside its executable, in the measured `claude agents
/// --json` shape: rewritten whole on each status change and removed at exit. `startedAt` is the
/// process's start time, which the listing compares, so a dead pid or a reused one is left out.
struct Registry {
    path: PathBuf,
    row: Mutex<Value>,
}

impl Registry {
    fn open(dir: &Path, session_id: &str, cwd: &str) -> Option<Self> {
        let pid = std::process::id();
        let started = process_start_time(pid)?;
        let sessions = dir.join(SESSIONS);
        fs::create_dir_all(&sessions).ok()?;
        let registry = Self {
            path: sessions.join(format!("{pid}.json")),
            row: Mutex::new(json!({
                "cwd": cwd,
                "kind": "interactive",
                "name": SESSION_NAME,
                "pid": pid,
                "sessionId": session_id,
                "startedAt": started.saturating_mul(1000),
                "status": "idle",
            })),
        };
        registry.set_status("idle");
        Some(registry)
    }

    /// The row with `status`, written to a sibling and renamed over the old one: a listing never
    /// reads half a row.
    fn set_status(&self, status: &str) {
        let mut row = self.row.lock().unwrap_or_else(PoisonError::into_inner);
        row["status"] = json!(status);
        let next = self.path.with_extension("json.next");
        if fs::write(&next, row.to_string()).is_ok() {
            let _ = fs::rename(&next, &self.path);
        }
    }

    fn remove(&self) {
        let _row = self.row.lock().unwrap_or_else(PoisonError::into_inner);
        let _ = fs::remove_file(&self.path);
    }
}

/// The directory holding this executable: where the `agents` mode files and the registry live.
fn exe_dir() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()?
        .parent()
        .map(Path::to_path_buf)
}

/// The `agents --json` mode: what [`agents_output`] gives for the mode file beside the executable,
/// on stdout, exit 0.
fn agents() -> ExitCode {
    let dir = exe_dir();
    let mode = dir
        .as_deref()
        .and_then(|d| fs::read_to_string(d.join(AGENTS_MODE)).ok());
    let out = agents_output(mode.as_deref().map(str::trim), dir.as_deref());
    let mut stdout = std::io::stdout().lock();
    let _ = stdout.write_all(&out).and_then(|()| stdout.flush());
    ExitCode::SUCCESS
}

/// `oversize`: one byte past the frame cap; `malformed`: a text that is not JSON; anything else:
/// one JSON array of the planted rows, then each live registry row.
fn agents_output(mode: Option<&str>, dir: Option<&Path>) -> Vec<u8> {
    match mode {
        Some("oversize") => vec![b' '; usize::try_from(MAX_FRAME).unwrap_or(usize::MAX - 1) + 1],
        Some("malformed") => b"No sessions\n".to_vec(),
        _ => {
            let mut rows = dir.map(planted_rows).unwrap_or_default();
            rows.extend(dir.map(live_sessions).unwrap_or_default());
            let mut text = Value::Array(rows).to_string();
            text.push('\n');
            text.into_bytes()
        }
    }
}

/// The rows of `<dir>/agents-rows.json`; a file that is absent or no JSON array gives none.
fn planted_rows(dir: &Path) -> Vec<Value> {
    match fs::read(dir.join(AGENTS_ROWS)).map(|b| serde_json::from_slice::<Value>(&b)) {
        Ok(Ok(Value::Array(rows))) => rows,
        _ => Vec::new(),
    }
}

/// The registry rows under `<dir>/sessions/` whose process is alive with the recorded start time,
/// in file-name order. The process table is read once, whatever the number of rows (rows of killed
/// agents stay behind); no signal is sent.
fn live_sessions(dir: &Path) -> Vec<Value> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir.join(SESSIONS))
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|e| e == "json"))
        .collect();
    paths.sort();
    let table = start_times();
    paths
        .into_iter()
        .filter_map(|path| serde_json::from_slice::<Value>(&fs::read(path).ok()?).ok())
        .filter(|row| {
            let pid = row["pid"].as_u64().and_then(|p| u32::try_from(p).ok());
            let started = row["startedAt"].as_u64().map(|ms| ms / 1000);
            pid.zip(started)
                .is_some_and(|(pid, started)| table.get(&pid) == Some(&started))
        })
        .collect()
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

/// Stdin as a terminal would deliver it: a bracketed paste is prompt content, every other byte is
/// a keystroke, CR submits whatever the prompt holds, `\x03` ends the process.
struct Input {
    esc: Vec<u8>,
    in_paste: bool,
    prompt: Vec<u8>,
}

enum Action {
    Key(u8),
    Submit(Vec<u8>),
    Exit,
}

impl Input {
    fn new() -> Self {
        Self {
            esc: Vec::new(),
            in_paste: false,
            prompt: Vec::new(),
        }
    }

    fn feed(&mut self, byte: u8) -> Vec<Action> {
        if self.in_paste {
            self.prompt.push(byte);
            if self.prompt.ends_with(PASTE_END) {
                self.prompt.truncate(self.prompt.len() - PASTE_END.len());
                self.in_paste = false;
            }
            return Vec::new();
        }
        if byte == 0x1b || !self.esc.is_empty() {
            self.esc.push(byte);
            if self.esc == PASTE_START {
                self.esc.clear();
                self.in_paste = true;
                return Vec::new();
            }
            if PASTE_START.starts_with(&self.esc) {
                return Vec::new();
            }
            let mut held = std::mem::take(&mut self.esc);
            // A fresh ESC that broke the match may itself open the next paste.
            if held.last() == Some(&0x1b) {
                held.pop();
                self.esc.push(0x1b);
            }
            return held.into_iter().flat_map(|b| self.plain(b)).collect();
        }
        self.plain(byte)
    }

    fn plain(&mut self, byte: u8) -> Vec<Action> {
        match byte {
            0x03 => vec![Action::Exit],
            b'\r' => {
                let mut actions = vec![Action::Key(byte)];
                if !self.prompt.is_empty() {
                    actions.push(Action::Submit(std::mem::take(&mut self.prompt)));
                }
                actions
            }
            _ => {
                self.prompt.push(byte);
                vec![Action::Key(byte)]
            }
        }
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

/// Reads stdin byte by byte until EOF or `\x03`. A Windows console is read the way the wrapper
/// reads it: std's console read turns a `^Z` typed alone into end of input.
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
                    agent.close_hooks();
                    if agent.opts.exit_no_eof {
                        hold_inherited_stdout(agent.opts.receipt.as_deref());
                    }
                    return ExitCode::SUCCESS;
                }
            }
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| (*s).to_owned()).collect()
    }

    /// Feeds bytes and renders the actions as `k:<hex>` / `p:<hex>` / `exit`.
    fn feed(input: &mut Input, bytes: &[u8]) -> Vec<String> {
        bytes
            .iter()
            .flat_map(|b| input.feed(*b))
            .map(|a| match a {
                Action::Key(b) => format!("k:{}", hex(&[b])),
                Action::Submit(p) => format!("p:{}", hex(&p)),
                Action::Exit => "exit".to_owned(),
            })
            .collect()
    }

    #[test]
    fn opts_parse_reads_values_flags_and_ignores_unknown() {
        let o = Opts::parse(&args(&[
            "--argv-sentinel",
            "--cli-version",
            "3.0.0",
            "--report-version",
            "4.0.0",
            "--script",
            "s.json",
            "--control",
            "c",
            "--receipt",
            "r",
            "--fixtures",
            "f",
            "--plugin-dir",
            "p",
            "--suppress-prompt-submit",
            "--local-command-mode",
            "--inject-harness-turn",
            "--exit-no-eof",
            "--vt100-panic-bytes",
            "--trusted-root",
            "t",
            "--screens",
            "--turn-stop",
            "--dialogs",
            "--framing",
            "--stop-receipt-hold-ms",
            "5000",
            "--paste-hint-ms",
            "11000",
            "--tag-turn-screen",
            "tagged",
            "--long-turn-screen",
            "longed",
            "--resume",
            "c0f0cc23-690b-45dd-bbfb-06d6cfd44942",
            "--fork-session",
            "--settings",
            "st.json",
            "--statusline-stdin",
            "sl",
            "--version",
            "-p",
            "a prompt",
        ]));
        assert_eq!(o.settings.as_deref(), Some(Path::new("st.json")));
        assert_eq!(o.statusline_stdin.as_deref(), Some(Path::new("sl")));
        assert_eq!(o.print.as_deref(), Some("a prompt"));
        assert_eq!(
            Opts::parse(&args(&["--print", "q"])).print.as_deref(),
            Some("q")
        );
        assert_eq!(o.cli_version(), "3.0.0");
        assert_eq!(o.version_answer(), "4.0.0 (Claude Code)");
        assert_eq!(o.script.as_deref(), Some(Path::new("s.json")));
        assert_eq!(o.control.as_deref(), Some(Path::new("c")));
        assert_eq!(o.receipt.as_deref(), Some(Path::new("r")));
        assert_eq!(o.fixtures.as_deref(), Some(Path::new("f")));
        assert_eq!(o.plugin_dir.as_deref(), Some(Path::new("p")));
        assert!(o.version && o.suppress_prompt_submit && o.local_command_mode);
        assert!(o.inject_harness_turn && o.exit_no_eof && !o.hold_stdout);
        assert!(o.vt100_panic_bytes);
        assert_eq!(o.trusted_root.as_deref(), Some(Path::new("t")));
        assert!(o.screens && o.turn_stop && o.dialogs && o.framing);
        assert_eq!(
            o.stop_receipt_hold,
            Some(Duration::from_millis(1000)),
            "capped"
        );
        assert_eq!(o.paste_hint, Some(Duration::from_millis(10_000)), "capped");
        assert_eq!(o.tag_turn_screen.as_deref(), Some("tagged"));
        assert_eq!(o.long_turn_screen.as_deref(), Some("longed"));
        assert_eq!(
            o.resume.as_deref(),
            Some("c0f0cc23-690b-45dd-bbfb-06d6cfd44942")
        );
        assert!(o.fork_session);
        assert!(Opts::parse(&args(&[HOLD_STDOUT])).hold_stdout);
    }

    #[test]
    fn opts_default_to_the_default_cli_version() {
        let o = Opts::parse(&[]);
        assert_eq!(o.cli_version(), DEFAULT_CLI_VERSION);
        assert_eq!(o.version_answer(), "2.1.287 (Claude Code)");
        assert!(!o.version && !o.exit_no_eof && !o.suppress_prompt_submit);
        assert!(!o.vt100_panic_bytes);
        assert!(o.print.is_none());
        assert!(!o.screens && !o.turn_stop && !o.dialogs && o.trusted_root.is_none());
        assert!(!o.framing);
        assert_eq!(o.stop_receipt_hold, None);
        assert_eq!(
            Opts::parse(&args(&["--stop-receipt-hold-ms", "100"])).stop_receipt_hold,
            Some(Duration::from_millis(100))
        );
        assert_eq!(
            Opts::parse(&args(&["--stop-receipt-hold-ms", "x"])).stop_receipt_hold,
            None
        );
        assert_eq!(o.paste_hint, None);
        assert_eq!(
            Opts::parse(&args(&["--paste-hint-ms", "6000"])).paste_hint,
            Some(Duration::from_millis(6000))
        );
        assert_eq!(
            Opts::parse(&args(&["--paste-hint-ms", "x"])).paste_hint,
            None
        );
        assert_eq!(o.tag_turn_screen, None);
        assert_eq!(
            Opts::parse(&args(&["--tag-turn-screen"])).tag_turn_screen,
            None
        );
        assert!(o.resume.is_none() && !o.fork_session);
        assert_eq!(Opts::parse(&args(&["--resume"])).resume, None);
        assert!(o.settings.is_none() && o.statusline_stdin.is_none());
        assert_eq!(Opts::parse(&args(&["--settings"])).settings, None);
        assert!(!o.trusted(), "no root: every cwd is untrusted");
    }

    #[test]
    fn opts_long_turn_screen_takes_the_phase_it_names_alone() {
        let o = Opts::parse(&args(&["--long-turn-screen", "long-modal"]));
        assert_eq!(o.long_turn_screen.as_deref(), Some("long-modal"));
        assert_eq!(o.tag_turn_screen, None);
    }

    #[test]
    fn opts_long_turn_screen_defaults_to_none() {
        assert_eq!(Opts::parse(&[]).long_turn_screen, None);
        assert_eq!(
            Opts::parse(&args(&["--long-turn-screen"])).long_turn_screen,
            None
        );
    }

    #[test]
    fn under_root_walks_up_from_the_cwd() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let deep = tmp.path().join("a").join("b");
        fs::create_dir_all(&deep).expect("dirs");
        assert!(under_root(&deep, tmp.path()));
        assert!(under_root(&deep, &deep));
        assert!(under_root(&deep, &tmp.path().join("a").join("..")));
        assert!(!under_root(tmp.path(), &deep));
        assert!(!under_root(&deep, &tmp.path().join("missing")));
        assert!(!under_root(&tmp.path().join("missing"), tmp.path()));
    }

    #[test]
    fn screen_reads_the_versioned_fixture_rows() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("3.0.0");
        fs::create_dir_all(&dir).expect("dir");
        fs::write(
            dir.join("Screen.ready.json"),
            r#"{"screen_phase":"ready","cols":80,"rows":["","a b"]}"#,
        )
        .expect("fixture");
        fs::write(dir.join("Screen.turn.json"), r#"{"rows":[1]}"#).expect("bad");
        let o = Opts::parse(&args(&[
            "--cli-version",
            "3.0.0",
            "--fixtures",
            tmp.path().to_str().expect("utf-8"),
        ]));
        assert_eq!(
            o.screen("ready"),
            Some(vec![String::new(), "a b".to_owned()])
        );
        assert_eq!(o.screen("turn"), None);
        assert_eq!(o.screen("modal"), None);
        assert_eq!(Opts::parse(&[]).screen("ready"), None);
        assert_eq!(
            render_screen(&["x".to_owned(), "y".to_owned()]),
            b"\x1b[2J\x1b[Hx\r\ny"
        );
    }

    #[test]
    fn input_paste_then_cr_is_one_submit() {
        let mut i = Input::new();
        assert_eq!(
            feed(&mut i, b"\x1b[200~a\rb\x1b[201~\r"),
            ["k:0d", "p:610d62"]
        );
    }

    #[test]
    fn input_typed_bytes_are_keys_and_cr_submits_them() {
        let mut i = Input::new();
        assert_eq!(feed(&mut i, b"ab\r"), ["k:61", "k:62", "k:0d", "p:6162"]);
        assert_eq!(feed(&mut i, b"\r"), ["k:0d"]);
        assert_eq!(feed(&mut i, b"\x03"), ["exit"]);
    }

    #[test]
    fn input_escape_sequences_that_are_not_a_paste_are_keys() {
        let mut i = Input::new();
        assert_eq!(feed(&mut i, b"\x1b[I"), ["k:1b", "k:5b", "k:49"]);
        assert_eq!(
            feed(&mut i, b"\x1b[20x"),
            ["k:1b", "k:5b", "k:32", "k:30", "k:78"]
        );
        assert!(feed(&mut i, b"\x1b[200").is_empty());
        assert_eq!(
            feed(&mut i, b"\x1b"),
            ["k:1b", "k:5b", "k:32", "k:30", "k:30"]
        );
        assert!(feed(&mut i, b"[200~x\x1b[201~").is_empty());
        assert_eq!(
            feed(&mut i, b"\r"),
            ["k:0d", "p:1b5b491b5b3230781b5b32303078"]
        );
    }

    #[test]
    fn parse_script_takes_registered_events_with_defaults() {
        let steps =
            parse_script(r#"{"v":1,"steps":[{"event":"Stop"},{"event":"PreToolUse","variant":"ask","gate":true}]}"#)
                .expect("script");
        assert_eq!(
            steps,
            [
                Step {
                    event: "Stop".to_owned(),
                    variant: "default".to_owned(),
                    gate: false,
                    harness: false
                },
                Step {
                    event: "PreToolUse".to_owned(),
                    variant: "ask".to_owned(),
                    gate: true,
                    harness: false
                },
            ]
        );
        assert_eq!(parse_script(r#"{"v":2,"steps":[]}"#), None);
        assert_eq!(parse_script(r#"{"v":1,"steps":[{"event":"Other"}]}"#), None);
        assert_eq!(parse_script(r#"{"v":1}"#), None);
        assert_eq!(parse_script("x"), None);
    }

    #[test]
    fn hook_commands_take_only_command_entries_for_the_event() {
        let doc = r#"{"hooks":{
            "Stop":[{"hooks":[{"type":"command","command":"/a","args":["x","y"]},{"type":"prompt","command":"/b"}]},
                    {"matcher":"m","hooks":[{"type":"command","command":"/c"}]}],
            "PreToolUse":[{"hooks":[{"type":"command","command":"/d"}]}]}}"#;
        assert_eq!(
            hook_commands(doc, "Stop", None),
            [
                ("/a".to_owned(), vec!["x".to_owned(), "y".to_owned()]),
                ("/c".to_owned(), vec![])
            ]
        );
        assert!(hook_commands(doc, "SessionEnd", None).is_empty());
        assert!(hook_commands("not json", "Stop", None).is_empty());
    }

    /// A group's matcher is read against a tool-bearing payload's `tool_name`, name by name.
    #[test]
    fn hook_commands_evaluate_the_matcher_against_the_tool() {
        let doc = r#"{"hooks":{"PreToolUse":[
            {"matcher":"AskUserQuestion|ExitPlanMode","hooks":[{"type":"command","command":"/dialog"}]},
            {"matcher":"","hooks":[{"type":"command","command":"/empty"}]},
            {"hooks":[{"type":"command","command":"/all"}]}]}}"#;
        let commands = |tool| {
            hook_commands(doc, "PreToolUse", tool)
                .into_iter()
                .map(|(c, _)| c)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            commands(Some("ExitPlanMode")),
            ["/dialog", "/empty", "/all"]
        );
        assert_eq!(
            commands(Some("AskUserQuestion")),
            ["/dialog", "/empty", "/all"]
        );
        assert_eq!(commands(Some("Bash")), ["/empty", "/all"]);
        assert_eq!(commands(Some("Exit")), ["/empty", "/all"]);
        assert_eq!(commands(None), ["/dialog", "/empty", "/all"]);
    }

    #[test]
    fn payload_sets_only_the_prompt_field() {
        let fixture = br#"{"prompt":"old","session_id":"s"}"#.to_vec();
        let out: Value =
            serde_json::from_slice(&payload(fixture.clone(), &[("prompt", "new")])).expect("json");
        assert_eq!(out, json!({"prompt": "new", "session_id": "s"}));
        assert_eq!(payload(fixture.clone(), &[]), fixture);
        assert_eq!(payload(b"[1]".to_vec(), &[("prompt", "new")]), b"[1]");
        assert_eq!(
            payload(b"{\"prompt\":\"old\"}\n".to_vec(), &[("prompt", "new")]),
            b"{\"prompt\":\"new\"}\n"
        );
        assert_eq!(
            payload(b"{\"prompt\":\"old\"}".to_vec(), &[("prompt", "new")]),
            b"{\"prompt\":\"new\"}"
        );
    }

    /// The recorded default SessionStart of the default set, as committed.
    fn recorded_session_start() -> Vec<u8> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures")
            .join("claude")
            .join(DEFAULT_CLI_VERSION)
            .join("SessionStart.default.json");
        fs::read(path).expect("the recorded SessionStart")
    }

    #[test]
    fn fake_resume_sets_the_source_and_the_given_id_on_the_recorded_session_start() {
        let fixture = recorded_session_start();
        let id = "11111111-2222-4333-8444-555555555555";
        let opts = Opts::parse(&args(&["--resume", id]));
        assert_eq!(
            opts.resume_fields(),
            [("source", "resume"), ("session_id", id)]
        );
        let out = payload(fixture.clone(), &opts.resume_fields());
        assert!(fixture.ends_with(b"\n") && out.ends_with(b"\n"));
        assert_eq!(out.iter().filter(|b| **b == b'\n').count(), 1);
        let mut want: Value = serde_json::from_slice(&fixture).expect("the fixture");
        assert_eq!(want["source"], "startup");
        assert_eq!(want["session_id"], "c0f0cc23-690b-45dd-bbfb-06d6cfd44942");
        want["source"] = json!("resume");
        want["session_id"] = json!(id);
        let got: Value = serde_json::from_slice(&out).expect("json");
        assert_eq!(got, want);
        let keys = |v: &Value| {
            v.as_object()
                .expect("object")
                .keys()
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(
            keys(&got),
            [
                "session_id",
                "transcript_path",
                "cwd",
                "hook_event_name",
                "source"
            ]
        );
    }

    #[test]
    fn fake_resume_with_fork_session_reports_the_compiled_fork_id() {
        let opts = Opts::parse(&args(&[
            "--resume",
            "11111111-2222-4333-8444-555555555555",
            "--fork-session",
        ]));
        assert_eq!(
            opts.resume_fields(),
            [
                ("source", "resume"),
                ("session_id", "0f0e0d0c-0b0a-4908-8706-050403020100")
            ]
        );
        let out: Value =
            serde_json::from_slice(&payload(recorded_session_start(), &opts.resume_fields()))
                .expect("json");
        assert_eq!(out["session_id"], "0f0e0d0c-0b0a-4908-8706-050403020100");
        assert_eq!(out["source"], "resume");
    }

    /// Under `--resume` the launch fires the set's `resume-1` where it is recorded, else `default`;
    /// without `--resume` always `default`.
    #[test]
    fn fake_resume_launches_on_the_recorded_resume_variant_when_the_set_holds_one() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let set = tmp.path().join("3.0.0");
        fs::create_dir_all(&set).expect("set");
        let variant = |extra: &[&str]| {
            let mut list = vec![
                "--cli-version".to_owned(),
                "3.0.0".to_owned(),
                "--fixtures".to_owned(),
                tmp.path().to_str().expect("utf-8").to_owned(),
            ];
            list.extend(extra.iter().map(|a| (*a).to_owned()));
            Agent::new(Opts::parse(&list)).launch_variant()
        };
        let id = "11111111-2222-4333-8444-555555555555";
        assert_eq!(variant(&["--resume", id]), "default", "no variant recorded");
        fs::write(set.join("SessionStart.resume-1.json"), b"{}\n").expect("variant");
        assert_eq!(variant(&["--resume", id]), "resume-1");
        assert_eq!(variant(&[]), "default", "no --resume");
    }

    /// Without `--resume` nothing is set, `--fork-session` alone included: the recorded bytes.
    #[test]
    fn fake_resume_absent_leaves_the_recorded_bytes() {
        let fixture = recorded_session_start();
        for list in [&[][..], &["--fork-session"][..]] {
            let opts = Opts::parse(&args(list));
            assert!(opts.resume_fields().is_empty(), "{list:?}");
            assert_eq!(payload(fixture.clone(), &opts.resume_fields()), fixture);
        }
    }

    #[test]
    fn fake_statusline_words_are_the_command_split_on_ascii_white_space() {
        let words = |doc: &str| statusline_words(doc.as_bytes());
        assert_eq!(
            words(
                r#"{"statusLine":{"type":"command","command":"/h/bin/k/viola hook  statusline"}}"#
            ),
            Some((
                "/h/bin/k/viola".to_owned(),
                vec!["hook".to_owned(), "statusline".to_owned()]
            ))
        );
        assert_eq!(
            words(r#"{"statusLine":{"command":"relative"}}"#),
            Some(("relative".to_owned(), vec![]))
        );
        assert_eq!(words(r#"{"statusLine":{"command":"  "}}"#), None);
        assert_eq!(words(r#"{"statusLine":{"command":7}}"#), None);
        assert_eq!(words(r#"{"model":"m"}"#), None);
        assert_eq!(words("not json"), None);
    }

    #[test]
    fn fake_statusline_echo_exit_is_the_code_after_the_marker_path() {
        assert_eq!(statusline_echo_exit(&args(&["m"])), 0);
        assert_eq!(statusline_echo_exit(&args(&["m", "--exit", "3"])), 3);
        assert_eq!(statusline_echo_exit(&args(&["m", "--exit"])), 0);
        assert_eq!(statusline_echo_exit(&args(&["m", "--exit", "x"])), 0);
        assert_eq!(statusline_echo_exit(&args(&["m", "--other", "3"])), 0);
        assert_eq!(statusline_echo_exit(&args(&["--exit", "3"])), 0);
        assert_eq!(statusline_echo_exit(&[]), 0);
        assert_eq!(STATUSLINE_ECHO, "statusline-echo");
        assert_eq!(STATUSLINE_ECHO_OUTPUT, b"viola-fake-statusline\n");
    }

    /// Without both options, or with a stdin file that cannot be read, nothing runs and nothing is
    /// receipted; a command that is not absolute is receipted and never run.
    #[test]
    fn fake_statusline_runs_only_with_both_options_and_only_an_absolute_command() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = |name: &str| tmp.path().join(name).to_str().expect("utf-8").to_owned();
        let (settings, stdin, receipt) = (path("settings.json"), path("stdin"), path("receipt"));
        fs::write(&settings, r#"{"statusLine":{"command":"relative arg"}}"#).expect("settings");
        fs::write(&stdin, b"payload").expect("stdin");
        let run = |list: &[&str]| {
            let mut list = list.to_vec();
            list.extend(["--receipt", &receipt]);
            Agent::new(Opts::parse(&args(&list))).run_statusline();
            fs::read_to_string(&receipt).unwrap_or_default()
        };
        assert_eq!(run(&["--settings", &settings]), "");
        assert_eq!(run(&["--statusline-stdin", &stdin]), "");
        let missing = path("missing");
        assert_eq!(
            run(&["--settings", &settings, "--statusline-stdin", &missing]),
            ""
        );
        assert_eq!(
            run(&["--settings", &missing, "--statusline-stdin", &stdin]),
            ""
        );
        let line: Value = serde_json::from_str(
            run(&["--settings", &settings, "--statusline-stdin", &stdin]).trim_end(),
        )
        .expect("one receipt line");
        assert_eq!(
            line,
            json!({"v": 1, "kind": "statusline", "command_absolute": false, "ran": false})
        );
    }

    /// An absolute command is run with the stdin file's bytes and receipted in the hook receipt's
    /// form, without an `event`.
    #[test]
    fn fake_statusline_receipts_the_run_of_an_absolute_command() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = |name: &str| tmp.path().join(name).to_str().expect("utf-8").to_owned();
        let (settings, stdin, receipt) = (path("settings.json"), path("stdin"), path("receipt"));
        let doc = json!({"statusLine": {"type": "command", "command": whoami()}});
        fs::write(&settings, doc.to_string()).expect("settings");
        fs::write(&stdin, b"payload").expect("stdin");
        let list = [
            "--settings",
            &settings,
            "--statusline-stdin",
            &stdin,
            "--receipt",
            &receipt,
        ];
        Agent::new(Opts::parse(&args(&list))).run_statusline();
        let text = fs::read_to_string(&receipt).expect("the receipt");
        let line: Value = serde_json::from_str(text.trim_end()).expect("one receipt line");
        assert_eq!(line["kind"], "statusline");
        assert!(line.get("event").is_none());
        assert_eq!(line["command_absolute"], true);
        assert_eq!(line["ran"], true);
        assert_eq!(line["exit_code"], 0);
        assert_eq!(line["stdin_hex"], hex(b"payload"));
        assert!(line["stdout_hex"].as_str().is_some_and(|h| !h.is_empty()));
        assert!(line["stderr_len"].is_u64());
    }

    /// A set holding the two statusline variants and a settings file naming `command`, then the
    /// receipt lines the launch's replay and two turns under `--framing --turn-stop` leave, with the
    /// extra arguments given.
    fn statusline_replayed(command: &str, variants: bool, extra: &[&str]) -> Vec<Value> {
        let tmp = tempfile::tempdir().expect("tempdir");
        let set = tmp.path().join("fixtures").join("3.0.0");
        fs::create_dir_all(&set).expect("set");
        if variants {
            fs::write(set.join("StatusLine.default-1.json"), b"{\"n\":1}\n").expect("default");
            fs::write(set.join("StatusLine.rate-limits-1.json"), b"{\"n\":2}\n").expect("limits");
        }
        let settings = tmp.path().join("settings.json");
        let doc = json!({"statusLine": {"type": "command", "command": command}});
        fs::write(&settings, doc.to_string()).expect("settings");
        let receipt = tmp.path().join("receipt.ndjson");
        let utf8 = |p: PathBuf| p.to_str().expect("utf-8").to_owned();
        let mut list: Vec<String> = [
            "--cli-version".to_owned(),
            "3.0.0".to_owned(),
            "--fixtures".to_owned(),
            utf8(tmp.path().join("fixtures")),
            "--settings".to_owned(),
            utf8(settings),
            "--receipt".to_owned(),
            utf8(receipt.clone()),
            "--turn-stop".to_owned(),
        ]
        .into();
        list.extend(extra.iter().map(|a| (*a).to_owned()));
        let agent = Agent::new(Opts::parse(&list));
        agent.replay_statusline(STATUSLINE_DEFAULT);
        agent.end_turn(None);
        agent.end_turn(None);
        fs::read_to_string(&receipt)
            .unwrap_or_default()
            .lines()
            .filter_map(|l| serde_json::from_str::<Value>(l).ok())
            .collect()
    }

    #[test]
    fn fake_statusline_replays_the_default_then_the_rate_limits_once_under_framing() {
        let lines = statusline_replayed(&whoami(), true, &["--framing"]);
        let stdins: Vec<&str> = lines
            .iter()
            .filter(|l| l["kind"] == "statusline")
            .map(|l| l["stdin_hex"].as_str().unwrap_or(""))
            .collect();
        assert_eq!(stdins, [hex(b"{\"n\":1}\n"), hex(b"{\"n\":2}\n")]);
        assert!(lines.iter().all(|l| l["ran"] == true));
    }

    #[test]
    fn fake_statusline_replays_nothing_without_framing_or_without_the_variants() {
        assert!(statusline_replayed(&whoami(), true, &[]).is_empty());
        assert!(statusline_replayed(&whoami(), false, &["--framing"]).is_empty());
    }

    #[test]
    fn fake_statusline_replay_of_a_relative_command_runs_nothing() {
        let lines = statusline_replayed("relative hook statusline", true, &["--framing"]);
        assert_eq!(lines.len(), 2);
        for line in lines {
            assert_eq!(
                line,
                json!({"v": 1, "kind": "statusline", "command_absolute": false, "ran": false})
            );
        }
    }

    #[test]
    fn fake_statusline_hands_the_shell_name_word_over_as_bin_sh() {
        let words = |list: &[&str]| list.iter().map(|w| (*w).to_owned()).collect::<Vec<_>>();
        assert_eq!(
            shell_expanded(words(&["hook", "statusline", "--shell", "\"$0\""])),
            words(&["hook", "statusline", "--shell", "/bin/sh"])
        );
        assert_eq!(
            shell_expanded(words(&["$0", "'$0'", "\"$0\"x", "\"$1\""])),
            words(&["$0", "'$0'", "\"$0\"x", "\"$1\""])
        );
        assert_eq!(SHELL_NAME_WORD, "\"$0\"");
    }

    #[test]
    fn hex_is_lowercase_two_digits_per_byte() {
        assert_eq!(hex(&[0x00, 0x0a, 0xff]), "000aff");
    }

    /// An absolute host program that prints one line and exits by itself.
    fn whoami() -> String {
        #[cfg(windows)]
        let path = PathBuf::from(std::env::var_os("SystemRoot").expect("SystemRoot"))
            .join("System32")
            .join("whoami.exe");
        #[cfg(not(windows))]
        let path = PathBuf::from("/usr/bin/whoami");
        path.to_str().expect("utf-8 path").to_owned()
    }

    /// The `event` of each `hook` receipt line `replay_dialogs("probe")` writes over a set whose
    /// call `probe-1` holds only `events`. Every hook command is relative: receipted, never run.
    fn replayed(events: &[&str]) -> Vec<String> {
        let tmp = tempfile::tempdir().expect("tempdir");
        let set = tmp.path().join("fixtures").join("3.0.0");
        fs::create_dir_all(&set).expect("set");
        for event in events {
            fs::write(set.join(format!("{event}.probe-1.json")), b"{}").expect("fixture");
        }
        let hooks = tmp.path().join("plugin").join("hooks");
        fs::create_dir_all(&hooks).expect("hooks dir");
        let group = json!([{"hooks": [{"type": "command", "command": "relative"}]}]);
        let doc = json!({"hooks": {
            "PreToolUse": group, "PermissionRequest": group, "PostToolUse": group,
        }});
        fs::write(hooks.join("hooks.json"), doc.to_string()).expect("hooks.json");
        let receipt = tmp.path().join("receipt.ndjson");
        let utf8 = |p: PathBuf| p.to_str().expect("utf-8").to_owned();
        let list = [
            "--cli-version".to_owned(),
            "3.0.0".to_owned(),
            "--fixtures".to_owned(),
            utf8(tmp.path().join("fixtures")),
            "--plugin-dir".to_owned(),
            utf8(tmp.path().join("plugin")),
            "--receipt".to_owned(),
            utf8(receipt.clone()),
        ];
        Agent::new(Opts::parse(&list)).replay_dialogs("probe");
        fs::read_to_string(&receipt)
            .expect("receipt")
            .lines()
            .filter_map(|l| serde_json::from_str::<Value>(l).ok())
            .filter(|l| l["kind"] == "hook")
            .map(|l| l["event"].as_str().unwrap_or("").to_owned())
            .collect()
    }

    #[test]
    fn replay_dialogs_replays_a_call_holding_only_a_pre_tool_use() {
        assert_eq!(replayed(&["PreToolUse"]), ["PreToolUse"]);
    }

    #[test]
    fn replay_dialogs_replays_a_call_holding_only_a_permission_request() {
        assert_eq!(replayed(&["PermissionRequest"]), ["PermissionRequest"]);
    }

    #[test]
    fn replay_dialogs_replays_nothing_for_a_stem_with_no_call() {
        assert!(replayed(&[]).is_empty());
    }

    #[test]
    fn run_hook_after_close_hooks_runs_nothing() {
        let agent = Agent::new(Opts::parse(&[]));
        assert!(
            agent.run_hook("Notification", &whoami(), &[], b""),
            "the program ran and printed"
        );
        agent.close_hooks();
        assert!(!agent.run_hook("Notification", &whoami(), &[], b""));
    }

    #[test]
    fn run_hook_of_a_stop_with_the_receipt_hold_takes_at_least_the_hold() {
        let agent = Agent::new(Opts::parse(&args(&["--stop-receipt-hold-ms", "1000"])));
        let hold = agent.opts.stop_receipt_hold.expect("the hold");
        let started = Instant::now();
        assert!(agent.run_hook("Stop", &whoami(), &[], b""));
        assert!(started.elapsed() >= hold, "{:?}", started.elapsed());
    }

    #[test]
    fn submit_of_the_long_paste_with_a_paste_hint_takes_at_least_the_hint() {
        let agent = Agent::new(Opts::parse(&args(&[
            "--framing",
            "--turn-stop",
            "--paste-hint-ms",
            "300",
        ])));
        let hint = agent.opts.paste_hint.expect("the hint");
        let started = Instant::now();
        agent.submit(PROBE_LONG_PASTE.as_bytes(), "human");
        assert!(started.elapsed() >= hint, "{:?}", started.elapsed());
    }

    #[test]
    fn agents_output_of_each_mode_is_its_bytes() {
        let oversize = agents_output(Some("oversize"), None);
        assert_eq!(oversize.len() as u64, MAX_FRAME + 1);
        assert!(oversize.iter().all(|b| *b == b' '));
        let malformed = agents_output(Some("malformed"), None);
        assert!(serde_json::from_slice::<Value>(&malformed).is_err());
        assert_eq!(agents_output(None, None), b"[]\n");
        assert_eq!(agents_output(Some("later-word"), None), b"[]\n");
        assert_eq!(AGENTS, "agents");
    }

    #[test]
    fn agents_output_recorded_lists_the_planted_rows_unchanged_then_the_live_sessions() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let planted = r#"[{"pid":7,"sessionId":"s-1","status":"idle","later":1}]"#;
        fs::write(tmp.path().join(AGENTS_ROWS), format!("{planted}\n")).expect("rows");
        assert_eq!(
            agents_output(Some("recorded"), Some(tmp.path())),
            format!("{planted}\n").into_bytes()
        );
        let live = Registry::open(tmp.path(), "s-live", "~/w").expect("this process registers");
        live.set_status("busy");
        let listed: Value =
            serde_json::from_slice(&agents_output(None, Some(tmp.path()))).expect("json");
        assert_eq!(listed[1]["sessionId"], "s-live");
        assert_eq!(listed[1]["status"], "busy");
        assert_eq!(listed[1]["pid"], std::process::id());
        assert_eq!(listed[1]["name"], SESSION_NAME);
        assert_eq!(listed[1]["cwd"], "~/w");
        assert_eq!(listed.as_array().map(Vec::len), Some(2));
        live.remove();
        assert_eq!(
            agents_output(None, Some(tmp.path())),
            format!("{planted}\n").into_bytes()
        );
    }

    /// The registry row's id and cwd come from the launch SessionStart as recorded, `~` for a
    /// recording without a cwd; with no launch fixture there is no row.
    /// Under `--framing`, `/compact` fires its recorded SessionStart with the last fired
    /// SessionStart's `session_id` (the launch one, then a replayed clear's) and nothing else
    /// changed; a set without the variant fires nothing for it.
    #[test]
    fn fake_compact_replay_carries_the_last_fired_session_id() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let set = tmp.path().join("fixtures").join("3.0.0");
        fs::create_dir_all(&set).expect("set");
        let write = |name: &str, body: &str| fs::write(set.join(name), body).expect("fixture");
        write(
            "SessionStart.default.json",
            r#"{"session_id":"s-launch","source":"startup"}"#,
        );
        write("SessionEnd.clear-1.json", r#"{"reason":"clear"}"#);
        write(
            "SessionStart.clear-1.json",
            r#"{"session_id":"s-clear","source":"clear"}"#,
        );
        let hooks = tmp.path().join("plugin").join("hooks");
        fs::create_dir_all(&hooks).expect("hooks dir");
        let group = json!([{"hooks": [{"type": "command", "command": whoami()}]}]);
        let doc = json!({"hooks": {"SessionStart": group, "SessionEnd": group}});
        fs::write(hooks.join("hooks.json"), doc.to_string()).expect("hooks.json");
        let receipt = tmp.path().join("receipt.ndjson");
        let utf8 = |p: PathBuf| p.to_str().expect("utf-8").to_owned();
        let list = [
            "--cli-version".to_owned(),
            "3.0.0".to_owned(),
            "--fixtures".to_owned(),
            utf8(tmp.path().join("fixtures")),
            "--plugin-dir".to_owned(),
            utf8(tmp.path().join("plugin")),
            "--receipt".to_owned(),
            utf8(receipt.clone()),
            "--framing".to_owned(),
        ];
        let agent = Agent::new(Opts::parse(&list));
        agent.fire("SessionStart", "default", &[]);
        agent.submit(b"/compact", "driver");
        write(
            "SessionStart.compact-1.json",
            "{\"session_id\":\"s-recorded\",\"source\":\"compact\",\"cwd\":\"~/rec\"}\n",
        );
        agent.submit(b"/compact", "driver");
        agent.submit(b"/clear", "driver");
        agent.submit(b"/compact", "driver");
        let lines: Vec<Value> = fs::read_to_string(&receipt)
            .expect("receipt")
            .lines()
            .filter_map(|l| serde_json::from_str::<Value>(l).ok())
            .collect();
        let submits: Vec<&str> = lines
            .iter()
            .filter(|l| l["kind"] == "prompt")
            .map(|l| l["submit"].as_str().unwrap_or(""))
            .collect();
        assert_eq!(submits, ["no-fixture", "fired", "fired", "fired"]);
        let starts: Vec<Value> = lines
            .iter()
            .filter(|l| l["kind"] == "hook" && l["event"] == "SessionStart")
            .map(|l| {
                let stdin = l["stdin_hex"].as_str().expect("stdin_hex");
                let bytes: Vec<u8> = (0..stdin.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&stdin[i..i + 2], 16).expect("hex"))
                    .collect();
                serde_json::from_slice(&bytes).expect("json")
            })
            .collect();
        assert_eq!(
            starts,
            [
                json!({"session_id": "s-launch", "source": "startup"}),
                json!({"session_id": "s-launch", "source": "compact", "cwd": "~/rec"}),
                json!({"session_id": "s-clear", "source": "clear"}),
                json!({"session_id": "s-clear", "source": "compact", "cwd": "~/rec"}),
            ]
        );
        assert!(
            !lines
                .iter()
                .any(|l| l["kind"] == "hook" && l["event"] == "UserPromptSubmit"),
            "no UserPromptSubmit"
        );
    }

    #[test]
    fn launch_session_replays_the_recorded_id_and_cwd() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let set = tmp.path().join("9.9.9");
        fs::create_dir_all(&set).expect("set");
        let root = tmp.path().to_str().expect("utf-8").to_owned();
        let agent = |extra: &[&str]| {
            let mut a = vec!["--fixtures", root.as_str(), "--cli-version", "9.9.9"];
            a.extend_from_slice(extra);
            Agent::new(Opts::parse(&args(&a)))
        };
        assert_eq!(agent(&[]).launch_session(), None);
        fs::write(
            set.join("SessionStart.default.json"),
            br#"{"session_id":"s-1","cwd":"~/rec"}"#,
        )
        .expect("launch");
        assert_eq!(
            agent(&[]).launch_session(),
            Some(("s-1".to_owned(), "~/rec".to_owned()))
        );
        let resumed = agent(&["--resume", "s-9"]).launch_session();
        assert_eq!(resumed, Some(("s-9".to_owned(), "~/rec".to_owned())));
        fs::write(
            set.join("SessionStart.default.json"),
            br#"{"session_id":"s-1"}"#,
        )
        .expect("launch");
        assert_eq!(
            agent(&[]).launch_session(),
            Some(("s-1".to_owned(), "~".to_owned()))
        );
    }

    #[rstest::rstest]
    #[case::not_json("not json")]
    #[case::an_object(r#"{"pid":7}"#)]
    #[case::empty("")]
    fn planted_rows_of_a_malformed_rows_file_are_none(#[case] text: &str) {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::write(tmp.path().join(AGENTS_ROWS), text).expect("rows");
        assert!(planted_rows(tmp.path()).is_empty());
        assert!(planted_rows(&tmp.path().join("absent")).is_empty());
    }

    /// A row whose process is gone, or alive under another start time, is left out; this
    /// process's own row with its own start time is listed.
    #[test]
    fn live_sessions_skip_a_dead_or_reused_pid() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let sessions = tmp.path().join(SESSIONS);
        fs::create_dir_all(&sessions).expect("sessions");
        let mut child = Command::new(std::env::current_exe().expect("exe"))
            .arg("--list")
            .stdout(Stdio::null())
            .spawn()
            .expect("child");
        let dead = child.id();
        child.wait().expect("reaped");
        let own = std::process::id();
        let started = process_start_time(own).expect("own start") * 1000;
        for (file, pid, at) in [
            ("1.json", dead, started),
            ("2.json", own, 1000),
            ("3.json", own, started),
        ] {
            let row = json!({"pid": pid, "sessionId": file, "startedAt": at, "status": "idle"});
            fs::write(sessions.join(file), row.to_string()).expect("row");
        }
        fs::write(sessions.join("4.json"), b"not json").expect("torn");
        fs::write(sessions.join("5.json.next"), b"{}").expect("a sibling");
        let ids: Vec<Value> = live_sessions(tmp.path())
            .into_iter()
            .map(|r| r["sessionId"].clone())
            .collect();
        assert_eq!(ids, [json!("3.json")]);
    }
}
