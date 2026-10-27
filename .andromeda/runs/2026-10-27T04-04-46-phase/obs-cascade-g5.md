# obs · cascade read · group 5 of 10

## Carried
- (.andromeda/obs-plan-detail/telemetry-strategy.md:11) Every refusal or fault the chunk re-asserts or adds at a bounded boundary (a paste refusal, a rejected parse, a recovered snapshot) must surface as a line of the closed `event` enum in its process role's file, never as a free string.
  > | Structured Logs | Every boundary call, state transition, refusal, fault/heal and process lifecycle, as the closed `event` enum (Section 3) | **The primary signal.**
  (.andromeda/obs-plan.md:294)
  > **Event naming:** the `event` field is a closed kebab-case enum (the tests list plus the Section 12 extensions).
- (.andromeda/obs-plan-detail/telemetry-strategy.md:12) The re-verified screen-model panic degrade must rest on the `catch_unwind` around the vt100 feed, with no external reporter added.
  > No external reporter: security says "no outbound network calls". A hand-written `std::panic::set_hook`, plus `std::panic::catch_unwind` at the main-thread catch site of every role, around `hook` dispatch, in the `run` PTY pump and handle-wait threads, and around the vt100 feed (Section 7)
- (.andromeda/obs-plan-detail/telemetry-strategy.md:13) Any count the chunk's closed-type or over-size readers keep (skipped records, rejected parses) must be a field on a log line derived by `jq`, never an in-process instrument or exporter.
  > | Metrics | `duration_ms` samples, counts of `event`/`detail` values, `skipped` counts | No in-process instruments and no exporter. Metrics are derived by `jq` over the ndjson
  (.andromeda/obs-plan.md:280)
  > metrics have no in-process instruments and no exporter and are derived by `jq` over the ndjson
- (.andromeda/obs-plan-detail/telemetry-strategy.md:20) A vt100 panic the witness forces on purpose must still produce exactly one single-line panic record, its payload and backtrace only in the detail file, and the test must sit inside a bounded exemption.
  > Every panic after init step 4 produces exactly one single-line structured record in its home-level role file: location, thread, role and instance.
  (.andromeda/obs-plan-detail/telemetry-strategy.md:20)
  > The payload (one escaped JSON string) and the backtrace (a one-line JSON array of frames) are written only to the instance-scoped detail file (Section 7).
  (.andromeda/obs-plan.md:288)
  > Every panic after init step 4 produces exactly one single-line structured record in its home-level role file: location, thread, role and instance, except the bounded exemptions listed in Section 10; the payload and the backtrace are written only to the instance-scoped detail file (Section 7).
- (.andromeda/obs-plan-detail/telemetry-strategy.md:40) The forced-panic witness is the chunk's own obligation, so it must keep the zero-unlogged-panics budget.
  > | error-budget-SLO | §7, §10 zero-unlogged-panics |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:28) The frame cap and closed method / params types the chunk holds on both channel sides fall under the IPC vector, whose lines carry no `params` or result body.
  > | logging-sensitive Vector 1 (IPC) | §4 surface table (channel), §6 channel field allow-list, §8 table |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:29) The paste rule the chunk re-verifies at the CLI client, the MCP client and the wrapper must log its refusal as `send-refused` with its deciding `side`.
  > | logging-sensitive Vector 2 (paste text) | §4 Scenario 2 (`send-refused` `side`), §8 |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:30) The `hook` stdin cap must fail open through the hook sink and its `hook-decision` `detail` codes, and a refused or absent instance must leave no trace at all.
  > | logging-sensitive Vector 4 (hook stdin) | §3 Logging stack (hook sink), §4 Scenario 4, §6 `hook-decision` `detail` codes.
  (.andromeda/obs-plan-detail/telemetry-strategy.md:30)
  > The event and dialog paths' refused or absent instance (a `VIOLA_DIR` naming no existing instance directory included) leaves none either: exit 0 at once, no `config.json` read, no role file, no stdin read, no line, no frame, no SessionEnd append.
  (.andromeda/obs-plan.md:310)
  > nothing when no command is recorded, on a refused instance, on oversize or malformed stdin, or under the hidden `--capture` arm, which prints nothing
