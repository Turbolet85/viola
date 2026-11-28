//! `viola console` as a human's terminal hosts it (test-plan · decision/tui-verdicts-from-records):
//! the built binary under an outer PTY of the test's own, in a home the test owns. Each verdict is
//! read from the console's codes-only role file (`<home>/diagnostics/console.ndjson`), its exit and
//! the terminal's line-mode flag, never from the screen it draws.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Instant;

use rstest::rstest;
use serde_json::Value;
use support::cli::{Running, viola};
use support::events::wait_events;
use support::fake;
use support::frozen::Frozen;
use support::home::{
    StampedHome, TestHome, UNWRAPPED, USER_HOME_VAR, VIOLA, Wrapper, private_dir, process_start,
    snapshot_data, stamped_home, still_widened, unwiden, widen, workspace_path,
};
use support::hygiene::load_schema;
use support::outer_pty::{EXIT_WITHIN, OuterPty};
use support::start::{kill_wrapper, listed};
use support::terminal::end_trees;
use support::verify::{seed_user_home, user_home_of};
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
/// names inherited and `PATH` an empty test-scoped bin dir beside the home, so its agents read
/// finds no `claude`.
fn start(home: &Path, size: Size) -> OuterPty {
    let scratch = home.parent().expect("the home has a parent");
    start_on(home, size, &fake::empty_bin(scratch))
}

/// `start` with `PATH` the bin dir `bin` alone.
fn start_on(home: &Path, size: Size, bin: &Path) -> OuterPty {
    let args: Vec<OsString> = vec!["--home".into(), home.into(), "console".into()];
    let path = bin.to_str().expect("utf-8 bin dir");
    OuterPty::spawn_without(
        Path::new(VIOLA),
        &args,
        &[("PATH", path)],
        &UNWRAPPED,
        size,
        &std::env::current_dir().expect("cwd"),
    )
}

