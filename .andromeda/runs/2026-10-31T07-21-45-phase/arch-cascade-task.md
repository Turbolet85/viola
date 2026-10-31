# Task — arch · 2026-10-31-sanitised-error-surfaces

## The chunk
Makes every error a `viola` process gives a machine reader a code and a fixed message only, on three surfaces: CLI `--json` error documents, `isError` results of the `viola mcp` tools, and the JSON-RPC errors a wrapper answers on the channel.
Makes every product `Display` impl that can reach one of those surfaces a fixed message with no path and no payload, and proves it: a witness per surface driving the failure with a path, an upstream text and a planted stripped `CLAUDE*` value in play, plus an enumeration of the surfaces' error bodies a reader can check.
Folds in three carried items: the `Refusal` enum beside `AgentError` in `viola-agent-claude` (fold it or have it ratified), the `-32602` causes the `channel-response` line and `fault_of` cannot tell apart, and the tests owed for the bounded-input clauses (a `viola mcp` stdin line past `MAX_FRAME` before `initialize` completes; a cap-driving case for five `BOUNDARIES` rows).
Re-reads, for the candidate claim `v1-47`, whether content and drift reports reach only owner-readable detail files and whether a secret scan after every end-to-end test proves the never-log floor; a leg it cannot prove is no claim.

## Reached files
- crates/viola-agent-claude/src/lib.rs · edits the `Refusal` enum, the `AgentError` enum
- crates/viola-channel/src/lib.rs · edits the `-32602` causes of `ProtocolError`, `InvalidParams` among them
- crates/viola-channel/src/client.rs · edits `fault_of`
- tests/contract_bounded_readers.rs · edits the five `BOUNDARIES` rows named by the scope: status-line command stdout, child pipe drain, the three `config.json` readers
- .andromeda/obs-plan-detail/metric-coverage.md · re-reads the `viola.compat.v_rejected` row, the contract the `channel-response` line's change is read against
- viola-0.1.0/verification-matrix.json · re-reads the `v1-47` acceptance and its notes

## Needs
- CLI `--json` error document: its `{v, error, detail}` form, the listed codes and exit codes, `instance-unreachable`, the revive codes, and what `client::refused_document` writes.
- `wrapper-fault`'s `detail` carries a code, a message and a data object as the plan stands, which the chunk's code-shaped-or-null `detail` must be read against.
- MCP result rules: what is an `isError: true` result, its `refused: <code>` text and `structuredContent` shape, `invalid-params` with no frame written, and which faults stay JSON-RPC errors of the MCP connection (`-32602` unknown tool, `-32603` handler fault).
- The MCP tool surface, `send · wait · last · answer · list`: `pause`, `release`, `link`, `unlink` are CLI-only, so their faults reach only the CLI and channel surfaces.
- Channel protocol fault schema: the codes `-32700` to `-32603`, the fixed message each carries, and `error.data` typed object or null.
- Every channel `-32602` and its data: unsupported `v` with `data.supported` and `data.wrapper` (a version string, read against code-or-null), `release-from-driver`, `InvalidParams` with `data: null`, and the per-method param checks.
- Every channel `-32603` the methods answer: an unrecordable link on `answer`, a `pause` or `release` whose record did not land, a `budget:true` release whose records did not land.
- The over-`MAX_FRAME` channel line answered `-32600` with the connection closed, and the id-less frame logged `parse-rejected` with detail `malformed`, as error bodies the enumeration must name.
- `RefusalReason` and its closed `detail` sets per reason, including an unknown detail being opaque text to a reader, as a carve-out to read against codes only.
- CLI exit codes: each sanitised error keeps its typed exit code (1, 20, 21 and the refusal codes), and `ServeError::Initialize` ends `serve` with exit 11, which the new test asserts.
- [Error Handling] and the Rust error type convention: one enum per crate, `PtyError`'s hand-written fixed messages, the open divergences `Refusal`, `ServeError`, `StampsError`, and `anyhow` only in the root bin with its chains for the owner's post-mortem only, never on a surface.
- `Refusal`'s variants as the plan knows them: `batch-script-child` and the `claude` npm shim resolution under [CI/CD], and `viola-agent-claude` staying pure with no I/O and no `viola-state` dependency under [Agent Coverage], which any fold into `AgentError` must keep.
- R8 strip under the environment variables: which `CLAUDE*` names are removed, `IDENTITY_FLOOR`, `claude_env_keep`, and names only with no value ever read; the planted value in the proof must rely on this.
- Host paths the plan keeps off error bodies: the snapshot's `cwd` and `statusline_command`, the pinned path, the viola home and `VIOLA_DIR`, as the values a witness can plant and check.
- Untrusted upstream text (R1) as a carve-out: dialog and `Stop` text travels in `ok` payloads and event `data` as content, which the error-body proof must not count as a leak and must keep out of errors.
- Diagnostic output channels: codes-only role files, content-bearing detail only in the instance `detail-<role>.ndjson`, and `mcp`'s JSON on stderr only when its log cannot open; the `v1-47` detail-file leg reads against these.
- [Validation]'s drift reports through `serde_path_to_error` and tolerant parsing, the source of a serde chain that must stay off the surfaces.
- The secret scan: `viola-harness secret-scan`, what it walks under `target/e2e-home`, its `chunk.diff` exception, its `hits.json`, and the `viola-chaos-*` home outside it, for the `v1-47` scan leg.
- `viola mcp` stdin bound: the `MAX_FRAME` line cap with its newline counted, the whole line refused, one `parse-rejected{parser:"mcp-stdin"}` line holding no byte, and the reader viola hands rmcp.
- Bounded readers: the rule and the contract binary's enumeration and census, which the five owed cap-driving cases extend.
- Outcomes of the cap-driven reads: `claude agents --json` over the cap leaves every `status` `unknown` and is never an error, and `claude --version` and the status-line command's stdout through their bounded reads.
- `config.json` over its `MAX_FRAME` cap: fallback to the defaults (`five_hour` 90, `seven_day` 85, `dialog_hold_s` 60, the keep list) with a `parse-rejected{parser:"config-json"}` line, the outcome the three reader cases assert.
- Protocol versioning and mixed-version tolerance: adding a field needs no bump, changing a field's meaning bumps `v`, and older readers skip unknowns; this rules over any change to an error's `data` or to the `channel-response` line.
- The `channel-response` line: codes only, its format owned by obs, and telling the `-32602` causes apart stays inside the codes-only rule.
- `Conventions`' error handling schema and `Standard Contracts`' error shapes are plan text a changed wire shape must keep true, as is the [Error Handling] divergence list if `Refusal` is folded or ratified.

