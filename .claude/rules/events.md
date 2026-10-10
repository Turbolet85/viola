---
paths:
  - "crates/viola-core/src/**"
  - "crates/viola-state/src/**"
  - "crates/viola-agent-claude/src/**"
  - "src/run/**"
---

# Event Rules

Path-scoped rules for viola's event log and state: the normalised event kinds, the `events.ndjson` line, snapshots and the hook → kind mapping. Authoritative source: `.andromeda/architecture.md` §Standard Contracts + §Conventions (Naming patterns, Data model conventions). There is no broker: the log on disk is the single source of truth.

## Event line
- `{"v":1,"ts":"<RFC3339 ms Z>","instance":"<ViolaName>","kind":"<kind>","source":"hook|wrapper|cli","data":{}}` — one complete object + `\n` per single `write` call; multi-line text travels as an escaped string.
- `data` is kind-specific and uses only viola's normalised fields; it never embeds a raw Claude payload.
- Normalised kinds live only in `viola-core`, never with Claude-specific names: `session-start`, `turn-ended`, `prompt-submitted`, `question`, `permission`, `plan`, `session-end`, `activity`, `link`, `unlink`, `wheel`, `budget-gate`, `send-issued` `{cursor, from?}`, `send-confirmed` `{cursor, confirmed?}` (`confirmed:false` only for an unconfirmable send), `send-refused` `{refusal, detail, cursor?}` (the CL-1 send records, `source: wrapper`; `cursor` absent for a refusal before `send-issued`).
- `wait` wakes only on `turn-ended`, `question`, `permission`, `plan`, `session-end`; `activity`, `wheel`, `budget-gate` and the send records are log-only.

## Hook → kind map (only in `viola-agent-claude`)
- SessionStart → `session-start` (`cause` startup|clear|resume|compact|unknown, `agent_session_id`); UserPromptSubmit → `prompt-submitted` (`text` normalised: long-paste wrapper removed, then tag escaping reversed; `origin` driver|human|harness — the hook files only human|harness, and `driver` comes only from the wrapper's relabel of the in-flight send's own prompt, matched exactly on the send's typed text, the sent text with every CR LF pair and every other CR as one LF and without its trailing CR and LF characters); PreToolUse AskUserQuestion → `question`; PreToolUse ExitPlanMode → `plan`; PermissionRequest → `permission` (a PermissionRequest repeating the armed `question` / `plan` dialog's tool with an equal `tool_input` is that dialog's continuation and raises no event); Stop → `turn-ended` (`last_assistant_message` or `null`); SessionEnd → `session-end`; Notification / PostToolUse / PostToolUseFailure → `activity`.
- Dialog hooks send only `hook.dialog`; the wrapper assigns `dialog_id` (monotonic, restored from the log on start) and appends the dialog event before it can wake `wait` or reply — even when it replies `null`. At most one dialog pending per instance.
- Harness turns (a prompt whose raw start is `<agent-message from=`, `<task-notification>`, `<\cross-session-message` or `<cross-session-message`) are `origin:"harness"` and never flip the wheel. The escaped cross-session form is the one escaped tag that classifies, so a human typing it at a prompt's start is filed `harness` (founder-ratified, 2026-09-29). As measured on 2.1.287, a human who types `<task-notification>` at a prompt's start is filed `harness` too.

## Cursors and retention
- `wait after`, the `send` `cursor` and SSE ids are byte offsets into the instance's `events.ndjson`: never truncate, rotate or rewrite it; instance dirs are reused and appended to.
- The link set is derived by replaying `link`/`unlink`; a repeated pair adds no line.

## Snapshots and liveness
- Envelope `{"v":1,"written_at","writer","data"}`, replaced atomically (tempfile `persist` through the one shared helper `viola_state::fs::replace_private`); only the instance's wrapper writes `instances/<name>/snapshot.json`.
- An unsupported `v` or a parse failure → ignore the snapshot and replay the log; replay recovers only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until` (`dialog_pending` reads false). As landed the classified read and the replay are library code (`viola_state::snapshot::read_snapshot_classified`, `viola_state::replay`) with one product reader, `viola revive`'s preflight; the other four snapshot readers still call `read_snapshot`. The replay writes no file, gives no `cwd` (a revive over a replayed snapshot is refused `cwd-missing`), and `links` replays empty until the link kinds land. `replay::session_chain` reads the logged session ids in one pass and logs nothing.
- Heartbeat touched every 1 s. A snapshot pid that is dead, or alive with another start time, is `gone` whatever the beat; otherwise a beat ≤ 5 s old is `live`, an older or absent one `stale`. Never a bare pid.

## Parsing
- The next append heals a torn last line: under the log's lock it writes one LF ahead of its line in the same single write, and logs one `state-recovered` line. A reader never writes. The one reader `events::read_from` counts what it steps over, per read, as `skipped{unknown_kinds, unknown_fields, torn_lines}`, never fatal: an unterminated last line, an over-long line and a line that is not one object are torn; a line of an unknown kind is not returned; a known-kind line with a key outside its contract is returned and counted once. No surface shows the counts yet. External payloads parse tolerantly (`#[serde(default)]`, `Option<T>`, no `deny_unknown_fields`), with serde_path_to_error drift reports going to the instance detail file only.
- Lock files are separate siblings (`<name>.lock`): append + exclusive lock fails on Windows.
- Timestamps via chrono `to_rfc3339_opts(SecondsFormat::Millis, true)`; malformed external values become `"unknown"`, never an error. JSON fields snake_case, enum values kebab-case.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