/// The fleet's rows drawn, as the frame lines say, in order.
fn shown(lines: &[Value]) -> Vec<u64> {
    frames(lines)
        .iter()
        .map(|l| l["shown"].as_u64().expect("shown"))
        .collect()
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

/// The empty home's first frame draws no row; a booted wrapper brings a frame drawing one, and its
/// stop one drawing none.
#[rstest]
fn console_draws_a_booted_wrapper_and_its_end(stamped_home: StampedHome) {
    let home = stamped_home.home.path().to_owned();
    let mut pty = start(&home, Size::DEFAULT);
    let seen = wait_lines(&mut pty, &home, "a frame", |l| !frames(l).is_empty());
    assert_eq!(shown(&seen), [0]);
    assert_eq!(frames(&seen)[0]["hidden"], 0);
    let wrapper = support::events::boot(stamped_home, None, &[]);
    wait_lines(&mut pty, &home, "a frame drawing the wrapper", |l| {
        shown(l).last() == Some(&1)
    });
    let (_, kept) = wrapper.stop_keep();
    let seen = wait_lines(&mut pty, &home, "a frame drawing none again", |l| {
        shown(l).ends_with(&[1, 0])
    });
    assert_eq!(shown(&seen), [0, 1, 0]);
    pty.write(b"q");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    let all = lines(&home);
    assert_exited(&all, 0);
    assert_schema(&all);
    drop(kept);
}

/// With the fake agent as `claude` on its `PATH`, the console makes its agents read off its loop,
/// one `agents-probe` pair, and `q` still quits at once.
#[test]
fn console_reads_the_agents_list_off_the_loop() {
    let home = TestHome::new();
    let bin = fake::claude_bin(home.scratch(), "claude-agents", None, None);
    let mut pty = start_on(home.path(), Size::DEFAULT, &bin);
    let probe = |l: &Value, event: &str| l["event"] == event && l["subject"] == "agents-probe";
    wait_lines(&mut pty, home.path(), "the agents-probe exit", |l| {
        l.iter().any(|line| probe(line, "process-exit"))
    });
    pty.write(b"q");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    let all = lines(home.path());
    assert!(all.iter().any(|line| probe(line, "process-start")));
    assert_eq!(keys(&all), ["quit"]);
    assert_exited(&all, 0);
    assert_schema(&all);
}

/// A home widened after the first frame ends the console: exit 1, the home pair on its terminal
/// and one `process-exit` carrying `strict-modes-failed`.
#[test]
fn console_ends_one_on_a_home_widened_mid_run() {
    let home = TestHome::new();
    let mut pty = start(home.path(), Size::DEFAULT);
    wait_lines(&mut pty, home.path(), "a frame", |l| !frames(l).is_empty());
    widen(home.path());
    let code = pty.wait_exit(EXIT_WITHIN);
    let text = String::from_utf8_lossy(&pty.finish()).into_owned();
    unwiden(home.path());
    assert_eq!(code, 1);
    assert!(text.contains(HOME_UNABLE), "{text:?}");
    assert!(text.contains(HOME_HINT), "{text:?}");
    let all = lines(home.path());
    let exits: Vec<&Value> = all
        .iter()
        .filter(|l| l["event"] == "process-exit" && l["subject"] == "self")
        .collect();
    assert_eq!(exits.len(), 1, "{all:?}");
    assert_eq!(exits[0]["exit_code"], 1);
    assert_eq!(exits[0]["detail"], "strict-modes-failed");
    assert_schema(&all);
}

// The actions (Part 1): each case boots fake-agent instances in its own stamped home, starts the
// console over them with `PATH` a test-scoped bin dir and `HOME` the test-owned user home, and
// reads its verdicts from `console.ndjson`, the instances' `events.ndjson` and exit codes.

/// A wrapper named `name` beside `stamped`'s home, its session-start landed.
fn boot_named(stamped: StampedHome, name: &str) -> Wrapper {
    let fixtures = workspace_path("fixtures/claude");
    let args = ["--fixtures", fixtures.to_str().expect("utf-8 path")];
    let wrapper = Wrapper::boot(stamped, name, None, &args);
    wait_events(&wrapper.instance_dir(), "the session-start record", |l| {
        l.iter().any(|e| e["kind"] == "session-start")
    });
    wrapper
}

/// The console on `home` at `size`, `PATH` the bin dir `bin` alone and `HOME` the test-owned user
/// home, seeded empty.
fn start_acting(home: &Path, size: Size, bin: &Path) -> OuterPty {
    seed_user_home(home, Some(&serde_json::json!({})));
    let user = user_home_of(home);
    let args: Vec<OsString> = vec!["--home".into(), home.into(), "console".into()];
    OuterPty::spawn_without(
        Path::new(VIOLA),
        &args,
        &[
            ("PATH", bin.to_str().expect("utf-8 bin dir")),
            (USER_HOME_VAR, user.to_str().expect("utf-8 user home")),
        ],
        &UNWRAPPED,
        size,
        &std::env::current_dir().expect("cwd"),
    )
}

/// The `console-action` lines' `(verb, exit_code)`, in order.
fn actions(lines: &[Value]) -> Vec<(String, i64)> {
    lines
        .iter()
        .filter(|l| l["event"] == "console-action")
        .map(|l| {
            (
                l["verb"].as_str().expect("a verb").to_owned(),
                l["exit_code"].as_i64().expect("an exit code"),
            )
        })
        .collect()
}

/// The lines once the console has written `n` key lines.
fn wait_keys(pty: &mut OuterPty, home: &Path, n: usize) -> Vec<Value> {
    wait_lines(pty, home, &format!("{n} key lines"), |l| keys(l).len() >= n)
}

/// The lines once the console has written `n` action lines.
fn wait_actions(pty: &mut OuterPty, home: &Path, n: usize) -> Vec<Value> {
    wait_lines(pty, home, &format!("{n} action lines"), |l| {
        actions(l).len() >= n
    })
}

/// The first frame drawing `rows` of the fleet's rows.
fn wait_shown(pty: &mut OuterPty, home: &Path, rows: u64) {
    wait_lines(pty, home, &format!("a frame of {rows} rows"), |l| {
        shown(l).last() == Some(&rows)
    });
}

fn wheel_records(instance_dir: &Path) -> Vec<Value> {
    support::events::events(instance_dir)
        .into_iter()
        .filter(|l| l["kind"] == "wheel")
        .map(|l| l["data"].clone())
        .collect()
}

/// `q`, then exit 0 with the self exit last and every line schema-valid; the lines.
fn quit(pty: &mut OuterPty, home: &Path) -> Vec<Value> {
    pty.write(b"q");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    let all = lines(home);
    assert_exited(&all, 0);
    assert_schema(&all);
    all
}

/// `viola <args…>` at a shell on `home`, `PATH` the bin dir `bin`: its exit code.
fn shell_code(bin: &Path, home: &Path, args: &[&str]) -> i64 {
    let ran = support::cli::viola_on(bin, home, args, None, None);
    i64::from(ran.code.expect("an exit code"))
}

#[rstest]
fn console_selection_moves_over_wrapped_rows_and_skips_unwrapped(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let twin = boot_named(builder.beside(), "twin");
    let home = builder.home().to_owned();
    let unwrapped = listed("0d5f1c7e-6a2b-4c3d-8e9f-0123456789ab");
    let bin = fake::claude_bin(
        home.parent().expect("a parent"),
        "agents-bin",
        None,
        Some(unwrapped.as_bytes()),
    );
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 3);
    pty.write(b"\x1b[B");
    wait_keys(&mut pty, &home, 1);
    pty.write(b"j");
    wait_keys(&mut pty, &home, 2);
    pty.write(b"j");
    wait_keys(&mut pty, &home, 3);
    pty.write(b"p");
    wait_actions(&mut pty, &home, 1);
    pty.write(b"k");
    wait_keys(&mut pty, &home, 5);
    pty.write(b"p");
    let seen = wait_actions(&mut pty, &home, 2);
    assert_eq!(
        actions(&seen),
        [("pause".to_owned(), 0), ("pause".to_owned(), 0)]
    );
    let all = quit(&mut pty, &home);
    assert_eq!(
        keys(&all),
        ["down", "down", "down", "action", "up", "action", "quit"]
    );
    assert_eq!(wheel_records(&twin.instance_dir()).len(), 2, "twin paused");
    assert_eq!(
        wheel_records(&builder.instance_dir()).len(),
        2,
        "builder paused once, after the up"
    );
}

