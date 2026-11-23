//! The test-only `viola-hypr-tap` spawned as it runs, against a planted `#!/bin/sh` stub `hyprctl`
//! that replays the committed `fixtures/hyprland/0.56.2/` replies (test-plan §7 Fixture hygiene;
//! security-plan · the Hyprland recorder's string allowlist): `record`, `record-dispatch` and
//! `record-monitors` each write their set, `record-move` moves the window of a planted stub terminal
//! it started and ends it, and every malformed argument form is refused exit 2 with nothing written. Every
//! child is owned and killed at a stated deadline; `HOME` is the case's own temp dir, so the scrub
//! has a home and a user word to strip.

use std::fs;
use std::io::Write as _;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

use serde_json::Value;

const TAP: &str = env!("CARGO_BIN_EXE_viola-hypr-tap");
const VERSION: &str = "0.56.2";
const WITHIN: Duration = Duration::from_secs(10);

/// The committed reply set the stub replays.
fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/hyprland")
        .join(VERSION)
}

/// The child's output once it exits, killed and failing the case past `WITHIN`.
fn finished(mut child: Child) -> Output {
    let until = Instant::now() + WITHIN;
    loop {
        if child.try_wait().expect("try_wait").is_some() {
            return child.wait_with_output().expect("output");
        }
        if Instant::now() >= until {
            let _ = child.kill();
            let _ = child.wait();
            panic!("the tap did not exit within {WITHIN:?}");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// A stub `hyprctl` planted 0700 in `dir`: `-j <word>` prints the committed reply's `stdout` for
/// that word, a `dispatch` prints `ok`; every handle is closed before any spawn.
fn stub(dir: &Path) -> PathBuf {
    for (word, file) in [
        ("version", "Version.recorded-1.json"),
        ("clients", "Clients.recorded-1.json"),
        ("activewindow", "ActiveWindow.recorded-1.json"),
        ("workspaces", "Workspaces.recorded-1.json"),
    ] {
        let doc: Value =
            serde_json::from_slice(&fs::read(fixtures().join(file)).expect("a committed reply"))
                .expect("a reply document");
        let stdout = doc["stdout"].as_str().expect("a recorded stdout");
        fs::write(dir.join(format!("reply-{word}")), stdout).expect("the reply file");
    }
    fs::write(dir.join("reply-monitors"), MONITORS).expect("the monitors reply");
    let body = format!(
        "case \"$1\" in\n  -j) cat '{}/reply-'\"$2\" ;;\n  dispatch) echo ok ;;\n  *) exit 1 ;;\nesac",
        dir.display()
    );
    planted(dir, "hyprctl", &body)
}

/// A synthetic `monitors` reply, every string a host would print standing in it.
const MONITORS: &str = r#"[{"id":0,"name":"DP-1","description":"a monitor","make":"a maker","model":"a model","serial":"0x1","width":2560,"height":1440,"x":0,"y":0,"focused":true,"activeWorkspace":{"id":1,"name":"1"}}]"#;

/// A `#!/bin/sh` script planted 0700 in `dir`, every handle closed before any spawn.
fn planted(dir: &Path, name: &str, body: &str) -> PathBuf {
    let path = dir.join(name);
    let mut file = fs::File::create(&path).expect("create");
    file.write_all(format!("#!/bin/sh\n{body}\n").as_bytes())
        .expect("write");
    file.sync_all().expect("sync");
    drop(file);
    fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).expect("chmod");
    path
}

/// A stub terminal that writes its pid to `<dir>/terminal-pid` and sleeps as that same process.
fn stub_terminal(dir: &Path) -> PathBuf {
    let body = format!(
        "echo $$ > '{}/terminal-pid'\nexec /bin/sleep 30",
        dir.display()
    );
    planted(dir, "terminal", &body)
}

/// How the stub `hyprctl` of `record-move` answers its dispatch and its `clients`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Move {
    /// The dispatch prints `ok` and moves the window; `clients` follows `/proc`.
    Taken,
    /// The dispatch prints a text other than `ok`.
    NotOk,
    /// The dispatch prints `ok` and moves nothing.
    Unseen,
    /// `clients` lists the terminal's pid whatever `/proc` says.
    Stays,
}

/// A stub `hyprctl` for `record-move`: `clients` names the stub terminal's pid on workspace 1 until a
/// dispatch, on `moved_to` after it, and no window once the pid is gone; `with_window` false names
/// no window of that pid at all; `answer` bends the dispatch or `clients` as its variant says.
fn stub_move(dir: &Path, moved_to: i32, with_window: bool, answer: Move) -> PathBuf {
    let version: Value = serde_json::from_slice(
        &fs::read(fixtures().join("Version.recorded-1.json")).expect("a committed reply"),
    )
    .expect("a reply document");
    fs::write(
        dir.join("reply-version"),
        version["stdout"].as_str().expect("a recorded stdout"),
    )
    .expect("the reply file");
    let d = dir.display();
    let alive = if answer == Move::Stays {
        String::new()
    } else {
        " && [ -d \"/proc/$pid\" ]".to_owned()
    };
    let clients = if with_window {
        format!(
            "pid=$(cat '{d}/terminal-pid' 2>/dev/null)\n      if [ -n \"$pid\" ]{alive}; then\n        ws=1; [ -f '{d}/moved' ] && ws=$(cat '{d}/moved')\n        printf '[{{\"address\":\"0x55d4c1a2b3c0\",\"pid\":%s,\"workspace\":{{\"id\":%s}}}}]' \"$pid\" \"$ws\"\n      else printf '[]'; fi"
        )
    } else {
        "printf '[{\"address\":\"0x55d4c1a2b3c0\",\"pid\":1,\"workspace\":{\"id\":1}}]'".to_owned()
    };
    let dispatch = match answer {
        Move::Taken | Move::Stays => format!("echo {moved_to} > '{d}/moved'; echo ok"),
        Move::NotOk => format!("echo {moved_to} > '{d}/moved'; echo not-taken"),
        Move::Unseen => "echo ok".to_owned(),
    };
    let body = format!(
        "case \"$1 $2\" in\n  '-j version') cat '{d}/reply-version' ;;\n  '-j clients')\n      {clients} ;;\n  dispatch*) printf '%s' \"$2\" > '{d}/dispatched'; {dispatch} ;;\n  *) exit 1 ;;\nesac"
    );
    planted(dir, "hyprctl", &body)
}

/// Whether the stub terminal's recorded pid is gone from `/proc`.
fn terminal_gone(dir: &Path) -> bool {
    let pid = fs::read_to_string(dir.join("terminal-pid")).expect("the terminal wrote its pid");
    !Path::new(&format!("/proc/{}", pid.trim())).exists()
}

/// `viola-hypr-tap <args>` with `HOME` set to `home`.
fn tap(args: &[&str], home: &Path) -> Output {
    let child = Command::new(TAP)
        .args(args)
        .env("HOME", home)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the tap");
    finished(child)
}

/// The case's own dir, its stub, and an empty out dir.
struct Rig {
    dir: tempfile::TempDir,
    program: String,
    out: PathBuf,
}

impl Rig {
    fn new() -> Self {
        let dir = tempfile::tempdir().expect("tempdir");
        let program = stub(dir.path()).to_str().expect("utf-8").to_owned();
        let out = dir.path().join("out");
        fs::create_dir(&out).expect("the out dir");
        Self { dir, program, out }
    }

    fn out_arg(&self) -> &str {
        self.out.to_str().expect("utf-8")
    }

    fn run(&self, args: &[&str]) -> Output {
        tap(args, self.dir.path())
    }

    /// Each of `names` written under `<out>/0.56.2/` and byte-identical to the committed reply it
    /// replays.
    fn assert_written(&self, names: &[&str], same_as_committed: bool) {
        let set = self.out.join(VERSION);
        for name in names {
            let written = fs::read(set.join(name)).unwrap_or_else(|_| panic!("{name} written"));
            if same_as_committed {
                let committed = fs::read(fixtures().join(name)).expect("committed");
                assert_eq!(written, committed, "{name}");
            }
        }
    }
}

#[test]
fn hypr_tap_record_writes_the_five_replies_from_a_stub_hyprctl() {
    let rig = Rig::new();
    let out = rig.run(&["record", "--program", &rig.program, "--out", rig.out_arg()]);
    assert_eq!(out.status.code(), Some(0), "{:?}", out.stderr);
    rig.assert_written(
        &[
            "Version.recorded-1.json",
            "Clients.recorded-1.json",
            "ActiveWindow.recorded-1.json",
            "Workspaces.recorded-1.json",
        ],
        true,
    );
    rig.assert_written(&["Version.absent-1.json"], false);
}

#[test]
fn hypr_tap_record_dispatch_writes_the_focus_and_workspace_replies() {
    let rig = Rig::new();
    let out = rig.run(&[
        "record-dispatch",
        "--program",
        &rig.program,
        "--out",
        rig.out_arg(),
    ]);
    assert_eq!(out.status.code(), Some(0), "{:?}", out.stderr);
    rig.assert_written(
        &["FocusWindow.recorded-1.json", "Workspace.recorded-1.json"],
        true,
    );
}

/// The names of the files written under `<out>/0.56.2/`, sorted.
fn written(out: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(out.join(VERSION))
        .expect("the set dir")
        .map(|e| {
            e.expect("an entry")
                .file_name()
                .into_string()
                .expect("utf-8")
        })
        .collect();
    names.sort();
    names
}

fn strings<'a>(value: &'a Value, out: &mut Vec<&'a str>) {
    match value {
        Value::String(s) => out.push(s),
        Value::Array(items) => items.iter().for_each(|v| strings(v, out)),
        Value::Object(map) => map.values().for_each(|v| strings(v, out)),
        _ => {}
    }
}

