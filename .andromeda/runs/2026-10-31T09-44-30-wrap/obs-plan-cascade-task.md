# Task — obs-plan · 2026-10-31-sanitised-error-surfaces

## The session
Error surfaces now carry codes and fixed messages only: CLI `--json` documents, MCP errors and channel error data hold no path and no upstream text, and a peer's reply is screened by `viola_channel::sanitised_fault` and `viola_core::is_code_word`.
The `channel-response` line gains a closed `detail` of three words in `schemas/diag-line.v1.json`, which separates the three `-32602` causes; the wire form, `v` 1, the seven `ProtocolError` bodies and the line's WARN level do not change.
Retired: `viola_agent_claude::Refusal`, folded into `AgentError`; both private `fault_detail` copies; `CoreError`, which measurement showed never existed in the code.
Measured and corrected: `viola mcp` exits 1, not 11, on a stdin line past `MAX_FRAME` before `initialize`, and the `mcp` role writes no self `process-exit` line; the test-plan's after-every-test error-body check was not built and is replaced.
Added: `tests/contract_error_bodies.rs` (the census), `tests/channel_error_bodies.rs` and `tests/cli_error_bodies.rs` (witnesses), and the check `error_body_hit` / `assert_error_body_clean` in `tests/support/events.rs`.
CI over `fa7be1f3` read green, 15 of 15 checks, with `secret-scan` success and no hits on all three `test` jobs; `v1-47` stands implemented in the ledger, its flip left to the coverage gate.

## Reached files
- tests/contract_error_bodies.rs
- tests/channel_error_bodies.rs
- tests/cli_error_bodies.rs
- crates/viola-core/src/lib.rs
- crates/viola-channel/src/lib.rs
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/server.rs
- schemas/diag-line.v1.json
- src/cmd/client.rs
- crates/viola-mcp/src/result.rs
- crates/viola-mcp/src/lib.rs
- crates/viola-mcp/tests/handlers.rs
- crates/viola-agent-claude/src/lib.rs
- src/run/mod.rs
- src/cmd/run.rs
- src/cmd/verify.rs
- tests/support/events.rs
- Cargo.toml
- .config/nextest.toml
- crates/viola-e2e/tests/mcp_tools.rs
- src/cmd/hook/statusline.rs
- src/run/version_gate.rs
- src/run/budget.rs
- src/run/dialog.rs
- src/run/env.rs
- tests/contract_bounded_readers.rs
- viola-0.1.0/verification-matrix.json

