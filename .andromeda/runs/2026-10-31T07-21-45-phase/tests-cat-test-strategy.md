# tests · Test Strategy — category read

## Carried
- (.andromeda/test-plan.md:24) The chunk's test breadth must meet the plan's Comprehensive tier.
  > **Test tier:** `Comprehensive (2)`
- (.andromeda/test-plan-detail/test-strategy.md:73) The chunk's proof of error sanitization must sit at the integration and E2E layers, in the suite and log clause this trigger row names.
  > | error sanitization / secret logging | integration + E2E | §6 Error sanitization and secret scan; §3 logs |
- (.andromeda/test-plan-detail/test-strategy.md:14) Each surface witness that crosses a contract boundary must drive a real endpoint, a real home and the real binary, never a stand-in.
  > Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.
- (.andromeda/test-plan-detail/test-strategy.md:10) The fixed-message and code-mapping cases that need no process must be inline unit tests, table-driven where they form a matrix.
  > Inline `#[cfg(test)] mod tests` in all 6 crates plus the root bin. rstest 0.27.0 `#[case]` tables cover the refusal-order matrices, the exit-code table (0/1/2/10–14/20/21) and the per-`<cli-version>` fixture replay.
- (.andromeda/test-plan-detail/test-strategy.md:11) The chunk's gate readings must come from nextest, not from a plain `cargo test`.
  > nextest is the runner; a plain `cargo test` run (`cargo test --bin viola`) is not supported
- (.andromeda/test-plan-detail/test-strategy.md:29) A remove-the-guard witness on any control this chunk adds must be bounded when its neutralised guard lets a product process start.
  > Beside it stands the per-chunk remove-the-guard rule (`.claude/rules/testing.md`): a control whose neutralised guard lets a product process start needs a bounded witness; the G4 / G10 witnesses spawn through `tests/support/cli.rs` `Running`, bounded by `WITHIN`, since nextest's default profile has no kill line (an unbounded witness hung 27 minutes, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md`).
- (.andromeda/test-plan-detail/test-strategy.md:29) The chunk's gate set must hold no mutation run; this carve-out leaves mutation to the epoch-boundary audit.
  > No per-chunk, pre-push or CI mutation gate.
- (.andromeda/test-plan-detail/test-strategy.md:42) Any new root witness for the CLI or channel surface must take its child runner, event wait and home widening from the shared helpers and define no copy of them.
  > No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct. `ndjson.rs` is the one complete-lines reader: every reader of a live-appended file parses only newline-terminated lines, because a Linux reader can observe an append part-way.
- (.andromeda/test-plan-detail/test-strategy.md:43) Any witness that needs the rmcp client or spans the surfaces must live in the test-only crate under its naming form.
  > **Tokio-client E2E suites and the harness binary `viola-harness`:** `crates/viola-e2e/` (publish = false). This covers the rmcp client, cross-surface parity and critical-path scenarios, as `tests/{path<N>,mcp,cross}_<topic>.rs`
- (.andromeda/test-plan.md:126) Every witness run with a planted stripped `CLAUDE*` value must leave that value out of every process log.
  > Logs must never contain the `Cookie` header or stripped `CLAUDE*` values (security excerpt, Error-sanitization).
- (.andromeda/test-plan.md:166) An MCP refusal's text must keep the `refused: <code>` form this path asserts.
  > `answer` exits 12. MCP `answer` gives `isError:true` `"refused: unverified-cli"`.
- (.andromeda/test-plan.md:182) The higher-`v` rejection is a required negative whose `error.data` holds this object, so the chunk's rule of a code or null for channel error data must be reconciled with it, and the error-body enumeration must name it either way.
  > higher `params.v` → `-32602` with `data.supported`/`wrapper`
- (.andromeda/test-plan.md:196) The MCP witness must assert code-only `isError` payloads, the rule the chunk carries across all five tools.
  > `isError` payloads carry codes only.
- (.andromeda/test-plan.md:269) This is the required test type for the chunk's whole property; the three surfaces' witnesses must assert each clause, and the planted values must prove the log grep.
  > assertions that CLI `--json`, MCP `isError` and channel `error.data` contain no absolute paths, serde paths or values, anyhow chains or tool `input`. `-32603` must be exactly `"internal error"` / `data:null`. A grep of all logs and `diagnostics/` must show no `Cookie` or stripped `CLAUDE*` values. `diagnostics/` must be 0600.
- (.andromeda/test-plan.md:309) The witnesses must not synchronise on sleeps or pass through a retry.
  > no nextest retries (`retries = 0`) and no `std::thread::sleep` / `tokio::time::sleep` used for synchronisation
