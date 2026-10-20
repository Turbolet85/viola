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
use crate::hook::{HookEvent, prompt_text};
use crate::screen::{COMPACT_CONFIRM_WINDOW, CONFIRM_WINDOW_FALLBACK, GATE_MAX_WAIT, SIGNATURES};
use crate::statusline::WRAPPED_HERE;

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
    StatusLineOverride,
    StatusLineShell,
    StatusLineRateLimits,
    ResumeSession,
    AgentsJoin,
    LocalCommandCompact,
}

impl LedgerRow {
    pub const ALL: [Self; 24] = [
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
        Self::StatusLineOverride,
        Self::StatusLineShell,
        Self::StatusLineRateLimits,
        Self::ResumeSession,
        Self::AgentsJoin,
        Self::LocalCommandCompact,
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
            Self::StatusLineOverride => "statusline-override",
            Self::StatusLineShell => "statusline-shell",
            Self::StatusLineRateLimits => "statusline-rate-limits",
            Self::ResumeSession => "resume-session",
            Self::AgentsJoin => "agents-join",
            Self::LocalCommandCompact => "local-command-compact",
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
            Self::StatusLineOverride => "a --settings statusLine runs in the status line's place",
            Self::StatusLineShell => "a statusline command runs through /bin/sh",
            Self::StatusLineRateLimits => {
                "the statusline payload carries two whole-number rate_limits windows"
            }
            Self::ResumeSession => "--resume <id> reopens the session under that id",
            Self::AgentsJoin => "claude agents --json names the live session by pid and session id",
            Self::LocalCommandCompact => "/compact keeps the session id and submits no prompt",
        }
    }

    /// Whether viola relies on this row where the status line is `wrapped` or not: the three
    /// statusline rows only where it is.
    const fn relied_on(self, wrapped: bool) -> bool {
        match self {
            Self::StatusLineOverride | Self::StatusLineShell | Self::StatusLineRateLimits => {
                wrapped
            }
            _ => true,
        }
    }

    /// The rows relied on this OS, in [`LedgerRow::ALL`] order: what `viola verify` checks and the
    /// stamp verdict reads.
    pub fn relied() -> Vec<Self> {
        Self::ALL
            .into_iter()
            .filter(|r| r.relied_on(WRAPPED_HERE))
            .collect()
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
}

/// The known built-in local commands, which fire no UserPromptSubmit, each with its delivery
/// post-condition or none (architecture [Delivery Confirmation]).
pub const LOCAL_COMMANDS: [(&str, Option<PostCondition>); 3] = [
    ("/clear", Some(PostCondition::NewSession)),
    ("/remote-control", None),
    ("/compact", Some(PostCondition::CompactedSession)),
];
/// The local command Run B pastes last: the list's `/clear` entry.
pub const PROBE_LOCAL_COMMAND: &str = LOCAL_COMMANDS[0].0;
/// The local command Run B pastes before `/clear`: the list's `/compact` entry.
pub const PROBE_COMPACT_COMMAND: &str = LOCAL_COMMANDS[2].0;

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
/// The plan run's prompt, in plan mode.
pub const PLAN_PROMPT: &str = "viola verify probe: make a plan only, do not run anything. The plan \
     has one step: reply with the word done. Present the plan with the ExitPlanMode tool. If the plan \
     is sent back, revise it once as asked and present it again with ExitPlanMode.";

/// The dialog run's `--settings`: a session-scoped `ask` rule, which outranks a user `allow`.
pub const DIALOG_SETTINGS: &str =
    r#"{"permissions":{"ask":["Bash(touch viola-probe-permission)"]}}"#;

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
pub const DIALOG_TURNS: [(DialogRun, &str, &str); 4] = [
    (DialogRun::Questions, DIALOG_PROMPT_QUESTIONS, "questions"),
    (DialogRun::Questions, DIALOG_PROMPT_PARALLEL, "parallel"),
    (DialogRun::Questions, DIALOG_PROMPT_PERMISSION, "permission"),
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
}

