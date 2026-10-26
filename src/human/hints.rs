//! The fixed `hint:` tables of `send` and `answer`.

/// The design-system cli pattern 2 hint for `human-typing`, whichever its detail. No hint names
/// the verb that hands the wheel back: that is the human's alone.
const HUMAN_TYPING_HINT: &str = "the human has the wheel; send again after the human hands it back";

/// The design-system cli pattern 2 hint for an `answer` refusal: its reason, or for
/// `not-delivered` its detail. Never the answer's text.
pub(crate) fn answer_hint(reason: &str, detail: Option<&str>) -> Option<&'static str> {
    match (reason, detail) {
        ("human-typing", _) => Some(HUMAN_TYPING_HINT),
        ("unverified-cli", _) => Some("run viola verify for this CLI version"),
        ("not-delivered", Some("unknown-dialog")) => {
            Some("that dialog is not pending; viola list shows the current DIALOG")
        }
        ("not-delivered", Some("control-character")) => {
            Some("the text contains a control character (only LF, CR, TAB are allowed)")
        }
        _ => None,
    }
}

/// The design-system cli pattern 2 hint for a `send` cause: a `not-delivered` detail, the
/// `human-typing` or `budget-paused` reason, or for exit 21 `not-running` or `server-verify`. Never
/// the sent text, a path or a pid.
pub(crate) fn send_hint(name: &str, cause: &str) -> Option<String> {
    Some(match cause {
        "human-typing" => HUMAN_TYPING_HINT.to_owned(),
        "budget-paused" => {
            "budget gate is closed; it lifts at the window reset, or when the human lifts it"
                .to_owned()
        }
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
        "not-running" => format!("{name} is not running; viola list shows the live instances"),
        "server-verify" => format!(
            "the process on {name}'s endpoint is not its recorded wrapper; it was not contacted"
        ),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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
    #[case::not_running(
        "not-running",
        "builder is not running; viola list shows the live instances"
    )]
    #[case::server_verify(
        "server-verify",
        "the process on builder's endpoint is not its recorded wrapper; it was not contacted"
    )]
    #[case::human_typing(
        "human-typing",
        "the human has the wheel; send again after the human hands it back"
    )]
    #[case::budget_paused(
        "budget-paused",
        "budget gate is closed; it lifts at the window reset, or when the human lifts it"
    )]
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
            "not-running",
            "server-verify",
            "human-typing",
            "budget-paused",
            "five-hour",
            "seven-day",
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
                hints.extend(answer_hint(reason, detail).map(str::to_owned));
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
    #[case::other_detail("not-delivered", Some("turn-running"), None)]
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
        assert_eq!(answer_hint(reason, detail), hint);
    }
}