- (.andromeda/test-plan.md:324) A new integration binary must carry a surface prefix so the layer filtersets select it.
  > Rust integration binaries use a `<surface>_<topic>.rs` prefix so nextest filtersets can select layers: `cli_`, `hook_`, `tui_`, `channel_`, `chaos_`, `contract_` in root; `path_`, `mcp_`, `cross_` in `viola-e2e`.
- (.andromeda/test-plan.md:327) New test functions must follow the plan's naming pattern.
  > General pattern: snake_case `<subject>_<condition>_<expected>`, for example `send_with_esc_refuses_control_character` or `hook_oversize_stdin_exits_zero_silent`.
- (.andromeda/test-plan.md:387) The `viola-mcp` unit level must hold the code-only payload on each re-validation refusal.
  > a non-`ViolaName` `target`, a non-`u64` `dialog_id`, an open-string `behavior` and control chars in `text` all give `isError:true` with code-only payloads
- (.andromeda/test-plan.md:390) An owed item naming this chunk its owner: the chunk must add a test that drives this clause.
  > not driven by any test: a stdin line past `MAX_FRAME` landing before `initialize` completes ends `serve` with `ServeError::Initialize` (exit 11), as a closed stdin before `initialize` does; owner: the route entry "Sanitised error surfaces", as a PREREQ
- (.andromeda/test-plan.md:433) A witness home must sit under `target/e2e-home`, fresh per test and created by viola, so the secret scan reads it, and started wrappers must sit in a `Drop` guard.
  > a fresh tempfile home per test, at a not-yet-existing path under `target/e2e-home` that viola creates, so obs-plan's CI gates, the secret scan and the upload cover integration runs; started wrappers sit in a `Drop` guard that kills and waits, and nothing is shared between tests.
- (.andromeda/test-plan.md:441) The channel witness must validate error shapes against the schemas within this suite's existing coverage.
  > **Wrapper channel:** every method through the crate and root channel suites, with schema-validated result, refusal and error shapes, the `cursor` equal to the log length at acceptance, `-32602` on a higher `v`, `-32600` and a close on an oversize frame, server verification, and wrapper-side paste validation in which `control-character` wins over every other refusal.
- (.andromeda/test-plan.md:452) The CLI witness must extend this suite's per-exit-code `--json` document coverage.
  > **CLI:** root `tests/cli_*.rs` cover the `--json` document per exit code,
- (.andromeda/test-plan.md:454) A witness that starts `viola` must drop these variables, and any path it searches an error body for must be the resolved home.
  > A test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`, and a path oracle compares `resolved(home)`, never `TestHome::path()`.
- (.andromeda/test-plan.md:456) MCP handler-fault and refusal cases at the integration level must go through this handle and this client.
  > **MCP:** adapter-level handler tests in `crates/viola-mcp/tests/handlers.rs` drive the transport-free `viola_mcp::Tools` handle (the same `Server::call` `serve` answers with) against a `viola_channel::Server` test wrapper (`TestWrapper`, its snapshot written through `viola_state::snapshot::write_snapshot`); the handlers under test use the Tokio client (`viola_channel::tokio_client::TokioClient`, `viola-channel`'s `tokio` feature), never the sync client (chunk 2026-10-17-mcp-server-for-drivers).
- (.andromeda/test-plan.md:477) A stdio MCP witness, including the pre-`initialize` PREREQ case if driven over stdio, must own its child this way.
  > an rmcp client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's `TokioChildProcess`
- (.andromeda/test-plan.md:603) This case form must hold under the chunk's code-only rule; the `invalid-params` body may name its field and nothing free-text, and the enumeration must name that field as a closed value.
  > nextest runs the MCP schema-violation cases (each `invalid-params` with its field, no frame written)
- (.andromeda/test-plan.md:605) The scan suite is the plan's standing form of the chunk's proof, including how a hit may be reported; a witness must never print the matched bytes.
  > after every E2E test, no absolute path, serde path or `Caused by` in any error payload, every `-32603` exactly `"internal error"` / `null`, no `Cookie` header or stripped-variable value in any log, `diagnostics/` files 0600 on Unix, and the fixed canary in no home-level diagnostics line; a hit is reported by place and class, never the matched bytes.
- (.andromeda/test-plan.md:616) A change to any refusal body must keep MCP and CLI equal for the same refusal.
  > for the same refusal, the MCP `structured_content` `{refusal, detail}` equals the CLI `--json` `{refusal, detail}`
- (.andromeda/test-plan.md:622) The PREREQ's cap-driving cases must update the witnesses of this table's rows and keep its checks passing, and this table is the plan's existing shape for an enumeration with a census.
  > `tests/contract_bounded_readers.rs` holds the bounded-reader enumeration: its `BOUNDARIES` table names each external reader's bounded read and a witness test, its `EXCEPTIONS` table the reads that take no external input, and a census over the product sources reads every reader bounded in-statement or named (`boundaries_each_name_a_bounded_read_that_stands`, `product_readers_are_each_bounded_or_named`, the census's self-check `census_finds_an_unbounded_read_and_passes_a_bounded_one`), beside `serde_json_unbounded_depth_is_banned_in_deny_toml`.
- (.andromeda/test-plan.md:623) An owed item naming this chunk its owner: each named row must gain a case that drives past its cap.
  > Owed: the enumeration's rows for the status-line command's stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` name a witness that reads the bounded read without driving past its cap, so their caps are held by the row's needle and the census alone; owner: the route entry "Sanitised error surfaces", as a PREREQ (a cap-driving case for each row).
