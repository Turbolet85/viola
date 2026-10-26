# Cascade compare — test-plan · phase · 2026-10-26-server-verification-before-any-frame
E items 234 · held 46 · lost at classify 9 · lost at strong 4 · beside a placed fact - · cascade only 16 · listed 4 · body 130 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#1.2 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:14
- (.andromeda/test-plan-detail/ci-integration.md:14) Any dependency or feature the verification adds (a `sysinfo` start-time read or a `windows-sys` pipe feature in a client crate) must pass the fail-closed root `cargo deny` check.
  > `cargo deny --format json check` (`deny.toml`: advisories, licences, sources, bans);

### ci-integration#1.2 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:14
- (.andromeda/test-plan-detail/ci-integration.md:14) The sync client's verification in `viola-channel` must keep its normal graph tokio-free, and the sync crates must pass the tokio ban with each one as sole root.
  > the tokio ban once per crate in `scripts/sync-crates.txt` as sole root, `cargo deny --config deny-sync.toml --manifest-path crates/<crate>/Cargo.toml check bans`, then, in the same step `Tokio ban per sync crate (sole root)`, `bash scripts/channel-tokio-free.sh` (chunk 2026-10-17-mcp-server-for-drivers: `cargo tree -p viola-channel -e normal --target all -i tokio --prefix none` must print no `tokio v` line — exit 0 "nothing to print", or exit 101 "did not match" while tokio is not in the lock; presence read from the printed line, never the exit — and its control with `--features tokio` must print one; verdict `channel-tokio-free: ok`);

### ci-integration#1.2 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:14
- (.andromeda/test-plan-detail/ci-integration.md:14) The ban probes must still all fire, so no change to deny configuration may weaken a ban.
  > `bash scripts/deny-probes.sh` (a permanent negative probe per ban + a clean control, last line `deny-probes: 13/13 banned, control clean`:

### ci-integration#1.7 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:34
- (.andromeda/test-plan-detail/ci-integration.md:34) A Windows-gated file the chunk adds or changes for the pipe server's identity must sit inside a `windows-mutants.yml` matrix item's Windows-gated file scope, so the audit's Windows mutation leg reaches it.
  > ten `mutants (<label>)` jobs on `windows-2025` (`fail-fast: false`, 120 min), one per `matrix.include` item (the count of its `- package:` items; the root package `viola` is five of them, split by file, the fifth `viola-cmd-verify-stop` over `src/cmd/verify/stop.rs` since chunk 2026-10-13-interrupted-verify-cleanup), each running `scripts/agent-run.ps1 run --mutants --package <member> --file …` over that item's Windows-gated files, report-only, no cache or upload

### ci-integration#1.9 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:42
- (.andromeda/test-plan-detail/ci-integration.md:42) The verification code and any crate it pulls in must build and pass the unit suite on the 1.96 MSRV toolchain.
  > `RUSTUP_TOOLCHAIN=1.96` steps for `rustc --version`, `cargo check --workspace` and `agent-run run --unit`, then `gate --require nextest-unit`;

### ci-integration#1.11 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:50
- (.andromeda/test-plan-detail/ci-integration.md:50) Any test seam the chunk adds to force a pid or start-time mismatch must stay behind a test-only feature, so the release build stays `viola only`.
  > It prints `release-check: viola only` or fails on an artifact built with a test-only feature (`test-support`, `fake-agent`), on a test-only binary or on no executable, and never lists `target/release/`.

### mocking-stubbing-discipline#2.3 · lost at classify · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:18
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18) Any double the server-verification tests use (a liveness or identity probe standing in for the server pid / start-time read) must be a mockall `automock` on a seam trait the product defines, and every external process in them a real stub executable, never a patched call.
  > mockall 0.15.0 on the `viola-pty` `Pty` trait and the liveness / `Clock` traits; native stub binaries (`viola-fake-agent`) | `#[cfg_attr(test, mockall::automock)]` on seam traits only; external processes are real stub executables, resolved through `CARGO_BIN_EXE_viola-fake-agent` in root `tests/` only. `viola-e2e` tests and the harness use the assert_cmd target-dir lookup (§3 boot step 5)

