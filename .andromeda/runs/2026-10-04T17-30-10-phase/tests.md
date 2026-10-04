# tests extract

## Relevance
relevant: the wheel is a named testable entity (test-plan §1, the root bin's "the wheel"), and critical path 5 (§6 Path 5) is this chunk's working entry. CARRY §10, §12 and §13 touch tests the plan owns (§5 CLI, §6 Path 4, §6 Contract suite).

## Constraints
- Tier is Comprehensive (test-plan §1). New wheel code is held to the per-OS gate of lines ≥ 85 %, functions ≥ 95 % and regions ≥ 80 % (test-plan §10 Coverage thresholds). `#[cfg(windows)]` code, such as a Windows stdin path for the `^Z` CARRY, counts only on the `windows-2025` leg.
- test-plan §4 root bin requires two unit tables. The first covers the wheel state machine: a human editing key → `human`, a harness-origin prompt → no flip, `release` only through the CLI path, and `release` carrying `from` → `release-from-driver`. The second covers refusal ordering, with literal oracles: `send` goes human-typing → budget-paused → turn-running → input-not-ready → no-prompt-submitted; `answer` goes human-typing → unverified-cli → unknown-dialog; `control-character` precedes every one of them. Both are rstest `#[case]` tables with readable case labels (test-plan §4 Conventions).
- test-plan §6 Path 5 requires one scenario whose test owns the outer PTY (`viola run builder` on a stamped home). It writes the editing key `h`, sends, writes `ello\r`, releases, pauses, sends a `release` carrying `VIOLA_NAME=overseer`, and relaunches with `--inject-harness-turn`. Its verdicts come from `events.ndjson`, the fake-agent receipt and exit codes. The scenario lists tui, cli, ipc-internal (channel + MCP) and web-spa. test-plan §11 Test Strategy forbids asserting a multi-surface path on one surface only. So the MCP `send` step (`:102`) and the Playwright WHEEL-cell step (Epoch 8 web) must be recorded as owed, in the "as landed … owed to" form Paths 2–4 use, never silently dropped.
- test-plan §6 Path 4 step 6 requires a dialog negative under a human wheel set by `viola pause builder`: the hook stdout is empty and `answer` exits 10. test-plan §11 Universal forbids any test that passes while a decision body is emitted without a verify stamp and wheel `driver`. This is CARRY §12's test surface. The `null`-at-once answer to a pending `hook.dialog` when the wheel moves has no plan-named test; the chunk adds it at the lowest layer that can hold it (test-plan §11 Test Strategy, first bullet).
- test-plan §5 CLI requires `tests/cli_controls_not_disableable.rs` to re-run "human wheel → `send` exit 10" against a live test wrapper for every table row, with the same verdict as without the setting (CARRY §10). The table's rows stay literals (test-plan §11 Unit).
- Every refusal row in the exit-cause matrix, `human-typing` included, ends in a last stderr `hint:` line that never contains `release`. That holds for `detail:null` and for `manual-pause` (test-plan §6 Exit-cause matrix, last bullet; §6 Path 5 hint bullet).
- Tests must be deterministic: no sleeps, `retries = 0`, and waits keyed on `events.ndjson` byte offsets or receipt lines (test-plan §2 Agent-runnable invariants; §10 Zero-flakiness budget). OS-branch behaviour (the `^Z` console-stdin CARRY §8, the ConPTY focus-report CARRY §9) is proven only on its own runner, never by a Linux-host run (test-plan §11 Test Strategy).

## Patterns to follow
- Use the root rstest chain `home → fake_agent_path → stamped_home → booted_wrapper`: `tests/support/outer_pty.rs`, `Wrapper::boot/stop` in `tests/support/home.rs`, and the shared 7 s `WITHIN` exit-aware waits. Path 5's Cleanup is "a test-owned outer PTY", so it uses this chain, not `harness_session` (test-plan §3 `run`, step 2).
- The fake-agent receipt is the oracle. It records `key` {`hex`} for each byte outside a paste, `prompt` {`text`, `bare_esc`, `origin` `human`|`harness`}, and `size` {`cols`, `rows`} as the resize oracle. `--inject-harness-turn` fires a `<task-notification>`-prefixed prompt (test-plan §7 Fake agent). Path 5 step 8 also names an `<agent-message from="x">` prompt; whether the existing mode delivers both prefixes is research's question.
- Naming decides the layer through the harness filtersets. `tui_<topic>.rs`, `chaos_`, `contract_` and `path_` binaries run in the E2E layer; `cli_` and `channel_` binaries run in integration. A critical-path function is named `path5_<slug>`, and everything else `<subject>_<condition>_<expected>` (test-plan §2 naming; §3 `run` step 1 layer filtersets).
- Channel `pause` / `release` frames are covered in root `tests/channel_*.rs`, with result, refusal and error shapes schema-validated and the endpoint read only from `snapshot.json` (test-plan §5 Wrapper channel; §6 drivers table, ipc-internal).
- The three `contract_*` walkers stay run-time walks (an empty walk fails), not rstest `#[files]` (test-plan §6 Contract suite; §7 Fixture hygiene). The U40 `andromeda:walks-tree` mark is a comment line only and must change no walk behaviour.

## Anti-patterns to avoid
- Never parse the rendered child screen or PTY master output to judge the wheel. Never detect exit by master EOF; use `child.wait()`. Never call assert_cmd `.assert()` on the long-lived `viola run`. Never synchronise a keystroke, paste or resize with a sleep (test-plan §11 E2E).
- Never import the product's refusal ordering, exit codes or `HARNESS_PREFIXES` as a test oracle. Write them as literals, and compare `RefusalReason` by its kebab value, never by its `Debug` text (test-plan §11 Unit).
- Never `#[ignore]` or `test.skip` a Windows-only case, such as `^Z` or ConPTY focus reports, as a parking place. An unmeasurable case is recorded as owed to a route entry, never skipped green (test-plan §10 Zero-flakiness budget; §11 Test Strategy).

## Contract bindings
- tests ↔ obs: the new `wheel{holder, cause}` lines (`human-input`, `manual-pause`, `release`) and the `send-refused` `wheel` field (obs-plan §4 Scenario CL-1, CARRY §11) must pass the schema-conformance check behind gate G4 and the canary secret scan over `target/e2e-home/**` (test-plan §6 Schema conformance; §6 Error sanitization and secret scan). Wheel lines must not wake `wait` (test-plan §6 Path 3, step 3 signal).
- tests ↔ a11y: a11y-plan §3's three portable-pty outer-PTY nextest cases (`a11y-plan.md:573`): (1) zero viola-originated bytes on the outer stream; (2) human keys unblocked past the current atomic paste; (3) `\x1b[I`/`\x1b[O`, mouse and resize do not move the wheel. They run through the same outer-PTY driver, on all three OS legs (test-plan §6 drivers table, tui row). Case (3) on `windows-2025` depends on CARRY §9.
- tests ↔ security: `release` carrying `from` → `-32602` `release-from-driver`, exit 20 (test-plan §6 Path 5). Whether the `pause` / `release` frames ride the liveness-only pre-check is a security dated-gap question. If they do, the exit-cause matrix (test-plan §6) needs no new row only when an existing row covers the dated gap.
- tests ↔ setup registry (U40): the gate header's `walk-class rust 3 (…)` line reads the marks in the three `contract_*` files.

## Acceptance criteria contributions
- `path5_<slug>` (root `tui_`-prefixed or the outer-PTY chain) passes on all three CI OSes with these signals:
  - one `wheel{holder:"human",cause:"human-input"}` record;
  - `send` exits 10 with `detail:null`;
  - the receipt shows `hello` delivered, and a `prompt-submitted{origin:"human"}` record exists;
  - `release` returns `{wheel:"driver",budget_paused:false}`, and the next `send` exits 0;
  - after `pause`, `send` exits 10 with `detail:"manual-pause"`;
  - `release` with `from` exits 20 with `-32602` `release-from-driver`;
  - harness-injected turns log `prompt-submitted{origin:"harness"}` and no `wheel` record;
  - no `hint:` line contains `release`;
  - the MCP and Playwright steps are recorded as owed in §6 Path 5's "as landed" text.

  (per test-plan §6 Path 5)
- Root-bin unit `#[case]` tables for the wheel state machine and the `send` / `answer` refusal order pass, with literal oracles and `control-character` first (per test-plan §4 root bin; §11 Unit).
- `tests/cli_controls_not_disableable.rs` gains the human-wheel → `send` exit 10 negative on every row, and the Path 4 step 6 negative holds: with the wheel human through `pause`, the hook stdout is empty and `answer` exits 10 (per test-plan §5 CLI; §6 Path 4).
- `scripts/agent-run.sh run --all` is green locally. CI's per-OS `test` job ends in `viola-harness gate --require coverage,doctest,playwright` with the per-OS coverage thresholds met and zero retries (per test-plan §9 Coverage report row; §10 Coverage thresholds / Zero-flakiness budget).
