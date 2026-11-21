//! The capability ledger (architecture §Established Decisions → [CLI Version Compatibility]): the
//! closed set of CLI behaviours a print-mode run of `claude` can measure, each with its
//! post-condition; the capture plugin that records the probe's raw hook payloads; the stamp a
//! `viola verify` run writes; and the scrub a recorded payload passes before it becomes a fixture.
//! Pure: no I/O. Upstream text is content, compared or copied, never interpreted.

use std::collections::BTreeMap;
use std::time::Duration;

use serde_json::{Map, Value, json};
use viola_core::MAX_FRAME;

use crate::AgentError;
use crate::dialog::{Permit, Response, Verdict, decision_body};
use crate::hook::{HookEvent, harness_prefix, prompt_text};
use crate::screen::{
    BOX_LITERAL_IDS, COMPACT_CONFIRM_WINDOW, CONFIRM_WINDOW_FALLBACK, FirstScreen, GATE_MAX_WAIT,
    RowFamily, SIGNATURES, UNTRUSTED_START_MODALS,
};
use crate::trust::TrustWord;

/// One measured behaviour, in the order `viola verify` checks and prints it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerRow {
    ShimResolution,
    SpineHooks,
    SessionStartFields,
    PromptVerbatim,
    StopMessage,
    LargestHookPayload,
    ModalSignature,
    InputBoxSignature,
    QuietPeriod,
    ConfirmWindow,
    QuestionAnswer,
    PlanApproveRevise,
    QuestionNotes,
    DialogConcurrency,
    LongPasteWrapper,
    TagEscaping,
    LocalCommandClear,
    PasteNewline,
    ResumeSession,
    AgentsJoin,
    LocalCommandCompact,
    LocalCommandEcho,
    TrustRecord,
    BrowserToolsPrompt,
    /// `/context`'s stdout tag. A CLI version whose `/context` writes no tag stamps this row
    /// `fail`, never `absent`, and reads unverified whole.
    LocalCommandContext,
    /// The Stop's `session_crons` as an array: the scheduled-wake count the wheel rule reads.
    StopSessionCrons,
    /// A question first raised by PermissionRequest takes the `allow` + `updatedInput` body. It
    /// reads `pass` or `fail`, never `absent`.
    PermissionRequestQuestion,
    /// Every `CLAUDE*` name the CLI adds to a hook child is on the identity floor or on the
    /// compiled non-identity hook names. It reads `pass` or `fail`, never `absent`.
    IdentityFloor,
    /// A message another session sends in arrives as a UserPromptSubmit whose raw start is a
    /// compiled harness literal. It reads `pass` or `fail`, never `absent`.
    CrossSessionPrefix,
}

impl LedgerRow {
    pub const ALL: [Self; 29] = [
        Self::ShimResolution,
        Self::SpineHooks,
        Self::SessionStartFields,
        Self::PromptVerbatim,
        Self::StopMessage,
        Self::LargestHookPayload,
        Self::ModalSignature,
        Self::InputBoxSignature,
        Self::QuietPeriod,
        Self::ConfirmWindow,
        Self::QuestionAnswer,
        Self::PlanApproveRevise,
        Self::QuestionNotes,
        Self::DialogConcurrency,
        Self::LongPasteWrapper,
        Self::TagEscaping,
        Self::LocalCommandClear,
        Self::PasteNewline,
        Self::ResumeSession,
        Self::AgentsJoin,
        Self::LocalCommandCompact,
        Self::LocalCommandEcho,
        Self::TrustRecord,
        Self::BrowserToolsPrompt,
        Self::LocalCommandContext,
        Self::StopSessionCrons,
        Self::PermissionRequestQuestion,
        Self::IdentityFloor,
        Self::CrossSessionPrefix,
    ];

    /// The kebab-case id, the key under a stamped version's `rows`.
    pub const fn id(self) -> &'static str {
        match self {
            Self::ShimResolution => "shim-resolution",
            Self::SpineHooks => "spine-hooks",
            Self::SessionStartFields => "session-start-fields",
            Self::PromptVerbatim => "prompt-verbatim",
            Self::StopMessage => "stop-message",
            Self::LargestHookPayload => "largest-hook-payload",
            Self::ModalSignature => "modal-signature",
            Self::InputBoxSignature => "input-box-signature",
            Self::QuietPeriod => "quiet-period",
            Self::ConfirmWindow => "confirm-window",
            Self::QuestionAnswer => "question-answer",
            Self::PlanApproveRevise => "plan-approve-revise",
            Self::QuestionNotes => "question-notes",
            Self::DialogConcurrency => "dialog-concurrency",
            Self::LongPasteWrapper => "long-paste-wrapper",
            Self::TagEscaping => "tag-escaping",
            Self::LocalCommandClear => "local-command-clear",
            Self::PasteNewline => "paste-newline",
            Self::ResumeSession => "resume-session",
            Self::AgentsJoin => "agents-join",
            Self::LocalCommandCompact => "local-command-compact",
            Self::LocalCommandEcho => "local-command-echo",
            Self::TrustRecord => "trust-record",
            Self::BrowserToolsPrompt => "browser-tools-prompt",
            Self::LocalCommandContext => "local-command-context",
            Self::StopSessionCrons => "stop-session-crons",
            Self::PermissionRequestQuestion => "permission-request-question",
            Self::IdentityFloor => "identity-floor",
            Self::CrossSessionPrefix => "cross-session-prefix",
        }
    }

    /// The fixed ASCII words `viola verify` prints after the id.
    pub const fn words(self) -> &'static str {
        match self {
            Self::ShimResolution => "claude resolves to a real executable",
            Self::SpineHooks => "spine hooks fire through the plugin dir",
            Self::SessionStartFields => "SessionStart carries session_id and source",
            Self::PromptVerbatim => "UserPromptSubmit carries the prompt as sent",
            Self::StopMessage => "Stop carries last_assistant_message",
            Self::LargestHookPayload => "every hook payload fits the frame cap",
            Self::ModalSignature => "an untrusted start shows a compiled modal literal",
            Self::InputBoxSignature => {
                "a trusted start shows a compiled input-box literal and no modal"
            }
            Self::QuietPeriod => "the screen settles within the gate's maximum wait",
            Self::ConfirmWindow => "the typed prompt reaches UserPromptSubmit within the window",
            Self::QuestionAnswer => "a question answered through PreToolUse takes effect",
            Self::PlanApproveRevise => "a plan revise and approve each take effect",
            Self::QuestionNotes => "free text and notes reach the question",
            Self::DialogConcurrency => "two parallel questions each raise a dialog",
            Self::LongPasteWrapper => "a long paste unwraps to the text as pasted",
            Self::TagEscaping => "tag-like text un-escapes to the text as pasted",
            Self::LocalCommandClear => "/clear starts a new session and submits no prompt",
            Self::PasteNewline => "an LF inside a paste stays inside the one prompt submitted",
            Self::ResumeSession => "--resume <id> reopens the session under that id",
            Self::AgentsJoin => "claude agents --json names the live session by pid and session id",
            Self::LocalCommandCompact => "/compact keeps the session id and submits no prompt",
            Self::LocalCommandEcho => {
                "/mcp writes its stdout tag to the transcript and submits no prompt"
            }
            Self::TrustRecord => {
                "a trust record written for an untrusted folder opens it with no modal"
            }
            Self::BrowserToolsPrompt => {
                "an interactive start's browser-tools prompt reads as a modal, nothing typed"
            }
            Self::LocalCommandContext => {
                "/context writes its stdout tag to the transcript and submits no prompt"
            }
            Self::StopSessionCrons => "Stop carries session_crons as an array",
            Self::PermissionRequestQuestion => {
                "a question raised by PermissionRequest takes the allow body"
            }
            Self::IdentityFloor => "every CLAUDE name the CLI adds to a hook is a known name",
            Self::CrossSessionPrefix => {
                "a message another session sends in opens with a harness prefix"
            }
        }
    }

    /// The rows relied on, every row in [`LedgerRow::ALL`] order: what `viola verify` checks and
    /// the stamp verdict reads.
    pub fn relied() -> Vec<Self> {
        Self::ALL.to_vec()
    }
}

/// The probe's synthetic prompt: ASCII, no tag characters.
pub const PROBE_PROMPT: &str = "viola verify probe: reply with the single word ok";

/// Run B's long paste: 1 500 bytes on one line, past the length at which the CLI wraps a paste in
/// its `pasted_content` pair. Synthetic ASCII; it does not end in a newline.
pub const PROBE_LONG_PASTE: &str = "viola verify probe: this message is one long synthetic paste and its filler words carry no meaning. Filler follows: \
     filler-0001 filler-0002 filler-0003 filler-0004 filler-0005 filler-0006 \
     filler-0007 filler-0008 filler-0009 filler-0010 filler-0011 filler-0012 \
     filler-0013 filler-0014 filler-0015 filler-0016 filler-0017 filler-0018 \
     filler-0019 filler-0020 filler-0021 filler-0022 filler-0023 filler-0024 \
     filler-0025 filler-0026 filler-0027 filler-0028 filler-0029 filler-0030 \
     filler-0031 filler-0032 filler-0033 filler-0034 filler-0035 filler-0036 \
     filler-0037 filler-0038 filler-0039 filler-0040 filler-0041 filler-0042 \
     filler-0043 filler-0044 filler-0045 filler-0046 filler-0047 filler-0048 \
     filler-0049 filler-0050 filler-0051 filler-0052 filler-0053 filler-0054 \
     filler-0055 filler-0056 filler-0057 filler-0058 filler-0059 filler-0060 \
     filler-0061 filler-0062 filler-0063 filler-0064 filler-0065 filler-0066 \
     filler-0067 filler-0068 filler-0069 filler-0070 filler-0071 filler-0072 \
     filler-0073 filler-0074 filler-0075 filler-0076 filler-0077 filler-0078 \
     filler-0079 filler-0080 filler-0081 filler-0082 filler-0083 filler-0084 \
     filler-0085 filler-0086 filler-0087 filler-0088 filler-0089 filler-0090 \
     filler-0091 filler-0092 filler-0093 filler-0094 filler-0095 filler-0096 \
     filler-0097 filler-0098 filler-0099 filler-0100 filler-0101 filler-0102 \
     filler-0103 filler-0104 filler-0105 filler-0106 filler-0107 filler-0108 \
     filler-0109 filler-0110xxxxxxx \
     End of the synthetic paste. Reply with the single word ok";
/// Run B's tag-like paste: a typed `pasted_content` pair and a typed `<task-notification>`, none at
/// the text's start, short enough to arrive unwrapped.
pub const PROBE_TAG_PASTE: &str = "viola verify probe: the next part is literal sample text and not markup: \
     <pasted_content id=\"1\"> sample </pasted_content id=\"1\"> \
     then <task-notification> and that is all. Reply with the single word ok";
/// Run B's newline paste: one LF, at neither end, no CR, short enough to arrive unwrapped.
pub const PROBE_NEWLINE_PASTE: &str = "viola verify probe: this paste holds one line break here\nand the next line ends it. Reply with the single word ok";

