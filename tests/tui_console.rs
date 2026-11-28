//! `viola console` as a human's terminal hosts it (test-plan · decision/tui-verdicts-from-records):
//! the built binary under an outer PTY of the test's own, in a home the test owns. Each verdict is
//! read from the console's codes-only role file (`<home>/diagnostics/console.ndjson`), its exit and
//! the terminal's line-mode flag, never from the screen it draws.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use serde_json::Value;
use support::cli::viola;
use support::home::{
    TestHome, UNWRAPPED, VIOLA, private_dir, still_widened, unwiden, widen, workspace_path,
};
use support::hygiene::load_schema;
use support::outer_pty::{EXIT_WITHIN, OuterPty};
use support::watch::{WITHIN, Watch};
use viola_pty::Size;

const NO_TERMINAL: &str = concat!(
    "unable: viola console needs a terminal\n",
    "hint: run viola console in a terminal window\n",
);
const HOME_UNABLE: &str = "unable: viola console could not use its home";
const HOME_HINT: &str =
    "hint: make the viola home readable by you alone, then run viola console again";

fn role_file(home: &Path) -> PathBuf {
    home.join("diagnostics").join("console.ndjson")
}

fn lines(home: &Path) -> Vec<Value> {
    support::ndjson::read_lines(&role_file(home))
}

/// `viola --home <home> console` under an outer PTY of `size`, with none of a wrapped session's
/// names inherited.
fn start(home: &Path, size: Size) -> OuterPty {
    let args: Vec<OsString> = vec!["--home".into(), home.into(), "console".into()];
    OuterPty::spawn_without(
        Path::new(VIOLA),
        &args,
        &[],
        &UNWRAPPED,
        size,
        &std::env::current_dir().expect("cwd"),
    )
}

fn frames(lines: &[Value]) -> Vec<&Value> {
    lines
        .iter()
        .filter(|l| l["event"] == "console-frame")
        .collect()
}

fn keys(lines: &[Value]) -> Vec<&str> {
    lines
        .iter()
        .filter(|l| l["event"] == "console-key")
        .filter_map(|l| l["key"].as_str())
        .collect()
}

/// The role file's lines once `pred` holds of them, within `WITHIN`; a console that exits first
/// fails the case at once.
fn wait_lines(
    pty: &mut OuterPty,
    home: &Path,
    what: &str,
    pred: impl Fn(&[Value]) -> bool,
) -> Vec<Value> {
    let watch = Watch::start("console");
    let deadline = Instant::now() + WITHIN;
    loop {
        let now = lines(home);
        if pred(&now) {
            return now;
        }
        let exit = pty.try_wait();
        watch.note(&format!("lines {} exit {exit:?}", now.len()));
        assert!(exit.is_none(), "the console exited {exit:?} before {what}");
        watch.deadline_check(deadline, &format!("timed out waiting for {what}"));
        std::thread::yield_now();
    }
}

/// Every line the console wrote, admitted by the diag-line schema and of its own role.
fn assert_schema(lines: &[Value]) {
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for line in lines {
        assert!(validator.is_valid(line), "{line}");
        assert_eq!(line["process"], "console");
        assert!(line.get("instance").is_none(), "{line}");
    }
}

/// The self exit's code, and that it is the last line.
fn assert_exited(lines: &[Value], code: u64) {
    let last = lines.last().expect("a line");
    assert_eq!(last["event"], "process-exit");
    assert_eq!(last["subject"], "self");
    assert_eq!(last["exit_code"], code);
}

/// Whether the terminal behind `fd` edits lines (cooked); read on the console's own terminal.
fn line_input_on(tty: &fs::File) -> bool {
    use std::os::fd::AsRawFd as _;
    // SAFETY: tcgetattr fills a struct owned here, on an fd this test holds open.
    unsafe {
        let mut t: libc::termios = std::mem::zeroed();
        assert_eq!(libc::tcgetattr(tty.as_raw_fd(), &mut t), 0, "tcgetattr");
        t.c_lflag & libc::ICANON != 0
    }
}

/// The console's terminal, opened through its stdin as the kernel names it, never made the test's
/// controlling terminal.
fn console_tty(pty: &OuterPty) -> fs::File {
    use std::os::unix::fs::OpenOptionsExt as _;
    let pid = pty.child_pid().expect("pid");
    fs::OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_NOCTTY)
        .open(format!("/proc/{pid}/fd/0"))
        .expect("the console's terminal")
}

#[test]
fn console_at_the_floor_logs_a_whole_frame_and_quits_on_q() {
    let home = TestHome::new();
    let mut pty = start(home.path(), Size::DEFAULT);
    let seen = wait_lines(&mut pty, home.path(), "a frame", |l| !frames(l).is_empty());
    let frame = frames(&seen)[0];
    assert_eq!(frame["cols"], 80);
    assert_eq!(frame["rows"], 24);
    assert_eq!(frame["fits"], true);
    pty.write(b"q");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    let all = lines(home.path());
    assert_eq!(keys(&all), ["quit"]);
    assert_exited(&all, 0);
    assert_eq!(all[0]["event"], "process-start");
    assert_schema(&all);
}

