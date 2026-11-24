//! The fixed `hint:` tables of `send` and `answer`.

/// The design-system cli pattern 2 hint for `human-typing`, whichever its detail. No hint names
/// the verb that hands the wheel back: that is the human's alone.
const HUMAN_TYPING_HINT: &str = "the human has the wheel; send again after the human hands it back";

/// The design-system hint for `unverified-cli`, the same for `send` and `answer`.
const UNVERIFIED_CLI_HINT: &str = "run viola verify for this CLI version";

/// The design-system cli pattern 2 hint for an `answer` refusal: its reason, or for
/// `not-delivered` its detail, a detail `send` shares taking `send`'s string. Never the
/// answer's text.
pub(crate) fn answer_hint(name: &str, reason: &str, detail: Option<&str>) -> Option<String> {
    match (reason, detail) {
        ("human-typing", _) => Some(HUMAN_TYPING_HINT.to_owned()),
        ("unverified-cli", _) => Some(UNVERIFIED_CLI_HINT.to_owned()),
        ("not-delivered", Some("unknown-dialog")) => {
            Some("that dialog is not pending; viola list shows the current DIALOG".to_owned())
        }
        (
            "not-delivered",
            Some(
                detail @ ("control-character"
                | "empty-text"
                | "input-not-ready"
                | "no-prompt-submitted"
                | "turn-running"),
            ),
        ) => send_hint(name, detail),
        _ => None,
    }
}

/// The design-system cli pattern 2 hint for a `send` cause: a `not-delivered` detail, the
/// `human-typing` or `unverified-cli` reason, or for exit 21 `not-running`,
/// `unwrapped-name`, `endpoint-lost`, `server-verify` or `socket-dir`. Never the sent text, a
/// path or a pid.
pub(crate) fn send_hint(name: &str, cause: &str) -> Option<String> {
    Some(match cause {
        "human-typing" => HUMAN_TYPING_HINT.to_owned(),
        "unverified-cli" => UNVERIFIED_CLI_HINT.to_owned(),
        "control-character" => {
            "the text contains a control character (only LF, CR, TAB are allowed)".to_owned()
        }
        "empty-text" => {
            "the text is empty once its trailing newlines are removed; send a text with content"
                .to_owned()
        }
        "input-not-ready" => {
            format!(
                "{name} was not ready for input; send again, and if it repeats a human must look at the session"
            )
        }
        "no-prompt-submitted" => {
            format!("{name} did not submit the prompt; check it, then send again")
        }
        "turn-running" => format!("a turn is running; viola wait {name} first"),
        "paste-cut" => format!(
            "{name} did not take the whole text in time; a human must look at the session before the next send"
        ),
        "not-running" => format!("{name} is not running; viola list shows the live instances"),
        "unwrapped-name" => format!(
            "{name} is not a wrapped session; only sessions started with viola run take commands"
        ),
        "endpoint-lost" => format!(
            "{name}'s wrapper went away during the call; viola list shows whether it is still running"
        ),
        "server-verify" => format!(
            "the process on {name}'s endpoint is not its recorded wrapper; it was not contacted"
        ),
        "socket-dir" => "viola's socket directory must be a directory you own, readable only by \
                         you; viola does not change its permissions"
            .to_owned(),
        _ => return None,
    })
}

/// `viola open`'s `terminal-not-found` pair: no `foot` on `PATH`, nothing spawned.
pub(crate) fn terminal_not_found_pair() -> (&'static str, &'static str) {
    (
        "no foot terminal on PATH",
        "install foot, the terminal viola open starts",
    )
}

/// `viola open`'s `not-live` pair: the instance did not come up within the live bound, and its
/// window was left standing.
pub(crate) fn not_live_pair(name: &str) -> (String, String) {
    (
        format!("{name} did not come up within 30 s"),
        format!("the window stays open; viola list shows {name} once it is live"),
    )
}

