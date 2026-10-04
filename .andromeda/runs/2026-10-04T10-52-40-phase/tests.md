# tests extract

## Relevance
relevant — the chunk delivers critical path P4 (dialog → `answer`), part of P7 (unverified CLI), the `pre-tool-use` perf row, a controls-table row, the S8 contract witness, the fake agent's `matcher`, and two CI reds that sit wholly in test code. MCP `answer` (`:102`) and the web / `/api/sessions` bullets of Path 4 (Epoch 8) are out of scope here.

## Constraints
- Tier is Comprehensive (per test-plan §1 Test tier). Each new surface needs unit + integration + E2E layers on all three CI OSes. The coverage floors are per OS: lines ≥ 85, functions ≥ 95, regions ≥ 80 (per test-plan §10 Coverage thresholds; §9 Matrix builds). Linux-only local greens are not proof for Windows or macOS (per test-plan §11 Test Strategy).
- Each case goes to the lowest layer that can hold it (per test-plan §11 Test Strategy, first bullet):
  - §4 root bin unit holds:
    - the S3 / S7 / S8 decision-body mapping, as `viola-agent-claude` units with insta `assert_json_snapshot!` and `.dialog_id` / `.ts` redactions (per test-plan §4 viola-agent-claude; §4 Fixture pattern);
    - `answer`'s refusal order `human-typing → unverified-cli → unknown-dialog` with `control-character` first, as an rstest `#[case]` table;
    - the exit-code table rows 12 / 13 (per test-plan §4 root bin).
  - The oracles in those tests are literals, never the product's own ordering, ledger rows or `Debug` strings (per test-plan §11 Unit).
- Decision bodies are insta-pinned in check mode only (`INSTA_UPDATE=no`). Snapshots change only through the local `viola verify` fixture refresh, never through `cargo insta review` (per test-plan §2 Contract row; §7 Seed strategies "Decision bodies"; §11 Universal). Whether the P4 fork in scope §5 allows a recording that would refresh them is P4's question. The test plan gives no path that pins them without one.
- No blanket approval:
  - Some test must fail if a decision body is emitted without both a verify stamp and wheel `driver` (per test-plan §1 Coverage triggers "discipline"; §11 Universal; §6 Path 4 step 6).
  - The wheel and `pause` belong to `:80`. Whether this chunk can build the human-wheel half of step 6, or only the unstamped half, is research's question.
- Determinism (per test-plan §2 Agent-runnable invariants; §11 E2E; §11 Universal; §10 Zero-flakiness budget):
  - No `sleep` synchronisation. Waits key on `events.ndjson` offsets, `viola wait --after`, and gated fake-script steps through `<home>/fake/<name>.control`.
  - `retries = 0`. No `#[ignore]` parking.
  - The dialog-deadline expiry is asserted as behaviour through the product's own exit or status signal. Its clock-driven verdict lives in a §4 unit case with the injected `viola_core::Clock`.
- The fake agent's hook `matcher` evaluation lands with this chunk. It is evaluated only against recorded tool-bearing fixtures, never docs-only behaviour (per test-plan §7 Fake agent, the hooks.json bullet). A green fake-agent run is no proof of real-CLI behaviour (per test-plan §11 Test Strategy). Fixture files follow the `<Event>.<variant>.json` PascalCase naming with no mapping table (per test-plan §2 File naming).
  - §6 Path 4 step 1 spells its fixtures `permission-request.bash.json` / `pre-tool-use.exit-plan-mode.json`, which conflicts with §2. P4 should settle which naming governs.
- Synthetic data and the canary:
  - Every dialog answer, question text and tool `input` a test feeds embeds the tests-owned canary constant.
  - The canary may appear only in `events.ndjson` and `instances/<name>/diagnostics/detail-*.ndjson`, never in a home-level `diagnostics/*.ndjson` line (per test-plan §6 Error sanitization and secret scan, Canary).
  - Stamps come only from `viola verify` against the fake agent. Snapshots and stamps are never hand-written (per test-plan §7 Seed strategies; §11 Test Data).

