# obs extract

## Relevance
Relevant. The chunk lands the `viola run` start sequence (collision check, pin copy, snapshot, heartbeat, child spawn env), which is obs-plan's must-trace Scenario 1, and it creates `viola-state`, one of the instrumentable crates.

## Constraints
- **Init before the collision check.** obs-plan §3 OTel SDK init, "Per-role anchors: `run`", requires init steps 1–6 (panic hook, clap, home/instance resolution, home strict-modes, diagnostics open, `process-start`) to finish before `run.collision_check`. That way every exit-1 refusal is logged. For `run`, the instance comes from its own `ViolaName` argument; an inherited `VIOLA_NAME` must never override it (§3 init step 3). Whether the current `src/run/` already meets this order is research's question.
- **Scenario 1 spans.** obs-plan §4 "Scenario: `viola run` start sequence to child spawn" requires the span chain `run.start` › `run.collision_check{outcome: free|already-live|squatted-name}` › `run.pin_copy{outcome: ok|pinned-hash-mismatch}` › `run.version_gate` › `channel.bind` › `state.snapshot_write{v}` › `state.heartbeat_start` › `pty.spawn{pty_backend, env_stripped_count}`. `channel.bind` belongs to the channel chunk.
  - Stale takeover has no `outcome` value in `run.collision_check`'s closed set. Adding one (or a new event) needs a Decisions Log entry (§11 Logs, the rename/add ban).
- **Exit-1 detail codes.** Per obs-plan §6 "`detail` code catalog" and §12 D-20, a refused start logs `process-exit{subject:"self", exit_code:1, detail}` at `error` (§6 Log levels mapping).
  - `detail` must be `already-live` (live-name refusal) or `pinned-hash-mismatch` (re-hash failure). `squatted-name` is left to the channel chunk.
  - Codes only: no path or pid (Founder Direction 4; §11 PII Scrubbing).
- **Fields that are never logged.** Per obs-plan §8 Data classification rules:
  - snapshot `endpoint` and `pinned_bin`: only `endpoint_kind` is allowed;
  - absolute paths in home-level lines: `file` fields are basenames;
  - `CLAUDE*` and `VIOLA_*` env values at spawn: "env maps never passed to a span or event". R8 is logged as names only (`env_stripped_count`, `env_stripped_known`, `env_kept`; D-34).
  - Per §1 (Persistent stores entity), `events.ndjson` payload content is never copied into process-log lines.
- **Heartbeat.** obs-plan §3 "Heartbeat ticks" and §10 "Liveness signal" (D-13) require that the 1 s heartbeat touch is **not** logged per tick. `liveness-changed` (D-02) is emitted by the `ui` reader, not by `run`.
- **Default-deny.** obs-plan §8 "Default-deny posture" allows only fields named in the §6 Additive field catalog. `schemas/diag-line.v1.json` (`unevaluatedProperties: false`) fails anything else in gate G4. If `viola-state` skips a symlinked entry during a scan, it must emit `parse-rejected{parser:"state-entry", detail:"symlink-ignored", count}` once per scan at `warn` (§6 Filesystem refusals).
- **New crate setup.** obs-plan §3 Bootstrap phases (logger-stack-install, obs-ci-gate-wire) require `viola-state` to:
  - take `tracing = "0.1.44"` and no tracing-subscriber;
  - stay free of Tokio and C crates (§1 viola-state entity: "instrumentation must be sync");
  - carry `[lints] workspace = true`, so the `print_*` and `dbg_macro` bans apply. The CI member-list assertion expects every product member to have it.

## Patterns to follow
- Emit every line through `obs_event!`, which attaches `event`, `process` and `instance` from `ProcessCtx` (obs-plan §3 Logging stack). `corr` is absent on these events (Scenario 1: `corr` null, key-absence encoding, §3 Null encoding).
- Declare spans as `#[instrument(skip_all, name = "state.snapshot_write", fields(v))]`: static `<area>.<operation>` names, explicit field allow-list, no `err`/`ret` (obs-plan §4 Span naming convention; §11 Spans / Traces).
- For a heartbeat refresher on its own std thread, pass a cloned `tracing::Span` into the thread and run under `span.in_scope(...)`. Pass `instance` explicitly as a value; never recover it from span context (obs-plan §3 Trace context propagation, "Internal async boundaries").
- Use the "one `write_all` per complete line on an append-mode handle" discipline from obs-plan §3 Logging stack for the diagnostics writer. The same rule the chunk applies to `events.ndjson` keeps the harness's torn-line tolerance meaningful (§3 Snapshot / paste-to-AI integration: `agent-run logs` merges `events.ndjson` by byte `offset` and emits torn lines as `{"torn":true}`).
- The child `process-start{subject:"claude-child", child_pid, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept}` is emitted on successful spawn (obs-plan §4 Scenario 1 Required log fields; §6 catalog `process-start` row).

