//! The capability ledger (architecture §Established Decisions → [CLI Version Compatibility]): the
//! closed set of CLI behaviours a print-mode run of `claude` can measure, each with its
//! post-condition; the capture plugin that records the probe's raw hook payloads; the stamp a
//! `viola verify` run writes; and the scrub a recorded payload passes before it becomes a fixture.
//! Pure: no I/O. Upstream text is content, compared or copied, never interpreted.

use std::time::Duration;

use serde_json::{Map, Value, json};
use viola_core::MAX_FRAME;

use crate::AgentError;
use crate::hook::HookEvent;
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
}

impl LedgerRow {
    pub const ALL: [Self; 10] = [
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
        }
    }
}

/// The probe's synthetic prompt: ASCII, no tag characters.
pub const PROBE_PROMPT: &str = "viola verify probe: reply with the single word ok";

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

/// The probe's plugin folder, each file as `(relative path, content)`: every spine event runs
/// `<pinned viola> hook <event> --capture <capture dir>` in exec form, the shape the run plugin
/// takes.
pub fn capture_plugin_files(
    pinned_bin_fwd: &str,
    capture_dir_fwd: &str,
) -> [(&'static str, String); 2] {
    let plugin = json!({
        "name": "viola-verify-probe",
        "version": crate::VERSION,
        "description": "viola verify: records the probe's raw hook payloads",
    });
    let mut hooks = Map::new();
    for event in CAPTURE_EVENTS {
        let mut hook = json!({
            "type": "command",
            "command": pinned_bin_fwd,
            "args": ["hook", event.as_str(), "--capture", capture_dir_fwd],
        });
        if event != HookEvent::SessionEnd {
            hook["timeout"] = json!(5);
        }
        hooks.insert(event_name(event).to_owned(), json!([{"hooks": [hook]}]));
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

/// Every measurement one `viola verify` run makes.
#[derive(Debug, Clone, PartialEq)]
pub struct Probes {
    pub print: ProbeRun,
    pub typed: TypedRun,
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
/// payloads and the typed probe's timings); every other
/// version and every unknown field is kept. Existing bytes of the wrong shape are replaced whole.
pub fn merge_stamp(
    existing: Option<&[u8]>,
    version: &str,
    results: &[(LedgerRow, bool)],
    largest: &[(HookEvent, usize)],
    typed: &TypedRun,
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
    fn ledger_rows_are_the_ten_measured_behaviours_in_order() {
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
        let files = capture_plugin_files("C:/a \"q\"/viola.exe", "C:/c");
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
        assert_eq!(verdicts(&clean_run()), [true; 10]);
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
                    },
                    "measured": {
                        "largest_hook_payload": {"SessionStart": 120, "Stop": 90},
                        "typed_probe": {
                            "ready_settle_ms": 1084, "turn_settle_ms": 314,
                            "prompt_latency_ms": 31, "max_turn_gap_ms": 224,
                        },
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
            }})
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
            "t",
        );
        let d = doc(&bytes);
        assert_eq!(d["data"]["note"], 1);
        assert_eq!(verified(&bytes, "2.1.0"), Ok(true));
    }

    #[test]
    fn verified_needs_every_row_pass_under_that_version() {
        let mut results = all_pass();
        let full = merge_stamp(None, "2.1.0", &results, &[], &TypedRun::default(), "t");
        assert_eq!(verified(&full, "2.1.0"), Ok(true));
        assert_eq!(verified(&full, "3.0.0"), Ok(false));
        results.pop();
        let short = merge_stamp(None, "2.1.0", &results, &[], &TypedRun::default(), "t");
        assert_eq!(verified(&short, "2.1.0"), Ok(false), "a missing row");
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
                true, true, true, true, true, true, false, false, false, false
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
    #[case::both_at_the_bound(Some(5000), Some(5000), true)]
    #[case::ready_past(Some(5001), Some(10), false)]
    #[case::turn_past(Some(10), Some(5001), false)]
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
}
