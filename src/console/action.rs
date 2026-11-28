//! The console's actions: each runs its CLI verb as a direct child of the console's own exe,
//! `--home` passed, with no shell, `VIOLA_NAME` and `VIOLA_DIR` removed from its env so the child
//! acts as the human, never as a wrapped session. Its stdout and stderr are each read through
//! `MAX_FRAME` on a thread of their own and come back, row-escaped, with its exit code, on a
//! channel. A child is not tied to the console: one running at a quit runs to its verb's own end.

use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};

use viola_core::MAX_FRAME;

use crate::human::escape_row;

/// The instance names a wrapped session's console inherits; none reaches a child.
pub(crate) const STRIPPED: [&str; 2] = ["VIOLA_NAME", "VIOLA_DIR"];

/// The verbs an action runs: its `console-action` word, which is also the CLI verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Verb {
    Last,
    Pause,
    Release,
    Focus,
    /// The revive action: `viola up <name>`, which re-opens the instance in a terminal of its
    /// own (a passed revive takes the terminal it runs in).
    Up,
    /// The editing line's text on the child's stdin.
    Send,
    /// The editing line's words, each one `--effort=` or `--model=` element.
    Switch,
    /// `viola open <typed> --parent <name> -- claude`: the typed text one element.
    Open,
    /// `viola answer <name> <dialog_id>` with a response body on the child's stdin.
    Answer,
}

impl Verb {
    /// The CLI verb and the `console-action` line's `verb` word.
    pub(crate) const fn word(self) -> &'static str {
        match self {
            Self::Last => "last",
            Self::Pause => "pause",
            Self::Release => "release",
            Self::Focus => "focus",
            Self::Up => "up",
            Self::Send => "send",
            Self::Switch => "switch",
            Self::Open => "open",
            Self::Answer => "answer",
        }
    }

    /// The word the human reads on the running line and the confirm.
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Up => "revive",
            other => other.word(),
        }
    }
}

/// One action: a verb on a selected name, any further arguments, and an optional stdin text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Action {
    pub(crate) verb: Verb,
    pub(crate) name: String,
    pub(crate) args: Vec<String>,
    pub(crate) stdin: Option<String>,
}

impl Action {
    pub(crate) fn on(verb: Verb, name: &str) -> Self {
        Self {
            verb,
            name: name.to_owned(),
            args: Vec::new(),
            stdin: None,
        }
    }

    /// `send <name>` with `text` on its stdin, unchanged.
    pub(crate) fn send(name: &str, text: &str) -> Self {
        Self {
            stdin: Some(text.to_owned()),
            ..Self::on(Verb::Send, name)
        }
    }

    /// `switch <name>` with each space-split word one element: an effort word `--effort=<word>`,
    /// any other `--model=<word>`, so a typed word stays a value and is never read as a flag.
    pub(crate) fn switch(name: &str, text: &str) -> Self {
        let args = text
            .split(' ')
            .filter(|word| !word.is_empty())
            .map(|word| {
                if viola_agent_claude::is_effort_level(word) {
                    format!("--effort={word}")
                } else {
                    format!("--model={word}")
                }
            })
            .collect();
        Self {
            args,
            ..Self::on(Verb::Switch, name)
        }
    }

    /// `open <typed> --parent <name> -- claude`: all else open's default.
    pub(crate) fn open(name: &str, typed: &str) -> Self {
        Self {
            args: [typed, "--parent", name, "--", "claude"]
                .map(str::to_owned)
                .to_vec(),
            ..Self::on(Verb::Open, name)
        }
    }

    /// `answer <name> <dialog_id>` with `{"behavior":"<behavior>"}` on its stdin.
    pub(crate) fn answer(name: &str, dialog_id: u64, behavior: Behavior) -> Self {
        Self {
            args: vec![dialog_id.to_string()],
            stdin: Some(format!(r#"{{"behavior":"{}"}}"#, behavior.word())),
            ..Self::on(Verb::Answer, name)
        }
    }
}

/// The four answers the console gives: a permission's allow or deny, a plan's approve or revise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Behavior {
    Allow,
    Deny,
    Approve,
    Revise,
}

impl Behavior {
    pub(crate) const fn word(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
            Self::Approve => "approve",
            Self::Revise => "revise",
        }
    }
}

/// A finished child: its verb, the name it acted on, its exit code (−1 when it could not be
/// spawned or ended by a signal) and its captured lines, stdout's then stderr's, each escaped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Outcome {
    pub(crate) verb: Verb,
    pub(crate) name: String,
    pub(crate) exit_code: i32,
    pub(crate) lines: Vec<String>,
}

/// What every child is run from: the console's own canonical exe and home.
#[derive(Debug, Clone)]
pub(crate) struct Runner {
    exe: PathBuf,
    home: PathBuf,
}

