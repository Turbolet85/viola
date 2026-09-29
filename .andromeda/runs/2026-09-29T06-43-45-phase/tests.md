# tests extract

## Relevance
relevant — the chunk builds the plan's stamped fixture chain, harness `boot` steps 3–5 and readiness line 3, `run --local-live` and `contract_ledger_probes.rs`, and it folds a red test under the zero-flake budget.

## Constraints
- **Stamps come only from `viola verify` run against the fake agent.** No test or harness code writes `ledger/stamps.json` (per test-plan §7 Seed strategies, the `ledger/stamps.json` row; per §12 `2026-09-24`, the stamps-conflict resolution and User review 1). The fake agent sits on a test-scoped `PATH` as `claude` through `Command::env`, never `set_var` (per test-plan §3 `boot` step 3; per §7 Test data lifecycle).
- **Harness `boot` contract** (per test-plan §3 5-command implementation, `boot`):
  - Unless `--unstamped` is given, step 4 runs `viola verify --home <home>` and requires exit 0 plus a last stdout line matching `^stamped \S+  \d+ pass  0 fail$`. A failure is exit 1 `reason:"verify-failed"`.
  - Readiness requires `events.ndjson` lines 1–3 to be `wheel{cause:"start"}`, then `budget-gate`, then `session-start{source:"hook"}`. The line-3 check is named as joining with this chunk. The bound is 20 s per instance, polled every 100 ms as a file-state probe.
  - `--cli-version` and `--unstamped` belong to the §3 `boot` grammar. A flag not yet built stays a usage error, never a vacuous pass (per §3 Exit codes).
  - All logic lives in the Rust `viola-harness`. The `.sh` and `.ps1` shims stay identical thin forwarders (per §3 Harness implementation).
