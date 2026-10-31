# Task — architecture · 2026-10-31-sanitised-error-surfaces

## The session
Closed the error surfaces: CLI `--json` documents, MCP errors and channel error data carry codes and fixed messages only, no path and no upstream text; the wire form, `v` 1 and the seven `ProtocolError` bodies did not change.
Added `viola_core::is_code_word`, `viola_channel::sanitised_fault` and `ProtocolError::cause`; `reply_of` and `of_reply` now screen a peer's reply; `channel-response`'s `detail` closed to a three-word enum in `schemas/diag-line.v1.json`.
Retired `viola_agent_claude::Refusal` (folded into `AgentError`) and both private `fault_detail` copies; measured that `CoreError` never existed in the code; the `thiserror` derive census reads 9, from 10.
Added three test binaries (`contract_error_bodies`, `channel_error_bodies`, `cli_error_bodies`) and the error-body check in `tests/support/events.rs`, renamed five `BOUNDARIES` witnesses to cap-driving cases, and moved the `ci` kill-line filter from 22 to 24 `binary(…)` terms.
Measured against standing claims: `viola mcp` exits 1, not 11, on a line past `MAX_FRAME` before `initialize`; an oversize `answer` writes a fixed stderr line and no document; the `mcp` role writes no self `process-exit`; the after-every-test helper was absent and is replaced, not built. CI `ci#41188681876` over `fa7be1f3` read green, 15/15.
No crate, dependency, config key, env var, harness verb, CI step or status shape moved. The P5 word behind the owner row and the `v1-47` concretization is the delegate overseer's, provisional, owed the founder's own word.

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
- dec-error-handling (`.andromeda/architecture-detail/established-decisions.md:267`) and the body's [Error Handling] (`.andromeda/architecture.md:65`): `Refusal` is gone, so the three named open divergences become `ServeError` and `StampsError` plus the census's `PinError`, `StampsReadError` and `Refused`; `PtyError` stays the one ratified exception.
- The body's Conventions, Rust error types (`.andromeda/architecture.md:106`): `CoreError` is listed but never existed and is retired from the list; `Refusal` beside `AgentError` is out; the divergences read as above.
- The body's Inherited Defaults, Errors (`.andromeda/architecture.md:449`): names `Refusal` as an open divergence and must read the new list.
- The owner of those open divergences is a CARRY on the route entry `Version done-check`, written by P5, provisional; the report names no architecture site for that route edit, so check only that any standing owner wording agrees.
- The crate-dependency-direction contract, line 6, at its `Refusal` mention (offset 1247): changed; its `resolve_program` mention on the same line (offset 1229) stays.
- conv-refusal-reason (`.andromeda/architecture-detail/conventions.md:15`) and the body (`.andromeda/architecture.md:98`): `unknown`'s detail is a code word or null, no longer opaque free text; a reader's opaque rule for an unknown detail is kept.
- conv-mcp-results (`.andromeda/architecture-detail/conventions.md:64`): a wrapper fault's MCP result now passes through `sanitised_fault`, its fields closed.
- conv-mcp-results, the unknown tool's fault: its wire `data` is null while rmcp's client reads that as `None`; check any wording that states the client's view.
- contract-cli-json-output and the body (`.andromeda/architecture.md:227`): `wrapper-fault`'s `detail` fields are closed: fixed `message`, `data` a closed object or null.
- The body's channel protocol fault shape (`.andromeda/architecture.md:96`) and the unsupported-version frame in Wrapper channel frames: re-read, unmoved on the wire; the screen sits in the readers.
- contract-channel-methods, `release`'s `release-from-driver` body: unmoved, but the same cause word now also appears as `detail` on the `channel-response` line, beside its own obs line.
- dec-mcp (`.andromeda/architecture-detail/established-decisions.md:130`): names `ServeError::Initialize` as exit 11; corrected to process exit 1. `budget-paused`'s exit 11 is a different code and does not move.
- The body's [MCP] stdin clause (`.andromeda/architecture.md:43`): its behaviour stands; check only that no exit it implies disagrees with exit 1 and both streams empty.
- res-env-vars, `MCP_SERVE_TEST_CHILD` and `serve_child_entry`: unmoved, and 11 is now stated as that unit child's own code, not the process's.
- occupied-resources for `mcp`: the plan expected an exit 11 correction there, and the `exit-eleven` sweep finds 0 hits in `.andromeda/architecture-detail/occupied-resources.md`; flag only a site that states that exit in other words.
- conv-exit-codes, `14` `unknown` refusal: re-read, unmoved; a code word outside the closed list stays visible and exits 14 by `reason_of`.
- res-filesystem (`.andromeda/architecture-detail/occupied-resources.md:205`): extended by the after-every-test replacement.
- The `ci` kill-line filter count: 22 to 24 `binary(…)` terms in `.config/nextest.toml`; `.andromeda/architecture.md:385` states the class and no count, no change, but any standing count or list of those terms moves.
- crosscut-bounded-readers (`.andromeda/architecture-detail/cross-cutting-patterns.md:27`) and the body (`.andromeda/architecture.md:422`): the binary is truly named, no change; five `BOUNDARIES` rows' witnesses renamed, so any standing witness name moves.
- The new cap-driving unit cases in `src/cmd/hook/statusline.rs`, `src/run/version_gate.rs`, `src/run/budget.rs`, `src/run/dialog.rs` and `src/run/env.rs` are test additions only: any test inventory of those readers moves, no product rule of theirs does.
- res-repository, the `schemas/diag-line.v1.json` entry (`.andromeda/architecture.md:361`): the schema's `channel-response` `detail` is closed; check any wording on what that schema holds.
- `resolve_program` at `.andromeda/architecture-detail/standard-contracts.md:93` and `:133`: truly named, no change, though it now returns `Result<PathBuf, AgentError>`; check any wording on its return or on `Refusal::NotFound`.
- The body's [CI/CD] `batch-script-child` refusal (`.andromeda/architecture.md:75`) and the `claude` not on `PATH` case at `.andromeda/architecture.md:228`: outputs unchanged, the Rust type behind them moved to `AgentError::BatchScriptChild` and `AgentError::ProgramNotFound`.
- dec-module-boundaries and the infra-crate-dependency-direction contract: `viola-mcp` and the root bin read `sanitised_fault` and `is_code_word`; re-read, no manifest gains an edge.
- `fault_detail`: any standing mention of either private copy is stale.
- Test inventories: `[[test]]` tables for `channel_error_bodies` and `cli_error_bodies` in `Cargo.toml`; any standing count or list of contract or integration binaries moves through them.
- Line citations into the reached files, chiefly `crates/viola-core/src/lib.rs`, `crates/viola-channel/src/lib.rs`, `crates/viola-channel/src/client.rs`, `crates/viola-channel/src/server.rs`, `src/cmd/client.rs` and `crates/viola-mcp/src/result.rs`: added lines shift any later cited line.

