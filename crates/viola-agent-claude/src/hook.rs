//! What a `claude` hook hands `viola hook` on stdin, read tolerantly and mapped to a normalised
//! event (architecture §Conventions Hook → kind map; §Standard Contracts Event `data` per kind).
//! Upstream text is content: it is copied into `data` field by field, never interpreted.

use serde::Deserialize;
use serde_json::{Value, json};
use viola_core::EventKind;

use crate::AgentError;

/// The hook events this build registers, each by its `viola hook <event>` argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookEvent {
    SessionStart,
    UserPromptSubmit,
    Stop,
    SessionEnd,
    Notification,
    PostToolUse,
    PostToolUseFailure,
}

impl HookEvent {
    pub const ALL: [Self; 7] = [
        Self::SessionStart,
        Self::UserPromptSubmit,
        Self::Stop,
        Self::SessionEnd,
        Self::Notification,
        Self::PostToolUse,
        Self::PostToolUseFailure,
    ];

    /// The kebab-case argument, equal to diag-line `$defs.hook_event`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionStart => "session-start",
            Self::UserPromptSubmit => "user-prompt-submit",
            Self::Stop => "stop",
            Self::SessionEnd => "session-end",
            Self::Notification => "notification",
            Self::PostToolUse => "post-tool-use",
            Self::PostToolUseFailure => "post-tool-use-failure",
        }
    }

    pub fn from_arg(arg: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|event| event.as_str() == arg)
    }

    pub const fn kind(self) -> EventKind {
        match self {
            Self::SessionStart => EventKind::SessionStart,
            Self::UserPromptSubmit => EventKind::PromptSubmitted,
            Self::Stop => EventKind::TurnEnded,
            Self::SessionEnd => EventKind::SessionEnd,
            Self::Notification | Self::PostToolUse | Self::PostToolUseFailure => {
                EventKind::Activity
            }
        }
    }
}

/// A hook payload mapped to its event: the kind, its `data`, and one drift entry per known field
/// that arrived with the wrong type (read as absent).
#[derive(Debug, Clone, PartialEq)]
pub struct Normalised {
    pub kind: EventKind,
    pub data: Value,
    pub drift: Vec<Value>,
}

/// The payload fields viola reads; every other field is skipped. All are strings on the CLI build
/// the ledger names, so a drift entry's `expected` is always `string`.
#[derive(Debug, Default, Deserialize)]
struct Known {
    #[serde(default)]
    session_id: Option<String>,
    #[serde(default)]
    source: Option<String>,
    #[serde(default)]
    prompt: Option<String>,
    #[serde(default)]
    last_assistant_message: Option<String>,
    #[serde(default)]
    tool_name: Option<String>,
}

const SESSION_CAUSES: [&str; 4] = ["startup", "clear", "resume", "compact"];
const HARNESS_PREFIXES: [&str; 4] = [
    "<agent-message from=",
    "<task-notification>",
    "<\\cross-session-message",
    "<cross-session-message",
];

/// Reads `bytes` as `event`'s payload. Only a payload that is not one JSON object is refused.
pub fn normalise(event: HookEvent, bytes: &[u8]) -> Result<Normalised, AgentError> {
    let mut payload: Value = serde_json::from_slice(bytes).map_err(|_| AgentError::Malformed)?;
    if !payload.is_object() {
        return Err(AgentError::Malformed);
    }
    let (known, drift) = known_fields(&mut payload);
    Ok(Normalised {
        kind: event.kind(),
        data: data_of(event, known),
        drift,
    })
}

/// The typed view of the payload object. A known field of the wrong type is recorded by its path
/// and the type expected, never its value, and removed so the rest still reads.
fn known_fields(payload: &mut Value) -> (Known, Vec<Value>) {
    let mut drift = Vec::new();
    loop {
        let error = match serde_path_to_error::deserialize::<_, Known>(&*payload) {
            Ok(known) => return (known, drift),
            Err(error) => error,
        };
        let path = error.path().to_string();
        let removed = payload.as_object_mut().and_then(|o| o.remove(&path));
        if removed.is_none() {
            return (Known::default(), drift);
        }
        drift.push(json!({"path": path, "expected": "string"}));
    }
}

