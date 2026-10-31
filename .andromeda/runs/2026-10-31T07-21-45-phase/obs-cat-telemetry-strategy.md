# obs · Telemetry Strategy — category read

## Carried
- (.andromeda/obs-plan-detail/telemetry-strategy.md:12) Every product error type that can reach one of the three surfaces must have a fixed `Display`, and its machine form must be a code, the signal's own strategy for errors.
  > typed thiserror 2.0.20 errors with fixed `Display`, exit codes plus distinct `detail` codes on `process-exit`
- (.andromeda/obs-plan-detail/telemetry-strategy.md:11) A refusal or fault this chunk sanitises must still be logged as a closed `event`; stripping a body must not remove its log line.
  > Every boundary call, state transition, refusal, fault/heal and process lifecycle, as the closed `event` enum (Section 3)
- (.andromeda/obs-plan-detail/telemetry-strategy.md:13) Correcting `viola.compat.v_rejected` must stay a `jq` derivation over a changed line; no counter or instrument may be added to tell the `-32602` causes apart.
  > No in-process instruments and no exporter. Metrics are derived by `jq` over the ndjson
- (.andromeda/obs-plan-detail/telemetry-strategy.md:20) A witness that forces a panic must find the payload and backtrace only in the instance detail file, never on a surface or in a home-level file.
  > The payload (one escaped JSON string) and the backtrace (a one-line JSON array of frames) are written only to the instance-scoped detail file (Section 7).