#[rstest]
fn console_selection_keeps_its_name_when_the_fleet_changes(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let twin = boot_named(builder.beside(), "twin");
    let home = builder.home().to_owned();
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 2);
    pty.write(b"j");
    wait_keys(&mut pty, &home, 1);
    let abel = boot_named(builder.beside(), "abel");
    wait_shown(&mut pty, &home, 3);
    pty.write(b"p");
    wait_actions(&mut pty, &home, 1);
    quit(&mut pty, &home);
    assert_eq!(wheel_records(&twin.instance_dir()).len(), 2, "twin paused");
    assert_eq!(wheel_records(&builder.instance_dir()).len(), 1);
    assert_eq!(wheel_records(&abel.instance_dir()).len(), 1);
}

/// Below the floor the frame is one line, but each `console-frame` still counts the rows the body
/// would draw: three wrapped rows in a body of three rows scroll with the selection.
#[rstest]
fn console_scroll_follows_the_selection(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let _twin = boot_named(builder.beside(), "twin");
    let _third = boot_named(builder.beside(), "zed");
    let home = builder.home().to_owned();
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size { cols: 80, rows: 5 }, &bin);
    let frame = |l: &Value| (l["shown"].as_u64(), l["hidden"].as_u64());
    wait_lines(&mut pty, &home, "the first frame", |l| {
        frames(l).last().map(|f| frame(f)) == Some((Some(1), Some(2)))
    });
    pty.write(b"j");
    wait_lines(&mut pty, &home, "the window moved down", |l| {
        frames(l).last().map(|f| frame(f)) == Some((Some(2), Some(1)))
    });
    pty.write(b"k");
    let seen = wait_lines(&mut pty, &home, "the window moved back", |l| {
        frames(l).len() >= 3
    });
    let counts: Vec<(Option<u64>, Option<u64>)> = frames(&seen).into_iter().map(frame).collect();
    assert_eq!(
        counts,
        [(Some(1), Some(2)), (Some(2), Some(1)), (Some(1), Some(2))]
    );
    quit(&mut pty, &home);
}

/// The console's pause writes the record and the exit a shell's `viola pause` writes on a twin.
#[rstest]
fn console_pause_runs_the_verb_as_a_child_like_a_shell(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let twin = boot_named(builder.beside(), "twin");
    let home = builder.home().to_owned();
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 2);
    pty.write(b"p");
    let seen = wait_actions(&mut pty, &home, 1);
    let at_shell = shell_code(&bin, &home, &["pause", "twin"]);
    assert_eq!(actions(&seen), [("pause".to_owned(), at_shell)]);
    assert_eq!(at_shell, 0);
    let console = wheel_records(&builder.instance_dir());
    let shell = wheel_records(&twin.instance_dir());
    assert_eq!(console.len(), 2);
    assert_eq!(console, shell, "the same wheel record");
    let all = quit(&mut pty, &home);
    assert_eq!(keys(&all), ["action", "quit"]);
}

#[rstest]
fn console_release_asks_then_runs_on_yes(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let home = builder.home().to_owned();
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    assert_eq!(shell_code(&bin, &home, &["pause", "builder"]), 0);
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"r");
    wait_keys(&mut pty, &home, 1);
    assert_eq!(
        wheel_records(&builder.instance_dir()).len(),
        2,
        "asked only"
    );
    pty.write(b"y");
    let seen = wait_actions(&mut pty, &home, 1);
    assert_eq!(actions(&seen), [("release".to_owned(), 0)]);
    let wheel = wheel_records(&builder.instance_dir());
    assert_eq!(wheel.len(), 3, "the release's record");
    assert_ne!(wheel[2], wheel[1]);
    let all = quit(&mut pty, &home);
    assert_eq!(keys(&all), ["action", "yes", "quit"]);
}

