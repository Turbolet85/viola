//! Test-only stand-in for the `claude` CLI (feature `fake-agent`, never in a release build).
//! Behaviour contract: test-plan §7 "Fake agent". Everything it observes goes to the `--receipt`
//! file; stdout carries only the `--version` answer, so wrapped and unwrapped runs stay comparable.

// It emulates the claude CLI on stdout/stderr; the workspace print bans are for product code.
#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::fs::{self, File, OpenOptions};
use std::io::{Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Value, json};

const DEFAULT_CLI_VERSION: &str = "2.1.0";
const PASTE_START: &[u8] = b"\x1b[200~";
const PASTE_END: &[u8] = b"\x1b[201~";
const HARNESS_TURN: &str = "<task-notification>synthetic harness turn</task-notification>";
/// The internal re-exec behind `--exit-no-eof`: a grandchild that holds the inherited stdout.
const HOLD_STDOUT: &str = "--hold-stdout-internal";
const HOLD_FOR: Duration = Duration::from_secs(10);
const CONTROL_POLL: Duration = Duration::from_millis(10);
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
    hold_stdout: bool,
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

/// The `command` + `args` of every `type: "command"` hook registered for `event`.
fn hook_commands(hooks_json: &str, event: &str) -> Vec<(String, Vec<String>)> {
    let Ok(doc) = serde_json::from_str::<Value>(hooks_json) else {
        return Vec::new();
    };
    let Some(groups) = doc["hooks"][event].as_array() else {
        return Vec::new();
    };
    groups
        .iter()
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

/// For `UserPromptSubmit` the documented `prompt` field carries the prompt; nothing else is built.
fn payload(fixture: Vec<u8>, prompt: Option<&str>) -> Vec<u8> {
    let Some(prompt) = prompt else { return fixture };
    match serde_json::from_slice::<Value>(&fixture) {
        Ok(Value::Object(mut obj)) => {
            obj.insert("prompt".to_owned(), json!(prompt));
            Value::Object(obj).to_string().into_bytes()
        }
        _ => fixture,
    }
}

struct Agent {
    opts: Opts,
    receipt: Receipt,
}

impl Agent {
    /// Fires `event`; the returned word is the prompt receipt's `submit` value.
    fn fire(&self, event: &str, variant: &str, prompt: Option<&str>) -> &'static str {
        let hooks = self
            .opts
            .plugin_dir
            .as_ref()
            .and_then(|d| fs::read_to_string(d.join("hooks").join("hooks.json")).ok());
        let commands = hooks.map_or_else(Vec::new, |h| hook_commands(&h, event));
        if commands.is_empty() {
            return "no-hooks";
        }
        let fixture = self.opts.fixtures.as_ref().and_then(|f| {
            fs::read(
                f.join(self.opts.cli_version())
                    .join(format!("{event}.{variant}.json")),
            )
            .ok()
        });
        let Some(fixture) = fixture else {
            return "no-fixture";
        };
        let body = payload(fixture, prompt);
        for (command, args) in commands {
            self.run_hook(event, &command, &args, &body);
        }
        "fired"
    }

    fn run_hook(&self, event: &str, command: &str, args: &[String], body: &[u8]) {
        if !Path::new(command).is_absolute() {
            self.receipt.write(
                "hook",
                json!({"event": event, "command_absolute": false, "ran": false}),
            );
            return;
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
        let fields = match output {
            Ok(out) => json!({
                "event": event,
                "command_absolute": true,
                "ran": true,
                "exit_code": out.status.code(),
                "stderr_len": out.stderr.len(),
                "stdout_hex": hex(&out.stdout),
            }),
            Err(_) => json!({"event": event, "command_absolute": true, "ran": false}),
        };
        self.receipt.write("hook", fields);
    }

    fn submit(&self, bytes: &[u8], origin: &str) {
        let text = String::from_utf8_lossy(bytes);
        let submit = if self.opts.suppress_prompt_submit {
            "suppressed"
        } else if self.opts.local_command_mode && text.starts_with('/') {
            "local-command"
        } else {
            self.fire("UserPromptSubmit", "default", Some(&text))
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
                self.fire(&step.event, &step.variant, None);
            }
        }
    }
}

