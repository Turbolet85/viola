//! Claude Code's status line, as viola wraps it: the payload the CLI hands a statusline command on
//! stdin, the settings key that names the user's own command, the per-session override that puts
//! `viola hook statusline` in its place, and the shell a statusline command runs through
//! (architecture §Standard Contracts → Hook contract). Pure functions over bytes and strings: no
//! file, clock or environment is read here.

use serde_json::{Value, json};

/// The flag that hands the per-session settings override to the `claude` child.
pub const SETTINGS_FLAG: &str = "--settings";

/// The user-scope settings file, relative to the user's home: the one file read for the user's own
/// statusline command. Project, local and managed settings are not read.
pub const USER_SETTINGS: [&str; 2] = [".claude", "settings.json"];

/// The shell a statusline command runs through.
pub const BIN_SH: &str = "/bin/sh";

/// The hidden capture arm's option naming the word the CLI's shell expands `"$0"` to.
pub const SHELL_FLAG: &str = "--shell";

/// The user's own statusline command in a settings document: `statusLine.command` where
/// `statusLine.type` is `command`. Any other document holds no command; none is an error.
pub fn user_command(settings: &[u8]) -> Option<String> {
    let doc: Value = serde_json::from_slice(settings).ok()?;
    let line = doc.get("statusLine")?;
    if line.get("type")?.as_str()? != "command" {
        return None;
    }
    let command = line.get("command")?.as_str()?;
    (!command.is_empty() && !command.contains('\0')).then(|| command.to_owned())
}

/// The per-session settings override: a `statusLine` whose command is the pinned binary's
/// forward-slash absolute path, one space, `hook statusline`. The CLI runs that string through a
/// shell, and the path is written unquoted, so a path holding anything but ASCII letters, digits
/// and `_ - . / :` gets no override.
pub fn override_document(pinned_bin_fwd: &str) -> Option<String> {
    if !pinned_bin_fwd.chars().all(is_shell_plain) {
        return None;
    }
    let command = format!("{pinned_bin_fwd} hook statusline");
    Some(json!({"statusLine": {"type": "command", "command": command}}).to_string())
}

fn is_shell_plain(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '/' | ':')
}

/// `viola verify`'s Run B settings: a `statusLine` whose command is the pinned binary, `hook
/// statusline --capture <captures dir> --shell "$0"`, so each payload is filed with the word the
/// CLI's shell expands `"$0"` to. Built only when both paths hold nothing but ASCII letters, digits
/// and `_ - . / :`.
pub fn probe_document(pinned_bin_fwd: &str, captures_dir_fwd: &str) -> Option<String> {
    let plain = |path: &str| path.chars().all(is_shell_plain);
    if !plain(pinned_bin_fwd) || !plain(captures_dir_fwd) {
        return None;
    }
    let command = format!(
        "{pinned_bin_fwd} hook statusline --capture {captures_dir_fwd} {SHELL_FLAG} \"$0\""
    );
    Some(json!({"statusLine": {"type": "command", "command": command}}).to_string())
}