/// Escape leaves the confirm and runs nothing: the next action is `last`, and the first action
/// line the console writes is its.
#[rstest]
fn console_release_confirm_left_by_escape_runs_nothing(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let home = builder.home().to_owned();
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    assert_eq!(shell_code(&bin, &home, &["pause", "builder"]), 0);
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"r");
    wait_keys(&mut pty, &home, 1);
    pty.write(b"\x1b");
    wait_keys(&mut pty, &home, 2);
    pty.write(b"l");
    let seen = wait_actions(&mut pty, &home, 1);
    assert_eq!(actions(&seen)[0].0, "last", "{:?}", actions(&seen));
    assert_eq!(keys(&seen)[..2], ["action", "escape"]);
    assert_eq!(wheel_records(&builder.instance_dir()).len(), 2);
    quit(&mut pty, &home);
}

#[rstest]
fn console_last_draws_the_verbs_reply(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let home = builder.home().to_owned();
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"l");
    let seen = wait_actions(&mut pty, &home, 1);
    let at_shell = shell_code(&bin, &home, &["last", "builder"]);
    assert_eq!(actions(&seen), [("last".to_owned(), at_shell)]);
    let all = quit(&mut pty, &home);
    assert_eq!(keys(&all), ["action", "quit"]);
}

#[rstest]
fn console_focus_runs_the_verb_over_the_fake_compositor(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let home = builder.home().to_owned();
    let scratch = home.parent().expect("a parent").to_owned();
    let set: Vec<PathBuf> = fs::read_dir(workspace_path("fixtures/hyprland/0.56.2"))
        .expect("the recorded set")
        .map(|f| f.expect("a fixture").path())
        .collect();
    let hyprctl = fake::hyprctl_bin(&scratch, Some("terminal"), &set);
    let bin = hyprctl.parent().expect("the bin dir").to_owned();
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"f");
    let seen = wait_actions(&mut pty, &home, 1);
    let at_shell = shell_code(&bin, &home, &["focus", "builder"]);
    assert_eq!(actions(&seen), [("focus".to_owned(), at_shell)]);
    assert_ne!(at_shell, -1, "the verb ran");
    quit(&mut pty, &home);
}

/// A frozen wrapper reads stale, and `viola pause` refuses it `instance-dead`: exit 21, the
/// action line carrying the verb's own code.
#[rstest]
fn console_refused_action_logs_the_verbs_exit_code(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let home = builder.home().to_owned();
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let dir = builder.instance_dir();
    let pid = snapshot_data(&dir).expect("the snapshot")["pid"]
        .as_u64()
        .and_then(|p| u32::try_from(p).ok())
        .expect("a pid");
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    let frozen = Frozen::attach(pid);
    let watch = Watch::start("stale");
    let deadline = Instant::now() + WITHIN;
    let beat = dir.join("heartbeat");
    while fs::metadata(&beat)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|at| at.elapsed().ok())
        .is_none_or(|age| age <= std::time::Duration::from_secs(5))
    {
        watch.note("the heartbeat is fresh");
        watch.deadline_check(deadline, "the frozen wrapper's heartbeat never went stale");
        std::thread::yield_now();
    }
    pty.write(b"p");
    let seen = wait_actions(&mut pty, &home, 1);
    drop(frozen);
    assert_eq!(actions(&seen), [("pause".to_owned(), 21)]);
    quit(&mut pty, &home);
}

/// Ctrl-C ends the console from the list, the confirm, the editing line and the
/// answer prompt: exit 0, the line mode cooked again.
#[rstest]
fn console_ctrl_c_ends_from_every_mode(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let home = builder.home().to_owned();
    let _hook = raised(
        &builder,
        "permission-request",
        "PermissionRequest.permission-1.json",
        "permission",
    );
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    for (before, words) in [
        (&b""[..], &["interrupt"][..]),
        (&b"r"[..], &["action", "interrupt"][..]),
        (&b"s"[..], &["action", "interrupt"][..]),
        (&b"a"[..], &["action", "interrupt"][..]),
    ] {
        let _ = fs::remove_file(role_file(&home));
        let mut pty = start_acting(&home, Size::DEFAULT, &bin);
        let tty = console_tty(&pty);
        wait_shown(&mut pty, &home, 1);
        if !before.is_empty() {
            pty.write(before);
            wait_keys(&mut pty, &home, 1);
        }
        assert!(!line_input_on(&tty), "raw while the frame stands");
        pty.write(b"\x03");
        assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
        assert!(line_input_on(&tty), "cooked again after the interrupt");
        let all = lines(&home);
        assert_eq!(keys(&all), words);
        assert!(actions(&all).is_empty());
        assert_exited(&all, 0);
        assert_schema(&all);
    }
    assert_eq!(wheel_records(&builder.instance_dir()).len(), 1);
}