/// `viola focus`'s refusal pair for its closed exit-1 `code`: no window found, or the Hyprland
/// seam's own word. Plain ASCII, naming only the parsed `name`.
pub(crate) fn focus_pair(code: &str, name: &str) -> Option<(String, String)> {
    let (unable, hint) = match code {
        "window-not-found" => (
            format!("no window of {name} was found"),
            format!("start {name} in its own terminal window, or open it with viola open"),
        ),
        "unsupported" => (
            "no hyprctl on PATH".to_owned(),
            "viola focus moves windows on Hyprland; run it in a Hyprland session".to_owned(),
        ),
        "unreachable" => (
            "Hyprland did not answer".to_owned(),
            "run hyprctl version to check the compositor answers".to_owned(),
        ),
        "oversize" => (
            "Hyprland's reply passed viola's frame cap".to_owned(),
            "viola reads no reply past its frame cap; nothing was focused".to_owned(),
        ),
        "malformed" => (
            "Hyprland's reply was not understood".to_owned(),
            "check that hyprctl matches the running Hyprland".to_owned(),
        ),
        "refused" => (
            format!("Hyprland refused to focus {name}'s window"),
            format!("the window may have closed; viola list shows {name}"),
        ),
        _ => return None,
    };
    Some((unable, hint))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn focus_pairs_are_the_fixed_lines() {
        let pair = |code| focus_pair(code, "builder").expect("a pair");
        let want = [
            (
                "window-not-found",
                "no window of builder was found",
                "start builder in its own terminal window, or open it with viola open",
            ),
            (
                "unsupported",
                "no hyprctl on PATH",
                "viola focus moves windows on Hyprland; run it in a Hyprland session",
            ),
            (
                "unreachable",
                "Hyprland did not answer",
                "run hyprctl version to check the compositor answers",
            ),
            (
                "oversize",
                "Hyprland's reply passed viola's frame cap",
                "viola reads no reply past its frame cap; nothing was focused",
            ),
            (
                "malformed",
                "Hyprland's reply was not understood",
                "check that hyprctl matches the running Hyprland",
            ),
            (
                "refused",
                "Hyprland refused to focus builder's window",
                "the window may have closed; viola list shows builder",
            ),
        ];
        for (code, unable, hint) in want {
            assert_eq!(pair(code), (unable.to_owned(), hint.to_owned()), "{code}");
            assert!(unable.is_ascii() && hint.is_ascii(), "{code}");
        }
        assert_eq!(focus_pair("other", "builder"), None);
    }

    #[test]
    fn open_pairs_are_the_fixed_lines() {
        assert_eq!(
            terminal_not_found_pair(),
            (
                "no foot terminal on PATH",
                "install foot, the terminal viola open starts"
            )
        );
        assert_eq!(
            not_live_pair("builder"),
            (
                "builder did not come up within 30 s".to_owned(),
                "the window stays open; viola list shows builder once it is live".to_owned()
            )
        );
    }

    #[rstest]
    #[case::control_character(
        "control-character",
        "the text contains a control character (only LF, CR, TAB are allowed)"
    )]
    #[case::empty_text(
        "empty-text",
        "the text is empty once its trailing newlines are removed; send a text with content"
    )]
    #[case::input_not_ready(
        "input-not-ready",
        "builder was not ready for input; send again, and if it repeats a human must look at the session"
    )]
    #[case::no_prompt_submitted(
        "no-prompt-submitted",
        "builder did not submit the prompt; check it, then send again"
    )]
    #[case::turn_running("turn-running", "a turn is running; viola wait builder first")]
    #[case::paste_cut(
        "paste-cut",
        "builder did not take the whole text in time; a human must look at the session before the next send"
    )]
    #[case::not_running(
        "not-running",
        "builder is not running; viola list shows the live instances"
    )]
    #[case::unwrapped_name(
        "unwrapped-name",
        "builder is not a wrapped session; only sessions started with viola run take commands"
    )]
    #[case::endpoint_lost(
        "endpoint-lost",
        "builder's wrapper went away during the call; viola list shows whether it is still running"
    )]
    #[case::server_verify(
        "server-verify",
        "the process on builder's endpoint is not its recorded wrapper; it was not contacted"
    )]
    #[case::socket_dir(
        "socket-dir",
        "viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions"
    )]
    #[case::human_typing(
        "human-typing",
        "the human has the wheel; send again after the human hands it back"
    )]
    #[case::unverified_cli("unverified-cli", "run viola verify for this CLI version")]
    fn send_hint_is_the_design_string(#[case] cause: &str, #[case] hint: &str) {
        assert_eq!(send_hint("builder", cause).as_deref(), Some(hint));
    }

    #[test]
    fn send_hint_has_none_for_an_opaque_cause() {
        assert_eq!(send_hint("builder", "unknown"), None);
        assert_eq!(send_hint("builder", "unknown-dialog"), None);
    }

    /// No hint either table hands out names `release`: handing the wheel back is the human's verb.
    #[test]
    fn no_hint_names_release() {
        let causes = [
            "control-character",
            "empty-text",
            "input-not-ready",
            "no-prompt-submitted",
            "turn-running",
            "paste-cut",
            "not-running",
            "unwrapped-name",
            "endpoint-lost",
            "server-verify",
            "socket-dir",
            "human-typing",
            "unknown-dialog",
            "manual-pause",
            "unverified-cli",
            "not-delivered",
            "unknown",
        ];
        let mut hints: Vec<String> = causes
            .iter()
            .filter_map(|cause| send_hint("builder", cause))
            .collect();
        for reason in causes {
            for detail in causes.iter().copied().map(Some).chain([None]) {
                hints.extend(answer_hint("builder", reason, detail));
            }
        }
        assert!(hints.len() >= 10, "{hints:?}");
        for hint in hints {
            assert!(!hint.contains("release"), "{hint}");
        }
    }

    #[rstest]
    #[case::unverified("unverified-cli", None, Some("run viola verify for this CLI version"))]
    #[case::unknown_dialog(
        "not-delivered",
        Some("unknown-dialog"),
        Some("that dialog is not pending; viola list shows the current DIALOG")
    )]
    #[case::control_character(
        "not-delivered",
        Some("control-character"),
        Some("the text contains a control character (only LF, CR, TAB are allowed)")
    )]
    #[case::empty_text(
        "not-delivered",
        Some("empty-text"),
        Some("the text is empty once its trailing newlines are removed; send a text with content")
    )]
    #[case::input_not_ready(
        "not-delivered",
        Some("input-not-ready"),
        Some(
            "builder was not ready for input; send again, and if it repeats a human must look at the session"
        )
    )]
    #[case::no_prompt_submitted(
        "not-delivered",
        Some("no-prompt-submitted"),
        Some("builder did not submit the prompt; check it, then send again")
    )]
    #[case::turn_running(
        "not-delivered",
        Some("turn-running"),
        Some("a turn is running; viola wait builder first")
    )]
    #[case::opaque_detail("not-delivered", Some("not-running"), None)]
    #[case::no_detail("not-delivered", None, None)]
    #[case::unknown("unknown", None, None)]
    #[case::human_typing(
        "human-typing",
        None,
        Some("the human has the wheel; send again after the human hands it back")
    )]
    #[case::manual_pause(
        "human-typing",
        Some("manual-pause"),
        Some("the human has the wheel; send again after the human hands it back")
    )]
    fn answer_hint_is_the_fixed_table(
        #[case] reason: &str,
        #[case] detail: Option<&str>,
        #[case] hint: Option<&str>,
    ) {
        assert_eq!(answer_hint("builder", reason, detail).as_deref(), hint);
    }
}