### quality-gates-coverage-targets#2.0 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:15
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) This chunk has no mutation gate of its own; its code is scored at the Epoch 6 boundary audit.
  > **Mutation gate (Founder Direction 1):** judged at the epoch boundary by `/andromeda-code-audit` and on demand by `run --mutants`; no chunk, pre-push or CI mutation gate exists since 2026-09-28 (the founder's ruling, 17:59).

### quality-gates-coverage-targets#2.0 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) The chunk's Windows-only verification arm (the pipe's server process id) will be scored at that audit by this rule, recorded by coordinate as owed, never as equivalent or caught.
  > A mutant this host cannot compile or reach — inside a `cfg(windows)` item or block, or behind a const such as `HOST_SCRATCH = cfg!(windows)` — is recorded "not measured here; owed to {the route entry that measures it}" by coordinate, never "equivalent", and never counted as caught;

### test-data-fixtures#1.5 · lost at strong · .andromeda/test-plan-detail/test-data-fixtures.md:14
- (.andromeda/test-plan-detail/test-data-fixtures.md:14) Any `revive` or `list` case this chunk adds or changes, such as a client check P3 finds in them, must run under a test-scoped `PATH`.
  > every test-started `list` / `revive` runs under a test-scoped `PATH` (`support::cli::spawn_on` / `viola_on`, or a plain `spawn` of `list` under an empty bin dir)

### test-data-fixtures#1.6 · lost at strong · .andromeda/test-plan-detail/test-data-fixtures.md:15
- (.andromeda/test-plan-detail/test-data-fixtures.md:15) Hook-client verification cases (`hook.event` for every event, and `hook.dialog`) must feed the checked-in recorded payloads on stdin. They must not use payloads built by the test.
  > Checked-in `fixtures/claude/<cli-version>/<Event>.<variant>.json` (§2 naming). The fake agent reads `<fixtures>/<cli-version>/<Event>.<variant>.json`.

### test-data-fixtures#2.1 · lost at strong · .andromeda/test-plan-detail/test-data-fixtures.md:27
- (.andromeda/test-plan-detail/test-data-fixtures.md:27) The hook client's verification is tested end to end only through the fake agent running the real `hook` commands from the plugin dir. Those are absolute commands, spawned directly.
  > It reads the hook commands from `<plugin-dir>/hooks/hooks.json` (the `--plugin-dir` that `run` passes; Claude Code hooks shape) and runs every `type:"command"` entry for the event.

### test-data-fixtures#5.1 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:62
- (.andromeda/test-plan-detail/test-data-fixtures.md:62) Any fake-agent turn script this chunk adds (for example, one that holds a turn while the endpoint's server changes) must pass the hygiene walk's schema.
  > Today the walk (`tests/contract_fixture_hygiene.rs`) covers `fixtures/fake-scripts/*.json` against `schemas/fake-script.v1.json` (draft 2020-12, unknown fields allowed, `v` const 1, closed nine-event enum).

### test-scope-summary#6.1 · lost at classify · .andromeda/test-plan-detail/test-scope-summary.md:76
- (.andromeda/test-plan-detail/test-scope-summary.md:76) The snapshot fields the verification reads (`pid`, `started_at`, `endpoint`) are the harness `boot` readiness fields and must keep being written as they are now.
  > Readiness, per instance: `instances/<name>/snapshot.json` exists with `pid`, `started_at`, `child_pid` and `endpoint`.

### test-scope-summary#6.1 · lost at classify · .andromeda/test-plan-detail/test-scope-summary.md:92
- (.andromeda/test-plan-detail/test-scope-summary.md:92) The harness `cleanup` check (a CLI verb against a gone name exits 21) must still hold once the verb verifies the server. This touches CARRY 2's window where an exiting wrapper's pipe is still open.
  > Deletes the temp home and verifies the endpoint is gone: a CLI verb against the name returns exit 21, and the Unix socket file or pipe no longer exists.

### test-scope-summary#8.2 · lost at strong · .andromeda/test-plan-detail/test-scope-summary.md:126
- (.andromeda/test-plan-detail/test-scope-summary.md:126) Client-side refusals must still come "before any frame". Where verification sits relative to them in that pre-frame order has to be fixed and tested.
  > A text whose typed text is empty is refused `not-delivered`/`empty-text` (exit 13) right after `control-character` and before anything is issued, by the client before any frame and by the wrapper.

### test-strategy#1.7 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:29
- (.andromeda/test-plan-detail/test-strategy.md:29) The chunk must add no per-chunk mutation gate; mutation stays at the epoch boundary.
  > No per-chunk, pre-push or CI mutation gate.

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:8
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) The new tests that drive `viola` children (CLI verbs, hooks, `viola mcp` against a mismatched or verified server) must keep their profiles in the coverage run; a test that clears the env must re-add the profile variable.
  > Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates. Tests that call `env_clear()` must re-add it.

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:9
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) A new case that kills a wrapper so that the hook's or a verb's server check runs against a gone or replaced server must not reopen the refused-profile red; the fake agent's process-group discipline must hold for it.
  > A second mechanism is measured: a test that kills the wrapper closes its PTY, and the hangup reaches the hooks the fake agent ran in that terminal's foreground process group, so a `viola hook` cut off in its exit-time profile write leaves a page-cut profile that `llvm-profdata` refuses, or none;

