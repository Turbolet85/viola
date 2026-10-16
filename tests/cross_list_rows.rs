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
    let mut pty = OuterPty::spawn_sized(Path::new(VIOLA), &args, &[("PATH", path)], wide);
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