## Touches
- §5 `viola.compat.v_rejected` at `.andromeda/obs-plan.md:418` and `.andromeda/obs-plan-detail/metric-coverage.md:28`: the over-count caveat and the until-the-line-tells-the-causes-apart condition are stale, because the line now tells them apart.
- §5 metric label discipline: `detail` as a closed grouping key now covers `channel-response`'s three words.
- §6 `.andromeda/obs-plan-detail/log-coverage.md:13` and the `detail` code catalog: `channel-response`'s `detail` moves from an open string to three closed words; the additive field catalog must agree.
- §6 boundary-call wrappers at `.andromeda/obs-plan.md:479`: the IPC invocation's field list names method, `corr`, `conn`, result class, error code and `sender`, and now omits `detail`.
- §6 log levels table: `channel-response` with `-32602` stays WARN and `-32603` stays ERROR; a re-read holds, with the `detail` field added beside them.
- §7 enum list at `.andromeda/obs-plan.md:496`: `CoreError` is named and does not exist, and `StampsError`, `PinError`, `StampsReadError`, `Refused` and `ServeError` are unnamed; pattern `enum-rule`.
- §7 stdin clause at `.andromeda/obs-plan-detail/error-capture-reporting.md:39`: `ServeError::Initialize` as `viola mcp`'s exit 11 is disproved; the process exits 1, and 11 is the unit child's own code only (`serve_child_entry`).
- §7 same line, `.andromeda/obs-plan-detail/error-capture-reporting.md:39`: its `process-exit` 0 once `initialize` has completed has no writer on the `mcp` role; pattern `mcp-exit-line`, changed there.
- §7 per-role behaviour, `mcp` main-thread branch at `.andromeda/obs-plan.md:527`: it says the process writes `process-exit` with detail `internal-error`; the report measures `mcp`'s catch site writing none, and its sweep did not list this line.
- §7 `cli` and `run` per-role lines at `.andromeda/obs-plan.md:523` and the mirror at `.andromeda/obs-plan-detail/obs-scope-summary.md:270`: the report re-read both for `mcp-exit-line` and found them unmoved.
- §7 scrubbing layers: layer 1's fixed `Display` strings and layer 2's fixed message before `--json`, MCP `isError` and channel `error.data` now have a census and witnesses, and a screen of peer replies sits beside them.
- §7 and §10 module-boundary error rule: its closed `detail` / `error_code` now reads a closed `detail` on `channel-response` too.
- §7 error classes, MCP rejects as codes only: unchanged in rule; `of_reply` now screens a peer's refusal and detail, and an unknown tool's `data` stays the wire's null.
- §8 default-deny posture and schema description: `schemas/diag-line.v1.json:42` now closes one field's value, not only the field allow-list.
- §8 integration points, error layer and verification: the new error-body check sits beside the secret scan; it does not change the scan, and any text treating the scan as the only error-body proof is short.
- §10 build failure conditions at `.andromeda/obs-plan.md:668`: a schema-conformance failure now also covers a `channel-response` `detail` outside the three words, through G4 reading the closed schema.
- §1 scope entities: `viola-core` gains `is_code_word`, `viola-channel` gains `sanitised_fault` and `ProtocolError::cause`, `viola-agent-claude` loses `Refusal`, `viola-mcp` loses its `fault_detail`; instrumentability is unmoved and no manifest edge is added.
- §1 surfaces: cli's fixed lines with no paths or upstream text, `ipc-internal` channel's `-32603` with `data:null`, and `viola mcp`'s codes-only `isError` hold, now witnessed; the `mcp` surface's oversize-line clause gains exit 1 only through §7's correction.
- §1 gap (a) and the Founder Direction 4 trigger: `release-from-driver` also becomes a `channel-response` `detail` word; its own logged event and the exit 21 and exit 1 detail codes are not replaced.
- §1 log format mirror and log-fields groups: if they enumerate per-event fields of `channel-response`, the closed `detail` reaches them; the required fields do not move.
- §4 scenarios `send`, `wait` / `last`, `answer`, the wheel and `release`: each cites the `channel-request` / `channel-response` pair; `-32602` on these paths now carries a `detail` word, and no name or param reaches the line (`link_verbs_log_the_channel_pair_and_no_names` green).
- §4 `.andromeda/obs-plan-detail/span-trace-coverage.md:44`: `resolve_program` is truly named and unmoved by pattern `refusal-enum`; it now returns `Result<PathBuf, AgentError>`, so a stated error type there would be stale.
- §4 E3 `link` / `unlink`: a bad name or pair answers `-32602` with `detail` `invalid-params`; the no-line-of-their-own rule is unmoved.
- Any obs-plan mention of `Refusal`, `Refusal::NotFound` or `fault_detail`: retired names, replaced by `AgentError::ProgramNotFound`, `AgentError::BatchScriptChild` and `sanitised_fault`; the two refusal messages are unchanged.
- Word boundaries for classifiers: the exit correction keys on `ServeError::Initialize` and `initialize`, so `budget-paused`'s exit 11 stays true; the `Refusal` retirement does not reach `RefusalReason`.
- §9 the home rule and the chaos home exception at `.andromeda/obs-plan-detail/ci-integration.md:59`: re-read as `v1-47`'s measured basis and unmoved; it stays a citation target.
- §9 and §10 test inventory: `ci` profile kill-line filter goes from 22 to 24 `binary(…)` terms, `contract_bounded_readers` five `BOUNDARIES` witnesses are renamed, and the `test` jobs passed 3347, 3301 and 3300; any standing count or witness name must agree.
- §10 perf rows: `run --perf` read 2.0 / 2.0 / 2.0 / 2.0 / 2.2 ms mean against 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms before, with no cause claimed; any measured value cited must not read as a moved budget.