- **`supervise` spawns the fake agent with `--cli-version <ver> --fixtures <workspace root>/fixtures/claude`.** `--fixtures` names the absolute parent dir, and the fake agent joins `<cli-version>` itself, so replay follows `--cli-version` even under `--report-version` (per test-plan §3 `boot` step 5; §7 Fake agent). Path 7 depends on this: boot step 4 stamps "the default fixture version" while an instance reports `0.0.0-unlisted` (per §6 Path 7 step 1).
- **One rstest chain against one contract.** Root suites use `home → fake_agent_path → stamped_home → booted_wrapper`, and `stamped_home` runs `viola verify` against the fake agent. The plan's "interim seam (`stamped:false`, writes no `ledger/stamps.json`)" wording is what this chunk retires (per test-plan §3 `run` step 2; §3 Test data bootstrap).
  - Root waits stay bounded by `WITHIN` (7 s, below the `mutants` profile's 10 s kill) and are exit-aware.
  - The `viola_e2e::fixtures` copy lands with its first consumer among the §6 scenarios. Whether this chunk creates that consumer is P3's question.
- **`contract_ledger_probes.rs` behaviour** (per test-plan §5 Cross-module patterns, CLI, `contract_ledger_probes.rs`):
  - For each `fixtures/claude/<ver>/` set, `viola verify --home <tmp>` against the fake agent lists every expected ledger row id exactly once, with its post-condition result.
  - Row ids are test literals, and `<n>` in `stamped <ver>  <n> pass  0 fail` equals the literal row count.
  - The same check runs against the real CLI under `--local-live`.
  - The `contract_` prefix places the binary in the E2E layer filterset (per §3 `run` steps 1–2; §2 binary naming).
- **`run --local-live`** appends `viola verify` against the real `claude`. It refuses to start when `CI` is set: exit 2, `reason:"live-in-ci"`, with the one JSON document (per test-plan §3 `run`, `--local-live`; §11 Test Strategy; §6 Skipped as "untestable").
- **Per-test endpoint isolation.** The FNV-1a endpoint hash is `viola-<12 hex>` of `ViolaName + "\0" + abs home`, in the Windows `\\.\pipe\` form too, so same-named wrappers in parallel tests never collide (per test-plan §4 viola-channel golden vectors; §3 Test data bootstrap, Per-test isolation; §3 `run` step 2, session id). Whether HEAD's Windows pipe name actually carries the home is research's question, and it decides item 8's hypothesis.

## Patterns to follow
- **The fail-open matrix pattern.** `tests/hook_fail_open.rs` is an rstest `#[case]` matrix under an `Instant` bound (`< 1.0 s` per case) asserting exit 0 and empty streams (per test-plan §6 Non-path suites, Security sweep; §10 Performance budgets, Status). Item 8's fix and witness extend this matrix rather than a new harness.
- **Pin `viola verify`'s human lines by literal asserts.** `tests/cli_verify.rs` pins `[NN/06] <row id> …` and `stamped <ver>  <n> pass  <m> fail` (per test-plan §5 CLI). `stamped_home`, boot step 4 and `contract_ledger_probes.rs` parse that same last line.
- **Walk fixture sets at run time.** Use a directory walk, not rstest `#[files]`, which refuses a glob that matches no file at compile time. `tests/contract_fixture_hygiene.rs` is the precedent (per test-plan §7 Fixture hygiene).
- **Watch reports.** Each root wait notes what it polls to `<temp dir>/viola-root-watch/<test>.<label>.report`, and fails at once when the wrapper exits first (per test-plan §3 `run` step 2). A new stamped-home wait follows the same shape.
- **Literal oracles.** Expected ledger row ids, counts and exit codes are written into the test, never imported from product ledger rows (per test-plan §11 Unit).

## Anti-patterns to avoid
- **Hand-written state.** Never hand-write `ledger/stamps.json` or `snapshot.json`, and never write stamps from a non-`verify` process (per test-plan §11 Test Data; §6 Security control negatives, Single writers).
- **The real CLI in CI.** Never run the real `claude`, or `viola verify` against it, in CI. `--local-live` must refuse under `CI` (per test-plan §11 Test Strategy).
- **Sleeps and retries.**
  - No `sleep(N)` synchronisation, no retry budget and no `#[ignore]` parking (per test-plan §11 E2E; §11 Quality; §10 Zero-flakiness budget).
  - For item 8, a green re-run never closes the red. The chunk stays red until the root cause is fixed within it (per §10 Zero-flakiness budget).
  - Never reuse an endpoint name across tests (per §11 Integration).

## Contract bindings
- **tests ↔ obs, the log format and G4.** Boot readiness reads `diagnostics/run-<name>.ndjson` `process-start` lines for `subject:"self"` and `subject:"claude-child"` (per test-plan §3 `boot` Readiness signal). Item 7's new `subject` value for the verify probe child, a `diag-line.v1.json` amendment, must pass the tests-owned `schema-check` over `target/e2e-home/**` (per §3 Internal harness subcommands, `schema-check`; §6 Schema conformance). `process-start` and `process-exit` keep `corr` absent (per §3 Log format).
- **tests ↔ security.**
  - Single writer of `stamps.json` (per test-plan §6 Security control negatives, Single writers).
  - Endpoint uniqueness per home (per §4 viola-channel).
  - Item 8 meets the dated `hook.event` no-verification exception. That exception is security's, so the witness asserts the fail-open outcome, not server verification.
- **tests ↔ arch/CLI, closed enums.** `verify-failed` already sits in the §3 `boot` exit-reason list. `live-in-ci` is named in §3 `run` `--local-live` but absent from §3 Closed enums' `run` `reason` list, and a new value needs a Decisions Log entry (per test-plan §3 Closed enums). That is a wrap-time spec reconciliation.
- **tests ↔ fake agent, spec drift.**
  - §7 Fake agent's mode text says "`DEFAULT_CLI_VERSION` stays 2.1.0". Item 6 moves the default to `2.1.283`, so that wording becomes a wrap amendment (per test-plan §7 Fake agent, Modes).
  - The interim-seam wording in §3 `run` step 2, and "`boot` checks lines 1–2 today" in §3 Readiness, become stale once the chunk lands.
- **tests ↔ CI coverage.** Harness code in `crates/viola-e2e` and `tests/support` is outside the coverage number (`COVERAGE_IGNORE`). Product changes (the `StampError` fold, the verify spawn logging) count toward the per-OS floors (per test-plan §10 Coverage thresholds, Stack adjustments).

## Acceptance criteria contributions
- **Stamped and unstamped homes.**
  - The stamped `stamped_home` form runs `viola verify` against the fake agent. Exit 0 and the last line matching `^stamped \S+  \d+ pass  0 fail$` are what set `stamped`. `ledger/stamps.json` exists, and no test or support code writes it.
  - The explicitly-unstamped form writes no `stamps.json`, and its consumers (the version gate, `hook_fail_open`) keep their verdicts.
  - (per test-plan §7 Seed strategies; §3 `run` step 2; §11 Test Data)
- **Harness `boot`.**
  - Default `boot` exits 0 with readiness `events.ndjson` lines 1–3 = `wheel{cause:"start"}`, `budget-gate`, `session-start{source:"hook"}`, with the fake agent replaying `--fixtures <root>/fixtures/claude` at `--cli-version`.
  - `boot --unstamped` skips step 4.
  - A failing verify is exit 1 `{"reason":"verify-failed",…}`.
  - Identical documents through `agent-run.sh` and `agent-run.ps1`.
  - (per test-plan §3 `boot` steps 4–5, Readiness signal, Exit code; §3 Harness implementation)
- **Contract probes and the live run.**
  - `contract_ledger_probes.rs` passes for every committed `fixtures/claude/<ver>/` set: every literal row id appears exactly once with its post-condition, and `<n>` equals the literal row count.
  - `agent-run run --local-live` with `CI` set exits 2 with one JSON document `reason:"live-in-ci"`.
  - (per test-plan §5 CLI `contract_ledger_probes.rs`; §3 `run` `--local-live`)
- **Item 8 and the CI gates.**
  - The item-8 witness forces the colliding condition and reads red with the fix removed, under nextest `retries = 0` and without sleeps.
  - The per-OS `test` job is green on all three OSes, with `gate --require coverage,doctest,playwright` and coverage at lines 85 / functions 95 / regions 80.
  - (per test-plan §10 Zero-flakiness budget; §10 Coverage thresholds; §9 Build failure conditions)