- (.andromeda/obs-plan-detail/telemetry-strategy.md:31) The `mcp` stdin cap and closed tool-argument types the chunk holds fall under the MCP vector's codes-only logging.
  > | logging-sensitive Vector 5 (MCP) | §4 surface table (mcp), §6 |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:32) `VIOLA_NAME` and the CLI positionals the chunk validates before a path join are CLI / env inputs, and the R8 strip's names-only record must hold over the spawns it bounds.
  > | logging-sensitive Vector 6 (CLI/env) | §4 Scenario 1 (`env_stripped_*`), §7 fixed messages, §8 |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:33) The bounded reads of `snapshot.json`, `events.ndjson`, `budget.json`, `ledger/stamps.json` and the other home files must keep their `state-recovered` and `skipped` signals.
  > | logging-sensitive Vector 7 (filesystem) | §3 Log file location (0600/0700), §6 `state-recovered` / `skipped`,
  (.andromeda/obs-plan-detail/telemetry-strategy.md:33)
  > §10 exemptions (a strict-modes check made before a read or open — the six frame verbs, `viola hook`'s event and dialog paths, `viola mcp`'s frame tools, `verify`, `run` / `revive`'s start, the `diagnostics/` opens — writes no line) |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:34) The `claude agents --json` and `claude --version` output caps are child-spawn boundaries, so an oversize or malformed read must surface as `parse-rejected` under its closed `subject`.
  > | logging-sensitive Vector 8 (child spawn) | §4 Scenario 1 (`subject` values), §6 `parse-rejected` |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:38) The re-verified vt100 feed-panic degrade (a chaos witness) and the replayed-snapshot read the chunk bounds must keep their `state-recovered` and chaos signals.
  > | chaos-instrumentation | §6 `state-recovered` / `process-exit` `exit_source` + `during`, §10 |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:44) The closed-type readers of measured-CLI output and of viola's own formats must report a rejected or skipped input through the `parse-rejected` / `skipped` signals and their counters.
  > | creator-explicit (installed-CLI measured) | §5 counters, §6 `parse-rejected` / `skipped` |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:45) The cap on the child's PTY output into the vt100 feed must log only the gate outcome, never screen bytes or a per-chunk line.
  > | creator-explicit (no screen scraping) | §4 Scenario 2 (vt100 gate outcome only), §11 |

## Bindings
- **telemetry-strategy (vt100 `catch_unwind`) ↔ error-capture-reporting**: the feed's catch site of the strategy row must yield exactly one `parse-rejected{parser:"vt100-feed"}` per poisoning episode and `run` continuing, `(.andromeda/obs-plan-detail/telemetry-strategy.md:12)` with `(.andromeda/obs-plan.md:535)`.
- **telemetry-strategy (panic invariant) ↔ slo-invariants-telemetry-budgets (§10 exemptions)**: the forced-panic witness keeps the one-record rule and runs only inside a listed exemption, `(.andromeda/obs-plan-detail/telemetry-strategy.md:20)` with `(.andromeda/obs-plan.md:288)`.
- **telemetry-strategy (Vector 8 / installed-CLI measured) ↔ log-coverage**: a failed `claude agents --json` read is one `parse-rejected{parser:"claude-agents-json"}` and `"unknown"`, `(.andromeda/obs-plan-detail/telemetry-strategy.md:34)` with `(.andromeda/obs-plan.md:482)`.
- **telemetry-strategy (Metrics row) ↔ metric-coverage**: counts stay `jq`-derived, and a new `-32602` from a channel name check adds to the `viola.compat.v_rejected` over-count, `(.andromeda/obs-plan-detail/telemetry-strategy.md:13)` with `(.andromeda/obs-plan.md:418)`.
- **telemetry-strategy (Vector 7) ↔ span-trace-coverage (E5)**: the bounded snapshot read keeps one `state-recovered` line per `read_snapshot_or_replay` call, `(.andromeda/obs-plan-detail/telemetry-strategy.md:33)` with `(.andromeda/obs-plan.md:394)`.