impl Runner {
    pub(crate) fn new(exe: PathBuf, home: PathBuf) -> Self {
        Self { exe, home }
    }

    /// `<exe> --home <home> <verb> <name> <args…>` (for open `<args…>` alone, which name the
    /// selected row as `--parent`), spawned directly with the console's env less the instance
    /// names and its cwd; stdin null or piped for a text, stdout and stderr piped.
    pub(crate) fn command(&self, action: &Action) -> Command {
        let mut command = Command::new(&self.exe);
        command
            .arg("--home")
            .arg(&self.home)
            .arg(action.verb.word());
        if action.verb != Verb::Open {
            command.arg(&action.name);
        }
        command.args(&action.args);
        for name in STRIPPED {
            command.env_remove(name);
        }
        let stdin = if action.stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        };
        command
            .stdin(stdin)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        command
    }

    /// Runs `action` on a thread of its own; its outcome comes on the channel returned.
    pub(crate) fn spawn(&self, action: Action) -> Receiver<Outcome> {
        let (tx, rx) = mpsc::channel();
        let command = self.command(&action);
        thread::spawn(move || {
            let _ = tx.send(run(command, action));
        });
        rx
    }
}

fn run(mut command: Command, action: Action) -> Outcome {
    let (exit_code, lines) = match command.spawn() {
        Ok(child) => finish(child, action.stdin.as_deref()),
        Err(_) => (-1, Vec::new()),
    };
    Outcome {
        verb: action.verb,
        name: action.name,
        exit_code,
        lines,
    }
}

/// Writes the text and closes stdin, reads both streams to their end through `MAX_FRAME`, then
/// waits for the child.
fn finish(mut child: Child, text: Option<&str>) -> (i32, Vec<String>) {
    let out = child.stdout.take().map(drain);
    let err = child.stderr.take().map(drain);
    if let (Some(mut stdin), Some(text)) = (child.stdin.take(), text) {
        let _ = stdin.write_all(text.as_bytes());
    }
    let (out, err) = (joined(out), joined(err));
    let code = child
        .wait()
        .ok()
        .and_then(|status| status.code())
        .unwrap_or(-1);
    let mut lines = lines_of(&out);
    lines.extend(lines_of(&err));
    (code, lines)
}

fn drain<R: Read + Send + 'static>(mut stream: R) -> JoinHandle<Vec<u8>> {
    thread::spawn(move || {
        let mut bytes = Vec::new();
        let _ = (&mut stream).take(MAX_FRAME).read_to_end(&mut bytes);
        // Past the cap the rest is read and dropped, so the child never blocks on a full pipe.
        let _ = io::copy(&mut stream, &mut io::sink());
        bytes
    })
}

fn joined(handle: Option<JoinHandle<Vec<u8>>>) -> Vec<u8> {
    handle.and_then(|h| h.join().ok()).unwrap_or_default()
}

