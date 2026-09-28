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

/// The parameters of every `ESC [ params m` sequence, an empty list read as `0`. Extended colour
/// sub-parameters (`38;5;n`, `38;2;r;g;b`) are skipped, so their values never read as attributes.
fn sgr_attributes(stream: &[u8]) -> Vec<u32> {
    let mut attributes = Vec::new();
    let mut i = 0;
    while i + 1 < stream.len() {
        if stream[i] != ESC || stream[i + 1] != b'[' {
            i += 1;
            continue;
        }
        let start = i + 2;
        let mut end = start;
        while end < stream.len() && (stream[end].is_ascii_digit() || stream[end] == b';') {
            end += 1;
        }
        if end < stream.len() && stream[end] == b'm' {
            let params = std::str::from_utf8(&stream[start..end]).expect("ascii");
            let mut values = params.split(';').map(|p| p.parse::<u32>().unwrap_or(0));
            while let Some(value) = values.next() {
                if matches!(value, 38 | 48 | 58) {
                    let _ = match values.next() {
                        Some(5) => values.next(),
                        Some(2) => values.nth(2),
                        _ => None,
                    };
                    continue;
                }
                attributes.push(value);
            }
        }
        i = end;
    }
    attributes
}

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
}
