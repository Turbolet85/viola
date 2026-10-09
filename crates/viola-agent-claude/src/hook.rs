//! What a `claude` hook hands `viola hook` on stdin, read tolerantly and mapped to a normalised
//! event (architecture §Conventions Hook → kind map; §Standard Contracts Event `data` per kind).
//! Upstream text is content: it is copied into `data` field by field, never interpreted.

use std::borrow::Cow;

use serde::Deserialize;
use serde_json::{Value, json};
use viola_core::EventKind;

use crate::AgentError;

/// The hook events this build registers, each by its `viola hook <event>` argument. The two
/// dialog-tier events raise a dialog (`crate::dialog`), never a `hook.event`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookEvent {
    SessionStart,
    UserPromptSubmit,
    Stop,
    SessionEnd,
    Notification,
    PostToolUse,
    PostToolUseFailure,
    PreToolUse,
    PermissionRequest,
}

impl HookEvent {
    pub const ALL: [Self; 9] = [
        Self::SessionStart,
        Self::UserPromptSubmit,
        Self::Stop,
        Self::SessionEnd,
        Self::Notification,
        Self::PostToolUse,
        Self::PostToolUseFailure,
        Self::PreToolUse,
        Self::PermissionRequest,
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
            Self::PreToolUse => "pre-tool-use",
            Self::PermissionRequest => "permission-request",
        }
    }

    /// The dialog tier: PreToolUse and PermissionRequest, sync, answered by a decision body.
    pub const fn is_dialog(self) -> bool {
        matches!(self, Self::PreToolUse | Self::PermissionRequest)
    }

    pub fn from_arg(arg: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|event| event.as_str() == arg)
    }

    /// The event kind a `hook.event` carries; `None` for the dialog tier, whose kind is the
    /// dialog's (`crate::dialog::classify`).
    pub const fn kind(self) -> Option<EventKind> {
        match self {
            Self::SessionStart => Some(EventKind::SessionStart),
            Self::UserPromptSubmit => Some(EventKind::PromptSubmitted),
            Self::Stop => Some(EventKind::TurnEnded),
            Self::SessionEnd => Some(EventKind::SessionEnd),
            Self::Notification | Self::PostToolUse | Self::PostToolUseFailure => {
                Some(EventKind::Activity)
            }
            Self::PreToolUse | Self::PermissionRequest => None,
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

/// Reads `bytes` as `event`'s payload. A payload that is not one JSON object is refused, and so is
/// a dialog-tier event, which never becomes a `hook.event`.
pub fn normalise(event: HookEvent, bytes: &[u8]) -> Result<Normalised, AgentError> {
    let kind = event.kind().ok_or(AgentError::NotAnEvent)?;
    let mut payload: Value = serde_json::from_slice(bytes).map_err(|_| AgentError::Malformed)?;
    if !payload.is_object() {
        return Err(AgentError::Malformed);
    }
    let (known, drift) = known_fields(&mut payload);
    Ok(Normalised {
        kind,
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
        HookEvent::SessionEnd | HookEvent::PreToolUse | HookEvent::PermissionRequest => json!({}),
        HookEvent::Notification | HookEvent::PostToolUse | HookEvent::PostToolUseFailure => {
            match known.tool_name {
                Some(tool) => json!({"tool": tool}),
                None => json!({}),
            }
        }
    }
}

/// `harness` when the prompt as the CLI sent it starts with a harness prefix, matched on the raw
/// start with no trim. Measured on 2.1.287: the CLI escapes a typed `pasted_content` tag
/// (`<\pasted_content`), and a typed `<task-notification>` arrives as typed, in the middle of a
/// prompt and at its very start, so a human who types it first is filed harness. A cross-session
/// message arrived unescaped there (`<cross-session-message from=… from-name=… from-mode=…>`); the
/// escaped form (`<\cross-session-message`, measured F115 on andromeda-worker, founder-ratified
/// 2026-09-29) stays a prefix, so a human who types that tag at a prompt's start is filed harness
/// too.
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
pub(crate) fn prompt_text(raw: &str) -> String {
    unescape_tags(&unwrap_pastes(raw))
}

/// A sent text as `send` types it: every CR LF pair as one LF, every other CR as one LF, and
/// without its trailing CR and LF characters, every one of them in any order. It holds no CR, and
/// nothing else changes (no TAB or space goes, and no LF that is not at the very end).
///
/// The ending. Measured on 2.1.287: the CLI drops a pasted text's newline ending before
/// UserPromptSubmit, so a text typed with it comes back short of the text `send` matches. One LF,
/// on a verified and an unverified home (chunk
/// 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host: `evidence/hint-window.md` step 7,
/// `scratch-session.md`, `live-shape-red-green.md`); one CR (chunk
/// 2026-10-08-first-live-test-and-self-drive: `evidence/live-readings.ndjson`, `trailing-cr`); one
/// CRLF and two CRs (chunk 2026-10-09-epoch-3-cleanup: `evidence/live-readings.ndjson`,
/// `trailing-crlf` and `trailing-cr-cr`). The width is every trailing CR and LF, so the typed text
/// never ends in one and what the CLI does with a longer run is not leaned on.
///
/// Inside the text. Measured on 2.1.287 before the rule (chunk 2026-10-09-epoch-3-cleanup:
/// `evidence/live-readings.ndjson`, `after-inner-crlf` and `after-inner-cr`): the CLI submits a
/// pasted text's inner CR LF as one LF and its lone inner CR as one LF, so a text typed with the
/// CR comes back unequal to the text `send` matches. The typed text carries that LF itself: no
/// `send` types a CR, and none leans on what the CLI does with one. Confirmed with the rule (chunk
/// 2026-10-09-inner-cr-and-crlf-in-a-sent-text, the same file: `rule-inner-crlf`, `rule-inner-cr`,
/// `rule-inner-cr-cr`, `rule-inner-lf-cr`, `rule-crlf-lines`, and `rule-inner-lf` for a typed LF).
/// The founder's rulings, live, the options shown each time: 2026-10-07T15:21Z for a trailing LF,
/// 2026-10-09 for a trailing CR, 2026-10-09T16:51Z, relayed, for a CR inside (architecture.md
/// [Delivery Confirmation]).
pub fn typed_text(text: &str) -> Cow<'_, str> {
    let text = text.trim_end_matches(['\r', '\n']);
    if text.contains('\r') {
        Cow::Owned(text.replace("\r\n", "\n").replace('\r', "\n"))
    } else {
        Cow::Borrowed(text)
    }
}

const PASTE_OPEN: &str = "<pasted_content id=\"";
/// What follows a close tag's own newline when the next pair comes at once.
const NEXT_FRAME: &str = "\n\n<pasted_content id=\"";

/// Each `<pasted_content id="X">\n…\n</pasted_content id="X">` becomes its inner text, and the
/// frame the CLI writes around its own pair goes with it: the two newlines directly before the
/// open tag and the one directly after the close, and a second after the close when text follows
/// that is not the next pair's own frame (measured on 2.1.287: typed text after a paste). A third
/// newline before, a second after at the prompt's end and a lone one before stay, and so does
/// everything else outside a pair; an open tag with no matching close removes nothing
/// (architecture.md [CLI Version Compatibility], the long-paste wrapper).
fn unwrap_pastes(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find(PASTE_OPEN) {
        let after = &rest[start + PASTE_OPEN.len()..];
        match paste_pair(after) {
            Some((inner, tail)) => {
                let lead = &rest[..start];
                out.push_str(lead.strip_suffix("\n\n").unwrap_or(lead));
                out.push_str(inner);
                let tail = tail.strip_prefix('\n').unwrap_or(tail);
                rest = match tail.strip_prefix('\n') {
                    Some(text) if !text.is_empty() && !tail.starts_with(NEXT_FRAME) => text,
                    _ => tail,
                };
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
                "pre-tool-use",
                "permission-request",
            ]
        );
        for event in HookEvent::ALL {
            assert_eq!(HookEvent::from_arg(event.as_str()), Some(event));
        }
        for other in ["statusline", "", "Stop", "PreToolUse"] {
            assert_eq!(HookEvent::from_arg(other), None, "{other}");
        }
    }

    #[test]
    fn hook_event_kinds_follow_the_map() {
        let kinds: Vec<Option<&str>> = HookEvent::ALL
            .iter()
            .map(|e| e.kind().map(EventKind::as_str))
            .collect();
        assert_eq!(
            kinds,
            [
                Some("session-start"),
                Some("prompt-submitted"),
                Some("turn-ended"),
                Some("session-end"),
                Some("activity"),
                Some("activity"),
                Some("activity"),
                None,
                None,
            ]
        );
        let dialog: Vec<bool> = HookEvent::ALL.iter().map(|e| e.is_dialog()).collect();
        assert_eq!(
            dialog,
            [false, false, false, false, false, false, false, true, true]
        );
    }

    #[rstest]
    #[case::pre_tool_use(HookEvent::PreToolUse)]
    #[case::permission_request(HookEvent::PermissionRequest)]
    fn normalise_refuses_a_dialog_event(#[case] event: HookEvent) {
        assert_eq!(
            normalise(event, br#"{"tool_name": "Bash"}"#),
            Err(AgentError::NotAnEvent)
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
        assert_eq!(
            AgentError::NotAnEvent.to_string(),
            "a dialog hook payload is not a hook event"
        );
        assert_eq!(
            AgentError::DialogMalformed.to_string(),
            "the dialog hook payload lacks a field it needs"
        );
        assert_eq!(
            AgentError::ResponseMalformed.to_string(),
            "the dialog answer has the wrong shape"
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
        "A paste"
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

    /// The frame the CLI writes around a long paste, as measured on 2.1.287: two newlines before
    /// the open tag and one after the close. Only that much goes, and only around a matched pair.
    #[rstest]
    #[case::measured_long(
        "\n\n<pasted_content id=\"7ccf\">\nA long paste\n</pasted_content id=\"7ccf\">\n",
        "A long paste"
    )]
    #[case::measured_tag(
        "viola verify probe: the next part is literal sample text and not markup: <\\pasted_content id=\"1\"> sample <\\/pasted_content id=\"1\"> then <task-notification> and that is all. Reply with the single word ok",
        "viola verify probe: the next part is literal sample text and not markup: <pasted_content id=\"1\"> sample </pasted_content id=\"1\"> then <task-notification> and that is all. Reply with the single word ok"
    )]
    #[case::third_newline_before_stays(
        "\n\n\n<pasted_content id=\"7ccf\">\nA long paste\n</pasted_content id=\"7ccf\">\n",
        "\nA long paste"
    )]
    #[case::second_newline_after_stays(
        "\n\n<pasted_content id=\"7ccf\">\nA long paste\n</pasted_content id=\"7ccf\">\n\n",
        "A long paste\n"
    )]
    #[case::one_newline_before_stays(
        "\n<pasted_content id=\"7ccf\">\nA long paste\n</pasted_content id=\"7ccf\">\n",
        "\nA long paste"
    )]
    #[case::text_then_framed_pair(
        "note:\n\n<pasted_content id=\"7ccf\">\nA long paste\n</pasted_content id=\"7ccf\">\n",
        "note:A long paste"
    )]
    #[case::two_framed_pairs(
        "\n\n<pasted_content id=\"eec9\">\none\n</pasted_content id=\"eec9\">\n\n\n<pasted_content id=\"eec9\">\ntwo\n</pasted_content id=\"eec9\">\n",
        "onetwo"
    )]
    #[case::unmatched_open_keeps_its_newlines(
        "\n\n<pasted_content id=\"7ccf\">\n",
        "\n\n<pasted_content id=\"7ccf\">\n"
    )]
    fn prompt_text_drops_the_cli_framing_around_a_pair(#[case] raw: &str, #[case] text: &str) {
        assert_eq!(prompt_text(raw), text);
        let got = read(HookEvent::UserPromptSubmit, &json!({"prompt": raw}));
        assert_eq!(got.data["text"], text);
    }

    /// The 1 500 bytes a live shape pasted: a head naming the shape, filler, padding and a tail.
    fn live_long(label: &str) -> String {
        let head = format!(
            "viola probe: {label} is one long synthetic paste and its filler words carry no meaning. "
        );
        let tail = "End of the synthetic paste. Reply with the single word ok";
        let room = 1500 - head.len() - tail.len();
        head + &"filler ".repeat(room / 7) + &"x".repeat(room % 7) + tail
    }

    const LIVE_OPEN: &str = "\n\n<pasted_content id=\"eec9\">\n";
    const LIVE_CLOSE: &str = "\n</pasted_content id=\"eec9\">\n";

    /// The prompts 2.1.287 submitted for five paste shapes in one session, each beside the text it
    /// normalises to and the measured prompt's length. Every pair of that session carried the one
    /// id, the two pairs of one prompt included. A pasted text's own last newline never reaches
    /// the hook: the CLI adds no newline before the close tag for it and drops it from an
    /// unwrapped text, so the last two cases, whose pasted bytes ended in LF, come back without it.
    #[rstest]
    #[case::typed_then_paste(
        format!("viola probe: typed lead before a paste. {LIVE_OPEN}{}{LIVE_CLOSE}", live_long("shape one")),
        format!("viola probe: typed lead before a paste. {}", live_long("shape one")),
        1598
    )]
    #[case::paste_then_typed(
        format!("{LIVE_OPEN}{}{LIVE_CLOSE}\n viola probe: typed tail after a paste.", live_long("shape two")),
        format!("{} viola probe: typed tail after a paste.", live_long("shape two")),
        1598
    )]
    #[case::two_pastes(
        format!(
            "{LIVE_OPEN}{}{LIVE_CLOSE}{LIVE_OPEN}{}{LIVE_CLOSE}",
            live_long("shape three a"),
            live_long("shape three b")
        ),
        format!("{}{}", live_long("shape three a"), live_long("shape three b")),
        3116
    )]
    #[case::long_ending_newline(
        format!("{LIVE_OPEN}{}{LIVE_CLOSE}", &live_long("shape four")[..1499]),
        live_long("shape four")[..1499].to_owned(),
        1557
    )]
    #[case::short_ending_newline(
        "viola probe: a short synthetic text whose last byte is a newline. Reply with the single word ok".to_owned(),
        "viola probe: a short synthetic text whose last byte is a newline. Reply with the single word ok".to_owned(),
        95
    )]
    fn prompt_text_live_shape_normalises_as_measured(
        #[case] raw: String,
        #[case] text: String,
        #[case] chars: usize,
    ) {
        assert_eq!(raw.len(), chars, "the measured prompt's length");
        assert_eq!(prompt_text(&raw), text);
        let got = read(HookEvent::UserPromptSubmit, &json!({"prompt": raw}));
        assert_eq!(got.data["text"], text.as_str());
    }

    /// The CR and LF characters at the very end go, every one of them in any order; an LF, a TAB
    /// or a space that is not at the very end stays where it is. The `measured` labels name the
    /// chunk whose live reading on 2.1.287 found the CLI submitting that ending's text without it.
    #[rstest]
    #[case::one("x\n", "x")]
    #[case::three("x\n\n\n", "x")]
    #[case::one_cr_measured_2026_10_08_first_live_test_and_self_drive("x\r", "x")]
    #[case::one_crlf_measured_2026_10_09_epoch_3_cleanup("x\r\n", "x")]
    #[case::two_crs_measured_2026_10_09_epoch_3_cleanup("x\r\r", "x")]
    #[case::crlf_twice("x\r\n\r\n", "x")]
    #[case::an_lf_before_a_cr("x\n\r", "x")]
    #[case::only_newlines("\n\n", "")]
    #[case::only_crs_and_lfs("\r\n\r\r\n", "")]
    #[case::an_inner_newline("x\ny", "x\ny")]
    #[case::a_space_after_it("x\n ", "x\n ")]
    #[case::a_tab_after_it("x\n\t", "x\n\t")]
    #[case::none("x", "x")]
    #[case::empty("", "")]
    fn typed_text_drops_every_trailing_newline(#[case] sent: &str, #[case] typed: &str) {
        assert_eq!(super::typed_text(sent), typed);
    }

    /// A CR LF pair inside the text is one LF and every other CR is one LF, wherever it stands;
    /// an LF before a CR is not a pair, so it stays beside that CR's LF. The `measured` labels
    /// name the chunk whose live reading on 2.1.287 found the CLI submitting that shape as one LF.
    #[rstest]
    #[case::one_crlf_inside_measured_2026_10_09_epoch_3_cleanup("x\r\ny", "x\ny")]
    #[case::one_lone_cr_inside_measured_2026_10_09_epoch_3_cleanup("x\ry", "x\ny")]
    #[case::two_crs_in_a_row("x\r\ry", "x\n\ny")]
    #[case::an_lf_before_a_cr("x\n\ry", "x\n\ny")]
    #[case::a_cr_before_a_crlf("x\r\r\ny", "x\n\ny")]
    #[case::crlf_on_every_line_with_a_crlf_ending("a\r\nb\r\nc\r\n", "a\nb\nc")]
    #[case::a_tab_after_a_crlf("x\r\n\t", "x\n\t")]
    #[case::a_cr_as_the_first_character("\rx", "\nx")]
    #[case::an_inner_cr_with_a_trailing_cr("x\ry\r", "x\ny")]
    #[case::no_cr("x\ny z", "x\ny z")]
    fn typed_text_every_inner_cr_is_one_lf(#[case] sent: &str, #[case] typed: &str) {
        assert_eq!(super::typed_text(sent), typed);
    }

    /// A text with no CR left inside it once its ending is gone is typed as a slice of the
    /// received text; only a text with an inner CR is copied.
    #[test]
    fn typed_text_copies_only_a_text_with_an_inner_cr() {
        assert!(matches!(
            super::typed_text("x\ny\r\n"),
            Cow::Borrowed("x\ny")
        ));
        assert!(matches!(super::typed_text("x\ry"), Cow::Owned(_)));
    }

    /// Two harness-prefix readings on 2.1.287: a `<task-notification>` typed at a prompt's very
    /// start arrives as typed, and a real cross-session message arrives unescaped under three
    /// attributes, their values replaced here.
    #[rstest]
    #[case::typed_task_notification_at_start(
        "<task-notification> viola probe: this tag was typed at the very start of a prompt and is sample text, not a notification. Reply with the single word ok"
    )]
    #[case::cross_session_message(
        "<cross-session-message from=\"<value>\" from-name=\"<value>\" from-mode=\"<value>\">\nviola probe: a synthetic cross-session message and nothing else. Reply with the single word ok\n</cross-session-message>"
    )]
    fn prompt_origin_live_shape_files_as_harness(#[case] prompt: &str) {
        let got = read(HookEvent::UserPromptSubmit, &json!({"prompt": prompt}));
        assert_eq!(got.data, json!({"text": prompt, "origin": "harness"}));
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

    /// A received text built from the pieces that matter to the typed text: short words, TAB,
    /// LF, CR and CR LF.
    fn received_text() -> impl Strategy<Value = String> {
        let piece = prop_oneof![
            "[a-z ]{0,4}",
            Just("\t".to_owned()),
            Just("\n".to_owned()),
            Just("\r".to_owned()),
            Just("\r\n".to_owned()),
        ];
        prop::collection::vec(piece, 0..12).prop_map(|pieces| pieces.concat())
    }

    /// The typed text's rule as the test's own walk over the received characters: a CR is an LF,
    /// an LF right after a CR is that CR's own and adds nothing, and the LFs left at the end go.
    fn every_cr_as_one_lf(received: &str) -> String {
        let mut out = String::with_capacity(received.len());
        let mut after_cr = false;
        for c in received.chars() {
            match c {
                '\r' => out.push('\n'),
                '\n' if after_cr => {}
                other => out.push(other),
            }
            after_cr = c == '\r';
        }
        while out.ends_with('\n') {
            out.pop();
        }
        out
    }

    fn one_of_the_events() -> impl Strategy<Value = HookEvent> {
        let spine: Vec<HookEvent> = HookEvent::ALL
            .into_iter()
            .filter(|e| !e.is_dialog())
            .collect();
        prop::sample::select(spine)
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

        /// The typed text holds no CR and no LF ending, holds nothing the received text did not
        /// hold except an LF, equals the walk's reading, and passes the paste validation whenever
        /// the received text does.
        #[test]
        fn typed_text_prop_holds_no_cr_and_every_inner_cr_is_one_lf(received in received_text()) {
            let typed = super::typed_text(&received);
            let typed: &str = &typed;
            prop_assert!(!typed.contains('\r'));
            prop_assert!(!typed.ends_with('\n'));
            prop_assert!(typed.chars().all(|c| c == '\n' || received.contains(c)));
            let walked = every_cr_as_one_lf(&received);
            prop_assert_eq!(typed, walked.as_str());
            if viola_core::validate_paste_text(&received).is_ok() {
                prop_assert!(viola_core::validate_paste_text(typed).is_ok());
            }
        }

        /// Escape the typed text as the CLI does, wrap it in the CLI's pair with one newline after
        /// the close and a second before a typed tail, normalise: the typed text comes back, typed
        /// pairs included, behind the lead less its two framing newlines and before the tail.
        #[test]
        fn prompt_text_prop_round_trips_a_wrapped_paste(
            text in typed_text(),
            id in "[a-zA-Z0-9]{1,8}",
            lead in prop::sample::select(vec![("", ""), ("\n\n", ""), ("note: ", "note: "), ("\n", "\n")]),
            tail in prop::sample::select(vec!["", " tail", "tail\n", "\ntail"]),
        ) {
            let (lead, kept) = lead;
            let open = String::from("<pasted_content id=\"") + &id + "\">\n";
            let close = String::from("\n</pasted_content id=\"") + &id + "\">";
            let framed_tail = if tail.is_empty() { String::new() } else { String::from("\n") + tail };
            let wire = String::from(lead) + &open + &cli_escape(&text) + &close + "\n" + &framed_tail;
            let expected = String::from(kept) + &text + tail;
            prop_assert_eq!(prompt_text(&wire), expected);
        }
    }
}
