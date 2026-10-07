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
use crate::screen::{CONFIRM_WINDOW_FALLBACK, GATE_MAX_WAIT, SIGNATURES};

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
}

impl LedgerRow {
    pub const ALL: [Self; 17] = [
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
        }
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

/// What confirms that a local command was delivered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostCondition {
    /// A SessionStart with source `clear` and a new `session_id`.
    NewSession,
}

/// The known built-in local commands, which fire no UserPromptSubmit, each with its delivery
/// post-condition or none (architecture [Delivery Confirmation]).
pub const LOCAL_COMMANDS: [(&str, Option<PostCondition>); 2] = [
    ("/clear", Some(PostCondition::NewSession)),
    ("/remote-control", None),
];
/// The local command Run B pastes: the list's entry that has a post-condition.
pub const PROBE_LOCAL_COMMAND: &str = LOCAL_COMMANDS[0].0;

/// Run B's three added pastes in paste order, each with the stem its recorded variants carry.
pub const FRAMING_TURNS: [(&str, &str); 3] = [
    (PROBE_LONG_PASTE, "paste-1"),
    (PROBE_TAG_PASTE, "paste-2"),
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
/// untrusted run) and of the `ready` and `turn` screens (the trusted run), and the timings in ms.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TypedRun {
    pub modal: Option<Vec<String>>,
    pub ready: Option<Vec<String>>,
    pub turn: Option<Vec<String>>,
    pub ready_settle_ms: Option<u64>,
    pub turn_settle_ms: Option<u64>,
    pub prompt_latency_ms: Option<u64>,
    pub max_turn_gap_ms: Option<u64>,
}

/// The captures of the two dialog runs, each in claim order: Run C's and Run D's.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DialogRuns {
    pub questions: Vec<Capture>,
    pub plan: Vec<Capture>,
}

