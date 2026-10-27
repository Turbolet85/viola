# tests extract

## Relevance
relevant: the chunk is the test plan's input-bound coverage triggers and parser property and fuzz surfaces, made complete and provable per boundary.

## Constraints
- test-plan §1 Coverage triggers (paste injection) requires negative tests showing ESC, other C0, DEL and C1 (U+0080–U+009F) in `send.text` and free-text answers refused (never stripped) as `not-delivered` / `control-character` ahead of every other refusal, at the client and at the wrapper alike, with LF / CR / TAB and multi-byte UTF-8 accepted. Whether every client (CLI `send`, MCP `send`, free-text `answer`) already runs the check before its frame is research's question.
- test-plan §1 Coverage triggers (hook stdin · CLI and env · child spawn · MCP) requires a negative per bounded input: oversize and malformed hook stdin fail open (exit 0, empty stderr, no body); path-traversal names (`../x`, uppercase, `/`) rejected by name parsing; oversize `answer` JSON; an out-of-range `Percent` in `config.json`; oversize or malformed `claude agents --json` read as `unknown`, a row `name` never usable as a target; schema-violating MCP arguments (non-`ViolaName` `target`, non-`u64` `dialog_id`, open-string `behavior`, control chars in `text`) rejected by the handler; a forced vt100 panic giving `not-delivered` / `input-not-ready` while passthrough continues.
- test-plan §6 Property suite requires proptest at `cases: 512` over the parser surfaces (`validate_paste_text`, ndjson framing at `MAX_FRAME` ± 1, the hook stdin and statusline readers, the `claude agents --json` parser, the vt100 feed under `catch_unwind`, `prompt-submitted` normalisation), with `proptest-regressions/` committed, and states that cargo-fuzz targets in the separate `fuzz/` workspace join as their parsers land. A newly bounded parser with no target is therefore a gap for this chunk to close or name.
- test-plan §4 Unit Test Strategy puts the per-boundary witnesses in each owning crate's inline tests: the `viola-channel` frame codec at `MAX_FRAME` − 1, `MAX_FRAME` and `MAX_FRAME` + 1; the `viola-state` reader counting an over-long (> `MAX_FRAME`) line in `torn_lines` and never panicking; `viola-core`'s `ViolaName`, `Percent` and `DialogHold` accept and reject tables and `validate_paste_text` on decoded `char`s. Whether every reader the scope lists already has its own witness is research's question.
- test-plan §5 Cross-module patterns requires the channel suites to show `-32600` and a close on an oversize frame, `-32602` on a higher `v`, and wrapper-side paste validation where `control-character` wins over every other refusal. It also requires schema-validated result, refusal and error shapes over a real endpoint.
- test-plan §11 Test Strategy forbids asserting at E2E any case a lower layer can hold: parser, cap, depth and closed-type cases belong in §4 unit tests, and §6 asserts only cross-process behaviour (the forced-panic passthrough, hook fail-open through the real binary).
- test-plan §10 Coverage thresholds holds the per-OS gate at lines ≥ 85 %, functions ≥ 95 % and regions ≥ 80 %. test-plan §10 Zero-flakiness budget holds `retries = 0` and allows no `#[ignore]` parking.

## Patterns to follow
- Table-driven witnesses use rstest `#[case::<label>]` with readable labels. `viola-core` instead uses literal tables in one `#[test]`, and `viola-e2e` uses a labelled `(label, …)` case table, because neither carries rstest (per test-plan §4 Conventions).
- Test names follow snake_case `<subject>_<condition>_<expected>`, for example `hook_oversize_stdin_exits_zero_silent` or `send_with_esc_refuses_control_character`. Root integration binaries carry the `<surface>_<topic>.rs` prefix that the §3 layer filtersets select on (per test-plan §2 Test directory + naming conventions).
- A reader's own unit case plants an oversize or malformed state file only by writing it through the product writer and then padding, replacing or re-versioning those bytes (per test-plan §11 Test Data).
- New fuzz targets take synthetic seeds only, and a binary corpus is kept byte-exact through `.gitattributes`. This follows the pattern of `fuzz/corpus/paste_text/**` and `claude_agents_json/**` (per test-plan §6 Property suite).
- The forced vt100 panic case sits in the chaos suite (`chaos_*.rs`; `send` exits 13 `input-not-ready`, human passthrough still reaches the receipt). `tests/chaos_feed_panic.rs` alone boots outside the scans (per test-plan §6 Chaos suite and test-plan §5 Setup / teardown lifecycle).

## Anti-patterns to avoid
- NEVER import the product's own list as the test oracle. The enumeration's expected set of external readers and their caps, like the identity floor and the refusal order, is written as literals in the test (per test-plan §11 Unit).
- NEVER hand-write `snapshot.json`, `budget.json` or `ledger/stamps.json` in a booted-wrapper test to reach an oversize read (per test-plan §11 Test Data).
- NEVER synchronise with `sleep`. A cap or panic case waits on an `events.ndjson` offset, the product's exit code or a status field (per test-plan §11 E2E).

## Contract bindings
- The tests §6 Schema conformance check is the body behind obs-plan gate G4. Any `parse-rejected` or `state-recovered` line that a cap or closed-type refusal writes must validate against obs-plan's `schemas/diag-line.v1.json` or `diag-detail.v1.json`.
- The tests §6 Error sanitization and secret scan binds to obs-plan §9 and security-plan §Error Handling. A cap refusal's `--json`, MCP `isError` and channel `error.data` carry codes only, and every `-32603` is exactly `"internal error"` / `null`. The sanitisation rework itself belongs to "Sanitised error surfaces".
- The tests §1 Coverage triggers bind to security-plan §Input Validation (Vectors 2, 4, 5, 6 and 8): each control there needs its negative here.
- The tests §3 `run` layer filtersets decide where a new witness reports. A `chaos_` / `cross_` / `contract_` binary runs in the E2E layer, and a root `hook_*` file such as `hook_fail_open.rs` runs in integration.

## Acceptance criteria contributions
- `scripts/agent-run.sh run --all` exits 0 with the new per-boundary witnesses present, each reporting in the layer its binary prefix selects (per test-plan §3 `run`).
- The per-OS `run --coverage` followed by `viola-harness gate --require` stays at or above lines 85 / functions 95 / regions 80 (per test-plan §10 Coverage thresholds).
- A paste negative for ESC, other C0, DEL and C1 refuses `control-character` before any frame at the CLI `send` and MCP `send` clients, and authoritatively at the wrapper, with LF / CR / TAB and multi-byte UTF-8 accepted (per test-plan §1 Coverage triggers (paste injection)).
- Every bounded parser surface has its proptest at `cases: 512` with `proptest-regressions/` committed, and `run --fuzz-replay` replays every `fuzz/` corpus green, including any target this chunk adds (per test-plan §6 Property suite).
