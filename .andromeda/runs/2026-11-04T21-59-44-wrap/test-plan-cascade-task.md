@task_prose part=session
Chunk 2026-11-04-linux-and-macos-parity changed test-side code only: no file under src/, crates/*/src/, schemas/, plugin/, fixtures/, .github/, Cargo.toml or Cargo.lock moved, and no product symbol, harness verb, status shape or CI step changed.
It moved the Frozen freeze guard out of tests/cli_instance_state.rs into a shared tests/support/frozen.rs, which is now its only home.
It added the outer-PTY witness of the stale row's dim span (cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal, Unix), took the Windows-only gate off run_child_ends_when_its_wrapper_is_terminated (every OS; SIGKILL on Unix), and added a census contract binary, contract_os_gated_tests, that names every OS-gated test with its reason.
It changed how the hangup case reads the signal mask: it now uses ps -o sigignore= in place of /proc SigIgn. The case was widened to cfg(unix) and then narrowed back to Linux only, because macOS ps has no keyword that prints the ignored-signal mask (measured at ci#41915621814).
It added run_cli to the ci profile's 20 s kill line (26 to 27 binaries), added scripts/junit-os-sets.py as an operator-pass reader of the per-OS JUnit, and recorded the census count (52 functions and 7 files) and the ubuntu-not-macOS split (43 + 5).
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
The Frozen guard's home moved from tests/cli_instance_state.rs to tests/support/frozen.rs. Any sentence that names tests/cli_instance_state.rs as the home of Frozen is now stale. One such sentence is in the integration strategy: Frozen is a guard on Windows and on Unix. Another is in the mocking discipline: under the Frozen guard, on Unix a SIGSTOP.
@task_item n=2 part=touches
Sentences that name the Frozen guard without naming a file stay true. Examples are the e2e and chaos text on the Frozen guard and the SIGSTOPped wrapper read stale. Frozen now has four callers in two files: three in tests/cli_instance_state.rs and the new Unix witness in tests/cross_list_rows.rs.
@task_item n=3 part=touches
cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone (crates/viola-e2e/tests/harness_lifecycle.rs, cfg(unix)) runs on both Unix runners. Any sentence that places it on ubuntu-latest alone is now stale.
@task_item n=4 part=touches
The v1-40 outer-PTY witness was owed. The e2e strategy says it is a CARRY on the Linux and macOS parity entry. It has now landed as cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal on the Unix legs (ubuntu-latest and macos-latest), so that CARRY sentence is discharged.
@task_item n=5 part=touches
run_child_ends_when_its_wrapper_is_terminated in tests/run_cli.rs no longer has a cfg(windows) gate and runs on every OS. Any text that places it, or the wrapper-terminated child cleanup, on Windows only is now stale.
@task_item n=6 part=touches
fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits (tests/cli_fake_agent.rs) is still Linux only. Its reader moved from /proc/<pid>/status SigIgn to ps -o sigignore=. The fake agent's text saying that on Unix its exit ignores SIGHUP is about product behaviour, and it stays true. Text that names this case's reader or its OS reach must read: Linux only, through ps.
@task_item n=7 part=touches
The ci profile's 20 s kill line (.config/nextest.toml:24) gained binary(run_cli) after binary(path1_run_start), so it now names 27 binaries. The e2e text that names other binaries on that kill line states no count and stays true. Any text that counts the line or lists its binaries moves.
@task_item n=8 part=touches
The contract suite (contract_*.rs) has a new root binary, tests/contract_os_gated_tests.rs. It sits beside contract_bounded_readers and follows the same table, census and self-check pattern. Text that lists the contract binaries or the census-style enumerations now has a second member.
@task_item n=9 part=touches
The list row integrity suite (tests/cross_list_rows.rs) now also holds a stale-row dim witness under the outer PTY. Before, it was described as escaping and column alignment only.
@task_item n=10 part=touches
The directory convention says root tests share one set of helpers in tests/support/ and no root test file copies them. That convention now covers the freeze guard: tests/support/mod.rs declares pub mod frozen, and the two local Frozen copies in tests/cli_instance_state.rs are gone.
@task_item n=11 part=touches
The multi-os-compat trigger and the OS-branch test claims are now held by a census. Examples of such claims: security_negatives_server_identity on Linux and Windows, the macOS no-peer-pid carve-out, security_negatives_socket_dir on Linux and macOS, and the hook fail-open cases on Linux and Windows. Every OS-gated test function and gated file under tests/ and crates/*/tests/ is a GATES row with a reason, and an unnamed or stale gate fails the run.
@task_item n=12 part=touches
Ten OS-gated tests gained a reason line, including the cross_exit_causes cases strict_modes_on_the_board_is_exit_21_and_opens_no_log and strict_modes_on_plugin_install_is_exit_21_and_creates_nothing. Their reasons name a widening by Unix mode bits. The top comment of tests/cross_exit_causes.rs at line 12 now names the guard's new home. The exit-cause matrix text that points at that top comment may need a re-read.
@task_item n=13 part=touches
The CI per-OS junit artifacts (junit-windows-2025, junit-macos-latest, junit-ubuntu-latest) now have an operator-pass reader, scripts/junit-os-sets.py, which uses gh run download. CI itself does not run it.
@task_fact n=1 slug=e2e-test-strategy
fact: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal (tests/cross_list_rows.rs, cfg(unix), on booted_wrapper) is v1-40's integration-tier witness. It freezes the wrapper through Frozen, back-dates its heartbeat 60 s, and runs viola list under the outer PTY with TERM=xterm-256color. Its escape sequences must be exactly [DIM, BOLD, RESET, DIM, RESET]: the stale row is dim from first byte to last, NAME is dim and bold, and BAY, caption and separator carry no SGR. stale must be shown. It passes on ubuntu-latest and macos-latest.
@task_fact n=2 slug=e2e-test-strategy
fact: run_child_ends_when_its_wrapper_is_terminated (tests/run_cli.rs) runs on every OS. On Windows it terminates the wrapper with OpenProcess and TerminateProcess; on Unix with kill -KILL of the wrapper pid, its exit asserted. It then asserts that the child, read by pid and start time from snapshot_data, is gone within WITHIN.
@task_fact n=3 slug=e2e-test-strategy
fact: fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits (tests/cli_fake_agent.rs) is cfg(target_os = linux). It reads the agent's SIGHUP disposition through ps -o sigignore= -p <pid>, and its failure carries ps's exit and both streams. It does not run on macos-latest because macOS ps has no keyword that prints the ignored-signal mask (measured at ci#41915621814). That gate was the operator's ruling. The fake agent's SIGHUP-ignoring exit stays shared Unix code and is witnessed on Linux only.
@task_fact n=4 slug=e2e-test-strategy
fact: tests/contract_os_gated_tests.rs is the census of OS-gated tests. Its GATES table has a file, test, gate and reason for each row. The census walks every .rs directly under tests/ and in each crates/*/tests/. It counts a leading #![cfg(...)], and a test function whose attribute run holds #[test], #[rstest] or #[tokio::test] together with a cfg or cfg_attr whose predicate names windows, unix or target_os outside string values. Its faults check works both ways: an unnamed gate, and a row that no longer stands. Its tests are os_gated_tests_are_each_named_with_their_reason, gates_table_rows_are_unique and census_finds_an_unnamed_gate_and_passes_a_named_one, the last a self-check over eight planted cases.
@task_fact n=5 slug=e2e-test-strategy
fact: The OS-gated census reads 52 OS-gated test functions and 7 gated files, which make 59 GATES rows. The count includes cfg_attr case rows, for example the two functions in tests/hook_fail_open.rs.
@task_fact n=6 slug=test-strategy
fact: contract_os_gated_tests is a root contract_ test binary. It is auto-discovered with no [[test]] table and is selected by the E2E layer's regex, so it runs under run --e2e in suite nextest-e2e.
@task_fact n=7 slug=ci-integration
fact: The ci profile's stamped-home override filter in .config/nextest.toml:24 is the 20 s kill: slow-timeout with period 10s and terminate-after 2. It names 27 binaries, binary(run_cli) among them, placed after binary(path1_run_start). The count is the number of binary(...) names on that filter line.
@task_fact n=8 slug=ci-integration
fact: scripts/junit-os-sets.py --run <id> is an operator-pass instrument and never a CI step. It uses python 3 stdlib only. It downloads junit-windows-2025, junit-macos-latest and junit-ubuntu-latest with gh run download into a temp dir removed on exit. It prints each OS's cases, failed and skipped counts, then the sorted classname::name sets for ubuntu-not-macos, macos-not-ubuntu, windows-not-ubuntu and ubuntu-not-windows. It exits 0, 1 (a report missing or unparsable, named) or 2 (usage).
@task_fact n=9 slug=ci-integration
fact: The per-OS JUnit at ci#41918694243 (cbdd3874, green 15/15) reads windows-2025 3386, macos-latest 3412 and ubuntu-latest 3458 cases, 0 failed and 0 skipped. Its set differences are ubuntu-not-macos 48, macos-not-ubuntu 2, windows-not-ubuntu 88 and ubuntu-not-windows 160. The 48 split as 43 cases of the macOS no-peer-pid carve-out plus 5 Linux-only cases, each with a named reason.
@task_item n=1 part=untouched
Section 10 is untouched: the coverage thresholds, the ignore-filename regex, the mutation gate and the hyperfine perf budgets. No gate threshold, gate suite or perf row changed.
@task_item n=2 part=untouched
The section 3 harness contract is untouched: agent-run verbs, the session record, exit codes, the status shape and gate suites. No harness subcommand, status shape or CI step changed.
@task_item n=3 part=untouched
Section 4 unit strategy and the pyramid's unit row are untouched, including the unit cases compiled out on macOS. No file under src/ or crates/*/src/ moved, and the census reads only tests/ and crates/*/tests/, not inline unit modules.
@task_item n=4 part=untouched
The fake agent's product behaviour is unchanged: its Unix exit ignores SIGHUP from its first step, its hooks run in their own process group, and its modes, receipts and scripts are as before. src/bin/viola-fake-agent.rs did not change, and the standing description stays true.
@task_item n=5 part=untouched
Section 7 is untouched: fixtures, fixture hygiene, recorded payloads, seed strategies and the test data lifecycle. No fixture or schema moved.
@task_item n=6 part=untouched
The section 9 workflow layout is untouched: the OS matrix, job list, Action SHA pins, artifact uploads, secret scan and build failure conditions. Nothing under .github/ changed.
@task_item n=7 part=untouched
The critical path scenarios 1 to 7 and E1, E2, E3 and E5 are untouched, along with their verification signals. No product behaviour changed.
@task_item n=8 part=untouched
Section 1 untestable zones and the test tier are untouched: macOS local runs CI-only, cross-OS-user peers, Windows DACL stubs and the real claude CLI kept out of CI.
@task_item n=9 part=untouched
The security vector triggers are untouched (IPC, paste injection, hook stdin, MCP, CLI and env, filesystem, child spawn and supply chain), and so are the property suite and section 11 anti-patterns. No negative case, property or ban changed.