/// What confirms that a local command was delivered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostCondition {
    /// A SessionStart with source `clear` and a new `session_id`.
    NewSession,
    /// A SessionStart with source `compact` and the session's own `session_id`.
    CompactedSession,
    /// One more [`ECHO_TAG`] in the session's transcript after the send's own pre-paste end.
    CommandStdout,
}

/// The known built-in local commands by name, which fire no UserPromptSubmit, each with its
/// delivery post-condition or none (architecture [Delivery Confirmation]).
pub const LOCAL_COMMANDS: [(&str, Option<PostCondition>); 7] = [
    ("/clear", Some(PostCondition::NewSession)),
    ("/remote-control", None),
    ("/compact", Some(PostCondition::CompactedSession)),
    ("/mcp", Some(PostCondition::CommandStdout)),
    ("/cost", None),
    ("/usage", None),
    ("/context", Some(PostCondition::CommandStdout)),
];
/// The local command Run B pastes last: the list's `/clear` entry.
pub const PROBE_LOCAL_COMMAND: &str = LOCAL_COMMANDS[0].0;
/// The local command Run B pastes before `/clear`: the list's `/compact` entry.
pub const PROBE_COMPACT_COMMAND: &str = LOCAL_COMMANDS[2].0;
/// The local command Run B pastes after the newline paste and before `/compact`: a `/mcp` call
/// naming a server no config holds, which writes one [`ECHO_TAG`] and changes nothing.
pub const PROBE_ECHO_COMMAND: &str = "/mcp disable viola-probe-absent";
/// Run B's eighth paste, right after [`PROBE_ECHO_COMMAND`]: the list's `/context` entry, which
/// writes one [`ECHO_TAG`] and changes nothing.
pub const PROBE_CONTEXT_COMMAND: &str = "/context";
/// The transcript tag a local command's own output is written under.
pub const ECHO_TAG: &str = "<local-command-stdout>";

/// The listed entry `typed` names: exactly a listed name, or a name, one space and a non-empty
/// rest. A bare leading slash, a longer word, a tab or a second space after the name is none.
pub fn local_command_of(typed: &str) -> Option<&'static (&'static str, Option<PostCondition>)> {
    LOCAL_COMMANDS.iter().find(|(name, _)| {
        typed.strip_prefix(name).is_some_and(|rest| {
            rest.is_empty()
                || rest
                    .strip_prefix(' ')
                    .is_some_and(|args| !args.is_empty() && !args.starts_with(' '))
        })
    })
}

/// A transcript path the CLI declared for `session_id`: absolute, no `..` component, and a file
/// name of exactly `{session_id}.jsonl` with `session_id` of the session-id shape.
pub fn transcript_path_ok(path: &str, session_id: &str) -> bool {
    let path = std::path::Path::new(path);
    crate::is_session_id(session_id)
        && path.is_absolute()
        && !path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        && path
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| name.strip_suffix(".jsonl"))
            == Some(session_id)
}

/// Run B's five added pastes in paste order, each with the stem its recorded variants carry.
pub const FRAMING_TURNS: [(&str, &str); 5] = [
    (PROBE_LONG_PASTE, "paste-1"),
    (PROBE_TAG_PASTE, "paste-2"),
    (PROBE_NEWLINE_PASTE, "paste-3"),
    (PROBE_COMPACT_COMMAND, "compact-1"),
    (PROBE_LOCAL_COMMAND, "clear-1"),
];

/// The variant stem of a compiled Run B paste; any other text has none.
pub fn framing_stem(text: &str) -> Option<&'static str> {
    FRAMING_TURNS
        .iter()
        .find(|(t, _)| *t == text)
        .map(|(_, stem)| *stem)
}

/// The dialog run's first prompt: one AskUserQuestion call carrying two questions.
pub const DIALOG_PROMPT_QUESTIONS: &str = "viola verify probe: call the AskUserQuestion tool exactly \
     once, with two questions in that one call. Question 1 is Probe color? with the options red and \
     blue. Question 2 is Probe size? with the options small and large. After the answers come back, \
     reply with the single word ok.";
/// The dialog run's second prompt: two AskUserQuestion calls in one message.
pub const DIALOG_PROMPT_PARALLEL: &str = "viola verify probe: in one single message, make two \
     AskUserQuestion tool calls in parallel, each call with one question. The first call asks Probe \
     left? with the options yes and no. The second call asks Probe right? with the options yes and \
     no. After both answers come back, reply with the single word ok.";
/// The dialog run's third prompt: one ordinary tool call the session's `ask` rule holds.
pub const DIALOG_PROMPT_PERMISSION: &str = "viola verify probe: run exactly this shell command with \
     the Bash tool, nothing else: touch viola-probe-permission. Then reply with the single word ok.";
/// The dialog run's fourth prompt, pasted last: one AskUserQuestion call whose PreToolUse is left
/// unanswered, so the CLI raises its PermissionRequest.
pub const DIALOG_PROMPT_PERMISSION_QUESTION: &str = "viola verify probe: call the AskUserQuestion \
     tool exactly once, with one question. The question is Probe shape? with the options round and \
     square. After the answer comes back, reply with the single word ok.";
/// The plan run's prompt, in plan mode.
pub const PLAN_PROMPT: &str = "viola verify probe: make a plan only, do not run anything. The plan \
     has one step: reply with the word done. Present the plan with the ExitPlanMode tool. If the plan \
     is sent back, revise it once as asked and present it again with ExitPlanMode.";

/// The dialog run's `--settings`: a session-scoped `ask` rule, which outranks a user `allow`.
pub const DIALOG_SETTINGS: &str =
    r#"{"permissions":{"ask":["Bash(touch viola-probe-permission)"]}}"#;

/// Run B's `--settings`: a message another session sends in is accepted at once, never held.
pub const CROSS_SESSION_SETTINGS: &str = r#"{"crossSessionInbound":"accept"}"#;

/// The message verify's print-mode sender sends into Run B: synthetic ASCII saying what it is.
pub const CROSS_SESSION_TEXT: &str = "viola verify probe: a synthetic cross-session message sent by viola verify; reply with the single word ok";
/// The sender's prompt up to the target socket address.
pub const CROSS_SESSION_SENDER_HEAD: &str = "viola verify probe: call the SendMessage tool exactly once, with to set to exactly this address: ";
/// The sender's prompt after the target socket address.
pub const CROSS_SESSION_SENDER_TAIL: &str = " and message set to exactly this text: viola verify probe: a synthetic cross-session message sent by viola verify; reply with the single word ok. Then reply with the single word ok.";

/// Whether `path` is absolute, holds only `[A-Za-z0-9/._-]` and no `.` or `..` component: no C0,
/// C1, DEL or space can reach a prompt through it, and nothing is stripped.
fn socket_path_shaped(path: &[u8]) -> bool {
    path.first() == Some(&b'/')
        && path
            .iter()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'/' | b'.' | b'_' | b'-'))
        && !path
            .split(|b| *b == b'/')
            .any(|part| part == b"." || part == b"..")
}

/// The socket address of the CLI session of `pid` whose runtime dir is `runtime`:
/// `uds:<runtime>/cc-socks/<pid>.sock`, the form 2.1.328 names its own inbox socket. One trailing
/// `/` of `runtime` is dropped, as the CLI's path join does. `None` when `runtime` is not absolute,
/// holds a `.` or `..` component or a byte outside `[A-Za-z0-9/._-]`, or when the composed path is
/// over the CLI's own bound. Composed only: nothing at the path is read.
pub fn uds_address(runtime: &std::ffi::OsStr, pid: u32) -> Option<String> {
    use std::os::unix::ffi::OsStrExt;
    let bytes = runtime.as_bytes();
    let dir = bytes.strip_suffix(b"/").unwrap_or(bytes);
    if !socket_path_shaped(dir) {
        return None;
    }
    let dir = std::str::from_utf8(dir).ok()?;
    let path = format!(
        "{dir}/{}/{pid}{}",
        crate::CLI_SOCKET_DIR,
        crate::CLI_SOCKET_SUFFIX
    );
    (path.len() <= crate::CLI_SOCKET_PATH_MAX).then(|| format!("{}{path}", crate::UDS_SCHEME))
}

/// The sender's prompt for the socket `address`; `None` when it does not open with `uds:` or its
/// path is not of `uds_address`'s shape and bound.
pub fn sender_prompt(address: &str) -> Option<String> {
    let path = address.strip_prefix(crate::UDS_SCHEME)?;
    (socket_path_shaped(path.as_bytes()) && path.len() <= crate::CLI_SOCKET_PATH_MAX)
        .then(|| format!("{CROSS_SESSION_SENDER_HEAD}{address}{CROSS_SESSION_SENDER_TAIL}"))
}

/// The plan run's `--settings`: the CLI's plan file goes into `plans_dir`, inside the run's own dir.
pub fn plan_settings(plans_dir: &str) -> String {
    json!({ "plansDirectory": plans_dir }).to_string()
}

/// The free text, note and revise message the probe answers with: ASCII, no tag characters.
pub const PROBE_FREE_TEXT: &str = "viola probe free text";
pub const PROBE_NOTE: &str = "viola probe note";
pub const PROBE_REVISE: &str =
    "viola probe revise: add one more step to the plan, then present it again with ExitPlanMode";

/// The two interactive runs that raise dialogs: Run C (questions and a permission), Run D (a plan).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogRun {
    Questions,
    Plan,
}

/// Each dialog prompt, its run and the stem its recorded variants carry, in paste order.
pub const DIALOG_TURNS: [(DialogRun, &str, &str); 5] = [
    (DialogRun::Questions, DIALOG_PROMPT_QUESTIONS, "questions"),
    (DialogRun::Questions, DIALOG_PROMPT_PARALLEL, "parallel"),
    (DialogRun::Questions, DIALOG_PROMPT_PERMISSION, "permission"),
    (
        DialogRun::Questions,
        DIALOG_PROMPT_PERMISSION_QUESTION,
        "question-permission",
    ),
    (DialogRun::Plan, PLAN_PROMPT, "plan"),
];

/// The variant stem of a compiled dialog prompt; any other text has none.
pub fn dialog_stem(prompt: &str) -> Option<&'static str> {
    DIALOG_TURNS
        .iter()
        .find(|(_, p, _)| *p == prompt)
        .map(|(_, _, stem)| *stem)
}

/// The answer the probe's capture hook gives one dialog event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeAnswer {
    QuestionsFirstAndFreeText,
    QuestionFirstOption,
    PlanRevise,
    PlanApprove,
    PermitAllow,
    PermissionQuestionFirstOption,
}

impl ProbeAnswer {
    pub const ALL: [Self; 6] = [
        Self::QuestionsFirstAndFreeText,
        Self::QuestionFirstOption,
        Self::PlanRevise,
        Self::PlanApprove,
        Self::PermitAllow,
        Self::PermissionQuestionFirstOption,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::QuestionsFirstAndFreeText => "questions-first-and-free-text",
            Self::QuestionFirstOption => "question-first-option",
            Self::PlanRevise => "plan-revise",
            Self::PlanApprove => "plan-approve",
            Self::PermitAllow => "permit-allow",
            Self::PermissionQuestionFirstOption => "permission-question-first-option",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|a| a.id() == id)
    }
}