- (.andromeda/test-plan.md:816) The per-OS coverage gates the chunk's code must clear.
  > | **Comprehensive (viola)** | **≥ 85%** | **≥ 80%** (enforced as region coverage `--fail-under-regions 80`) | **≥ 95%** |
- (.andromeda/test-plan.md:857) A flake in a new witness keeps the chunk red until its root cause is fixed within the chunk.
  > For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:874) Message-text and code-mapping cases must sit at the lowest layer that holds them, with E2E kept to cross-process behaviour.
  > NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:875) A claim across the three surfaces must have a driver step on each of them, not one surface read for the rest.
  > NEVER assert a multi-surface critical path on one surface only. Every surface a §6 scenario lists needs its own driver step and transition signal.
- (.andromeda/test-plan.md:879) OS-branch error paths the chunk touches must be read on their own CI runners before they are claimed.
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:887) The enumeration's expected codes and fixed messages must be literals in the test, never imported from the product.
  > NEVER import the product's own `IDENTITY_FLOOR`, ledger rows or refusal ordering as the test oracle. The expected values (the 11 identity-floor names, the refusal order, exit codes) are written as literals in the test.
- (.andromeda/test-plan.md:888) Witnesses must assert the wire value of a refusal and the integer exit code.
  > NEVER compare `RefusalReason` or exit codes by `Debug` string. Assert on the serde kebab-case value and the integer exit code.
- (.andromeda/test-plan.md:896) A planted `CLAUDE*` value must reach the child through `Command::env`.
  > NEVER use `std::env::set_var` in tests. It is `unsafe` in edition 2024 and races across threads; use `Command::env` per child.
- (.andromeda/test-plan.md:897) A witness that clears the environment to control the planted values must re-add the profile variable.
  > NEVER call `.env_clear()` without re-adding `LLVM_PROFILE_FILE`, because that run silently drops out of coverage.

## Bindings
- **Error sanitization suite ↔ security plan**: the required assertions take their source from the security excerpt's Error-sanitization expectations and Anti-Patterns (Secrets and logging), so the chunk's witnesses and that plan's rule must state one property (.andromeda/test-plan.md:268)
- **Secret scan ↔ obs-plan**: a witness's home must be under `target/e2e-home` for obs-plan's gates and secret scan to read it (.andromeda/test-plan.md:433), and every artifact leaves CI only behind obs-plan §9's scan (.andromeda/test-plan.md:777)
- **Higher-`v` `error.data` ↔ the scope's channel clause and the arch Wrapper channel contract**: the plan requires `data.supported`/`wrapper` on this `-32602` (.andromeda/test-plan.md:182) where the scope asks a code or null in `error.data`; the two must be reconciled before either is asserted
- **Exit-cause matrix ↔ route entry "Exit-cause code catalogue"**: the matrix's per-cause documents and hints stay with that entry, which the scope leaves out of this chunk, and this chunk's witnesses must not take them (.andromeda/test-plan.md:609)
- **Bounded-reader PREREQ ↔ route entry and body §4 / §6**: both owed lines name this entry as owner and close together with its cap-driving and pre-`initialize` cases (.andromeda/test-plan.md:390), (.andromeda/test-plan.md:623)
- **Trigger row ↔ body §3 logs and §6 suite**: the row names both homes of the sanitization proof, the log rule and the scan suite (.andromeda/test-plan-detail/test-strategy.md:73), (.andromeda/test-plan.md:126), (.andromeda/test-plan.md:605)
- **Coverage ↔ §10 harness and child processes**: a witness that clears its environment must keep `LLVM_PROFILE_FILE` or its children fall out of the §10 number (.andromeda/test-plan.md:823), (.andromeda/test-plan.md:897)
