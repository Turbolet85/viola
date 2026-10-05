# obs extract

## Relevance
partial — W3d-a drives the must-trace Dialog → `answer` path and the `wait` wake for the `permission` kind through the fake agent (obs line, schema and scan gates apply); W3d-b touches obs only if it builds a body or a new `null` reason; W6 and F1 are tests-owned.

## Constraints
- obs-plan §4 Scenario "Dialog → `answer` (question / permission / plan)" requires, for a `permission-request` dialog, the wrapper lines `dialog-raised{corr:<dialog_id>, dialog_kind, hook_event}` then `dialog-answered{corr:<dialog_id>, dialog_kind, from, from_trust, deadline_hit, duration_ms}`, and the hook lines `hook-invoked{corr:<dialog_id>, hook_event, stdin_bytes, invoked_at}` (written only after the `hook.dialog` response, D-07) then `hook-decision{corr:<dialog_id>, hook_event, decision_emitted, deadline_hit, detail, duration_ms}`; spans `hook.handle` › `channel.request(hook.dialog)` › `run.dialog_register` (`dialog_kind`, `pending_conflict`) › `run.dialog_await` › `hook.decision_emit`, driver side `answer.client` › `channel.request(answer)`. Whether the `permission-request` arm already emits all four lines with these fields is research's question.
- obs-plan §4 Scenario "`wait` / `last` event-driven readback" requires `run.wait_dispatch.woken_kind` and the `wait` `channel-response.outcome` to be `permission` when a PermissionRequest wakes the parked `wait`, derived once in `viola-channel` over the shared wake set; no result body on either side.
- obs-plan §3 Log format JSON schema (`log-format-json-schema.md`) fixes `corr` = `dialog_id` on `dialog-*` lines and on dialog `hook-*` lines; `schemas/diag-line.v1.json` requires `corr` on `hook-decision` when `hook_event` is `permission-request` and no `detail` is set; null is key absence, never a literal `null`.
- obs-plan §8 PII Scrubbing (data classification) puts tool `input`, dialog questions and answers (including a `deny` answer's `message`) in the High class: they appear in no home-level line, only in `detail-<process>.ndjson` inside a chain or drift report, with `#[redact]` on `HookDialogParams.data` and `AnswerParams.response`.
- obs-plan §9 Pipeline integration (Integration tests row) requires the new case's viola homes under `target/e2e-home/`, so that G2, G4, the secret scan and the `diag-<os>` upload cover them. The only carve-out is `tests/chaos_feed_panic.rs`'s `outside_scan()` home.
- obs-plan §6 `detail` code catalog and §8 Default-deny posture: a W3d-b outcome (a question body built from PermissionRequest, or the `null` to the human kept) uses only existing catalog fields and codes (`decision_emitted:false` with empty stdout, or a listed fail-open `detail`). A new `detail`, field or `event` value needs a Decisions Log entry and a `schemas/diag-line.v1.json` change.
- obs-plan §10 Standard+ invariants (correlation propagation, deterministic emission) require every dialog line to carry `dialog_id`. The case's waits key on hook events and `events.ndjson` offsets, never on sleeps or screen content.

## Patterns to follow
- Join the hook process's lines and the wrapper's lines by `instance` + `corr` = `dialog_id`, since there is no inbound context from Claude Code (per obs-plan §3 Trace context propagation, "Claude Code → hook stdin"). Join the `answer` client and wrapper lines by `(conn, corr)`.
- Any new emission (a question body in `dialog.rs`, a branch in the `src/cmd/hook/` `permission-request` arm) logs only through `obs_event!` with explicit typed fields, and passes `corr` / `instance` as explicit values. Spans are `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]` with static names (per obs-plan §4 span naming convention, §8 Integration points 1).
- When the case asserts telemetry, mirror the `question` and `plan` cases in `tests/cli_answer.rs`. Select lines with `jq`-equivalent filters by `event` + `corr`, as in obs-plan §3 Log format JSON schema "Agent parsing". Whether those cases assert diagnostics lines at HEAD or only `events.ndjson` / exit codes is research's question.
- On a `null` decision, the hook writes `hook-decision{decision_emitted:false}`, exits 0 and leaves stdout empty, so the dialog renders for the human (per obs-plan §4 Dialog Scenario Cleanup; §7 per-role behaviour `hook`).

## Anti-patterns to avoid
- A bare `#[instrument]`, `err` / `ret`, or a `?value` capture on PermissionRequest payloads or `AnswerParams`. These leak tool `input` or the deny `message`, and dialog ids must never appear in span names (per obs-plan §11 Spans / Traces).
- Adding an `event` value or field, or renaming a tests field, for the permission case or W3d-b without a Decisions Log entry (per obs-plan §11 Logs; §11 PII Scrubbing "default-deny").
- A test home outside `target/e2e-home/`, a retry-once, or a sleep-based wait to steady the case (per obs-plan §11 SLO; §9 Integration tests row). F1's designed-floor exception moves a bound and adds no retry: `retries = 0` stays.

## Contract bindings
- obs ↔ tests §3 log format: the case's assertions and gate G4 read `schemas/diag-line.v1.json` / `diag-detail.v1.json`. Obs owns both schemas; the check body is tests-owned (obs-plan §9 G4).
- obs ↔ security (NEVER-log floor, High class): the §8 item 6 canary rule binds the tests-owned secret scan. The case replays recorded `PermissionRequest.permission-1.json` / `PostToolUse.permission-1.json` rather than a synthetic payload. Whether that replayed content carries the tests-owned canary is research's question. The `diag-<os>` detail-file upload is admissible only because every input is synthetic (obs-plan §8 item 6).
- obs ↔ arch Standard Contracts (`hook.dialog`, `answer`): `corr` = the wrapper-assigned `dialog_id` that `viola answer` addresses (obs-plan §3 Trace context propagation).
- obs ↔ tests §10 / F1: the zero-flakiness ↔ zero-unlogged-panics parallel (obs-plan §10 Cross-input parallel). If a planted-hang control is added, it must leave no unexempted `event:"panic"` line in G2 scope (obs-plan §9 G2).

## Acceptance criteria contributions
- (obs) After the `permission` case, the run's home-level diagnostics hold exactly one `dialog-raised{dialog_kind:"permission", hook_event:"permission-request"}` and one `dialog-answered`, both with `corr` = the `dialog_id` that `viola answer` used. The hook's `hook-invoked` / `hook-decision` carry the same `corr`, with `decision_emitted:true` for `allow` and for `deny` (per obs-plan §4 Scenario Dialog → `answer`).
- (obs) The parked `viola wait`'s `channel-response` on both sides carries `outcome:"permission"`, with no result body (per obs-plan §4 Scenario `wait` / `last`).
- (obs) G2 (zero panics) and G4 (schema conformance) exit 0 over the case's home under `target/e2e-home/` on all three CI OSes (per obs-plan §9 Gate commands; §10 Error budget).
- (obs) The PermissionRequest tool `input` and the `deny` `message` appear in no home-level `diagnostics/*.ndjson` line, and the secret scan reports 0 hits (per obs-plan §8 PII Scrubbing, Integration points 6).
