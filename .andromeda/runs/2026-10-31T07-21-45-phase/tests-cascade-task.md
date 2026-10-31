# Task — tests · 2026-10-31-sanitised-error-surfaces

## The chunk
Makes every error a `viola` process gives to a machine reader a code and a fixed message and nothing else, on CLI `--json`, MCP `isError` results and channel JSON-RPC errors, with every product `thiserror` `Display` that can reach them a fixed message.
Proves it with one witness per surface, driving the failure with a path, an upstream text and a planted stripped `CLAUDE*` value in play, and with an enumeration of the three surfaces' error bodies, so a new body carrying free text is visible.
Folds in two carries: the `viola-agent-claude` `Refusal` enum folded into `AgentError` or ratified as an exception, and the `-32602` causes told apart on the `channel-response` line so `viola.compat.v_rejected` stops over-counting.
Owes the missing tests of the bounded-inputs chunk: a `viola mcp` stdin line past `MAX_FRAME` before `initialize` completes, and a cap-driving case for each of five `BOUNDARIES` rows.
Tallies the Path 1 CI timeout watch on this chunk's CI reads, and P3 reads whether the remaining legs of the candidate claim `v1-47` stand at HEAD.

## Reached files
- crates/viola-agent-claude/src/lib.rs · edits `Refusal`, `AgentError`
- crates/viola-channel/src/lib.rs · edits `ProtocolError::InvalidParams` and the other `-32602` causes
- crates/viola-channel/src/client.rs · edits `fault_of`
- tests/contract_bounded_readers.rs · edits five `BOUNDARIES` rows (the status-line command's stdout, the child pipe drain, the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep`)
- .andromeda/obs-plan-detail/metric-coverage.md · re-reads the `viola.compat.v_rejected` row

## Needs
- CLI `--json` error documents of every verb, the `{"v":1,"error":…,"detail":…}` form: a closed code, a code-shaped or null `detail`, no free text for a failure the verb cannot name; this reaches the `--json` document per exit code in root `tests/cli_*.rs`, the exit-code contract and the `RefusalReason` `unknown` fallback at exit 14.
- MCP `isError` results of the tools `send · wait · last · answer · list`: `refusal` / `detail`, `instance-unreachable`, `wrapper-fault` or `invalid-params` only, text `refused: <code>`, a handler fault exactly `"internal error"` / `data: null`.
- Assertions already pinning those MCP bodies stay true: the schema-violation cases (`invalid-params` with its field, no frame written), `instance-unreachable` on a killed wrapper and `refused: unverified-cli`.
- Channel JSON-RPC errors a wrapper answers: a fixed message and `error.data` a code or null, across `-32600` on an oversize frame, `-32602` with `data.supported` / `wrapper`, `release-from-driver` and `InvalidParams` (`data: null`), and `-32603` exactly `"internal error"` / `data: null`; this reaches the JSON-RPC error mapping unit cases and the schema-validated error shapes of the channel suites.
- Product `thiserror` `Display` impls that can reach a surface: each a fixed message holding no path and no payload.
- The error sanitization and secret logging trigger and its §6 suite: no absolute path, serde path or value, anyhow chain or `Caused by`, or tool `input` in any error payload; a hit reported by place and class, never the matched bytes.
- The planted stripped value, through the R8 strip and scenario E2: it must sit under a name the strip removes, because persistent-environment names, `claude_env_keep` names and the 11-name identity floor are kept, and a value under a kept name proves nothing.
- The enumeration of the three surfaces' error bodies, one a reader can check and a new free-text body trips; the bounded-reader table and census of the contract suite is the plan's nearest form of such an enumeration.
- Cross-surface parity: for the same refusal the MCP `structured_content` `{refusal, detail}` equals the CLI `--json` `{refusal, detail}`.
- The recorded MCP exchange `Mcp.<variant>.json` and its replay, where a changed error body meets a recording; a recording changes only through `viola-mcp-tap` with its scrub and refusal.
- The `Refusal` carry: `BatchScriptChild`, `NotFound` and the `resolve_program` refusal keep their behaviour whichever way the question is answered, reaching the `.cmd` / `.bat` child exit 1 cases and the npm-shim resolution cases.
- The `-32602` carry: the `channel-response` line carrying only `error_code`, `fault_of` labelling every `-32602` unsupported-version, the `viola.compat.v_rejected` metric, the higher-`v` rejection cases; a changed line is read against obs-plan and its diagnostics line schema.
- PREREQ, MCP stdin before `initialize`: a line past `MAX_FRAME` landing before `initialize` completes ends `serve` with `ServeError::Initialize` (exit 11), as a closed stdin before `initialize` does; this is the §4 viola-mcp clause naming this entry owner, next to the stdin line cap's edge cases and the session-end behaviour after `initialize`.
- PREREQ, the five `BOUNDARIES` rows: the §6 Contract suite's owed clause naming this entry owner; each row gets a witness that drives past its cap, and `boundaries_each_name_a_bounded_read_that_stands`, `product_readers_are_each_bounded_or_named` and the census self-check keep holding over the edited rows.
- The status-line row's witness, through `viola hook statusline`: the pass-through runs its command on Unix only, the Windows arm runs none, and `statusline_command` never runs when strict-modes fails; the status line source is written by the test.
- The child pipe drain row's witness, shared by `claude --version` and `claude agents --json`: `viola-fake-agent` as `claude` on a test-scoped `PATH`, answering from files beside its exe with no env var; oversize `claude agents --json` reads `unknown`; the harness's `agents_mode` `oversize` is the existing form.
- The `config.json` rows' witnesses: the thresholds with `Percent` 0–100, the dialog hold with `DialogHold` 60–3600 s and `claude_env_keep`, a Unix-only name list; each row's case is an oversize file, and what the reader falls back to is read from the product.
- Forcing a handler fault for `-32603` / `"internal error"`: only process and file faults or existing mockall seams, no failpoint crate; a test-only seam stays behind the `fake-agent` feature and out of the release build.
- The per-surface witness drivers: `viola_mcp::Tools` against `TestWrapper` in `crates/viola-mcp/tests/handlers.rs`, on the Tokio client; an rmcp client over a `viola mcp` child the test owns for the stdio path; a raw interprocess stream with jsonschema for the channel, its endpoint read only from `snapshot.json`; `assert_cmd` for the CLI.
- The candidate claim `verification-matrix.json#v1-47`: content and drift reports reaching only owner-readable detail files (`instances/*/diagnostics/detail-*.ndjson`, `diagnostics/` 0600 on Unix, `diag-detail.v1.json`) and the secret scan after every E2E test; a leg this chunk cannot prove is no claim.
- The Path 1 watch: `path1_run_start_one_home_in_order` against the ci profile's 20 s kill line, which does not move; a recurrence reopens the owner question for the founder, and 3 green runs retire it.
- The bound on all of it: no testing past what the plan lists; a witness traces to a clause of the plan.

## Standing conventions
- S1 · A new root or `viola-e2e` test file takes its `<surface>_<topic>.rs` prefix, a test function `<subject>_<condition>_<expected>` in snake_case · §2 Test directory + naming conventions; test-strategy.md
- S2 · A test owns a fresh home at a not-yet-existing path under `target/e2e-home`, made by viola or `create_private_dir`, its started wrappers in a `Drop` guard, nothing shared between tests · §5 Setup / teardown lifecycle, §7 Seed strategies; integration-test-strategy.md
- S3 · A child's env is set by `Command::env`, never `set_var`; `.env_clear()` re-adds `LLVM_PROFILE_FILE`; a test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` · §5 Cross-module patterns, §11 Integration
- S4 · No sleep or test-owned timer for synchronisation, and verdicts come from exit codes, `events.ndjson`, channel payloads and receipts, never the rendered child screen · §2 Agent-runnable invariants, §11 E2E and Universal
- S5 · Oracles are literals in the test, never the product's own tables; codes are compared by serde kebab-case value and integer exit code, never by `Debug` · §11 Unit
- S6 · A case sits at the lowest layer that can hold it, and §6 asserts only cross-process behaviour · §11 Test Strategy, §2 Test pyramid
- S7 · Table-driven cases are rstest `#[case]` with readable labels; `viola-e2e` uses a labelled case table and `viola-core` a literal table, adding no manifest edge · §4 Conventions; unit-test-strategy.md
- S8 · Root tests share `tests/support/` helpers uncopied, and a reader of a live-appended file goes through `tests/support/ndjson.rs` · §2 Directory pattern; test-strategy.md
- S9 · `ledger/stamps.json`, `snapshot.json` and `budget.json` are never hand-written, and a reader's unit case alters only the writer's own bytes · §7 Seed strategies, §11 Test Data; test-data-fixtures.md
- S10 · A new or changed diagnostics line validates against obs-plan's schema, and a failing check prints place and keyword, never content · §6 Schema conformance; e2e-test-strategy.md
- S11 · A new test runs on all three CI OSes, an OS branch being verified on its own runner, never by a Linux-only local run · §1 multi-os-compat trigger, §11 Test Strategy; ci-integration.md
- S12 · New code stays inside the per-OS coverage gates, `retries = 0`, and no `#[ignore]` as a parking place · §10 Coverage thresholds and Zero-flakiness budget; quality-gates-coverage-targets.md

## Not needed
- The Exit-cause matrix suite (`cross_exit_causes.rs`), its cause hints and `revive`'s start-refusal documents: owned by the route entry "Exit-cause code catalogue".
- Security control negatives for the socket directory, peer credentials, the Windows pipe DACL and `--home` DACL, strict-modes refusals, and the macOS server-identity case: owned by the route entry "Unix endpoint and home hardening".
- Problem Details, the GUI's token, cookie, launch URL and a11y artifact clauses, and scenario E4: carried to 0.2.0, with no frontend in this version.
- Windows-only live readings, including the Windows byte reading of the forced vt100 panic: owned by the route entry "Windows-only live measurements".
- The flows of Paths 2 and 6 and scenarios E3 and E5, and the dialog decision bodies and their insta snapshots of Path 4: this chunk changes no send confirmation, budget governor, link or replay behaviour and no decision body.
- Human-mode output: the `list` row integrity suite, trycmd layouts, the `viola list` board, the `viola --help` literal and C0/C1 escaping; a human reader is not one of the three surfaces.
- The mutation gate, `windows-mutants.yml` and the nightly fuzz job: mutation is judged at the epoch boundary and no epoch boundary falls inside this run.
- Property cases over `validate_paste_text`, the vt100 feed and `prompt-submitted` normalisation: this chunk's work does not touch those parsers.
- The chaos cases for torn appends, a SIGSTOPped wrapper, a no-EOF child and snapshot replay: none is an error body on the three surfaces.
- Recorded hook payload sets, ledger probes, stamp rows, `agent-run run --local-live` and the real `claude` CLI: this chunk records no hook fixture and adds no ledger row.
- The harness's five commands, its session record and its own JSON documents: harness output is no product error surface, and this chunk's witnesses run under the existing suites.