## New facts
- F1 · cross-cutting-patterns · Sanitised error surfaces: CLI `--json` documents, MCP error results and channel error data carry codes and fixed messages only, no path and no upstream text; a peer's `message`, `data`, `refusal` or `detail` reaches no document or text, and a code word outside the closed list stays visible by design.
- F2 · conventions · `viola_core::is_code_word(&str) -> bool` defines a code word: 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first; its callers are `reply_of` and `of_reply`.
- F3 · conventions · A peer's reply is screened by `reply_of` and `of_reply`: a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads null, and a non-string `detail` is still no known reply.
- F4 · standard-contracts · `viola_channel::sanitised_fault(&Value) -> Option<Value>` is the one fault reader of `reply_of` and `of_reply`: `None` without an integer `code`; each of the five codes reads its fixed text; `-32602` keeps `supported` and `wrapper` only when `data.supported` is an integer and `data.wrapper` is version text (1 to 64 ASCII letters, digits, `.`, `-` or `+`), keeps `data.reason` only when exactly `release-from-driver`, and otherwise reads `invalid params` with `data` null; every other known code reads `data` null.
- F5 · standard-contracts · A fault code outside the five reads `{"code": <code>, "message": "wrapper fault", "data": null}`: the literal `wrapper fault` is a new fixed output word on `--json` and MCP, and the human line `error: wrapper fault  <code>` is unchanged.
- F6 · standard-contracts · `viola_channel::ProtocolError::cause(self) -> Option<&'static str>` names the three `-32602` faults `unsupported-version`, `invalid-params` and `release-from-driver`, and is `None` for the other four.
- F7 · occupied-resources · `channel-response`'s `detail` in `schemas/diag-line.v1.json` is the closed enum `unsupported-version`, `invalid-params`, `release-from-driver`, written on both the server's and the client's line at WARN by `ResponseLine`'s `Drop`, the client's `fault_of(&Value)` reading `data` for `-32602`, and absent on every other `channel-response` line, holding no param value or name.
- F8 · established-decisions · `viola_agent_claude::Refusal` was folded, not ratified, `StampError` the precedent: `AgentError` gains `BatchScriptChild` and `ProgramNotFound`, messages and outputs unchanged, and `resolve_program` returns `Result<PathBuf, AgentError>` in the crate and in `src/run/mod.rs`.
- F9 · established-decisions · The product sources hold 9 `thiserror::Error` derives: `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError`.
- F10 · established-decisions · `viola mcp` on a line past `MAX_FRAME` before `initialize` exits 1 with both streams empty and one `parse-rejected{mcp-stdin, oversize, count:1}` line; `src/main.rs` `exit_code` gives `Role::Mcp` with `Failed` exit 1, and 11 is only the unit child's own code.
- F11 · cross-cutting-patterns · `tests/contract_error_bodies.rs` (carrying `andromeda:walks-tree`) enumerates every error body in its tables `CLI_DOCUMENTS`, `MCP_RESULTS`, `CHANNEL_FAULTS`, `NOT_ERROR_BODIES` and `PLUMBING`, each body naming a writer and a witness that stand, censuses the product's error writers, and holds that `thiserror` messages interpolate nothing.
- F12 · cross-cutting-patterns · The three surface witnesses are `cli_error_documents_hold_no_path_or_upstream_text` in `tests/cli_error_bodies.rs`, `channel_error_bodies_hold_no_path_or_upstream_text` in `tests/channel_error_bodies.rs` and `mcp_tools_error_results_hold_no_path_or_upstream_text` in `crates/viola-e2e/tests/mcp_tools.rs`; the two root binaries have `required-features = ["fake-agent"]` and sit in the `ci` profile's kill-line filter.
- F13 · cross-cutting-patterns · No `-32603` is driven in a scanned home and no failpoint or counted panic was added for it; the unit layer holds it through `answer_a_panicking_dispatch_is_internal_error_and_closes` and `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`.
- F14 · occupied-resources · `tests/support/events.rs` holds `error_body_hit` and `assert_error_body_clean`, classes `absolute-path`, `caused-by`, `serde-path` and `planted`, reporting place and class, never bytes; no after-every-test helper exists, the three witnesses, the enumeration and the census of writers replacing it.
- F15 · occupied-resources · `v1-47`'s after every end-to-end test is concretized to after each job's end-to-end run, on `AGENT_RUN_KEEP_HOMES` `"1"` and `if: always()` in `.github/workflows/ci.yml`, the `viola-chaos-*` home out; accepted by the delegate overseer's word, provisional.

