//! `viola list` row integrity (test-plan §6 Non-path suites; design-system §Surface: cli, pattern 1;
//! layout-templates §Output structure — `viola list`): `claude agents --json` rows whose session
//! ids hold LF, TAB and a C1 character, listed beside a booted wrapper, print as one escaped line
//! each under the unwrapped separator, in the caption's columns, piped, piped under `NO_COLOR` and
//! `TERM=dumb`, and inside the outer PTY; `--json` keeps each id as listed. The fake agent planted as
//! `claude` in a test-scoped bin dir answers the read, the only directory on the run's `PATH`.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

use rstest::rstest;
use serde_json::{Value, json};
use support::cli::viola_on;
use support::fake;
use support::home::{StampedHome, VIOLA, stamped_home};
use support::outer_pty::{EXIT_WITHIN, OuterPty};
use viola_pty::Size;

/// Three rows no wrapped instance joins, each session id holding one control character.
const PLANTED: &str = r#"[{"cwd":"~/a","kind":"interactive","name":"t","pid":4294967295,"sessionId":"e\u0085f","startedAt":1,"status":"busy"},{"cwd":"~/b","kind":"interactive","name":"t","sessionId":"a\nb","startedAt":2,"status":"idle"},{"cwd":"~/c","kind":"interactive","name":"t","sessionId":"c\td","startedAt":3}]"#;

const BOARD: &str = "BAY  5H unknown  7D unknown  read unknown  gate open  skipped 0 - 0 - 0\n\
     NAME     LIVE   STATUS   WHEEL                    DIALOG             CLI\n\
     builder  live   unknown  driver                   none               2.1.287 verified\n\
     -- UNWRAPPED - READ-ONLY --\n\
     a\\x0Ab   live   idle     n/a                      n/a                n/a\n\
     c\\x09d   live   unknown  n/a                      n/a                n/a\n\
     e\\x85f   live   busy     n/a                      n/a                n/a\n";

/// Each row's value columns start where the caption's words do: the byte before each is a space,
/// the byte at it is not.
fn assert_columns(board: &str) {
    let lines: Vec<&str> = board.lines().collect();
    let caption = lines[1];
    for row in lines[2..].iter().filter(|l| !l.starts_with("--")) {
        for word in ["LIVE", "STATUS", "WHEEL", "DIALOG", "CLI"] {
            let at = caption.find(word).expect("a caption word");
            assert_eq!(row.as_bytes()[at - 1], b' ', "{word} in {row:?}");
            assert_ne!(row.as_bytes()[at], b' ', "{word} in {row:?}");
        }
        assert!(!row.ends_with(' '), "{row:?}");
    }
}

fn assert_board(board: &str) {
    assert_eq!(board, BOARD);
    assert_columns(board);
    for line in board.lines() {
        assert!(line.bytes().all(|b| (b' '..=b'~').contains(&b)), "{line:?}");
    }
}

/// `viola --home <home> list` piped, `PATH` the bin dir alone, with `env` set.
fn piped(bin: &Path, home: &Path, env: &[(&str, &str)]) -> (Option<i32>, String) {
    let out = Command::new(VIOLA)
        .arg("--home")
        .arg(home)
        .arg("list")
        .env_remove("VIOLA_NAME")
        .env("PATH", bin)
        .envs(env.iter().copied())
        .output()
        .expect("viola");
    assert!(out.stderr.is_empty(), "{:?}", out.stderr);
    (
        out.status.code(),
        String::from_utf8(out.stdout).expect("utf-8"),
    )
}