### test-data-fixtures#1.2 · listed (lost at classify) · .andromeda/test-plan-detail/test-data-fixtures.md:11
- (.andromeda/test-plan-detail/test-data-fixtures.md:11) Hook-dialog and `answer` cases under the new check that need a non-`null` decision must get their stamp from `viola verify` against the fake agent. No new test or helper may write `stamps.json`.
  > `viola verify --home <home>` run against the fake agent. No test or harness code writes the file. | per-test (rstest `stamped_home`) or per harness session

### test-scope-summary#7.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-scope-summary.md:105
- (.andromeda/test-plan-detail/test-scope-summary.md:105) Each verification test must bootstrap its own fresh home.
  > Self-bootstrapping per test: a fresh `--home` temp dir.

### test-scope-summary#7.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-scope-summary.md:112
- (.andromeda/test-plan-detail/test-scope-summary.md:112) A mismatch case must not rely on developer-seeded state. The mismatched snapshot has to come from a real run's own bytes.
  > No developer-seeded state.

### test-strategy#2.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-strategy.md:41
- (.andromeda/test-plan-detail/test-strategy.md:41) New crate-level integration tests (for `viola-channel`'s server-identity read, `viola-state`'s checks, `viola-mcp`'s handlers) must follow the crate `tests/` path pattern.
  > **Crate-level integration tests:** `crates/viola-<area>/tests/<topic>.rs`.

### test-strategy#2.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-strategy.md:42
- (.andromeda/test-plan-detail/test-strategy.md:42) Root tests of the verified CLI and hook clients must use the shared `tests/support/` reader, wait and guard, never a copy, and read live-appended files only through the complete-lines reader.
  > No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct. `ndjson.rs` is the one complete-lines reader: every reader of a live-appended file parses only newline-terminated lines, because a Linux reader can observe an append part-way.

### test-strategy#2.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-strategy.md:43
- (.andromeda/test-plan-detail/test-strategy.md:43) A new E2E binary for the verification (an MCP or cross-surface case) must carry a prefix the E2E layer regex selects.
  > both homes report in the E2E layer, whose regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` (§3 5-command `run` step 2).

### test-strategy#2.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-strategy.md:44
- (.andromeda/test-plan-detail/test-strategy.md:44) Tests of the MCP Tokio client's verification belong where tokio is allowed; the root package and the sync crates must gain no tokio dev-dependency.
  > `crates/viola-e2e` hosts `viola-harness` and the tokio-based test clients (rmcp client), so the root package and the sync crates carry no tokio dev-dependency.

## Held

- ci-integration#1.1
- ci-integration#1.6
- ci-integration#1.8
- e2e-test-strategy#1.2
- e2e-test-strategy#1.4
- e2e-test-strategy#1.5
- e2e-test-strategy#11.1
- e2e-test-strategy#12.1
- e2e-test-strategy#13.1
- e2e-test-strategy#14.1
- e2e-test-strategy#15.1
- e2e-test-strategy#17.1
- e2e-test-strategy#2.1
- e2e-test-strategy#4.3
- integration-test-strategy#1.4
- integration-test-strategy#1.6
- integration-test-strategy#2.0
- integration-test-strategy#3.1
- integration-test-strategy#3.2
- integration-test-strategy#4.1
- mocking-stubbing-discipline#2.4
- quality-gates-coverage-targets#3.1
- test-data-fixtures#1.1
- test-scope-summary#1.1
- test-scope-summary#10.1
- test-scope-summary#3.1
- test-scope-summary#4.1
- test-scope-summary#5.2
- test-scope-summary#5.3
- test-scope-summary#8.1
- test-strategy#1.1
- test-strategy#1.2
- test-strategy#1.3
- test-strategy#1.5
- test-strategy#1.6
- test-strategy#3.1
- test-strategy#3.10
- test-strategy#3.12
- test-strategy#3.13
- test-strategy#3.16
- test-strategy#3.3
- test-strategy#3.4
- test-strategy#3.6
- unit-test-strategy#1.1
- unit-test-strategy#1.2
- unit-test-strategy#2.1

## Cascade only

- ci-integration#1.5
- e2e-test-strategy#1.1
- e2e-test-strategy#2.2
- e2e-test-strategy#3.1
- e2e-test-strategy#4.1
- e2e-test-strategy#4.2
- e2e-test-strategy#5.1
- e2e-test-strategy#6.1
- e2e-test-strategy#6.2
- e2e-test-strategy#6.3
- e2e-test-strategy#8.2
- mocking-stubbing-discipline#1.1
- mocking-stubbing-discipline#2.2
- test-data-fixtures#3.1
- test-strategy#3.11
- test-strategy#3.5
