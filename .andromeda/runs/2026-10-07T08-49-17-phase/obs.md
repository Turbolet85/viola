# obs extract

## Relevance
partial — the chunk is live measurement on the real CLI; obs binds only where a probe path adds a `verify` spawn or a recorded env-name list, where a live `send` / `wait` reading is taken from process logs, and where a paste-unwrap or leftover-dir fix would touch a logged path.

## Constraints
- obs-plan §4 Edge flows (`verify` / `plugin install`) requires every `viola verify` child spawn to be a `process-start{subject}` / `process-exit{subject, child_exit_status, duration_ms}` pair in `cli-<name>.ndjson` when `VIOLA_NAME` is set, with no row text, prompt or path in any line; it fixes today's count at six spawns (one `version-probe`, one `verify-probe`, four `verify-pty-probe`). A probe path that adds a spawn (W1's second session, W2's env-name probe) moves that count and is a wrap amendment target; whether the code logs at the call site as required is research's question.
- obs-plan §6 Boundary-call wrappers requires `subject` to stay the closed `$defs.subject.enum` of `schemas/diag-line.v1.json`; a new spawn either reuses an existing value or needs the schema, the §6 catalog and a Decisions Log entry moved together.
- obs-plan §4 Edge flows (the hidden `hook --capture` arm) requires that arm to stay uninstrumented: no `VIOLA_*` read, no obs init, no role or detail file, no `hook-invoked` / `hook-decision`, stderr empty, exit 0. W2's widening (the arm recording environment names) is outside that ruled shape from the obs side too; nothing in obs-plan admits it before the founder's card.
- obs-plan §4 Edge flows (E2, R8 strip) requires environment data to be names only, comma-joined, never values, with no env map passed to a span or event; the only admitted name-bearing fields are `env_stripped_known` / `env_kept` on `process-start{subject:"claude-child"}`. Whether that existing line can carry W2's reading on 2.1.287 is research's question.
- obs-plan §8 PII Scrubbing requires `CLAUDE*` values to be never-log in every sink, detail files included, and prompt / send text (W3's paste shapes, W1's prefixed prompts) to stay out of every home-level line: `text_bytes` only.
- obs-plan §8 PII Scrubbing (integration point 6, detail-file upload) admits the `diag-<os>` / `harness-<os>` uploads only because every input is synthetic; a live-round home holds real CLI content, so it must never sit in an upload path. Whether any live round's home lands under a scanned or uploaded root is research's question.
- obs-plan §4 Scenario: Confirmed `send` requires a send refused at the gate to be one wrapper `send-refused{refusal, detail:"input-not-ready", side:"wrapper", wheel}` with `corr` the end offset, `conn` and `rpc_id`, and no `send-issued`; `run.readiness_gate` carries `outcome` (`ready|input-not-ready`) only. W4's in-window `send` adds no field, detail or event before the founder's answer.

## Patterns to follow
- Read W4's live `send` from the wrapper's `send-refused` line and its `channel-response{result_class, refusal, detail, duration_ms}` pair, joined by `(conn, rpc_id)` = `(conn, corr)` (per obs-plan §4 Scenario: Confirmed `send`), not from the screen.
- Read W4's live `wait` from `channel-response{method:"wait"}.outcome` (the waking kind or `timed-out`) and the client `process-exit{exit_code, detail, during}` (per obs-plan §4 Scenario: `wait` / `last` event-driven readback): the "no later event to wake on" hypothesis is settled by that field.
- With no instance resolved, `verify`'s agent-readable outcome is its exit code plus stdout: one `[NN/MM] <row id> <row words>  pass|fail` line per ledger row and the last line `stamped <version>  N pass  N fail` (per obs-plan §4 Edge flows); a landed row moves `MM` with `LEDGER_ROWS`.
- A ledger read failure stays `parse-rejected{parser:"ledger-stamps", detail, count}` with `cli_verified:false`, path and stamp contents never logged (per obs-plan §4 Edge flows, Capability ledger); a re-verify after a row lands uses this path unchanged.
- Any new log call goes through `obs_event!` under `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`, e.g. `cli.verify_step` per step (per obs-plan §4 Span / Trace Coverage).

## Anti-patterns to avoid
- NEVER derive telemetry from screen content: the hint-window timing is a test-side measurement, and the product logs gate outcomes only (per obs-plan §11 Obs Anti-Patterns § Telemetry Strategy).
- NEVER log a field outside the §6 catalog or a `CLAUDE*` value at any level in any file (per obs-plan §11 Obs Anti-Patterns § PII Scrubbing).
- NEVER add an `event` value or rename a tests field without a Decisions Log entry (per obs-plan §11 Obs Anti-Patterns § Logs).

## Contract bindings
- obs ↔ tests: `schemas/diag-line.v1.json` (obs-owned) closes `subject` and the per-event field lists; the G4 check body and the validator are tests-owned (obs-plan §9 Gate commands). A new `verify` spawn subject or field moves both sides.
- obs ↔ tests: the secret-scan test carries one canary per identity-floor name plus an unknown name (obs-plan §8 PII Scrubbing); if W2's reading moves `IDENTITY_FLOOR`, the tests-owned canary set follows.
- obs ↔ tests: the real-`claude` verify never runs in CI and `run --local-live` refuses under `CI`; no obs gate depends on verify's own lines (obs-plan §4 Edge flows) — ties to test-plan §3 5-command implementation's `--local-live` bullet (W6).
- obs ↔ security: the NEVER-log floor for R8-stripped `CLAUDE*` values and the capture arm's ratified shape (obs-plan §8 PII Scrubbing; security-plan §Authentication & Authorization, the `--capture` ruling) — W2's widening is a security Decisions Log matter first.
- obs ↔ architecture: refusal details are the arch `RefusalReason` details verbatim (obs-plan §6 `detail` code catalog); a hint-window remedy that needs a new detail is arch-owned and mirrored in §6, after the card.
- obs ↔ design: `verify`'s stdout step and summary lines are design-system §Streams' format (obs-plan §4 Edge flows).

## Acceptance criteria contributions
- (obs) Every `verify` child spawn this chunk adds is a `process-start` / `process-exit{child_exit_status, duration_ms}` pair with a `subject` from the closed enum and no row text, prompt or path, when `VIOLA_NAME` is set (per obs-plan §4 Edge flows).
- (obs) No `CLAUDE*` value appears in any `diagnostics/` line, detail file, fixture or evidence file the chunk writes; environment data is names only (per obs-plan §8 PII Scrubbing).
- (obs) G2 (zero panics), G4 (schema conformance, no field outside the catalog) and the secret scan read clean over every test home the chunk's tests create under `target/e2e-home/` (per obs-plan §10 SLO Invariants & Telemetry Budgets, Error budget).
- (obs) If a ledger row lands, `verify`'s stdout carries one step line per ledger row and a last line whose pass / fail counts sum to the row count (per obs-plan §4 Edge flows).