/// Which answer goes to which dialog event: (run, event, ordinal of that event in the run). Run D's
/// first PreToolUse is left unanswered, so the CLI raises the plan's PermissionRequest; so is Run
/// C's fourth PreToolUse, so the CLI raises the last question's PermissionRequest.
pub const PROBE_ANSWERS: [(DialogRun, HookEvent, u32, ProbeAnswer); 7] = [
    (
        DialogRun::Questions,
        HookEvent::PreToolUse,
        1,
        ProbeAnswer::QuestionsFirstAndFreeText,
    ),
    (
        DialogRun::Questions,
        HookEvent::PreToolUse,
        2,
        ProbeAnswer::QuestionFirstOption,
    ),
    (
        DialogRun::Questions,
        HookEvent::PreToolUse,
        3,
        ProbeAnswer::QuestionFirstOption,
    ),
    (
        DialogRun::Questions,
        HookEvent::PermissionRequest,
        1,
        ProbeAnswer::PermitAllow,
    ),
    (
        DialogRun::Questions,
        HookEvent::PermissionRequest,
        2,
        ProbeAnswer::PermissionQuestionFirstOption,
    ),
    (
        DialogRun::Plan,
        HookEvent::PermissionRequest,
        1,
        ProbeAnswer::PlanRevise,
    ),
    (
        DialogRun::Plan,
        HookEvent::PreToolUse,
        2,
        ProbeAnswer::PlanApprove,
    ),
];

/// `<Event>.<ordinal>`: an answer file's name in a run's answers dir.
pub fn answer_file_name(event: HookEvent, ordinal: u32) -> String {
    format!("{}.{ordinal}", event_name(event))
}

/// The body the product's own mapping (`dialog::decision_body`) gives `answer` over a captured
/// `payload`, keyed by the payload's own question texts; `None` when the payload does not carry what
/// the answer needs.
pub fn probe_body(event: HookEvent, payload: &[u8], answer: ProbeAnswer) -> Option<String> {
    let payload: Value = serde_json::from_slice(payload).ok()?;
    let input = payload.get("tool_input")?;
    let response = match answer {
        ProbeAnswer::QuestionsFirstAndFreeText
        | ProbeAnswer::QuestionFirstOption
        | ProbeAnswer::PermissionQuestionFirstOption => {
            question_response(input, answer == ProbeAnswer::QuestionsFirstAndFreeText)?
        }
        ProbeAnswer::PlanRevise => Response::Plan {
            behavior: Verdict::Revise,
            message: Some(PROBE_REVISE.to_owned()),
        },
        ProbeAnswer::PlanApprove => Response::Plan {
            behavior: Verdict::Approve,
            message: None,
        },
        ProbeAnswer::PermitAllow => Response::Permission {
            behavior: Permit::Allow,
            message: None,
        },
    };
    decision_body(event, input, &response)
}

/// Every question's first option; with `free_text`, the second question's answer is the free text
/// and the first question carries the note.
fn question_response(input: &Value, free_text: bool) -> Option<Response> {
    let questions = input["questions"].as_array()?;
    let mut answers = BTreeMap::new();
    for (i, question) in questions.iter().enumerate() {
        let text = question["question"].as_str()?;
        let value = if free_text && i == 1 {
            PROBE_FREE_TEXT
        } else {
            question["options"][0]["label"].as_str()?
        };
        answers.insert(text.to_owned(), value.to_owned());
    }
    let annotations = if free_text {
        let first = questions.first()?["question"].as_str()?;
        let mut map = Map::new();
        map.insert(first.to_owned(), json!({ "notes": PROBE_NOTE }));
        Some(map)
    } else {
        None
    };
    Some(Response::Question {
        answers,
        annotations,
    })
}

/// The spine events the capture plugin registers, in the order one print-mode turn fires them.
pub const CAPTURE_EVENTS: [HookEvent; 4] = [
    HookEvent::SessionStart,
    HookEvent::UserPromptSubmit,
    HookEvent::Stop,
    HookEvent::SessionEnd,
];

/// The CLI's PascalCase hook event name, which is also its `hook_event_name` and a fixture's stem.
pub const fn event_name(event: HookEvent) -> &'static str {
    match event {
        HookEvent::SessionStart => "SessionStart",
        HookEvent::UserPromptSubmit => "UserPromptSubmit",
        HookEvent::Stop => "Stop",
        HookEvent::StopFailure => "StopFailure",
        HookEvent::SessionEnd => "SessionEnd",
        HookEvent::Notification => "Notification",
        HookEvent::PostToolUse => "PostToolUse",
        HookEvent::PostToolUseFailure => "PostToolUseFailure",
        HookEvent::PreToolUse => "PreToolUse",
        HookEvent::PermissionRequest => "PermissionRequest",
    }
}

/// The version `--version` printed: a first line of exactly `X.Y.Z (Claude Code)`, three decimal
/// fields, trailing whitespace allowed. Anything else is `None`.
pub fn parse_version(stdout: &[u8]) -> Option<String> {
    let first = stdout.split(|b| *b == b'\n').next()?;
    let line = std::str::from_utf8(first).ok()?.trim_end();
    let version = line.strip_suffix(" (Claude Code)")?;
    let fields: Vec<&str> = version.split('.').collect();
    let decimal = |f: &&str| !f.is_empty() && f.bytes().all(|b| b.is_ascii_digit());
    (fields.len() == 3 && fields.iter().all(decimal)).then(|| version.to_owned())
}

/// The tools whose PreToolUse the dialog plugin registers: the two that raise a dialog.
pub const DIALOG_MATCHER: &str = "AskUserQuestion|ExitPlanMode";

/// Which events a probe plugin registers: the spine alone (the print probe, Run B), or the spine and
/// the dialog tier, whose two dialog events also answer from `answers_dir_fwd` (Run C, Run D).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbePlugin<'a> {
    Spine,
    Dialog {
        answers_dir_fwd: &'a str,
    },
    /// The print-mode sender's: PreToolUse with no matcher and PostToolUse on the send tool, each
    /// in the names-only mode; no spine event.
    Sender,
}

/// The names-only flag the sender plugin's hooks carry beside `--capture`.
pub const NAMES_ONLY_FLAG: &str = "--names-only";

/// The sender plugin's `hooks` map: two groups, each `hook <event> --capture <dir> --names-only`.
fn sender_hooks(pinned_bin_fwd: &str, capture_dir_fwd: &str) -> Map<String, Value> {
    let mut hooks = Map::new();
    for (event, matcher) in [
        (HookEvent::PreToolUse, None),
        (HookEvent::PostToolUse, Some(SEND_TOOL)),
    ] {
        let hook = json!({
            "type": "command",
            "command": pinned_bin_fwd,
            "args": ["hook", event.as_str(), "--capture", capture_dir_fwd, NAMES_ONLY_FLAG],
            "timeout": 5,
        });
        let group = match matcher {
            Some(matcher) => json!({"matcher": matcher, "hooks": [hook]}),
            None => json!({"hooks": [hook]}),
        };
        hooks.insert(event_name(event).to_owned(), json!([group]));
    }
    hooks
}

/// The probe's plugin folder, each file as `(relative path, content)`: every registered event runs
/// `<pinned viola> hook <event> --capture <capture dir>` in exec form, the shape the run plugin
/// takes; a dialog plugin's PreToolUse and PermissionRequest add `--answers <answers dir>`.
pub fn capture_plugin_files(
    pinned_bin_fwd: &str,
    capture_dir_fwd: &str,
    kind: ProbePlugin<'_>,
) -> [(&'static str, String); 2] {
    let plugin = json!({
        "name": "viola-verify-probe",
        "version": crate::VERSION,
        "description": "viola verify: records the probe's raw hook payloads",
    });
    if kind == ProbePlugin::Sender {
        return [
            (".claude-plugin/plugin.json", pretty(&plugin)),
            (
                "hooks/hooks.json",
                pretty(&json!({"hooks": sender_hooks(pinned_bin_fwd, capture_dir_fwd)})),
            ),
        ];
    }
    let mut hooks = Map::new();
    let dialog = [
        HookEvent::PreToolUse,
        HookEvent::PermissionRequest,
        HookEvent::PostToolUse,
    ];
    let events = match kind {
        ProbePlugin::Spine | ProbePlugin::Sender => &dialog[..0],
        ProbePlugin::Dialog { .. } => &dialog[..],
    };
    for event in CAPTURE_EVENTS.iter().chain(events).copied() {
        let mut args = vec!["hook", event.as_str(), "--capture", capture_dir_fwd];
        if let (ProbePlugin::Dialog { answers_dir_fwd }, true) = (kind, event.is_dialog()) {
            args.extend(["--answers", answers_dir_fwd]);
        }
        let mut hook = json!({
            "type": "command",
            "command": pinned_bin_fwd,
            "args": args,
        });
        if event != HookEvent::SessionEnd {
            hook["timeout"] = json!(5);
        }
        let mut group = json!({"hooks": [hook]});
        if event == HookEvent::PreToolUse {
            group = json!({"matcher": DIALOG_MATCHER, "hooks": [hook]});
        }
        hooks.insert(event_name(event).to_owned(), json!([group]));
    }
    [
        (".claude-plugin/plugin.json", pretty(&plugin)),
        ("hooks/hooks.json", pretty(&json!({"hooks": hooks}))),
    ]
}

fn pretty(value: &Value) -> String {
    let mut text = serde_json::to_string_pretty(value).unwrap_or_default();
    text.push('\n');
    text
}

/// `<PascalEvent>.<k>.json`: `k` counts across every event, so the names sort in arrival order.
pub fn capture_file_name(event: HookEvent, k: u32) -> String {
    format!("{}.{k}.json", event_name(event))
}

/// `Env.<k>.json`: the env-names file the capture arm writes beside a SessionStart's `<k>`. `Env`
/// is no hook event, so [`parse_capture_file_name`] reads no such name.
pub fn env_file_name(k: u32) -> String {
    format!("Env.{k}.json")
}

/// The env-names document of one hook child, over NAMES alone: never a value. A name whose text
/// opens `CLAUDE` and is ASCII `[A-Z0-9_]` only is kept in `claude_names` (sorted, once each); any
/// other `CLAUDE`-prefixed name is counted in `claude_unshaped`, and every other name in
/// `other_count`, unnamed.
pub fn env_capture_document(names: impl IntoIterator<Item = std::ffi::OsString>) -> Vec<u8> {
    let mut claude = std::collections::BTreeSet::new();
    let (mut unshaped, mut other) = (0u64, 0u64);
    for name in names {
        let text = name.to_string_lossy();
        if !text.starts_with("CLAUDE") {
            other += 1;
        } else if text
            .bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
        {
            claude.insert(text.into_owned());
        } else {
            unshaped += 1;
        }
    }
    json!({
        "v": 1,
        "claude_names": claude,
        "claude_unshaped": unshaped,
        "other_count": other,
    })
    .to_string()
    .into_bytes()
}

/// The CLI's own cross-session send tool, read in the 2.1.328 bundle; a `viola verify` reading.
pub const SEND_TOOL: &str = "SendMessage";
/// The CLI's tool that fetches a deferred tool, read in the 2.1.328 bundle.
pub const SEARCH_TOOL: &str = "ToolSearch";
/// The CLI's tool that lists the sessions a send can reach, read in the 2.1.328 bundle.
pub const LIST_TOOL: &str = "ListAgents";

/// The closed word of a tool a print-mode sender called: one of the three compiled tool names, or
/// `other`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolWord {
    Send,
    Search,
    List,
    Other,
}