## Standing conventions
- S1 · A new test runs in its own viola home under `target/e2e-home`, passed by `--home`, sharing no endpoint, log or `budget.json` · established-decisions.md, [CI/CD] in the body; occupied-resources.md res-repository
- S2 · A new test or witness drives the fake agent, named after `--`; the real `claude` runs only in local live tests · Established Decisions, [CI/CD]
- S3 · A new test endpoint stays outside the `viola-<h12>` namespace, named per test and never bound by `viola` · occupied-resources.md res-ipc-endpoints
- S4 · A new test-only file or directory is a recorded row of the filesystem or repository list, and `viola` never reads it · occupied-resources.md res-filesystem, res-repository
- S5 · A new fixture holds synthetic text only and passes its schema and hygiene walk · occupied-resources.md res-repository
- S6 · A new harness variable carries the `AGENT_RUN_` prefix and the binary never reads it; a test seam disables no control and widens no redaction · occupied-resources.md res-env-vars
- S7 · A new or changed log line stays codes only and conforms to the default-deny `schemas/diag-line.v1.json`, content going only to the detail schema · Cross-cutting Patterns, Diagnostic output channels; occupied-resources.md res-repository
- S8 · New code on `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O; only `viola-mcp` builds a runtime · Cross-cutting Patterns, Tokio containment
- S9 · Every OS-specific branch of a new test or code path compiles and is tested on its CI runner; nothing shells out, and children spawn directly · Cross-cutting Patterns, Cross-platform discipline
- S10 · A new wire code is a kebab-case string and a new JSON field is snake_case; a new error enum variant goes in the owning crate's enum · Conventions, Naming patterns and Rust error types
- S11 · A new product reader of external bytes is bounded in its own statement and joins the contract census · cross-cutting-patterns.md crosscut-bounded-readers

## Not needed
- Any frontend contract, [GUI Control Scope] and [Real-time Strategy]: the frontend is deferred to 0.2.0.
- Problem Details and the GUI token, cookie, launch URL and a11y clauses of the cap: carried to 0.2.0 by the founder's ruling.
- Hardening of endpoints: socket directory and file modes, peer pid and start-time checks, the SQOS open, endpoint hashing; another entry owns them.
- Windows-only live readings: the OPEN ConPTY and DA1 items under [PTY], and the sideloaded companions.
- The readiness gate, the screen signatures and the delivery confirmation window: the chunk changes no send outcome.
- Wheel moves and byte-source claims under [Human Takeover / Wheel], and the budget gate's pause and lift behaviour: no error surface changes them.
- Hook transport tiers and hook decision bodies: the hook prints no error document and is none of the three surfaces.
- The snapshot writer helpers, pinned copy placement and re-hash, and plugin file layout: the chunk changes no write path.
- Capability-ledger rows, `viola verify` probe design and the stamps merge: no new dependence on `claude` behaviour.
- The heartbeat file and the liveness verdict from pid and start time: the chunk changes no liveness reading.
- The vendored ConPTY binaries and their script, and the perf and coverage target directories: untouched.
- [Hosting], [Mobile], [AI/ML], [Primary Keys] and Docker resources: they do not apply.