/// The world `viola up` re-opens an instance in: the fake agent planted as `claude`, as the
/// stand-in `foot` and as `hyprctl` in its `terminal` mode in one bin dir, the console's whole
/// `PATH`; every stand-in terminal and its tree ended at the case's end.
struct Reopen {
    bin: PathBuf,
}

impl Reopen {
    fn new(scratch: &Path) -> Self {
        let bin = scratch.join("reopen-bin");
        let set = bin.join("hyprctl-set");
        fs::create_dir_all(&set).expect("the hyprctl set");
        for fixture in fs::read_dir(workspace_path("fixtures/hyprland/0.56.2")).expect("set") {
            let fixture = fixture.expect("a fixture").path();
            fs::copy(&fixture, set.join(fixture.file_name().expect("a name")))
                .expect("the fixture copy");
        }
        fs::write(bin.join("hyprctl-mode"), "terminal").expect("the hyprctl mode");
        fs::write(bin.join("agents-mode"), "malformed").expect("the agents mode");
        let fixtures = workspace_path("fixtures/claude");
        let cwd = std::env::current_dir().expect("cwd");
        fake::write_fake_args(
            &bin,
            &[
                "--fixtures",
                fixtures.to_str().expect("utf-8 path"),
                "--cli-version",
                fake::RECORDED_CLI_VERSION,
                "--trusted-root",
                cwd.to_str().expect("utf-8 cwd"),
            ],
        );
        for name in ["claude", "foot", "hyprctl"] {
            fs::copy(fake::FAKE, bin.join(name)).expect("the fake copy");
        }
        Self { bin }
    }

    fn spawns(&self) -> Vec<Value> {
        support::ndjson::read_lines(&self.bin.join("foot-spawns.ndjson"))
    }
}

impl Drop for Reopen {
    fn drop(&mut self) {
        let roots: Vec<(u32, u64)> = self
            .spawns()
            .iter()
            .filter_map(|spawn| spawn["pid"].as_u64().and_then(|p| u32::try_from(p).ok()))
            .filter_map(|pid| process_start(pid).map(|started| (pid, started)))
            .collect();
        end_trees(&roots, &[]);
    }
}

/// This host's boot time from `/proc/stat`'s `btime` line.
fn boot_time() -> std::time::SystemTime {
    let stat = fs::read_to_string("/proc/stat").expect("/proc/stat");
    let secs: u64 = stat
        .lines()
        .find_map(|line| line.strip_prefix("btime "))
        .expect("a btime line")
        .trim()
        .parse()
        .expect("whole seconds");
    std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(secs)
}

/// `v` asks, `y` runs `viola up <name>`, whose stand-in terminal is spawned naming
/// `revive <name> --origin`. The instance is a reboot's leaving: its wrapper killed, its
/// heartbeat set before this host's boot.
#[rstest]
fn console_revive_asks_then_runs_up_on_yes(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let home = builder.home().to_owned();
    let world = Reopen::new(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &world.bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"v");
    wait_keys(&mut pty, &home, 1);
    let dir = builder.instance_dir();
    let data = snapshot_data(&dir).expect("the snapshot");
    let pid = u32::try_from(data["pid"].as_u64().expect("pid")).expect("u32");
    let child = data["child_pid"]
        .as_u64()
        .and_then(|p| u32::try_from(p).ok());
    let started = process_start(pid).expect("the wrapper runs");
    kill_wrapper(pid);
    let watch = Watch::start("killed");
    let deadline = Instant::now() + WITHIN;
    while process_start(pid) == Some(started) || child.is_some_and(|c| process_start(c).is_some()) {
        watch.note("the wrapper or its child runs");
        watch.deadline_check(
            deadline,
            "the killed wrapper or its child outlived the kill",
        );
        std::thread::yield_now();
    }
    fs::OpenOptions::new()
        .append(true)
        .open(dir.join("heartbeat"))
        .expect("the heartbeat")
        .set_modified(boot_time() - std::time::Duration::from_secs(60))
        .expect("mtime");
    pty.write(b"y");
    let seen = wait_actions(&mut pty, &home, 1);
    assert_eq!(actions(&seen), [("up".to_owned(), 0)]);
    let spawns = world.spawns();
    assert_eq!(spawns.len(), 1, "{spawns:?}");
    let argv: Vec<&str> = spawns[0]["argv"]
        .as_array()
        .expect("an argv")
        .iter()
        .map(|w| w.as_str().expect("a word"))
        .collect();
    assert!(
        argv.windows(3)
            .any(|w| w == ["revive", "builder", "--origin"]),
        "{argv:?}"
    );
    let all = quit(&mut pty, &home);
    assert_eq!(keys(&all), ["action", "yes", "quit"]);
    // The re-opened tree first, while the home it runs in still stands.
    drop(pty);
    drop(world);
    drop(builder);
}

// The actions (Part 2): the editing line, send, switch, open and answer, under the same
// conditions as Part 1's.

