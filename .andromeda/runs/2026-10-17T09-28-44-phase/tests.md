# tests extract

## Relevance
relevant: the chunk builds the plan's sixth surface, ipc-internal (MCP stdio). Tests for it are owed by the §4 viola-mcp unit cases, the §5 MCP adapter suite, and the MCP steps of §6 Paths 2–7, the Security sweep and Cross-surface parity.

## Constraints
- test-plan §1 Test Scope Summary (ipc-internal MCP surface) requires a language-native MCP stdio client. It spawns `viola mcp --home <tmp>`, speaks JSON-RPC over stdin/stdout and checks results against a JSON schema. The signals it names are:
  - `isError:false` with `structuredContent` equal to the `ok` payload
  - `isError:true` with `{refusal, detail}` and the text `"refused: <reason>"`
  - `isError:true` with `{"error":"instance-unreachable"|"wrapper-fault"}`
  - no `release` / `pause` / `link` / `unlink` tool

  The schemas are advisory, so negative inputs must break the advertised schema. The tier is `Comprehensive (2)`.
- test-plan §1 Test Scope Summary (security-vector-coverage (MCP)) requires schema-violating tool calls to be rejected by the handler, with `isError` payloads that carry codes only. The calls are a non-`ViolaName` `target`, a non-`u64` `dialog_id`, an open-string `behavior` and control characters in `text`.
- test-plan §4 Unit Test Strategy (viola-mcp) requires handler re-validation cases for those same four inputs, each giving `isError:true` with a code-only payload. It also requires that `tools/list` is exactly `send, wait, last, answer, list`.
- test-plan §5 Integration Test Strategy (MCP) puts adapter-level handler tests in `crates/viola-mcp/tests/`. They run against a test wrapper, with the sync channel client inside the handler under test. The full stdio path belongs to E2E. The same section (Module ↔ DB, Concurrent-append check) requires every line to parse as one JSON object when two `viola mcp` processes append to `mcp.ndjson` and to the shared `detail-mcp` file at once, with at least one detail line over 4 KiB. Its On-disk strict-modes bullet requires `mcp` to exit 1 on a strict-modes refusal.
- test-plan §6 E2E Test Strategy (drivers per surface) fixes the MCP E2E driver:
  - an rmcp 3.4.1 client, `TokioChildProcess` spawning `viola mcp --home <tmp>`, plus jsonschema
  - it lives in `crates/viola-e2e`, with dev-dependency features `client` and `transport-child-process` only

  Binaries there are named `tests/{path<N>,mcp,cross}_<topic>.rs` (per test-plan §2 Test Strategy, directory and naming conventions).
- test-plan §6 E2E Test Strategy (Paths 2–7) owes these MCP legs:
  - Path 2: `call_tool("send")` gives `is_error:false`, with a `structured_content` key set equal to the CLI's `{submitted_at, cursor}`.
  - Path 3: `last` equals the CLI's. A live `wait` wakes only on the `turn-ended` record with the CLI's `cursor`, never on the `activity` record. A `wait` on a dead instance gives `is_error:true` `{"error":"instance-unreachable"}`.
  - Path 4: `answer` gives `is_error:false`.
  - Path 5: the text is `refused: human-typing`, and neither the text nor `structured_content` contains `release`.
  - Path 6: the MCP form of the `budget-paused` refusal is owed by title to this entry.
  - Path 7: the text is `refused: unverified-cli`.

  §6's Cross-surface parity adds a requirement: for the same refusal, the MCP `{refusal, detail}` equals the CLI `--json` one. Path 3's MCP error shape omits the CLI's `"detail":null`. Whether the MCP error document carries `detail` is research's question.