#[test]
fn console_below_the_floor_logs_a_frame_that_does_not_fit() {
    let home = TestHome::new();
    let mut pty = start(home.path(), Size { cols: 60, rows: 20 });
    let seen = wait_lines(&mut pty, home.path(), "a frame", |l| !frames(l).is_empty());
    let frame = frames(&seen)[0];
    assert_eq!(frame["cols"], 60);
    assert_eq!(frame["rows"], 20);
    assert_eq!(frame["fits"], false);
    pty.write(b"q");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    assert_schema(&lines(home.path()));
}

#[test]
fn console_resized_logs_a_new_frame_at_the_new_size() {
    let home = TestHome::new();
    let mut pty = start(
        home.path(),
        Size {
            cols: 100,
            rows: 30,
        },
    );
    let seen = wait_lines(&mut pty, home.path(), "a frame", |l| !frames(l).is_empty());
    assert_eq!(frames(&seen)[0]["fits"], true);
    pty.resize(Size { cols: 70, rows: 20 });
    let seen = wait_lines(&mut pty, home.path(), "a second frame", |l| {
        frames(l).len() >= 2
    });
    let second = frames(&seen)[1];
    assert_eq!(second["cols"], 70);
    assert_eq!(second["rows"], 20);
    assert_eq!(second["fits"], false);
    pty.write(b"q");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    assert_schema(&lines(home.path()));
}

#[test]
fn console_logs_an_unbound_key_by_class_alone() {
    let home = TestHome::new();
    let mut pty = start(home.path(), Size::DEFAULT);
    wait_lines(&mut pty, home.path(), "a frame", |l| !frames(l).is_empty());
    pty.write(b"x");
    let seen = wait_lines(&mut pty, home.path(), "an unbound key", |l| {
        !keys(l).is_empty()
    });
    let line = seen
        .iter()
        .find(|l| l["event"] == "console-key")
        .expect("the key line");
    let mut fields: Vec<&str> = line
        .as_object()
        .expect("an object")
        .keys()
        .map(String::as_str)
        .collect();
    fields.sort_unstable();
    assert_eq!(
        fields,
        [
            "event",
            "key",
            "level",
            "message",
            "process",
            "target",
            "timestamp"
        ]
    );
    assert_eq!(line["key"], "unbound");
    pty.write(b"q");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    let all = lines(home.path());
    for l in &all {
        let values = l.as_object().expect("an object").values();
        assert!(!values.into_iter().any(|v| v == "x"), "the key's byte: {l}");
    }
    assert_eq!(keys(&all), ["unbound", "quit"]);
    assert_schema(&all);
}

#[test]
fn console_on_interrupt_exits_zero() {
    let home = TestHome::new();
    let mut pty = start(home.path(), Size::DEFAULT);
    wait_lines(&mut pty, home.path(), "a frame", |l| !frames(l).is_empty());
    pty.write(b"\x03");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    let all = lines(home.path());
    assert_eq!(keys(&all), ["interrupt"]);
    assert_exited(&all, 0);
    assert_schema(&all);
}

#[test]
fn console_restores_the_terminal_on_quit() {
    let home = TestHome::new();
    let mut pty = start(home.path(), Size::DEFAULT);
    let tty = console_tty(&pty);
    wait_lines(&mut pty, home.path(), "a frame", |l| !frames(l).is_empty());
    assert!(!line_input_on(&tty), "raw while the frame stands");
    pty.write(b"q");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    assert!(line_input_on(&tty), "cooked again after the quit");
}

#[test]
fn console_without_a_terminal_refuses_before_any_line() {
    let home = TestHome::new();
    let ran = viola(home.path(), &["console"], None, None);
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, NO_TERMINAL);
    assert!(!home.path().join("diagnostics").exists());
}

#[test]
fn console_with_a_widened_home_refuses_and_writes_no_line() {
    let home = TestHome::new();
    private_dir(home.path(), "the home");
    widen(home.path());
    let mut pty = start(home.path(), Size::DEFAULT);
    let code = pty.wait_exit(EXIT_WITHIN);
    let shown = pty.finish();
    let widened = still_widened(home.path());
    let made = role_file(home.path()).exists();
    unwiden(home.path());
    assert_eq!(code, 1);
    assert!(widened, "the home's mode as found");
    assert!(!made, "no role file");
    let text = String::from_utf8_lossy(&shown);
    assert!(text.contains(HOME_UNABLE), "{text:?}");
    assert!(text.contains(HOME_HINT), "{text:?}");
}