fn start_receipts(agent: &Agent) {
    agent
        .receipt
        .write("start", json!({"cli_version": agent.opts.cli_version()}));
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

fn hold_inherited_stdout() {
    let _ = Command::new(std::env::current_exe().unwrap_or_default())
        .arg(HOLD_STDOUT)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
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

/// The terminal size as a `size` receipt, written at start and before any byte read after it
/// changed (the resize oracle); nothing when stdin/stdout is not a terminal.
fn receipt_size(agent: &Agent, last: &mut Option<viola_pty::Size>) {
    let now = viola_pty::host_size();
    if let Some(size) = now.filter(|s| Some(*s) != *last) {
        agent
            .receipt
            .write("size", json!({"cols": size.cols, "rows": size.rows}));
        *last = now;
    }
}

/// Reads stdin byte by byte until EOF or `\x03`.
fn read_stdin(agent: &Agent) -> ExitCode {
    let mut input = Input::new();
    let mut stdin = std::io::stdin().lock();
    let mut byte = [0u8; 1];
    let mut size = None;
    receipt_size(agent, &mut size);
    while let Ok(1) = stdin.read(&mut byte) {
        receipt_size(agent, &mut size);
        for action in input.feed(byte[0]) {
            match action {
                Action::Key(b) => agent.receipt.write("key", json!({"hex": hex(&[b])})),
                Action::Submit(bytes) => agent.submit(&bytes, "human"),
                Action::Exit => {
                    if agent.opts.exit_no_eof {
                        hold_inherited_stdout();
                    }
                    return ExitCode::SUCCESS;
                }
            }
        }
    }
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let opts = Opts::parse(&args);
    if opts.hold_stdout {
        std::thread::sleep(HOLD_FOR);
        return ExitCode::SUCCESS;
    }
    if opts.version {
        println!("{}", opts.version_answer());
        return ExitCode::SUCCESS;
    }
    let Some(steps) = script_steps(&opts) else {
        eprintln!("fake agent: unreadable script");
        return ExitCode::from(2);
    };
    let agent = Arc::new(Agent {
        receipt: Receipt::open(opts.receipt.as_deref()),
        opts,
    });
    // Raw like the real CLI: on a cooked terminal keys wait for Enter and `\x03` never arrives.
    // The `start` receipt below is written only once the mode is set.
    let _terminal = viola_pty::HostTerminal::enter();
    start_receipts(&agent);
    if !steps.is_empty() {
        let runner = Arc::clone(&agent);
        std::thread::spawn(move || runner.run_steps(&steps));
    }
    read_stdin(&agent)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            "--version",
        ]));
        assert_eq!(o.cli_version(), "3.0.0");
        assert_eq!(o.version_answer(), "4.0.0 (Claude Code)");
        assert_eq!(o.script.as_deref(), Some(Path::new("s.json")));
        assert_eq!(o.control.as_deref(), Some(Path::new("c")));
        assert_eq!(o.receipt.as_deref(), Some(Path::new("r")));
        assert_eq!(o.fixtures.as_deref(), Some(Path::new("f")));
        assert_eq!(o.plugin_dir.as_deref(), Some(Path::new("p")));
        assert!(o.version && o.suppress_prompt_submit && o.local_command_mode);
        assert!(o.inject_harness_turn && o.exit_no_eof && !o.hold_stdout);
        assert!(Opts::parse(&args(&[HOLD_STDOUT])).hold_stdout);
    }

    #[test]
    fn opts_default_to_the_default_cli_version() {
        let o = Opts::parse(&[]);
        assert_eq!(o.cli_version(), DEFAULT_CLI_VERSION);
        assert_eq!(o.version_answer(), "2.1.0 (Claude Code)");
        assert!(!o.version && !o.exit_no_eof && !o.suppress_prompt_submit);
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
            hook_commands(doc, "Stop"),
            [
                ("/a".to_owned(), vec!["x".to_owned(), "y".to_owned()]),
                ("/c".to_owned(), vec![])
            ]
        );
        assert!(hook_commands(doc, "SessionEnd").is_empty());
        assert!(hook_commands("not json", "Stop").is_empty());
    }

    #[test]
    fn payload_sets_only_the_prompt_field() {
        let fixture = br#"{"prompt":"old","session_id":"s"}"#.to_vec();
        let out: Value =
            serde_json::from_slice(&payload(fixture.clone(), Some("new"))).expect("json");
        assert_eq!(out, json!({"prompt": "new", "session_id": "s"}));
        assert_eq!(payload(fixture.clone(), None), fixture);
        assert_eq!(payload(b"[1]".to_vec(), Some("new")), b"[1]");
    }

    #[test]
    fn hex_is_lowercase_two_digits_per_byte() {
        assert_eq!(hex(&[0x00, 0x0a, 0xff]), "000aff");
    }
}
