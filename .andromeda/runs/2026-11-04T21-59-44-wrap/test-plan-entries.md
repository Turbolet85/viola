@amendment marker=2026-11-04-linux-and-macos-parity
title: the ci kill line at 27 binaries
section: §9 CI Integration (ci-integration.md: the `ci` kill line); §3 Bootstrap phases (key file bootstrap-phases-derive-for-route-setup-project: `[profile.ci]`'s stamped-home override)
change:
  - §9 gains the `ci` kill line: the `ci` profile's stamped-home override filter (`.config/nextest.toml:24`) is the 20 s kill (`slow-timeout = { period = "10s", terminate-after = 2 }`) over 27 binaries, the count's rule the `binary(…)` names on that filter line (the list in §3 Bootstrap phases, `test-runner-install`).
  - §3's override reads the twenty-seven verify-driven binaries (was twenty-six); `run_cli` joins the `binary(…)` list between `path1_run_start` and `security_negatives_socket_dir`, and the joined notes read that `run_cli` (`tests/run_cli.rs`) stands on it too. The 20 s kill and the count's rule are unchanged.
why: the report's Schema / config: `.config/nextest.toml:24` gains `binary(run_cli)` after `binary(path1_run_start)`, 26 → 27 binaries by that rule; its Outcome reads the kill line naming `binary(run_cli)`, 27 binaries, met. The key file stated the count in words, the one master site of it.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
@amendment marker=2026-11-04-linux-and-macos-parity
title: the per-OS JUnit reader and its reading
section: §9 CI Integration (ci-integration.md: the per-OS JUnit reader, the per-OS JUnit reading)
change:
  - New: `scripts/junit-os-sets.py --run <id>` (python 3 standard library and the host's `gh`) is the operator pass's per-OS JUnit reading, never a CI step: it downloads `junit-windows-2025`, `junit-macos-latest` and `junit-ubuntu-latest` with `gh run download` into a temporary directory removed on exit, prints `{os}: {n} cases · {f} failed · {s} skipped` per OS, then each set's count and sorted `classname::name` lines for `ubuntu-not-macos`, `macos-not-ubuntu`, `windows-not-ubuntu` and `ubuntu-not-windows`; exits 0, 1 (a report missing or unparsable, named) or 2 (usage); reads no environment variable, writes nothing in the tree.
  - New: the reading at ci#41918694243 (`cbdd3874`, green, checks 15/15): `windows-2025` 3386 · `macos-latest` 3412 · `ubuntu-latest` 3458 cases, 0 failed, 0 skipped; `ubuntu-not-macos` 48 (43 of the macOS no-peer-pid carve-out, 5 Linux-only, each by a named reason) · `macos-not-ubuntu` 2 · `windows-not-ubuntu` 88 · `ubuntu-not-windows` 160.
why: the report adds the script as an operator-pass instrument, never run by CI, and records the final pushed sha's per-OS JUnit counts; the ubuntu-not-macOS split counted case by case is 43 + 5, research's 44 + 4 a miscount.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
@amendment marker=2026-11-04-linux-and-macos-parity
title: the v1-40 stale-row dim witness landed
section: §6 E2E Test Strategy (e2e-test-strategy.md: the cli row; Scenario E5, the stale-row dim witness); §1 Test Scope Summary (test-scope-summary.md: the cli surface's Notes)
change:
  - §6 cli row: dim `stale` reads at the unit tier (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`) and under the outer PTY on `ubuntu-latest` and `macos-latest` (`cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal`, `tests/cross_list_rows.rs`, `cfg(unix)`; `list` row integrity); was at the unit tier only, its outer-PTY witness a CARRY on "Linux and macOS parity".
  - §6 E5 gains the witness: `cfg(unix)`, on `booted_wrapper`, `v1-40`'s integration-tier witness; it freezes the wrapper through `Frozen`, back-dates its `heartbeat` 60 s and runs `viola list` under the outer PTY with `TERM=xterm-256color`; the escape sequences are exactly `[DIM, BOLD, RESET, DIM, RESET]` (the stale row dim first byte to last, NAME dim and bold, no SGR on BAY, caption or separator), `stale` shown; green on `ubuntu-latest` and `macos-latest` (ci#41918694243).
  - §1 cli Notes name the same witness under the outer PTY on Unix.
why: the report lands the case as `v1-40`'s integration-tier witness and its Outcome reads it passing on both Unix runners at ci#41918694243; the CARRY it discharges was owed to this chunk.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
@amendment marker=2026-11-04-linux-and-macos-parity
title: a terminated wrapper's child on every OS
section: §6 E2E Test Strategy (e2e-test-strategy.md: Scenario Path 3, a terminated wrapper's child)
change:
  - New: `tests/run_cli.rs` `run_child_ends_when_its_wrapper_is_terminated`, on every OS, terminates the wrapper (Windows `OpenProcess` + `TerminateProcess`; Unix `kill -KILL <wrapper pid>`, its exit asserted) and asserts that the child, its pid and start time read from `snapshot_data`, is gone within `WITHIN`.
why: the report: the case's `#[cfg(windows)]` gate is removed, the terminate step written per OS, and it passes on the three OSes on the pushed sha (ci#41918694243).
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
@amendment marker=2026-11-04-linux-and-macos-parity
title: the hangup case on Linux only
section: §6 E2E Test Strategy (e2e-test-strategy.md: Scenario E5, a hangup landing while the fake agent exits)
change:
  - New: `tests/cli_fake_agent.rs` `fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits` is `cfg(target_os = "linux")`; it reads the agent's SIGHUP disposition through `ps -o sigignore= -p <pid>` (`ignores_sighup`), a failed read carrying `ps`'s exit and both streams; it does not run on `macos-latest`, so the fake agent's SIGHUP-ignoring exit, shared Unix code, is witnessed on Linux only (§7 Fake agent).
why: measured at ci#41915621814: macOS's `ps` has no keyword that prints the ignored-signal mask, which disproves the plan's reading on both Unix OSes; the gate is the overseer's technical answer (founder-delegated) to the implement dialog, as the operator's wrap directive relays it. The reader replaces `/proc/<pid>/status`'s `SigIgn`.
kept: §7 Fake agent's statement that on Unix the fake agent's exit ignores SIGHUP stands: the product behaviour is unchanged.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
@amendment marker=2026-11-04-linux-and-macos-parity
title: the census of OS-gated tests
section: §6 E2E Test Strategy (e2e-test-strategy.md: Scenario E5, the census and its count); §2 Test Strategy (test-strategy.md: root `contract_` binaries; the multi-os-compat row)
change:
  - §6 gains the census, `tests/contract_os_gated_tests.rs` (a `contract_` binary, `run --e2e`, carrying the `andromeda:walks-tree` token): its `GATES` table holds `file`, `test`, `gate` and `reason` per row; it walks every `.rs` directly under `tests/` and in each `crates/*/tests/`, counting a leading `#![cfg(…)]` and a test function whose contiguous attribute run holds `#[test]`, `#[rstest]` or `#[tokio::test]` and a `#[cfg(…)]` or `#[cfg_attr(…)]` naming `windows`, `unix` or `target_os` outside its string values; `faults` reads both ways (an unnamed gate, a row that no longer stands). Tests: `os_gated_tests_are_each_named_with_their_reason`, `gates_table_rows_are_unique`, the self-check `census_finds_an_unnamed_gate_and_passes_a_named_one` (eight planted cases).
  - §6 gains the count: 52 OS-gated test functions and 7 gated files, 59 `GATES` rows (rule: the rows of `GATES`), the rule taking `tests/hook_fail_open.rs`'s two `cfg_attr` case rows.
  - §2: the binary is a root `contract_` binary, auto-discovered with no `[[test]]` table, selected by the E2E layer's regex, so it runs under `run --e2e` in suite `nextest-e2e`.
  - §2 multi-os-compat row: was `CI matrix` · `§9 Matrix builds`; now `CI matrix + contract` · `§9 Matrix builds; §6 Contract suite`, an unnamed or stale gate failing the run.
why: the report adds the census and its Outcome reads it met on three OSes (ci#41918694243); the plan predicted 50 functions, its own rule taking the `cfg_attr` case rows.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
@amendment marker=2026-11-04-linux-and-macos-parity
title: the force-killed-wrapper cleanup on both Unix runners
section: §5 Integration Test Strategy (integration-test-strategy.md: Harness `cleanup` witnesses)
change:
  - `cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone` (`crates/viola-e2e/tests/harness_lifecycle.rs`, `#[cfg(unix)]`) reads on both Unix runners, `ubuntu-latest` and `macos-latest`; was on `ubuntu-latest`.
why: the report: the case is `#[cfg(unix)]` and is read in both of this chunk's JUnit readings, in neither `ubuntu-not-macos` set.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
@amendment marker=2026-11-04-linux-and-macos-parity
title: the Frozen guard's one home
section: §5 Integration Test Strategy (integration-test-strategy.md: `Frozen`); §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md: Process faults); §2 Test Strategy (test-strategy.md: Sync (tokio-free) E2E suites)
change:
  - §5: `Frozen` lives in `tests/support/frozen.rs`, declared by `pub mod frozen;` in `tests/support/mod.rs`, its only home (was `tests/cli_instance_state.rs`); the cases on it are `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` (every OS), `run_refuses_a_stale_name` (Unix), `run_refuses_a_frozen_wrapper_as_stale_on_windows` (Windows) in `tests/cli_instance_state.rs`, and `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (Unix) in `tests/cross_list_rows.rs` (was two cases).
  - §8 Process faults: the `Frozen` guard is `tests/support/frozen.rs`, its sole home, shared through `tests/support/mod.rs` (was `tests/cli_instance_state.rs`).
  - §2: the helper set reads `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs,frozen.rs}`; `frozen.rs` holds `Frozen` (`attach(pid)` and `Drop`): on Unix `kill -STOP`, `ps -o stat=` polled until `T` within `WITHIN`, `kill -CONT` on drop; on Windows a debugger attach, detached on drop; the no-copy rule names the freeze guard too.
why: the report retires the two local `Frozen` twins in `tests/cli_instance_state.rs`, moved whole to `tests/support/frozen.rs`, its sole home, four callers in two files.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
@amendment marker=2026-11-04-linux-and-macos-parity
title: the root waits' count at 32 sites in 24 files
section: §3 5-command implementation (key file 5-command-implementation: Cleanup, `booted_wrapper`'s `WITHIN`)
change:
  - `Instant::now() + WITHIN` reads 32 sites in 24 files under `tests/` (`grep -rn 'Instant::now() + WITHIN' tests/ | wc -l` and `grep -rln 'Instant::now() + WITHIN' tests/ | wc -l`), as measured at chunk 2026-11-04-linux-and-macos-parity's wrap on the Linux dev host; was 28 sites in 20 files, as measured at chunk 2026-10-10-budget-governor's wrap.
why: the report's gate reads 32 sites by its rule, unchanged within the chunk; the 28-in-20 reading predated it. The file count is this wrap's own reading by the `grep -rln` rule, the report giving none.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