impl ProbeAnswer {
    pub const ALL: [Self; 5] = [
        Self::QuestionsFirstAndFreeText,
        Self::QuestionFirstOption,
        Self::PlanRevise,
        Self::PlanApprove,
        Self::PermitAllow,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::QuestionsFirstAndFreeText => "questions-first-and-free-text",
            Self::QuestionFirstOption => "question-first-option",
            Self::PlanRevise => "plan-revise",
            Self::PlanApprove => "plan-approve",
            Self::PermitAllow => "permit-allow",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|a| a.id() == id)
    }
}

/// Which answer goes to which dialog event: (run, event, ordinal of that event in the run). Run D's
/// first PreToolUse is left unanswered, so the CLI raises the plan's PermissionRequest.
pub const PROBE_ANSWERS: [(DialogRun, HookEvent, u32, ProbeAnswer); 6] = [
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
        ProbeAnswer::QuestionsFirstAndFreeText | ProbeAnswer::QuestionFirstOption => {
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
    Dialog { answers_dir_fwd: &'a str },
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
    let mut hooks = Map::new();
    let dialog = [
        HookEvent::PreToolUse,
        HookEvent::PermissionRequest,
        HookEvent::PostToolUse,
    ];
    let events = match kind {
        ProbePlugin::Spine => &dialog[..0],
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

/// `StatusLine.<k>.json`: one status line payload the probe command filed.
pub fn statusline_capture_file_name(k: u32) -> String {
    format!("{STATUSLINE_STEM}.{k}.json")
}

/// `StatusLine.<k>.shell`: the shell verdict filed beside payload `k`.
pub fn statusline_shell_file_name(k: u32) -> String {
    format!("{STATUSLINE_STEM}.{k}.shell")
}

const STATUSLINE_STEM: &str = "StatusLine";

/// The `k` of a `StatusLine.<k>.json` name; any other name has none.
pub fn parse_statusline_capture_file_name(name: &str) -> Option<u32> {
    let k = name
        .strip_suffix(".json")?
        .strip_prefix(STATUSLINE_STEM)?
        .strip_prefix('.')?;
    k.parse().ok().filter(|k| *k > 0)
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

/// What one probe measured: the resolved program, whether it answered `--version`, and the
/// captures in arrival order.
#[derive(Debug, Clone, PartialEq)]
pub struct ProbeRun {
    pub program_is_script: bool,
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
}

/// The captures of the two dialog runs, each in claim order: Run C's and Run D's.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DialogRuns {
    pub questions: Vec<Capture>,
    pub plan: Vec<Capture>,
}

/// Which shell ran a statusline command: the word the CLI's shell expanded `"$0"` to was exactly
/// [`crate::statusline::BIN_SH`], or anything else (no word included).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellVerdict {
    BinSh,
    Other,
}

impl ShellVerdict {
    /// The word a capture's shell file holds.
    pub const fn word(self) -> &'static str {
        match self {
            Self::BinSh => "bin-sh",
            Self::Other => "other",
        }
    }

    /// A shell file's bytes: exactly `bin-sh` is [`ShellVerdict::BinSh`], anything else `Other`.
    pub fn read(bytes: &[u8]) -> Self {
        if bytes == Self::BinSh.word().as_bytes() {
            Self::BinSh
        } else {
            Self::Other
        }
    }
}

/// The one `claude agents --json` read Run B makes while its session is live: Run B's child pid
/// and the read's stdout, raw.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentsCapture {
    pub child_pid: Option<u32>,
    pub bytes: Vec<u8>,
}

/// One payload Run B's status line handed the probe command, as filed, and the shell that ran it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusLineCapture {
    pub bytes: Vec<u8>,
    pub shell: ShellVerdict,
}

