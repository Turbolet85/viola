## 2026-10-31-sanitised-error-surfaces — `unknown`'s detail a code word or null
**Section:** §Conventions, Error handling schema (architecture.md: the `RefusalReason` line; conventions.md: `conv-refusal-reason`)
**Change:**
- `unknown`'s `detail`: was any value; now a code word (`viola_core::is_code_word`) or `null`.
- The reader's rule: was an unrecognised `detail` opaque text; now an unrecognised code-word `detail` opaque text, never an error. The `RefusalReason` line reads each `detail` from a closed set per reason (`unknown`'s a code word or null), an unrecognised code word opaque text to a reader (was an unknown one).
- New, a code word: `viola_core::is_code_word(&str) -> bool`, 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first; `reply_of` and `of_reply` call it.
- New, a peer's reply: `reply_of` and `of_reply` read a `refusal` that is no code word as `unknown`, a string `detail` that is no code word as `null`, and a non-string `detail` as no known reply.
**Why:** a peer's `refusal` or `detail` reaches no document or text but as a closed value. A code word outside the closed list stays visible by design, its exit `unknown`'s 14. The new reply rule stood against the old reader's rule as a contradiction; the overseer, as the founder's delegate, answered apply, PROVISIONAL.
**Kept:** a reader still treats an unrecognised code word as opaque text and never as an error.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — `wrapper-fault`'s fields closed through `sanitised_fault`
**Section:** §Conventions, MCP (conventions.md: `conv-mcp-results`) · §Standard Contracts, CLI `--json` output (architecture.md; standard-contracts.md: `contract-cli-json-output`)
**Change:**
- `wrapper-fault`'s `detail`: was the JSON-RPC fault's `{code,message,data}`, unqualified; now that object as `viola_channel::sanitised_fault` reads it, a fixed `message` and `data` a closed object or null, on the MCP result and the exit-20 CLI document.
- New, `viola_channel::sanitised_fault(&Value) -> Option<Value>`, the one fault reader, of `reply_of` and `of_reply`: `None` without an integer `code`; for each of the five codes `message` is the fault's fixed text; `-32602` with an integer `data.supported` and a version-text `data.wrapper` (1 to 64 ASCII letters, digits, `.`, `-` or `+`) reads the unsupported-version body with those two values, `data.reason` exactly `release-from-driver` reads that body, and any other `-32602` reads `invalid params` with `data` null; every other known code reads `data` null.
- New, a fault code outside the five: `{"code": <code>, "message": "wrapper fault", "data": null}`. The literal `wrapper fault` is a fixed output word on `--json` and MCP; the human line reads `error: wrapper fault  <code>`.
**Why:** a peer's `message` or `data` reaches no document or text but as a closed value. `sanitised_fault` replaces the two private `fault_detail` copies, both gone. The seven wire bodies are unchanged and `v` stays 1.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the sanitised error surfaces pattern and its contract binary
**Section:** §Cross-cutting Patterns (cross-cutting-patterns.md: new group `crosscut-sanitised-error-surfaces`; architecture.md: the subjects list)
**Change:**
- New, the rule: CLI `--json` documents, MCP error results and channel error data carry codes and fixed messages only, no path and no upstream text; a peer's `message`, `data`, `refusal` or `detail` reaches no document or text but as a closed value; a code word outside the closed list stays visible by design.
- New, its holder: the E2E contract binary `tests/contract_error_bodies.rs` (run under `run --e2e`, carrying `andromeda:walks-tree`). Tables `CLI_DOCUMENTS`, `MCP_RESULTS` and `CHANNEL_FAULTS` enumerate the error bodies, and `error_bodies_each_name_a_writer_and_a_witness_that_stand` checks each body's writer and witness; `NOT_ERROR_BODIES` names what is no error body, and `PLUMBING` the census lines that build or route a document a row names, so a row stays one body. The census `product_error_writers_are_each_named` covers the product's error writers; `thiserror_messages_interpolate_nothing` holds that a `thiserror` message interpolates nothing.
- New, three surface witnesses: `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`), `channel_error_bodies_hold_no_path_or_upstream_text` (`tests/channel_error_bodies.rs`), `mcp_tools_error_results_hold_no_path_or_upstream_text` (`crates/viola-e2e/tests/mcp_tools.rs`). The two root binaries' `[[test]]` tables require `fake-agent`, and both sit in the `ci` profile's kill-line filter.
- New, the limit: no `-32603` is driven in a scanned home and no failpoint or counted panic exists for it; the unit layer holds it, through `answer_a_panicking_dispatch_is_internal_error_and_closes` and `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`.
- The subjects list gains `crosscut-sanitised-error-surfaces`.
**Why:** this chunk put codes and fixed messages only on the three surfaces, and no master named its three new binaries.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — `viola mcp`'s exit on a stdin cut before `initialize`
**Section:** §Established Decisions, [MCP] (established-decisions.md: `dec-mcp`) · §Occupied Resources, Subcommands (occupied-resources.md: `res-binaries`)
**Change:**
- `ServeError::Initialize` at a cut before `initialize`: was exit 11; now exit 1.
- `mcp`'s subcommand line: was 11 before `initialize`; now 1 before `initialize`, both streams empty.
- New: on a line past `MAX_FRAME` before `initialize`, `viola mcp` exits 1 with both streams empty and one `parse-rejected{mcp-stdin, oversize, count:1}` line (`mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`); `src/main.rs`'s `exit_code` gives `Role::Mcp` with `Failed` exit 1, and 11 is only the unit child's own code (`serve_child_entry`).
**Why:** the stated exit 11 was disproved by measurement; the process exit stays 1 and the masters move. The new line stood against the old exit as a contradiction; the overseer, as the founder's delegate, answered apply, PROVISIONAL.
**Kept:** exit 0 once initialized; `budget-paused`'s exit 11 among the CLI exit codes is another code and did not move.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the enum census: `Refusal` folded, `CoreError` out, five open divergences
**Section:** §Established Decisions, [Error Handling] (architecture.md; established-decisions.md: `dec-error-handling`) · §Conventions, Rust error types · §Inherited Defaults, Errors (architecture.md) · registry contract `crate-dependency-direction.md`
**Change:**
- Open divergences at [Error Handling] and Rust error types: was three, `Refusal`, `ServeError`, `StampsError`; now two, `ServeError` and `StampsError`, with `PinError`, `StampsReadError` and `Refused` the three the census adds. Inherited Defaults, Errors: was `Refusal` and `ServeError`; now the same two and three.
- `dec-error-handling`: the owner of all five is a CARRY on the working-route entry "Version done-check" (PROVISIONAL).
- Rust error types: `CoreError` off the `<Crate>Error` list, which reads `PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`.
- New: `AgentError` holds `resolve_program`'s two refusals, `BatchScriptChild` and `ProgramNotFound`, each with a fixed message; `resolve_program` returns `Result<PathBuf, AgentError>`, in the crate and in the root wrapper `src/run/mod.rs`. The contract's `viola-agent-claude` list drops `Refusal` and states that return.
- New: the product sources hold 9 `thiserror::Error` derives (`AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError`), by `command grep -rn "thiserror::Error" src crates --include=*.rs`; Rust error types states the nine and that command.
**Why:** `Refusal` was folded, not ratified, `StampError` the precedent, its messages and outputs unchanged. `CoreError` never existed in the code: its grep over `crates` and `src` reads 0 hits. Research found the masters' list stale in both directions, and the derives read 9, from 10 once `Refusal` is folded; the fix was a census, not a wider fold. The owner is the word of the overseer as the founder's delegate, 2026-10-31, PROVISIONAL, owed the founder's own word; the route edit itself is not in this pass. The two new bullets stood against the old line as contradictions; the same delegate answered apply, PROVISIONAL.
**Kept:** one enum per workspace crate stays the rule, with `PtyError`'s ratified exception; each divergence's fold or ratification stays open.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — `mcp.ndjson` holds no self `process-exit`
**Section:** §Occupied Resources, `diagnostics/` (occupied-resources.md: `res-filesystem`)
**Change:**
- `mcp.ndjson`'s list: was `process-exit` on stdin EOF or at that cut; now no self `process-exit`, none on stdin EOF or at that cut (`command grep -n 'ObsEvent::ProcessExit' -r src crates --include=*.rs`).
- The line opening "with no name, or on a refused scan" stands as a continuation line (was its own `- ` bullet).
**Why:** the claim that `mcp` writes a `process-exit` was disproved by measurement: that grep shows no self `process-exit` written on the `mcp` role.
**Kept:** none when Claude Code ends it by a signal, as measured live on claude 2.1.287 at chunk 2026-10-17-mcp-server-for-drivers.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — `channel-response`'s `detail` closed to three words
**Section:** §Occupied Resources, `diagnostics/` (occupied-resources.md: `res-filesystem`) · §Standard Contracts, channel methods (standard-contracts.md: `contract-channel-methods`)
**Change:**
- New: `channel-response`'s `detail` in `schemas/diag-line.v1.json` is the closed enum `unsupported-version` · `invalid-params` · `release-from-driver`, written at WARN on both the server's and the client's line by `ResponseLine`'s `Drop` (`ProtocolError::cause`), the client's `fault_of(&Value)` reading `data` for `-32602`. It is absent on every other `channel-response` line and holds no param value or name.
- New: `viola_channel::ProtocolError::cause(self) -> Option<&'static str>` reads those three words for the three `-32602` faults and `None` for the other four.
**Why:** the schema's `detail` closed from any string to these three words. The wire form does not change, `v` stays 1, the seven `ProtocolError` bodies are unchanged and the level is unchanged.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the error-body check, and no after-every-test helper
**Section:** §Occupied Resources, Test-only (occupied-resources.md: `res-filesystem`)
**Change:**
- New: `tests/support/events.rs` holds the error-body check, `error_body_hit` and `assert_error_body_clean`, classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes.
- New: no after-every-test helper is built; the three surface witnesses, the enumeration and the census of writers replace it, reading this check.
**Why:** the after-every-test check of test-plan §6 was measured absent before this chunk and was not built; the witnesses, the enumeration and the census replace it.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — `v1-47`'s reading over the homes CI keeps
**Section:** §Occupied Resources, Repository (occupied-resources.md: `res-repository`)
**Change:**
- New: `v1-47`'s after every end-to-end test is after each job's end-to-end run: `AGENT_RUN_KEEP_HOMES` `"1"` on the job and `if: always()` in `.github/workflows/ci.yml`, the `viola-chaos-*` home out. PROVISIONAL, owed the founder's own word.
**Why:** the concretization is accepted by the word of the overseer as the founder's delegate, 2026-10-31, PROVISIONAL, owed the founder's own word, and measured in the workflow. The `viola-chaos-*` home is out by the founder's ruling of 2026-10-04.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/
