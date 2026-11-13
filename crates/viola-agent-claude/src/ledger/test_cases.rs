use super::*;
use rstest::rstest;

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
        long_turn: Some(rows(&["● long", "  ⏸ manual mode on · ← for agents"])),
        ready_settle_ms: Some(1084),
        turn_settle_ms: Some(314),
        prompt_latency_ms: Some(31),
        max_turn_gap_ms: Some(224),
        compact_latency_ms: Some(18_779),
        box_only_guards: Some(2),
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
        statusline: clean_statusline(),
        resumed: clean_resumed(),
        agents: Some(clean_agents()),
    }
}

fn verdicts(run: &ProbeRun) -> Vec<bool> {
    LedgerRow::ALL
        .iter()
        .map(|row| check(*row, &probes(run)))
        .collect()
}

fn all_pass() -> Vec<(LedgerRow, bool)> {
    LedgerRow::ALL.iter().map(|r| (*r, true)).collect()
}

fn doc(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).expect("json")
}

fn typed_check(row: LedgerRow, typed: TypedRun) -> bool {
    check(
        row,
        &Probes {
            print: clean_run(),
            typed,
            trusted: clean_trusted(),
            dialogs: clean_dialogs(),
            statusline: clean_statusline(),
            resumed: clean_resumed(),
            agents: Some(clean_agents()),
        },
    )
}

fn verdicts_with(typed: TypedRun) -> Vec<bool> {
    LedgerRow::ALL
        .iter()
        .map(|row| typed_check(*row, typed.clone()))
        .collect()
}

fn rule(n: usize) -> String {
    "─".repeat(n)
}

/// A screen whose footer a notice displaced while the box stands: a rule, the prompt row, a rule.
fn box_only(top: &str) -> Vec<String> {
    vec![
        top.to_owned(),
        rule(80),
        "❯ ".to_owned(),
        rule(80),
        "  ctrl+g to edit in Vim · Update available!".to_owned(),
    ]
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

fn fault(live: &[&str]) -> Option<ScreenFault> {
    let live = rows(live);
    screen_fault(&live, &signature_rows(&live), "/srv/plantuser", "plantuser")
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
            statusline: clean_statusline(),
            resumed: clean_resumed(),
            agents: Some(clean_agents()),
        },
    )
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

/// The newline text as the CLI sends it: unchanged, its one LF kept (`rule-inner-lf`).
const NEWLINE_PROMPT: &str = "viola verify probe: this paste holds one line break here\nand \
         the next line ends it. Reply with the single word ok";

/// Run B as the live 2.1.287 probe recorded it, in claim order: the first turn, the long, the
/// tag-like and the newline turns, the SessionStart `/compact` fired, the two hooks the local
/// command fired, then the exit's SessionEnd.
fn clean_trusted() -> Vec<Capture> {
    let stop = capture(HookEvent::Stop, json!({"last_assistant_message": "ok"}));
    vec![
        session_start("startup", RUN_B_ID),
        prompted(PROBE_PROMPT),
        stop.clone(),
        prompted(&wrapped(PROBE_LONG_PASTE)),
        stop.clone(),
        prompted(TAG_PROMPT),
        stop.clone(),
        prompted(NEWLINE_PROMPT),
        stop,
        session_start("compact", RUN_B_ID),
        session_end("clear"),
        session_start("clear", "s-2"),
        session_end("prompt_input_exit"),
    ]
}

/// Run B's first session id, of the session-id shape the CLI gives one.
const RUN_B_ID: &str = "3f2b9c1e-7a4d-4e8b-9c0f-1d2e3a4b5c6d";

/// Run E as the live probe records it: the one SessionStart of the resumed session before the
/// kill.
fn clean_resumed() -> Vec<Capture> {
    vec![session_start("resume", RUN_B_ID)]
}

/// Run B's child pid in the agents read.
const RUN_B_PID: u32 = 4242;

fn agents_row(pid: Option<u32>, id: &str, status: &str) -> Value {
    let mut row = json!({
        "cwd": "~/p",
        "kind": "interactive",
        "name": "a title",
        "sessionId": id,
        "startedAt": 1_760_000_000_000_i64,
        "status": status,
    });
    if let Some(pid) = pid {
        row["pid"] = json!(pid);
    }
    row
}

fn agents_of(child_pid: Option<u32>, rows: &[Value]) -> AgentsCapture {
    AgentsCapture {
        child_pid,
        bytes: Value::from(rows.to_vec()).to_string().into_bytes(),
    }
}

