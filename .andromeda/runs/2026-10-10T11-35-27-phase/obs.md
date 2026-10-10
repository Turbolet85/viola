# obs extract

## Relevance
relevant — a new verb that starts a wrapper (a new process-log writer), the first product reader of the log
replay (the first product writer of two `state-recovered` details), new refusal codes, and a new child argv.

## Constraints
- **No role is named for `revive`.** obs-plan §7 Panic hooks (main-thread catch site) takes the role from argv and
  classes every verb outside `run` / `hook` / `mcp` / `ui` as `cli`; obs-plan §3 Log format JSON schema closes
  `process` to five values and obs-plan §3 Logging stack (Sink) fixes one role file per value. The wrapper arm of
  revive runs the `run` start order and then holds a child, so the plan must be read as requiring it to log under
  one of the existing values, with no sixth: which one fixes its role file, its instance source (obs-plan §3 OTel
  SDK init, step 3: a wrapper's instance is its own name argument, never an inherited `VIOLA_NAME`) and its
  panic exit path (obs-plan §7 Panic hooks, per-role behaviour). The plan does not say; this is a planning
  decision and a master gap for the wrap. `--list` is a short-lived read and falls under the `cli` rule as written.
- **Obs init completes before the preflight.** obs-plan §3 OTel SDK init (per-role anchors) requires init steps
  1–6 to finish before the collision check so every start refusal is logged; obs-plan §7 Error classes captured
  requires each refusal to end in one logged `process-exit{subject:"self", exit_code, detail}`. All four
  preflight refusals fall under it, and a refused revive spawns no child and writes no child `process-start`
  (obs-plan §4 Scenario: `viola run` start sequence to child spawn).
- **The `detail` catalog is closed and three of the four codes are not in it.** obs-plan §6 `detail` code catalog
  lists the `process-exit` details per exit code: the existing collision refusal is there as `already-live` (the
  scope's `instance-live`), while `cwd-missing`, `session-live` and `no-session` are absent. Each new code is a
  catalog extension, kebab-case, never a path or pid (Founder Direction 4). The line's level follows the exit
  code (obs-plan §6 Log levels mapping). Whether `schemas/diag-line.v1.json` closes `process-exit.detail`, so
  that G4 would refuse a new code, is research's question.
- **The replay's recovery line becomes a product line here.** obs-plan §4 Edge flows (E5) requires the snapshot
  read-or-replay to write one `state-recovered` line per call on its replay arm (unreadable or newer snapshot),
  at WARN with no `corr`, and none for a present or an absent snapshot or a failed replay, under the spans
  `state.snapshot_recover` and `state.replay`; the fields are those of obs-plan §6 Additive field catalog
  (`state-recovered`). Revive is the first process to write these, into its own role file. Whether the landed
  schema and the landed function already satisfy this for a product caller is research's question.
- **Default-deny on the new values.** obs-plan §8 Default-deny posture admits only fields named in the §6
  catalog. The recorded `cwd` is an absolute path: obs-plan §8 PII Scrubbing (data classification table) keeps
  home-level lines path-free and never logs path-valued snapshot fields. The session id is logged only as a
  typed field (same table), and no §6 process-log row names one today, so a revive line that carries the id
  needs a catalog and schema addition first. The extra args after `--` and the `--id` argv are never a field.
- **Spans stay inside the closed area list.** obs-plan §2 Telemetry Strategy (Naming conventions) closes the span
  `area` set and has no `revive` area: the start order reuses the `run.*` / `state.*` / `pty.*` spans and
  `cli.<verb>` names a short-lived dispatch. obs-plan §4 Span / Trace Coverage (naming convention) requires
  `skip_all`, a static name, an explicit field list, and neither `err` nor `ret`; the collision span's `outcome`
  is a closed set (obs-plan §4 Scenario: `viola run` start sequence to child spawn).
- **A `claude agents --json` read, if this chunk builds one, is a logged child spawn.** obs-plan §6 Boundary-call
  wrappers (Child / shell spawns) requires a `process-start` / `process-exit` pair with `subject:"agents-probe"`
  and, on a parse failure, `parse-rejected{parser:"claude-agents-json"}` with the value read as `unknown`;
  obs-plan §8 PII Scrubbing (data classification table) forbids logging a row verbatim.

## Patterns to follow
- The start sequence's span chain and log lines, reused unchanged for the revived wrapper: the child's
  `process-start{subject:"claude-child"}` with its closed field list (no argv, so `--resume` and the id are not
  on it), and the handle-wait `process-exit` (obs-plan §4 Scenario: `viola run` start sequence to child spawn).
- A pre-spawn refusal's human output is the fixed two-line `unable:` / `hint:` pair through `src/human.rs`
  `refuse`, one write, no print macro, no path or pid (obs-plan §3 Observability Harness Contract, intro list).
  The intro names `run` as that writer's only caller; a second caller is a sentence the wrap amends.
- Every line through `obs_event!` with explicit typed fields; `corr` absent on `process-start`, `process-exit`
  and `state-recovered` (obs-plan §3 Logging stack; obs-plan §3 Log format JSON schema).
- Product events are asserted from `events.ndjson` through `agent-run logs --kind`, never copied into process
  logs: the E2E's `session-start{cause:"resume"}` and its id are an event-stream read (obs-plan §4 Scenario:
  `viola run` start sequence to child spawn, product order bullet).
- A short-lived verb with a resolved instance logs `process-start` / `process-exit{exit_code, detail}` to
  `cli-<name>.ndjson`, and with none writes no file; its result is its stdout and exit code (obs-plan §4
  Instrumentation per surface, `cli (short-lived verbs)` row). This is the shape for `--list`.

## Anti-patterns to avoid
- No path and no pid inside a `detail` code, and no field outside the §6 catalog: the recorded cwd, a foreign
  session's pid and the resumed id may not ride a code or an uncatalogued key (obs-plan §11 Obs Anti-Patterns,
  PII Scrubbing). The scope's `session-live` "names the pid": on the obs side a pid is a typed field only.