impl ToolWord {
    pub const ALL: [Self; 4] = [Self::Send, Self::Search, Self::List, Self::Other];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Send => "send",
            Self::Search => "search",
            Self::List => "list",
            Self::Other => "other",
        }
    }

    pub fn from_word(word: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|w| w.as_str() == word)
    }
}

/// The closed word of the send tool's result: its own boolean read `true` or `false`, or any other
/// shape `unread`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultWord {
    Delivered,
    Refused,
    Unread,
}

impl ResultWord {
    pub const ALL: [Self; 3] = [Self::Delivered, Self::Refused, Self::Unread];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Delivered => "delivered",
            Self::Refused => "refused",
            Self::Unread => "unread",
        }
    }

    pub fn from_word(word: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|w| w.as_str() == word)
    }
}

/// The word of a tool hook payload's `tool_name`, read as a string and kept as nothing else.
pub fn tool_word(payload: &[u8]) -> ToolWord {
    let name = serde_json::from_slice::<Value>(payload)
        .ok()
        .and_then(|p| p.get("tool_name")?.as_str().map(str::to_owned));
    match name.as_deref() {
        Some(SEND_TOOL) => ToolWord::Send,
        Some(SEARCH_TOOL) => ToolWord::Search,
        Some(LIST_TOOL) => ToolWord::List,
        _ => ToolWord::Other,
    }
}

/// The send's result from a PostToolUse payload: only a JSON boolean `success` at
/// `tool_response.success` or `tool_response.data.success`, the two shapes the tool's own code
/// states. No text is matched; any other shape reads `unread`, never `refused`.
pub fn send_result_word(payload: &[u8]) -> ResultWord {
    let Ok(payload) = serde_json::from_slice::<Value>(payload) else {
        return ResultWord::Unread;
    };
    let response = &payload["tool_response"];
    let success = response["success"]
        .as_bool()
        .or_else(|| response["data"]["success"].as_bool());
    match success {
        Some(true) => ResultWord::Delivered,
        Some(false) => ResultWord::Refused,
        None => ResultWord::Unread,
    }
}

/// The names-only capture's document: the event's side, the tool's word and, on a PostToolUse,
/// the result's word. Closed words only.
pub fn tool_capture_document(
    event: HookEvent,
    tool: ToolWord,
    result: Option<ResultWord>,
) -> Vec<u8> {
    let side = if event == HookEvent::PostToolUse {
        "post"
    } else {
        "pre"
    };
    json!({
        "v": 1,
        "event": side,
        "tool": tool.as_str(),
        "result": result.map(ResultWord::as_str),
    })
    .to_string()
    .into_bytes()
}

/// `Tool.<k>.json`: a names-only capture's file. `Tool` is no hook event, so
/// [`parse_capture_file_name`] reads no such name.
pub fn tool_file_name(k: u32) -> String {
    format!("Tool.{k}.json")
}

/// A `Tool.<k>.json` name's `k`.
pub fn parse_tool_file_name(name: &str) -> Option<u32> {
    name.strip_prefix("Tool.")?
        .strip_suffix(".json")?
        .parse()
        .ok()
        .filter(|k| *k > 0)
}

pub fn parse_capture_file_name(name: &str) -> Option<(HookEvent, u32)> {
    let (stem, k) = name.strip_suffix(".json")?.rsplit_once('.')?;
    let event = HookEvent::ALL
        .into_iter()
        .find(|e| event_name(*e) == stem)?;
    let k = k.parse().ok().filter(|k| *k > 0)?;
    Some((event, k))
}

/// One captured payload: its size as written and, when it parses, the JSON.
#[derive(Debug, Clone, PartialEq)]
pub struct Capture {
    pub event: HookEvent,
    pub bytes: usize,
    pub payload: Option<Value>,
}

impl Capture {
    pub fn read(event: HookEvent, bytes: &[u8]) -> Self {
        Self {
            event,
            bytes: bytes.len(),
            payload: serde_json::from_slice(bytes).ok(),
        }
    }

    fn field(&self, name: &str) -> Option<&str> {
        self.payload.as_ref()?.get(name)?.as_str()
    }

    fn is(&self, event: HookEvent, tool: &str) -> bool {
        self.event == event && self.field("tool_name") == Some(tool)
    }
}

/// What one probe measured: whether the resolved program answered `--version`, and the captures
/// in arrival order.
#[derive(Debug, Clone, PartialEq)]
pub struct ProbeRun {
    pub version_answered: bool,
    pub captures: Vec<Capture>,
}

impl ProbeRun {
    fn first(&self, event: HookEvent) -> Option<&Value> {
        self.captures
            .iter()
            .find(|c| c.event == event)
            .and_then(|c| c.payload.as_ref())
    }
}

/// What the two interactive runs measured: the recorded row text of the `modal` screen (the
/// untrusted run) and of the `ready` and `turn` screens (the trusted run), the rows Run B's input
/// box settled on after the long paste's turn (`long_turn`), and the timings in ms.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TypedRun {
    pub modal: Option<Vec<String>>,
    pub ready: Option<Vec<String>>,
    pub turn: Option<Vec<String>>,
    pub long_turn: Option<Vec<String>>,
    pub ready_settle_ms: Option<u64>,
    pub turn_settle_ms: Option<u64>,
    pub prompt_latency_ms: Option<u64>,
    pub max_turn_gap_ms: Option<u64>,
    /// From Run B's `/compact` paste to the instant its answer was read.
    pub compact_latency_ms: Option<u64>,
    /// From Run B's `/mcp` paste to the instant one more stdout tag was read in its transcript.
    pub echo_latency_ms: Option<u64>,
    /// From Run B's `/context` paste to the instant one more stdout tag was read in its transcript.
    pub context_latency_ms: Option<u64>,
    /// How many of Run B's paste guards read the input box up on the box's own rows, with no
    /// footer literal on any row.
    pub box_only_guards: Option<u32>,
    /// Run F's first settled screen: the untrusted folder started with its trust record written.
    pub trust_frame: Option<Vec<String>>,
    /// What writing Run F's trust record did; `None` when Run F never wrote.
    pub trust_write: Option<TrustWord>,
    /// What removing Run F's trust record did; `None` when nothing was written to remove.
    pub trust_remove: Option<TrustWord>,
    /// Each interactive run's first settled screen, in run order A–F; `None` for a run that never
    /// started (or was stopped before its settle).
    pub first: [Option<FirstReading>; 6],
    /// The rows of the settle each `first` reading was taken at, in run order A–F; `None` where no
    /// reading was kept or the screen was poisoned. Held in memory for `verify`'s rig-only
    /// first-screen recording alone: no stamp key and no row reads them.
    pub first_rows: [Option<Vec<String>>; 6],
    /// Run B's cross-session turn; `None` when the sender never started.
    pub cross_session: Option<CrossSessionRun>,
}

/// How the print-mode sender ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SenderEnd {
    Exited,
    Failed,
    Deadline,
}

/// What Run B's cross-session turn measured: the UserPromptSubmit captures standing before the
/// sender started, how the sender ended, the ms from the sender's start to one more
/// UserPromptSubmit read, and the sender's own closed words: the tools it called (first-seen
/// order, once each; empty when no file stood) and the send's result (`None` when no post file
/// stood).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossSessionRun {
    pub prompts_before: usize,
    pub sender: SenderEnd,
    pub latency_ms: Option<u64>,
    pub sender_tools: Vec<ToolWord>,
    pub send_result: Option<ResultWord>,
}

/// The sender's `Tool.<k>.json` documents in `k` order, read as closed words: the `pre` words in
/// first-seen order, once each, and the first `post` file's result word. A document of another
/// shape is skipped.
pub fn sender_words(documents: &[Value]) -> (Vec<ToolWord>, Option<ResultWord>) {
    let mut tools = Vec::new();
    let mut result = None;
    for doc in documents {
        match doc["event"].as_str() {
            Some("pre") => {
                if let Some(word) = doc["tool"].as_str().and_then(ToolWord::from_word)
                    && !tools.contains(&word)
                {
                    tools.push(word);
                }
            }
            Some("post") if result.is_none() => {
                result = doc["result"].as_str().and_then(ResultWord::from_word);
            }
            _ => {}
        }
    }
    (tools, result)
}

/// The cross-session row's reading: its outcome word and, on `harness`, the matched literal's id.
/// `not-run` · `sender-failed` · `sender-deadline` · `no-prompt` (no UserPromptSubmit past the
/// ones standing before the sender) · `unprefixed` · `harness`.
pub fn cross_session_reading(
    trusted: &[Capture],
    typed: &TypedRun,
) -> (&'static str, Option<&'static str>) {
    let Some(run) = &typed.cross_session else {
        return ("not-run", None);
    };
    match run.sender {
        SenderEnd::Failed => return ("sender-failed", None),
        SenderEnd::Deadline => return ("sender-deadline", None),
        SenderEnd::Exited => {}
    }
    let arrived = trusted
        .iter()
        .filter(|c| c.event == HookEvent::UserPromptSubmit)
        .nth(run.prompts_before);
    let Some(arrived) = arrived else {
        return ("no-prompt", None);
    };
    match arrived.field("prompt").and_then(harness_prefix) {
        Some(prefix) => ("harness", Some(prefix)),
        None => ("unprefixed", None),
    }
}

/// The first UserPromptSubmit whose raw start is a cross-session literal: the arrival `--record`
/// keeps as `cross-session-1`.
fn cross_session_arrival(trusted: &[Capture]) -> Option<&Capture> {
    trusted.iter().find(|c| {
        c.event == HookEvent::UserPromptSubmit
            && matches!(
                c.field("prompt").and_then(harness_prefix),
                Some("cross-session" | "cross-session-escaped")
            )
    })
}

/// One interactive run's first settled screen: its closed word and the closed facts the run held
/// at that settle — numbers and closed words only, no screen text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirstReading {
    pub screen: FirstScreen,
    /// From the spawn to the settle.
    pub settle_ms: u64,
    /// Every byte the child's terminal produced by the settle.
    pub bytes: u64,
    /// The child had not ended at the read.
    pub alive: bool,
    /// The child's exit status, only when it had ended by the read.
    pub exit: Option<u32>,
    /// Non-empty rows; `None` for a poisoned screen.
    pub rows: Option<usize>,
    pub box_any: bool,
    /// From the spawn to the last byte fed before the read; `None` when nothing was fed.
    pub last_byte_ms: Option<u64>,
    /// Per `BOX_LITERAL_IDS`, the first row the literal stands on.
    pub literals: [Option<usize>; 4],
    pub rule_above_prompt: bool,
    pub first_row: Option<RowFamily>,
    pub last_col: Option<usize>,
}