fn data_of(event: HookEvent, known: Known) -> Value {
    match event {
        HookEvent::SessionStart => {
            let cause = known
                .source
                .as_deref()
                .filter(|s| SESSION_CAUSES.contains(s))
                .unwrap_or("unknown");
            json!({"cause": cause, "agent_session_id": known.session_id})
        }
        HookEvent::UserPromptSubmit => {
            let raw = known.prompt.unwrap_or_default();
            json!({"text": prompt_text(&raw), "origin": prompt_origin(&raw)})
        }
        HookEvent::Stop => json!({"last_assistant_message": known.last_assistant_message}),
        HookEvent::SessionEnd => json!({}),
        HookEvent::Notification | HookEvent::PostToolUse | HookEvent::PostToolUseFailure => {
            match known.tool_name {
                Some(tool) => json!({"tool": tool}),
                None => json!({}),
            }
        }
    }
}

/// `harness` when the prompt as the CLI sent it starts with a harness prefix, matched on the raw
/// start with no trim. A tag the user typed arrives escaped (`<\task-notification>`), so it never
/// classifies — with one exception: the cross-session message is itself injected escaped
/// (`<\cross-session-message`, measured F115 on andromeda-worker, founder-ratified 2026-09-29), so
/// a human who types that tag at a prompt's start is filed harness too.
fn prompt_origin(raw: &str) -> &'static str {
    if HARNESS_PREFIXES.iter().any(|p| raw.starts_with(p)) {
        "harness"
    } else {
        "human"
    }
}

/// The CLI's long-paste pairs unwrapped, then the typed tag escaping reversed: in that order, a
/// pair the user typed (escaped) is never taken for the CLI's own (architecture.md
/// [CLI Version Compatibility], the paste-wrapper and tag-escaping rows).
fn prompt_text(raw: &str) -> String {
    unescape_tags(&unwrap_pastes(raw))
}

const PASTE_OPEN: &str = "<pasted_content id=\"";