## Patterns to follow
- **Landed-surface scoping of a critical path.** Path 2 and Path 3 each carry an "As landed" line naming the test file, the surfaces it covers on all three OSes, and the route entry that owns each remaining half (per test-plan §6 Path 2 / Path 3 Surfaces lines). Path 4 should get the same: the cli + wrapper-channel half lands here, with MCP owed to `:102` and the `/api/sessions` + Playwright bullets owed to their Epoch 8 entries. Path 3's line already names `:78` as the owner of the end-to-end `question` / `permission` / `plan` wake witness (CARRY 5).
- **Boot chain and gated scripts.**
  - Root `tests/` suites use the rstest chain `home → fake_agent_path → stamped_home → booted_wrapper`, and `StampedHome::unstamped` for the unverified path (per test-plan §3 `run` step 2, 5-command-implementation key).
  - Scenario turn scripts are checked in as `fixtures/fake-scripts/<scenario>.json` (`path4.json`), validated by `schemas/fake-script.v1.json` and walked by `contract_fixture_hygiene.rs` (per test-plan §7 Fake agent; §7 Fixture hygiene).
  - Tool-bearing `fixtures/claude/<ver>/*.json` join the same hygiene walk against `schemas/claude-fixture.v1.json`.
- **A write-side error class that does not hide the verdict** (for CI red A). The oversize-frame channel test accepts EPIPE / ENOTCONN / a reset on its write and still requires the -32600 reply and the close (per test-plan §5 Cross-module patterns, Wrapper channel, `MAX_FRAME` bullet). The `cli_send` stdin write can follow the same shape: tolerate only `BrokenPipe` on the write, then still assert the usage-error exit 2 and the `--json` document.
- **Precedent for mutants timeouts** (for CI red B):
  - The two real-cargo-mutants harness tests were scoped by a measured, runner-side cause: compiled out on macOS with `#[cfg(not(target_os = "macos"))]`, never `#[ignore]`, with the measurement recorded under the chunk's `evidence/` (per test-plan §10 Mutation gate).
  - The root wait bound (`WITHIN` 7 s) is held below cargo-mutants' 20 s auto-timeout floor so that a mutant grades as caught, not Timeout (per test-plan §3 `run` step 2, 5-command-implementation key).
  - The counting rule `survived = missed + timeout` is the run's Output contract (per test-plan §3 `run` Output format), so the fix follows the measured cause and leaves the rule alone.
- **The `pre-tool-use` perf row** (per test-plan §10 Performance budgets, perf table and Perf run rules; §3 `gate`, 5-command-implementation key). It joins `run::PERF_ROWS`, which `gate` requires by name (`perf-pre-tool-use.json`). It is fed a synthetic, canary-bearing payload through `--input` with `VIOLA_NAME` / `VIOLA_DIR` set, so it takes the real channel path. It is gated on `max < viola_core::SPINE_DEADLINE`.
  - The perf session boots stamped (`stamp: true`), but the row is specified "on an unverified CLI (`hook.dialog` → `null` at once)". How the row reaches the unverified path inside a stamped perf session is research's question.
  - The `--perf` suite's "6 passed on green" count and the gate's four-row text are written for four rows.

## Anti-patterns to avoid
- Never write a test that auto-approves a dialog, or one that passes when a body is emitted without a stamp and wheel `driver` (per test-plan §11 Universal).
- Never "fix" a red with a retry, a re-run, `#[ignore]`, a swallowed error or a loosened counting rule. A fix that swallows every stdin write error, skips the usage-error assertion, or stops counting a timeout as a survivor without a measured cause breaks the zero-flakiness and quality bans (per test-plan §10 Zero-flakiness budget; §11 Quality "NEVER trust the cargo-mutants exit code alone").
- Never parse the child screen for a dialog verdict. Verdicts come from hook receipts (`exit_code`, `stderr_len`, `stdout_hex`), `events.ndjson` and channel payloads (per test-plan §11 E2E). Never stub `viola hook`: the fake agent runs the real pinned binary through the plugin files (per test-plan §11 Mocking).

