use std::time::{Duration, Instant};

use nutype::nutype;

pub mod obs;

pub const SERVICE_NAME: &str = "viola";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The byte cap on every external reader (`Read::take`) and on a channel frame.
pub const MAX_FRAME: u64 = 16 * 1024 * 1024;

/// The bound every spine hook process meets, read by the perf gate (test-plan §10 Spine deadline).
pub const SPINE_DEADLINE: Duration = Duration::from_secs(1);

/// How long the wrapper holds a pending dialog for a driver's answer before it leaves the dialog to
/// the human; the embedded `hooks.json` dialog `timeout` exceeds it, so viola, never Claude Code, ends
/// the wait (architecture [Hook Contract]). PROVISIONAL, unmeasured.
pub const DIALOG_DEADLINE: Duration = Duration::from_secs(60);

/// The injected time source: sync code takes its instants from here, so a test drives time
/// without the wall clock (test-plan §8 Time).
pub trait Clock: Send + Sync {
    fn now(&self) -> Instant;
}

/// The production clock.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

/// An instance name: ASCII `[a-z0-9-]`, 1–32 characters, starting with a letter
/// (architecture §Conventions). The only way to build one is `ViolaName::try_new`,
/// so a name that reaches a path join has passed this check.
#[nutype(
    validate(predicate = is_valid_name),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Display)
)]
pub struct ViolaName(String);

/// The closed set of normalised `events.ndjson` kinds; a kind is added here only, never under a
/// Claude-specific name (architecture §Standard Contracts).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventKind {
    SessionStart,
    PromptSubmitted,
    TurnEnded,
    SessionEnd,
    Activity,
    Wheel,
    BudgetGate,
    SendIssued,
    SendConfirmed,
    SendRefused,
    Question,
    Permission,
    Plan,
}

impl EventKind {
    /// The kinds that end a `wait` (architecture §Standard Contracts); every other kind is
    /// log-only.
    pub const WAIT_WAKE: [Self; 5] = [
        Self::TurnEnded,
        Self::Question,
        Self::Permission,
        Self::Plan,
        Self::SessionEnd,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionStart => "session-start",
            Self::PromptSubmitted => "prompt-submitted",
            Self::TurnEnded => "turn-ended",
            Self::SessionEnd => "session-end",
            Self::Activity => "activity",
            Self::Wheel => "wheel",
            Self::BudgetGate => "budget-gate",
            Self::SendIssued => "send-issued",
            Self::SendConfirmed => "send-confirmed",
            Self::SendRefused => "send-refused",
            Self::Question => "question",
            Self::Permission => "permission",
            Self::Plan => "plan",
        }
    }
}

/// Why a request was refused: a normal outcome, never a protocol fault (architecture §Conventions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RefusalReason {
    HumanTyping,
    BudgetPaused,
    UnverifiedCli,
    NotDelivered,
    #[serde(other)]
    Unknown,
}

impl RefusalReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanTyping => "human-typing",
            Self::BudgetPaused => "budget-paused",
            Self::UnverifiedCli => "unverified-cli",
            Self::NotDelivered => "not-delivered",
            Self::Unknown => "unknown",
        }
    }
}

/// The closed `not-delivered` details of a `send`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NotDelivered {
    InputNotReady,
    NoPromptSubmitted,
    TurnRunning,
    UnknownDialog,
    ControlCharacter,
}

impl NotDelivered {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InputNotReady => "input-not-ready",
            Self::NoPromptSubmitted => "no-prompt-submitted",
            Self::TurnRunning => "turn-running",
            Self::UnknownDialog => "unknown-dialog",
            Self::ControlCharacter => "control-character",
        }
    }
}

/// The closed `human-typing` details besides `null` (a human key took the wheel): the wheel was
/// taken by `viola pause`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HumanTyping {
    ManualPause,
}

impl HumanTyping {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManualPause => "manual-pause",
        }
    }
}