- (.andromeda/obs-plan-detail/telemetry-strategy.md:28) The channel error surface answers to this trigger; its `error.data` and log lines must meet the sections named here.
  > | logging-sensitive Vector 1 (IPC) | §4 surface table (channel), §6 channel field allow-list, §8 table |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:31) The MCP error surface answers to this trigger; its `isError` results and log lines must meet the sections named here.
  > | logging-sensitive Vector 5 (MCP) | §4 surface table (mcp), §6 |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:32) The planted stripped `CLAUDE*` value and the fixed-message requirement answer to this trigger, so the witnesses must prove both against these sections.
  > | logging-sensitive Vector 6 (CLI/env) | §4 Scenario 1 (`env_stripped_*`), §7 fixed messages, §8 |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:40) A sanitised handler fault must keep the zero-unlogged-panics invariant whole; a `-32603` or `"internal error"` body must not stand in for its panic line.
  > | error-budget-SLO | §7, §10 zero-unlogged-panics |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:43) The enumeration of `--json` error bodies must name these closed documents, and a pre-log strict-modes refusal must keep writing no line and carrying no `detail` code; the revive start-refusal documents owed to the exit-cause catalogue stay out of this chunk.
  > a strict-modes refusal made before any process log opens (the six frame verbs and `link` / `unlink`, exit 21; `run`, `revive`'s start and `verify`, exit 1) writes no line and carries no `detail` code: its machine-readable form is the exit code, and under `--json` the closed `instance-unreachable` document (the frame verbs) or `strict-modes-failed` (`revive`)
- (.andromeda/obs-plan.md:47) The root bin must be the one error-capture boundary, and a serde-sourced chain must become a fixed message there.
  > the only crate that uses anyhow, so the error-capture boundary: a chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to `diagnostics/` only.
- (.andromeda/obs-plan.md:67) The acceptance's drift-report leg and the planted `CLAUDE*` value must both meet this floor, across every sink and not only the three surfaces.
  > drift reports go to `diagnostics/` only, and R8-stripped `CLAUDE*` values are never logged anywhere.
- (.andromeda/obs-plan.md:95) An absolute path from the state stores or config must never reach an error body on any surface.
  > the absolute paths in `snapshot.json` and config values are scrubbed from external output.
- (.andromeda/obs-plan.md:111) The CLI surface must hold this standard: its `--json` results go to stdout, its stderr lines stay fixed, and neither carries a path, pid, chain or upstream text.
  > no stdout exporter (stdout carries `--json` results); the instance comes from `VIOLA_NAME` or an argument, never from the working directory; refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text.
- (.andromeda/obs-plan.md:123) The channel witness must assert this exact `-32603` body.
  > `-32603` is `"internal error"` with `data:null`, and the self-reported `from` is never logged as an authenticated identity.
- (.andromeda/obs-plan.md:125) The owed witness for a `viola mcp` stdin line past `MAX_FRAME` before `initialize` completes must be read against this surface rule, which ends the session as at a closed stdin.
  > its stdin read in lines of at most `MAX_FRAME`, a longer line refused whole and the session ended as at a closed stdin
- (.andromeda/obs-plan.md:127) MCP results and the log lines of rejected calls must carry codes only, and no telemetry may go to stdout.
  > stdout is forbidden because it carries MCP frames, so the writer is set explicitly; `isError` results and the log lines for rejected calls carry codes only.
- (.andromeda/obs-plan.md:205) The MCP witness must prove the absence of each of these three kinds of text.
  > rejected tool calls are logged with codes only, with no upstream text, absolute paths or type names.
- (.andromeda/obs-plan.md:210) The floor must hold in every sink and fixture, chains must be replaced before they are printed, and no switch may turn redaction off.
  > a never-log floor for every R8-stripped `CLAUDE*` value across all sinks and fixtures; the strip is logged as a count or variable names only, never values, and Phase 3 confirms the names are admissible; a serde-sourced anyhow chain becomes a fixed message before stderr; no env, flag or config may disable redaction.
- (.andromeda/obs-plan.md:280) Any new or changed signal must fit the pyramid: logs first, no reporter, no instruments, and no event payload copied into a process log.
  > errors have no external reporter; metrics have no in-process instruments and no exporter and are derived by `jq` over the ndjson; `events.ndjson` is the product audit stream, not a compliance trail, and obs never copies event payloads into process logs.
- (.andromeda/obs-plan.md:286) The proof's enumeration must be machine-checked: a new free-text body must fail an assertion that sets an exit code, not wait for a reader to notice it.
  > No human-review-gated log analysis. Every verdict is a `jq` / harness assertion that sets an exit code.
- (.andromeda/obs-plan.md:296) A value or field added to tell the `-32602` causes apart must follow these forms.
  > **Log fields:** snake_case field names. Enum values are kebab-case (arch JSON / enum naming).
- (.andromeda/obs-plan.md:361) The CLI witness must match this exact refused-instance document, and the MCP counterpart must return `instance-unreachable` with no line.
  > first, a home or instance tree the instance check refuses ends a CLI `wait` / `last` before any role file opens (exit 21, the instance pair, or exactly `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`; no line), and MCP's returns `instance-unreachable` with no line.
- (.andromeda/obs-plan.md:418) The carried `-32602` freight closes this over-count; the line must tell these causes apart, or the over-count stands and stays recorded.
  > `viola.compat.v_rejected` over-counts: `-32602` also answers `send`'s invalid params, `release` carrying `from` and a bad `link` / `unlink` name or pair, until the line tells the causes apart.
- (.andromeda/obs-plan.md:496) Each error type must keep a fixed-message `Display`, and anyhow must stay at the dispatch edge; the carried `Refusal` enum is not on this list, so folding it or ratifying it must leave this line true.
  > The mechanism is typed errors with fixed-message `Display` impls — hand-written on `PtyError` (`viola-pty` takes no thiserror), thiserror 2.0.20 on `ChannelError`, `StateError`, `AgentError`, `McpError` and `CoreError`. anyhow 1.0.104 is used at the root-bin dispatch edge and its catch-site reporter (`viola::obs::report_internal_error`) only
- (.andromeda/obs-plan.md:509) The chunk must make these three scrubbing layers hold on all named surfaces, with the full chain kept only in the instance detail file.
  > 1. fixed `Display` strings on every thiserror type (no fields holding paths or payloads);
  > 2. at dispatch, an anyhow chain whose source is a `serde_json::Error` / `serde_path_to_error::Error` is replaced by the fixed message `internal error` before it reaches stderr, `--json`, MCP `isError` or channel `error.data`;
  > 3. the full chain is written as a one-line JSON array `chain:[...]` to `instances/<name>/diagnostics/detail-<process>.ndjson` only, and only when an instance resolves.
- (.andromeda/obs-plan.md:537) The MCP witness must assert that `instance-unreachable` leaves no line, its `isError` code the only record.
  > MCP rejects are `isError` codes only; a logged reject's line is codes-only, and `instance-unreachable` — a frame tool's refused instance check (made before the liveness pre-check), the liveness pre-check's, or a frame tool's server-verification mismatch on Linux and Windows — is logged by no line, its outcome kept inside `mcp.tool_call`, the `isError` code its only record.
- (.andromeda/obs-plan.md:570) The acceptance's secret-scan leg must be read against this verification; a leg the chunk cannot prove at HEAD is no claim.
  > **Verification:** the tests secret-scan test (`Cookie` header, `CLAUDE*` values, the content canary, 0600) over every `diagnostics/` file, run in CI before artifact upload; the canary never appears in a home-level `diagnostics/*.ndjson`;
- (.andromeda/obs-plan.md:576) A field added to `channel-response` must be named in the §6 catalog and admitted by the schema first, or G4 fails.
  > a field is logged only if it is named in this plan's Section 6 catalog. Unknown fields are a build failure (§10), caught by gate G4 (§9) against `schemas/diag-line.v1.json`.
- (.andromeda/obs-plan.md:643) A typed error crossing a seam must still be logged once with a closed code when its surface body is reduced.
  > **Module-boundary error logging:** every typed error that crosses a crate seam is logged once at `WARN`/`ERROR` with a closed `detail` / `error_code`. The full chain goes only to the instance detail file.
- (.andromeda/obs-plan.md:684) No log line this chunk touches may fall back to free text; this is the Telemetry Strategy ban closest to the chunk's intent.
  > NEVER let the signal pyramid collapse into free text. Every line carries the closed `event` enum and `corr`.
- (.andromeda/obs-plan.md:715) The `-32602` cause must be told apart without a renamed field, and a new `event` value needs a Decisions Log entry.
  > NEVER rename tests fields (for example `corr` → `correlation_id`, `event` → `event_type`) or add `event` values without a Decisions Log entry.
- (.andromeda/obs-plan.md:724) The serde-source ban names the chunk's surfaces, so a witness must drive a chain with such a source to each of them.
  > NEVER print an anyhow chain that still contains a serde_json / serde_path_to_error source to stderr, `--json`, MCP `isError` or channel `error.data`.
- (.andromeda/obs-plan.md:734) The planted `CLAUDE*` value must appear in no file at any level, detail files included, a stricter rule than the one for other content.
  > NEVER log `CLAUDE*` values or a `Cookie` header, at any level, in any file, detail files included.
- (.andromeda/obs-plan.md:735) Any code-shaped `detail` the chunk defines must hold no path or pid.
  > NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.
- (.andromeda/obs-plan.md:759) The floor is proved by the secret-scan test, which the chunk's claim must rest on and must not replace.
  > NEVER skip scrubbing before any byte leaves the process. In v1 the only destination is local files, and the secret-scan test proves the floor.

## Bindings
- **Fixed `Display` strategy ↔ obs-plan §7 Error Capture & Reporting**: the signal row sets the strategy and §7's three scrubbing layers its mechanism; both must hold for every `Display` that can reach a surface, (.andromeda/obs-plan-detail/telemetry-strategy.md:12) and (.andromeda/obs-plan.md:509).
- **Vector 5 and Vector 6 triggers ↔ security plan Vectors 5 and 6**: the trigger rows route to obs sections and the required signals come from the security plan, so a witness satisfies both sides, (.andromeda/obs-plan-detail/telemetry-strategy.md:31), (.andromeda/obs-plan-detail/telemetry-strategy.md:32), (.andromeda/obs-plan.md:205) and (.andromeda/obs-plan.md:210).
- **Machine-checked verdict ↔ test plan and the chunk's proof**: the scope's enumeration must be a test or harness assertion, and the plan must already list it, since the directive bars testing past the plan, (.andromeda/obs-plan.md:286).
- **`channel-response` cause ↔ obs-plan §5, §6 and `schemas/diag-line.v1.json`**: telling the `-32602` causes apart is a line contract change that moves the metric's derivation, the field catalog and the schema together, (.andromeda/obs-plan.md:418) and (.andromeda/obs-plan.md:576).
- **Closed `--json` documents ↔ "Exit-cause code catalogue"**: this chunk enumerates the documents and the later entry owns the catalogue and the revive start-refusal documents, so `strict-modes-failed` is named here and not redefined, (.andromeda/obs-plan-detail/telemetry-strategy.md:43).
- **`Refusal` enum ↔ architecture [Error Handling]**: architecture names the one open divergence from the one-enum rule, and the obs mechanism line lists the thiserror types; the fold or ratification must agree with both, (.andromeda/obs-plan.md:496).
- **Secret-scan floor ↔ `verification-matrix.json#v1-47` and test plan**: the candidate claim's scan leg depends on the tests secret-scan test and its CI step, (.andromeda/obs-plan.md:570) and (.andromeda/obs-plan.md:759).
