# Classified — obs · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · no · standing S9 · a span on a new path is `#[instrument(skip_all, ...)]` with a static name and an allow-list, the form the task lists
telemetry-strategy#1.2 · yes · the new causes and refusals log as typed process-exit and refusal lines in the closed event enum, one file per process role, which the exit 21 and exit 1 causes need
telemetry-strategy#1.3 · yes · typed errors with fixed Display strings and distinct process-exit detail codes, the scope adds one distinct detail per exit-21 and exit-1 cause
telemetry-strategy#1.4 · yes · counts of detail values: the new process-exit detail values join the jq-derived counts, a derived-metric row the scope's Needs name
telemetry-strategy#1.5 · no · the chunk does not touch `events.ndjson`, the product audit stream, and its new lines carry codes only, so the no-payload-copy rule is a re-check every change owes
telemetry-strategy#2.1 · yes · the exemption list names the pre-log refusals the chunk states (revive start, verify, the frame verbs' refused instance and home tree, list scan, link and unlink), each the carve-out its no-line decision must meet
telemetry-strategy#3.1 · yes · IPC: the channel client's vanished-server read as `instance-unreachable` and the channel `error.data` pass through the channel field allow-list
telemetry-strategy#3.2 · no · paste text is logged only as `send-refused` `side` for exits 10–14, which are excluded; the chunk's send causes are exit 21 codes that log no paste text
telemetry-strategy#3.3 · yes · the hook's edge (exit 0, no line on a refused instance, `hook-decision` as its only cause record, no hook cause added) and the `server-verify-failed` WARN line on Linux and Windows, in the server check the chunk edits
telemetry-strategy#3.4 · yes · MCP: `isError` results and rejected-call lines carry codes only, the new codes pass through `viola-mcp` result.rs and tools.rs, and stdout holds MCP frames alone
telemetry-strategy#3.5 · yes · CLI/env fixed messages: the new codes' fixed Display strings and the fixed `internal-error` stderr message with no path or pid, under §7's fixed-message rule
telemetry-strategy#3.6 · yes · filesystem: the strict-modes refusals before a log opens write no line, and the `pinned-hash-mismatch` and `sideload_fallback` `strict-modes-failed` codes sit in the detail catalog the chunk extends
telemetry-strategy#3.7 · no · child-spawn subject values in Scenario 1 and `parse-rejected` are not changed; the `batch-script-child` and `checked_exe` refusals are start-time checks writing a closed detail, not spawn subjects
telemetry-strategy#3.8 · no · supply chain: the chunk adds no dependency, feature or cargo-deny change
telemetry-strategy#3.9 · no · `corr` and `conn` correlation joins are excluded by the task, and no correlation field changes
telemetry-strategy#3.10 · no · perf budgets and hyperfine gates are not reached, and the hook latency gates stay as they are
telemetry-strategy#3.11 · yes · chaos: the `server-verify-failed` `process-exit` carries `during:"connect"`, and the `verify_stopped` WATCH tally reads `process-exit` `exit_source` and `during`
telemetry-strategy#3.12 · no · log file location (0600/0700 against the home DACL), the concurrent-append check and the `endpoint_kind` and `pty_backend` fields are unchanged; the per-OS server-verify gap is carried by row 3.3
telemetry-strategy#3.13 · yes · error budget: the `internal-error` line through `obs::internal_error_exit_line` and `report_internal_error` is an error edge the logged-once rule covers, each typed error ending logged once
telemetry-strategy#3.14 · no · audit-log retention: no rotation or retention of `events.ndjson` or `diagnostics/` is changed
telemetry-strategy#3.15 · yes · §3 schema verbatim: the chunk edits the `process-exit` `detail` values in `schemas/diag-line.v1.json`, so the schema and the plan's §3 copy must agree
telemetry-strategy#3.16 · yes · Founder Direction 4 is the trigger: a distinct `detail` per exit-21 and exit-1 cause, and strict-modes refusals before any log writes no line and carry no `detail` code, with the `--json` document as their machine-readable form
telemetry-strategy#3.17 · no · installed-CLI measured counters (§5) and `parse-rejected` and `skipped` are not moved, since the chunk logs no parse or skip event
telemetry-strategy#3.18 · no · no screen scraping: the PTY pump and vt100 gate are not reached, and the chunk's tests assert exit code, `--json` and the stderr `hint:` line