/// A gated `PostToolUse`, then a gated `Stop`: a send's turn, started and held.
const PATH3: &str = "fixtures/fake-scripts/path3.json";

/// The `console-key` lines' `(key, count)`, in order.
fn key_counts(lines: &[Value]) -> Vec<(String, Option<u64>)> {
    lines
        .iter()
        .filter(|l| l["event"] == "console-key")
        .map(|l| {
            (
                l["key"].as_str().expect("a key word").to_owned(),
                l["count"].as_u64(),
            )
        })
        .collect()
}

/// The scalars the `text` lines count, summed.
fn text_typed(lines: &[Value]) -> u64 {
    key_counts(lines)
        .iter()
        .filter(|(key, _)| key == "text")
        .filter_map(|(_, count)| *count)
        .sum()
}

fn of_kind<'a>(lines: &'a [Value], kind: &str) -> Vec<&'a Value> {
    lines.iter().filter(|l| l["kind"] == kind).collect()
}

/// A recorded hook payload of the stamped CLI version.
fn fixture(name: &str) -> Value {
    let path = workspace_path("fixtures/claude")
        .join(fake::RECORDED_CLI_VERSION)
        .join(name);
    serde_json::from_slice(&fs::read(path).expect("fixture")).expect("json")
}

/// `viola hook <event>` with `payload` on stdin as the wrapped child would run it, left running:
/// a dialog hook waits for its answer.
fn spawn_hook(wrapper: &Wrapper, event: &str, payload: &Value) -> Running {
    use std::io::Write as _;
    let mut command = support::tied::viola();
    command
        .args(["hook", event])
        .env("VIOLA_NAME", "builder")
        .env("VIOLA_DIR", wrapper.instance_dir())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().expect("viola hook");
    let mut pipe = child.stdin.take().expect("stdin");
    pipe.write_all(payload.to_string().as_bytes())
        .expect("stdin");
    drop(pipe);
    Running::over(child)
}

/// A dialog of `kind` raised by a hook the test runs with `payload`, its line logged (so the
/// console's first board read holds it): the hook, still waiting.
fn raised(wrapper: &Wrapper, event: &str, payload: &str, kind: &str) -> Running {
    let hook = spawn_hook(wrapper, event, &fixture(payload));
    wait_events(&wrapper.instance_dir(), "the dialog line", |l| {
        !of_kind(l, kind).is_empty()
    });
    hook
}

/// The behaviour of a finished dialog hook's decision body.
fn decided(hook: Running) -> Value {
    let ran = hook.finish();
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    let body: Value = serde_json::from_str(&ran.stdout).expect("one decision body");
    body["hookSpecificOutput"]["decision"]["behavior"].clone()
}

fn dialogs_answered(home: &Path) -> usize {
    support::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"))
        .iter()
        .filter(|l| l["event"] == "dialog-answered")
        .count()
}

#[rstest]
fn console_send_hands_the_line_to_the_verb_and_logs_its_exit(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, Some(PATH3), &[]);
    let home = builder.home().to_owned();
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"s");
    wait_keys(&mut pty, &home, 1);
    pty.write(b"step four");
    wait_lines(&mut pty, &home, "the typed text", |l| text_typed(l) == 9);
    pty.write(b"\r");
    let seen = wait_actions(&mut pty, &home, 1);
    assert_eq!(actions(&seen), [("send".to_owned(), 0)]);
    let events = support::events::events(&builder.instance_dir());
    let kinds: Vec<&Value> = events
        .iter()
        .filter(|e| e["kind"].as_str().is_some_and(|k| k.starts_with("send-")))
        .map(|e| &e["kind"])
        .collect();
    assert_eq!(kinds, ["send-issued", "send-confirmed"]);
    let prompts = of_kind(&events, "prompt-submitted");
    assert_eq!(prompts.len(), 1);
    assert_eq!(prompts[0]["data"]["text"], "step four");
    let all = quit(&mut pty, &home);
    assert!(
        !fs::read_to_string(role_file(&home))
            .expect("the role file")
            .contains("step four"),
        "no typed text in a line"
    );
    assert_eq!(keys(&all).last(), Some(&"quit"));
}

#[rstest]
fn console_send_empty_line_is_the_verbs_refusal(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, Some(PATH3), &[]);
    let home = builder.home().to_owned();
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"s");
    wait_keys(&mut pty, &home, 1);
    pty.write(b"\r");
    let seen = wait_actions(&mut pty, &home, 1);
    assert_eq!(actions(&seen), [("send".to_owned(), 13)]);
    assert!(
        of_kind(
            &support::events::events(&builder.instance_dir()),
            "send-issued"
        )
        .is_empty()
    );
    let all = quit(&mut pty, &home);
    assert_eq!(keys(&all), ["action", "enter", "quit"]);
}

