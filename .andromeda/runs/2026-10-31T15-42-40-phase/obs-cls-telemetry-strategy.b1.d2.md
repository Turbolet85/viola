# Classified — obs · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · no · standing S9 · A span added on a new path is `#[instrument(skip_all, ...)]` with an allow-list, a static name, no `err` or `ret`
telemetry-strategy#1.2 · yes · the refusal and process-lifecycle structured logs in the closed event enum, which the chunk's new process-exit lines for exit 21 and exit 1 are written into
telemetry-strategy#1.3 · yes · typed thiserror errors with fixed Display and distinct `detail` codes on `process-exit`, which the new codes, fixed messages and error-layer crossings extend
telemetry-strategy#1.4 · yes · counts of `detail` values are derived metrics, and the chunk adds and moves `detail` values (Needs: derived metrics keyed on a moved word)
telemetry-strategy#1.5 · no · the rule covers the `events.ndjson` product audit stream and payload copying, which the chunk does not read or change; its process-exit lines carry closed codes only
telemetry-strategy#2.1 · yes · the one-record panic rule and its exemptions list of pre-log refusals (revive start and `--list`, verify, run start home tree, frame verbs, link/unlink), which the chunk's no-line carve-out and revive's check-before-log re-read rest on
telemetry-strategy#3.1 · yes · IPC logging vector under the §6 channel allow-list, which the vanished-server `instance-unreachable` read, the endpoint-lost line in `viola-channel/src/client.rs` and channel `error.data` write to
telemetry-strategy#3.2 · no · paste-text vector covers paste text and the `send-refused` side (exits 10–14, outside the catalogue); the chunk's exit-21 not-running causes on `send` log no paste text
telemetry-strategy#3.3 · yes · hook stdin vector: a refused or absent instance writes no line, and `server-verify-failed` logs at WARN on Linux and Windows with the macOS gap, both named in Needs for the hook edge and the client's server check
telemetry-strategy#3.4 · yes · MCP vector: `isError` results and rejected-call lines carry codes only and `instance-unreachable` is logged by no line, which `viola mcp`, `result.rs` and `tools.rs` carry
telemetry-strategy#3.5 · yes · fixed messages (§7) for the new fixed `Display` strings, the replaced serde-sourced chain and the `internal-error` stderr with no hint
telemetry-strategy#3.6 · yes · filesystem vector: strict-modes refusals before a role file opens write no line (frame verbs, `verify`, `run` and `revive` start), and the `pinned-hash-mismatch` filesystem detail is one of the run refusals the chunk edits
telemetry-strategy#3.7 · yes · child-spawn subject values and `parse-rejected`: run's `checked_exe` / `refuse_tree` script-child and pinned-exe mismatch refusals and their refusal telemetry sit on the spawn path
telemetry-strategy#3.8 · no · supply-chain vector covers the logging feature set and cargo-deny gates; the chunk's Needs name no dependency or logging feature change
telemetry-strategy#3.9 · no · `corr` and `conn` join rule; Not needed states no correlation field or join changes
telemetry-strategy#3.10 · no · perf budget table (§5, §10); Not needed states hyperfine budgets and hook latency gates stay as they are
telemetry-strategy#3.11 · yes · process-exit `exit_source` and `during` shape, which the `server-verify-failed` (`during:"connect"`) and endpoint-lost lines carry, and the WATCH tally on CI reads
telemetry-strategy#3.12 · no · OS rules cover log-file 0600/0700 vs home DACL, the concurrent-append check and per-OS artifacts; the chunk's Linux/Windows versus macOS split is the peer-pid check and changes no log file mode or CI artifact
telemetry-strategy#3.13 · yes · zero-unlogged-panics SLO: the `internal-error` `process-exit` line through `obs::internal_error_exit_line` for Run and Cli must be logged once
telemetry-strategy#3.14 · no · no-rotation rule for the log file; Not needed states nothing here rotates a file
telemetry-strategy#3.15 · yes · the verbatim §3 schema copy must follow the chunk's `process-exit` `detail` value edits to `schemas/diag-line.v1.json`
telemetry-strategy#3.16 · yes · Founder Direction 4 is the creator trigger behind the chunk; its strict-modes no-line carve-out and `detail` catalogue (§6, D-01) are what the chunk gives effect to
telemetry-strategy#3.17 · no · installed-CLI counters (§5) and `parse-rejected` / `skipped` (§6) are not moved, and the chunk adds no measurement
telemetry-strategy#3.18 · no · the rule limits the vt100 gate outcome in Scenario 2; the chunk's table asserts a stderr `hint:` line and `--json`, not a PTY screen
