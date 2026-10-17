//! viola's own help and usage text is plain (design-system §Surface: cli bans colour on anything
//! but `DIALOG` / `stale` and any SGR under a non-TTY; a11y-plan §6 CLI equivalent): no variable
//! forces colour into a pipe, and a terminal gets no bold or underline headers. On Windows ConPTY
//! re-renders the stream with its own `ESC[m` / cursor / title bytes, so the terminal oracle reads
//! the SGR parameters, never the screen text.

#[allow(dead_code)]
mod support;

use std::path::Path;
use std::process::{Command, Output};

use support::home::VIOLA;
use support::outer_pty::{EXIT_WITHIN, OuterPty};
#[cfg(unix)]
use support::sgr::redraws;
use support::sgr::{sets_a_foreground, sgr_attributes};

const ESC: u8 = 0x1B;

/// `viola <args>` with stdout and stderr piped and every colour-forcing variable set.
fn forced_into_a_pipe(args: &[&str]) -> Output {
    Command::new(VIOLA)
        .args(args)
        .env_remove("VIOLA_NAME")
        .env_remove("VIOLA_DIR")
        .env("CLICOLOR_FORCE", "1")
        .env("CLICOLOR", "1")
        .output()
        .expect("viola")
}

fn escapes(bytes: &[u8]) -> usize {
    bytes.iter().filter(|&&b| b == ESC).count()
}

/// The support reader's self-check, run once here rather than in every binary that declares
/// `mod support`.
#[test]
fn sgr_attributes_reads_bold_and_underline_but_not_colour_values() {
    assert_eq!(
        sgr_attributes(b"\x1b[1m\x1b[4mUsage:\x1b[0m \x1b[m"),
        [1, 4, 0, 0]
    );
    assert_eq!(
        sgr_attributes(b"\x1b[38;5;1mx\x1b[38;2;1;4;1;22my\x1b[?25l"),
        [22]
    );
    assert!(sets_a_foreground(b"\x1b[38;5;172mDIALOG\x1b[0m"));
    assert!(sets_a_foreground(b"\x1b[1;33mDIALOG"));
    assert!(!sets_a_foreground(b"\x1b[1m\x1b[48;5;33m\x1b[2m\x1b[0m"));
}

/// `viola --help` is the grouped verb table: the served verbs under their group labels, then the
/// options (layout-templates §Output structure — `viola --help`).
const HELP: &str = concat!(
    "Usage: viola [--home <dir>] <verb> [args]\n",
    "\n",
    "board:    list\n",
    "traffic:  send, wait, last, answer\n",
    "wheel:    pause, release\n",
    "setup:    run, revive, verify\n",
    "\n",
    "Options:\n",
    "      --home <HOME>  The viola home (default: <user home>/.viola)\n",
    "  -h, --help         Print help\n",
    "  -V, --version      Print version\n",
);

#[test]
fn help_is_the_grouped_verb_table() {
    let out = forced_into_a_pipe(&["--help"]);
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stderr.is_empty(), "{:?}", out.stderr);
    assert_eq!(String::from_utf8(out.stdout).expect("utf-8"), HELP);
}

/// Every verb the table lists answers its own `--help`; the table offers no verb that exits 2.
#[test]
fn help_lists_only_served_verbs() {
    let served = [
        "list", "send", "wait", "last", "answer", "pause", "release", "run", "revive", "verify",
    ];
    for verb in served {
        let out = forced_into_a_pipe(&[verb, "--help"]);
        assert_eq!(out.status.code(), Some(0), "{verb}");
        // A verb's own usage names the program by its executable: `viola.exe` on Windows.
        let text = String::from_utf8_lossy(&out.stdout).into_owned();
        let usage: Vec<&str> = text
            .lines()
            .find_map(|line| line.strip_prefix("Usage: "))
            .unwrap_or_default()
            .split_whitespace()
            .take(2)
            .collect();
        assert_eq!(
            usage
                .first()
                .map(|program| program.trim_end_matches(".exe")),
            Some("viola"),
            "{verb}: {text:?}"
        );
        assert_eq!(usage.get(1).copied(), Some(verb), "{verb}: {text:?}");
    }
    let table = forced_into_a_pipe(&["--help"]);
    let rows: Vec<String> = String::from_utf8(table.stdout)
        .expect("utf-8")
        .lines()
        .take_while(|line| *line != "Options:")
        .map(str::to_owned)
        .collect();
    let words: Vec<&str> = rows
        .iter()
        .filter_map(|row| row.split_once(':').map(|(_, verbs)| verbs))
        .flat_map(|verbs| verbs.split(','))
        .map(str::trim)
        .filter(|word| !word.is_empty() && !word.contains(' '))
        .collect();
    assert_eq!(words, served, "{rows:?}");
    for unserved in ["hook", "mcp", "help", "link", "unlink", "plugin"] {
        assert!(!words.contains(&unserved), "{unserved} in {rows:?}");
    }
}

#[test]
fn help_is_plain_when_colour_is_forced_into_a_pipe() {
    let out = forced_into_a_pipe(&["--help"]);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(escapes(&out.stdout), 0, "ESC on stdout");
    assert_eq!(escapes(&out.stderr), 0, "ESC on stderr");
    assert!(String::from_utf8_lossy(&out.stdout).contains("Usage:"));
}

#[test]
fn usage_error_is_plain_when_colour_is_forced_into_a_pipe() {
    let out = forced_into_a_pipe(&["frobnicate"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(escapes(&out.stdout), 0, "ESC on stdout");
    assert_eq!(escapes(&out.stderr), 0, "ESC on stderr");
    assert!(String::from_utf8_lossy(&out.stderr).contains("error:"));
}

#[test]
fn help_is_plain_on_a_terminal() {
    let mut pty = OuterPty::spawn(Path::new(VIOLA), &["--help".into()], &[]);
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    let stream = pty.finish();
    let attributes = sgr_attributes(&stream);
    assert!(
        !attributes.iter().any(|&a| a == 1 || a == 4),
        "bold or underline in {attributes:?}"
    );
    // A Unix terminal adds CR before each LF and nothing else: no `\r` redraw. ConPTY adds cursor
    // bytes of its own.
    #[cfg(unix)]
    assert_eq!(redraws(&stream), 0, "{stream:?}");
}