#[rstest]
fn cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode(stamped_home: StampedHome) {
    let wrapper = support::events::boot(stamped_home, None, &[]);
    let home = wrapper.home().to_path_buf();
    let scratch = home.parent().expect("the home has a parent").to_path_buf();
    let bin = fake::claude_bin(&scratch, "claude-rows", None, Some(PLANTED.as_bytes()));

    let (code, board) = piped(&bin, &home, &[]);
    assert_eq!(code, Some(0));
    assert_board(&board);

    let (code, board) = piped(&bin, &home, &[("NO_COLOR", "1"), ("TERM", "dumb")]);
    assert_eq!(code, Some(0));
    assert_board(&board);

    let args: Vec<OsString> = vec!["--home".into(), home.clone().into(), "list".into()];
    let path = bin.to_str().expect("utf-8 path");
    // Wide enough that no row wraps where the terminal re-renders the stream (ConPTY).
    let wide = Size {
        cols: 200,
        ..Size::DEFAULT
    };
    // `NO_COLOR` keeps the board plain, so the reading is that a name's control never reaches the
    // terminal raw.
    let mut pty = OuterPty::spawn_sized(
        Path::new(VIOLA),
        &args,
        &[("PATH", path), ("NO_COLOR", "1")],
        wide,
    );
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    let stream = pty.finish();
    // A Unix terminal adds CR before each LF and nothing else: viola wrote no escape sequence.
    #[cfg(unix)]
    assert!(!stream.contains(&0x1b), "an escape sequence on a terminal");
    let shown = String::from_utf8(stream).expect("utf-8");
    #[cfg(unix)]
    assert_board(&shown.replace("\r\n", "\n"));
    // ConPTY re-renders the stream with cursor moves of its own, so each escaped name is read as
    // text, and no raw LF, TAB or C1 character of a name reaches the terminal.
    #[cfg(windows)]
    {
        for text in [
            "-- UNWRAPPED - READ-ONLY --",
            "a\\x0Ab",
            "c\\x09d",
            "e\\x85f",
        ] {
            assert!(shown.contains(text), "{text} in {shown:?}");
        }
        for raw in ["a\nb", "c\td", "e\u{85}f"] {
            assert!(!shown.contains(raw), "{raw:?} in {shown:?}");
        }
    }

    let ran = viola_on(&bin, &home, &["list", "--json"], None, None);
    assert_eq!(ran.code, Some(0), "stderr: {}", ran.stderr);
    let doc: Value = serde_json::from_str(&ran.stdout).expect("one document");
    let names: Vec<&Value> = doc["ok"]["items"]
        .as_array()
        .expect("items")
        .iter()
        .map(|item| &item["name"])
        .collect();
    assert_eq!(
        names,
        [
            &json!("builder"),
            &json!("a\nb"),
            &json!("c\td"),
            &json!("e\u{85}f")
        ]
    );
    wrapper.stop();
}

/// The variables the colour decision reads, and the instance a test-started `viola` must not take.
const INHERITED: [&str; 6] = [
    "NO_COLOR",
    "TERM",
    "COLORTERM",
    "VIOLA_NAME",
    "VIOLA_DIR",
    "VIOLA_BIN",
];

/// `viola --home <home> list` under an outer PTY `cols` wide: `PATH` the bin dir alone, `env` set
/// and every other `INHERITED` name removed. The stream it printed, as the terminal produced it.
fn on_a_terminal(bin: &Path, home: &Path, env: &[(&str, &str)], cols: u16) -> Vec<u8> {
    let args: Vec<OsString> = vec!["--home".into(), home.into(), "list".into()];
    let path = bin.to_str().expect("utf-8 path");
    let mut set = vec![("PATH", path)];
    set.extend_from_slice(env);
    let remove: Vec<&str> = INHERITED
        .into_iter()
        .filter(|name| !env.iter().any(|(set, _)| set == name))
        .collect();
    let size = Size {
        cols,
        ..Size::DEFAULT
    };
    let cwd = std::env::current_dir().expect("cwd");
    let mut pty = OuterPty::spawn_without(Path::new(VIOLA), &args, &set, &remove, size, &cwd);
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    pty.finish()
}

/// `viola --home <home> list [extra]` piped, `PATH` the bin dir alone, `env` set and every other
/// `INHERITED` name removed: its exit, stdout and stderr.
fn piped_with(
    bin: &Path,
    home: &Path,
    extra: &[&str],
    env: &[(&str, &str)],
) -> (Option<i32>, Vec<u8>, Vec<u8>) {
    let mut command = Command::new(VIOLA);
    command.arg("--home").arg(home).arg("list").args(extra);
    for name in INHERITED {
        command.env_remove(name);
    }
    let out = command
        .env("PATH", bin)
        .envs(env.iter().copied())
        .output()
        .expect("viola");
    (out.status.code(), out.stdout, out.stderr)
}