#[test]
fn hypr_tap_record_monitors_writes_the_monitors_reply_alone() {
    let rig = Rig::new();
    let out = rig.run(&[
        "record-monitors",
        "--program",
        &rig.program,
        "--out",
        rig.out_arg(),
    ]);
    assert_eq!(out.status.code(), Some(0), "{:?}", out.stderr);
    assert_eq!(written(&rig.out), ["Monitors.recorded-1.json"]);
    let doc: Value = serde_json::from_slice(
        &fs::read(rig.out.join(VERSION).join("Monitors.recorded-1.json")).expect("read"),
    )
    .expect("a fixture");
    assert_eq!(doc["request"], "monitors");
    assert_eq!(doc["args"], serde_json::json!(["-j", "monitors"]));
    let reply: Value =
        serde_json::from_str(doc["stdout"].as_str().expect("stdout text")).expect("a reply");
    assert_eq!(reply[0]["id"], 0);
    assert_eq!(reply[0]["width"], 2560);
    let mut texts = Vec::new();
    strings(&reply, &mut texts);
    assert_eq!(texts.len(), 6, "{texts:?}");
    assert!(
        texts.iter().all(|t| *t == "viola fixture text"),
        "{texts:?}"
    );
}

#[test]
fn hypr_tap_record_move_moves_the_window_of_its_own_terminal_and_ends_it() {
    let rig = Rig::new();
    let terminal = stub_terminal(rig.dir.path());
    let program = stub_move(rig.dir.path(), 7, true, Move::Taken);
    let out = rig.run(&[
        "record-move",
        "--program",
        program.to_str().expect("utf-8"),
        "--terminal",
        terminal.to_str().expect("utf-8"),
        "--workspace",
        "7",
        "--out",
        rig.out_arg(),
    ]);
    assert_eq!(out.status.code(), Some(0), "{:?}", out.stderr);
    assert_eq!(written(&rig.out), ["MoveToWorkspace.recorded-1.json"]);
    let doc: Value = serde_json::from_slice(
        &fs::read(
            rig.out
                .join(VERSION)
                .join("MoveToWorkspace.recorded-1.json"),
        )
        .expect("read"),
    )
    .expect("a fixture");
    let move_arg = r#"hl.dsp.window.move({ workspace = 7, window = "address:0x55d4c1a2b3c0" })"#;
    assert_eq!(
        doc,
        serde_json::json!({"v": 1, "hyprland": VERSION, "request": "move-to-workspace",
                           "args": ["dispatch", move_arg], "exit": 0, "stdout": "ok\n"})
    );
    assert_eq!(
        fs::read_to_string(rig.dir.path().join("dispatched")).expect("dispatched"),
        move_arg
    );
    assert!(terminal_gone(rig.dir.path()));
}