/// Why the wheel holder last changed: the `cause` of a `wheel` event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WheelCause {
    Start,
    HumanInput,
    ManualPause,
    Release,
}

impl WheelCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::HumanInput => "human-input",
            Self::ManualPause => "manual-pause",
            Self::Release => "release",
        }
    }
}

/// Paste text admits LF, CR and TAB; every other C0, DEL and C1 is refused, never stripped
/// (security-plan §Input Validation, "Paste text").
pub fn validate_paste_text(text: &str) -> Result<(), NotDelivered> {
    if text.chars().any(is_refused_control) {
        Err(NotDelivered::ControlCharacter)
    } else {
        Ok(())
    }
}

fn is_refused_control(c: char) -> bool {
    match c {
        '\n' | '\r' | '\t' => false,
        '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}' => true,
        _ => false,
    }
}

fn is_valid_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    let Some(first) = bytes.first() else {
        return false;
    };
    bytes.len() <= 32
        && first.is_ascii_lowercase()
        && bytes
            .iter()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-')
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use proptest::test_runner::FileFailurePersistence;

    fn valid(name: &str) -> bool {
        ViolaName::try_new(name.to_owned()).is_ok()
    }

    /// The literal oracle `^[a-z][a-z0-9-]{0,31}$`, written out independently of the product check.
    fn oracle(name: &str) -> bool {
        let mut chars = name.chars();
        let Some(first) = chars.next() else {
            return false;
        };
        name.chars().count() <= 32
            && first.is_ascii_lowercase()
            && chars.all(|c| matches!(c, 'a'..='z' | '0'..='9' | '-'))
    }

    fn config() -> ProptestConfig {
        ProptestConfig {
            cases: 512,
            failure_persistence: Some(Box::new(FileFailurePersistence::SourceParallel(
                "proptest-regressions",
            ))),
            ..ProptestConfig::default()
        }
    }

    proptest! {
        #![proptest_config(config())]

        #[test]
        fn viola_name_prop_agrees_with_the_oracle(
            name in prop_oneof!["(?s).{0,40}", "[a-zA-Z0-9_./ -]{0,40}"],
        ) {
            prop_assert_eq!(valid(&name), oracle(&name));
        }

        #[test]
        fn viola_name_prop_accepts_every_valid_shape(name in "[a-z][a-z0-9-]{0,31}") {
            prop_assert!(valid(&name));
        }

        #[test]
        fn viola_name_prop_rejects_hostile_neighbours(
            stem in "[a-z][a-z0-9-]{0,20}",
            bad in prop::sample::select(vec!["..", "/", "\\", "A", "_", ".", " ", "\u{0}", "\u{1b}", "\u{85}", "é"]),
        ) {
            let (before, after) = (stem.clone() + bad, bad.to_owned() + &stem);
            prop_assert!(!valid(&before));
            prop_assert!(!valid(&after));
        }

        #[test]
        fn viola_name_prop_rejects_over_32_chars(name in "[a-z][a-z0-9-]{32,40}") {
            prop_assert!(!valid(&name));
        }
    }

    #[test]
    fn viola_name_plain_words_accepted() {
        assert!(valid("builder"));
        assert!(valid("overseer"));
        assert!(valid("a"));
        assert!(valid("b2-x9"));
    }

    #[test]
    fn viola_name_empty_rejected() {
        assert!(!valid(""));
    }

    #[test]
    fn viola_name_length_boundary_holds() {
        assert!(valid(&"a".repeat(32)));
        assert!(!valid(&"a".repeat(33)));
    }

    #[test]
    fn viola_name_leading_non_letter_rejected() {
        assert!(!valid("1abc"));
        assert!(!valid("-abc"));
    }

    #[test]
    fn viola_name_outside_charset_rejected() {
        for bad in ["Builder", "a_b", "a/b", "..", "a.b", "a b", "é", "a\\b"] {
            assert!(!valid(bad), "{bad:?} must be rejected");
        }
    }

    #[test]
    fn viola_name_digits_and_hyphen_after_first_accepted() {
        assert!(valid("a1"));
        assert!(valid("a-"));
    }

    #[test]
    fn viola_name_displays_its_value() {
        let name = ViolaName::try_new("builder".to_owned()).expect("valid");
        assert_eq!(name.to_string(), "builder");
        assert_eq!(name.as_ref(), "builder");
    }

    #[test]
    fn max_frame_is_sixteen_mib() {
        assert_eq!(MAX_FRAME, 16_777_216);
    }

    #[test]
    fn spine_deadline_is_one_second() {
        assert_eq!(SPINE_DEADLINE.as_millis(), 1000);
    }

    #[test]
    fn dialog_deadline_is_sixty_seconds() {
        assert_eq!(DIALOG_DEADLINE.as_secs(), 60);
    }

    #[test]
    fn system_clock_readings_never_go_backwards() {
        let first = SystemClock.now();
        let second = SystemClock.now();
        assert!(second >= first);
    }

    #[test]
    fn event_kind_as_str_is_kebab_case() {
        assert_eq!(EventKind::SessionStart.as_str(), "session-start");
        assert_eq!(EventKind::PromptSubmitted.as_str(), "prompt-submitted");
        assert_eq!(EventKind::TurnEnded.as_str(), "turn-ended");
        assert_eq!(EventKind::SessionEnd.as_str(), "session-end");
        assert_eq!(EventKind::Activity.as_str(), "activity");
        assert_eq!(EventKind::Wheel.as_str(), "wheel");
        assert_eq!(EventKind::BudgetGate.as_str(), "budget-gate");
        assert_eq!(EventKind::SendIssued.as_str(), "send-issued");
        assert_eq!(EventKind::SendConfirmed.as_str(), "send-confirmed");
        assert_eq!(EventKind::SendRefused.as_str(), "send-refused");
        assert_eq!(EventKind::Question.as_str(), "question");
        assert_eq!(EventKind::Permission.as_str(), "permission");
        assert_eq!(EventKind::Plan.as_str(), "plan");
    }

    #[test]
    fn event_kind_wait_wake_is_the_five_driver_relevant_kinds() {
        let wake: Vec<&str> = EventKind::WAIT_WAKE.iter().map(|k| k.as_str()).collect();
        assert_eq!(
            wake,
            [
                "turn-ended",
                "question",
                "permission",
                "plan",
                "session-end"
            ]
        );
    }

    #[test]
    fn refusal_reason_round_trips_kebab_case() {
        let table = [
            (RefusalReason::HumanTyping, "human-typing"),
            (RefusalReason::BudgetPaused, "budget-paused"),
            (RefusalReason::UnverifiedCli, "unverified-cli"),
            (RefusalReason::NotDelivered, "not-delivered"),
            (RefusalReason::Unknown, "unknown"),
        ];
        for (reason, text) in table {
            assert_eq!(reason.as_str(), text);
            let json = serde_json::to_string(&reason).expect("serialize");
            assert_eq!(json, format!("\"{text}\""));
            let back: RefusalReason = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, reason);
        }
    }

    #[test]
    fn refusal_reason_unlisted_string_reads_unknown() {
        let read: RefusalReason = serde_json::from_str("\"wheel-held\"").expect("deserialize");
        assert_eq!(read, RefusalReason::Unknown);
    }

    #[test]
    fn refusal_reason_not_delivered_details_round_trip_kebab_case() {
        let table = [
            (NotDelivered::InputNotReady, "input-not-ready"),
            (NotDelivered::NoPromptSubmitted, "no-prompt-submitted"),
            (NotDelivered::TurnRunning, "turn-running"),
            (NotDelivered::UnknownDialog, "unknown-dialog"),
            (NotDelivered::ControlCharacter, "control-character"),
        ];
        for (detail, text) in table {
            assert_eq!(detail.as_str(), text);
            let json = serde_json::to_string(&detail).expect("serialize");
            assert_eq!(json, format!("\"{text}\""));
            let back: NotDelivered = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, detail);
        }
    }

    #[test]
    fn human_typing_details_round_trip_kebab_case() {
        let detail = HumanTyping::ManualPause;
        assert_eq!(detail.as_str(), "manual-pause");
        let json = serde_json::to_string(&detail).expect("serialize");
        assert_eq!(json, "\"manual-pause\"");
        let back: HumanTyping = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, detail);
        assert!(serde_json::from_str::<HumanTyping>("\"human-input\"").is_err());
    }

    #[test]
    fn wheel_causes_round_trip_kebab_case() {
        let table = [
            (WheelCause::Start, "start"),
            (WheelCause::HumanInput, "human-input"),
            (WheelCause::ManualPause, "manual-pause"),
            (WheelCause::Release, "release"),
        ];
        for (cause, text) in table {
            assert_eq!(cause.as_str(), text);
            let json = serde_json::to_string(&cause).expect("serialize");
            assert_eq!(json, format!("\"{text}\""));
            let back: WheelCause = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, cause);
        }
        assert!(serde_json::from_str::<WheelCause>("\"human-key\"").is_err());
    }

    #[test]
    fn paste_text_literal_table() {
        for ok in ["\n", "\r", "\t", "é", "中", "🙂", "line one\nline two", ""] {
            assert_eq!(validate_paste_text(ok), Ok(()), "{ok:?} must pass");
        }
        for bad in [
            "\u{1b}",
            "\u{0}",
            "\u{7}",
            "\u{7f}",
            "\u{85}",
            "\u{9f}",
            "ok\u{1b}[201~",
        ] {
            assert_eq!(
                validate_paste_text(bad),
                Err(NotDelivered::ControlCharacter),
                "{bad:?} must be refused"
            );
        }
    }

    #[test]
    fn paste_text_boundaries_of_each_class() {
        assert!(validate_paste_text("\u{1f}").is_err());
        assert!(validate_paste_text(" ").is_ok());
        assert!(validate_paste_text("~").is_ok());
        assert!(validate_paste_text("\u{80}").is_err());
        assert!(validate_paste_text("\u{a0}").is_ok());
    }

    fn refused_char() -> impl Strategy<Value = char> {
        prop_oneof![
            (0u32..=0x1f)
                .prop_filter("allowed C0", |c| ![0x09, 0x0a, 0x0d].contains(c))
                .prop_map(|c| char::from_u32(c).expect("C0")),
            Just('\u{7f}'),
            (0x80u32..=0x9f).prop_map(|c| char::from_u32(c).expect("C1")),
        ]
    }

    fn allowed_char() -> impl Strategy<Value = char> {
        any::<char>().prop_filter("refused control", |c| !is_refused_control(*c))
    }

    proptest! {
        #![proptest_config(config())]

        #[test]
        fn paste_text_prop_refuses_any_injected_control(
            text in any::<String>(),
            bad in refused_char(),
            at in any::<prop::sample::Index>(),
        ) {
            let mut chars: Vec<char> = text.chars().collect();
            let i = at.index(chars.len() + 1);
            chars.insert(i, bad);
            let injected: String = chars.into_iter().collect();
            prop_assert_eq!(validate_paste_text(&injected), Err(NotDelivered::ControlCharacter));
        }

        #[test]
        fn paste_text_prop_passes_only_allowed_chars(
            chars in prop::collection::vec(allowed_char(), 0..64),
        ) {
            let text: String = chars.into_iter().collect();
            prop_assert_eq!(validate_paste_text(&text), Ok(()));
        }
    }

    #[test]
    fn version_is_the_package_version() {
        assert_eq!(VERSION, "0.1.0");
        assert_eq!(SERVICE_NAME, "viola");
    }
}