/// A stream's bytes as its lines, split on LF, each row-escaped: no control character of a
/// child's output reaches the console's surface. A last empty line is no line.
pub(crate) fn lines_of(bytes: &[u8]) -> Vec<String> {
    let text = String::from_utf8_lossy(bytes);
    let text = text.strip_suffix('\n').unwrap_or(&text);
    if text.is_empty() {
        return Vec::new();
    }
    text.split('\n').map(escape_row).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    fn runner() -> Runner {
        Runner::new(PathBuf::from("/opt/viola"), PathBuf::from("/h/viola-home"))
    }

    #[test]
    fn action_child_env_drops_the_instance_names() {
        let command = runner().command(&Action::on(Verb::Pause, "builder"));
        let removed: Vec<&OsStr> = command
            .get_envs()
            .filter(|(_, value)| value.is_none())
            .map(|(key, _)| key)
            .collect();
        assert_eq!(removed, [OsStr::new("VIOLA_DIR"), OsStr::new("VIOLA_NAME")]);
        assert!(
            command.get_envs().all(|(_, value)| value.is_none()),
            "no variable is set"
        );
    }

    #[test]
    fn action_child_is_the_own_exe_with_home_then_the_verb_and_name() {
        let mut action = Action::on(Verb::Up, "builder");
        action.args = vec!["--json".to_owned()];
        let command = runner().command(&action);
        assert_eq!(command.get_program(), OsStr::new("/opt/viola"));
        let args: Vec<&OsStr> = command.get_args().collect();
        assert_eq!(
            args,
            ["--home", "/h/viola-home", "up", "builder", "--json"].map(OsStr::new)
        );
        assert_eq!(command.get_current_dir(), None, "the console's own cwd");
    }

    #[test]
    fn action_verb_words_are_the_cli_verbs() {
        let verbs = [
            Verb::Last,
            Verb::Pause,
            Verb::Release,
            Verb::Focus,
            Verb::Up,
            Verb::Send,
            Verb::Switch,
            Verb::Open,
            Verb::Answer,
        ];
        assert_eq!(
            verbs.map(Verb::word),
            [
                "last", "pause", "release", "focus", "up", "send", "switch", "open", "answer"
            ]
        );
        assert_eq!(
            verbs.map(Verb::label),
            [
                "last", "pause", "release", "focus", "revive", "send", "switch", "open", "answer"
            ]
        );
    }

    fn args_of(action: &Action) -> Vec<String> {
        runner()
            .command(action)
            .get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect()
    }

    #[test]
    fn action_send_puts_the_text_on_stdin_unchanged() {
        let action = Action::send("builder", "  привет  ");
        assert_eq!(action.stdin.as_deref(), Some("  привет  "));
        assert_eq!(
            args_of(&action),
            ["--home", "/h/viola-home", "send", "builder"]
        );
    }

    #[test]
    fn action_switch_words_are_values_never_flags() {
        let action = Action::switch("builder", "opus  high --home /tmp");
        assert_eq!(
            args_of(&action),
            [
                "--home",
                "/h/viola-home",
                "switch",
                "builder",
                "--model=opus",
                "--effort=high",
                "--model=--home",
                "--model=/tmp",
            ]
        );
        assert_eq!(action.stdin, None);
    }

    #[test]
    fn action_open_takes_the_typed_name_as_one_element() {
        let action = Action::open("builder", "--cwd /etc");
        assert_eq!(
            args_of(&action),
            [
                "--home",
                "/h/viola-home",
                "open",
                "--cwd /etc",
                "--parent",
                "builder",
                "--",
                "claude",
            ]
        );
    }

    #[test]
    fn action_answer_bodies_are_the_four_response_shapes() {
        for (behavior, body) in [
            (Behavior::Allow, r#"{"behavior":"allow"}"#),
            (Behavior::Deny, r#"{"behavior":"deny"}"#),
            (Behavior::Approve, r#"{"behavior":"approve"}"#),
            (Behavior::Revise, r#"{"behavior":"revise"}"#),
        ] {
            let action = Action::answer("builder", 7, behavior);
            assert_eq!(action.stdin.as_deref(), Some(body));
            assert_eq!(
                args_of(&action),
                ["--home", "/h/viola-home", "answer", "builder", "7"]
            );
            let value: serde_json::Value = serde_json::from_str(body).expect("json");
            assert!(viola_agent_claude::dialog::Response::parse(&value).is_ok());
        }
    }

    #[test]
    fn action_output_escapes_every_control() {
        let lines = lines_of(b"ok\x1b[2J\r\tdone\n\xc2\x9bnext\x7f\n");
        assert_eq!(lines, ["ok\\x1B[2J\\x0D\\x09done", "\\x9Bnext\\x7F"]);
        for line in &lines {
            assert!(!line.chars().any(char::is_control), "{line:?}");
        }
    }

    #[test]
    fn action_output_splits_on_lf_and_drops_the_last_empty_line() {
        assert_eq!(lines_of(b""), Vec::<String>::new());
        assert_eq!(lines_of(b"one\n"), ["one"]);
        assert_eq!(lines_of(b"one\n\ntwo"), ["one", "", "two"]);
    }

    /// A planted program standing in for the exe: it prints a line on each stream and exits 3.
    #[test]
    fn action_spawn_returns_the_exit_code_and_both_streams() {
        let dir = tempfile::tempdir().expect("tempdir");
        let exe = dir.path().join("stand-in");
        std::fs::write(
            &exe,
            "#!/bin/sh\nprintf 'out %s\\n' \"$4\"\nprintf 'err\\n' >&2\nexit 3\n",
        )
        .expect("the stand-in");
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&exe, std::fs::Permissions::from_mode(0o700)).expect("chmod");
        let runner = Runner::new(exe, dir.path().to_owned());
        let outcome = runner
            .spawn(Action::on(Verb::Last, "builder"))
            .recv()
            .expect("an outcome");
        assert_eq!(
            outcome,
            Outcome {
                verb: Verb::Last,
                name: "builder".to_owned(),
                exit_code: 3,
                lines: vec!["out builder".to_owned(), "err".to_owned()],
            }
        );
    }

    #[test]
    fn action_spawn_of_a_missing_exe_reads_minus_one() {
        let runner = Runner::new(PathBuf::from("/nonexistent/viola"), PathBuf::from("/h"));
        let outcome = runner
            .spawn(Action::on(Verb::Pause, "builder"))
            .recv()
            .expect("an outcome");
        assert_eq!((outcome.exit_code, outcome.lines.len()), (-1, 0));
    }
}