/// Every measurement one `viola verify` run makes; `trusted` is Run B's captures in claim order,
/// `statusline` its status line's in `k` order, `resumed` Run E's in claim order and `agents` Run
/// B's agents read, when one was made.
#[derive(Debug, Clone, PartialEq)]
pub struct Probes {
    pub print: ProbeRun,
    pub typed: TypedRun,
    pub trusted: Vec<Capture>,
    pub dialogs: DialogRuns,
    pub statusline: Vec<StatusLineCapture>,
    pub resumed: Vec<Capture>,
    pub agents: Option<AgentsCapture>,
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
/// PermissionRequest anywhere in the run.
fn question_answered(questions: &[Capture]) -> bool {
    let took = answered_question(questions).is_some_and(|(input, response)| {
        let first = &input["questions"][0];
        let text = first["question"].as_str();
        let label = first["options"][0]["label"].as_str();
        text.zip(label)
            .is_some_and(|(text, label)| response["answers"][text] == label)
    });
    took && !questions
        .iter()
        .any(|c| c.is(HookEvent::PermissionRequest, ASK))
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

/// Run B's captures named for `--record`: the long, the tag-like and the newline prompts as
/// `paste-1`, `paste-2` and `paste-3`, as `compact-1` the SessionStart the probed `/compact` fired,
/// and as `clear-1` the hooks the probed local command fired, the SessionEnd captured directly
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

/// An input-box literal on some row and no modal literal on any.
fn input_box_ready(rows: Option<&Vec<String>>) -> bool {
    any_row(rows, SIGNATURES.input_box) && !any_row(rows, SIGNATURES.modals)
}

fn within(ms: Option<u64>, bound: Duration) -> bool {
    ms.is_some_and(|ms| u128::from(ms) <= bound.as_millis())
}

/// `row`'s post-condition over the probes.
pub fn check(row: LedgerRow, probes: &Probes) -> bool {
    let (run, typed) = (&probes.print, &probes.typed);
    match row {
        LedgerRow::ShimResolution => !run.program_is_script && run.version_answered,
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
        LedgerRow::ModalSignature => any_row(typed.modal.as_ref(), SIGNATURES.modals),
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
        LedgerRow::StatusLineOverride => !probes.statusline.is_empty(),
        LedgerRow::StatusLineShell => {
            !probes.statusline.is_empty()
                && probes
                    .statusline
                    .iter()
                    .all(|c| c.shell == ShellVerdict::BinSh)
        }
        LedgerRow::StatusLineRateLimits => probes
            .statusline
            .iter()
            .any(|c| whole_rate_limits(&c.bytes)),
        LedgerRow::ResumeSession => session_resumed(&probes.trusted, &probes.resumed),
        LedgerRow::AgentsJoin => agents_joined(&probes.trusted, probes.agents.as_ref()),
        LedgerRow::LocalCommandCompact => local_command_compacted(&probes.trusted, typed),
    }
}

/// One JSON object whose `rate_limits` holds `five_hour` and `seven_day`, each with a whole-number
/// `used_percentage` from 0 to 100 and a whole non-negative `resets_at`: the shape 2.1.287 sends,
/// narrower than what `statusline::reading` tolerates.
fn whole_rate_limits(bytes: &[u8]) -> bool {
    let Ok(Value::Object(payload)) = serde_json::from_slice::<Value>(bytes) else {
        return false;
    };
    let whole = |window: &Value| {
        window.is_object()
            && window["used_percentage"].as_u64().is_some_and(|p| p <= 100)
            && window["resets_at"].as_u64().is_some()
    };
    payload
        .get("rate_limits")
        .is_some_and(|limits| whole(&limits["five_hour"]) && whole(&limits["seven_day"]))
}

/// The recorded status line payload of a session before its first response.
pub const STATUSLINE_DEFAULT: &str = "StatusLine.default-1.json";
/// The recorded status line payload that carries both `rate_limits` windows.
pub const STATUSLINE_RATE_LIMITS: &str = "StatusLine.rate-limits-1.json";

/// Run B's status line payloads named for `--record`: [`STATUSLINE_DEFAULT`] is the first whose
/// object holds no `rate_limits`, [`STATUSLINE_RATE_LIMITS`] the first the rate-limits row passes.
pub fn statusline_variants(captures: &[StatusLineCapture]) -> Vec<(String, &[u8])> {
    let without = captures.iter().find(|c| {
        matches!(
            serde_json::from_slice::<Value>(&c.bytes),
            Ok(Value::Object(payload)) if !payload.contains_key("rate_limits")
        )
    });
    let with = captures.iter().find(|c| whole_rate_limits(&c.bytes));
    [
        (STATUSLINE_DEFAULT, without),
        (STATUSLINE_RATE_LIMITS, with),
    ]
    .into_iter()
    .filter_map(|(name, c)| c.map(|c| (name.to_owned(), c.bytes.as_slice())))
    .collect()
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
    parallel_both_before_first_post: Option<bool>,
    written_at: &str,
) -> Vec<u8> {
    let mut doc = match existing.and_then(|b| serde_json::from_slice(b).ok()) {
        Some(Value::Object(doc)) if stamp_shape_ok(&doc) => doc,
        _ => Map::new(),
    };
    let rows: Map<String, Value> = results
        .iter()
        .map(|(row, pass)| {
            (
                row.id().to_owned(),
                json!(if *pass { "pass" } else { "fail" }),
            )
        })
        .collect();
    let measured: Map<String, Value> = largest
        .iter()
        .map(|(event, bytes)| (event_name(*event).to_owned(), json!(bytes)))
        .collect();
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
            },
            "dialog_probe": {
                "parallel_both_before_first_post": parallel_both_before_first_post,
            },
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

/// `true` only when every row relied on this OS ([`LedgerRow::relied`]) reads `"pass"` under
/// `version`. Bytes that are not a `v:1`
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
        .all(|row| rows[row.id()] == "pass"))
}