/// The run letters `measured`'s first-screen maps are keyed by, in run order.
pub const RUN_LETTERS: [&str; 6] = ["a", "b", "c", "d", "e", "f"];

fn first_maps(first: &[Option<FirstReading>; 6]) -> (Value, Value, Value) {
    let mut screens = Map::new();
    let mut settles = Map::new();
    let mut facts = Map::new();
    for (letter, reading) in RUN_LETTERS.iter().zip(first) {
        let key = (*letter).to_owned();
        screens.insert(key.clone(), json!(reading.map(|r| r.screen.as_str())));
        settles.insert(key.clone(), json!(reading.map(|r| r.settle_ms)));
        facts.insert(
            key,
            reading.map_or(Value::Null, |r| {
                let literals: Map<String, Value> = BOX_LITERAL_IDS
                    .iter()
                    .zip(r.literals)
                    .map(|(id, row)| ((*id).to_owned(), json!(row)))
                    .collect();
                json!({
                    "bytes": r.bytes,
                    "alive": r.alive,
                    "exit": r.exit,
                    "rows": r.rows,
                    "box_any": r.box_any,
                    "last_byte_ms": r.last_byte_ms,
                    "literals": literals,
                    "rule_above_prompt": r.rule_above_prompt,
                    "first_row": r.first_row.map(RowFamily::as_str),
                    "last_col": r.last_col,
                })
            }),
        );
    }
    (
        Value::Object(screens),
        Value::Object(settles),
        Value::Object(facts),
    )
}

/// The captures of the two dialog runs, each in claim order: Run C's and Run D's.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DialogRuns {
    pub questions: Vec<Capture>,
    pub plan: Vec<Capture>,
}

/// The one `claude agents --json` read Run B makes while its session is live: Run B's child pid
/// and the read's stdout, raw.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentsCapture {
    pub child_pid: Option<u32>,
    pub bytes: Vec<u8>,
}

/// Every measurement one `viola verify` run makes; `trusted` is Run B's captures in claim order,
/// `resumed` Run E's in claim order and `agents` Run B's agents read, when one was made.
#[derive(Debug, Clone, PartialEq)]
pub struct Probes {
    pub print: ProbeRun,
    pub typed: TypedRun,
    pub trusted: Vec<Capture>,
    pub dialogs: DialogRuns,
    pub resumed: Vec<Capture>,
    pub agents: Option<AgentsCapture>,
    /// Run B's first SessionStart hook child's env names, kept names already gone.
    pub env: EnvReading,
}

/// The identity-floor reading of one hook child's env-names document: whether it was read,
/// the `CLAUDE*` names it carried less every name the config keeps (`added`), those of `added`
/// on neither the floor nor the non-identity hook names (`unknown`), and its two counts. Names
/// and counts only.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EnvReading {
    pub read: bool,
    pub added: Vec<String>,
    pub unknown: Vec<String>,
    pub other_count: Option<u64>,
    pub unshaped: u64,
}

/// The reading of `env` (the capture arm's `Env.<k>.json`, parsed): `kept` (the config's
/// `claude_env_keep`) is removed from its names first, so a kept name reaches nothing after this.
/// A document of another shape reads unread.
pub fn env_reading(env: Option<&Value>, kept: &[String]) -> EnvReading {
    let Some(env) = env else {
        return EnvReading::default();
    };
    let names: Option<Vec<&str>> = env["claude_names"]
        .as_array()
        .and_then(|names| names.iter().map(Value::as_str).collect());
    let (Some(names), Some(other_count), Some(unshaped)) = (
        names,
        env["other_count"].as_u64(),
        env["claude_unshaped"].as_u64(),
    ) else {
        return EnvReading::default();
    };
    let added: Vec<String> = names
        .into_iter()
        .filter(|name| !kept.iter().any(|k| k == name))
        .map(str::to_owned)
        .collect();
    let unknown = added
        .iter()
        .filter(|name| {
            !crate::IDENTITY_FLOOR.contains(&name.as_str())
                && !crate::NON_IDENTITY_HOOK_NAMES.contains(&name.as_str())
        })
        .cloned()
        .collect();
    EnvReading {
        read: true,
        added,
        unknown,
        other_count: Some(other_count),
        unshaped,
    }
}

/// The recorded env variant's file name.
pub const ENV_RECORDED: &str = "Env.recorded-1.json";

/// The env reading as `--record` keeps it: the added names alone.
pub fn env_variant(env: &EnvReading) -> Option<Value> {
    env.read.then(|| json!({"v": 1, "added": env.added}))
}

/// What a stamp's `measured` holds beyond the typed run's timings: whether the two parallel
/// questions were both open before either was answered, the env reading, and the cross-session
/// row's outcome word and prefix id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Readings {
    pub parallel_both_before_first_post: Option<bool>,
    pub env: EnvReading,
    pub cross_session: (&'static str, Option<&'static str>),
}

impl Default for Readings {
    fn default() -> Self {
        Self {
            parallel_both_before_first_post: None,
            env: EnvReading::default(),
            cross_session: ("not-run", None),
        }
    }
}

impl Readings {
    pub fn of(probes: &Probes) -> Self {
        Self {
            parallel_both_before_first_post: parallel_both_before_first_post(
                &probes.dialogs.questions,
            ),
            env: probes.env.clone(),
            cross_session: cross_session_reading(&probes.trusted, &probes.typed),
        }
    }
}

/// The stamp's `cross_session_probe`: closed words and one number. `sender_tools` and
/// `send_result` read `none` where no file stood, and are null where the sender never started.
fn cross_session_block(reading: (&str, Option<&str>), typed: &TypedRun) -> Value {
    let run = typed.cross_session.as_ref();
    let tools = run.map(|r| {
        if r.sender_tools.is_empty() {
            json!("none")
        } else {
            json!(
                r.sender_tools
                    .iter()
                    .map(|w| w.as_str())
                    .collect::<Vec<_>>()
            )
        }
    });
    let result = run.map(|r| r.send_result.map_or("none", ResultWord::as_str));
    json!({
        "outcome": reading.0,
        "prefix": reading.1,
        "latency_ms": run.and_then(|r| r.latency_ms),
        "sender_tools": tools,
        "send_result": result,
    })
}

const ASK: &str = "AskUserQuestion";
const PLAN: &str = "ExitPlanMode";

/// The captures after the UserPromptSubmit that carries `prompt`, up to the next UserPromptSubmit;
/// empty when no capture carries it.
fn turn<'a>(captures: &'a [Capture], prompt: &str) -> &'a [Capture] {
    let is_prompt = |c: &Capture| c.event == HookEvent::UserPromptSubmit;
    let Some(start) = captures
        .iter()
        .position(|c| is_prompt(c) && c.field("prompt") == Some(prompt))
    else {
        return &[];
    };
    let rest = &captures[start + 1..];
    let end = rest.iter().position(is_prompt).unwrap_or(rest.len());
    &rest[..end]
}

/// The questions turn's first AskUserQuestion PreToolUse input and the `tool_response` of the
/// PostToolUse carrying its `tool_use_id`.
fn answered_question(questions: &[Capture]) -> Option<(&Value, &Value)> {
    let turn = turn(questions, DIALOG_PROMPT_QUESTIONS);
    let pre = turn.iter().find(|c| c.is(HookEvent::PreToolUse, ASK))?;
    let id = pre.field("tool_use_id")?;
    let post = turn
        .iter()
        .find(|c| c.is(HookEvent::PostToolUse, ASK) && c.field("tool_use_id") == Some(id))?;
    Some((
        &pre.payload.as_ref()?["tool_input"],
        &post.payload.as_ref()?["tool_response"],
    ))
}

/// The answered question's first label took effect, and no AskUserQuestion reached a
/// PermissionRequest in its own turn.
fn question_answered(questions: &[Capture]) -> bool {
    let took = answered_question(questions).is_some_and(|(input, response)| {
        let first = &input["questions"][0];
        let text = first["question"].as_str();
        let label = first["options"][0]["label"].as_str();
        text.zip(label)
            .is_some_and(|(text, label)| response["answers"][text] == label)
    });
    took && !turn(questions, DIALOG_PROMPT_QUESTIONS)
        .iter()
        .any(|c| c.is(HookEvent::PermissionRequest, ASK))
}

/// The last question turn: an AskUserQuestion PreToolUse, after it a PermissionRequest with the
/// same `tool_input`, the PostToolUse of the PreToolUse's `tool_use_id`, and in it every question's
/// first option label as its answer.
fn question_permission_answered(questions: &[Capture]) -> bool {
    let turn = turn(questions, DIALOG_PROMPT_PERMISSION_QUESTION);
    let Some(at) = turn.iter().position(|c| c.is(HookEvent::PreToolUse, ASK)) else {
        return false;
    };
    let pre = &turn[at];
    let (Some(input), Some(id)) = (
        pre.payload.as_ref().map(|p| &p["tool_input"]),
        pre.field("tool_use_id"),
    ) else {
        return false;
    };
    let raised = turn[at + 1..].iter().any(|c| {
        c.is(HookEvent::PermissionRequest, ASK)
            && c.payload
                .as_ref()
                .is_some_and(|p| p["tool_input"] == *input)
    });
    let post = turn
        .iter()
        .find(|c| c.is(HookEvent::PostToolUse, ASK) && c.field("tool_use_id") == Some(id));
    let answered = post
        .and_then(|c| c.payload.as_ref())
        .is_some_and(|p| first_labels_answered(input, &p["tool_response"]["answers"]));
    raised && answered
}

/// Every question of `input` answered with its first option label in `answers`.
fn first_labels_answered(input: &Value, answers: &Value) -> bool {
    input["questions"].as_array().is_some_and(|questions| {
        !questions.is_empty()
            && questions.iter().all(|q| {
                let text = q["question"].as_str();
                let label = q["options"][0]["label"].as_str();
                text.zip(label)
                    .is_some_and(|(text, label)| answers[text] == label)
            })
    })
}

/// The second question's answer is the free text and the first question carries the note.
fn notes_reached(questions: &[Capture]) -> bool {
    answered_question(questions).is_some_and(|(input, response)| {
        let first = input["questions"][0]["question"].as_str();
        let second = input["questions"][1]["question"].as_str();
        first.zip(second).is_some_and(|(first, second)| {
            response["answers"][second] == PROBE_FREE_TEXT
                && response["annotations"][first]["notes"] == PROBE_NOTE
        })
    })
}

/// Run D's ExitPlanMode captures read PreToolUse, PermissionRequest (the revise), PreToolUse (the
/// re-plan), then the PostToolUse of the re-plan's call and no PermissionRequest after it.
fn plan_took_effect(plan: &[Capture]) -> bool {
    let calls: Vec<&Capture> = plan
        .iter()
        .filter(|c| c.field("tool_name") == Some(PLAN))
        .collect();
    let [first, revised, again, rest @ ..] = calls.as_slice() else {
        return false;
    };
    let approved = again.field("tool_use_id").is_some_and(|id| {
        rest.iter()
            .any(|c| c.event == HookEvent::PostToolUse && c.field("tool_use_id") == Some(id))
    });
    first.event == HookEvent::PreToolUse
        && revised.event == HookEvent::PermissionRequest
        && again.event == HookEvent::PreToolUse
        && approved
        && !rest.iter().any(|c| c.event == HookEvent::PermissionRequest)
}