/// Every `ESC … final byte` sequence in the stream, in order.
#[cfg(unix)]
fn escape_sequences(stream: &[u8]) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < stream.len() {
        if stream[i] != 0x1b {
            i += 1;
            continue;
        }
        let mut end = i + 1;
        while end < stream.len() && !stream[end].is_ascii_alphabetic() {
            end += 1;
        }
        out.push(String::from_utf8_lossy(&stream[i..=end.min(stream.len() - 1)]).into_owned());
        i = end + 1;
    }
    out
}

#[cfg(unix)]
const BOLD: &str = "\x1b[1m";
#[cfg(unix)]
const RESET: &str = "\x1b[0m";

/// A booted `builder` with its permission dialog raised and pending, and a bin dir whose `claude`
/// lists no session.
fn builder_with_a_dialog(
    stamped_home: StampedHome,
) -> (support::home::Wrapper, std::path::PathBuf) {
    let wrapper = support::events::boot(
        stamped_home,
        Some("fixtures/fake-scripts/path4-permission.json"),
        &[],
    );
    support::events::raise(&wrapper, "permission", 1);
    let scratch = wrapper
        .home()
        .parent()
        .expect("the home has a parent")
        .to_path_buf();
    let bin = fake::claude_bin(&scratch, "claude-none", None, Some(b"[]"));
    (wrapper, bin)
}

/// On a terminal that takes colour, `viola list` puts SGR only beside its word: bold on the NAME
/// value, the attention token on `DIALOG` with its kind word plain, and no dim on a live board
/// (a11y-plan §4 P6; design-system §Tokens (platform-specific)).
#[rstest]
fn cross_list_rows_style_each_span_beside_its_word_on_a_terminal(stamped_home: StampedHome) {
    let (wrapper, bin) = builder_with_a_dialog(stamped_home);
    let home = wrapper.home().to_path_buf();
    for (env, amber) in [
        (&[("TERM", "xterm-256color")][..], "\x1b[38;5;172m"),
        (
            &[("TERM", "xterm-256color"), ("COLORTERM", "truecolor")][..],
            "\x1b[38;2;217;119;6m",
        ),
        (&[("TERM", "xterm")][..], "\x1b[33m"),
    ] {
        let stream = on_a_terminal(&bin, &home, env, 200);
        #[cfg(unix)]
        {
            let shown = String::from_utf8(stream.clone()).expect("utf-8");
            assert_eq!(
                escape_sequences(&stream),
                [BOLD, RESET, amber, RESET],
                "{env:?}: {shown:?}"
            );
            assert!(
                shown.contains(&format!("{BOLD}builder{RESET}  live")),
                "{shown:?}"
            );
            assert!(
                shown.contains(&format!("{amber}DIALOG{RESET} permission  ")),
                "{shown:?}"
            );
        }
        // ConPTY re-renders the stream: the attributes it carries are read, not its sequence.
        #[cfg(windows)]
        {
            let _ = amber;
            let attributes = support::sgr::sgr_attributes(&stream);
            assert!(attributes.contains(&1), "{env:?}: {attributes:?}");
            assert!(support::sgr::sets_a_foreground(&stream), "{env:?}");
            assert!(!attributes.contains(&2), "{env:?}: {attributes:?}");
        }
    }
    wrapper.stop();
}