/// A recorded payload with the home rewritten to `~` (both separator spellings, case folded when
/// `case_insensitive`) and `user` as a whole word rewritten to `<user>`, in every string and key.
pub fn scrub(value: &Value, home: &str, user: &str, case_insensitive: bool) -> Value {
    match value {
        Value::String(s) => Value::String(scrub_text(s, home, user, case_insensitive)),
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|v| scrub(v, home, user, case_insensitive))
                .collect(),
        ),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(k, v)| {
                    (
                        scrub_text(k, home, user, case_insensitive),
                        scrub(v, home, user, case_insensitive),
                    )
                })
                .collect(),
        ),
        other => other.clone(),
    }
}

fn scrub_text(text: &str, home: &str, user: &str, case_insensitive: bool) -> String {
    let mut out = text.to_owned();
    if !home.is_empty() {
        for spelling in [home.replace('\\', "/"), home.replace('/', "\\")] {
            out = replace_all(&out, &spelling, "~", case_insensitive);
        }
    }
    if !user.is_empty() {
        out = replace_word(&out, user, "<user>");
    }
    out
}

/// ASCII case folding keeps every byte offset, so a match in the folded text cuts the original.
fn fold(text: &str, case_insensitive: bool) -> String {
    if case_insensitive {
        text.to_ascii_lowercase()
    } else {
        text.to_owned()
    }
}

fn replace_all(text: &str, needle: &str, with: &str, case_insensitive: bool) -> String {
    let folded = fold(text, case_insensitive);
    let needle = fold(needle, case_insensitive);
    let mut out = String::new();
    let mut last = 0;
    for (at, _) in folded.match_indices(&needle) {
        out.push_str(&text[last..at]);
        out.push_str(with);
        last = at + needle.len();
    }
    out.push_str(&text[last..]);
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
/// `\Users\` path, or `user` as a whole word: the checks the committed-fixture hygiene walk applies.
pub fn unclean(value: &Value, user: &str) -> Option<Unclean> {
    let mut texts = Vec::new();
    strings(value, &mut texts);
    texts.iter().find_map(|s| text_fault(s, user))
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

/// `rows` with every row that holds no [`SIGNATURES`] literal written `""`: what a recorded
/// `Screen.<phase>.json` keeps.
pub fn signature_rows(rows: &[String]) -> Vec<String> {
    rows.iter()
        .map(|r| {
            if SIGNATURES.holds_any(r) {
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
    if scrub_text(text, home, "", crate::CASE_INSENSITIVE) != text {
        return Some(Unclean::HomePath);
    }
    text_fault(text, user).or_else(|| has_email(text).then_some(Unclean::Email))
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
