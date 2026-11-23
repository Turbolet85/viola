//! `viola-hypr-tap`: the test-only recorder of the relied-on `hyprctl` replies, read through the
//! seam's own read (`viola_hyprland::read`) on the live host, written as the per-version fixture set
//! `fixtures/hyprland/<version>/` that the fake `hyprctl` replays (test-plan §7 Fixture hygiene).
//!
//! - `record --program <abs> --out <dir>`: `version`, then `clients`, `activewindow`, `workspaces`,
//!   then `version` once more with `HYPRLAND_INSTANCE_SIGNATURE` taken out of the child's
//!   environment (the `absent` variant: the reply with no instance named).
//! - `record-dispatch --program <abs> --out <dir>`: `activewindow`, then a focus dispatch on that same
//!   window and `workspace` on its own workspace: the desktop does not change.
//! - `record-monitors --program <abs> --out <dir>`: one read-only `monitors` read, written alone.
//! - `record-move --program <abs> --terminal <abs> --workspace <i32> --out <dir>`: the terminal
//!   spawned directly, its own window found by its pid in `clients`, moved once to the workspace,
//!   seen there, the terminal killed and reaped and its window seen gone; then the move written
//!   alone. Every failure is a closed code on stderr, never an address, pid, title or path.
//!
//! Each reply keeps a string only where its key stands on the request's closed allowlist in the
//! form the seam parses (`monitors` keeps none, a dispatch only its success text); every other
//! string, at any depth, becomes the placeholder. The recording is
//! then scrubbed (home → `~`, the user word → `<user>`) and one still holding a home, an absolute
//! path, the user word or an email is refused by file name and closed code, never by its content:
//! nothing is written. Files are 0600 in a 0700 directory.

use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitCode, Stdio};
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use viola_agent_claude::ledger;
use viola_hyprland::reply::DISPATCH_OK;
use viola_hyprland::{
    HyprlandVersion, Read, Reply, Request, Window, WorkspaceId, is_address, read,
};

const USAGE: &str = "usage: viola-hypr-tap record|record-dispatch|record-monitors --program <absolute hyprctl> --out <dir> | record-move --program <absolute hyprctl> --terminal <absolute terminal> --workspace <i32> --out <dir>";
/// The argv the terminal is started with: a window of its own, running only a sleep that ends it
/// by itself should the tap die.
const TERMINAL_ARGS: [&str; 4] = [
    "--app-id=viola.hypr-tap",
    "--title=viola-hypr-tap",
    "/usr/bin/sleep",
    "30",
];
/// How often, and for how long, `clients` is read for the terminal's window to stand or leave.
const WINDOW_POLL: Duration = Duration::from_millis(100);
const WINDOW_WAIT: Duration = Duration::from_secs(5);
/// What every string off the allowlist is replaced with.
const PLACEHOLDER: &str = "viola fixture text";
/// The variable whose absence makes `hyprctl` name no instance.
const INSTANCE_VAR: &str = "HYPRLAND_INSTANCE_SIGNATURE";
const RECORDED: &str = "recorded-1";
const ABSENT: &str = "absent-1";

enum Verb {
    Record,
    RecordDispatch,
    RecordMonitors,
    RecordMove {
        terminal: PathBuf,
        workspace: WorkspaceId,
    },
}

