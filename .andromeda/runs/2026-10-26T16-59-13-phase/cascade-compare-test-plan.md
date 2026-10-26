# Cascade compare — test-plan · phase · 2026-10-26-home-and-code-bearing-file-integrity
E items 184 · held 48 · lost at classify 2 · lost at strong 1 · beside a placed fact - · cascade only 16 · listed 4 · body 84 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#1.2 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:15
- (.andromeda/test-plan-detail/ci-integration.md:15) Any dependency or `windows-sys` feature the chunk adds for the DACL read or the re-hash must pass the fail-closed supply-chain job.
  > ubuntu job `supply-chain`, fail-closed `shell: bash` steps: `cargo deny --format json check` (`deny.toml`: advisories, licences, sources, bans);

### ci-integration#1.9 · lost at strong · .andromeda/test-plan-detail/ci-integration.md:43
- (.andromeda/test-plan-detail/ci-integration.md:43) Code the chunk adds must build and pass the unit suite on the 1.96 floor, so it must use no std API newer than 1.96.
  > ubuntu `msrv`: `rustup toolchain install 1.96 --profile minimal`, then `RUSTUP_TOOLCHAIN=1.96` steps for `rustc --version`, `cargo check --workspace` and `agent-run run --unit`, then `gate --require nextest-unit`;

### test-strategy#1.7 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:29
- (.andromeda/test-plan-detail/test-strategy.md:29) The chunk carries no mutation gate of its own; mutation is judged at the epoch boundary, and Epoch 6 has none inside it.
  > No per-chunk, pre-push or CI mutation gate.

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:8
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) Every new test that spawns `viola` (a hook strict-modes negative, a 0770 `--home` negative, a seeded-home case) and clears its env must re-add `LLVM_PROFILE_FILE`, or that run drops out of the coverage number.
  > - Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates. Tests that call `env_clear()` must re-add it.

### quality-gates-coverage-targets#2.0 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:15
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) The chunk's gate set carries no mutation gate: mutation is judged at the Epoch 6 boundary audit or on demand, never as a chunk gate.
  > **Mutation gate (Founder Direction 1):** judged at the epoch boundary by `/andromeda-code-audit` and on demand by `run --mutants`; no chunk, pre-push or CI mutation gate exists since 2026-09-28 (the founder's ruling, 17:59).

### test-strategy#1.2 · listed (lost at strong) · .andromeda/test-plan-detail/test-strategy.md:14
- (.andromeda/test-plan-detail/test-strategy.md:14) The entry-point checks (CLI verbs, `hook`, `mcp`, `run`, `verify`) must be proven at integration level over a real tempfile home and the real binary, not a mocked filesystem.
  > Required. Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.

### test-strategy#2.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-strategy.md:42
- (.andromeda/test-plan-detail/test-strategy.md:42) New strict-modes negatives (the 0770 `--home` case, the seeded-home question, single writers) must live in the named root suite prefixes and share `tests/support/` helpers (`home.rs` holds the home and the ConPTY seed), never a local copy.
  > **Sync (tokio-free) E2E suites:** root `tests/{cli,hook,tui,channel,chaos,contract,security_negatives}_<topic>.rs`, sharing helpers in `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs}`.

### test-strategy#2.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-strategy.md:42
- (.andromeda/test-plan-detail/test-strategy.md:42) No new test file may define its own reader, wait, guard or result struct.
  > No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct.

## Held

- ci-integration#1.1
- ci-integration#1.11
- ci-integration#1.6
- ci-integration#1.8
- e2e-test-strategy#11.1
- e2e-test-strategy#12.1
- e2e-test-strategy#13.1
- e2e-test-strategy#14.1
- e2e-test-strategy#15.1
- e2e-test-strategy#19.1
- e2e-test-strategy#2.1
- e2e-test-strategy#2.2
- e2e-test-strategy#2.3
- e2e-test-strategy#7.2
- e2e-test-strategy#7.3
- e2e-test-strategy#8.1
- integration-test-strategy#1.2
- integration-test-strategy#1.4
- integration-test-strategy#1.5
- integration-test-strategy#1.6
- integration-test-strategy#2.0
- integration-test-strategy#3.1
- integration-test-strategy#3.2
- integration-test-strategy#4.1
- mocking-stubbing-discipline#2.3
- mocking-stubbing-discipline#2.4
- quality-gates-coverage-targets#3.1
- test-data-fixtures#1.1
- test-data-fixtures#1.2
- test-data-fixtures#2.1
- test-data-fixtures#4.1
- test-scope-summary#1.1
- test-scope-summary#10.1
- test-scope-summary#3.1
- test-scope-summary#4.1
- test-scope-summary#6.1
- test-scope-summary#7.1
- test-scope-summary#8.1
- test-strategy#1.1
- test-strategy#1.5
- test-strategy#1.6
- test-strategy#3.1
- test-strategy#3.13
- test-strategy#3.16
- test-strategy#3.5
- test-strategy#3.6
- unit-test-strategy#1.2
- unit-test-strategy#2.1

## Cascade only

- e2e-test-strategy#1.2
- e2e-test-strategy#1.5
- e2e-test-strategy#4.3
- e2e-test-strategy#7.1
- e2e-test-strategy#8.2
- test-data-fixtures#3.1
- test-data-fixtures#5.1
- test-data-fixtures#6.1
- test-scope-summary#5.3
- test-scope-summary#9.2
- test-strategy#1.3
- test-strategy#3.10
- test-strategy#3.11
- test-strategy#3.18
- test-strategy#3.3
- test-strategy#3.4