/// The shell a user's statusline command runs through, with the command as ONE argument: the
/// program by its absolute path, never a lookup.
pub fn shell_argv(command: &str) -> [&str; 3] {
    [BIN_SH, "-c", command]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::a_command(
        json!({"statusLine": {"type": "command", "command": "~/.claude/statusline.sh"}}),
        Some("~/.claude/statusline.sh")
    )]
    #[case::beside_other_keys(
        json!({"model": "m", "statusLine": {"type": "command", "command": "a b 'c d'", "padding": 0}}),
        Some("a b 'c d'")
    )]
    #[case::no_status_line(json!({"model": "m"}), None)]
    #[case::status_line_null(json!({"statusLine": null}), None)]
    #[case::status_line_a_string(json!({"statusLine": "echo x"}), None)]
    #[case::no_type(json!({"statusLine": {"command": "echo x"}}), None)]
    #[case::another_type(json!({"statusLine": {"type": "static", "command": "echo x"}}), None)]
    #[case::type_in_another_case(json!({"statusLine": {"type": "Command", "command": "echo x"}}), None)]
    #[case::type_not_a_string(json!({"statusLine": {"type": 1, "command": "echo x"}}), None)]
    #[case::no_command(json!({"statusLine": {"type": "command"}}), None)]
    #[case::command_not_a_string(json!({"statusLine": {"type": "command", "command": ["echo", "x"]}}), None)]
    #[case::empty_command(json!({"statusLine": {"type": "command", "command": ""}}), None)]
    #[case::a_nul_in_the_command(json!({"statusLine": {"type": "command", "command": "echo\u{0}x"}}), None)]
    #[case::an_array_document(json!([{"statusLine": {"type": "command", "command": "echo x"}}]), None)]
    fn statusline_user_command_is_the_command_of_a_command_status_line(
        #[case] settings: Value,
        #[case] command: Option<&str>,
    ) {
        assert_eq!(
            user_command(settings.to_string().as_bytes()).as_deref(),
            command
        );
    }

    #[rstest]
    #[case::not_json(b"not json".as_slice())]
    #[case::empty(b"".as_slice())]
    #[case::cut_short(br#"{"statusLine":{"type":"command","command":"echo"#.as_slice())]
    fn statusline_user_command_of_bytes_that_are_no_document_is_none(#[case] settings: &[u8]) {
        assert_eq!(user_command(settings), None);
    }

    #[test]
    fn statusline_override_is_the_pinned_path_then_hook_statusline() {
        let path = "/home/u/.viola/bin/0.1.0-0123456789abcdef/viola";
        assert_eq!(
            override_document(path).as_deref(),
            Some(
                r#"{"statusLine":{"type":"command","command":"/home/u/.viola/bin/0.1.0-0123456789abcdef/viola hook statusline"}}"#
            )
        );
        let drive = override_document("C:/h/bin/k/viola.exe").expect("an override");
        let doc: Value = serde_json::from_str(&drive).expect("json");
        assert_eq!(
            doc["statusLine"]["command"],
            "C:/h/bin/k/viola.exe hook statusline"
        );
    }

    /// The user's own command is read back from the override the way the CLI would read it.
    #[test]
    fn statusline_override_reads_back_as_a_command_status_line() {
        let doc = override_document("/h/bin/k/viola").expect("an override");
        assert_eq!(
            user_command(doc.as_bytes()).as_deref(),
            Some("/h/bin/k/viola hook statusline")
        );
    }

    #[rstest]
    #[case::a_space("/home/a user/.viola/bin/k/viola")]
    #[case::a_quote("/home/u'/.viola/bin/k/viola")]
    #[case::a_double_quote("/home/u\"/bin/k/viola")]
    #[case::a_dollar("/home/$USER/bin/k/viola")]
    #[case::a_backslash("C:\\h\\bin\\k\\viola.exe")]
    #[case::a_semicolon("/h/bin;k/viola")]
    #[case::an_ampersand("/h/bin&k/viola")]
    #[case::a_tilde("~/.viola/bin/k/viola")]
    #[case::a_paren("/h/bin(k)/viola")]
    #[case::a_newline("/h/bin\nk/viola")]
    #[case::a_non_ascii_letter("/home/\u{e9}/bin/k/viola")]
    #[case::a_non_ascii_digit("/home/\u{663}/bin/k/viola")]
    fn statusline_override_of_a_path_a_shell_could_misread_is_none(#[case] path: &str) {
        assert_eq!(override_document(path), None);
    }

    #[test]
    fn statusline_override_on_this_host_is_built() {
        assert!(override_document("/h/bin/k/viola").is_some());
        assert_eq!(override_document("/h/bin k/viola"), None);
        assert_eq!(SETTINGS_FLAG, "--settings");
        assert_eq!(USER_SETTINGS, [".claude", "settings.json"]);
    }

    #[test]
    fn statusline_probe_document_runs_the_capture_arm_with_the_shell_word() {
        assert_eq!(
            probe_document(
                "/home/u/.viola/bin/0.1.0-0123456789abcdef/viola",
                "/home/u/.viola/ledger/probes/7/typed/statusline",
            )
            .as_deref(),
            Some(
                r#"{"statusLine":{"type":"command","command":"/home/u/.viola/bin/0.1.0-0123456789abcdef/viola hook statusline --capture /home/u/.viola/ledger/probes/7/typed/statusline --shell \"$0\""}}"#
            )
        );
        let doc = probe_document("/h/bin/k/viola", "/h/p/typed/statusline").expect("a document");
        assert_eq!(
            user_command(doc.as_bytes()).as_deref(),
            Some("/h/bin/k/viola hook statusline --capture /h/p/typed/statusline --shell \"$0\"")
        );
        assert_eq!(SHELL_FLAG, "--shell");
        assert_eq!(BIN_SH, "/bin/sh");
    }

    #[rstest]
    #[case::a_space_in_the_pinned_path("/home/a user/bin/k/viola", "/h/p/typed/statusline")]
    #[case::a_quote_in_the_pinned_path("/home/u'/bin/k/viola", "/h/p/typed/statusline")]
    #[case::a_backslash_in_the_pinned_path("C:\\h\\bin\\viola.exe", "/h/p/typed/statusline")]
    #[case::a_space_in_the_captures_dir("/h/bin/k/viola", "/h/a dir/typed/statusline")]
    #[case::a_dollar_in_the_captures_dir("/h/bin/k/viola", "/h/$HOME/typed/statusline")]
    #[case::a_semicolon_in_the_captures_dir("/h/bin/k/viola", "/h/p;rm/typed/statusline")]
    #[case::a_non_ascii_letter_in_the_captures_dir("/h/bin/k/viola", "/h/\u{e9}/statusline")]
    fn statusline_probe_document_of_a_path_a_shell_could_misread_is_none(
        #[case] pinned: &str,
        #[case] captures: &str,
    ) {
        assert_eq!(probe_document(pinned, captures), None);
    }

    #[test]
    fn statusline_probe_document_on_a_wrapped_host_is_the_capture_command_document() {
        let expected = Some(
            r#"{"statusLine":{"type":"command","command":"/h/bin/k/viola hook statusline --capture /h/p/typed/statusline --shell \"$0\""}}"#,
        );
        assert_eq!(
            probe_document("/h/bin/k/viola", "/h/p/typed/statusline").as_deref(),
            expected
        );
    }

    #[test]
    fn statusline_shell_is_bin_sh_with_the_command_as_one_argument() {
        let command = "printf '%s' \"a b\"; exit 3";
        assert_eq!(shell_argv(command), ["/bin/sh", "-c", command]);
    }
}
