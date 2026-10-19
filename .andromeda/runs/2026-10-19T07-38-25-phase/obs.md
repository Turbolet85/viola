# obs extract

## Relevance
relevant — the chunk changes the dialog hold's deadline (a must-trace path's `deadline_ms` / `deadline_hit`), adds a record of an expired hold that `wait` wakes on, and may add a config value, each touching obs-plan's closed event, field and `outcome` enums.

## Constraints
- A new process-log `event` value (for example a dedicated expired-hold line) is a closed-enum extension: obs-plan §3 Log format JSON schema requires a Decisions Log entry for it, its `corr` rule stated (a dialog line's `corr` is `dialog_id`), and the tests' Log format amended to match, as the D-21 values were. Whether the existing `dialog-answered{deadline_hit}` / `hook-decision{deadline_hit}` pair already records the expiry well enough to need no new value is research's question (obs-plan §3 Log format JSON schema).
- The Dialog → `answer` scenario requires the `run.dialog_await` span to carry `deadline_ms` and `deadline_hit`, the wrapper's `dialog-answered` line to carry `deadline_hit` and `duration_ms` under `corr` = `dialog_id`, and, at the deadline, the hook to write `hook-decision{decision_emitted:false, deadline_hit:true}` and exit 0 with empty stdout; a longer or configurable hold changes the value `deadline_ms` reports, not the shape. Whether the wrapper writes `dialog-answered` at all on an expired hold today is research's question (obs-plan §4 Scenario: Dialog → `answer`).
- The `wait` scenario closes the waking kind twice, `run.wait_dispatch` `woken_kind` and the `channel-response` `outcome` for method `wait` (`turn-ended|question|permission|plan|session-end` or `timed-out`), the latter derived once in `viola-channel` over the shared wake set; a new product kind that wakes `wait` has to enter both enums together, and `schemas/diag-line.v1.json` with them (obs-plan §4 Scenario: `wait` / `last` event-driven readback).
- Default-deny: any field the expiry path or a hold setting puts on a line (a hold length, a new `detail` word) must first be in the §6 Additive field catalog and in `schemas/diag-line.v1.json`, or G4 fails the build (obs-plan §8 Default-deny posture).
- The `detail` code catalog is closed, kebab-case and never a path or pid; a bad hold value in `config.json` is one `parse-rejected{parser:"config-json"}` line with a catalogued detail (`malformed` · `unreadable` · `unknown-keys`), at `warn` (obs-plan §6 `detail` code catalog).
- Codes only: the question, the answer and tool `input` never appear in any line of the dialog path, and `events.ndjson` payloads are never copied into process logs (obs-plan §4 Scenario: Dialog → `answer`; obs-plan §8 PII Scrubbing).
- Every dialog line carries `dialog_id` as `corr`, copied unchanged (obs-plan §10 Standard+ invariants, Correlation propagation).

## Patterns to follow
- Emit only through `obs_event!` under `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]` with an explicit field allow-list, never `err` / `ret` (obs-plan §4 Span naming convention).
- Product-event order is verified from `events.ndjson` through `agent-run logs --kind`, not duplicated into process logs: the expired-hold record, if it is a product kind, is checked the same way (obs-plan §4 Scenario: `viola run` start sequence to child spawn).
- Level mapping: a state transition or refusal is `info`; a `hook-decision` with a fail-open `detail` (`deadline`) is `warn` (obs-plan §6 Log levels mapping).
- Config values such as thresholds are logged structured as low-confidentiality values; a hold setting follows the same rule (obs-plan §8 PII Scrubbing, data-class table).

## Anti-patterns to avoid
- Never add an `event` value or rename a tests field without a Decisions Log entry (obs-plan §11 Logs).
- Never put a dialog id in a span name or content in a span field; never a bare `#[instrument]` on the `hook.dialog` path (obs-plan §11 Spans / Traces).
- Never derive the expiry from screen content: events key on hook events and `events.ndjson` offsets (obs-plan §11 Telemetry Strategy; obs-plan §10 Standard+ invariants, Deterministic emission).

## Contract bindings
- obs ↔ tests §3 Log format: a new `event` value or `outcome` word changes the reproduced binding text and `schemas/diag-line.v1.json`, which G4 checks over every home-level line under `target/e2e-home/` (obs-plan §3 Log format JSON schema; obs-plan §9 G4).
- obs ↔ arch §Standard Contracts Event line: the expired-hold record's `kind` is arch-owned; obs follows with the `woken_kind` / `outcome` enums (obs-plan §4 Scenario: `wait` / `last` event-driven readback).
- obs ↔ tests perf gate: the `pre-tool-use` hyperfine row runs on an unverified CLI and is gated `max <` the spine deadline; moving the hook's own wait (`DIALOG_DEADLINE + REPLY_GRACE`) must not move that row (obs-plan §10 Performance budgets).
- obs ↔ security §Data Protection / NEVER-log floor: codes-only dialog lines (obs-plan §8 PII Scrubbing).

## Acceptance criteria contributions
- An expired hold leaves, under `corr` = its `dialog_id`, `deadline_hit:true` on the wrapper's dialog line and `hook-decision{decision_emitted:false, deadline_hit:true}` at `warn` in `hook-<name>.ndjson`, every line passing `schemas/diag-line.v1.json` (per obs-plan §4 Scenario: Dialog → `answer`)
- A `wait` woken by the expired-hold record logs, on both sides, `channel-response` with `outcome` equal to the new kind, and `run.wait_dispatch` `woken_kind` the same word (per obs-plan §4 Scenario: `wait` / `last` event-driven readback)
- No home-level line on the expiry or late-`answer` path holds question, answer or tool `input` text, and G4 reads zero schema-conformance failures (per obs-plan §10 Error budget)
- The `pre-tool-use` perf row stays green under `gate --require perf` after the hook wait moves (per obs-plan §10 Performance budgets)
