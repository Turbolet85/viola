# tests extract

## Relevance
relevant — the chunk changes a product deadline (the dialog hold), may add an event kind that `wait` wakes on, may add a `config.json` key, and moves the `hooks.json` dialog `timeout`. Each of these has a test-plan home.

## Constraints
- The hold's expiry is a clock case and belongs at the unit tier. test-plan §11 Test Strategy bans asserting at E2E anything a lower layer can hold: parser, refusal-order, threshold and clock cases go to §4. So expiry at the bound (not before, exactly once) needs a §4 case driven by the injected `viola_core::Clock` (per test-plan §8 Mocking & Stubbing Discipline, Time row). Whether `src/run/dialog.rs`'s hold already reads its deadline through the `Clock` seam is research's question.
- An E2E step may block on the hold's real deadline only under both conditions in test-plan §11 Universal: the verdict is the product's own exit code or status field, and the same deadline is clock-driven in a §4 unit case. test-plan §11 Universal also bans a test-owned timer, sleep or elapsed-time verdict, and bans `MockClock` steering a spawned `viola run`. A hold of minutes is therefore not waited out in E2E.
- If the expired-hold record is a new `events.ndjson` kind, two §4/§5 cases must take it:
  - the round-trip over every kind of `EventKind::ALL`, with zero on all three reader counts (per test-plan §5 Cross-module patterns, On-disk; today 15 kinds in `crates/viola-state/tests/state_events.rs`)
  - the channel `wait` handler's wake set, today exactly `turn-ended` / `question` / `permission` / `plan` / `session-end`, clock-driven (per test-plan §4 What unit tests cover, root bin)
- If a configurable hold lands as a `config.json` key (or a flag or `VIOLA_*` variable), it must become a row of `tests/cli_controls_not_disableable.rs`. It is re-run against the five control negatives with disabling-shaped values (`0`, `false`, `off`, empty, out of range). The completeness case requires `schemas/config.v1.json`'s properties to equal `CONFIG_KEYS` (per test-plan §5 Cross-module patterns, CLI). An out-of-range value must take a bounded default, never "forever" or off (per test-plan §5 Cross-module patterns, Wrapper channel, the out-of-range `Percent` precedent).
- Expected values go into the test as literals: a new bound, a new kind name, a refusal order, and the ledger `ROW_IDS` if a new ledger row is added. They are never imported from the product (per test-plan §11 Unit). A ledger row would also move `contract_ledger_probes.rs`'s `ROW_IDS` literal and its `<n> pass` count (per test-plan §5 Cross-module patterns, CLI).
- The real CLI honouring a dialog-hook `timeout` past 75 s cannot be proven in CI. CI never runs the real `claude` (per test-plan §11 Test Strategy), and a green fake-agent run is no proof of real-CLI behaviour (per test-plan §11 Test Strategy). A live reading is a local measurement kept in the chunk's `evidence/`, not a test layer (per test-plan §6 E2E Test Strategy, Skipped as "untestable").
- Gates that apply:
  - the default harness `run` (`--all`: unit, integration, doctest, E2E) (per test-plan §3 `run`)
  - per-OS coverage at ≥ 85% lines, ≥ 95% functions and ≥ 80% regions (per test-plan §10 Coverage thresholds)
  - lint/clippy failure is a build failure (per test-plan §10 Build failure conditions), which carries the PREREQ clippy run
  - zero flakiness, `retries = 0`, and a flake keeps the chunk red until its root cause is fixed (per test-plan §10 Zero-flakiness budget); this applies to the carried `send_budget_paused…` case_4 WATCH
  - no per-chunk mutation gate (per test-plan §10 Mutation gate)

## Patterns to follow
- Order dialog steps deterministically with a gated fake script, `fixtures/fake-scripts/<scenario>.json`, synthetic text only, released by control lines. The model is Path 4's `path4.json` / `path4-permission.json` (per test-plan §7 Fake agent).
- Path 4's existing late-`answer` → exit 13 `unknown-dialog` witness (`tests/cli_answer.rs`) and Path 3's parked-`wait` wake witnesses are the end-to-end shape for an expired hold seen through `wait` (per test-plan §6 Path 3 and Path 4).
- MCP parity: the `mcp` `wait` result is compared with the CLI's `--json` document less `v` in `crates/viola-e2e/tests/mcp_tools.rs`. The MCP `wait` bound is the pure clamp `wait_timeout(given)` = `min(given, 50 000)` (per test-plan §6 Path 3; test-plan §7 Seed strategies, Time-sensitive data).
- Write table cases as rstest `#[case::label]` rows with readable labels (per test-plan §4 Conventions). Pin decision bodies with insta, `.dialog_id` redacted, if an expiry reply body changes (per test-plan §4 What unit tests cover, viola-agent-claude).
- Root tests read and wait through the one `tests/support/events.rs` reader and `tests/support/cli.rs` runner, with no copy in a test file (per test-plan §2 Test directory + naming conventions).

## Anti-patterns to avoid
- No `sleep(N)` or test-owned real-time wait for the hold to expire. Wait on an `events.ndjson` offset or `viola wait --after` (per test-plan §11 E2E; test-plan §11 Universal).
- No hand-written `snapshot.json`, `ledger/stamps.json` or `budget.json` to stage a pending or expired dialog. Stamps come from `viola verify` against the fake agent (per test-plan §11 Test Data).
- No test that auto-approves a dialog, or that passes when a decision body is emitted without a stamp and wheel `driver`. A viola-built reply at expiry must not slip past that oracle (per test-plan §11 Universal).

## Contract bindings
- tests ↔ obs: a new event kind enters obs-plan's event catalog. The tests side is the §5 On-disk round-trip over `EventKind::ALL` and the G4 schema-conformance scan of every home under `target/e2e-home` (per test-plan §5 Setup / teardown lifecycle; test-plan §5 Cross-module patterns, On-disk).
- tests ↔ architecture §Standard Contracts (event line, Hook Contract): the hook-timeout margin over the hold is held by `hooks_json_dialog_timeouts_outlast_the_dialog_deadline`. A per-start `timeout` makes the rendered `plugin/hooks/hooks.json` a start-time output, read back the way Path 1 reads the rewritten plugin files (per test-plan §6 Path 1).
- tests ↔ security §Input Validation: a hold setting is a bounded closed range that no value switches off, held by `tests/cli_controls_not_disableable.rs` (per test-plan §5 Cross-module patterns, CLI).

## Acceptance criteria contributions
- `scripts/agent-run.sh run` (default `--all`) exits 0, and `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` is clean (per test-plan §3 `run`; test-plan §10 Build failure conditions).
- A §4 unit case driven by the injected `Clock` shows:
  - a held dialog is not expired one tick before the bound
  - at the bound it expires with exactly one expired-hold record carrying `dialog_id` and the dialog kind and no upstream text
  - a human key before the bound still hands it back at once

  (per test-plan §11 Universal; test-plan §4 What unit tests cover, root bin)
- If a new kind lands, the `state_events.rs` round-trip covers it with zero skip counts, and the clock-driven `wait` handler case wakes on it (per test-plan §5 Cross-module patterns, On-disk; test-plan §4 What unit tests cover, root bin).
- If a hold setting lands, it is a row of `tests/cli_controls_not_disableable.rs`, and the completeness case passes with the schema's properties equal to `CONFIG_KEYS` (per test-plan §5 Cross-module patterns, CLI).