/// No SGR under `NO_COLOR` (an empty value included), `TERM=dumb`, a piped stdout or `--json`,
/// over the same board with its dialog (a11y-plan §4 P6).
#[rstest]
fn cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json(stamped_home: StampedHome) {
    let (wrapper, bin) = builder_with_a_dialog(stamped_home);
    let home = wrapper.home().to_path_buf();
    let assert_plain = |label: &str, stream: &[u8]| {
        #[cfg(unix)]
        assert!(!stream.contains(&0x1b), "{label}: {stream:?}");
        #[cfg(windows)]
        {
            let attributes = support::sgr::sgr_attributes(stream);
            assert!(
                attributes.iter().all(|&a| a == 0),
                "{label}: {attributes:?}"
            );
        }
        assert!(
            String::from_utf8_lossy(stream).contains("DIALOG permission"),
            "{label}"
        );
    };
    for (label, env) in [
        (
            "NO_COLOR=1",
            [("TERM", "xterm-256color"), ("NO_COLOR", "1")],
        ),
        (
            "NO_COLOR empty",
            [("TERM", "xterm-256color"), ("NO_COLOR", "")],
        ),
        ("TERM=dumb", [("TERM", "dumb"), ("COLORTERM", "truecolor")]),
    ] {
        assert_plain(label, &on_a_terminal(&bin, &home, &env, 200));
    }
    let colourful = [("TERM", "xterm-256color"), ("COLORTERM", "truecolor")];
    let (code, stdout, stderr) = piped_with(&bin, &home, &[], &colourful);
    assert_eq!(code, Some(0));
    assert!(stderr.is_empty(), "{stderr:?}");
    assert!(!stdout.contains(&0x1b), "piped: {stdout:?}");
    assert!(String::from_utf8_lossy(&stdout).contains("DIALOG permission"));
    let (code, stdout, stderr) = piped_with(&bin, &home, &["--json"], &colourful);
    assert_eq!(code, Some(0));
    assert!(stderr.is_empty(), "{stderr:?}");
    assert!(!stdout.contains(&0x1b), "--json: {stdout:?}");
    let doc: Value = serde_json::from_slice(&stdout).expect("one document");
    assert_eq!(doc["ok"]["items"][0]["dialog_pending"], json!(true));
    wrapper.stop();
}

/// A 36-character session-shaped id, the name of a session viola does not wrap.
const LONG_SESSION: &str = "0b7c9f4e-1d2a-4c3b-8e5f-6a7b8c9d0e1f";

/// On a terminal too narrow for it an unwrapped NAME is cut to end in `...`; a wide terminal, a
/// pipe and `--json` keep it whole (design-system §Surface: cli Width).
#[rstest]
fn cross_list_rows_truncate_an_unwrapped_name_only_on_a_terminal_too_narrow_for_it(
    stamped_home: StampedHome,
) {
    let wrapper = support::events::boot(stamped_home, None, &[]);
    let home = wrapper.home().to_path_buf();
    let scratch = home.parent().expect("the home has a parent").to_path_buf();
    let row = format!(
        r#"[{{"cwd":"~/a","kind":"interactive","name":"t","sessionId":"{LONG_SESSION}","startedAt":1,"status":"idle"}}]"#
    );
    let bin = fake::claude_bin(&scratch, "claude-long", None, Some(row.as_bytes()));
    let plain = [("NO_COLOR", "1")];

    let narrow = String::from_utf8(on_a_terminal(&bin, &home, &plain, 80)).expect("utf-8");
    assert!(narrow.contains("0b7c9f4e-1d2...  live"), "{narrow:?}");
    assert!(!narrow.contains(LONG_SESSION), "{narrow:?}");

    let wide = String::from_utf8(on_a_terminal(&bin, &home, &plain, 200)).expect("utf-8");
    assert!(wide.contains(&format!("{LONG_SESSION}  live")), "{wide:?}");

    let (code, stdout, _) = piped_with(&bin, &home, &[], &plain);
    assert_eq!(code, Some(0));
    assert!(
        String::from_utf8_lossy(&stdout).contains(&format!("{LONG_SESSION}  live")),
        "{stdout:?}"
    );

    let (code, stdout, _) = piped_with(&bin, &home, &["--json"], &plain);
    assert_eq!(code, Some(0));
    let doc: Value = serde_json::from_slice(&stdout).expect("one document");
    let names: Vec<&Value> = doc["ok"]["items"]
        .as_array()
        .expect("items")
        .iter()
        .map(|item| &item["name"])
        .collect();
    assert_eq!(names, [&json!("builder"), &json!(LONG_SESSION)]);
    wrapper.stop();
}