/// Every measurement one `viola verify` run makes; `trusted` is Run B's captures in claim order.
#[derive(Debug, Clone, PartialEq)]
pub struct Probes {
    pub print: ProbeRun,
    pub typed: TypedRun,
    pub trusted: Vec<Capture>,
    pub dialogs: DialogRuns,
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
    let first_post = turn
        .iter()
        .position(|c| c.is(HookEvent::PostToolUse, ASK))
        .unwrap_or(turn.len());
    Some(pres.iter().all(|i| *i < first_post))
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
    let after = pasted(trusted, PROBE_TAG_PASTE)? + 1;
    trusted[after..]
        .iter()
        .position(|c| c.event == HookEvent::SessionStart && c.field("source") == Some("clear"))
        .map(|at| after + at)
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

/// Run B's captures named for `--record`: the long and the tag-like prompts as `paste-1` and
/// `paste-2`, and as `clear-1` the hooks the probed local command fired, the SessionEnd captured
/// directly before its SessionStart and that SessionStart. The exit's SessionEnd, captured after
/// it, is not one of them.
pub fn framing_variants(trusted: &[Capture]) -> Vec<(&'static str, &Capture)> {
    let [(long, long_stem), (tag, tag_stem), (_, clear_stem)] = FRAMING_TURNS;
    let mut out = Vec::new();
    for (text, stem) in [(long, long_stem), (tag, tag_stem)] {
        if let Some(at) = pasted(trusted, text) {
            out.push((stem, &trusted[at]));
        }
    }
    // The tag-like turn's prompt precedes it, so `at` is never 0.
    if let Some(at) = clear_start(trusted) {
        let ended = Some(&trusted[at - 1]).filter(|c| c.event == HookEvent::SessionEnd);
        out.extend(ended.map(|c| (clear_stem, c)));
        out.push((clear_stem, &trusted[at]));
    }
    out
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
            input_box_ready(typed.ready.as_ref()) && input_box_ready(typed.turn.as_ref())
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
        let mut calls: Vec<(Option<&str>, Option<&str>)> = Vec::new();
        for capture in turn(captures, prompt) {
            let tool = capture.field("tool_name");
            let id = capture.field("tool_use_id");
            let call = match capture.event {
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
            };
            let Some(n) = call else { continue };
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

/// `true` only when every [`LedgerRow`] reads `"pass"` under `version`. Bytes that are not a `v:1`
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
    Ok(LedgerRow::ALL.iter().all(|row| rows[row.id()] == "pass"))
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn ledger_rows_are_the_seventeen_measured_behaviours_in_order() {
        let ids: Vec<&str> = LedgerRow::ALL.iter().map(|r| r.id()).collect();
        assert_eq!(
            ids,
            [
                "shim-resolution",
                "spine-hooks",
                "session-start-fields",
                "prompt-verbatim",
                "stop-message",
                "largest-hook-payload",
                "modal-signature",
                "input-box-signature",
                "quiet-period",
                "confirm-window",
                "question-answer",
                "plan-approve-revise",
                "question-notes",
                "dialog-concurrency",
                "long-paste-wrapper",
                "tag-escaping",
                "local-command-clear",
            ]
        );
        let words: Vec<&str> = LedgerRow::ALL.iter().map(|r| r.words()).collect();
        assert_eq!(
            words,
            [
                "claude resolves to a real executable",
                "spine hooks fire through the plugin dir",
                "SessionStart carries session_id and source",
                "UserPromptSubmit carries the prompt as sent",
                "Stop carries last_assistant_message",
                "every hook payload fits the frame cap",
                "an untrusted start shows a compiled modal literal",
                "a trusted start shows a compiled input-box literal and no modal",
                "the screen settles within the gate's maximum wait",
                "the typed prompt reaches UserPromptSubmit within the window",
                "a question answered through PreToolUse takes effect",
                "a plan revise and approve each take effect",
                "free text and notes reach the question",
                "two parallel questions each raise a dialog",
                "a long paste unwraps to the text as pasted",
                "tag-like text un-escapes to the text as pasted",
                "/clear starts a new session and submits no prompt",
            ]
        );
        assert!(words.iter().all(|w| w.is_ascii()));
    }

    #[test]
    fn probe_prompt_is_fixed_ascii_without_tags() {
        assert_eq!(
            PROBE_PROMPT,
            "viola verify probe: reply with the single word ok"
        );
        assert!(!PROBE_PROMPT.contains('<') && !PROBE_PROMPT.contains('>'));
    }

    #[rstest]
    #[case::claude(b"2.1.283 (Claude Code)\n", Some("2.1.283"))]
    #[case::no_newline(b"2.1.0 (Claude Code)", Some("2.1.0"))]
    #[case::crlf(b"10.20.30 (Claude Code)\r\n", Some("10.20.30"))]
    #[case::trailing_space(b"2.1.0 (Claude Code)  \nmore", Some("2.1.0"))]
    #[case::later_line_ignored(b"2.1.0 (Claude Code)\n\xff\xfe", Some("2.1.0"))]
    #[case::viola(b"viola 0.1.0\n", None)]
    #[case::garbage(b"garbage (Claude Code)\n", None)]
    #[case::two_fields(b"2.1 (Claude Code)\n", None)]
    #[case::four_fields(b"2.1.0.1 (Claude Code)\n", None)]
    #[case::empty_field(b"2..0 (Claude Code)\n", None)]
    #[case::letter_field(b"2.1.x (Claude Code)\n", None)]
    #[case::leading_space(b" 2.1.0 (Claude Code)\n", None)]
    #[case::prefix(b"v2.1.0 (Claude Code)\n", None)]
    #[case::other_product(b"2.1.0 (Other)\n", None)]
    #[case::second_line(b"\n2.1.0 (Claude Code)\n", None)]
    #[case::empty(b"", None)]
    #[case::not_utf8(b"2.1.\xff (Claude Code)\n", None)]
    fn parse_version_takes_only_the_documented_first_line(
        #[case] stdout: &[u8],
        #[case] expected: Option<&str>,
    ) {
        assert_eq!(parse_version(stdout).as_deref(), expected);
    }

    /// The rendered probe plugin, written out as the oracle.
    const PROBE_HOOKS_JSON: &str = r#"{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe",
            "args": [
              "hook",
              "session-start",
              "--capture",
              "C:/h/ledger/probes/7/captures"
            ],
            "timeout": 5
          }
        ]
      }
    ],
    "UserPromptSubmit": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe",
            "args": [
              "hook",
              "user-prompt-submit",
              "--capture",
              "C:/h/ledger/probes/7/captures"
            ],
            "timeout": 5
          }
        ]
      }
    ],
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe",
            "args": [
              "hook",
              "stop",
              "--capture",
              "C:/h/ledger/probes/7/captures"
            ],
            "timeout": 5
          }
        ]
      }
    ],
    "SessionEnd": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe",
            "args": [
              "hook",
              "session-end",
              "--capture",
              "C:/h/ledger/probes/7/captures"
            ]
          }
        ]
      }
    ]
  }
}
"#;

    #[test]
    fn capture_plugin_files_register_the_spine_on_the_pinned_path() {
        let files = capture_plugin_files(
            "C:/h/bin/0.1.0-0123456789abcdef/viola.exe",
            "C:/h/ledger/probes/7/captures",
            ProbePlugin::Spine,
        );
        assert_eq!(files[0].0, ".claude-plugin/plugin.json");
        assert_eq!(
            files[0].1,
            "{\n  \"name\": \"viola-verify-probe\",\n  \"version\": \"0.1.0\",\n  \
             \"description\": \"viola verify: records the probe's raw hook payloads\"\n}\n"
        );
        assert_eq!(files[1].0, "hooks/hooks.json");
        assert_eq!(files[1].1, PROBE_HOOKS_JSON);
    }

    #[test]
    fn capture_plugin_files_escape_a_path_as_json() {
        let files = capture_plugin_files("C:/a \"q\"/viola.exe", "C:/c", ProbePlugin::Spine);
        let doc: Value = serde_json::from_str(&files[1].1).expect("json");
        assert_eq!(
            doc["hooks"]["Stop"][0]["hooks"][0]["command"],
            "C:/a \"q\"/viola.exe"
        );
    }

    #[test]
    fn event_names_are_the_cli_pascal_names() {
        let names: Vec<&str> = HookEvent::ALL.into_iter().map(event_name).collect();
        assert_eq!(
            names,
            [
                "SessionStart",
                "UserPromptSubmit",
                "Stop",
                "SessionEnd",
                "Notification",
                "PostToolUse",
                "PostToolUseFailure",
                "PreToolUse",
                "PermissionRequest",
            ]
        );
    }

    #[rstest]
    #[case::first("SessionStart.1.json", Some((HookEvent::SessionStart, 1)))]
    #[case::later("PostToolUseFailure.12.json", Some((HookEvent::PostToolUseFailure, 12)))]
    #[case::zero("Stop.0.json", None)]
    #[case::no_k("Stop.json", None)]
    #[case::dialog_event("PreToolUse.1.json", Some((HookEvent::PreToolUse, 1)))]
    #[case::unknown_event("Statusline.1.json", None)]
    #[case::not_json("Stop.1.txt", None)]
    #[case::not_a_number("Stop.x.json", None)]
    #[case::kebab("session-start.1.json", None)]
    fn parse_capture_file_name_reads_event_and_k(
        #[case] name: &str,
        #[case] expected: Option<(HookEvent, u32)>,
    ) {
        assert_eq!(parse_capture_file_name(name), expected);
    }

    #[test]
    fn capture_file_name_round_trips() {
        let name = capture_file_name(HookEvent::UserPromptSubmit, 3);
        assert_eq!(name, "UserPromptSubmit.3.json");
        assert_eq!(
            parse_capture_file_name(&name),
            Some((HookEvent::UserPromptSubmit, 3))
        );
    }

    fn capture(event: HookEvent, payload: Value) -> Capture {
        Capture::read(event, payload.to_string().as_bytes())
    }

    /// The four spine captures a clean probe leaves, in firing order.
    fn clean_captures() -> Vec<Capture> {
        vec![
            capture(
                HookEvent::SessionStart,
                json!({"hook_event_name": "SessionStart", "session_id": "s-1", "source": "startup"}),
            ),
            capture(
                HookEvent::UserPromptSubmit,
                json!({"hook_event_name": "UserPromptSubmit", "prompt": PROBE_PROMPT}),
            ),
            capture(
                HookEvent::Stop,
                json!({"hook_event_name": "Stop", "last_assistant_message": "ok"}),
            ),
            capture(
                HookEvent::SessionEnd,
                json!({"hook_event_name": "SessionEnd", "reason": "other"}),
            ),
        ]
    }

    fn clean_run() -> ProbeRun {
        ProbeRun {
            program_is_script: false,
            version_answered: true,
            captures: clean_captures(),
        }
    }

    /// A clean trusted and untrusted pair of interactive runs.
    fn clean_typed() -> TypedRun {
        TypedRun {
            modal: Some(rows(&["", " ❯ No, exit", "   Yes, I trust this folder"])),
            ready: Some(rows(&["❯ Try it", "  ⏸ manual mode on · ← for agents"])),
            turn: Some(rows(&["● ok", "  ⏸ manual mode on · ← for agents"])),
            ready_settle_ms: Some(1084),
            turn_settle_ms: Some(314),
            prompt_latency_ms: Some(31),
            max_turn_gap_ms: Some(224),
        }
    }

    fn rows(list: &[&str]) -> Vec<String> {
        list.iter().map(|r| (*r).to_owned()).collect()
    }

    fn probes(run: &ProbeRun) -> Probes {
        Probes {
            print: run.clone(),
            typed: clean_typed(),
            trusted: clean_trusted(),
            dialogs: clean_dialogs(),
        }
    }

    fn verdicts(run: &ProbeRun) -> Vec<bool> {
        LedgerRow::ALL
            .iter()
            .map(|row| check(*row, &probes(run)))
            .collect()
    }

    #[test]
    fn capture_read_keeps_the_size_and_a_parsed_payload() {
        let c = Capture::read(HookEvent::Stop, b"{\"a\":1}");
        assert_eq!(c.bytes, 7);
        assert_eq!(c.payload, Some(json!({"a": 1})));
        assert_eq!(Capture::read(HookEvent::Stop, b"not json").payload, None);
    }

    #[test]
    fn check_passes_every_row_on_a_clean_probe() {
        assert_eq!(verdicts(&clean_run()), [true; 17]);
    }

    #[test]
    fn check_shim_resolution_needs_a_real_executable_that_answered() {
        let script = ProbeRun {
            program_is_script: true,
            ..clean_run()
        };
        assert!(!check(LedgerRow::ShimResolution, &probes(&script)));
        let silent = ProbeRun {
            version_answered: false,
            ..clean_run()
        };
        assert!(!check(LedgerRow::ShimResolution, &probes(&silent)));
    }

    #[test]
    fn check_spine_hooks_needs_the_four_in_order_each_naming_itself() {
        let mut missing = clean_run();
        missing.captures.remove(2);
        assert!(!check(LedgerRow::SpineHooks, &probes(&missing)));

        let mut swapped = clean_run();
        swapped.captures.swap(1, 2);
        assert!(!check(LedgerRow::SpineHooks, &probes(&swapped)));

        let mut doubled = clean_run();
        let again = doubled.captures[3].clone();
        doubled.captures.push(again);
        assert!(!check(LedgerRow::SpineHooks, &probes(&doubled)));

        let mut misnamed = clean_run();
        misnamed.captures[0] = capture(HookEvent::SessionStart, json!({"hook_event_name": "Stop"}));
        assert!(!check(LedgerRow::SpineHooks, &probes(&misnamed)));

        let mut unparsed = clean_run();
        unparsed.captures[3] = Capture::read(HookEvent::SessionEnd, b"not json");
        assert!(!check(LedgerRow::SpineHooks, &probes(&unparsed)));

        let mut not_object = clean_run();
        not_object.captures[3] = capture(HookEvent::SessionEnd, json!(["SessionEnd"]));
        assert!(!check(LedgerRow::SpineHooks, &probes(&not_object)));
    }

    #[rstest]
    #[case::empty_session(json!({"session_id": "", "source": "startup"}))]
    #[case::no_session(json!({"source": "startup"}))]
    #[case::number_session(json!({"session_id": 1, "source": "startup"}))]
    #[case::resume(json!({"session_id": "s", "source": "resume"}))]
    #[case::no_source(json!({"session_id": "s"}))]
    fn check_session_start_fields_fails_without_both_fields(#[case] payload: Value) {
        let mut run = clean_run();
        run.captures[0] = capture(HookEvent::SessionStart, payload);
        assert!(!check(LedgerRow::SessionStartFields, &probes(&run)));
        run.captures.remove(0);
        assert!(!check(LedgerRow::SessionStartFields, &probes(&run)));
    }

    #[test]
    fn check_prompt_verbatim_needs_the_exact_prompt() {
        let mut run = clean_run();
        run.captures[1] = capture(
            HookEvent::UserPromptSubmit,
            json!({"prompt": format!("{PROBE_PROMPT} ")}),
        );
        assert!(!check(LedgerRow::PromptVerbatim, &probes(&run)));
        run.captures.remove(1);
        assert!(!check(LedgerRow::PromptVerbatim, &probes(&run)));
    }

    #[test]
    fn check_stop_message_needs_a_string() {
        let mut run = clean_run();
        run.captures[2] = capture(HookEvent::Stop, json!({"last_assistant_message": null}));
        assert!(!check(LedgerRow::StopMessage, &probes(&run)));
        run.captures.remove(2);
        assert!(!check(LedgerRow::StopMessage, &probes(&run)));
    }

    #[test]
    fn check_largest_hook_payload_holds_at_the_cap_and_fails_past_it() {
        let cap = usize::try_from(MAX_FRAME).expect("fits");
        let mut run = clean_run();
        run.captures[1].bytes = cap;
        assert!(check(LedgerRow::LargestHookPayload, &probes(&run)));
        run.captures[1].bytes = cap + 1;
        assert!(!check(LedgerRow::LargestHookPayload, &probes(&run)));
    }

    #[test]
    fn check_reads_the_first_capture_of_an_event() {
        let mut run = clean_run();
        run.captures.push(capture(
            HookEvent::Stop,
            json!({"last_assistant_message": 1}),
        ));
        assert!(check(LedgerRow::StopMessage, &probes(&run)));
    }

    #[test]
    fn largest_keeps_the_biggest_per_event_in_arrival_order() {
        let sized = |event, bytes| Capture {
            event,
            bytes,
            payload: None,
        };
        let got = largest(&[
            sized(HookEvent::Stop, 5),
            sized(HookEvent::SessionStart, 9),
            sized(HookEvent::Stop, 7),
            sized(HookEvent::Stop, 6),
        ]);
        assert_eq!(got, [(HookEvent::Stop, 7), (HookEvent::SessionStart, 9)]);
        assert!(largest(&[]).is_empty());
    }

    fn all_pass() -> Vec<(LedgerRow, bool)> {
        LedgerRow::ALL.iter().map(|r| (*r, true)).collect()
    }

    fn doc(bytes: &[u8]) -> Value {
        serde_json::from_slice(bytes).expect("json")
    }

    #[test]
    fn merge_stamp_writes_the_envelope_for_a_first_version() {
        let bytes = merge_stamp(
            None,
            "2.1.0",
            &all_pass(),
            &[(HookEvent::SessionStart, 120), (HookEvent::Stop, 90)],
            &clean_typed(),
            Some(false),
            "2026-09-28T10:00:00.000Z",
        );
        assert_eq!(
            doc(&bytes),
            json!({
                "v": 1,
                "written_at": "2026-09-28T10:00:00.000Z",
                "writer": "verify",
                "data": {"versions": {"2.1.0": {
                    "verified_at": "2026-09-28T10:00:00.000Z",
                    "rows": {
                        "shim-resolution": "pass", "spine-hooks": "pass",
                        "session-start-fields": "pass", "prompt-verbatim": "pass",
                        "stop-message": "pass", "largest-hook-payload": "pass",
                        "modal-signature": "pass", "input-box-signature": "pass",
                        "quiet-period": "pass", "confirm-window": "pass",
                        "question-answer": "pass", "plan-approve-revise": "pass",
                        "question-notes": "pass", "dialog-concurrency": "pass",
                        "long-paste-wrapper": "pass", "tag-escaping": "pass",
                        "local-command-clear": "pass",
                    },
                    "measured": {
                        "largest_hook_payload": {"SessionStart": 120, "Stop": 90},
                        "typed_probe": {
                            "ready_settle_ms": 1084, "turn_settle_ms": 314,
                            "prompt_latency_ms": 31, "max_turn_gap_ms": 224,
                        },
                        "dialog_probe": {"parallel_both_before_first_post": false},
                    },
                }}},
            })
        );
        assert_eq!(verified(&bytes, "2.1.0"), Ok(true));
    }

    #[test]
    fn merge_stamp_keeps_unknown_versions_and_fields_and_replaces_its_own() {
        let existing = json!({
            "v": 1, "written_at": "old", "writer": "verify", "later": true,
            "data": {"note": "kept", "versions": {
                "3.0.0": {"verified_at": "old", "rows": {"future-row": "pass"}},
                "2.1.0": {"verified_at": "old", "rows": {"spine-hooks": "pass"}, "extra": 1},
            }},
        })
        .to_string();
        let mut results = all_pass();
        results[4].1 = false;
        let bytes = merge_stamp(
            Some(existing.as_bytes()),
            "2.1.0",
            &results,
            &[],
            &TypedRun::default(),
            None,
            "new",
        );
        let d = doc(&bytes);
        assert_eq!(d["later"], true);
        assert_eq!(d["written_at"], "new");
        assert_eq!(d["data"]["note"], "kept");
        assert_eq!(d["data"]["versions"]["3.0.0"]["rows"]["future-row"], "pass");
        let own = &d["data"]["versions"]["2.1.0"];
        assert_eq!(own["verified_at"], "new");
        assert_eq!(own["rows"]["stop-message"], "fail");
        assert_eq!(own["rows"]["spine-hooks"], "pass");
        assert!(own.get("extra").is_none());
        assert_eq!(
            own["measured"],
            json!({"largest_hook_payload": {}, "typed_probe": {
                "ready_settle_ms": null, "turn_settle_ms": null,
                "prompt_latency_ms": null, "max_turn_gap_ms": null,
            }, "dialog_probe": {"parallel_both_before_first_post": null}})
        );
        assert_eq!(verified(&bytes, "2.1.0"), Ok(false));
    }

    #[rstest]
    #[case::not_json(b"not json".as_slice())]
    #[case::array(b"[1]".as_slice())]
    #[case::data_not_object(br#"{"v":1,"data":[1]}"#.as_slice())]
    #[case::versions_not_object(br#"{"v":1,"data":{"versions":1},"keep":1}"#.as_slice())]
    fn merge_stamp_replaces_bytes_of_the_wrong_shape_whole(#[case] existing: &[u8]) {
        let bytes = merge_stamp(
            Some(existing),
            "2.1.0",
            &all_pass(),
            &[],
            &TypedRun::default(),
            None,
            "t",
        );
        let d = doc(&bytes);
        assert!(d.get("keep").is_none());
        assert_eq!(verified(&bytes, "2.1.0"), Ok(true));
    }

    #[test]
    fn merge_stamp_fills_a_data_object_without_versions() {
        let existing = br#"{"v":1,"data":{"note":1}}"#;
        let bytes = merge_stamp(
            Some(existing),
            "2.1.0",
            &all_pass(),
            &[],
            &TypedRun::default(),
            None,
            "t",
        );
        let d = doc(&bytes);
        assert_eq!(d["data"]["note"], 1);
        assert_eq!(verified(&bytes, "2.1.0"), Ok(true));
    }

    #[test]
    fn verified_needs_every_row_pass_under_that_version() {
        let mut results = all_pass();
        let full = merge_stamp(
            None,
            "2.1.0",
            &results,
            &[],
            &TypedRun::default(),
            None,
            "t",
        );
        assert_eq!(verified(&full, "2.1.0"), Ok(true));
        assert_eq!(verified(&full, "3.0.0"), Ok(false));
        results.pop();
        let short = merge_stamp(
            None,
            "2.1.0",
            &results,
            &[],
            &TypedRun::default(),
            None,
            "t",
        );
        assert_eq!(verified(&short, "2.1.0"), Ok(false), "a missing row");
    }

    /// A stamp written before the three framing rows holds the fourteen older ids alone: every one
    /// `pass`, and the version still reads unverified.
    #[test]
    fn verified_reads_a_fourteen_row_stamp_as_unverified() {
        let rows: Map<String, Value> = [
            "shim-resolution",
            "spine-hooks",
            "session-start-fields",
            "prompt-verbatim",
            "stop-message",
            "largest-hook-payload",
            "modal-signature",
            "input-box-signature",
            "quiet-period",
            "confirm-window",
            "question-answer",
            "plan-approve-revise",
            "question-notes",
            "dialog-concurrency",
        ]
        .into_iter()
        .map(|id| (id.to_owned(), json!("pass")))
        .collect();
        let mut stamp = json!({"v": 1, "data": {"versions": {"2.1.0": {"rows": rows}}}});
        assert_eq!(verified(stamp.to_string().as_bytes(), "2.1.0"), Ok(false));
        for id in ["long-paste-wrapper", "tag-escaping", "local-command-clear"] {
            stamp["data"]["versions"]["2.1.0"]["rows"][id] = json!("pass");
        }
        assert_eq!(verified(stamp.to_string().as_bytes(), "2.1.0"), Ok(true));
    }

    #[rstest]
    #[case::not_json(b"not json".as_slice())]
    #[case::array(b"[]".as_slice())]
    #[case::no_v(br#"{"data":{"versions":{}}}"#.as_slice())]
    #[case::v2(br#"{"v":2,"data":{"versions":{}}}"#.as_slice())]
    #[case::data_not_object(br#"{"v":1,"data":1}"#.as_slice())]
    #[case::versions_not_object(br#"{"v":1,"data":{"versions":[]}}"#.as_slice())]
    fn verified_refuses_a_malformed_envelope(#[case] bytes: &[u8]) {
        assert_eq!(verified(bytes, "2.1.0"), Err(AgentError::StampsMalformed));
    }

    #[test]
    fn verified_reads_an_envelope_without_data_as_unverified() {
        assert_eq!(verified(br#"{"v":1}"#, "2.1.0"), Ok(false));
        assert_eq!(
            AgentError::StampsMalformed.to_string(),
            "the capability stamps are malformed"
        );
    }

    #[rstest]
    #[case::backslash_home(r"C:\Users\Plantuser\x", r"C:\Users\Plantuser", "~\\x")]
    #[case::slash_spelling(r"C:/Users/Plantuser/x", r"C:\Users\Plantuser", "~/x")]
    #[case::unix_home("/home/plantuser/p/q", "/home/plantuser", "~/p/q")]
    #[case::twice("/home/plantuser/a:/home/plantuser/b", "/home/plantuser", "~/a:~/b")]
    #[case::not_home("/srv/x", "/home/plantuser", "/srv/x")]
    fn scrub_rewrites_the_home_in_both_spellings(
        #[case] text: &str,
        #[case] home: &str,
        #[case] expected: &str,
    ) {
        assert_eq!(scrub(&json!(text), home, "", false), json!(expected));
    }

    #[test]
    fn scrub_folds_the_home_case_only_when_asked() {
        let text = json!(r"c:\users\plantuser\x");
        assert_eq!(scrub(&text, r"C:\Users\Plantuser", "", true), json!("~\\x"));
        assert_eq!(scrub(&text, r"C:\Users\Plantuser", "", false), text);
    }

    #[rstest]
    #[case::projects_dir(
        r"~\.claude\projects\C--Users-plantuser--viola\s.jsonl",
        r"~\.claude\projects\C--Users-<user>--viola\s.jsonl"
    )]
    #[case::other_case("by PlantUser today", "by <user> today")]
    #[case::whole_word_only("plantusers xplantuser plant_user", "plantusers xplantuser plant_user")]
    #[case::at_edges("plantuser", "<user>")]
    #[case::underscore_joined("plantuser_x", "plantuser_x")]
    fn scrub_rewrites_the_user_as_a_whole_word(#[case] text: &str, #[case] expected: &str) {
        assert_eq!(scrub(&json!(text), "", "plantuser", false), json!(expected));
    }

    #[test]
    fn scrub_walks_keys_arrays_and_nested_objects_and_keeps_other_values() {
        let payload = json!({
            "cwd": "/home/plantuser/w",
            "plantuser": [1, true, null, {"deep": "/home/plantuser"}],
            "n": 2.5,
        });
        let got = scrub(&payload, "/home/plantuser", "plantuser", false);
        assert_eq!(
            got,
            json!({"cwd": "~/w", "<user>": [1, true, null, {"deep": "~"}], "n": 2.5})
        );
        assert!(is_clean(&got, "plantuser"));
        assert!(!is_clean(&payload, "plantuser"));
    }

    #[test]
    fn scrub_with_no_home_and_no_user_changes_nothing() {
        let payload = json!({"cwd": "/home/plantuser/w"});
        assert_eq!(scrub(&payload, "", "", true), payload);
    }

    #[rstest]
    #[case::drive_backslash(json!(r"D:\outside\x"))]
    #[case::drive_slash(json!("d:/outside/x"))]
    #[case::linux_home(json!("see /home/other/x"))]
    #[case::macos_users(json!("see /Users/other/x"))]
    #[case::windows_users(json!(r"at \Users\other\x"))]
    #[case::user_word(json!({"note": "PLANTUSER wrote this"}))]
    #[case::in_a_key(json!({"/home/x/y": 1}))]
    #[case::in_an_array(json!(["ok", "C:/x"]))]
    #[case::bare_drive(json!("C:/"))]
    #[case::user_after_a_space(json!("by plantuser"))]
    fn is_clean_refuses_a_path_or_the_user(#[case] value: Value) {
        assert!(!is_clean(&value, "plantuser"));
    }

    #[rstest]
    #[case::tilde(json!(r"~\.viola-record\ledger"))]
    #[case::drive_without_separator(json!("C:x"))]
    #[case::digit_drive(json!("1:/x"))]
    #[case::placeholder(json!("<user> ran it"))]
    #[case::relative(json!("home/x"))]
    #[case::other_values(json!([1, true, null]))]
    #[case::partial_words(json!("plantusers xplantuser plant_user"))]
    fn is_clean_passes_scrubbed_text(#[case] value: Value) {
        assert!(is_clean(&value, "plantuser"));
    }

    #[test]
    fn is_clean_ignores_an_empty_or_placeholder_user() {
        assert!(is_clean(&json!("plantuser"), ""));
        assert!(is_clean(&json!("<user>"), "<user>"));
    }

    fn typed_check(row: LedgerRow, typed: TypedRun) -> bool {
        check(
            row,
            &Probes {
                print: clean_run(),
                typed,
                trusted: clean_trusted(),
                dialogs: clean_dialogs(),
            },
        )
    }

    #[test]
    fn check_typed_rows_fail_without_a_measurement() {
        for row in [
            LedgerRow::ModalSignature,
            LedgerRow::InputBoxSignature,
            LedgerRow::QuietPeriod,
            LedgerRow::ConfirmWindow,
        ] {
            assert!(!typed_check(row, TypedRun::default()), "{}", row.id());
        }
        let six = verdicts_with(TypedRun::default());
        assert_eq!(
            six,
            [
                true, true, true, true, true, true, false, false, false, false, true, true, true,
                true, true, true, true
            ]
        );
    }

    fn verdicts_with(typed: TypedRun) -> Vec<bool> {
        LedgerRow::ALL
            .iter()
            .map(|row| typed_check(*row, typed.clone()))
            .collect()
    }

    #[rstest]
    #[case::trust("   Yes, I trust this folder", true)]
    #[case::imports("   Yes, allow external imports", true)]
    #[case::input_box_only("  ← for agents", false)]
    #[case::cancel_only(" ❯ No, exit", false)]
    fn check_modal_signature_needs_a_modal_literal(#[case] row: &str, #[case] expected: bool) {
        let typed = TypedRun {
            modal: Some(rows(&["", row])),
            ..clean_typed()
        };
        assert_eq!(typed_check(LedgerRow::ModalSignature, typed), expected);
    }

    #[test]
    fn check_input_box_signature_needs_both_screens_without_a_modal() {
        assert!(typed_check(LedgerRow::InputBoxSignature, clean_typed()));
        let no_turn = TypedRun {
            turn: None,
            ..clean_typed()
        };
        assert!(!typed_check(LedgerRow::InputBoxSignature, no_turn));
        let turn_without = TypedRun {
            turn: Some(rows(&["● ok"])),
            ..clean_typed()
        };
        assert!(!typed_check(LedgerRow::InputBoxSignature, turn_without));
        let ready_without = TypedRun {
            ready: Some(rows(&["❯ Try it"])),
            ..clean_typed()
        };
        assert!(!typed_check(LedgerRow::InputBoxSignature, ready_without));
        for modal in [
            "   Yes, allow external imports",
            "   Yes, I trust this folder",
        ] {
            let ready_modal = TypedRun {
                ready: Some(rows(&[modal, "  ← for agents"])),
                ..clean_typed()
            };
            assert!(
                !typed_check(LedgerRow::InputBoxSignature, ready_modal),
                "{modal}"
            );
            let turn_modal = TypedRun {
                turn: Some(rows(&["  ← for agents", modal])),
                ..clean_typed()
            };
            assert!(
                !typed_check(LedgerRow::InputBoxSignature, turn_modal),
                "{modal}"
            );
        }
    }

    #[rstest]
    #[case::both_at_the_bound(Some(8500), Some(8500), true)]
    #[case::ready_past(Some(8501), Some(10), false)]
    #[case::turn_past(Some(10), Some(8501), false)]
    #[case::ready_missing(None, Some(10), false)]
    #[case::turn_missing(Some(10), None, false)]
    fn check_quiet_period_bounds_both_settles(
        #[case] ready: Option<u64>,
        #[case] turn: Option<u64>,
        #[case] expected: bool,
    ) {
        let typed = TypedRun {
            ready_settle_ms: ready,
            turn_settle_ms: turn,
            ..clean_typed()
        };
        assert_eq!(typed_check(LedgerRow::QuietPeriod, typed), expected);
    }

    #[rstest]
    #[case::at_the_window(Some(10_000), true)]
    #[case::past_the_window(Some(10_001), false)]
    #[case::zero(Some(0), true)]
    #[case::missing(None, false)]
    fn check_confirm_window_bounds_the_latency(#[case] ms: Option<u64>, #[case] expected: bool) {
        let typed = TypedRun {
            prompt_latency_ms: ms,
            ..clean_typed()
        };
        assert_eq!(typed_check(LedgerRow::ConfirmWindow, typed), expected);
    }

    #[test]
    fn signature_rows_keep_only_literal_rows() {
        let live = rows(&[
            " Accessing workspace:",
            " /home/plantuser/work",
            " ❯ No, exit",
            "   Yes, I trust this folder",
            "  ⏸ manual mode on · ← for agents",
            "   Yes, allow external imports",
        ]);
        assert_eq!(
            signature_rows(&live),
            rows(&[
                "",
                "",
                "",
                "   Yes, I trust this folder",
                "  ⏸ manual mode on · ← for agents",
                "   Yes, allow external imports",
            ])
        );
    }

    fn clean(live: &[&str]) -> bool {
        let live = rows(live);
        screen_is_clean(
            &live,
            &signature_rows(&live),
            "/home/plantuser",
            "plantuser",
        )
    }

    #[test]
    fn screen_is_clean_passes_a_measured_screen() {
        assert!(clean(&[
            " ▝▝   ▝▝   ~/dev/projects/viola/.viola-verify-7",
            "────────────────────────",
            "  ctx ? · Haiku 4.5",
            "  ⏸ manual mode on · ← for agents",
        ]));
        assert!(clean(&[
            " /home/plantuser/work/a-long-path",
            " ❯ No, exit",
            "   Yes, I trust this folder",
            " Enter to confirm · Esc to cancel",
        ]));
    }

    #[rstest]
    #[case::home_path(&["  ← for agents /home/plantuser/w"])]
    #[case::other_home(&["  ← for agents /home/other/w"])]
    #[case::username(&["  ← for agents · plantuser"])]
    #[case::email(&["  ← for agents · a.b@example.com"])]
    #[case::user_split_above(&[" /x/plant", "user ← for agents"])]
    #[case::user_split_below(&["  ← for agents plant", "user/x"])]
    #[case::user_split_over_box_drawing(&["  ← for agents plant ──", "── user"])]
    #[case::email_split_below(&["  ← for agents a.b@", "example.com"])]
    fn screen_is_clean_refuses_a_kept_row_or_seam(#[case] live: &[&str]) {
        assert!(!clean(live));
    }

    #[test]
    fn screen_is_clean_reads_no_dropped_row_alone() {
        assert!(clean(&[
            " /home/plantuser/work",
            "",
            "  ← for agents",
            "",
            " a.b@example.com",
        ]));
    }

    #[test]
    fn unclean_names_the_first_check_that_fails() {
        assert_eq!(Unclean::HomePath.code(), "home-path");
        assert_eq!(Unclean::AbsolutePath.code(), "absolute-path");
        assert_eq!(Unclean::Username.code(), "username");
        assert_eq!(Unclean::Email.code(), "email");
        assert_eq!(
            unclean(&json!({"a": "ok", "b": "/home/other/x"}), "plantuser"),
            Some(Unclean::AbsolutePath)
        );
        assert_eq!(
            unclean(&json!(["by plantuser"]), "plantuser"),
            Some(Unclean::Username)
        );
        assert_eq!(
            unclean(&json!({"plantuser": 1}), "plantuser"),
            Some(Unclean::Username)
        );
        assert_eq!(unclean(&json!({"a": "~/x"}), "plantuser"), None);
    }

    fn fault(live: &[&str]) -> Option<ScreenFault> {
        let live = rows(live);
        screen_fault(&live, &signature_rows(&live), "/srv/plantuser", "plantuser")
    }

    #[test]
    fn screen_fault_names_the_row_the_seam_and_the_check() {
        assert_eq!(fault(&["x", "  ← for agents"]), None);
        assert_eq!(
            fault(&["x", "  ← for agents /srv/plantuser/w"]),
            Some(ScreenFault {
                row: 1,
                seam: false,
                why: Unclean::HomePath
            })
        );
        assert_eq!(
            fault(&["  ← for agents /home/other/w"]),
            Some(ScreenFault {
                row: 0,
                seam: false,
                why: Unclean::AbsolutePath
            })
        );
        assert_eq!(
            fault(&["x", "y", "  ← for agents by plantuser"]),
            Some(ScreenFault {
                row: 2,
                seam: false,
                why: Unclean::Username
            })
        );
        assert_eq!(
            fault(&["  ← for agents a@example.com"]),
            Some(ScreenFault {
                row: 0,
                seam: false,
                why: Unclean::Email
            })
        );
        assert_eq!(
            fault(&["ctx plant", "user ← for agents"]),
            Some(ScreenFault {
                row: 1,
                seam: true,
                why: Unclean::Username
            })
        );
        assert_eq!(
            fault(&["  ← for agents a@", "example.com"]),
            Some(ScreenFault {
                row: 0,
                seam: true,
                why: Unclean::Email
            })
        );
    }

    #[rstest]
    #[case::plain("a@example.com", true)]
    #[case::dotted("x.y+z@mail.example.co", true)]
    #[case::no_tld("a@localhost", false)]
    #[case::numeric_tld("a@1.2", false)]
    #[case::one_letter_tld("a@b.c", false)]
    #[case::no_local("@example.com", false)]
    #[case::no_host("a@.com", false)]
    #[case::none("ctx 23% 45k/200k", false)]
    fn has_email_reads_local_at_domain_dot_tld(#[case] text: &str, #[case] expected: bool) {
        assert_eq!(has_email(text), expected);
    }

    /// A captured tool event: `tool_name`, an optional `tool_use_id`, and `extra`'s fields.
    fn tool(event: HookEvent, name: &str, id: Option<&str>, extra: Value) -> Capture {
        let mut payload = json!({"tool_name": name});
        if let Some(id) = id {
            payload["tool_use_id"] = json!(id);
        }
        if let (Some(payload), Value::Object(extra)) = (payload.as_object_mut(), extra) {
            payload.extend(extra);
        }
        capture(event, payload)
    }

    fn prompted(prompt: &str) -> Capture {
        capture(HookEvent::UserPromptSubmit, json!({"prompt": prompt}))
    }

    fn two_questions() -> Value {
        json!({"questions": [
            {"question": "Probe color?", "options": [{"label": "red"}, {"label": "blue"}]},
            {"question": "Probe size?", "options": [{"label": "small"}, {"label": "large"}]},
        ]})
    }

    fn one_question(text: &str) -> Value {
        json!({"questions": [{"question": text, "options": [{"label": "yes"}, {"label": "no"}]}]})
    }

    const PRE: HookEvent = HookEvent::PreToolUse;
    const PR: HookEvent = HookEvent::PermissionRequest;
    const POST: HookEvent = HookEvent::PostToolUse;

    /// The dialog and plan runs as the live 2.1.288 probe recorded them, in claim order (structure
    /// of `evidence/step0-dialog-shapes.md`).
    fn clean_dialogs() -> DialogRuns {
        let stop = capture(HookEvent::Stop, json!({"last_assistant_message": "ok"}));
        let answered = json!({
            "tool_input": two_questions(),
            "tool_response": {
                "questions": two_questions()["questions"],
                "answers": {"Probe color?": "red", "Probe size?": PROBE_FREE_TEXT},
                "annotations": {"Probe color?": {"notes": PROBE_NOTE}},
            },
        });
        let reply = |q: &str| json!({"tool_response": {"answers": {q: "yes"}}});
        DialogRuns {
            questions: vec![
                capture(HookEvent::SessionStart, json!({"source": "startup"})),
                prompted(DIALOG_PROMPT_QUESTIONS),
                tool(
                    PRE,
                    "AskUserQuestion",
                    Some("q1"),
                    json!({"tool_input": two_questions()}),
                ),
                tool(POST, "AskUserQuestion", Some("q1"), answered),
                stop.clone(),
                prompted(DIALOG_PROMPT_PARALLEL),
                tool(
                    PRE,
                    "AskUserQuestion",
                    Some("p1"),
                    json!({"tool_input": one_question("Probe left?")}),
                ),
                tool(POST, "AskUserQuestion", Some("p1"), reply("Probe left?")),
                tool(
                    PRE,
                    "AskUserQuestion",
                    Some("p2"),
                    json!({"tool_input": one_question("Probe right?")}),
                ),
                tool(POST, "AskUserQuestion", Some("p2"), reply("Probe right?")),
                stop.clone(),
                prompted(DIALOG_PROMPT_PERMISSION),
                tool(
                    PR,
                    "Bash",
                    None,
                    json!({"tool_input": {"command": "touch viola-probe-permission"}}),
                ),
                tool(POST, "Bash", Some("b1"), json!({})),
                stop,
            ],
            plan: vec![
                prompted(PLAN_PROMPT),
                tool(POST, "Write", Some("w1"), json!({})),
                tool(
                    PRE,
                    "ExitPlanMode",
                    Some("e1"),
                    json!({"tool_input": {"plan": "p"}}),
                ),
                tool(
                    PR,
                    "ExitPlanMode",
                    None,
                    json!({"tool_input": {"plan": "p"}}),
                ),
                tool(POST, "Write", Some("w2"), json!({})),
                tool(
                    PRE,
                    "ExitPlanMode",
                    Some("e2"),
                    json!({"tool_input": {"plan": "p2"}}),
                ),
                tool(POST, "ExitPlanMode", Some("e2"), json!({})),
            ],
        }
    }

    const DIALOG_ROWS: [LedgerRow; 4] = [
        LedgerRow::QuestionAnswer,
        LedgerRow::PlanApproveRevise,
        LedgerRow::QuestionNotes,
        LedgerRow::DialogConcurrency,
    ];

    fn dialog_check(row: LedgerRow, dialogs: DialogRuns) -> bool {
        check(
            row,
            &Probes {
                print: clean_run(),
                typed: clean_typed(),
                trusted: clean_trusted(),
                dialogs,
            },
        )
    }

    #[test]
    fn check_dialog_rows_pass_on_the_recorded_shape() {
        for row in DIALOG_ROWS {
            assert!(dialog_check(row, clean_dialogs()), "{}", row.id());
        }
    }

    /// Absence is never a pass: no run, or runs whose turns raised no dialog, fail all four.
    #[test]
    fn check_dialog_rows_fail_when_no_dialog_was_raised() {
        let stop = || capture(HookEvent::Stop, json!({}));
        let silent = DialogRuns {
            questions: vec![
                prompted(DIALOG_PROMPT_QUESTIONS),
                stop(),
                prompted(DIALOG_PROMPT_PARALLEL),
                stop(),
                prompted(DIALOG_PROMPT_PERMISSION),
                stop(),
            ],
            plan: vec![prompted(PLAN_PROMPT), stop()],
        };
        for dialogs in [DialogRuns::default(), silent] {
            for row in DIALOG_ROWS {
                assert!(!dialog_check(row, dialogs.clone()), "{}", row.id());
            }
        }
    }

    /// Run B's wrapped long prompt as 2.1.287 sent it (structure of `evidence/step0-shapes.md`).
    fn wrapped(text: &str) -> String {
        format!("\n\n<pasted_content id=\"7ccf\">\n{text}\n</pasted_content id=\"7ccf\">\n")
    }

    /// The tag-like text as the CLI escapes it: both typed `pasted_content` tags, nothing else.
    const TAG_PROMPT: &str = "viola verify probe: the next part is literal sample text and not \
         markup: <\\pasted_content id=\"1\"> sample <\\/pasted_content id=\"1\"> then \
         <task-notification> and that is all. Reply with the single word ok";

    fn session_start(source: &str, id: &str) -> Capture {
        capture(
            HookEvent::SessionStart,
            json!({"source": source, "session_id": id}),
        )
    }

    fn session_end(reason: &str) -> Capture {
        capture(HookEvent::SessionEnd, json!({"reason": reason}))
    }

    /// Run B as the live 2.1.287 probe recorded it, in claim order: the first turn, the long and
    /// the tag-like turns, the two hooks the local command fired, then the exit's SessionEnd.
    fn clean_trusted() -> Vec<Capture> {
        let stop = capture(HookEvent::Stop, json!({"last_assistant_message": "ok"}));
        vec![
            session_start("startup", "s-1"),
            prompted(PROBE_PROMPT),
            stop.clone(),
            prompted(&wrapped(PROBE_LONG_PASTE)),
            stop.clone(),
            prompted(TAG_PROMPT),
            stop,
            session_end("clear"),
            session_start("clear", "s-2"),
            session_end("prompt_input_exit"),
        ]
    }

    const FRAMING_ROWS: [LedgerRow; 3] = [
        LedgerRow::LongPasteWrapper,
        LedgerRow::TagEscaping,
        LedgerRow::LocalCommandClear,
    ];

    fn framing_check(row: LedgerRow, trusted: Vec<Capture>) -> bool {
        check(
            row,
            &Probes {
                print: clean_run(),
                typed: clean_typed(),
                trusted,
                dialogs: clean_dialogs(),
            },
        )
    }

    fn with_trusted(edit: impl FnOnce(&mut Vec<Capture>)) -> Vec<Capture> {
        let mut trusted = clean_trusted();
        edit(&mut trusted);
        trusted
    }

    #[test]
    fn check_framing_rows_pass_on_the_recorded_shape() {
        for row in FRAMING_ROWS {
            assert!(framing_check(row, clean_trusted()), "{}", row.id());
        }
    }

    /// Each paste row reads its own capture: no UserPromptSubmit at all fails it, and so does a
    /// capture whose prompt does not normalise to the compiled text.
    #[test]
    fn check_paste_rows_fail_without_a_matching_capture() {
        let silent: Vec<Capture> = clean_trusted()
            .into_iter()
            .filter(|c| c.event != HookEvent::UserPromptSubmit)
            .collect();
        for (row, at, text) in [
            (LedgerRow::LongPasteWrapper, 3, PROBE_LONG_PASTE),
            (LedgerRow::TagEscaping, 5, PROBE_TAG_PASTE),
        ] {
            assert!(!framing_check(row, silent.clone()), "{} silent", row.id());
            assert!(!framing_check(row, Vec::new()), "{} no run", row.id());
            let other = with_trusted(|t| t[at] = prompted(&format!("{text} and more")));
            assert!(!framing_check(row, other), "{} another text", row.id());
            let framed = with_trusted(|t| t[at] = prompted(&format!("\n{text}")));
            assert!(!framing_check(row, framed), "{} a kept newline", row.id());
        }
        let unwrapped = with_trusted(|t| t[3] = prompted(PROBE_LONG_PASTE));
        assert!(framing_check(LedgerRow::LongPasteWrapper, unwrapped));
    }

    #[test]
    fn check_local_command_clear_fails_on_each_broken_post_condition() {
        let row = LedgerRow::LocalCommandClear;
        let no_start = with_trusted(|t| {
            t.remove(8);
        });
        assert!(!framing_check(row, no_start), "no clear SessionStart");
        let other_source = with_trusted(|t| t[8] = session_start("resume", "s-2"));
        assert!(!framing_check(row, other_source), "another source");
        let same_session = with_trusted(|t| t[8] = session_start("clear", "s-1"));
        assert!(!framing_check(row, same_session), "the first session_id");
        let empty_session = with_trusted(|t| t[8] = session_start("clear", ""));
        assert!(!framing_check(row, empty_session), "an empty session_id");
        let no_session = with_trusted(|t| {
            t[8] = capture(HookEvent::SessionStart, json!({"source": "clear"}));
        });
        assert!(!framing_check(row, no_session), "no session_id");
        let submitted = with_trusted(|t| t.insert(7, prompted(PROBE_LOCAL_COMMAND)));
        assert!(!framing_check(row, submitted), "the command as a prompt");
        let before_the_tag_turn = with_trusted(|t| {
            let start = t.remove(8);
            t.insert(5, start);
        });
        assert!(
            !framing_check(row, before_the_tag_turn),
            "a clear before the tag-like turn"
        );
        let no_tag_turn = with_trusted(|t| {
            t.remove(5);
        });
        assert!(!framing_check(row, no_tag_turn), "no tag-like turn");
        assert!(!framing_check(row, Vec::new()), "no run");
    }

    #[test]
    fn framing_variants_name_the_two_prompts_and_the_hooks_the_local_command_fired() {
        let trusted = clean_trusted();
        let named: Vec<(&str, &Capture)> = framing_variants(&trusted);
        let at: Vec<(&str, usize)> = named
            .iter()
            .map(|(stem, c)| {
                let index = trusted
                    .iter()
                    .position(|t| std::ptr::eq(t, *c))
                    .expect("a capture of the run");
                (*stem, index)
            })
            .collect();
        assert_eq!(
            at,
            [
                ("paste-1", 3),
                ("paste-2", 5),
                ("clear-1", 7),
                ("clear-1", 8)
            ]
        );
        assert_eq!(trusted[7].event, HookEvent::SessionEnd);
        assert_eq!(trusted[8].event, HookEvent::SessionStart);

        let no_end = with_trusted(|t| t[7] = capture(HookEvent::Stop, json!({})));
        let stems: Vec<&str> = framing_variants(&no_end).iter().map(|v| v.0).collect();
        assert_eq!(stems, ["paste-1", "paste-2", "clear-1"]);
        let echoed = vec![
            session_start("startup", "s-1"),
            prompted(PROBE_LONG_PASTE),
            prompted(PROBE_TAG_PASTE),
            prompted(PROBE_LOCAL_COMMAND),
        ];
        let stems: Vec<&str> = framing_variants(&echoed).iter().map(|v| v.0).collect();
        assert_eq!(stems, ["paste-1", "paste-2"]);
        assert!(framing_variants(&[]).is_empty());
    }

    /// The two paste texts are the bytes the live probe pasted, rebuilt here from their parts.
    #[test]
    fn framing_texts_are_fixed_ascii_each_with_its_stem() {
        let filler: Vec<String> = (1..=110).map(|n| format!("filler-{n:04}")).collect();
        let long = format!(
            "viola verify probe: this message is one long synthetic paste and its filler words \
             carry no meaning. Filler follows: {}xxxxxxx End of the synthetic paste. Reply with \
             the single word ok",
            filler.join(" ")
        );
        assert_eq!(PROBE_LONG_PASTE, long);
        assert_eq!(PROBE_LONG_PASTE.len(), 1500);
        assert_eq!(
            PROBE_TAG_PASTE,
            "viola verify probe: the next part is literal sample text and not markup: \
             <pasted_content id=\"1\"> sample </pasted_content id=\"1\"> then <task-notification> \
             and that is all. Reply with the single word ok"
        );
        assert_eq!(PROBE_TAG_PASTE.len(), 200);
        for text in [PROBE_LONG_PASTE, PROBE_TAG_PASTE] {
            assert!(text.is_ascii() && !text.contains('\n') && !text.contains('@'));
            assert!(text.starts_with("viola verify probe: "));
            assert!(text.ends_with("Reply with the single word ok"));
        }
        assert_eq!(framing_stem(PROBE_LONG_PASTE), Some("paste-1"));
        assert_eq!(framing_stem(PROBE_TAG_PASTE), Some("paste-2"));
        assert_eq!(framing_stem("/clear"), Some("clear-1"));
        for other in [PROBE_PROMPT, "/remote-control", "/clear ", ""] {
            assert_eq!(framing_stem(other), None, "{other}");
        }
        assert_eq!(dialog_stem(PROBE_LONG_PASTE), None);
    }

    #[test]
    fn local_commands_are_the_two_known_ones_and_only_clear_is_probed() {
        assert_eq!(
            LOCAL_COMMANDS,
            [
                ("/clear", Some(PostCondition::NewSession)),
                ("/remote-control", None)
            ]
        );
        assert_eq!(PROBE_LOCAL_COMMAND, "/clear");
        let probed: Vec<&str> = LOCAL_COMMANDS
            .iter()
            .filter(|(_, post)| post.is_some())
            .map(|(command, _)| *command)
            .collect();
        assert_eq!(probed, [PROBE_LOCAL_COMMAND]);
    }

    fn with_questions(edit: impl FnOnce(&mut Vec<Capture>)) -> DialogRuns {
        let mut dialogs = clean_dialogs();
        edit(&mut dialogs.questions);
        dialogs
    }

    fn with_plan(edit: impl FnOnce(&mut Vec<Capture>)) -> DialogRuns {
        let mut dialogs = clean_dialogs();
        edit(&mut dialogs.plan);
        dialogs
    }

    fn post_response(answers: Value, annotations: Value) -> Capture {
        tool(
            POST,
            "AskUserQuestion",
            Some("q1"),
            json!({"tool_response": {"answers": answers, "annotations": annotations}}),
        )
    }

    #[test]
    fn check_question_answer_needs_the_label_its_own_post_and_no_permission_request() {
        let notes = json!({"Probe color?": {"notes": PROBE_NOTE}});
        let wrong_label = with_questions(|q| {
            q[3] = post_response(
                json!({"Probe color?": "blue", "Probe size?": PROBE_FREE_TEXT}),
                notes,
            );
        });
        assert!(!dialog_check(LedgerRow::QuestionAnswer, wrong_label));
        let other_id = with_questions(|q| {
            q[3] = tool(POST, "AskUserQuestion", Some("q9"), json!({}));
        });
        assert!(!dialog_check(LedgerRow::QuestionAnswer, other_id));
        let raised = with_questions(|q| {
            q.push(tool(PR, "AskUserQuestion", None, json!({})));
        });
        assert!(!dialog_check(LedgerRow::QuestionAnswer, raised));
        let first_option_only = with_questions(|q| {
            q[3] = post_response(json!({"Probe color?": "red"}), json!({}));
        });
        assert!(dialog_check(LedgerRow::QuestionAnswer, first_option_only));
    }

    #[test]
    fn check_question_notes_needs_the_free_text_and_the_note() {
        let notes = json!({"Probe color?": {"notes": PROBE_NOTE}});
        let no_free_text = with_questions(|q| {
            q[3] = post_response(
                json!({"Probe color?": "red", "Probe size?": "small"}),
                notes.clone(),
            );
        });
        assert!(!dialog_check(LedgerRow::QuestionNotes, no_free_text));
        let no_note = with_questions(|q| {
            q[3] = post_response(
                json!({"Probe color?": "red", "Probe size?": PROBE_FREE_TEXT}),
                json!({"Probe color?": {"notes": "other"}}),
            );
        });
        assert!(!dialog_check(LedgerRow::QuestionNotes, no_note));
        let one = with_questions(|q| {
            q[2] = tool(
                PRE,
                "AskUserQuestion",
                Some("q1"),
                json!({"tool_input": one_question("Probe color?")}),
            );
        });
        assert!(!dialog_check(LedgerRow::QuestionNotes, one));
    }

    #[test]
    fn check_plan_approve_revise_reads_the_revise_the_re_plan_and_the_approve() {
        assert!(!dialog_check(
            LedgerRow::PlanApproveRevise,
            with_plan(|p| p.push(tool(PR, "ExitPlanMode", None, json!({}))))
        ));
        let approve_not_taken = with_plan(|p| {
            p[6] = tool(PR, "ExitPlanMode", None, json!({}));
        });
        assert!(!dialog_check(
            LedgerRow::PlanApproveRevise,
            approve_not_taken
        ));
        let no_revise = with_plan(|p| {
            p.remove(3);
        });
        assert!(!dialog_check(LedgerRow::PlanApproveRevise, no_revise));
        let other_call = with_plan(|p| {
            p[6] = tool(POST, "ExitPlanMode", Some("e1"), json!({}));
        });
        assert!(!dialog_check(LedgerRow::PlanApproveRevise, other_call));
        let starts_with_a_request = with_plan(|p| {
            p.swap(2, 3);
        });
        assert!(!dialog_check(
            LedgerRow::PlanApproveRevise,
            starts_with_a_request
        ));
        let re_plan_missing = with_plan(|p| {
            p[5] = tool(POST, "ExitPlanMode", Some("e2"), json!({}));
        });
        assert!(!dialog_check(LedgerRow::PlanApproveRevise, re_plan_missing));
    }

    #[test]
    fn check_dialog_concurrency_needs_two_distinct_calls_each_answered() {
        let one = with_questions(|q| {
            q.drain(8..10);
        });
        assert!(!dialog_check(LedgerRow::DialogConcurrency, one));
        let same_id = with_questions(|q| {
            q[8] = tool(PRE, "AskUserQuestion", Some("p1"), json!({}));
            q[9] = tool(POST, "AskUserQuestion", Some("p1"), json!({}));
        });
        assert!(!dialog_check(LedgerRow::DialogConcurrency, same_id));
        let unanswered = with_questions(|q| {
            q.remove(9);
        });
        assert!(!dialog_check(LedgerRow::DialogConcurrency, unanswered));
        let three = with_questions(|q| {
            q.insert(10, tool(PRE, "AskUserQuestion", Some("p3"), json!({})));
            q.insert(11, tool(POST, "AskUserQuestion", Some("p3"), json!({})));
        });
        assert!(!dialog_check(LedgerRow::DialogConcurrency, three));
        let no_id = with_questions(|q| {
            q[8] = tool(PRE, "AskUserQuestion", None, json!({}));
        });
        assert!(!dialog_check(LedgerRow::DialogConcurrency, no_id));
    }

    #[test]
    fn parallel_both_before_first_post_reads_the_claim_order() {
        assert_eq!(
            parallel_both_before_first_post(&clean_dialogs().questions),
            Some(false)
        );
        let open_together = with_questions(|q| q.swap(7, 8)).questions;
        assert_eq!(parallel_both_before_first_post(&open_together), Some(true));
        let unanswered = with_questions(|q| {
            q.remove(9);
            q.remove(7);
        })
        .questions;
        assert_eq!(parallel_both_before_first_post(&unanswered), Some(true));
        let one = with_questions(|q| {
            q.drain(8..10);
        })
        .questions;
        assert_eq!(parallel_both_before_first_post(&one), None);
        assert_eq!(parallel_both_before_first_post(&[]), None);
    }

    #[test]
    fn dialog_variants_name_each_call_of_each_turn() {
        let dialogs = clean_dialogs();
        let names: Vec<String> = dialog_variants(&dialogs)
            .iter()
            .map(|(variant, c)| format!("{}.{variant}", event_name(c.event)))
            .collect();
        assert_eq!(
            names,
            [
                "PreToolUse.questions-1",
                "PostToolUse.questions-1",
                "PreToolUse.parallel-1",
                "PostToolUse.parallel-1",
                "PreToolUse.parallel-2",
                "PostToolUse.parallel-2",
                "PermissionRequest.permission-1",
                "PostToolUse.permission-1",
                "PreToolUse.plan-1",
                "PermissionRequest.plan-1",
                "PreToolUse.plan-2",
                "PostToolUse.plan-2",
            ]
        );
        let posts: Vec<Option<&str>> = dialog_variants(&dialogs)
            .iter()
            .filter(|(_, c)| c.event == POST)
            .map(|(_, c)| c.field("tool_use_id"))
            .collect();
        assert_eq!(
            posts,
            [Some("q1"), Some("p1"), Some("p2"), Some("b1"), Some("e2")]
        );
        assert!(dialog_variants(&DialogRuns::default()).is_empty());
    }

    /// A second capture of one event and variant is not recorded: the first stands.
    #[test]
    fn dialog_variants_keep_the_first_capture_of_a_variant() {
        let mut dialogs = clean_dialogs();
        dialogs.questions.insert(
            4,
            tool(POST, "AskUserQuestion", Some("q1"), json!({"n": 2})),
        );
        let kept: Vec<&Capture> = dialog_variants(&dialogs)
            .into_iter()
            .filter(|(v, c)| v == "questions-1" && c.event == POST)
            .map(|(_, c)| c)
            .collect();
        assert_eq!(kept.len(), 1);
        assert!(
            kept[0]
                .payload
                .as_ref()
                .is_some_and(|p| p.get("n").is_none())
        );
    }

    #[test]
    fn dialog_prompts_are_fixed_ascii_without_tags_each_with_its_stem() {
        for (run, prompt, stem) in DIALOG_TURNS {
            assert!(prompt.is_ascii() && !prompt.contains('<') && !prompt.contains('>'));
            assert!(prompt.starts_with("viola verify probe: "));
            assert_eq!(dialog_stem(prompt), Some(stem));
            assert_eq!(run == DialogRun::Plan, stem == "plan");
        }
        let stems: Vec<&str> = DIALOG_TURNS.iter().map(|t| t.2).collect();
        assert_eq!(stems, ["questions", "parallel", "permission", "plan"]);
        assert!(DIALOG_PROMPT_PERMISSION.contains("touch viola-probe-permission"));
        assert_eq!(dialog_stem(PROBE_PROMPT), None);
        assert_eq!(dialog_stem(""), None);
        for text in [PROBE_FREE_TEXT, PROBE_NOTE, PROBE_REVISE] {
            assert!(text.is_ascii() && !text.contains('<'));
        }
    }

    #[test]
    fn dialog_settings_are_the_ask_rule_and_the_plans_directory() {
        assert_eq!(
            serde_json::from_str::<Value>(DIALOG_SETTINGS).expect("json"),
            json!({"permissions": {"ask": ["Bash(touch viola-probe-permission)"]}})
        );
        assert_eq!(
            serde_json::from_str::<Value>(&plan_settings("C:\\w \"x\"/plans")).expect("json"),
            json!({"plansDirectory": "C:\\w \"x\"/plans"})
        );
    }

    #[test]
    fn probe_answers_are_the_compiled_table() {
        let ids: Vec<&str> = ProbeAnswer::ALL.iter().map(|a| a.id()).collect();
        assert_eq!(
            ids,
            [
                "questions-first-and-free-text",
                "question-first-option",
                "plan-revise",
                "plan-approve",
                "permit-allow"
            ]
        );
        for answer in ProbeAnswer::ALL {
            assert_eq!(ProbeAnswer::from_id(answer.id()), Some(answer));
        }
        assert_eq!(ProbeAnswer::from_id("plan-approve\n"), None);
        let table: Vec<String> = PROBE_ANSWERS
            .iter()
            .map(|(run, event, j, answer)| {
                format!(
                    "{run:?} {} {} {}",
                    answer_file_name(*event, *j),
                    answer.id(),
                    j
                )
            })
            .collect();
        assert_eq!(
            table,
            [
                "Questions PreToolUse.1 questions-first-and-free-text 1",
                "Questions PreToolUse.2 question-first-option 2",
                "Questions PreToolUse.3 question-first-option 3",
                "Questions PermissionRequest.1 permit-allow 1",
                "Plan PermissionRequest.1 plan-revise 1",
                "Plan PreToolUse.2 plan-approve 2",
            ]
        );
    }

    fn body_of(event: HookEvent, payload: &Value, answer: ProbeAnswer) -> Option<Value> {
        probe_body(event, payload.to_string().as_bytes(), answer)
            .map(|b| serde_json::from_str(&b).expect("a JSON body"))
    }

    #[test]
    fn probe_body_builds_each_answer_through_the_product_mapping() {
        let asked = json!({"tool_name": "AskUserQuestion", "tool_input": two_questions()});
        let first = body_of(PRE, &asked, ProbeAnswer::QuestionsFirstAndFreeText).expect("a body");
        let updated = &first["hookSpecificOutput"]["updatedInput"];
        assert_eq!(updated["questions"], two_questions()["questions"]);
        assert_eq!(
            updated["answers"],
            json!({"Probe color?": "red", "Probe size?": PROBE_FREE_TEXT})
        );
        assert_eq!(
            updated["annotations"],
            json!({"Probe color?": {"notes": PROBE_NOTE}})
        );
        let option = body_of(PRE, &asked, ProbeAnswer::QuestionFirstOption).expect("a body");
        let updated = &option["hookSpecificOutput"]["updatedInput"];
        assert_eq!(
            updated["answers"],
            json!({"Probe color?": "red", "Probe size?": "small"})
        );
        assert!(updated.get("annotations").is_none());

        let plan =
            json!({"tool_name": "ExitPlanMode", "tool_input": {"plan": "p", "planFilePath": "f"}});
        let revise = body_of(PR, &plan, ProbeAnswer::PlanRevise).expect("a body");
        assert_eq!(
            revise["hookSpecificOutput"]["decision"],
            json!({"behavior": "deny", "message": PROBE_REVISE})
        );
        let approve = body_of(PRE, &plan, ProbeAnswer::PlanApprove).expect("a body");
        assert_eq!(
            approve["hookSpecificOutput"],
            json!({"hookEventName": "PreToolUse", "permissionDecision": "allow",
                "updatedInput": {"plan": "p", "planFilePath": "f"}})
        );
        let bash = json!({"tool_name": "Bash", "tool_input": {"command": "touch x"}});
        let allow = body_of(PR, &bash, ProbeAnswer::PermitAllow).expect("a body");
        assert_eq!(
            allow["hookSpecificOutput"]["decision"],
            json!({"behavior": "allow"})
        );
    }

    #[test]
    fn probe_body_with_one_question_answers_it_and_notes_it() {
        let asked = json!({"tool_input": one_question("Probe left?")});
        let got = body_of(PRE, &asked, ProbeAnswer::QuestionsFirstAndFreeText).expect("a body");
        let updated = &got["hookSpecificOutput"]["updatedInput"];
        assert_eq!(updated["answers"], json!({"Probe left?": "yes"}));
        assert_eq!(
            updated["annotations"],
            json!({"Probe left?": {"notes": PROBE_NOTE}})
        );
    }

    #[rstest]
    #[case::not_json(PRE, None, ProbeAnswer::QuestionFirstOption)]
    #[case::no_tool_input(PRE, Some(json!({"tool_name": "AskUserQuestion"})), ProbeAnswer::QuestionFirstOption)]
    #[case::no_questions(PRE, Some(json!({"tool_input": {}})), ProbeAnswer::QuestionFirstOption)]
    #[case::question_not_string(PRE, Some(json!({"tool_input": {"questions": [{"question": 1, "options": [{"label": "a"}]}]}})), ProbeAnswer::QuestionFirstOption)]
    #[case::no_option(PRE, Some(json!({"tool_input": {"questions": [{"question": "q", "options": []}]}})), ProbeAnswer::QuestionFirstOption)]
    #[case::no_first_question(PRE, Some(json!({"tool_input": {"questions": []}})), ProbeAnswer::QuestionsFirstAndFreeText)]
    #[case::revise_on_pre_tool_use(PRE, Some(json!({"tool_input": {"plan": "p"}})), ProbeAnswer::PlanRevise)]
    #[case::approve_on_permission_request(PR, Some(json!({"tool_input": {"plan": "p"}})), ProbeAnswer::PlanApprove)]
    #[case::allow_on_pre_tool_use(PRE, Some(json!({"tool_input": {}})), ProbeAnswer::PermitAllow)]
    fn probe_body_is_none_where_the_answer_cannot_map(
        #[case] event: HookEvent,
        #[case] payload: Option<Value>,
        #[case] answer: ProbeAnswer,
    ) {
        let bytes = payload.map_or_else(|| b"not json".to_vec(), |p| p.to_string().into_bytes());
        assert_eq!(probe_body(event, &bytes, answer), None);
    }

    #[test]
    fn probe_body_with_no_questions_answers_nothing_by_first_option() {
        let asked = json!({"tool_input": {"questions": []}});
        assert_eq!(
            body_of(PRE, &asked, ProbeAnswer::QuestionFirstOption)
                .map(|b| { b["hookSpecificOutput"]["updatedInput"]["answers"].clone() }),
            Some(json!({}))
        );
    }

    #[test]
    fn capture_plugin_files_for_the_dialog_tier_add_the_three_events_and_the_answers() {
        let files = capture_plugin_files(
            "C:/h/viola.exe",
            "C:/h/c",
            ProbePlugin::Dialog {
                answers_dir_fwd: "C:/h/a",
            },
        );
        let doc: Value = serde_json::from_str(&files[1].1).expect("json");
        let events: Vec<&str> = doc["hooks"]
            .as_object()
            .expect("hooks")
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            events,
            [
                "SessionStart",
                "UserPromptSubmit",
                "Stop",
                "SessionEnd",
                "PreToolUse",
                "PermissionRequest",
                "PostToolUse"
            ]
        );
        let group = |event: &str| doc["hooks"][event][0].clone();
        let args = |event: &str| group(event)["hooks"][0]["args"].clone();
        assert_eq!(
            args("PreToolUse"),
            json!([
                "hook",
                "pre-tool-use",
                "--capture",
                "C:/h/c",
                "--answers",
                "C:/h/a"
            ])
        );
        assert_eq!(
            group("PreToolUse")["matcher"],
            "AskUserQuestion|ExitPlanMode"
        );
        assert_eq!(
            args("PermissionRequest"),
            json!([
                "hook",
                "permission-request",
                "--capture",
                "C:/h/c",
                "--answers",
                "C:/h/a"
            ])
        );
        assert!(group("PermissionRequest").get("matcher").is_none());
        assert_eq!(
            args("PostToolUse"),
            json!(["hook", "post-tool-use", "--capture", "C:/h/c"])
        );
        assert_eq!(args("Stop"), json!(["hook", "stop", "--capture", "C:/h/c"]));
        assert_eq!(group("PostToolUse")["hooks"][0]["timeout"], 5);
        assert!(group("SessionEnd")["hooks"][0].get("timeout").is_none());
        assert_eq!(group("PreToolUse")["hooks"][0]["command"], "C:/h/viola.exe");
        let spine = capture_plugin_files("C:/h/viola.exe", "C:/h/c", ProbePlugin::Spine);
        assert!(!spine[1].1.contains("--answers") && !spine[1].1.contains("PreToolUse"));
    }
}
