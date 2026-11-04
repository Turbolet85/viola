@task_prose part=session
Chunk 2026-11-04-linux-and-macos-parity is test-side only: no file under src/, crates/*/src/, schemas/, plugin/, fixtures/, .github/, Cargo.toml or Cargo.lock moved, and no product symbol, IPC method, event, env var, port, socket, dependency or config key was added or changed.
It added the census contract binary tests/contract_os_gated_tests.rs (a GATES table of 59 rows, 52 OS-gated test functions and 7 gated files, each with a reason), the shared freeze guard tests/support/frozen.rs (moved whole out of tests/cli_instance_state.rs), and the operator-pass script scripts/junit-os-sets.py, which reads per-OS JUnit and never runs in CI.
It removed the Windows-only gate from run_child_ends_when_its_wrapper_is_terminated, which now runs on every OS (SIGKILL on Unix). It landed the v1-40 outer-PTY witness cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal on the Unix legs and added binary(run_cli) to the ci profile's 20 s kill line, so that line now names 27 binaries instead of 26.
Measured: macOS ps has no sigignore keyword, so the fake-agent hangup case reads ps -o sigignore= and stays Linux-only on the operator's word. The fake agent's SIGHUP-ignoring exit stays shared Unix code.
Measured: ci#41918694243 on cbdd3874 was green 15/15, with JUnit windows-2025 3386, macos-latest 3412, ubuntu-latest 3458. The 48 ubuntu-not-macos cases are 43 from the macOS no-peer-pid carve-out plus 5 Linux-only by a named reason.
Retired: the two local Frozen copies in tests/cli_instance_state.rs, and /proc/<pid>/status SigIgn as the hangup case's reader.
@task_file n=1 path=tests/support/frozen.rs
@task_file n=2 path=tests/contract_os_gated_tests.rs
@task_file n=3 path=scripts/junit-os-sets.py
@task_file n=4 path=tests/support/mod.rs
@task_file n=5 path=tests/cli_instance_state.rs
@task_file n=6 path=tests/cross_exit_causes.rs
@task_file n=7 path=tests/run_cli.rs
@task_file n=8 path=tests/cross_list_rows.rs
@task_file n=9 path=tests/cli_fake_agent.rs
@task_file n=10 path=tests/cli_list.rs
@task_file n=11 path=tests/cli_plugin_install.rs
@task_file n=12 path=tests/cli_verify.rs
@task_file n=13 path=.config/nextest.toml
@task_item n=1 part=touches
Infrastructure Patterns and the Repository list of occupied resources: scripts/junit-os-sets.py is a new repository script. The plan's expected amendment places it beside the repository's scripts as the operator pass's per-OS JUnit reading. It is never a CI step, a pre-push stage or a harness command.
@task_item n=2 part=touches
Cross-cutting Patterns, cross-platform discipline (every OS-specific branch compiles and is tested on its CI runner): this rule is now backed test-side by the census contract binary tests/contract_os_gated_tests.rs. Every OS-gated test function and gated file must be a GATES row with a reason, and no row may outlive its gate.
@task_item n=3 part=touches
Cross-cutting Patterns, bounded readers: tests/contract_bounded_readers.rs is no longer the only contract binary that censuses the tree. tests/contract_os_gated_tests.rs (andromeda:walks-tree) censuses every .rs file directly under tests/ and under each crates/*/tests/. Any text that names the contract binaries, or counts the census-style ones, has moved.
@task_item n=4 part=touches
The E2E layer's contract_ binaries: contract_os_gated_tests is a new auto-discovered root test binary with no [[test]] table. The contract_ regex selects it under run --e2e (suite nextest-e2e). Any count or list of the contract_ binaries moves by one.
@task_item n=5 part=touches
Root test support (tests/support/, with tests/support/home.rs named in the filesystem entries): tests/support/mod.rs gains pub mod frozen. The Frozen guard now lives only in tests/support/frozen.rs, where tests/cli_instance_state.rs used to hold it. It has four callers in two files.
@task_item n=6 part=touches
Test-only filesystem entry for the root test support's Watch (<temp dir>/viola-root-watch/<test name>.<label>.report, bound WITHIN = 7 s): on Unix, Frozen polls ps -o stat= until it reads T, under a Watch bounded by super::watch::WITHIN. That makes it a new user of this Watch and its report file. The root count of Instant::now() + WITHIN waits still reads 32.
@task_item n=7 part=touches
target/nextest/ci/junit.xml and the junit-<os> upload: the artifacts junit-windows-2025, junit-macos-latest and junit-ubuntu-latest now have a reader, the operator script. It downloads them with gh run download into a tempfile.mkdtemp dir outside the tree, removes that dir on exit, and writes nothing in the tree.
@task_item n=8 part=touches
The nextest ci profile's stamped-home 20 s kill line (.config/nextest.toml:24, slow-timeout period 10s, terminate-after 2) gains binary(run_cli) after binary(path1_run_start). It now names 27 binaries instead of 26, counted by the binary(...) names on that filter line.
@task_item n=9 part=touches
Instance snapshot pid and started_at, and the recorded macOS gap (macOS reports no peer pid): the per-OS reading counts 43 ubuntu-not-macos cases that come from this no-peer-pid carve-out, the same in ci#41339639367 and ci#41918694243. The gap itself is unchanged.
@task_item n=10 part=touches
Test-only binary viola-fake-agent: its exit still ignores SIGHUP on Unix (src/bin/viola-fake-agent.rs ignore_hangup, cfg(unix), unchanged). The case that reads this, fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits, stays cfg(target_os = linux). Its reader moved from /proc/<pid>/status SigIgn to ps -o sigignore=, because macOS ps has no keyword that prints the ignored-signal mask.
@task_item n=11 part=touches
Design Philosophy (cross-platform, Windows first) and the CI/CD decision (three-OS matrix against the fake agent): run_child_ends_when_its_wrapper_is_terminated has lost its Windows-only gate and runs on every OS. Unix ends the wrapper with kill -KILL, Windows uses OpenProcess and TerminateProcess, and both then check that the child is gone by pid and start time.
@task_item n=12 part=touches
CI evidence on three OSes: ci#41918694243 on cbdd3874 is green 15/15 (fuzz-replay, lint x3, msrv, perf x3, release x3, supply-chain, test x3). Its JUnit reads windows-2025 3386, macos-latest 3412, ubuntu-latest 3458, with 0 failed and 0 skipped. Any architecture text stating per-OS case counts or CI run ids has moved.
@task_item n=13 part=touches
Session liveness (a beat older than 5 s is stale) and the list payload liveness field: the new Unix witness freezes the wrapper through Frozen, back-dates its heartbeat 60 s and runs viola list under the outer PTY. stale is shown. This is the first end-to-end exercise of the stale row on a terminal; the rule is unchanged.
@task_item n=14 part=touches
TERM, read by every build's human output (src/human/style.rs read, a value containing 256color picks 256 colours): the witness runs viola list with TERM=xterm-256color. Its stream's SGR sequences are exactly DIM, BOLD, RESET, DIM, RESET: the stale row is dim from its first byte to its last, NAME is dim and bold, and BAY, the caption and the separator carry no SGR. The reading rule is unchanged.
@task_item n=15 part=touches
cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone (crates/viola-e2e/tests/harness_lifecycle.rs, cfg(unix)) runs on both Unix runners, not on ubuntu-latest alone. Both JUnit readings show this. Any architecture text that names it as ubuntu-latest only is stale.
@task_fact n=1 slug=occupied-resources
fact: scripts/junit-os-sets.py (python 3 stdlib, operator pass only, never CI) takes --run <id>, downloads the junit-windows-2025, junit-macos-latest and junit-ubuntu-latest artifacts with gh run download into a removed temp dir, and prints per-OS case, failed and skipped counts plus the sorted case lists of ubuntu-not-macos, macos-not-ubuntu, windows-not-ubuntu and ubuntu-not-windows; it exits 0, 1 for a missing or unparsable report, or 2 for usage, reads no environment variable and writes nothing in the tree.
@task_fact n=2 slug=cross-cutting-patterns
fact: The contract binary tests/contract_os_gated_tests.rs censuses OS-gated tests: a file with a leading #![cfg(...)], or a test function (#[test], #[rstest] or #[tokio::test]) whose contiguous attributes hold a #[cfg(...)] or #[cfg_attr(...)] naming windows, unix or target_os outside string values, must each be a GATES row with a non-empty reason, and a row whose gate no longer stands fails; it reads 59 rows (52 functions and 7 files) and self-checks with eight planted cases.
@task_fact n=3 slug=occupied-resources
fact: Test-only, root test support: tests/support/frozen.rs holds the Frozen guard, one twin per OS (Unix sends kill -STOP, polls ps -o stat= until T, and sends kill -CONT on drop; Windows attaches a debugger with kill-on-exit off and detaches on drop), the one shared way a test freezes a wrapper so its heartbeat goes stale.
@task_fact n=4 slug=established-decisions
fact: Measured at ci#41915621814: macOS ps has no sigignore keyword (its keywords include blocked, sigmask, pending and sig), so a process's ignored-signal mask has no ps reader on macOS, and a test that reads it is gated to Linux with that reason.
@task_item n=1 part=untouched
Stack and technology versions, dependencies and workspace members and features: Cargo.toml and Cargo.lock are unchanged, nothing was added or bumped, and the script uses only the python stdlib and the host's gh.
@task_item n=2 part=untouched
Channel methods, frames, the JSON-RPC envelope, refusal reasons, CLI exit codes, MCP results and the CLI --json documents: no product symbol, method or output shape moved.
@task_item n=3 part=untouched
IPC endpoint names, the socket-directory rule, peer-euid admission and the test-only channel endpoints: no endpoint or socket was added or changed.
@task_item n=4 part=untouched
Hook contract, hook transport, registered hook events, plugin scope and Claude Code integration names: plugin/ and the hook sources are unchanged.
@task_item n=5 part=untouched
Viola home layout (config.json, bin/, plugin/, marketplace/, ledger/, instances/, budget.json, diagnostics/) and the schemas under schemas/, including diag-line.v1.json and diag-detail.v1.json: no product path or schema moved, despite the chunk title's mention of one diagnostics schema.
@task_item n=6 part=untouched
Environment variables viola sets or reads, the AGENT_RUN_ names and the FAKE_AGENT_ test seams: no env var was added, and the new script reads none.
@task_item n=7 part=untouched
Error Handling census (nine thiserror derives over product sources, the open divergences) and module boundaries: no file under src/ or crates/*/src/ changed.
@task_item n=8 part=untouched
Wheel, delivery confirmation, budget governor, links, screen model, PTY seam, snapshot writer and the capability ledger decisions: none of these product behaviours changed.
@task_item n=9 part=untouched
fuzz/ workspace, vendor/conpty, proptest regressions, fixtures/ and the harness target dirs (target/agent-run, target/run-archive, target/mutants and the others): none was touched.
@task_item n=10 part=untouched
CI workflow jobs and steps (.github/ unchanged): the operator script runs from the plan's operator leg and is not a CI step.