#[test]
fn hypr_tap_record_move_refuses_with_no_window_of_its_terminal_and_writes_nothing() {
    let rig = Rig::new();
    let terminal = stub_terminal(rig.dir.path());
    let program = stub_move(rig.dir.path(), 7, false, Move::Taken);
    let out = rig.run(&[
        "record-move",
        "--program",
        program.to_str().expect("utf-8"),
        "--terminal",
        terminal.to_str().expect("utf-8"),
        "--workspace",
        "7",
        "--out",
        rig.out_arg(),
    ]);
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&out.stderr),
        "viola-hypr-tap: no-terminal-window\n"
    );
    assert_eq!(fs::read_dir(&rig.out).expect("the out dir").count(), 0);
    assert!(!rig.dir.path().join("dispatched").exists());
    assert!(terminal_gone(rig.dir.path()));
}

/// `record-move` over a stub bent as `answer`: exit 1 with `code` alone on stderr, nothing written
/// under the out dir, and the stub terminal it started gone.
fn assert_move_refused(answer: Move, code: &str) {
    let rig = Rig::new();
    let terminal = stub_terminal(rig.dir.path());
    let program = stub_move(rig.dir.path(), 7, true, answer);
    let out = rig.run(&[
        "record-move",
        "--program",
        program.to_str().expect("utf-8"),
        "--terminal",
        terminal.to_str().expect("utf-8"),
        "--workspace",
        "7",
        "--out",
        rig.out_arg(),
    ]);
    assert_eq!(out.status.code(), Some(1), "{code}");
    assert_eq!(
        String::from_utf8_lossy(&out.stderr),
        format!("viola-hypr-tap: {code}\n")
    );
    assert_eq!(
        fs::read_dir(&rig.out).expect("the out dir").count(),
        0,
        "{code}"
    );
    assert!(
        rig.dir.path().join("dispatched").exists(),
        "{code}: no dispatch"
    );
    assert!(terminal_gone(rig.dir.path()), "{code}");
}