## Untouched
- [PTY] and the ConPTY sideload (`crates/viola-pty`, `src/conpty.rs`): no error type or reader there moved; `PtyError` stays the ratified exception.
- dec-database-state-store and dec-snapshot-writer: `viola-state` was not edited; `StampsError`, `StampsReadError` and `PinError` are only named as open divergences, their code the same.
- dec-session-liveness and contract-session-liveness: no heartbeat, pid or start-time rule moved.
- contract-list-payload and the `claude agents --json` read: `src/run/agents.rs` reads `resolve_program` through `let Ok(..) else`, unchanged, and no `list` field moved.
- `src/cmd/revive.rs`'s private `Refusal`: another type, it stays; a bare mention of `Refusal` there or of `RefusalReason` elsewhere is not the retired enum.
- The colour names `NO_COLOR`, `TERM` and `COLORTERM` in res-env-vars, and the human output in `src/human/style.rs` and `src/human/mirror.rs`: unchanged, no SGR or human line moved.
- The test seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC` in res-env-vars: no seam or failpoint was added.
- `schemas/diag-detail.v1.json`, `schemas/config.v1.json` and the fixture schemas: none of these moved.
- `fuzz/` and its targets and corpus: no target or seed was added.
- res-repository's harness outputs (`target/agent-run/`, `target/run-archive/`, `target/secret-scan/`): no harness verb, CI step or status shape moved, and `.github/workflows/ci.yml` was only read.
- The `viola answer` and `viola send` exit-2 stderr lines (`src/cmd/answer.rs`, `src/cmd/send.rs`): the `answer-oversize` correction's sites are in test-plan and integration-test-strategy, none in architecture.
- The `mcp` role's `process-exit` correction: the `mcp-exit-line` sites are in obs-plan and its detail, none in architecture.