/// The parallel turn's AskUserQuestion PreToolUse ids, in claim order; `None` when one has no id.
fn parallel_calls(questions: &[Capture]) -> Option<Vec<&str>> {
    turn(questions, DIALOG_PROMPT_PARALLEL)
        .iter()
        .filter(|c| c.is(HookEvent::PreToolUse, ASK))
        .map(|c| c.field("tool_use_id"))
        .collect()
}

/// Exactly two parallel calls with distinct ids, each followed by its own PostToolUse.
fn both_parallel_answered(questions: &[Capture]) -> bool {
    let turn = turn(questions, DIALOG_PROMPT_PARALLEL);
    parallel_calls(questions).is_some_and(|ids| {
        ids.len() == 2
            && ids[0] != ids[1]
            && ids.iter().all(|id| {
                turn.iter().any(|c| {
                    c.is(HookEvent::PostToolUse, ASK) && c.field("tool_use_id") == Some(*id)
                })
            })
    })
}

/// Whether both parallel PreToolUse captures were claimed before either PostToolUse; `None` when the
/// turn did not hold exactly two.
pub fn parallel_both_before_first_post(questions: &[Capture]) -> Option<bool> {
    let turn = turn(questions, DIALOG_PROMPT_PARALLEL);
    let pres: Vec<usize> = turn
        .iter()
        .enumerate()
        .filter(|(_, c)| c.is(HookEvent::PreToolUse, ASK))
        .map(|(i, _)| i)
        .collect();
    if pres.len() != 2 {
        return None;
    }
    // A capture is never both a PreToolUse and a PostToolUse, so no PostToolUse sits at the second
    // PreToolUse's own index: "none before it" already says both were claimed first.
    let before_second = &turn[..pres[1]];
    Some(
        !before_second
            .iter()
            .any(|c| c.is(HookEvent::PostToolUse, ASK)),
    )
}

/// The index of the first UserPromptSubmit capture whose `prompt` normalises to `text`.
fn pasted(trusted: &[Capture], text: &str) -> Option<usize> {
    trusted.iter().position(|c| {
        c.event == HookEvent::UserPromptSubmit
            && c.field("prompt")
                .is_some_and(|prompt| prompt_text(prompt) == text)
    })
}

/// The index of the SessionStart the probed local command fired: the first one with source `clear`
/// captured after the tag-like turn's prompt.
fn clear_start(trusted: &[Capture]) -> Option<usize> {
    // The capture at the paste's own index is a UserPromptSubmit, never the SessionStart searched
    // for, so the search starts at it.
    let from = pasted(trusted, PROBE_TAG_PASTE)?;
    trusted[from..]
        .iter()
        .position(|c| c.event == HookEvent::SessionStart && c.field("source") == Some("clear"))
        .map(|at| from + at)
}

/// The probed local command started a session under a new, non-empty `session_id`, and no
/// UserPromptSubmit carried the command.
fn local_command_cleared(trusted: &[Capture]) -> bool {
    let first = trusted
        .iter()
        .find(|c| c.event == HookEvent::SessionStart)
        .and_then(|c| c.field("session_id"));
    let new_session = clear_start(trusted)
        .and_then(|at| trusted[at].field("session_id"))
        .is_some_and(|id| !id.is_empty() && Some(id) != first);
    new_session && pasted(trusted, PROBE_LOCAL_COMMAND).is_none()
}

/// The index of the SessionStart the probed `/compact` fired: the first one with source `compact`
/// captured after the newline turn's prompt.
fn compact_start(trusted: &[Capture]) -> Option<usize> {
    let from = pasted(trusted, PROBE_NEWLINE_PASTE)?;
    trusted[from..]
        .iter()
        .position(|c| c.event == HookEvent::SessionStart && c.field("source") == Some("compact"))
        .map(|at| from + at)
}

/// The probed `/compact` started its SessionStart under Run B's first, non-empty `session_id`
/// within the compact window, and no UserPromptSubmit carried the command.
fn local_command_compacted(trusted: &[Capture], typed: &TypedRun) -> bool {
    let first = trusted
        .iter()
        .find(|c| c.event == HookEvent::SessionStart)
        .and_then(|c| c.field("session_id"));
    let same_session = compact_start(trusted)
        .and_then(|at| trusted[at].field("session_id"))
        .is_some_and(|id| !id.is_empty() && Some(id) == first);
    same_session
        && pasted(trusted, PROBE_COMPACT_COMMAND).is_none()
        && within(typed.compact_latency_ms, COMPACT_CONFIRM_WINDOW)
}

/// The stem the cross-session arrival's recorded variant carries.
pub const CROSS_SESSION_STEM: &str = "cross-session-1";

/// Run B's captures named for `--record`: the long, the tag-like and the newline prompts as
/// `paste-1`, `paste-2` and `paste-3`, as `compact-1` the SessionStart the probed `/compact` fired,
/// as `cross-session-1` the first UserPromptSubmit opening a cross-session literal, and as `clear-1` the hooks the probed local command fired, the SessionEnd captured directly
/// before its SessionStart and that SessionStart. The exit's SessionEnd, captured after it, is not
/// one of them.
pub fn framing_variants(trusted: &[Capture]) -> Vec<(&'static str, &Capture)> {
    let [
        (long, long_stem),
        (tag, tag_stem),
        (newline, newline_stem),
        (_, compact_stem),
        (_, clear_stem),
    ] = FRAMING_TURNS;
    let mut out = Vec::new();
    for (text, stem) in [(long, long_stem), (tag, tag_stem), (newline, newline_stem)] {
        if let Some(at) = pasted(trusted, text) {
            out.push((stem, &trusted[at]));
        }
    }
    if let Some(at) = compact_start(trusted) {
        out.push((compact_stem, &trusted[at]));
    }
    if let Some(arrival) = cross_session_arrival(trusted) {
        out.push((CROSS_SESSION_STEM, arrival));
    }
    // The tag-like turn's prompt precedes it, so `at` is never 0.
    if let Some(at) = clear_start(trusted) {
        let ended = Some(&trusted[at - 1]).filter(|c| c.event == HookEvent::SessionEnd);
        out.extend(ended.map(|c| (clear_stem, c)));
        out.push((clear_stem, &trusted[at]));
    }
    out
}

/// The session Run E resumes: the `session_id` of Run B's first SessionStart, the session that holds
/// the probe turn and the pastes, only when it has a session id's shape.
pub fn resume_target(trusted: &[Capture]) -> Option<&str> {
    trusted
        .iter()
        .find(|c| c.event == HookEvent::SessionStart)
        .and_then(|c| c.field("session_id"))
        .filter(|id| crate::is_session_id(id))
}

/// Run E's first SessionStart, when it has one.
fn resumed_start(resumed: &[Capture]) -> Option<&Capture> {
    resumed.iter().find(|c| c.event == HookEvent::SessionStart)
}

/// Run E's first SessionStart has source `resume` and the id of the session it resumed.
fn session_resumed(trusted: &[Capture], resumed: &[Capture]) -> bool {
    let Some(target) = resume_target(trusted) else {
        return false;
    };
    resumed_start(resumed).is_some_and(|c| {
        c.field("source") == Some("resume") && c.field("session_id") == Some(target)
    })
}

/// The agents read lists exactly one row with Run B's child pid, and that row names the session of
/// Run B's first SessionStart and reads idle or busy.
fn agents_joined(trusted: &[Capture], agents: Option<&AgentsCapture>) -> bool {
    let Some(AgentsCapture {
        child_pid: Some(pid),
        bytes,
    }) = agents
    else {
        return false;
    };
    let first = trusted
        .iter()
        .find(|c| c.event == HookEvent::SessionStart)
        .and_then(|c| c.field("session_id"));
    let Ok(list) = crate::agents::parse(bytes) else {
        return false;
    };
    let mut own = list.rows.iter().filter(|row| row.pid == Some(*pid));
    match (own.next(), own.next(), first) {
        (Some(row), None, Some(first)) => {
            row.session_id == first
                && matches!(
                    row.status,
                    crate::agents::AgentStatus::Idle | crate::agents::AgentStatus::Busy
                )
        }
        _ => false,
    }
}

/// The recorded agents variant's file name.
pub const AGENTS_RECORDED: &str = "Agents.recorded-1.json";

/// The agents read kept for `--record`: one JSON array of the rows whose `pid` is Run B's child, as
/// listed. Every other session's row is left out: the read lists every live session on the host,
/// and their titles are user content. None when the read did not parse or holds no such row.
pub fn agents_variant(agents: &AgentsCapture) -> Option<Value> {
    let pid = agents.child_pid?;
    let list = crate::agents::parse(&agents.bytes).ok()?;
    let own: Vec<Value> = list
        .rows
        .into_iter()
        .filter(|row| row.pid == Some(pid))
        .map(|row| row.raw)
        .collect();
    (!own.is_empty()).then_some(Value::Array(own))
}

/// Run E's captures named for `--record`: its first SessionStart as `resume-1`.
pub fn resume_variants(resumed: &[Capture]) -> Vec<(&'static str, &Capture)> {
    resumed_start(resumed)
        .map(|c| ("resume-1", c))
        .into_iter()
        .collect()
}

fn any_row(rows: Option<&Vec<String>>, literals: &[&str]) -> bool {
    rows.is_some_and(|rows| rows.iter().any(|r| literals.iter().any(|l| r.contains(l))))
}

/// The input box up on the screen ([`crate::screen::Signatures::input_box_up`]).
fn input_box_ready(rows: Option<&Vec<String>>) -> bool {
    rows.is_some_and(|rows| SIGNATURES.input_box_up(rows))
}

fn within(ms: Option<u64>, bound: Duration) -> bool {
    ms.is_some_and(|ms| u128::from(ms) <= bound.as_millis())
}