#[rstest]
fn console_send_a_cyrillic_line_as_typed(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, Some(PATH3), &[]);
    let home = builder.home().to_owned();
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"s");
    wait_keys(&mut pty, &home, 1);
    pty.write("привет мир".as_bytes());
    wait_lines(&mut pty, &home, "the typed text", |l| text_typed(l) == 10);
    pty.write(b"\r");
    let seen = wait_actions(&mut pty, &home, 1);
    assert_eq!(actions(&seen), [("send".to_owned(), 0)]);
    let events = support::events::events(&builder.instance_dir());
    let prompts = of_kind(&events, "prompt-submitted");
    assert_eq!(prompts.len(), 1);
    assert_eq!(prompts[0]["data"]["text"], "привет мир");
    assert_eq!(prompts[0]["data"]["origin"], "driver");
    assert_eq!(of_kind(&events, "send-confirmed").len(), 1);
    quit(&mut pty, &home);
}

#[rstest]
fn console_editing_line_left_by_escape_runs_nothing(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, Some(PATH3), &[]);
    let home = builder.home().to_owned();
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"s");
    wait_keys(&mut pty, &home, 1);
    pty.write(b"abc");
    wait_lines(&mut pty, &home, "the typed text", |l| text_typed(l) == 3);
    pty.write(b"\x1b");
    wait_lines(&mut pty, &home, "the escape", |l| {
        keys(l).contains(&"escape")
    });
    pty.write(b"l");
    let seen = wait_actions(&mut pty, &home, 1);
    assert_eq!(actions(&seen)[0].0, "last", "{:?}", actions(&seen));
    assert!(
        of_kind(
            &support::events::events(&builder.instance_dir()),
            "send-issued"
        )
        .is_empty()
    );
    quit(&mut pty, &home);
}

#[rstest]
fn console_editing_line_logs_one_text_line_per_read(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let home = builder.home().to_owned();
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"o");
    wait_keys(&mut pty, &home, 1);
    pty.write(&[b'x'; 200]);
    let seen = wait_lines(&mut pty, &home, "the pasted text", |l| text_typed(l) == 200);
    let texts = key_counts(&seen)
        .into_iter()
        .filter(|(key, _)| key == "text")
        .count();
    assert!((1..=4).contains(&texts), "{texts} text lines for 200 bytes");
    pty.write(b"\x1b");
    wait_lines(&mut pty, &home, "the escape", |l| {
        keys(l).contains(&"escape")
    });
    let all = quit(&mut pty, &home);
    assert!(actions(&all).is_empty());
}

/// The session `SESSION` adopted as `builder` in a folder of the test's own, every submitted turn
/// ending; its session-start logged.
fn adopted(stamped: StampedHome) -> Wrapper {
    const SESSION: &str = "11111111-2222-4333-8444-555555555555";
    let typed_in = support::start::made_dir(&stamped.home, "typed-in");
    let args = [SESSION, "--name", "builder"];
    let wrapper = Wrapper::adopt(stamped, "builder", &typed_in, &args, &["--turn-stop"]);
    wait_events(&wrapper.instance_dir(), "the session-start record", |l| {
        l.iter().any(|e| e["kind"] == "session-start")
    });
    support::start::wait_session_start_hooks(&wrapper.receipt(), 1);
    wrapper
}

#[rstest]
fn console_switch_hands_its_words_to_the_verb(stamped_home: StampedHome) {
    let builder = adopted(stamped_home);
    let home = builder.home().to_owned();
    let dir = builder.instance_dir();
    let before = of_kind(&support::events::events(&dir), "launch").len();
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"m");
    wait_keys(&mut pty, &home, 1);
    pty.write(b"sonnet high");
    wait_lines(&mut pty, &home, "the typed words", |l| text_typed(l) == 11);
    pty.write(b"\r");
    let seen = wait_actions(&mut pty, &home, 1);
    assert_eq!(actions(&seen), [("switch".to_owned(), 0)]);
    let events = wait_events(&dir, "the switch's launch line", |l| {
        of_kind(l, "launch").len() > before
    });
    let launch = of_kind(&events, "launch")[before];
    assert_eq!(launch["data"]["model"], "sonnet");
    assert_eq!(launch["data"]["effort"], "high");
    quit(&mut pty, &home);
}

#[rstest]
fn console_switch_bad_word_is_the_verbs_refusal(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let home = builder.home().to_owned();
    let dir = builder.instance_dir();
    let before = support::events::events(&dir).len();
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"m");
    wait_keys(&mut pty, &home, 1);
    pty.write(b"Bad!");
    wait_lines(&mut pty, &home, "the typed word", |l| text_typed(l) == 4);
    pty.write(b"\r");
    let seen = wait_actions(&mut pty, &home, 1);
    assert_eq!(actions(&seen), [("switch".to_owned(), 2)]);
    assert_eq!(
        support::events::events(&dir).len(),
        before,
        "nothing logged"
    );
    quit(&mut pty, &home);
}