- test-plan §8 Mocking & Stubbing Discipline requires `#[tokio::test(start_paused = true)]` only for a timer the MCP adapter itself owns. The `wait` `timeout_ms` deadline runs in the wrapper and is driven only by the injected `viola_core::Clock` in the §4 root-bin case. If the server bounds `wait` itself (the scope's "below the tool-call timeout"), that bound is an adapter-owned timer and is tested under a paused tokio clock. Whether the code already has such a bound is research's question.

## Patterns to follow
- Harness-session E2E in `viola-e2e`, as landed at `crates/viola-e2e/tests/path6_budget_harness.rs` `path6_two_instance_release_over_a_harness_session`: boot through the harness session and read verdicts from `--json` documents, `events.ndjson` and snapshots (per test-plan §6 E2E Test Strategy, Path 6 "As landed").
- Gated fake scripts order events deterministically: `fixtures/fake-scripts/path3.json` for `wait` / `last`, and `path4.json` / `path4-permission.json` for `answer`. Waits key on `events.ndjson` byte offsets or `--after <cursor>` (per test-plan §6 E2E Test Strategy, Paths 3 and 4; test-plan §2 Test Strategy, agent-runnable invariants).
- Seed state only through the product:
  - stamps from `viola verify` against the fake agent (`stamped_home`), for the `answer` and Path 7 legs
  - `budget.json` only from `viola hook statusline` stdin, for the `budget-paused` leg
  - each test makes its own tempfile `--home`

  (per test-plan §7 Test Data & Fixtures, Seed strategies)
- The control-completeness case in `tests/cli_controls_not_disableable.rs` fails if `viola --help` lists a long flag missing from its table. The `HELP` literal in `tests/cli_output_plain.rs` pins the grouped verb table. A new `viola mcp` subcommand or flag must keep both green (per test-plan §5 Integration Test Strategy, CLI).
- After every E2E test, the error-sanitization scan and the canary scan also cover MCP `isError` bodies. They must hold no absolute path, serde path or `Caused by`, and no canary may appear in a home-level `diagnostics/*.ndjson` line (per test-plan §6 E2E Test Strategy, Error sanitization and secret scan).

## Anti-patterns to avoid
- Never use a paused tokio clock or `MockClock` to drive the spawned wrapper's `wait` deadline. An in-process clock never reaches another process, so cross-process expiry is asserted in the §4 unit case (per test-plan §11 Test Anti-Patterns, Universal).
- Never use `sleep(N)` to synchronise an rmcp call against wrapper events. Wait for an `events.ndjson` offset or a cursor (per test-plan §11 Test Anti-Patterns, E2E).
- Never run the real `claude` in CI, and never treat a green fake-agent MCP run as proof of real-CLI behaviour. Any live start is local only, and its number is the founder's P4 fork (per test-plan §11 Test Anti-Patterns, Test Strategy).

## Contract bindings
- tests ↔ obs (log format): process logs carry `process` `mcp` and are written to `diagnostics/mcp.ndjson` and `instances/<name>/diagnostics/detail-mcp.ndjson`. G4 schema conformance validates those lines against `schemas/diag-line.v1.json` / `diag-detail.v1.json` (per test-plan §3 Log format; test-plan §6 E2E Test Strategy, Schema conformance). The harness `logs` takes `--process mcp` (per test-plan §3 5-command implementation, `logs`).
- tests ↔ harness layers: the E2E filterset regex `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` (suite `nextest-e2e`) decides where a test binary reports. An `mcp_` binary in `viola-e2e` reports as E2E. A non-matching binary under `crates/viola-mcp/tests/` reports in the integration layer (per test-plan §3 5-command implementation, `run` step 2).
- tests ↔ security (supply chain), lint: the rmcp release-graph assertion `cargo tree -e features -p viola --edges normal` (rmcp exactly `server`, `transport-io`) joins the Lint row with this chunk, beside the `deny.toml` `[[bans.features]]` gate (per test-plan §9 CI Integration, Lint).
- tests ↔ security (supply chain), tokio ban: the sole-root `deny-sync.toml` tokio ban runs once per crate in `scripts/sync-crates.txt`. An `--exclude` false-fails on a feature-unified optional tokio. That matters for `viola-channel` once `viola-mcp` enables `viola-channel[tokio]` (per test-plan §9 CI Integration, Supply-chain; test-plan §2 Test Strategy, why a test-only crate).
- tests ↔ security (server verification): the Exit-cause matrix places the MCP impostor case in `cross_exit_causes.rs` (`call_tool("send")` gives `is_error:true` `{"error":"instance-unreachable"}`). It presumes server verification before a frame, so whether it lands here depends on the founder's boundary fork (per test-plan §5 Integration Test Strategy, Wrapper channel; test-plan §6 E2E Test Strategy, Exit-cause matrix). The Single writers negative includes `mcp` tool calls: `snapshot.json` and `ledger/stamps.json` must not change (per test-plan §6 E2E Test Strategy, Security control negatives).
- tests ↔ coverage: the coverage ignore regex leaves out `crates[/\\]viola-e2e` but not `viola-mcp`, so the new crate counts toward the per-OS gate (per test-plan §10 Quality Gates & Coverage Targets).

## Acceptance criteria contributions
- (tests) `scripts/agent-run.sh run --e2e` passes on all three CI OSes with the rmcp-client legs of Paths 2, 3, 4, 5 and 7 and the Path 6 `budget-paused` MCP form. Each leg asserts `is_error`, the refusal text and `structured_content` equal to the CLI `--json` result for the same channel result, and no MCP refusal text or `structured_content` contains `release` (per test-plan §6 E2E Test Strategy, Paths 2–7 and Cross-surface parity).
- (tests) `tools/list` is exactly `send, wait, last, answer, list`. Schema-violating calls give `isError:true` with a code-only payload: a non-`ViolaName` `target`, a non-`u64` `dialog_id`, an open-string `behavior`, and control characters in `text` (per test-plan §4 Unit Test Strategy, viola-mcp; test-plan §6 E2E Test Strategy, Security sweep `mcp_schema_violations.rs`).
- (tests) The Lint stage holds a non-vacuous rmcp feature assertion: rmcp is present in `cargo tree -e features -p viola --edges normal` and resolves to exactly `server` + `transport-io` (per test-plan §9 CI Integration, Lint).
- (tests) The per-OS coverage gate stays at `--fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80` with `viola-mcp` included, and the ignore regex is not widened (per test-plan §10 Quality Gates & Coverage Targets).