#[test]
fn hypr_tap_record_move_refuses_a_move_not_answered_ok() {
    assert_move_refused(Move::NotOk, "move-refused");
}

#[test]
fn hypr_tap_record_move_refuses_a_move_not_seen() {
    assert_move_refused(Move::Unseen, "move-unseen");
}

#[test]
fn hypr_tap_record_move_refuses_a_window_that_never_leaves() {
    assert_move_refused(Move::Stays, "window-left");
}

#[test]
fn hypr_tap_refuses_each_malformed_argument_form_with_exit_2_and_writes_nothing() {
    let rig = Rig::new();
    let program = rig.program.as_str();
    let out = rig.out_arg();
    let terminal = "/usr/bin/foot";
    let cases: [(&str, Vec<&str>); 9] = [
        (
            "--prog for --program",
            vec!["record", "--prog", program, "--out", out],
        ),
        (
            "--o for --out",
            vec!["record", "--program", program, "--o", out],
        ),
        ("both wrong", vec!["record", "--prog", program, "--o", out]),
        (
            "an unknown verb",
            vec!["recorded", "--program", program, "--out", out],
        ),
        (
            "a relative program",
            vec!["record", "--program", "hyprctl", "--out", out],
        ),
        (
            "four arguments",
            vec!["record", "--program", program, "--out"],
        ),
        (
            "a relative terminal",
            vec![
                "record-move",
                "--program",
                program,
                "--terminal",
                "foot",
                "--workspace",
                "2",
                "--out",
                out,
            ],
        ),
        (
            "a workspace that is no i32",
            vec![
                "record-move",
                "--program",
                program,
                "--terminal",
                terminal,
                "--workspace",
                "two",
                "--out",
                out,
            ],
        ),
        (
            "a missing --workspace",
            vec![
                "record-move",
                "--program",
                program,
                "--terminal",
                terminal,
                "--out",
                out,
            ],
        ),
    ];
    for (label, args) in cases {
        let ran = rig.run(&args);
        assert_eq!(ran.status.code(), Some(2), "{label}");
        let written = fs::read_dir(&rig.out).expect("the out dir").count();
        assert_eq!(written, 0, "{label}");
    }
}
