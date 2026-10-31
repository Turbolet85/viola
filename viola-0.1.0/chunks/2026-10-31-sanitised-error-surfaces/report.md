# Report — 2026-10-31-sanitised-error-surfaces

**Chunk:** codes and fixed messages only on CLI --json, MCP errors and channel error data, no paths or upstream text
**Date:** 2026-10-31T09:46:31Z
**Commits:** `fa7be1f3` chore(2026-10-31-sanitised-error-surfaces): operator pre-CI commit, for the run this chunk's verdict reads · not this chunk's, carried by its first push: `7bf5c32a` chore(records-door): commit B, the friction family moved into the store · `2cedb584` chore(records-door): commit A, the store's gitignore fragment · `8ebed5bc` chore(records-door): the bookkeeping left after the last wrap, ahead of the friction door (`git log --format='%h %s' 5273ab8c..HEAD`)

## Changes (structured — detectors read this)
- **Files:** 26 by `gate.py scope` (new 3 · modify 23). New: `tests/contract_error_bodies.rs` · `tests/channel_error_bodies.rs` · `tests/cli_error_bodies.rs`. Modified: `crates/viola-core/src/lib.rs` · `crates/viola-channel/src/lib.rs` · `crates/viola-channel/src/client.rs` · `crates/viola-channel/src/server.rs` · `schemas/diag-line.v1.json` · `src/cmd/client.rs` · `crates/viola-mcp/src/result.rs` · `crates/viola-mcp/src/lib.rs` · `crates/viola-mcp/tests/handlers.rs` · `crates/viola-agent-claude/src/lib.rs` · `src/run/mod.rs` · `src/cmd/run.rs` · `src/cmd/verify.rs` · `tests/support/events.rs` · `Cargo.toml` · `.config/nextest.toml` · `crates/viola-e2e/tests/mcp_tools.rs` · `src/cmd/hook/statusline.rs` · `src/run/version_gate.rs` · `src/run/budget.rs` · `src/run/dialog.rs` · `src/run/env.rs` · `tests/contract_bounded_readers.rs`. Ledger: `viola-0.1.0/verification-matrix.json` (`v1-47` `implemented`).
- **Symbols / APIs:**
  - `viola_core::is_code_word(&str) -> bool` (`crates/viola-core/src/lib.rs:347-357`): 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first. Callers: `src/cmd/client.rs` `reply_of`, `crates/viola-mcp/src/result.rs` `of_reply`; both also keep a `RefusalReason::Unknown.as_str()` fallback.
  - `viola_channel::ProtocolError::cause(self) -> Option<&'static str>` (`crates/viola-channel/src/lib.rs:102-110`): `unsupported-version` · `invalid-params` · `release-from-driver` for the three `-32602` faults, `None` for the other four.
  - `viola_channel::sanitised_fault(&Value) -> Option<Value>` (`crates/viola-channel/src/lib.rs:113-142`), with its private `is_version_text` (`:146-149`, 1 to 64 ASCII letters, digits, `.`, `-` or `+`): `None` without an integer `code`. For the five codes, `message` is the fault's fixed text. `-32602` with an integer `data.supported` and a version-text `data.wrapper` reads the unsupported-version body with those two values, `data.reason` exactly `release-from-driver` reads that body, and any other `-32602` reads `invalid params`, `data: null`. Every other known code reads `data: null`. A code outside the five reads `{"code": <code>, "message": "wrapper fault", "data": null}`. It replaces the two private copies of `fault_detail` (`src/cmd/client.rs`, `crates/viola-mcp/src/result.rs`), both gone; its two callers are `reply_of` and `of_reply`.
  - `reply_of` and `of_reply`: a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads `None` / `null`, a non-string `detail` is still no known reply. A code word outside the closed list stays visible in the document and in `refused: <word>`, and its exit stays `unknown`'s 14 by `reason_of`.
  - `crates/viola-channel/src/client.rs` `fault_of(&Value)` (was `fault_of(i64)`, sole non-test caller `reply_class`) reads `data` for `-32602`: `supported` · `reason` · neither.
  - `crates/viola-channel/src/server.rs` `ResponseLine`'s `Drop` writes `detail = fault.and_then(ProtocolError::cause)`, on both the server's and the client's line, level unchanged (WARN).
  - `viola_agent_claude::Refusal` removed. `AgentError` gains `BatchScriptChild` and `ProgramNotFound` (was `Refusal::NotFound`), their two messages unchanged. `resolve_program` returns `Result<PathBuf, AgentError>`, in the crate and in the root wrapper `src/run/mod.rs`. Callers `src/cmd/run.rs` and `src/cmd/verify.rs` match `AgentError::BatchScriptChild` and `Err(_)` for the rest (`start`'s `internal-error`, `measure`'s "not found" pair); `src/run/agents.rs` reads it through `let Ok(..) else`, unchanged. `src/cmd/revive.rs`'s private `Refusal` is another type and stays.
  - Tests' check: `tests/support/events.rs` `error_body_hit` (`:18`) and `assert_error_body_clean` (`:45`), classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes.
  - `crates/viola-mcp/src/result.rs`, `src/cmd/client.rs`: no public signature moved.