## New facts
- F1 · log-coverage · `channel-response`'s `detail` is the closed set `unsupported-version`, `invalid-params`, `release-from-driver`, written for the three `-32602` faults on both the server's and the client's line, and absent on every other `channel-response` line.
- F2 · span-trace-coverage · `ResponseLine`'s `Drop` writes `detail` from `ProtocolError::cause`, which is `None` for the other four faults, and the client's `fault_of` now reads the fault's `data` (`supported`, `reason` or neither) to choose the word.
- F3 · metric-coverage · `viola.compat.v_rejected` derives from `channel-response` with `error_code` -32602 and `detail` `unsupported-version`, and so counts version rejections alone.
- F4 · error-capture-reporting · `viola_channel::sanitised_fault` is the one reader of a peer's fault for `reply_of` and `of_reply`: no integer `code` reads nothing, the five known codes read their fixed text with closed `data`, and any other code reads message `wrapper fault` with `data` null.
- F5 · error-capture-reporting · `reply_of` and `of_reply` screen a peer's reply by `is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first): a non-code `refusal` reads `unknown`, a non-code string `detail` reads null, and a code word outside the closed list stays visible with exit 14.
- F6 · error-capture-reporting · A `viola mcp` stdin line past `MAX_FRAME` before `initialize` gives process exit 1, both streams empty and exactly one `parse-rejected` of source `mcp-stdin`, cause `oversize`, count 1.
- F7 · error-capture-reporting · The `mcp` role writes no self `process-exit` line: `internal_error_exit_line` covers `Run` and `Cli` only, so `mcp`'s catch site writes none.
- F8 · error-capture-reporting · The product's `thiserror::Error` derives read 9: `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError`; `AgentError` gains `BatchScriptChild` and `ProgramNotFound`.
- F9 · error-capture-reporting · The open enum divergences are `ServeError`, `StampsError`, `PinError`, `StampsReadError` and `Refused`, owned by a CARRY on Version done-check, on the delegate overseer's PROVISIONAL word, owed the founder's own.
- F10 · error-capture-reporting · The exit-2 paths of `viola answer` and `viola send` write the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line to stderr whatever `--json` says, with no document, and are named in `NOT_ERROR_BODIES`.
- F11 · error-capture-reporting · No `-32603` is driven in a scanned home: no failpoint or counted panic was added, and the unit layer holds that fault (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`).
- F12 · pii-scrubbing-compliance · `tests/contract_error_bodies.rs` is a census of every error body: each names a writer and a witness that stand, every product writer must be named, and `thiserror_messages_interpolate_nothing` holds thiserror messages free of interpolation; it carries `andromeda:walks-tree`.
- F13 · pii-scrubbing-compliance · `tests/support/events.rs` holds `error_body_hit` and `assert_error_body_clean`, classes `absolute-path`, `caused-by`, `serde-path` and `planted`, reporting place and class, never bytes; it does not run after every test, and the witnesses, enumeration and census replace that plan.
- F14 · pii-scrubbing-compliance · The live witnesses of clean error bodies are `channel_error_bodies_hold_no_path_or_upstream_text`, `cli_error_documents_hold_no_path_or_upstream_text` and `mcp_tools_error_results_hold_no_path_or_upstream_text`.
- F15 · ci-integration · `v1-47`'s secret-scan reading is after each job's end-to-end run, not after every test: `AGENT_RUN_KEEP_HOMES` is `1` on the job, the scan is `if: always()`, and the `viola-chaos-*` home stays out by the founder's ruling of 2026-10-04.

## Untouched
- §9 gate commands G1 to G4, step order, upload gates, artifact retention, `nightly.yml` and `windows-mutants.yml`: no CI step, harness verb or status shape moved, and their rules live in `.github/workflows/` and `scripts/`.
- `schemas/diag-detail.v1.json` and the detail-file scope in §8: this schema was not edited, and detail files gain no field.
- §7 panic hook mechanics and the `hook`, `cli` and `run` per-role lines: `src/obs.rs` and `src/main.rs` were not reached; only the `mcp` branch is in question.
- Logger stack, sink layout, file modes 0600 / 0700, `diagnostics/` paths and retention: `src/obs.rs` and `crates/viola-core/src/obs.rs` were not reached, and no config key or env var moved.
- `viola-state` behaviour, meaning heartbeat, torn-line healing, snapshot replay and `state-recovered`: `crates/viola-state` was not reached; `StateError` only appears in the derive count.
- The `viola-pty` seam and `PtyError`'s hand-written `Display`: `crates/viola-pty` was not reached.
- cargo-deny bans, veil, the rejected sentry and tracing-panic, and the OTel carve-outs: no dependency was added or bumped, and `Cargo.lock` is unchanged.
- §10 budgets and their gate statistic: hyperfine `max` through `gate --require perf` holds, and no bound moved.
- The verify margin red (`verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1`): the bound lives in `tests/support/fake.rs`, no bound moved, and its owner is P5; it is no obs-plan matter.
- Tier, the signal pyramid, the agent-readable invariants and the span, metric and field naming conventions: no span, event value, role or exporter was added.
- `src/cmd/revive.rs`'s private `Refusal`: another type, not retired, and revive's refusals stand.
- The human `error: wrapper fault  <code>` line in `src/human/mirror.rs` and the `src/human.rs` refusal writer: both were not reached, and their output is unchanged.