/// The read as a host with another live session lists it: Run B's own row and one more.
fn clean_agents() -> AgentsCapture {
    agents_of(
        Some(RUN_B_PID),
        &[
            agents_row(Some(1), "0b7c9f4e-1d2a-4c3b-8e5f-6a7b8c9d0e1f", "busy"),
            agents_row(Some(RUN_B_PID), RUN_B_ID, "idle"),
        ],
    )
}

fn agents_check(agents: Option<AgentsCapture>) -> bool {
    check(
        LedgerRow::AgentsJoin,
        &Probes {
            agents,
            ..probes(&clean_run())
        },
    )
}

fn resume_check(trusted: Vec<Capture>, resumed: Vec<Capture>) -> bool {
    check(
        LedgerRow::ResumeSession,
        &Probes {
            print: clean_run(),
            typed: clean_typed(),
            trusted,
            dialogs: clean_dialogs(),
            statusline: clean_statusline(),
            resumed,
            agents: Some(clean_agents()),
        },
    )
}

const FRAMING_ROWS: [LedgerRow; 5] = [
    LedgerRow::LongPasteWrapper,
    LedgerRow::TagEscaping,
    LedgerRow::LocalCommandClear,
    LedgerRow::PasteNewline,
    LedgerRow::LocalCommandCompact,
];

fn framing_check(row: LedgerRow, trusted: Vec<Capture>) -> bool {
    check(
        row,
        &Probes {
            print: clean_run(),
            typed: clean_typed(),
            trusted,
            dialogs: clean_dialogs(),
            statusline: clean_statusline(),
            resumed: clean_resumed(),
            agents: Some(clean_agents()),
        },
    )
}

fn with_trusted(edit: impl FnOnce(&mut Vec<Capture>)) -> Vec<Capture> {
    let mut trusted = clean_trusted();
    edit(&mut trusted);
    trusted
}

fn compact_check(trusted: Vec<Capture>, typed: TypedRun) -> bool {
    check(
        LedgerRow::LocalCommandCompact,
        &Probes {
            print: clean_run(),
            typed,
            trusted,
            dialogs: clean_dialogs(),
            statusline: clean_statusline(),
            resumed: clean_resumed(),
            agents: Some(clean_agents()),
        },
    )
}

/// A status line payload as 2.1.287 hands one over after a turn: both windows, whole numbers.
fn limited(five_hour: Value, seven_day: Value) -> StatusLineCapture {
    let payload = json!({
        "session_id": "s-1",
        "model": {"id": "claude-haiku"},
        "rate_limits": {"five_hour": five_hour, "seven_day": seven_day},
    });
    StatusLineCapture {
        bytes: payload.to_string().into_bytes(),
        shell: ShellVerdict::BinSh,
    }
}

fn window(used: Value, resets_at: Value) -> Value {
    json!({"used_percentage": used, "resets_at": resets_at})
}

/// Run B's status line as the live 2.1.287 probe would file it: a first payload without
/// `rate_limits`, then one with both windows, each run by `/bin/sh`.
fn clean_statusline() -> Vec<StatusLineCapture> {
    let first = json!({"session_id": "s-1", "model": {"id": "claude-haiku"}});
    vec![
        StatusLineCapture {
            bytes: first.to_string().into_bytes(),
            shell: ShellVerdict::BinSh,
        },
        limited(
            window(json!(23), json!(1_760_400_000)),
            window(json!(91), json!(1_760_900_000)),
        ),
    ]
}

fn statusline_check(row: LedgerRow, statusline: Vec<StatusLineCapture>) -> bool {
    check(
        row,
        &Probes {
            print: clean_run(),
            typed: clean_typed(),
            trusted: clean_trusted(),
            dialogs: clean_dialogs(),
            statusline,
            resumed: clean_resumed(),
            agents: Some(clean_agents()),
        },
    )
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

fn body_of(event: HookEvent, payload: &Value, answer: ProbeAnswer) -> Option<Value> {
    probe_body(event, payload.to_string().as_bytes(), answer)
        .map(|b| serde_json::from_str(&b).expect("a JSON body"))
}

#[path = "test_cases_stamps.rs"]
mod stamps;

#[path = "test_cases_checks.rs"]
mod checks;

#[path = "test_cases_runs.rs"]
mod runs;

#[path = "test_cases_framing.rs"]
mod framing;