## Contract bindings
- **tests ↔ obs:**
  - The new `dialog-raised` / `dialog-answered` / `hook-decision{decision_emitted, deadline_hit}` lines fall under the G4 schema check against `schemas/diag-line.v1.json` / `diag-detail.v1.json` and under G2 zero panics (per test-plan §6 Schema conformance; §3 Internal harness subcommands `schema-check`).
  - The canary scan must find no question or answer text in home-level diagnostics (per test-plan §6 Error sanitization and secret scan).
- **tests ↔ security:**
  - The no-blanket-approval negative and the unstamped `answer` → exit 12 row in `tests/cli_controls_not_disableable.rs` re-run the control under every disabling-shaped setting (per test-plan §5 CLI, controls table).
  - Control-character refusal is tested at both client and wrapper for a free-text answer and a revise `message` (per test-plan §5 Wrapper channel, paste-validation bullet).
  - Oversize `answer` stdin leaves the dialog pending (per test-plan §5 Wrapper channel, oversize `answer` bullet).
  - Leading-slash argv in `answer`'s positions is a usage error (per test-plan §5 CLI, `cli_slash_args.rs`).
- **tests ↔ arch:**
  - The `--json` refusal documents for exit 12 / 13 and the human-mode `wait` line are asserted by literal.
  - Exit codes are asserted as integers, and refusal values as serde kebab-case (per test-plan §11 Unit).
- **tests ↔ harness:**
  - `run --perf` and `gate --require perf` gain the fifth row (per test-plan §3 `run` `--perf` and `gate`, 5-command-implementation key).
  - The msrv job's `run --unit` runs the real-cargo-mutants harness tests on ubuntu (per test-plan §9 MSRV row; §10 Mutation gate).

## Acceptance criteria contributions
- (tests) The Path 4 Rust scenario passes on all three CI OSes over the wrapper channel and CLI:
  - each `question` / `permission` / `plan` event is logged exactly once with a `dialog_id`, and `wait` returns it;
  - each hook receipt reads `exit_code:0`, `stderr_len:0`, with stdout equal to the insta snapshot (check mode, `.dialog_id` redacted) for the question + `annotations` (S8), permission allow / deny + `message`, plan approve via PreToolUse, and plan revise;
  - a second concurrent dialog's hook stdout is empty;
  - `viola answer builder 999999 --json` exits 13 `{"refusal":"not-delivered","detail":"unknown-dialog"}`;
  - a no-blanket-approval negative fails if any body is emitted without a stamp.
  
  (per test-plan §6 Path 4; §6 Contract suite S8 bullet; §11 Universal)
- (tests) On an unstamped home (`StampedHome::unstamped`):
  - `hook.dialog` gives empty hook stdout with exit 0 within the spine deadline;
  - `viola answer` exits 12 `{"refusal":"unverified-cli","detail":null}`;
  - `tests/cli_controls_not_disableable.rs` gains that `answer` exit-12 negative as a table row.
  
  (per test-plan §6 Path 7; §5 CLI controls table)
- (tests) `run --perf` times `pre-tool-use`, and `gate --require perf` requires `perf-pre-tool-use.json` by name:
  - an absent file is its own `artifact-missing` breach;
  - its `.results[0].max` is `< viola_core::SPINE_DEADLINE` on every OS.
  
  (per test-plan §10 Performance budgets; §3 `gate`, 5-command-implementation key)
- (tests) Both CI reds of ci#37196414168 close on their measured cause, with no retry, `#[ignore]` or broadened error swallowing:
  - `send_leading_slash_argument_is_a_usage_error` still asserts exit 2 and its usage document when the child exits before the stdin write;
  - `run_mutants_passes_when_the_change_is_tested` passes on msrv with `survived = missed + timeout` and `mutants_suite_counts_missed_and_timeout_as_survivors` unchanged, and the untested-change twin still reports survivors;
  - the measurement is recorded under the chunk's `evidence/`.
  
  (per test-plan §10 Zero-flakiness budget; §10 Mutation gate; §11 Quality)
