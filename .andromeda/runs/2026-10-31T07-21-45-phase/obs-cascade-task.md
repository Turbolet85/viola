# Task — obs · 2026-10-31-sanitised-error-surfaces

## The chunk
Makes every error a `viola` process gives a machine reader a code plus a fixed message, with no absolute path, upstream text, serde-sourced anyhow chain or stripped `CLAUDE*` value, on three surfaces: CLI `--json` error documents, `viola mcp` `isError` results, and channel JSON-RPC errors with their `error.data`.
Makes every product `Display` that can reach those surfaces a fixed message, and proves the property with one witness per surface (driving the failure with a path, an upstream text and a planted stripped `CLAUDE*` value in play) plus a checkable enumeration of the error bodies.
Folds in three carries: the `Refusal` enum beside `AgentError` in `viola-agent-claude` (folded or ratified), the `-32602` causes the `channel-response` line cannot tell apart, and the missing cap-driving tests (a `viola mcp` stdin line past `MAX_FRAME` before `initialize` completes, and five `BOUNDARIES` rows).
Candidate claim is `verification-matrix.json#v1-47`, its web clauses out: re-reads at HEAD whether its legs stand, namely content and drift reports reaching only owner-readable detail files and a secret scan after every end-to-end test proving the never-log floor.
Re-reads CI on this chunk's pushes to tally the Path 1 timeout watch on `path1_run_start_one_home_in_order`, whose kill line does not move.

## Reached files
- crates/viola-agent-claude/src/lib.rs · edits the `Refusal` enum and the `AgentError` enum
- crates/viola-channel/src/lib.rs · edits `ProtocolError::InvalidParams`
- crates/viola-channel/src/client.rs · edits `fault_of`
- tests/contract_bounded_readers.rs · edits the `BOUNDARIES` rows of the status-line command's stdout, the child pipe drain, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep`

## Needs
- CLI `--json` error documents (the `{"v":1,"error":…,"detail":…}` form): a closed code and a code-shaped or null `detail` on every verb, including the instance-check form with `instance-unreachable` and a null `detail`; the stdout role of `--json` results.
- The short-lived CLI's fault path: `viola::obs::report_internal_error`, the fixed internal-error stderr line, `process-exit` with `detail:"internal-error"`, and what a failure a verb cannot name turns into.
- The error-capture rule the chunk's code must act on: an anyhow chain with a `serde_json` or `serde_path_to_error` source becomes the fixed `internal error` before stderr, `--json`, MCP `isError` or channel `error.data`; the full chain goes only as the one-line `chain` array in the instance `detail-<process>.ndjson`, and only when an instance resolves.
- `viola mcp` tool results for `send`, `wait`, `last`, `answer`, `list`: `isError` carries codes only (`refusal`, `detail`, `instance-unreachable`, `wrapper-fault`, `invalid-params`), text `refused: <code>`, a handler fault exactly `internal error` with null data.
- Rejected MCP calls: the codes-only log line, no upstream text, absolute path or type name, including text that rmcp or a deserialiser would supply; `instance-unreachable` has no line and its `isError` code is its only record.
- Channel JSON-RPC errors a wrapper answers: fixed message, `error.data` a code or null, `-32603` exactly `internal error` with null data; a self-reported `from` stays unauthenticated.
- `-32602` causes (`ProtocolError::InvalidParams`, unsupported version, `release-from-driver`, a bad `link` or `unlink` name or pair): the client's `fault_of` labels them all unsupported-version and the `channel-response` line carries only `error_code`.
- The `channel-response` line as a contract read: its fields, level by `error_code`, its `Drop` guard emission, and any way of telling causes apart read against the §6 catalog, `schemas/diag-line.v1.json` and the `detail` code catalog.
- The derived metric `viola.compat.v_rejected` and its recorded over-count, which reads `channel-response` and depends on the causes being told apart.
- Fixed-message `Display` impls on the §7 list: thiserror on `ChannelError`, `StateError`, `AgentError`, `McpError`, `CoreError`, hand-written on `PtyError`, and `ServeError`: no field holding a path or payload reaches a message.
- `Refusal` (`BatchScriptChild`, `NotFound`, the `resolve_program` refusal) beside `AgentError`: the §7 enum list and the codes its callers log or print, for either outcome of the carry.
- The never-log floor for R8-stripped `CLAUDE*` values across every sink, detail files included; the strip recorded by count or names only (`env_stripped_count`, `env_stripped_known`, `env_kept`), which the planted value of the witnesses tests.
- Absolute paths and their sources: `snapshot.json` and config values scrubbed from external output, `endpoint` and `pinned_bin` never logged, paths appearing only in instance detail files, `detail` codes closed kebab-case with no path or pid.
- Upstream failures captured at the viola seam: `rmcp`, `portable_pty`, `sysinfo`, `interprocess` targets off in the sink, their text absent from all three surfaces.
- Detail-file legs of `v1-47`: drift reports and content-bearing records only in `detail-<process>.ndjson`, 0600 in 0700, validated by `schemas/diag-detail.v1.json`, the `chain` and `drift_report` fields.
- Secret-scan leg of `v1-47`: its patterns, its inputs (`target/e2e-home/**/diagnostics/*.ndjson`, `target/agent-run/*`, `target/nextest/ci/junit.xml`), its hit report, and how a witness's planted values stay out of those inputs unless a real leak put them there.
- `viola mcp` stdin bound: a line past `MAX_FRAME` refused whole and the session ended as at a closed stdin; before `initialize` completes it ends `serve` with `ServeError::Initialize`, exit 11; stdout carries MCP frames only on this path.
- The bounded status-line read: the command's stdout passes through unchanged and is never logged, `statusline_command` stays out of home-level files, `STATUSLINE_DEADLINE`, and what `viola hook statusline` prints on oversize.
- The child pipe drain shared by `claude --version` and `claude agents --json`: a parse failure gives `unknown` plus one `parse-rejected{parser:"claude-agents-json"}`, rows never logged verbatim.
- The `config.json` readers of the thresholds, the dialog hold and `claude_env_keep`: only the load-time parse is observable, config values stay out of external output, kept variables appear as names only.