/// `row`'s post-condition over the probes.
pub fn check(row: LedgerRow, probes: &Probes) -> bool {
    let (run, typed) = (&probes.print, &probes.typed);
    match row {
        LedgerRow::ShimResolution => run.version_answered,
        LedgerRow::SpineHooks => spine_in_order(&run.captures),
        LedgerRow::SessionStartFields => run.first(HookEvent::SessionStart).is_some_and(|p| {
            p["session_id"].as_str().is_some_and(|s| !s.is_empty()) && p["source"] == "startup"
        }),
        LedgerRow::PromptVerbatim => run
            .first(HookEvent::UserPromptSubmit)
            .is_some_and(|p| p["prompt"].as_str() == Some(PROBE_PROMPT)),
        LedgerRow::StopMessage => run
            .first(HookEvent::Stop)
            .is_some_and(|p| p["last_assistant_message"].is_string()),
        LedgerRow::LargestHookPayload => run.captures.iter().all(|c| fits_frame(c.bytes)),
        LedgerRow::ModalSignature => any_row(typed.modal.as_ref(), UNTRUSTED_START_MODALS),
        LedgerRow::InputBoxSignature => {
            input_box_ready(typed.ready.as_ref())
                && input_box_ready(typed.turn.as_ref())
                && input_box_ready(typed.long_turn.as_ref())
        }
        LedgerRow::QuietPeriod => {
            within(typed.ready_settle_ms, GATE_MAX_WAIT)
                && within(typed.turn_settle_ms, GATE_MAX_WAIT)
        }
        LedgerRow::ConfirmWindow => within(typed.prompt_latency_ms, CONFIRM_WINDOW_FALLBACK),
        LedgerRow::QuestionAnswer => question_answered(&probes.dialogs.questions),
        LedgerRow::PlanApproveRevise => plan_took_effect(&probes.dialogs.plan),
        LedgerRow::QuestionNotes => notes_reached(&probes.dialogs.questions),
        LedgerRow::DialogConcurrency => both_parallel_answered(&probes.dialogs.questions),
        LedgerRow::LongPasteWrapper => pasted(&probes.trusted, PROBE_LONG_PASTE).is_some(),
        LedgerRow::TagEscaping => pasted(&probes.trusted, PROBE_TAG_PASTE).is_some(),
        LedgerRow::LocalCommandClear => local_command_cleared(&probes.trusted),
        // A CLI that submitted at the LF leaves no capture equal to the whole text.
        LedgerRow::PasteNewline => pasted(&probes.trusted, PROBE_NEWLINE_PASTE).is_some(),
        LedgerRow::ResumeSession => session_resumed(&probes.trusted, &probes.resumed),
        LedgerRow::AgentsJoin => agents_joined(&probes.trusted, probes.agents.as_ref()),
        LedgerRow::LocalCommandCompact => local_command_compacted(&probes.trusted, typed),
        LedgerRow::LocalCommandEcho => {
            pasted(&probes.trusted, PROBE_ECHO_COMMAND).is_none()
                && within(typed.echo_latency_ms, CONFIRM_WINDOW_FALLBACK)
        }
        LedgerRow::TrustRecord => trust_record_opened(typed),
        LedgerRow::BrowserToolsPrompt => browser_tools_met(typed).is_some(),
        LedgerRow::LocalCommandContext => {
            pasted(&probes.trusted, PROBE_CONTEXT_COMMAND).is_none()
                && within(typed.context_latency_ms, CONFIRM_WINDOW_FALLBACK)
        }
        LedgerRow::StopSessionCrons => run
            .first(HookEvent::Stop)
            .is_some_and(|p| p["session_crons"].is_array()),
        LedgerRow::PermissionRequestQuestion => {
            question_permission_answered(&probes.dialogs.questions)
        }
        LedgerRow::IdentityFloor => {
            let env = &probes.env;
            env.read && env.unknown.is_empty() && env.unshaped == 0
        }
        LedgerRow::CrossSessionPrefix => {
            cross_session_reading(&probes.trusted, typed).0 == "harness"
        }
    }
}

/// Whether some interactive run's first screen was the browser-tools prompt; `None` when any run's
/// first screen read `none` (an unknown screen, a reworded prompt among them), which never reads
/// as the prompt being absent. A run with no reading is skipped. Nothing was typed into the
/// prompt by construction: verify types only into a screen `input_box_up` accepts.
fn browser_tools_met(typed: &TypedRun) -> Option<bool> {
    let screens = || typed.first.iter().flatten().map(|r| r.screen);
    if screens().any(|s| s == FirstScreen::None) {
        return None;
    }
    Some(screens().any(|s| s == FirstScreen::BrowserTools))
}

/// Whether an interactive run's first screen was the browser-tools prompt: verify then says so
/// before its summary. False on every other reading, an unreadable first screen included.
pub fn browser_tools_stood(typed: &TypedRun) -> bool {
    browser_tools_met(typed) == Some(true)
}

/// Run F's record was written, Run A (its untrusted twin) showed a modal literal, and Run F's first
/// settled screen holds no modal literal and the input box up: the record alone opened the folder.
fn trust_record_opened(typed: &TypedRun) -> bool {
    typed.trust_write == Some(TrustWord::Written)
        && any_row(typed.modal.as_ref(), UNTRUSTED_START_MODALS)
        && !any_row(typed.trust_frame.as_ref(), SIGNATURES.modals)
        && input_box_ready(typed.trust_frame.as_ref())
}

/// A row's stamped word, the one source of it for the stamp and verify's step line: `pass`, or
/// `fail`; the browser-tools row that holds reads `absent` when no run met the prompt, and the
/// trust-record row that wrote nothing reads the writer's own closed word instead, never `pass`.
pub fn row_word(row: LedgerRow, pass: bool, typed: &TypedRun) -> &'static str {
    match (row, pass, typed.trust_write) {
        (LedgerRow::BrowserToolsPrompt, true, _) if browser_tools_met(typed) == Some(false) => {
            "absent"
        }
        (_, true, _) => "pass",
        (LedgerRow::TrustRecord, false, Some(word)) if word != TrustWord::Written => word.as_str(),
        _ => "fail",
    }
}

/// Exactly the four spine events, one each, in firing order, each payload naming its event.
fn spine_in_order(captures: &[Capture]) -> bool {
    captures.len() == CAPTURE_EVENTS.len()
        && captures.iter().zip(CAPTURE_EVENTS).all(|(c, event)| {
            c.event == event
                && c.payload
                    .as_ref()
                    .is_some_and(|p| p["hook_event_name"].as_str() == Some(event_name(event)))
        })
}

fn fits_frame(bytes: usize) -> bool {
    u64::try_from(bytes).is_ok_and(|b| b <= MAX_FRAME)
}

/// The largest payload per event, in first-arrival order: the largest-hook-payload measurement.
pub fn largest(captures: &[Capture]) -> Vec<(HookEvent, usize)> {
    let mut out: Vec<(HookEvent, usize)> = Vec::new();
    for c in captures {
        match out.iter_mut().find(|(e, _)| *e == c.event) {
            Some((_, bytes)) => *bytes = (*bytes).max(c.bytes),
            None => out.push((c.event, c.bytes)),
        }
    }
    out
}

/// The stamps envelope's parts every reader relies on: an object, `data` an object, `versions`
/// an object; each absent part is empty.
fn stamp_shape_ok(doc: &Map<String, Value>) -> bool {
    match doc.get("data") {
        None => true,
        Some(Value::Object(data)) => data.get("versions").is_none_or(Value::is_object),
        Some(_) => false,
    }
}

/// The stamps with `version`'s entry replaced by this run's rows and measurements (the largest hook
/// payloads, the typed probe's timings, and whether the two parallel questions were both open
/// before either was answered); every other
/// version and every unknown field is kept. Existing bytes of the wrong shape are replaced whole.
pub fn merge_stamp(
    existing: Option<&[u8]>,
    version: &str,
    results: &[(LedgerRow, bool)],
    largest: &[(HookEvent, usize)],
    typed: &TypedRun,
    readings: &Readings,
    written_at: &str,
) -> Vec<u8> {
    let mut doc = match existing.and_then(|b| serde_json::from_slice(b).ok()) {
        Some(Value::Object(doc)) if stamp_shape_ok(&doc) => doc,
        _ => Map::new(),
    };
    let rows: Map<String, Value> = results
        .iter()
        .map(|(row, pass)| (row.id().to_owned(), json!(row_word(*row, *pass, typed))))
        .collect();
    let measured: Map<String, Value> = largest
        .iter()
        .map(|(event, bytes)| (event_name(*event).to_owned(), json!(bytes)))
        .collect();
    let (first_screen, first_settle_ms, first_facts) = first_maps(&typed.first);
    let entry = json!({
        "verified_at": written_at,
        "rows": rows,
        "measured": {
            "largest_hook_payload": measured,
            "typed_probe": {
                "ready_settle_ms": typed.ready_settle_ms,
                "turn_settle_ms": typed.turn_settle_ms,
                "prompt_latency_ms": typed.prompt_latency_ms,
                "max_turn_gap_ms": typed.max_turn_gap_ms,
                "compact_latency_ms": typed.compact_latency_ms,
                "echo_latency_ms": typed.echo_latency_ms,
                "context_latency_ms": typed.context_latency_ms,
                "box_only_guards": typed.box_only_guards,
            },
            "dialog_probe": {
                "parallel_both_before_first_post": readings.parallel_both_before_first_post,
            },
            "trust_probe": {
                "write": typed.trust_write.map(TrustWord::as_str),
                "remove": typed.trust_remove.map(TrustWord::as_str),
            },
            "first_screen": first_screen,
            "first_settle_ms": first_settle_ms,
            "first_facts": first_facts,
            "env_probe": {
                "read": readings.env.read,
                "added": readings.env.added,
                "unknown": readings.env.unknown,
                "other_count": readings.env.other_count,
                "unshaped": readings.env.unshaped,
            },
            "cross_session_probe": cross_session_block(readings.cross_session, typed),
        },
    });
    doc.insert("v".to_owned(), json!(1));
    doc.insert("written_at".to_owned(), json!(written_at));
    doc.insert("writer".to_owned(), json!("verify"));
    let data = doc.entry("data").or_insert_with(|| json!({}));
    if let Some(data) = data.as_object_mut() {
        let versions = data.entry("versions").or_insert_with(|| json!({}));
        if let Some(versions) = versions.as_object_mut() {
            versions.insert(version.to_owned(), entry);
        }
    }
    Value::Object(doc).to_string().into_bytes()
}

/// A tool call of a dialog turn: its `tool_name` and its `tool_use_id`.
type Call<'a> = (Option<&'a str>, Option<&'a str>);

/// The 1-based number of the call `capture` joins among the turn's `calls` so far, opening one
/// where `dialog_variants` says so; `None` for a capture that joins none.
fn joined_call<'a>(calls: &mut Vec<Call<'a>>, capture: &'a Capture) -> Option<usize> {
    let tool = capture.field("tool_name");
    let id = capture.field("tool_use_id");
    match capture.event {
        HookEvent::PreToolUse => {
            calls.push((tool, id));
            Some(calls.len())
        }
        HookEvent::PermissionRequest => match calls.iter().rposition(|(t, _)| *t == tool) {
            Some(i) => Some(i + 1),
            None => {
                calls.push((tool, None));
                Some(calls.len())
            }
        },
        HookEvent::PostToolUse => calls
            .iter()
            .position(|(_, i)| id.is_some() && *i == id)
            .or_else(|| calls.iter().rposition(|(t, i)| *t == tool && i.is_none()))
            .map(|i| i + 1),
        _ => None,
    }
}

