//! The status line: its command read from a settings document, and the `statusline-echo` mode.

use super::*;

/// `statusLine.command` of a settings document, split on ASCII white space into the program and its
/// arguments.
pub(super) fn statusline_words(settings: &[u8]) -> Option<(String, Vec<String>)> {
    let doc: Value = serde_json::from_slice(settings).ok()?;
    let mut words = doc["statusLine"]["command"]
        .as_str()?
        .split_ascii_whitespace()
        .map(str::to_owned);
    let command = words.next()?;
    Some((command, words.collect()))
}

/// A statusline command's arguments as the real CLI's `/bin/sh` hands them over (2.1.287, read
/// live): the word `"$0"` becomes `/bin/sh`; every other word is unchanged. No shell is run.
pub(super) fn shell_expanded(args: Vec<String>) -> Vec<String> {
    args.into_iter()
        .map(|word| {
            if word == SHELL_NAME_WORD {
                BIN_SH.to_owned()
            } else {
                word
            }
        })
        .collect()
}

/// The `statusline-echo <marker file> [--exit <code>]` mode: stdin read to its end, the fixed
/// output printed, one line holding the hex of that stdin appended to the marker file, then the
/// exit code asked for (0 without one).
pub(super) fn statusline_echo(args: &[String]) -> ExitCode {
    let mut stdin = Vec::new();
    let _ = std::io::stdin().lock().read_to_end(&mut stdin);
    if let Some(marker) = args.first() {
        let line = hex(&stdin) + "\n";
        let file = OpenOptions::new().create(true).append(true).open(marker);
        let _ = file.and_then(|mut f| f.write_all(line.as_bytes()));
    }
    let mut out = std::io::stdout().lock();
    let _ = out
        .write_all(STATUSLINE_ECHO_OUTPUT)
        .and_then(|()| out.flush());
    ExitCode::from(statusline_echo_exit(args))
}

/// The code after `--exit`, when that word follows the marker path; 0 otherwise.
pub(super) fn statusline_echo_exit(args: &[String]) -> u8 {
    match args {
        [_, flag, code, ..] if flag == "--exit" => code.parse().unwrap_or(0),
        _ => 0,
    }
}
