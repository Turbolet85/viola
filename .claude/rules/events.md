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
- Normalised kinds live only in `viola-core`, never with Claude-specific names: `session-start`, `turn-ended`, `prompt-submitted`, `question`, `permission`, `plan`, `session-end`, `activity`, `link`, `unlink`, `wheel`, `budget-gate` (+ the CL-1 send issue/outcome records per requirements v1-10).
- `wait` wakes only on `turn-ended`, `question`, `permission`, `plan`, `session-end`; `activity`, `wheel`, `budget-gate` and the send records are log-only.

## Hook → kind map (only in `viola-agent-claude`)
- SessionStart → `session-start` (`cause` startup|clear|resume|compact|unknown, `agent_session_id`); UserPromptSubmit → `prompt-submitted` (`text` normalised: long-paste wrapper removed, then tag escaping reversed; `origin` driver|human|harness); PreToolUse AskUserQuestion → `question`; PreToolUse ExitPlanMode → `plan`; PermissionRequest → `permission`; Stop → `turn-ended` (`last_assistant_message` or `null`); SessionEnd → `session-end`; Notification / PostToolUse / PostToolUseFailure → `activity`.
- Dialog hooks send only `hook.dialog`; the wrapper assigns `dialog_id` (monotonic, restored from the log on start) and appends the dialog event before it can wake `wait` or reply — even when it replies `null`. At most one dialog pending per instance.
- Harness turns (`<agent-message from=`, `<task-notification>`) are `origin:"harness"` and never flip the wheel.

## Cursors and retention
- `wait after`, the `send` `cursor` and SSE ids are byte offsets into the instance's `events.ndjson`: never truncate, rotate or rewrite it; instance dirs are reused and appended to.
- The link set is derived by replaying `link`/`unlink`; a repeated pair adds no line.

## Snapshots and liveness
- Envelope `{"v":1,"written_at","writer","data"}`, replaced atomically (atomic-write-file); only the instance's wrapper writes `instances/<name>/snapshot.json`.
- An unsupported `v` or a parse failure → ignore the snapshot and replay the log; replay recovers only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until` (`dialog_pending` reads false).
- Heartbeat touched every 1 s; older than 5 s = gone unless pid + start time are alive (`stale`). Never a bare pid.

## Parsing
- Readers heal a torn last line and count it; unknown kinds and fields are skipped and counted (`skipped`), never fatal. External payloads parse tolerantly (`#[serde(default)]`, `Option<T>`, no `deny_unknown_fields`), with serde_path_to_error drift reports going to the instance detail file only.
- Lock files are separate siblings (`<name>.lock`): append + exclusive lock fails on Windows.
- Timestamps via chrono `to_rfc3339_opts(SecondsFormat::Millis, true)`; malformed external values become `"unknown"`, never an error. JSON fields snake_case, enum values kebab-case.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