#[rstest]
fn console_open_hands_the_name_and_parent_to_the_verb(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let home = builder.home().to_owned();
    let world = Reopen::new(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &world.bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"o");
    wait_keys(&mut pty, &home, 1);
    pty.write(b"child");
    wait_lines(&mut pty, &home, "the typed name", |l| text_typed(l) == 5);
    pty.write(b"\r");
    let seen = wait_actions(&mut pty, &home, 1);
    assert_eq!(actions(&seen), [("open".to_owned(), 0)]);
    let spawns = world.spawns();
    assert_eq!(spawns.len(), 1, "{spawns:?}");
    let argv: Vec<&str> = spawns[0]["argv"]
        .as_array()
        .expect("an argv")
        .iter()
        .map(|w| w.as_str().expect("a word"))
        .collect();
    assert!(argv.windows(2).any(|w| w == ["run", "child"]), "{argv:?}");
    assert!(
        argv.windows(2).any(|w| w == ["--parent", "builder"]),
        "{argv:?}"
    );
    quit(&mut pty, &home);
    drop(pty);
    drop(world);
    drop(builder);
}

#[rstest]
fn console_answer_permission_by_y(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let home = builder.home().to_owned();
    let hook = raised(
        &builder,
        "permission-request",
        "PermissionRequest.permission-1.json",
        "permission",
    );
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"a");
    wait_keys(&mut pty, &home, 1);
    pty.write(b"y");
    let seen = wait_actions(&mut pty, &home, 1);
    assert_eq!(actions(&seen), [("answer".to_owned(), 0)]);
    assert_eq!(decided(hook), "allow");
    assert_eq!(dialogs_answered(&home), 1);
    let all = quit(&mut pty, &home);
    assert_eq!(keys(&all), ["action", "yes", "quit"]);
}

#[rstest]
fn console_answer_plan_by_n(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let home = builder.home().to_owned();
    let hook = raised(
        &builder,
        "permission-request",
        "PermissionRequest.plan-1.json",
        "plan",
    );
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"a");
    wait_keys(&mut pty, &home, 1);
    pty.write(b"n");
    let seen = wait_actions(&mut pty, &home, 1);
    assert_eq!(actions(&seen), [("answer".to_owned(), 0)]);
    assert_eq!(decided(hook), "deny", "a revise denies the plan");
    assert_eq!(dialogs_answered(&home), 1);
    let all = quit(&mut pty, &home);
    assert_eq!(keys(&all), ["action", "no", "quit"]);
}

#[rstest]
fn console_answer_question_draws_one_line_and_runs_nothing(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let home = builder.home().to_owned();
    let payload = "PreToolUse.question-permission-1.json";
    let mut hook = raised(&builder, "pre-tool-use", payload, "question");
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"a");
    wait_keys(&mut pty, &home, 1);
    let all = quit(&mut pty, &home);
    assert_eq!(keys(&all), ["question", "quit"]);
    assert!(actions(&all).is_empty());
    assert_eq!(dialogs_answered(&home), 0);
    assert!(hook.still_running(), "the question stands");
    let text = fs::read_to_string(role_file(&home)).expect("the role file");
    let input = &fixture(payload)["tool_input"]["questions"][0];
    let mut words = vec![input["question"].as_str().expect("a question").to_owned()];
    for option in input["options"].as_array().expect("options") {
        words.push(option["label"].as_str().expect("a label").to_owned());
        words.push(
            option["description"]
                .as_str()
                .expect("a description")
                .to_owned(),
        );
    }
    for word in &words {
        assert!(!text.contains(word.as_str()), "{word:?} reached a line");
    }
}

#[rstest]
fn console_answer_left_by_escape_answers_nothing(stamped_home: StampedHome) {
    let builder = support::events::boot(stamped_home, None, &[]);
    let home = builder.home().to_owned();
    let mut hook = raised(
        &builder,
        "permission-request",
        "PermissionRequest.permission-1.json",
        "permission",
    );
    let bin = fake::empty_bin(home.parent().expect("a parent"));
    let mut pty = start_acting(&home, Size::DEFAULT, &bin);
    wait_shown(&mut pty, &home, 1);
    pty.write(b"a");
    wait_keys(&mut pty, &home, 1);
    pty.write(b"\x1b");
    wait_keys(&mut pty, &home, 2);
    let all = quit(&mut pty, &home);
    assert_eq!(keys(&all), ["action", "escape", "quit"]);
    assert!(actions(&all).is_empty());
    assert_eq!(dialogs_answered(&home), 0);
    assert!(hook.still_running(), "the permission stands");
}
