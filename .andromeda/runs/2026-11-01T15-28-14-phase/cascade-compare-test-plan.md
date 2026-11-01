# Cascade compare — test-plan · phase · 2026-11-01-unix-endpoint-and-home-hardening
E items 275 · held 43 · lost at classify 2 · lost at strong 3 · beside a placed fact - · cascade only 22 · listed 6 · body 155 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#1.12 · lost at strong · .andromeda/test-plan-detail/ci-integration.md:60
- (.andromeda/test-plan-detail/ci-integration.md:60) The pre-push gate must not be read as the chunk's macOS proof, since it runs on Linux only.
  > The local pre-push gate runs natively on the Linux dev host and installs nothing

### ci-integration#1.11 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:51
- (.andromeda/test-plan-detail/ci-integration.md:51) Any seam built to drive a refusal must stay out of the release build, which this check fails on a test-only feature or binary.
  > It prints `release-check: viola only` or fails on an artifact built with a test-only feature (`test-support`, `fake-agent`), on a test-only binary or on no executable

### ci-integration#1.2 · lost at strong · .andromeda/test-plan-detail/ci-integration.md:15
- (.andromeda/test-plan-detail/ci-integration.md:15) The euid check on the Tokio client must sit behind `viola-channel`'s `tokio` feature, so that the crate's normal graph still prints no tokio line.
  > then, in the same step `Tokio ban per sync crate (sole root)`, `bash scripts/channel-tokio-free.sh` (chunk 2026-10-17-mcp-server-for-drivers: `cargo tree -p viola-channel -e normal --target all -i tokio --prefix none` must print no `tokio v` line

### ci-integration#1.2 · lost at strong · .andromeda/test-plan-detail/ci-integration.md:15
- (.andromeda/test-plan-detail/ci-integration.md:15) Any dependency the chunk adds to read peer credentials or directory ownership must pass the deny check's bans, licences and sources.
  > ubuntu job `supply-chain`, fail-closed `shell: bash` steps: `cargo deny --format json check` (`deny.toml`: advisories, licences, sources, bans,

### ci-integration#1.9 · lost at strong · .andromeda/test-plan-detail/ci-integration.md:43
- (.andromeda/test-plan-detail/ci-integration.md:43) Whatever API the chunk uses for peer credentials and directory checks must compile and pass the unit suite on Rust 1.96.
  > ubuntu `msrv`: `rustup toolchain install 1.96 --profile minimal`, then `RUSTUP_TOOLCHAIN=1.96` steps for `rustc --version`, `cargo check --workspace` and `agent-run run --unit`, then `gate --require nextest-unit`;

### test-data-fixtures#1.2 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:11
- (.andromeda/test-plan-detail/test-data-fixtures.md:11) A board case that widens a home holding an instance must, where it runs on Windows x64, start that home unseeded, so its widening cannot reach other seeded homes through the shared companion.
  > so a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded (`run_refuses_a_widened_home_tree_before_its_log`).

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### integration-test-strategy#3.0 · listed (lost at classify) · .andromeda/test-plan-detail/integration-test-strategy.md:39
- (.andromeda/test-plan-detail/integration-test-strategy.md:39) Every new socket-directory, peer-euid and board-home case must run in its own fresh home at a path viola creates, unless it states why it needs a home made first.
  > there is a fresh tempfile home per test (the rstest `home` fixture). Like harness homes (§3 `boot` step 2), it is a not-yet-existing `home` path inside a `tempdir_in("<workspace root>/target/e2e-home")` parent, so viola creates it

### integration-test-strategy#3.0 · listed (lost at classify) · .andromeda/test-plan-detail/integration-test-strategy.md:39
- (.andromeda/test-plan-detail/integration-test-strategy.md:39) The not-a-symlink check belongs to the socket directory, so it must not refuse a home reached through the dev host's operator-set `target/e2e-home` link.
  > on the Linux dev host `target/e2e-home` is an operator-set symbolic link to the owner-only tmpfs directory `/tmp/viola-e2e-home-<uid>`, so the homes' bytes sit outside the working directory while every home class keeps its path, viola still creates the home itself

### integration-test-strategy#3.0 · listed (lost at classify) · .andromeda/test-plan-detail/integration-test-strategy.md:40
- (.andromeda/test-plan-detail/integration-test-strategy.md:40) A case driving the board's home check or another check-guarded path on every OS must start from this fixture rather than a home that the one-rule check would refuse.
  > A case that drives a check-guarded path on every OS starts from `stamped_home` (§7 Seed strategies).

### integration-test-strategy#3.0 · listed (lost at classify) · .andromeda/test-plan-detail/integration-test-strategy.md:40
- (.andromeda/test-plan-detail/integration-test-strategy.md:40) Because a killed wrapper leaves its socket file, this kill-then-wait guard must leave no leftover or loosened per-user directory shared with another test; a test that loosens or replaces the directory needs its own per-test location.
  > Wrappers started in a test are held in a `Drop` guard that calls `Child::kill()` then `wait()`. Nothing is shared between tests.

### integration-test-strategy#3.0 · listed (lost at classify) · .andromeda/test-plan-detail/integration-test-strategy.md:41
- (.andromeda/test-plan-detail/integration-test-strategy.md:41) The bare temp dir's 0755 reading must hold for the board once its home check is one rule, so a `viola list` case needing an accepted empty home must use a viola-made one.
  > a bare temp dir reads 0755 under the umask and the read rule refuses it; `tool_inputs_that_pass_reach_the_liveness_check` reaches the check it names from a viola-made `empty_home` and reads `unwrapped-name`.

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:8
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) Every test this chunk adds that clears a spawned `viola`'s environment must re-add the profile name, so the refusals it asserts stay in the coverage number.
  > Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates. Tests that call `env_clear()` must re-add it.

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:8
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) A case this chunk adds that kills an instrumented child (the leftover-file takeover, the force-killed wrapper) must not land the kill in the child's exit-time profile write, or the merge of the coverage gate goes red.
  > One mechanism for such a profile is measured (as measured at chunk 2026-10-07-a-send-ending-in-a-newline-is-confirmed, `evidence/profraw-red-green.md`): an instrumented child that exits by itself and is killed during its exit-time profile write leaves a short profile, which `llvm-profdata` refuses.

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:9
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) A kill-shaped case this chunk adds must run its fake agent's commands in process groups of their own, so a wrapper kill's hangup cuts no `viola hook` profile.
  > A second mechanism is measured: a test that kills the wrapper closes its PTY, and the hangup reaches the hooks the fake agent ran in that terminal's foreground process group, so a `viola hook` cut off in its exit-time profile write leaves a page-cut profile that `llvm-profdata` refuses, or none; with every command the fake agent spawns in a process group of its own (the fake agent's exit bullet, §7) the census over the kill-shaped `chaos_wait_parked_wrapper_killed_exits_21` reads 0 refused profiles of 800 in 200 runs (as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/evidence/profraw-cause.md`).

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:9
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) The refused-profile count of a new kill-shaped case is read with this dev-host witness, and the chunk must wire it into no CI job, `pre-push` stage or harness command.
  > The witness is `scripts/profraw-census.sh <runs> <workers> [<name>]`, with a second form `--case <binary stem> <test name> <runs> <workers> [<name>]` over one integration case that offers each profile alone to `llvm-profdata merge` and counts the refused ones, a dev-host script (architecture §Occupied Resources → Repository): no CI job, `pre-push` stage or harness command runs it, and it has no unit test, only its recorded one-shot controls (must-fail on the untouched test, must-pass, and two planted readings).

### quality-gates-coverage-targets#2.0 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:15
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) This chunk opens Epoch 7 and must add no mutation gate of its own; mutation stays out of its gate set.
  > **Mutation gate (Founder Direction 1):** judged at the epoch boundary by `/andromeda-code-audit` and on demand by `run --mutants`; no chunk, pre-push or CI mutation gate exists since 2026-09-28 (the founder's ruling, 17:59).

### quality-gates-coverage-targets#2.0 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) A mutant in this chunk's macOS-only or otherwise host-unreachable code, on any `run --mutants`, must be recorded as owed by coordinate, never read as equivalent or caught.
  > A mutant this host cannot compile or reach — inside a `cfg(windows)` item or block, or behind a const such as `HOST_SCRATCH = cfg!(windows)` — is recorded "not measured here; owed to {the route entry that measures it}" by coordinate, never "equivalent", and never counted as caught; `missed == 0` then reads over the measurable set (the overseer's ruling, 2026-10-04).

### test-data-fixtures#2.3 · listed (lost at strong) · .andromeda/test-plan-detail/test-data-fixtures.md:32
- (.andromeda/test-plan-detail/test-data-fixtures.md:32) A home this chunk's tests make before viola, to pass the read rule, must be made through `create_private_dir`, so the board's one rule refuses only the home a case widened on purpose.
  > A home a test makes before viola, where the case needs it to pass the strict-modes read rule, is made through `viola_state::fs::create_private_dir`, never `create_dir_all` (a `create_dir_all` home is refused, On-disk product state above), as `tests/contract_diag_schema.rs`, `tests/cli_machine_contract.rs` and `tests/tui_env_strip.rs` do.

### test-scope-summary#7.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-scope-summary.md:122
- (.andromeda/test-plan-detail/test-scope-summary.md:122) A test that makes a home or directory ahead of viola must use the private maker, and only a deliberate widening may make a directory the checks refuse.
  > A test that makes a home before viola runs makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`: the instance and home-tree checks refuse a `create_dir_all` home (Unix: its 0755 mode fails the read rule;

### test-strategy#2.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-strategy.md:42
- (.andromeda/test-plan-detail/test-strategy.md:42) A test that loosens the socket directory or a home path must use the shared helpers and hold no copy of its own outside `tests/support/`.
  > `home.rs` holds the home-widening helpers `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted), through which a root test widens or read-grants a home path, restores it and reads whether it is still widened. No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct.

### test-strategy#2.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-strategy.md:43
- (.andromeda/test-plan-detail/test-strategy.md:43) The killed-wrapper and leftover-socket takeover cases must start from the shared start helpers, `kill_wrapper` among them.
  > `tests/support/start.rs` shares the start setups across root test files: `first_life`, `wait_session_start_hooks`, `made_dir`, `stopped_life`, `remove_dir_once_free`, `plant_agents`, `listed`, `revive` and `kill_wrapper`.

### test-strategy#2.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-strategy.md:44
- (.andromeda/test-plan-detail/test-strategy.md:44) The new refusal rows land in this `cross_` binary, which must stay fake-agent gated and selected by the E2E layer's regex.
  > a `cross_` binary (`tests/cross_exit_causes.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`); both homes report in the E2E layer, whose regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`

## Held

- ci-integration#1.1
- ci-integration#1.5
- ci-integration#1.6
- ci-integration#1.8
- e2e-test-strategy#12.1
- e2e-test-strategy#13.1
- e2e-test-strategy#14.1
- e2e-test-strategy#15.1
- e2e-test-strategy#19.1
- e2e-test-strategy#2.2
- e2e-test-strategy#4.3
- e2e-test-strategy#4.4
- integration-test-strategy#1.4
- integration-test-strategy#4.1
- integration-test-strategy#4.2
- integration-test-strategy#5.1
- mocking-stubbing-discipline#2.3
- mocking-stubbing-discipline#2.4
- mocking-stubbing-discipline#2.5
- mocking-stubbing-discipline#2.6
- quality-gates-coverage-targets#3.1
- test-data-fixtures#1.1
- test-data-fixtures#1.10
- test-data-fixtures#1.6
- test-data-fixtures#1.7
- test-data-fixtures#2.1
- test-data-fixtures#2.2
- test-data-fixtures#6.1
- test-scope-summary#1.1
- test-scope-summary#10.1
- test-scope-summary#3.1
- test-scope-summary#4.1
- test-scope-summary#5.3
- test-scope-summary#6.1
- test-scope-summary#8.1
- test-strategy#1.1
- test-strategy#1.7
- test-strategy#3.1
- test-strategy#3.6
- unit-test-strategy#1.1
- unit-test-strategy#1.2
- unit-test-strategy#1.3
- unit-test-strategy#2.1

## Cascade only

- e2e-test-strategy#1.1
- e2e-test-strategy#1.2
- e2e-test-strategy#1.4
- e2e-test-strategy#1.5
- e2e-test-strategy#17.1
- e2e-test-strategy#2.1
- e2e-test-strategy#2.3
- e2e-test-strategy#4.1
- e2e-test-strategy#4.2
- integration-test-strategy#1.6
- test-data-fixtures#1.15
- test-scope-summary#5.2
- test-strategy#1.3
- test-strategy#1.5
- test-strategy#1.6
- test-strategy#3.10
- test-strategy#3.11
- test-strategy#3.12
- test-strategy#3.13
- test-strategy#3.16
- test-strategy#3.3
- test-strategy#3.5