/// The verb, the program and the out dir, or none for any other argument form.
fn parse_args(args: &[String]) -> Option<(Verb, PathBuf, PathBuf)> {
    let (verb, program, out) = match args {
        [verb, p, program, o, out] if p == "--program" && o == "--out" => {
            let verb = match verb.as_str() {
                "record" => Verb::Record,
                "record-dispatch" => Verb::RecordDispatch,
                "record-monitors" => Verb::RecordMonitors,
                _ => return None,
            };
            (verb, program, out)
        }
        [verb, p, program, t, terminal, w, workspace, o, out]
            if verb == "record-move"
                && p == "--program"
                && t == "--terminal"
                && w == "--workspace"
                && o == "--out" =>
        {
            let terminal = PathBuf::from(terminal);
            if !terminal.is_absolute() {
                return None;
            }
            let workspace = WorkspaceId::new(workspace.parse::<i32>().ok()?);
            (
                Verb::RecordMove {
                    terminal,
                    workspace,
                },
                program,
                out,
            )
        }
        _ => return None,
    };
    let program = PathBuf::from(program);
    program
        .is_absolute()
        .then(|| (verb, program, PathBuf::from(out)))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some((verb, program, out)) = parse_args(&args) else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    let recorded = match verb {
        Verb::Record => record(&program),
        Verb::RecordDispatch => record_dispatch(&program),
        Verb::RecordMonitors => record_monitors(&program),
        Verb::RecordMove {
            terminal,
            workspace,
        } => record_move(&program, &terminal, workspace),
    };
    let (version, files) = match recorded {
        Ok(recorded) => recorded,
        Err(why) => {
            eprintln!("viola-hypr-tap: {why}");
            return ExitCode::from(1);
        }
    };
    match write_set(&out.join(version.to_string()), &files) {
        Ok(()) => ExitCode::SUCCESS,
        Err(why) => {
            eprintln!("viola-hypr-tap: {why}");
            ExitCode::from(1)
        }
    }
}

/// One fixture to write: its file name and its document.
type Fixture = (String, Value);

fn version_of(program: &Path) -> Result<HyprlandVersion, &'static str> {
    match read(Some(program), &Request::Version, &[]).reply {
        Ok(Reply::Version(version)) => Ok(version),
        _ => Err("version-unread"),
    }
}

fn record(program: &Path) -> Result<(HyprlandVersion, Vec<Fixture>), &'static str> {
    let version = version_of(program)?;
    let mut files = Vec::new();
    for request in [
        Request::Version,
        Request::Clients,
        Request::ActiveWindow,
        Request::Workspaces,
    ] {
        let got = read(Some(program), &request, &[]);
        files.push(fixture(version, &request, &got, RECORDED));
    }
    let absent = read(
        Some(program),
        &Request::Version,
        &[OsString::from(INSTANCE_VAR)],
    );
    files.push(fixture(version, &Request::Version, &absent, ABSENT));
    Ok((version, files))
}

fn record_dispatch(program: &Path) -> Result<(HyprlandVersion, Vec<Fixture>), &'static str> {
    let version = version_of(program)?;
    let Ok(Reply::ActiveWindow(Some(window))) =
        read(Some(program), &Request::ActiveWindow, &[]).reply
    else {
        return Err("no-active-window");
    };
    let Some(workspace) = window.workspace else {
        return Err("no-active-workspace");
    };
    let mut files = Vec::new();
    for request in [
        Request::FocusWindow(window.address.clone()),
        Request::Workspace(workspace),
    ] {
        let got = read(Some(program), &request, &[]);
        files.push(fixture(version, &request, &got, RECORDED));
    }
    Ok((version, files))
}

fn record_monitors(program: &Path) -> Result<(HyprlandVersion, Vec<Fixture>), &'static str> {
    let version = version_of(program)?;
    let got = read(Some(program), &Request::Monitors, &[]);
    Ok((
        version,
        vec![fixture(version, &Request::Monitors, &got, RECORDED)],
    ))
}

/// The terminal the tap started, killed and reaped when it is ended or dropped.
struct Terminal(Child);

impl Terminal {
    fn end(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.end();
    }
}

/// The windows `clients` lists, or none when the read gave no list.
fn windows(program: &Path) -> Option<Vec<Window>> {
    match read(Some(program), &Request::Clients, &[]).reply {
        Ok(Reply::Clients(windows)) => Some(windows),
        _ => None,
    }
}

fn window_of(program: &Path, pid: u32) -> Option<Window> {
    windows(program)?
        .into_iter()
        .find(|window| window.pid == Some(pid))
}

/// `probe` read every `WINDOW_POLL` until it gives a value, for at most `WINDOW_WAIT`.
fn within<T>(mut probe: impl FnMut() -> Option<T>) -> Option<T> {
    let until = Instant::now() + WINDOW_WAIT;
    loop {
        if let Some(found) = probe() {
            return Some(found);
        }
        if Instant::now() >= until {
            return None;
        }
        std::thread::sleep(WINDOW_POLL);
    }
}