/// Each dialog run's captures named for `--record`: `<stem>-<n>`, `n` counting the turn's tool calls
/// in claim order. A PreToolUse opens a call; a PermissionRequest joins the latest call of its tool,
/// else opens one; a PostToolUse joins the call of its `tool_use_id`, else the latest id-less call
/// of its tool, else is not recorded. Only the first capture of each event and variant is kept.
pub fn dialog_variants(dialogs: &DialogRuns) -> Vec<(String, &Capture)> {
    let mut out: Vec<(String, &Capture)> = Vec::new();
    for (run, prompt, stem) in DIALOG_TURNS {
        let captures = match run {
            DialogRun::Questions => &dialogs.questions,
            DialogRun::Plan => &dialogs.plan,
        };
        let mut calls: Vec<Call<'_>> = Vec::new();
        for capture in turn(captures, prompt) {
            let Some(n) = joined_call(&mut calls, capture) else {
                continue;
            };
            let variant = format!("{stem}-{n}");
            if !out
                .iter()
                .any(|(v, c)| *v == variant && c.event == capture.event)
            {
                out.push((variant, capture));
            }
        }
    }
    out
}

/// Whether a stamped `word` counts toward verified: `pass`, or `absent` on the browser-tools row
/// alone.
pub fn counts(row: LedgerRow, word: &str) -> bool {
    word == "pass" || (row == LedgerRow::BrowserToolsPrompt && word == "absent")
}

/// `true` only when every row relied on ([`LedgerRow::relied`]) [`counts`] under `version`. Bytes that are not a `v:1`
/// envelope of the documented shape are `StampsMalformed`; an absent version is simply unverified.
pub fn verified(stamps: &[u8], version: &str) -> Result<bool, AgentError> {
    let doc: Value = serde_json::from_slice(stamps).map_err(|_| AgentError::StampsMalformed)?;
    let shaped = doc
        .as_object()
        .is_some_and(|d| d.get("v").and_then(Value::as_u64) == Some(1) && stamp_shape_ok(d));
    if !shaped {
        return Err(AgentError::StampsMalformed);
    }
    let rows = &doc["data"]["versions"][version]["rows"];
    Ok(LedgerRow::relied()
        .iter()
        .all(|row| rows[row.id()].as_str().is_some_and(|w| counts(*row, w))))
}

/// `true` when `version`'s entry holds every row relied on ([`LedgerRow::relied`]), whatever each
/// word reads: the row set was tried on that version. An id no longer relied on changes nothing.
/// Bytes that are not a `v:1` envelope of the documented shape are `StampsMalformed`, as in
/// [`verified`].
pub fn attempted(stamps: &[u8], version: &str) -> Result<bool, AgentError> {
    let doc: Value = serde_json::from_slice(stamps).map_err(|_| AgentError::StampsMalformed)?;
    let shaped = doc
        .as_object()
        .is_some_and(|d| d.get("v").and_then(Value::as_u64) == Some(1) && stamp_shape_ok(d));
    if !shaped {
        return Err(AgentError::StampsMalformed);
    }
    let rows = &doc["data"]["versions"][version]["rows"];
    Ok(LedgerRow::relied()
        .iter()
        .all(|row| rows[row.id()].as_str().is_some()))
}

/// A recorded payload with the home rewritten to `~` (both separator spellings) and `user` as a
/// whole word rewritten to `<user>`, in every string and key.
pub fn scrub(value: &Value, home: &str, user: &str) -> Value {
    match value {
        Value::String(s) => Value::String(scrub_text(s, home, user)),
        Value::Array(items) => Value::Array(items.iter().map(|v| scrub(v, home, user)).collect()),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(k, v)| (scrub_text(k, home, user), scrub(v, home, user)))
                .collect(),
        ),
        other => other.clone(),
    }
}

fn scrub_text(text: &str, home: &str, user: &str) -> String {
    let mut out = text.to_owned();
    if !home.is_empty() {
        for spelling in [home.replace('\\', "/"), home.replace('/', "\\")] {
            out = out.replace(&spelling, "~");
        }
    }
    if !user.is_empty() {
        out = replace_word(&out, user, "<user>");
    }
    out
}

fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// `word` wherever it stands whole (no word character on either side), case folded.
fn replace_word(text: &str, word: &str, with: &str) -> String {
    let folded = text.to_ascii_lowercase();
    let word = word.to_ascii_lowercase();
    let mut out = String::new();
    let mut last = 0;
    for (at, _) in folded.match_indices(&word) {
        let end = at + word.len();
        let before = text[..at].chars().next_back().is_none_or(|c| !is_word(c));
        let after = text[end..].chars().next().is_none_or(|c| !is_word(c));
        if before && after {
            out.push_str(&text[last..at]);
            out.push_str(with);
            last = end;
        }
    }
    out.push_str(&text[last..]);
    out
}

/// Why a recorded text is not clean: a closed code, never the text itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unclean {
    HomePath,
    AbsolutePath,
    Username,
    Email,
}

impl Unclean {
    pub const fn code(self) -> &'static str {
        match self {
            Self::HomePath => "home-path",
            Self::AbsolutePath => "absolute-path",
            Self::Username => "username",
            Self::Email => "email",
        }
    }
}

/// The first string or key of `value` holding a drive-letter path, a `/home/`, `/Users/` or
/// `\Users\` path, `user` as a whole word, or an email-shaped token: the checks the
/// committed-fixture hygiene walk applies.
pub fn unclean(value: &Value, user: &str) -> Option<Unclean> {
    let mut texts = Vec::new();
    strings(value, &mut texts);
    texts.iter().find_map(|s| text_fault(s, user))
}

/// An env-names document's first name holding `user` as one of its `_`-joined parts, case folded:
/// [`unclean`] reads a word with `_` on either side as no whole word, and a `CLAUDE_*` name joins
/// every part so.
pub fn env_names_fault(value: &Value, user: &str) -> Option<Unclean> {
    if user.is_empty() || user == "<user>" {
        return None;
    }
    let user = user.to_lowercase();
    let names = value["added"].as_array()?;
    names
        .iter()
        .filter_map(Value::as_str)
        .any(|name| format!("_{}_", name.to_lowercase()).contains(&format!("_{user}_")))
        .then_some(Unclean::Username)
}

/// No string or key of `value` is [`unclean`].
pub fn is_clean(value: &Value, user: &str) -> bool {
    unclean(value, user).is_none()
}

fn text_fault(text: &str, user: &str) -> Option<Unclean> {
    if has_absolute_path(text) {
        Some(Unclean::AbsolutePath)
    } else if has_user_word(text, user) {
        Some(Unclean::Username)
    } else if has_email(text) {
        Some(Unclean::Email)
    } else {
        None
    }
}

fn strings<'a>(value: &'a Value, out: &mut Vec<&'a str>) {
    match value {
        Value::String(s) => out.push(s),
        Value::Array(items) => items.iter().for_each(|v| strings(v, out)),
        Value::Object(map) => map.iter().for_each(|(k, v)| {
            out.push(k);
            strings(v, out);
        }),
        _ => {}
    }
}

fn has_absolute_path(text: &str) -> bool {
    let b = text.as_bytes();
    let drive =
        b.len() >= 3 && b[0].is_ascii_alphabetic() && b[1] == b':' && matches!(b[2], b'\\' | b'/');
    drive || text.contains("/home/") || text.contains("/Users/") || text.contains("\\Users\\")
}

fn has_user_word(text: &str, user: &str) -> bool {
    if user.is_empty() || user == "<user>" {
        return false;
    }
    let hay = text.to_lowercase();
    let needle = user.to_lowercase();
    hay.match_indices(&needle).any(|(at, m)| {
        let before = hay[..at].chars().next_back().is_none_or(|c| !is_word(c));
        let after = hay[at + m.len()..]
            .chars()
            .next()
            .is_none_or(|c| !is_word(c));
        before && after
    })
}

/// What a recorded `Screen.<phase>.json` keeps: the box's rule as its run of the rule character
/// alone, the prompt row right below it as the prompt glyph alone (never the text after it), a row
/// holding a [`SIGNATURES`] literal whole, and every other row `""`.
pub fn signature_rows(rows: &[String]) -> Vec<String> {
    rows.iter()
        .enumerate()
        .map(|(i, r)| {
            let below_rule = i
                .checked_sub(1)
                .and_then(|j| rows.get(j))
                .is_some_and(|above| SIGNATURES.is_box_rule(above));
            if SIGNATURES.is_box_rule(r) {
                r.trim_end_matches(' ').to_owned()
            } else if below_rule && r.starts_with(SIGNATURES.box_prompt) {
                SIGNATURES.box_prompt.to_string()
            } else if SIGNATURES.holds_any(r) {
                r.clone()
            } else {
                String::new()
            }
        })
        .collect()
}

/// Where a screen first fails its check: the kept row, whether its seam with a live neighbour (not
/// the row alone) holds it, and why.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenFault {
    pub row: usize,
    pub seam: bool,
    pub why: Unclean,
}

/// The first kept (non-empty) row that, alone or at its seam with the live row above or below it,
/// holds the home (what the scrub would rewrite), anything [`unclean`] refuses, or an email-shaped
/// token. `kept` is `signature_rows(live_rows)`.
pub fn screen_fault(
    live_rows: &[String],
    kept: &[String],
    home: &str,
    user: &str,
) -> Option<ScreenFault> {
    for (i, row) in kept.iter().enumerate() {
        if row.is_empty() {
            continue;
        }
        let mut texts = vec![(false, row.clone())];
        if let Some(above) = i.checked_sub(1).and_then(|j| live_rows.get(j)) {
            texts.push((true, seam(above, row)));
        }
        if let Some(below) = live_rows.get(i + 1) {
            texts.push((true, seam(row, below)));
        }
        for (seam, text) in texts {
            if let Some(why) = screen_text_fault(&text, home, user) {
                return Some(ScreenFault { row: i, seam, why });
            }
        }
    }
    None
}

/// No kept row or seam of the screen has a [`screen_fault`].
pub fn screen_is_clean(live_rows: &[String], kept: &[String], home: &str, user: &str) -> bool {
    screen_fault(live_rows, kept, home, user).is_none()
}

fn screen_text_fault(text: &str, home: &str, user: &str) -> Option<Unclean> {
    if scrub_text(text, home, "") != text {
        return Some(Unclean::HomePath);
    }
    text_fault(text, user)
}

fn is_seam_char(c: char) -> bool {
    c == ' ' || ('\u{2500}'..='\u{257f}').contains(&c)
}

/// `upper` then `lower` joined with spaces and box-drawing characters trimmed at the join.
fn seam(upper: &str, lower: &str) -> String {
    format!(
        "{}{}",
        upper.trim_end_matches(is_seam_char),
        lower.trim_start_matches(is_seam_char)
    )
}

fn is_local_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || "._%+-".contains(c)
}

/// `local@domain.tld`: a local part, then a domain holding a dot followed by two or more letters.
fn has_email(text: &str) -> bool {
    text.match_indices('@').any(|(at, _)| {
        let local = text[..at].chars().next_back().is_some_and(is_local_char);
        let domain: String = text[at + 1..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '.' || *c == '-')
            .collect();
        let tld = domain.rsplit_once('.').is_some_and(|(host, tld)| {
            !host.is_empty() && tld.len() >= 2 && tld.chars().all(|c| c.is_ascii_alphabetic())
        });
        local && tld
    })
}

// cargo-modules reads an out-of-line test module as an orphan unless its file stem begins `test_`.
#[cfg(test)]
#[path = "ledger/test_cases.rs"]
mod tests;