- No new `event` value for revive and no renamed tests field without a Decisions Log entry; no print macro and
  nothing of viola's own on stdout or on the `run` terminal while a child runs (obs-plan §11 Obs Anti-Patterns,
  Logs).
- No bare `#[instrument]`, no `err` / `ret`, no instance name, id or path in a span name (obs-plan §11 Obs
  Anti-Patterns, Spans / Traces).

## Contract bindings
- **obs ↔ tests (gates over the E2E home).** The revive E2E's home must sit under `target/e2e-home/` so G2, G4
  and the secret scan read it (obs-plan §9 CI Integration, Pipeline integration, Integration tests row; homes
  kept until the gates ran, obs-plan §9 Step order and conditions). The test kills a wrapper: a non-JSON last
  line in a role file is skipped and counted by G2 and G4 (obs-plan §9 Gate commands), and the reviving
  process's first append to the killed wrapper's log may write `state-recovered{detail:"torn-line-healed"}`
  (obs-plan §4 Edge flows, Torn append), which no gate counts as red. Whether a pid kill leaves a torn last
  line on this host is research's question.
- **obs ↔ tests (harness readiness).** The harness `boot` stage `start_records` reads `events.ndjson` lines 1–3
  (obs-plan §4 Scenario: `viola run` start sequence to child spawn). A revived instance appends to an existing
  log, so that position check does not describe its start; the check is tests-owned (test-plan §3). The
  founder's open question on the revived wheel bears on the first record of that order.
- **obs ↔ tests (log format).** The `process` enum and the harness `--process` filter are tests-bound
  (obs-plan §3 Log format JSON schema ↔ test-plan §3 Log format): revive adds no value on either side.
- **obs ↔ architecture.** The four refusals' exit codes and code names are architecture §Conventions'; obs-plan
  §6 `detail` code catalog mirrors them and `schemas/diag-line.v1.json` is obs-owned.
- **obs ↔ security.** Home strict-modes runs before any diagnostics file is created or opened (obs-plan §3 OTel
  SDK init, step 4 ↔ security-plan §Authentication & Authorization); `strict-modes-failed` already stands in
  obs-plan §6 `detail` code catalog for a refused read. Path handling binds to security-plan §Error Handling.

## Acceptance criteria contributions
- (obs) Each preflight refusal leaves exactly one `process-exit{subject:"self", exit_code, detail}` line in the
  revive process's home-level role file, its `detail` a closed kebab-case code with no path and no pid, and no
  `process-start{subject:"claude-child"}` after it (per obs-plan §6 `detail` code catalog).
- (obs) A revive over an unreadable or a newer snapshot writes exactly one `state-recovered` line (WARN, no
  `corr`, `file` a basename) in its role file, and a revive over a readable snapshot writes none (per obs-plan
  §4 Edge flows, E5).
- (obs) G2 and G4 pass over the revive E2E's home: zero counted `panic` lines and every home-level line the
  revive and the killed wrapper wrote conforms to `schemas/diag-line.v1.json` (per obs-plan §9 Gate commands).
- (obs) No home-level `diagnostics/*.ndjson` line of a revive run holds the recorded cwd, the extra args, or
  the session id outside a catalogued typed field (per obs-plan §8 Default-deny posture).