fn record_move(
    program: &Path,
    terminal: &Path,
    workspace: WorkspaceId,
) -> Result<(HyprlandVersion, Vec<Fixture>), &'static str> {
    let version = version_of(program)?;
    let child = Command::new(terminal)
        .args(TERMINAL_ARGS)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| "no-terminal-window")?;
    let mut terminal = Terminal(child);
    let pid = terminal.0.id();
    let window = within(|| window_of(program, pid)).ok_or("no-terminal-window")?;
    let request = Request::MoveToWorkspace(window.address, workspace);
    let got = read(Some(program), &request, &[]);
    if !matches!(got.reply, Ok(Reply::Dispatched)) {
        return Err("move-refused");
    }
    if window_of(program, pid).and_then(|w| w.workspace) != Some(workspace) {
        return Err("move-unseen");
    }
    terminal.end();
    let gone = || {
        windows(program)
            .filter(|all| all.iter().all(|w| w.pid != Some(pid)))
            .map(|_| ())
    };
    within(gone).ok_or("window-left")?;
    Ok((version, vec![fixture(version, &request, &got, RECORDED)]))
}

/// `FocusWindow` for `focus-window`: the fixture file stem of a request word.
fn stem(word: &str) -> String {
    word.split('-')
        .map(|part| {
            let mut chars = part.chars();
            chars
                .next()
                .map(|first| first.to_ascii_uppercase().to_string() + chars.as_str())
                .unwrap_or_default()
        })
        .collect()
}

fn fixture(version: HyprlandVersion, request: &Request, got: &Read, variant: &str) -> Fixture {
    let args: Vec<String> = request
        .args()
        .into_iter()
        .map(|a| a.to_string_lossy().into_owned())
        .collect();
    let doc = json!({
        "v": 1,
        "hyprland": version.to_string(),
        "request": request.word(),
        "args": args,
        "exit": got.exit,
        "stdout": kept_stdout(request.word(), &got.stdout),
    });
    (format!("{}.{variant}.json", stem(request.word())), doc)
}

/// Whether `value`, standing under `key` in a reply to `word`, is kept: the key on that request's
/// allowlist and the value in the form the seam parses.
fn allowed(word: &str, key: &str, value: &str) -> bool {
    match (word, key) {
        ("clients" | "active-window", "address" | "swallowing" | "grouped") => is_address(value),
        ("workspaces", "lastwindow") => is_address(value),
        ("version", "tag") => HyprlandVersion::from_tag(value).is_some(),
        _ => false,
    }
}

/// `value` with every string off the allowlist replaced by the placeholder, at any depth; an array's
/// entries stand under the array's own key.
fn keep(word: &str, key: Option<&str>, value: &Value) -> Value {
    match value {
        Value::String(s) if key.is_some_and(|k| allowed(word, k, s)) => value.clone(),
        Value::String(_) => Value::from(PLACEHOLDER),
        Value::Array(items) => items.iter().map(|v| keep(word, key, v)).collect(),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), keep(word, Some(k), v)))
                .collect(),
        ),
        other => other.clone(),
    }
}

/// The reply as recorded: a JSON reply with only its allowlisted strings kept; any other reply kept
/// only when it is the dispatch's success text, else the placeholder whole.
fn kept_stdout(word: &str, stdout: &[u8]) -> String {
    match serde_json::from_slice::<Value>(stdout) {
        Ok(value) => keep(word, None, &value).to_string(),
        Err(_) => {
            let text = stdout.strip_suffix(b"\n").unwrap_or(stdout);
            if text == DISPATCH_OK.as_bytes() {
                String::from_utf8_lossy(stdout).into_owned()
            } else {
                PLACEHOLDER.to_owned()
            }
        }
    }
}

fn texts<'a>(value: &'a Value, out: &mut Vec<&'a str>) {
    match value {
        Value::String(s) => out.push(s),
        Value::Array(items) => items.iter().for_each(|v| texts(v, out)),
        Value::Object(map) => map.iter().for_each(|(k, v)| {
            out.push(k);
            texts(v, out);
        }),
        _ => {}
    }
}

/// The first closed code a document earns: each string and key checked as one row.
fn fault(doc: &Value, home: &str, user: &str) -> Option<&'static str> {
    let mut all = Vec::new();
    texts(doc, &mut all);
    all.into_iter().find_map(|text| {
        let row = [text.to_owned()];
        ledger::screen_fault(&row, &row, home, user).map(|f| f.why.code())
    })
}