- **Crates / modules:** no crate added or removed. `viola-mcp` and the root bin's `src/cmd/client.rs` read `viola_channel::sanitised_fault` and `viola_core::is_code_word`; no manifest gains an edge.
- **Dependencies:** none added or bumped (`Cargo.lock` under the guard entry's list, unchanged).
- **Schema / config:** `schemas/diag-line.v1.json:42`: `channel-response`'s `detail` closes from `{"type":"string"}` to the enum `unsupported-version` · `invalid-params` · `release-from-driver`. Absent on every other `channel-response` line. The wire form does not change, `v` stays 1, and the seven `ProtocolError` bodies are unchanged. No config key, no env var.
- **Spec-master edits:** none before this wrap.
- **Retired:** `viola_agent_claude::Refusal`; both private `fault_detail` copies. `CoreError`, which architecture, security-plan and obs-plan list, never existed in the code: see Spec claims disproved. No version-census row applies.
- **Counts / qualifiers moved:**
  - `Refusal` folded: the `thiserror::Error` derives over product sources read 9, from 10 (`command grep -rn "thiserror::Error" src crates --include=*.rs` at research, less `Refusal`): `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError`. Pattern `refusal-enum`: 11 standing · 6 changed (`architecture.md:65`, `:106`, `:449`, `established-decisions.md:267`, `crate-dependency-direction.md:6` @c1247, `input-validation.md:74`) · 5 no-change (`resolve_program` truly named: `input-validation.md:73`, `:75`, `standard-contracts.md:93`, `:133`, `span-trace-coverage.md:44`).
  - Open divergences, three named to two named plus three the census adds. Pattern `enum-rule`: 38 standing · 6 changed (`architecture.md:65`, `:106`, `:449`, `established-decisions.md:267`, `security-plan.md:411`, `obs-plan.md:496`) · 4 handed to `exit-eleven` · 28 no-change (a variant truly named, e.g. `StampsError::StrictModes`, `PinError::HashMismatch`, `Refused`).
  - `channel-response`'s `detail`: pattern `response-line`: 47 standing · 2 changed (`log-coverage.md:13`, `metric-coverage.md:28`) · 45 no-change (the pair named, other fields stated).
  - The `ci` profile's kill-line filter: 22 to 24 `binary(…)` terms (a python count of `binary(` over `.config/nextest.toml` reads 24; the diff's removed line lists 22; `channel_error_bodies`, `cli_error_bodies` added). Pattern `kill-line`: 8 standing · 1 changed where the class is stated (`bootstrap-phases-derive-for-route-setup-project.md:10` @c2811) · 7 no-change (`architecture.md:385`, `e2e-test-strategy.md:38`, `:306`, `:322`, `quality-gates-coverage-targets.md:19`, `test-strategy.md:29`, `bootstrap…:12`).
  - Fact `new-binaries`: no master names `contract_error_bodies`, `channel_error_bodies` or `cli_error_bodies` yet: its basis is the refusal line `new-binaries · control UNFIRED over the seven masters (bodies and sets) at fa7be1f3`.
  - Line over 2 000 chars, per (line, fact): `crate-dependency-direction.md:6` @c1229 `resolve_program` no-change, @c1247 `Refusal` changed · `5-command-implementation.md:27` @c1876, @c1968 `WITHIN` no-change · `bootstrap…:10` @c2811 `kill-line` changed, @c3469, @c3825, @c4210, @c4613 `kill-line` no-change, @c4631 `WITHIN` no-change.
  - Sweep reach: 15 patterns · 14 moved facts, 14 with a pattern (the report's own count) · owner pattern 2 standing · by line 3 lines · 6 offsets. The generic-phrase and `new-binaries` controls fired nothing (`owner, new-binaries never fired over the seven masters' pre-pass text`).
- **Dev-tool versions:** none — no host tool changed.
- **Harness / gate surface:** `tests/contract_error_bodies.rs` carries `andromeda:walks-tree`. `tests/contract_bounded_readers.rs`: five `BOUNDARIES` rows' `witness` renamed to cap-driving cases (`:91`, `:109`, `:118`, `:127`, `:217`). `[[test]]` tables for `channel_error_bodies` and `cli_error_bodies` (`required-features = ["fake-agent"]`). No harness verb, CI step or status shape moved.
- **Cross-project / external claims:**
  - `ci#41188681876` over `fa7be1f37746`: `verdict: green · checks 15/15 · wall 838 s`, completed/success. Jobs `test (ubuntu-latest)` 3347 passed, `test (macos-latest)` 3301 passed, `test (windows-2025)` 3300 passed (`evidence/operator-pass.md`).
  - I1 unchanged · copy no-repo · `../additional/viola-overseer/phase-sanitised-errors-directive.md`, cited `scope.md` and `plan.md`.
  - inputs#I2 · n/a — a message has no live source · copy message · the overseer as the founder delegate, 2026-10-31T08:53:12Z, carrying the P5 word on `.andromeda/runs/2026-10-31T07-21-45-phase/p5-card-2.md` and three notes on `p5-card-1.md`. Card 2 governs; card 1 stands beneath it, its row reading "Proposed owner". The `## Beyond the plan` row of card 2, verbatim, authority delegate overseer, 2026-10-31, PROVISIONAL:
    - **An owner for the remaining enum divergences.** `ServeError` and `StampsError` stay open as the founder's Epoch 5 batch left them, and the census adds `PinError`, `StampsReadError` and `Refused`. Owner: a CARRY on the existing entry «Version done-check». The wrap writes it.
  - inputs#I3 · unchanged · copy no-repo · `../additional/viola-overseer/wrap-sanitised-errors-directive.md`, snapped at this wrap: the operator directive (the overseer, founder-delegated).
  - `inputs: 3 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 1 · uncited 2 · unparsed 0`, the two uncited being I2 and I3, cited here. No DRIFT.
- **Reverted / negative API facts:** none. No `-32603` is driven in a scanned home: no failpoint or counted panic was added; the unit layer holds it (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - «`ServeError::Initialize` (exit 11)» as `viola mcp`'s exit. Stated at `test-plan.md:390`, `established-decisions.md:130`, `input-validation.md:31`, `error-capture-reporting.md:39`, leaf `.claude/docs/services/viola-mcp.md:23`. Measured: `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` reads exit 1, both streams empty, one `parse-rejected{mcp-stdin, oversize, count:1}`; 11 is the unit child's own code (`serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error`, `serve_child_entry`); `src/main.rs` `exit_code` gives `Role::Mcp` + `Failed` exit 1. Pattern `exit-eleven`: 5 standing · 4 changed · 1 no-change (`unit-test-strategy.md:52`, no exit stated).
  - `CoreError` among the crate enums. Stated at `architecture.md:106`, `security-plan.md:411`, `obs-plan.md:496`, leaves `conventions.md:15`, `stack.md:23`, `services/viola-core.md:14`, `:25`. Measured: `command grep -n "CoreError" -r crates src --include=*.rs` 0 hits. Pattern `enum-rule`.
  - «oversize `answer` … exits non-zero with one `--json` error document» at `integration-test-strategy.md:59`. Measured: the exit-2 paths write the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line to stderr whatever `--json` says (`src/cmd/answer.rs:91`, `src/cmd/send.rs:187`), no document, named in `NOT_ERROR_BODIES`. Pattern `answer-oversize`: 2 standing · 1 changed · 1 no-change (`test-plan.md:202`, a label).
  - `mcp` writing a `process-exit` line. `.claude/rules/observability.md:36` says `mcp` main-thread writes `process-exit{detail:"internal-error"}`; `src/obs.rs:232` `internal_error_exit_line` writes for `Run` and `Cli` only. `error-capture-reporting.md:39` says «`process-exit` 0 once `initialize` has completed», and `command grep -n 'ObsEvent::ProcessExit' -r src crates --include=*.rs` shows no self `process-exit` written on the `mcp` role (`src/run/agents.rs:64` is the `agents-probe` pair); the case above asserts exactly one `parse-rejected` and does not read a self line at exit 0. Pattern `mcp-exit-line`: 3 standing · 1 changed (`error-capture-reporting.md:39`) · 2 no-change (`obs-plan.md:523`, `obs-scope-summary.md:270`).
  - test-plan §6's after-every-test error-body check («after every E2E test … the `events` support helper checks», `test-plan.md:605`, `e2e-test-strategy.md:259`). Measured absent at `tests/support/events.rs` before this chunk (research §Premise closure). It is not built: the three surface witnesses, the enumeration and the census of writers replace it, reading the check in `tests/support/events.rs`. Pattern `after-every-test`: 3 standing · 2 changed · 1 extended (`occupied-resources.md:205`).
  - The scope's «`error.data` a code or null» and «`detail` code-shaped or null» for `wrapper-fault`: disproved at phase and premise-corrected in `scope.md`; no master states them (no master site). The contract's closed objects stand.
- **Expected amendments (from plan):**
  - architecture §Established Decisions [Error Handling] · §Conventions, Rust error types · §Inherited Defaults, Errors — carried: Symbols / APIs and Counts: `Refusal` folded, `ServeError` and `StampsError` the open divergences, `CoreError` retired, `PinError`, `StampsReadError` and `Refused` named, 9 derives. Search: patterns `refusal-enum`, `enum-rule`; architecture hits `architecture.md:65`, `:106`, `:449`, `established-decisions.md:267`.
  - architecture §Conventions, Error handling schema — carried: `unknown`'s detail a code word or null, a reader's opaque rule kept. §Standard Contracts, CLI `--json` output — carried: `wrapper-fault`'s fields closed. §Established Decisions [MCP] · §Occupied Resources (`mcp`) — carried: "exit 11" corrected. Search: `code-word` (9 standing: `architecture.md:98`, `conventions.md:15`, `color-palette.md:102`, `surface.md:81`, `:147`, `layout-templates.md:331`, `a11y-plan.md:257`, `:345`, `cognitive-accessibility.md:19`, all 9 changed); `fault-reader` (15 standing · 3 changed: `architecture.md:227`, `conventions.md:64`, `security-plan.md:413` · 12 no-change, the word `wrapper-fault` truly used); `exit-eleven` (`established-decisions.md:130`; 0 hits in `occupied-resources.md`, so the §Occupied Resources half has no site under that pattern, the detector's own sweep the control).
  - security-plan §Error Handling (the enum list, the clients' screen, `sanitised_fault`) · §Input Validation (MCP stdin, the exit) — carried. Search: `enum-rule` (`security-plan.md:411`), `fault-reader` (`security-plan.md:413`), `exit-eleven` (`input-validation.md:31`), `refusal-enum` (`input-validation.md:74`).
  - obs-plan §5 (`viola.compat.v_rejected` ← `channel-response{error_code:-32602, detail:"unsupported-version"}`, the over-count retired) · §6 (the three `detail` words) · §7 (the enum list, the stdin clause's exit, `mcp`'s catch site writes no `process-exit`) — carried. Search: `v-rejected` (2 standing · 2 changed: `obs-plan.md:418`, `metric-coverage.md:28`), `response-line` (`log-coverage.md:13`), `enum-rule` (`obs-plan.md:496`), `exit-eleven` and `mcp-exit-line` (`error-capture-reporting.md:39`). The leaf `.claude/rules/observability.md:36` is the cascade's.
  - test-plan §4 viola-mcp, §6 Contract suite: the two owed clauses discharged — carried: Harness and Outcome. Search: `title` (`test-plan.md:390`, `:623`, 2 standing, both changed), `bound-rows` (6 standing · 1 changed `test-plan.md:623` · 5 no-change, the binary truly named: `architecture.md:422`, `cross-cutting-patterns.md:27`, `input-validation.md:98`, `test-plan.md:622`, `e2e-test-strategy.md:371`). §5 the `viola answer` oversize sentence — carried, `answer-oversize`. §6 the after-every-test helper, stating plainly that it was replaced and not built (inputs#I2 note 2; inputs#I3 item 4) — carried, `after-every-test`. The `ci` profile's kill line count, by its `binary(…)` rule — carried, `kill-line`.
  - design-system §Surface: cli, Exit-code phraseology row 14 · surface.md · §Semantic Colors `RefusalReason::unknown`: a code word or nothing, not opaque free text — carried. Search: `code-word` (`color-palette.md:102`, `surface.md:81`, `:147`).
  - layout-templates §Component — Primary content block 2 · a11y-plan §8 Error recovery: `unknown`'s detail the same — carried. Search: `code-word` (`layout-templates.md:331`, `a11y-plan.md:257`, `:345`, `cognitive-accessibility.md:19`).
  - The directive's owner row: a CARRY on «Version done-check» for `ServeError`, `StampsError`, `PinError`, `StampsReadError` and `Refused` (inputs#I2 card row; inputs#I3 item 4) — no master site for the route edit: P5 writes it.
- **Coverage of new surfaces:**
  - `reply_of` / `of_reply`'s screen (a peer's reply) → validation `is_code_word`, `sanitised_fault`✓ · instrumentation n/a (no line added) · PII redacted✓ · tests unit (`reply_of_reads_the_three_shapes`, `of_reply_maps_each_reply_shape`), integ (`handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail`) · a11y n/a · tokens n/a
  - `channel-response.detail` → validation closed enum✓ · instrumentation log✓ (`obs_event!`, schema) · PII no param value or name✓ · tests unit (`answer_invalid_params_names_its_cause_and_no_param`, `answer_faults_other_than_invalid_params_carry_no_detail`, `reply_from_logs_the_cause_of_each_invalid_params`), integ (`channel_error_bodies_hold_no_path_or_upstream_text`) · a11y n/a · tokens n/a
  - The `message` literal `wrapper fault` in `wrapper-fault`'s detail, for a code outside the five → a new fixed output word on `--json` and MCP. validation n/a · instrumentation n/a · PII fixed✓ · tests unit (`sanitised_fault_carries_nothing_of_the_peer_but_closed_values`) · a11y n/a · tokens n/a. The human line `error: wrapper fault  <code>` (`src/human/mirror.rs`) is unchanged.
  - The new `AgentError` variants → messages unchanged, outputs unchanged · tests (`tests/cli_program_resolution.rs`, `tests/cli_verify.rs`, `refusal_messages_are_fixed`).

## Deviations from intent
From implement's outcome (`.andromeda/runs/2026-10-31T08-55-19-implement/outcome-2026-10-31-sanitised-error-surfaces.md`):
1. Step 7 boots through `OuterPty::spawn_without`, not `Wrapper` or `Command::env`: `Wrapper` takes no environment and `tests/support/home.rs` is outside the plan's lists and the guard's allow. The planted value reaches that child through the PTY spawn's `env_set`, as `tests/tui_env_strip.rs` does.
2. `tests/contract_error_bodies.rs` holds its own scanner beside `tests/contract_bounded_readers.rs`'s: that one blanks string literals, this census reads them, and a shared helper would be a `tests/support/` file outside the allow.
3. A `PLUMBING` table is added to step 11's tables: it names the census lines that build or route a document a row names, so a row stays one body.
4. The MCP witness's session is unstamped: no row reaches a stamped path.
5. The first block's red, an oracle of this chunk: the unknown tool's `data` was asserted `Some(Null)`; rmcp's client reads the wire's `"data":null` as `None`, so the live case reads `None`. The wire's null is the product's `Some(Value::Null)` (`crates/viola-mcp/src/tools.rs`).

scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 26 · listed 26 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 585`).

## Decisions & corrections
- The P5 word is the delegate overseer's, PROVISIONAL, owed the founder's own word (inputs#I2; inputs#I3 item 5).
- `v1-47`'s "after every end-to-end test" is concretized to "after each job's end-to-end run", accepted by that same delegate word, measured: `AGENT_RUN_KEEP_HOMES` `"1"` on the job (`.github/workflows/ci.yml:20`), `if: always()` (`:104`), the `viola-chaos-*` home out by the founder's ruling of 2026-10-04 (`.andromeda/obs-plan-detail/ci-integration.md:59`).
- Leaned at the plan, standing: `Refusal` folded, not ratified, `StampError` the precedent. The process exit stays 1 and the masters move. A refusal's non-code `detail` reads null.
- A plan's reason removed at review (inputs#I2 note 1): a directive item holding execution to the finished plan decides nothing about what a plan lists, so it was struck as a reason for not wiring the check after every test.
- The enum census: research counted 10 `thiserror` derives and found the masters' list stale in both directions (`CoreError` absent, `PinError`, `StampsReadError`, `Refused` unnamed). The fix was a census, not a wider fold; the freight owned `Refusal` alone.
- Stamp-ahead hook, one refusal: a journal line stamped `09:17Z` by estimate while the clock read 09:14:46Z, corrected from `date -u`. A recurrence of the Tier 1 learning on timestamps. The journal's earlier `09:08Z` and `09:11Z` were written the same way and passed the hook.
- Bash guard, one refusal at implement: a `cat` heredoc to a run-dir script, re-issued once as a Write-tool file. At this wrap, one refusal: `route.py epoch … | sed` (a pipeline tool's output into a filter), re-issued without the tool.
- `controls.py`'s last-line summary was shadowed by its own loop and printed `no summary`; the record's line is copied from its printed output. Its fourteenth plant did not build and proved nothing, so it was re-planted and read red (`evidence/remove-the-guard.md`).
- A test oracle written against the wire form reads red through the client: rmcp's client turns the wire's `"data":null` into `None` (deviation 5).
- A red the operator pass met outside the block: `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1`, both cases, at the 7 s `WITHIN` under a host record of `CONTENDED`; about 4.3 s alone. Its owner or note is P5's (Outcome below).
- Sweep hazard: a pattern on `exit 11` alone reads `budget-paused`'s exit 11; `mcp_tools.rs:190`'s `Some(11)` is that code (research §Sweeps). The sweep's pattern keys on `ServeError::Initialize` and `initialize`.
- Sweep hazard: `Refusal` as a bare token reads `RefusalReason` and `src/cmd/revive.rs`'s private enum; the pattern is the backticked name and its variants.
- Operator directive for this wrap (inputs#I3): the sampler forced, every subagent and Workflow transcript and three run dirs kept; a hard cascade failure, agents writing nothing or doing the orchestrator's work stops the wrap with no re-spawn around it; each check 7 `contradiction` raised at the P2 halt; no next chunk taken up; anything the store changes under the wrap reported; past about 80 % before Apply, stop at a batch boundary.

## Outcome
**Acceptance, re-asserted against the diff:**
- `v1-47`: met on the diff as far as P7's flip reads it: the enumeration, the three witnesses and the step 12 witnesses read green at the final HEAD's run; `secret-scan` `success` on the three `test` jobs, each document `"ok":true`, `"hits":[]`, no line naming `claude-stripped` (`evidence/operator-pass.md`).
- A peer's `message`, `data`, `refusal` or `detail` reaches no document or text: met (`sanitised_fault`'s table, `reply_of_reads_the_three_shapes`, `of_reply_maps_each_reply_shape`, `handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail`). A code word outside the closed list stays visible by design.
- The seven wire bodies unchanged, `v` 1, a too-new request answered `-32602` with the wrapper's version: met (`protocol_error_body_is_the_literal_wire_object`, `channel_error_bodies_hold_no_path_or_upstream_text`).
- Each CLI `--json` failure the witness drives: one document, literal exit, stderr empty, no ESC, clean: met (`cli_error_documents_hold_no_path_or_upstream_text`).
- `channel-response`'s three words on both sides, level unchanged, schema closed, no name or param on the line: met (`link_verbs_log_the_channel_pair_and_no_names` green).
- `Refusal` gone, outputs unchanged, no manifest edge: met (`tests/cli_program_resolution.rs`, `tests/cli_verify.rs`).
- The stdin line before `initialize`: met (`serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error`, `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`).
- The five `BOUNDARIES` rows each name a cap-driving case: met (`statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap`, `statusline_shell_out_a_command_the_cut_ends_is_a_failed_command`, `drain_keeps_cap_bytes_of_an_endless_pipe`, `read_thresholds_keeps_no_key_past_the_frame_cap`, `read_hold_keeps_no_key_past_the_frame_cap`, `env_keep_keeps_no_key_past_the_frame_cap`). The census and its self-check green.
- `run --unit` and the pre-push gate green, coverage per OS, `secret-scan` clean: met (`"suite":"coverage","passed":3347,"failed":0`, `gate` `"breaches":[]`, the second pre-push).
- Human-mode refusals keep their pair, `hint:` last, `unknown` and the faults none, no SGR in a `--json` document: met (the integration entry's pins).

**Gates (implement's second firing, `entries 25 · green 21 · red 0 · recorded 1 · timeout 0 · not-run 3`; P7's light gate re-reads them):**
- `df -h /tmp` `recorded`, exit 0, report-only · `cargo fmt --all --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green · the same under `CARGO_TARGET_DIR=target/wincheck … --target x86_64-pc-windows-msvc` green · the G1 `rg` green (exit 1, no output) · `bash scripts/lint-probes.sh` green · `bash scripts/agent-run.sh run --unit` green.
- `cargo nextest … binary(channel_error_bodies) | binary(cli_error_bodies)` green, `16 tests run: 16 passed` · the pins entry green, `294 tests run: 294 passed` · `run --e2e` over `mcp_tools`, `mcp_recorded_exchange`, `contract_bounded_readers`, `contract_diag_schema`, `path1_run_start` green · `run --e2e` over `contract_error_bodies` green.
- `run --perf` green; 2.0 / 2.0 / 2.0 / 2.0 / 2.2 ms mean, against 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms before; no cause claimed for the difference.
- `cargo deny check` green · `bash scripts/channel-tokio-free.sh` green · `gate.py guard` green (`guard: allowed 26`, `excluded 0`).
- The smoke entries (`cleanup`, `boot --unstamped`, `status`, `g2-zero-panics`, `schema-check`, `cleanup`) green · `bash scripts/agent-run.sh pre-push` green.
- `leg = 'operator'` entries: `gate.py hygiene` `hygiene: clean` · the push green, history moved `5273ab8c→fa7be1f3` · `ci.py conclusion` `fa7be1f37746 verdict: green · checks 15/15`, every OS-sensitive case PASS on the jobs that run it, the two `cfg(unix)` status-line cases not run on Windows by design, and `.cmd` refusal cases PASS on `windows-2025` (`evidence/operator-pass.md`).

**A red outside the block, for P5:** `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1`, `case_1_sigint` (7.535 s) and `case_2_sigterm` (7.188 s), `timed out waiting for Run E's start` at the 7 s `WITHIN` of `tests/support/fake.rs`'s `wait_for`, in the operator pass's first pre-push re-run: `3347 tests run: 3345 passed (22 slow), 2 failed`, host record `CONTENDED … 20 s stalled on IO: 09:20:14Z..09:20:34Z`, `escher` and `viola` builds alive, load peak 90.7. The same tree read green in the block's second firing, in five rounds alone (about 4.3 s, four `QUIET`, one `CONTENDED`) and in the next pre-push (`QUIET`). This chunk's only edit on the path is the `AgentError` rename in `src/cmd/verify.rs`, ahead of any run. Not measured: the nextest log carries no wall clock per test, so the two cases' place inside the stalled window is inferred from their position (391 and 400 of 3347), not read. So the cause reads as host contention by inference, not proof. No bound moved. Owner or note: P5 (inputs#I3 item 4).

**Watches:** Path 1's CI timeout · 3 green runs [`ci#40458923095` 15.010 s, `ci#40467224735` 14.205 s, `ci#41188681876` 9.067 s, all on `test (ubuntu-latest)`], its retire condition reached. The handoff's "1/3" predates Setup 5a's reading; the tally is `scope.md`'s and the operator pass's. Its runner-variance reading stays PROVISIONAL as written until the founder's own word (inputs#I3 item 4). P5 retires it.

**Outcome basis:** the operator pass ran: `fa7be1f3` the only commit from `7bf5c32a` on that is this chunk's, no fix commit, final HEAD's run `ci#41188681876` recorded in `evidence/operator-pass.md`, which P7 re-verifies. Implement's persisted report is `.andromeda/runs/2026-10-31T08-55-19-implement/outcome-2026-10-31-sanitised-error-surfaces.md`, read whole. Controls: 14 of 14 guards red under their plant and green restored (`evidence/remove-the-guard.md`). Smoke by hand on `p-errors-hand`: boot exit 0, status `ready`, cleanup `processes_gone` and `endpoint_gone` true. Run this wrap's `gate.py scope`, `inputs.py verify`, `cites.py added`, `cascade.py sweep --pre-pass`.

**Requests for P5 and P7:** P5: the CARRY on «Version done-check»; the verify margin's owner or note; the WATCH's retirement; no next chunk taken up (inputs#I3 item 6). P7.3: `v1-47` flips by the coverage gate.

**Process hygiene:** implement's census `census: 8 running`, none from this repository's `target/`. Re-measured at this wrap, `gate.py census --comm viola --comm viola-fake-agent --comm viola-harness --args projects/viola/target`: `census: 8 running`, none from this repository's `target/`: the `viola run` and `viola wait` rows under `~/dev/projects/additional/viola-lab/prototype/target/debug/` (another tree's bridge), `~/.viola/bin/0.1.0-231c0225dac912d9/viola mcp` (another session's server), `~/.local/viola-product/bin/viola wait viola-builder …` (the overseer pair's bridge). Left running: not this chunk's, their owners stop them. The harness sessions `p-errors-smoke` and `p-errors-hand` terminated.
## New text, by line
Generated by `cites.py added` (cites v1.7); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 7bf5c32a (the parent of the oldest pre-CI commit fa7be1f3) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .config/nextest.toml — added 1 line(s) in 1 range(s)
added: 24
### Cargo.toml — added 10 line(s) in 1 range(s)
added: 31-40
### crates/viola-agent-claude/src/lib.rs — added 15 line(s) in 12 range(s)
added: 209 · 233 · 238 · 251 · 448 · 452 · 461 · 470 · 540 · 546 · 696 · 699-702
### crates/viola-channel/src/client.rs — added 50 line(s) in 8 range(s)
added: 205 · 210-220 · 225 · 346 · 613-616 · 618 · 669-670 · 672-700
  - 214-220 «let invalid = if error["data"].get("supported").is_some() {»
### crates/viola-channel/src/lib.rs — added 169 line(s) in 2 range(s)
added: 101-149 · 305-424
  - 102-110 @103 «pub const fn cause(self) -> Option<&'static str> {»
- 113-142 @117 «pub fn sanitised_fault(error: &Value) -> Option<Value> {»
  - 120-122 «let wrapper = data["wrapper"]»
  - 123-140 «let (message, data) = match code {»
  - 146-149 «(1..=64).contains(&text.len())»
  - 305-318 @313 «fn protocol_error_cause_tells_the_three_invalid_params_apart(»
  - 332-337 @333 «fn sanitised_fault_keeps_each_wire_body(#[case] wire: &str) {»
  - 398-404 «fn sanitised_fault_carries_nothing_of_the_peer_but_closed_values(»
  - 406-415 @407 «fn sanitised_fault_keeps_a_wrapper_of_sixty_four_version_characters() {»
  - 417-423 @421 «fn sanitised_fault_without_an_integer_code_is_nothing(#[case] error: Value) {»
### crates/viola-channel/src/server.rs — added 40 line(s) in 4 range(s)
added: 500 · 609 · 614-650 · 723
  - 614-637 @616 «fn answer_invalid_params_names_its_cause_and_no_param() {»
  - 639-649 @640 «fn answer_faults_other_than_invalid_params_carry_no_detail() {»
### crates/viola-core/src/lib.rs — added 58 line(s) in 2 range(s)
added: 347-358 · 869-914
- 347-357 @350 «pub fn is_code_word(word: &str) -> bool {»
  - 352-356 «bytes.len() <= 64»
  - 869-913 @870 «fn code_word_literal_table() {»
### crates/viola-e2e/tests/mcp_tools.rs — added 311 line(s) in 7 range(s)
added: 441-442 · 447-451 · 453 · 455 · 464-763 · 796 · 816
- 467-483 @468 «fn exit_within_bound(child: &mut std::process::Child) -> Option<std::process::ExitStatus> {»
  - 472-482 «loop {»
- 485-517 @487 «fn assert_one_oversize_line(home: &Path) {»
  - 490-493 «let rejected: Vec<&Value> = lines»
  - 498-500 «let schema: Value = serde_json::from_slice(»
  - 503-506 «let errors: Vec<String> = validator»
  - 509-516 «for file in files_under(home) {»
- 519-561 @522 «fn mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line() {»
  - 530-534 «let out = std::thread::spawn(move || {»
  - 535-539 «let err = std::thread::spawn(move || {»
  - 541-548 «let writer = std::thread::spawn(move || {»
  - 553-557 «assert_eq!(»
- 563-636 @566 «fn mcp_tools_a_wrong_shaped_call_answers_no_caller_text() {»
  - 576-583 «let reader = std::thread::spawn(move || {»
  - 585-589 «let opening = concat!(»
  - 593-598 «let call = json!({»
  - 600-603 «let rest = format!(»
  - 607-618 «let listed = loop {»
  - 623-626 «for line in &lines {»
  - 627-630 «assert!(»
  - 631-635 «assert_eq!(»
- 638-669 @640 «fn assert_no_path_or_upstream_text(label: &str, value: &Value, home: &Path) {»
  - 641-651 «fn strings<'a>(value: &'a Value, out: &mut Vec<&'a str>) {»
  - 655-668 «for text in all {»
- 671-752 @674 «fn mcp_tools_error_results_hold_no_path_or_upstream_text() {»
  - 675-682 «let s = harness_session(»
  - 687-718 «let rows: [(&str, &'static str, Value, Value); 5] = [»
  - 719-731 «for (label, tool, args, want) in rows {»
  - 733-738 «let unknown = mcp.runtime.block_on(»
  - 739-741 «let Err(rmcp::ServiceError::McpError(fault)) = unknown else {»
### crates/viola-mcp/src/lib.rs — added 20 line(s) in 1 range(s)
added: 165-184
  - 165-183 @168 «fn serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error() {»
### crates/viola-mcp/src/result.rs — added 40 line(s) in 5 range(s)
added: 9-10 · 58 · 71-75 · 78 · 153-183
  - 71-75 «let reason = if is_code_word(reason) {»
### crates/viola-mcp/tests/handlers.rs — added 27 line(s) in 1 range(s)
added: 244-270
- 244-269 @246 «fn handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail() {»
  - 247-253 «let wrapper = booted(»
  - 254-258 «let result = call(»
  - 261-264 «assert_eq!(»
  - 265-268 «assert_eq!(»
### schemas/diag-line.v1.json — added 1 line(s) in 1 range(s)
added: 42
### src/cmd/client.rs — added 31 line(s) in 6 range(s)
added: 12 · 14 · 70 · 76-81 · 84 · 373-393
  - 77-81 «let reason = if is_code_word(reason) {»
### src/cmd/hook/statusline.rs — added 28 line(s) in 1 range(s)
added: 652-679
### src/cmd/run.rs — added 3 line(s) in 3 range(s)
added: 13 · 266 · 270
### src/cmd/verify.rs — added 4 line(s) in 4 range(s)
added: 31 · 151 · 156 · 163
### src/run/budget.rs — added 19 line(s) in 1 range(s)
added: 430-448
  - 430-447 @433 «fn read_thresholds_keeps_no_key_past_the_frame_cap() {»
### src/run/dialog.rs — added 21 line(s) in 1 range(s)
added: 789-809
  - 789-808 @792 «fn read_hold_keeps_no_key_past_the_frame_cap() {»
### src/run/env.rs — added 25 line(s) in 1 range(s)
added: 213-237
  - 213-236 @216 «fn env_keep_keeps_no_key_past_the_frame_cap() {»
### src/run/mod.rs — added 2 line(s) in 2 range(s)
added: 20 · 144
### src/run/version_gate.rs — added 7 line(s) in 1 range(s)
added: 218-224
  - 218-223 @219 «fn drain_keeps_cap_bytes_of_an_endless_pipe() {»
### tests/channel_error_bodies.rs — new file · 231 line(s)
- 22-25 «use support::home::{»
- 38-70 @40 «fn booted_with_the_value(stamped: &StampedHome) -> OuterPty {»
  - 56-63 «let pty = OuterPty::spawn_without(»
  - 64-68 «wait_events(»
- 72-74 «fn frame(id: u64, method: &str, params: &Value) -> String {»
- 76-97 @77 «fn wait_details(home: &Path, wanted: &[(u64, &str)]) -> Vec<Value> {»
  - 81-96 «loop {»
- 99-231 @100 «fn channel_error_bodies_hold_no_path_or_upstream_text(stamped_home: StampedHome) {»
  - 107-110 «let name = endpoint»
  - 116-175 «let rows: [(&str, String, u64, &str); 7] = [»
  - 176-193 «for (label, line, id, error) in &rows {»
  - 195-204 «let lines = wait_details(»
  - 207-210 «for line in &lines {»
  - 211-214 «let unknown_method = lines»
  - 216-220 «let mut another_word = lines»
  - 222-225 «assert!(»
### tests/cli_error_bodies.rs — new file · 296 line(s)
- 26-35 @28 «struct Row {»
- 37-46 «fn row(label: &'static str, args: &'static [&'static str], exit: i32, doc: &'static str) -> Row {»
  - 38-45 «Row {»
- 48-60 @49 «fn fed(»
  - 56-59 «Row {»
- 62-102 @64 «fn assert_row(home: &Path, bin: &Path, row: &Row) {»
  - 66-78 «command»
  - 79-81 «for name in UNWRAPPED {»
  - 82-84 «if let Some(from) = row.from {»
  - 86-92 «if let Some(text) = &row.stdin {»
- 104-107 @105 «fn loaded(home: &Path, tail: &str) -> String {»
- 109-112 @110 «fn loaded_answer(home: &Path) -> String {»
- 114-260 @115 «fn cli_error_documents_hold_no_path_or_upstream_text(stamped_home: StampedHome) {»
  - 120-177 «let dead = [»
  - 178-180 «for row in &dead {»
  - 186-194 «let refused = [»
  - 195-197 «for row in &refused {»
  - 203-255 «let live = [»
  - 256-258 «for row in &live {»
- 262-284 @277 «fn error_body_check_reads_each_class_as_a_hit(#[case] body: Value, #[case] hit: Option<&str>) {»
  - 279-283 «assert_eq!(»
- 286-296 @288 «fn error_body_check_reports_no_byte_of_what_it_read() {»
  - 290-292 «let panicked = std::panic::catch_unwind(|| {»
### tests/contract_bounded_readers.rs — added 5 line(s) in 5 range(s)
added: 91 · 109 · 118 · 127 · 217
### tests/contract_error_bodies.rs — new file · 791 line(s)
- 15-26 @16 «struct Body {»
- 33-34 «const SEND_REFUSAL: &str =»
- 36-181 «const CLI_DOCUMENTS: &[Body] = &[»
  - 37-44 «Body {»
  - 45-52 «Body {»
  - 53-60 «Body {»
  - 61-68 «Body {»
  - 69-76 «Body {»
  - 77-84 «Body {»
  - 85-92 «Body {»
  - 93-100 «Body {»
  - 101-108 «Body {»
  - 109-116 «Body {»
  - 117-124 «Body {»
  - 125-132 «Body {»
  - 133-140 «Body {»
  - 141-148 «Body {»
  - 149-156 «Body {»
  - 157-164 «Body {»
  - 165-172 «Body {»
  - 173-180 «Body {»
- 186-243 «const MCP_RESULTS: &[Body] = &[»
  - 187-194 «Body {»
  - 195-202 «Body {»
  - 203-210 «Body {»
  - 211-218 «Body {»
  - 219-226 «Body {»
  - 227-234 «Body {»
  - 235-242 «Body {»
- 245-253 @246 «const MCP_FIELD_WORDS: [&str; 6] = [»
- 255-292 @256 «const CHANNEL_FAULTS: &[(ProtocolError, &str, &str)] = &[»
  - 257-261 «(»
  - 262-266 «(»
  - 267-271 «(»
  - 272-276 «(»
  - 277-281 «(»
  - 282-286 «(»
  - 287-291 «(»
- 294-306 @295 «const CHANNEL_METHODS: [&str; 10] = [»
- 308-360 @309 «const NOT_ERROR_BODIES: &[(&str, &str, &str)] = &[»
  - 310-314 «(»
  - 315-319 «(»
  - 320-324 «(»
  - 325-329 «(»
  - 330-334 «(»
  - 335-339 «(»
  - 340-344 «(»
  - 345-349 «(»
  - 350-354 «(»
  - 355-359 «(»
- 362-399 @363 «const PLUMBING: &[(&str, &str, &str)] = &[»
  - 364-368 «(»
  - 369-373 «(»
  - 374-378 «(»
  - 379-383 «(»
  - 384-388 «(»
  - 389-393 «(»
  - 394-398 «(»
- 401-409 «const PRODUCT_ROOTS: [&str; 7] = [»
- 414-416 «fn workspace() -> PathBuf {»
- 418-420 «fn read(path: &Path) -> String {»
- 422-431 «fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {»
  - 423-430 «for entry in fs::read_dir(dir).into_iter().flatten().flatten() {»
- 433-455 @435 «fn product_files(root: &Path) -> Vec<(String, String)> {»
  - 437-439 «for dir in PRODUCT_ROOTS {»
  - 440-454 «files»
- 457-541 @459 «fn blanked(text: &str, strings: bool) -> Vec<u8> {»
  - 462-468 «let blank = |out: &mut Vec<u8>, from: usize, to: usize| {»
  - 471-539 «while i < bytes.len() {»
- 543-548 @544 «fn raw_string_opening(rest: &[u8]) -> Option<(usize, usize)> {»
- 550-566 @551 «fn char_literal_len(rest: &[u8]) -> Option<usize> {»
  - 552-555 «if rest.get(1) == Some(&b'\\') {»
  - 556-562 «let text = std::str::from_utf8(&rest[1..rest.len().min(6)])»
- 568-602 @570 «fn product_view(text: &str) -> String {»
  - 575-600 «while let Some(found) = code[at..].windows(marker.len()).position(|w| w == marker) {»
- 604-617 @605 «fn unnamed_writers(file: &str, text: &str, named: &[(&str, &str)]) -> Vec<String> {»
  - 606-616 «product_view(text)»
- 619-636 @621 «fn error_attributes(text: &str) -> (usize, usize) {»
  - 625-634 «for (at, _) in view.match_indices("#[error(") {»
- 638-645 @639 «fn defined_functions(root: &Path) -> String {»
  - 641-643 «for dir in ["src", "crates", "tests"] {»
- 647-649 «fn every_body() -> impl Iterator<Item = &'static Body> {»
- 651-708 @652 «fn error_bodies_each_name_a_writer_and_a_witness_that_stand() {»
  - 655-659 «let defined = |witness: &str| {»
  - 661-673 «for body in every_body() {»
  - 674-676 «for body in CLI_DOCUMENTS {»
  - 677-683 «for body in MCP_RESULTS {»
  - 684-688 «for (_, _, witness) in CHANNEL_FAULTS {»
  - 690-694 «for word in MCP_FIELD_WORDS {»
  - 696-700 «for method in CHANNEL_METHODS {»
  - 701-706 «for (file, needle, why) in NOT_ERROR_BODIES.iter().chain(PLUMBING) {»
- 710-717 @712 «fn channel_faults_are_the_seven_literal_bodies() {»
  - 713-715 «for (fault, wire, _) in CHANNEL_FAULTS {»
- 719-740 @720 «fn product_error_writers_are_each_named() {»
  - 722-725 «let named: Vec<(&str, &str)> = every_body()»
  - 728-734 «for (file, text) in product_files(&root) {»
  - 736-739 «assert!(»
- 742-756 @743 «fn thiserror_messages_interpolate_nothing() {»
  - 747-753 «for (file, text) in product_files(&root) {»
- 776-791 @783 «fn census_finds_an_unnamed_writer_and_an_interpolated_message(»
### tests/support/events.rs — added 60 line(s) in 2 range(s)
added: 10-11 · 15-72
- 15-41 @18 «pub fn error_body_hit(body: &Value, home: &Path, planted: &[&str]) -> Option<&'static str> {»
  - 22-40 «strings.into_iter().find_map(|text| {»
- 43-49 @45 «pub fn assert_error_body_clean(place: &str, body: &Value, home: &Path, planted: &[&str]) {»
  - 46-48 «if let Some(class) = error_body_hit(body, home, planted) {»
- 51-61 «fn strings_of<'a>(value: &'a Value, out: &mut Vec<&'a str>) {»
  - 52-60 «match value {»
- 63-71 @64 «fn has_drive_letter(text: &str) -> bool {»
  - 65-70 «text.as_bytes().windows(3).enumerate().any(|(at, window)| {»