/// Each `<pasted_content id="X">\n…\n</pasted_content id="X">` becomes its inner text; everything
/// outside a pair, and an open tag with no matching close, is kept byte for byte.
fn unwrap_pastes(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find(PASTE_OPEN) {
        let after = &rest[start + PASTE_OPEN.len()..];
        match paste_pair(after) {
            Some((inner, tail)) => {
                out.push_str(&rest[..start]);
                out.push_str(inner);
                rest = tail;
            }
            None => {
                out.push_str(&rest[..start + PASTE_OPEN.len()]);
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

/// `after` follows an open tag's `id="`: the inner text and what follows the matching close.
fn paste_pair(after: &str) -> Option<(&str, &str)> {
    let (id, rest) = after.split_at(after.find('"')?);
    let body = rest.strip_prefix("\">\n")?;
    let close = format!("\n</pasted_content id=\"{id}\">");
    let end = body.find(&close)?;
    Some((&body[..end], &body[end + close.len()..]))
}

/// `<\` before an ASCII letter or `/` loses its backslash: the CLI's escaping of typed tag text.
fn unescape_tags(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find("<\\") {
        out.push_str(&rest[..=at]);
        let after = &rest[at + 2..];
        if !after.starts_with(|c: char| c.is_ascii_alphabetic() || c == '/') {
            out.push('\\');
        }
        rest = after;
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use proptest::test_runner::FileFailurePersistence;
    use rstest::rstest;
    use serde_json::Map;

    fn read(event: HookEvent, payload: &Value) -> Normalised {
        normalise(event, payload.to_string().as_bytes()).expect("an object")
    }

    #[test]
    fn hook_event_args_are_the_diag_line_names() {
        let names: Vec<&str> = HookEvent::ALL.iter().map(|e| e.as_str()).collect();
        assert_eq!(
            names,
            [
                "session-start",
                "user-prompt-submit",
                "stop",
                "session-end",
                "notification",
                "post-tool-use",
                "post-tool-use-failure",
            ]
        );
        for event in HookEvent::ALL {
            assert_eq!(HookEvent::from_arg(event.as_str()), Some(event));
        }
        for other in [
            "pre-tool-use",
            "permission-request",
            "statusline",
            "",
            "Stop",
        ] {
            assert_eq!(HookEvent::from_arg(other), None, "{other}");
        }
    }

    #[test]
    fn hook_event_kinds_follow_the_map() {
        let kinds: Vec<&str> = HookEvent::ALL.iter().map(|e| e.kind().as_str()).collect();
        assert_eq!(
            kinds,
            [
                "session-start",
                "prompt-submitted",
                "turn-ended",
                "session-end",
                "activity",
                "activity",
                "activity",
            ]
        );
    }

    #[rstest]
    #[case::not_json(b"not json".as_slice())]
    #[case::array(b"[1,2]".as_slice())]
    #[case::string(b"\"x\"".as_slice())]
    #[case::empty(b"".as_slice())]
    fn normalise_refuses_anything_but_one_object(#[case] bytes: &[u8]) {
        assert_eq!(
            normalise(HookEvent::Stop, bytes),
            Err(AgentError::Malformed)
        );
    }

    #[test]
    fn agent_error_message_is_fixed() {
        assert_eq!(
            AgentError::Malformed.to_string(),
            "the hook payload is not one JSON object"
        );
    }

    #[rstest]
    #[case::startup(json!("startup"), "startup")]
    #[case::clear(json!("clear"), "clear")]
    #[case::resume(json!("resume"), "resume")]
    #[case::compact(json!("compact"), "compact")]
    #[case::other(json!("reboot"), "unknown")]
    #[case::absent(Value::Null, "unknown")]
    fn session_start_cause_is_a_known_word_or_unknown(#[case] source: Value, #[case] cause: &str) {
        let got = read(
            HookEvent::SessionStart,
            &json!({"source": source, "session_id": "s-1", "transcript_path": "/x"}),
        );
        assert_eq!(got.kind, EventKind::SessionStart);
        assert_eq!(got.data, json!({"cause": cause, "agent_session_id": "s-1"}));
        assert!(got.drift.is_empty());
    }

    #[test]
    fn session_start_without_a_session_id_carries_null() {
        let got = read(HookEvent::SessionStart, &json!({}));
        assert_eq!(
            got.data,
            json!({"cause": "unknown", "agent_session_id": null})
        );
    }

    /// A known field of the wrong type is read as absent and reported by path and expected type,
    /// never its value; the other known fields still read.
    #[test]
    fn a_wrong_typed_known_field_drifts_and_reads_as_absent() {
        let got = read(
            HookEvent::SessionStart,
            &json!({"session_id": 7, "source": ["canary-chain-value-5c1e"], "extra": true}),
        );
        assert_eq!(
            got.data,
            json!({"cause": "unknown", "agent_session_id": null})
        );
        assert_eq!(
            got.drift,
            [
                json!({"path": "session_id", "expected": "string"}),
                json!({"path": "source", "expected": "string"}),
            ]
        );
        let got = read(
            HookEvent::Stop,
            &json!({"last_assistant_message": {"n": 1}, "session_id": "kept"}),
        );
        assert_eq!(got.data, json!({"last_assistant_message": null}));
        assert_eq!(
            got.drift,
            [json!({"path": "last_assistant_message", "expected": "string"})]
        );
        let got = read(
            HookEvent::SessionStart,
            &json!({"source": "clear", "session_id": 1}),
        );
        assert_eq!(
            got.data,
            json!({"cause": "clear", "agent_session_id": null})
        );
    }

    #[test]
    fn stop_carries_the_last_message_or_null() {
        let got = read(HookEvent::Stop, &json!({"last_assistant_message": "done"}));
        assert_eq!(got.kind, EventKind::TurnEnded);
        assert_eq!(got.data, json!({"last_assistant_message": "done"}));
        let got = read(HookEvent::Stop, &json!({"last_assistant_message": null}));
        assert_eq!(got.data, json!({"last_assistant_message": null}));
    }

    #[test]
    fn session_end_data_is_empty() {
        let got = read(
            HookEvent::SessionEnd,
            &json!({"reason": "exit", "session_id": "s"}),
        );
        assert_eq!(got.kind, EventKind::SessionEnd);
        assert_eq!(got.data, json!({}));
    }

    #[rstest]
    #[case::notification(HookEvent::Notification)]
    #[case::post_tool_use(HookEvent::PostToolUse)]
    #[case::post_tool_use_failure(HookEvent::PostToolUseFailure)]
    fn activity_names_the_tool_when_there_is_one(#[case] event: HookEvent) {
        let got = read(
            event,
            &json!({"tool_name": "Bash", "tool_input": {"command": "ls"}}),
        );
        assert_eq!(got.kind, EventKind::Activity);
        assert_eq!(got.data, json!({"tool": "Bash"}));
        assert_eq!(read(event, &json!({"message": "m"})).data, json!({}));
    }

    #[rstest]
    #[case::task_notification("<task-notification>done</task-notification>", "harness")]
    #[case::agent_message("<agent-message from=\"a\">hi</agent-message>", "harness")]
    #[case::typed_prefix("<\\task-notification>x", "human")]
    #[case::inner_prefix("see <task-notification>", "human")]
    #[case::plain("hello", "human")]
    fn prompt_origin_reads_the_raw_prefix(#[case] prompt: &str, #[case] origin: &str) {
        let got = read(HookEvent::UserPromptSubmit, &json!({"prompt": prompt}));
        assert_eq!(got.kind, EventKind::PromptSubmitted);
        assert_eq!(got.data["origin"], origin);
    }

    #[rstest]
    #[case::cross_escaped(
        "<\\cross-session-message from=\"uds:x\" from-name=\"overseer1\">\nhi",
        "harness",
        "<cross-session-message from=\"uds:x\" from-name=\"overseer1\">\nhi"
    )]
    #[case::cross_plain(
        "<cross-session-message from=\"uds:x\">\nhi",
        "harness",
        "<cross-session-message from=\"uds:x\">\nhi"
    )]
    #[case::cross_escaped_inner(
        "typed <\\cross-session-message later",
        "human",
        "typed <cross-session-message later"
    )]
    #[case::cross_plain_inner("see <cross-session-message", "human", "see <cross-session-message")]
    #[case::leading_space(" <cross-session-message", "human", " <cross-session-message")]
    fn prompt_origin_files_the_cross_session_tag_as_harness(
        #[case] prompt: &str,
        #[case] origin: &str,
        #[case] text: &str,
    ) {
        let got = read(HookEvent::UserPromptSubmit, &json!({"prompt": prompt}));
        assert_eq!(got.data, json!({"text": text, "origin": origin}));
    }

    #[test]
    fn prompt_submitted_without_a_prompt_is_empty_human_text() {
        let got = read(HookEvent::UserPromptSubmit, &json!({"session_id": "s"}));
        assert_eq!(got.data, json!({"text": "", "origin": "human"}));
    }

    #[rstest]
    #[case::cli_pair(
        "\n\n<pasted_content id=\"2f85\">\nA paste\n</pasted_content id=\"2f85\">\n",
        "\n\nA paste\n"
    )]
    #[case::two_pairs(
        "a<pasted_content id=\"1\">\nx\n</pasted_content id=\"1\">b<pasted_content id=\"2\">\ny\n</pasted_content id=\"2\">c",
        "axbyc"
    )]
    #[case::typed_pair(
        "<\\pasted_content id=\"1\">x<\\/pasted_content id=\"1\">",
        "<pasted_content id=\"1\">x</pasted_content id=\"1\">"
    )]
    #[case::mismatched_id(
        "<pasted_content id=\"1\">\nx\n</pasted_content id=\"2\">",
        "<pasted_content id=\"1\">\nx\n</pasted_content id=\"2\">"
    )]
    #[case::open_without_newline(
        "<pasted_content id=\"1\">x\n</pasted_content id=\"1\">",
        "<pasted_content id=\"1\">x\n</pasted_content id=\"1\">"
    )]
    #[case::unclosed_id("<pasted_content id=\"1", "<pasted_content id=\"1")]
    #[case::bare_close(
        "<pasted_content id=\"1\">\nx\n</pasted_content>",
        "<pasted_content id=\"1\">\nx\n</pasted_content>"
    )]
    #[case::unmatched_then_pair(
        "<pasted_content id=\"9\">z<pasted_content id=\"1\">\nx\n</pasted_content id=\"1\">",
        "<pasted_content id=\"9\">zx"
    )]
    #[case::typed_task("<\\task-notification>", "<task-notification>")]
    #[case::backslash_kept("<\\1 and <\\\\ and <\\", "<\\1 and <\\\\ and <\\")]
    #[case::plain("no tags \u{e9}", "no tags \u{e9}")]
    fn prompt_text_unwraps_the_cli_pair_then_unescapes(#[case] raw: &str, #[case] text: &str) {
        assert_eq!(prompt_text(raw), text);
        let got = read(HookEvent::UserPromptSubmit, &json!({"prompt": raw}));
        assert_eq!(got.data["text"], text);
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

    /// The CLI's typed-text escaping, written out as the test's own oracle: `<` before an ASCII
    /// letter becomes `<\`, and `</` becomes `<\/`.
    fn cli_escape(text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        let mut chars = text.chars().peekable();
        while let Some(c) = chars.next() {
            out.push(c);
            if c == '<'
                && chars
                    .peek()
                    .is_some_and(|n| n.is_ascii_alphabetic() || *n == '/')
            {
                out.push('\\');
            }
        }
        out
    }

    /// Text built from the pieces that matter to the normaliser: tag text, newlines, prefixes.
    fn typed_text() -> impl Strategy<Value = String> {
        let piece = prop_oneof![
            "[a-z0-9 \u{e9}]{0,6}",
            Just("\n".to_owned()),
            Just("<".to_owned()),
            Just("/".to_owned()),
            "<pasted_content id=\"[a-z0-9]{1,4}\">",
            "</pasted_content id=\"[a-z0-9]{1,4}\">",
            Just("<task-notification>".to_owned()),
        ];
        prop::collection::vec(piece, 0..12).prop_map(|pieces| pieces.concat())
    }

    fn one_of_the_events() -> impl Strategy<Value = HookEvent> {
        prop::sample::select(HookEvent::ALL.to_vec())
    }

    proptest! {
        #![proptest_config(config())]

        #[test]
        fn hook_stdin_prop_arbitrary_bytes_never_panic(
            event in one_of_the_events(),
            bytes in prop::collection::vec(any::<u8>(), 0..256),
        ) {
            let _ = normalise(event, &bytes);
        }

        #[test]
        fn hook_stdin_prop_arbitrary_objects_never_panic(
            event in one_of_the_events(),
            keys in prop::collection::vec(
                prop::sample::select(vec![
                    "session_id", "source", "prompt", "last_assistant_message", "tool_name", "x",
                ]),
                0..6,
            ),
            values in prop::collection::vec(
                prop_oneof![
                    Just(Value::Null),
                    any::<bool>().prop_map(Value::from),
                    any::<i64>().prop_map(Value::from),
                    ".{0,12}".prop_map(Value::from),
                    Just(json!([1, "a"])),
                    Just(json!({"k": "v"})),
                ],
                0..6,
            ),
        ) {
            let object: Map<String, Value> = keys
                .into_iter()
                .map(str::to_owned)
                .zip(values)
                .collect();
            let wrong: usize = object
                .iter()
                .filter(|(k, v)| k.as_str() != "x" && !v.is_string() && !v.is_null())
                .count();
            let got = normalise(event, Value::Object(object).to_string().as_bytes());
            let got = got.expect("an object always reads");
            prop_assert_eq!(got.drift.len(), wrong);
            prop_assert!(got.data.is_object());
        }

        #[test]
        fn hook_stdin_prop_well_typed_fields_survive(
            session in "[a-z0-9-]{0,12}",
            message in ".{0,24}",
            tool in "[A-Za-z]{1,8}",
            prompt in "[^<]{0,24}",
        ) {
            let payload = json!({
                "session_id": session, "source": "resume", "last_assistant_message": message,
                "tool_name": tool, "prompt": prompt,
            });
            let bytes = payload.to_string();
            let start = normalise(HookEvent::SessionStart, bytes.as_bytes()).expect("object");
            prop_assert_eq!(&start.data["agent_session_id"], &json!(session));
            prop_assert_eq!(&start.data["cause"], "resume");
            let stop = normalise(HookEvent::Stop, bytes.as_bytes()).expect("object");
            prop_assert_eq!(&stop.data["last_assistant_message"], &json!(message));
            let activity = normalise(HookEvent::PostToolUse, bytes.as_bytes()).expect("object");
            prop_assert_eq!(&activity.data["tool"], &json!(tool));
            let submitted = normalise(HookEvent::UserPromptSubmit, bytes.as_bytes()).expect("object");
            prop_assert_eq!(&submitted.data["text"], &json!(prompt));
            prop_assert!(start.drift.is_empty() && submitted.drift.is_empty());
        }

        /// Escape the typed text as the CLI does, wrap it in the CLI's pair, normalise: the typed
        /// text comes back, typed pairs included.
        #[test]
        fn prompt_text_prop_round_trips_a_wrapped_paste(
            text in typed_text(),
            id in "[a-zA-Z0-9]{1,8}",
            lead in prop::sample::select(vec!["", "\n\n", "note: "]),
        ) {
            let open = String::from("<pasted_content id=\"") + &id + "\">\n";
            let close = String::from("\n</pasted_content id=\"") + &id + "\">";
            let wire = String::from(lead) + &open + &cli_escape(&text) + &close + "\n";
            let expected = String::from(lead) + &text + "\n";
            prop_assert_eq!(prompt_text(&wire), expected);
        }
    }
}