## Standing conventions
- S1 · Every integration-test home lives under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover it · §9 CI Integration in the body
- S2 · Every log line goes through `obs_event!` with explicit typed fields, never a raw tracing macro · §11 Logs in the body
- S3 · A logged field or value is named in the §6 catalog and conforms to its schema under default-deny, field names snake_case and values kebab-case · §6 Log Coverage and §8 PII Scrubbing in the body, .andromeda/obs-plan-detail/log-coverage.md
- S4 · Every span is `#[instrument(skip_all, fields(...))]` or `info_span!` with an allow-list, without `err` or `ret` · §4 Span / Trace Coverage and §11 Spans / Traces in the body
- S5 · No telemetry on stdout on any role, no print or `dbg!` macros in `hook`, `run` or `mcp` code paths outside the output modules · §11 Logs in the body
- S6 · A test run leaves zero `event:"panic"` lines in home-level role files and uses no retry-once policy · §10 SLO Invariants and §11 SLO in the body
- S7 · Test verdicts set an exit code and key on hook events and log offsets, with no sleep-based log wait · §10 Standard+ invariants and §11 Universal in the body
- S8 · Inputs written to scanned or uploaded files are synthetic, because detail-file uploads are admissible only on that basis · §8 PII Scrubbing in the body, .andromeda/obs-plan-detail/pii-scrubbing-compliance.md

## Not needed
- The obs tier, its justification and the Minimal exporter carve-outs: the chunk changes no tier and adds no exporter.
- OTel SDK, OTLP, APM and external reporter choices: no pipeline or reporter is added; the file sink stands.
- The exit-cause code catalogue and the revive start-refusal documents: boundary of this chunk, owned by the chunk of that name.
- The hardening behind a refusal (socket, peer-credential and home-tree mechanisms): owned by "Unix endpoint and home hardening"; only the error body a refusal produces is in.
- Problem Details and any frontend or GUI surface: carried to 0.2.0.
- Windows-only live measurements: owned by their own chunk.
- Heartbeat ticks and liveness flip timing: no error body depends on the flip's timing.
- Torn-line healing, snapshot replay and `state-recovered` lines: crash-recovery telemetry the chunk does not change.
- PTY pump, vt100 feed and paste instrumentation, and the sideload companion fallback: none produces a body on the three surfaces.
- The `wheel` product event and the `/compact` confirmation window: outside the error bodies.
- `hook-invoked` `corr` timing and W3C trace context: the chunk changes no correlation scheme.
- `events.ndjson` and `diagnostics/` retention and rotation: unchanged.
- Artifact retention days and pinned Action versions: the scope adds no upload.