## Anti-patterns to avoid
- No bare `#[instrument]`, no raw `tracing::{info,warn,error,debug,trace,event}!`, and no `print!`/`eprintln!`/`dbg!` in `run` or `viola-state` paths (obs-plan §11 Spans / Traces; §11 Logs). The only allowed `run` stderr site is the pre-spawn `.cmd`/`.bat` refusal function.
- No log line per heartbeat tick. Diagnostics files are never rotated (obs-plan §11 SLO).
- No `endpoint`, `pinned_bin`, home or plugin paths, or env values in any span field, `detail` code or home-level line (obs-plan §11 PII Scrubbing; §8 table).

## Contract bindings
- **obs ↔ tests §3 (harness `logs`):** start-event order (`wheel{cause:"start"}` → `budget-gate` → `session-start`) is checked from `events.ndjson` through `agent-run logs --kind`. It is not duplicated into process logs (obs-plan §4 Scenario 1). The event-line shape `{"v":1,"ts","instance","kind","source","data"}` is the tests-bound stream in §3 Log format JSON schema.
- **obs ↔ tests (concurrent append):** a second `viola run <name>` on a live name appends its `process-start` and its exit-1 `process-exit{detail:"already-live"}` to the live process's `run-<name>.ndjson`. The tests-owned concurrent-append check covers this collision path (obs-plan §3 Logging stack; D-29).
- **obs ↔ spawn env:** `hook` and `mcp` resolve the home as the grandparent of `VIOLA_DIR` (obs-plan §3 init step 3; D-09). The `VIOLA_DIR` this chunk sets at spawn must therefore be `<home>/instances/<name>`, or wrapped hooks and mcp write diagnostics under the wrong home.
- **obs ↔ security (Vector 7 strict-modes):** the home is strict-mode-checked before anything under it, `diagnostics/` included, is created (obs-plan §3 init step 4). `strict-modes-failed` is catalogued only on exit 21 and `hook-decision` (§6). A strict-modes refusal on the `run` name-refusal read has no catalogued `run` exit-1 code. If that half lands here, it needs a Decisions Log entry; otherwise it defers with the scope's Epoch-6 premise.

## Acceptance criteria contributions
- A second `viola run <name>` against a live instance leaves, in `<home>/diagnostics/run-<name>.ndjson`, its own `process-start{subject:"self"}` followed by one `level:"ERROR"` `process-exit{subject:"self", exit_code:1, detail:"already-live"}` line, with no path or pid in any field (per obs-plan §4 Scenario 1; §6 `detail` code catalog).
- A tampered pinned copy yields exit 1 and exactly one `process-exit{exit_code:1, detail:"pinned-hash-mismatch"}` line. No home-level line contains the `pinned_bin` path, the `endpoint` string or any env value (per obs-plan §6 `detail` code catalog; §8 Data classification rules).
- Every `diagnostics/*.ndjson` line the chunk's start-order and refusal tests produce passes G4 against `schemas/diag-line.v1.json`: no event or field outside the §6 catalog, and zero `event:"panic"` lines (G2) (per obs-plan §8 Default-deny posture; §9 Gate commands).
- In a successful `viola run` that stays up for 5 s or more, the number of `run-<name>.ndjson` lines does not grow with heartbeat ticks (no per-tick line) (per obs-plan §11 SLO; §3 Heartbeat ticks).

## Relevant amendment history
- **2026-09-25-pty-wrapper-on-windows** (§4 Scenario 1, §6 `process-start`, §8 R8 row, D-34):
  - `batch-script-child` is decided by program resolution before the strip plan and `pty.spawn`, not by `run.pin_copy`.
  - The claude-child `process-start` gained `env_kept` (names only).
  - A proposal to make `viola-pty` tracing-free was rejected. The record says no `#[instrument]` existed workspace-wide at that point, and a route CARRY hands the first spans (`pty.spawn`, seam spans) to "Wrapper channel". So whether this chunk or the channel chunk lands the Scenario 1 spans (`run.start`, `run.collision_check`, `run.pin_copy`, `state.snapshot_write`, `state.heartbeat_start`) is a sequencing premise for P3 to close.
- **2026-09-24-diagnostics-plane** (§3 Logging stack, §8 default-deny): `obs_event!` attaches only `event`, `process` and `instance`; `corr` is a caller-supplied typed field. Default-deny is enforced by a top-level `unevaluatedProperties: false`. A CARRY on the wrapper-channel chunk makes `corr` required for corr-bearing events. That does not affect this chunk's events, which all have null `corr`, but any new field it emits must be in the schema.
- **2026-09-24-observability-gates** (§3 logger-stack-install, §11 Logs, D-33): the raw-tracing ban is clippy `disallowed-macros` on the level macros plus a fail-closed raw-`event!` grep in `scripts/lint-probes.sh`. It applies to the new `viola-state` crate.
- **2026-09-24-three-os-ci-headless-harness-skeleton** (§3 obs-ci-gate-wire): the CI member-list assertion names only `viola-e2e` as lacking `[lints] workspace = true`. A new `crates/viola-state` must carry it or that assertion fails.