/// Each fixture scrubbed, or the first one still unclean, by file name and closed code.
fn cleaned(files: &[Fixture], home: &str, user: &str) -> Result<Vec<Fixture>, String> {
    files
        .iter()
        .map(|(name, doc)| {
            let scrubbed = ledger::scrub(doc, home, user);
            match fault(&scrubbed, home, user) {
                Some(code) => Err(format!("{name} {code}")),
                None => Ok((name.clone(), scrubbed)),
            }
        })
        .collect()
}

fn write_set(dir: &Path, files: &[Fixture]) -> Result<(), String> {
    let user_home = std::env::home_dir().unwrap_or_default();
    let home = user_home.to_string_lossy().into_owned();
    let user = user_home
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let files = cleaned(files, &home, &user)?;
    fs::create_dir_all(dir).map_err(|_| "the set dir could not be made".to_owned())?;
    fs::set_permissions(dir, fs::Permissions::from_mode(0o700))
        .map_err(|_| "the set dir could not be narrowed".to_owned())?;
    for (name, doc) in files {
        let mut body = serde_json::to_string_pretty(&doc).unwrap_or_default();
        body.push('\n');
        let path = dir.join(&name);
        let written = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(&path)
            .and_then(|mut f| {
                f.set_permissions(fs::Permissions::from_mode(0o600))?;
                f.write_all(body.as_bytes())
            });
        if written.is_err() {
            return Err(format!("{name} could not be written"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kept(word: &str, doc: Value) -> Value {
        serde_json::from_str(&kept_stdout(word, doc.to_string().as_bytes())).expect("json")
    }

    #[test]
    fn record_replaces_every_string_off_the_allowlist() {
        let clients = kept(
            "clients",
            json!([{
                "address": "0x55d4c1a2b3c0",
                "title": "a window",
                "initialTitle": "a window",
                "class": "firefox",
                "workspace": {"id": 3, "name": "web"},
                "tags": ["one", "0x1"],
                "grouped": ["0x55d4c1a2b3c0", "not an address"],
                "swallowing": "0x0",
                "later": "unlisted",
                "pid": 7,
                "mapped": true,
            }]),
        );
        assert_eq!(
            clients,
            json!([{
                "address": "0x55d4c1a2b3c0",
                "title": PLACEHOLDER,
                "initialTitle": PLACEHOLDER,
                "class": PLACEHOLDER,
                "workspace": {"id": 3, "name": PLACEHOLDER},
                "tags": [PLACEHOLDER, PLACEHOLDER],
                "grouped": ["0x55d4c1a2b3c0", PLACEHOLDER],
                "swallowing": "0x0",
                "later": PLACEHOLDER,
                "pid": 7,
                "mapped": true,
            }])
        );
        let active = kept(
            "active-window",
            json!({"address": "0xABC", "title": "x", "swallowing": "0x0"}),
        );
        assert_eq!(
            active,
            json!({"address": PLACEHOLDER, "title": PLACEHOLDER, "swallowing": "0x0"})
        );
        let workspaces = kept(
            "workspaces",
            json!([{"id": 1, "name": "1", "monitor": "DP-1", "lastwindow": "0x55d4c1a2b3c0",
                    "lastwindowtitle": "a window", "address": "0x1"}]),
        );
        assert_eq!(
            workspaces,
            json!([{"id": 1, "name": PLACEHOLDER, "monitor": PLACEHOLDER,
                    "lastwindow": "0x55d4c1a2b3c0", "lastwindowtitle": PLACEHOLDER,
                    "address": PLACEHOLDER}])
        );
        let version = kept(
            "version",
            json!({"branch": "main", "commit_message": "x", "tag": "v0.56.2",
                   "flags": ["debug"], "commit_date": "today"}),
        );
        assert_eq!(
            version,
            json!({"branch": PLACEHOLDER, "commit_message": PLACEHOLDER, "tag": "v0.56.2",
                   "flags": [PLACEHOLDER], "commit_date": PLACEHOLDER})
        );
        let off_form = kept("version", json!({"tag": "v0.56.2-dirty"}));
        assert_eq!(off_form, json!({"tag": PLACEHOLDER}));
    }

    #[test]
    fn record_keeps_a_non_json_reply_only_when_it_is_the_success_text() {
        assert_eq!(kept_stdout("focus-window", b"ok\n"), "ok\n");
        assert_eq!(kept_stdout("workspace", b"ok"), "ok");
        assert_eq!(
            kept_stdout("version", b"HYPRLAND_INSTANCE_SIGNATURE not set!\n"),
            PLACEHOLDER
        );
        assert_eq!(kept_stdout("clients", b""), PLACEHOLDER);
        assert_eq!(kept_stdout("clients", b"\xff\xfe"), PLACEHOLDER);
    }

    #[test]
    fn record_keeps_no_monitors_string() {
        let monitors = kept(
            "monitors",
            json!([{
                "id": 0,
                "name": "DP-1",
                "description": "a monitor",
                "make": "a maker",
                "model": "a model",
                "serial": "0x1",
                "width": 2560,
                "focused": true,
                "activeWorkspace": {"id": 4, "name": "4"},
                "availableModes": ["2560x1440@60.00Hz"],
            }]),
        );
        assert_eq!(
            monitors,
            json!([{
                "id": 0,
                "name": PLACEHOLDER,
                "description": PLACEHOLDER,
                "make": PLACEHOLDER,
                "model": PLACEHOLDER,
                "serial": PLACEHOLDER,
                "width": 2560,
                "focused": true,
                "activeWorkspace": {"id": 4, "name": PLACEHOLDER},
                "availableModes": [PLACEHOLDER],
            }])
        );
    }

    #[test]
    fn record_keeps_a_move_reply_only_as_the_success_text() {
        assert_eq!(kept_stdout("move-to-workspace", b"ok\n"), "ok\n");
        assert_eq!(kept_stdout("move-to-workspace", b"ok"), "ok");
        assert_eq!(
            kept_stdout("move-to-workspace", b"Invalid dispatcher\n"),
            PLACEHOLDER
        );
        assert_eq!(
            kept("move-to-workspace", json!({"address": "0x1", "ok": "ok"})),
            json!({"address": PLACEHOLDER, "ok": PLACEHOLDER})
        );
    }

    #[test]
    fn parse_args_takes_each_verb_form_and_refuses_every_other() {
        let args = |line: &str| -> Vec<String> { line.split(' ').map(str::to_owned).collect() };
        let verb = |line: &str| {
            parse_args(&args(line)).map(|(verb, program, out)| {
                let named = match verb {
                    Verb::Record => "record".to_owned(),
                    Verb::RecordDispatch => "record-dispatch".to_owned(),
                    Verb::RecordMonitors => "record-monitors".to_owned(),
                    Verb::RecordMove {
                        terminal,
                        workspace,
                    } => format!(
                        "record-move {} {}",
                        terminal.display(),
                        workspace.into_inner()
                    ),
                };
                (named, program, out)
            })
        };
        let p = || PathBuf::from("/usr/bin/hyprctl");
        let o = || PathBuf::from("out");
        for (label, line, want) in [
            (
                "record",
                "record --program /usr/bin/hyprctl --out out",
                Some(("record".to_owned(), p(), o())),
            ),
            (
                "record-dispatch",
                "record-dispatch --program /usr/bin/hyprctl --out out",
                Some(("record-dispatch".to_owned(), p(), o())),
            ),
            (
                "record-monitors",
                "record-monitors --program /usr/bin/hyprctl --out out",
                Some(("record-monitors".to_owned(), p(), o())),
            ),
            (
                "record-move",
                "record-move --program /usr/bin/hyprctl --terminal /usr/bin/foot --workspace -98 --out out",
                Some(("record-move /usr/bin/foot -98".to_owned(), p(), o())),
            ),
            (
                "record-move in the short form",
                "record-move --program /usr/bin/hyprctl --out out",
                None,
            ),
            (
                "a relative terminal",
                "record-move --program /usr/bin/hyprctl --terminal foot --workspace 2 --out out",
                None,
            ),
            (
                "a relative program",
                "record-move --program hyprctl --terminal /usr/bin/foot --workspace 2 --out out",
                None,
            ),
            (
                "a workspace past i32",
                "record-move --program /usr/bin/hyprctl --terminal /usr/bin/foot --workspace 2147483648 --out out",
                None,
            ),
            (
                "the long form for another verb",
                "record --program /usr/bin/hyprctl --terminal /usr/bin/foot --workspace 2 --out out",
                None,
            ),
            (
                "--term for --terminal",
                "record-move --program /usr/bin/hyprctl --term /usr/bin/foot --workspace 2 --out out",
                None,
            ),
            (
                "--ws for --workspace",
                "record-move --program /usr/bin/hyprctl --terminal /usr/bin/foot --ws 2 --out out",
                None,
            ),
        ] {
            assert_eq!(verb(line), want, "{label}");
        }
    }

    #[test]
    fn stem_is_the_pascal_case_of_each_request_word() {
        let stems: Vec<String> = viola_hyprland::WORDS.iter().map(|w| stem(w)).collect();
        assert_eq!(
            stems,
            [
                "Clients",
                "ActiveWindow",
                "Workspaces",
                "Version",
                "FocusWindow",
                "Workspace",
                "Monitors",
                "MoveToWorkspace"
            ]
        );
    }

    #[test]
    fn fixture_is_the_envelope_with_the_reply_kept() {
        let version = HyprlandVersion::from_tag("v0.56.2").expect("version");
        let got = Read {
            reply: Ok(Reply::Dispatched),
            exit: Some(0),
            stdout: b"ok\n".to_vec(),
        };
        let request = Request::Workspace(WorkspaceId::new(-98));
        let (name, doc) = fixture(version, &request, &got, RECORDED);
        assert_eq!(name, "Workspace.recorded-1.json");
        assert_eq!(
            doc,
            json!({"v": 1, "hyprland": "0.56.2", "request": "workspace",
                   "args": ["dispatch", "hl.dsp.focus({ workspace = -98 })"], "exit": 0, "stdout": "ok\n"})
        );
    }

    #[test]
    fn cleaned_scrubs_a_home_path_and_refuses_an_email_whole() {
        let home = "/srv/h/plantuser";
        let doc = |text: &str| json!({"v": 1, "stdout": text});
        let files = vec![("A.recorded-1.json".to_owned(), doc("/srv/h/plantuser/x"))];
        let got = cleaned(&files, home, "plantuser").expect("clean");
        assert_eq!(got[0].1, doc("~/x"));
        let dirty = vec![
            ("A.recorded-1.json".to_owned(), doc("viola fixture text")),
            ("B.recorded-1.json".to_owned(), doc("a.b@example.com")),
        ];
        assert_eq!(
            cleaned(&dirty, home, "plantuser"),
            Err("B.recorded-1.json email".to_owned())
        );
    }

    #[test]
    fn fault_names_each_closed_code_and_a_clean_document_none() {
        for (label, text, want) in [
            ("clean", "~/work", None),
            ("home", "/srv/h/plantuser/x", Some("home-path")),
            ("absolute", "/home/someone/x", Some("absolute-path")),
            ("user", "made by plantuser", Some("username")),
            ("email", "a.b@example.com", Some("email")),
        ] {
            let doc = json!({"stdout": text});
            assert_eq!(
                fault(&doc, "/srv/h/plantuser", "plantuser"),
                want,
                "{label}"
            );
        }
    }

    #[test]
    fn fault_reads_a_string_inside_an_array() {
        let doc = json!({"stdout": ["clean", "a.b@example.com"]});
        assert_eq!(fault(&doc, "/srv/h/plantuser", "plantuser"), Some("email"));
    }

    #[test]
    fn write_set_writes_each_file_0600_in_a_0700_dir() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("0.56.2");
        let files = vec![("Clients.recorded-1.json".to_owned(), json!({"v": 1}))];
        write_set(&dir, &files).expect("written");
        let mode = |p: &Path| fs::metadata(p).expect("meta").permissions().mode() & 0o777;
        assert_eq!(mode(&dir), 0o700);
        let file = dir.join("Clients.recorded-1.json");
        assert_eq!(mode(&file), 0o600);
        assert_eq!(
            fs::read_to_string(&file).expect("read"),
            "{\n  \"v\": 1\n}\n"
        );
    }
}
