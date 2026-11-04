@task_prose part=session
A test-only chunk (Linux and macOS parity): no file under src/, crates/*/src/, schemas/, plugin/, fixtures/ or .github/ moved, and no product symbol, CLI copy, SGR rule, exit code or refusal changed.
It landed the v1-40 outer-PTY witness of dim only on stale rows: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal in tests/cross_list_rows.rs, cfg(unix), green on ubuntu-latest and macos-latest (ci#41918694243), not run on windows-2025.
It added tests/contract_os_gated_tests.rs, a census in the E2E layer (run --e2e, suite nextest-e2e, no new suite value) that names every OS-gated test with its reason: 52 functions and 7 files, 59 GATES rows. It also added a reason line to each of ten OS-gated tests, among them Unix-mode refusal cases in cli_list, cli_verify, cli_plugin_install and cross_exit_causes.
It moved the Frozen guard into tests/support/frozen.rs, made run_child_ends_when_its_wrapper_is_terminated run on every OS, kept the fake agent hangup case Linux-only with a ps -o sigignore= reader, and put binary(run_cli) on the ci profile's 20 s kill line (26 to 27 binaries).
It measured per-OS JUnit with the operator-only scripts/junit-os-sets.py: ubuntu-not-macos 48 = 43 from the macOS no-peer-pid carve-out + 5 Linux-only by a named reason.
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
The Owed line under section 4 P6 (a11y-plan.md, fct-lchghj7u) says dim only on stale rows is proven at the unit tier only and its outer-PTY witness is owed to Linux and macOS parity. That witness has now landed as cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal, so the line is stale and the debt is discharged.
@task_item n=2 part=touches
The path-p6-board procedure in aria-patterns-roles (fct-uok334yy) says its outer-PTY witness is a CARRY on Linux and macOS parity. That CARRY is discharged: the witness passes on ubuntu-latest and macos-latest.
@task_item n=3 part=touches
The visual-sgr values in visual-design-verification (fct-es7le4as) say the outer-PTY witness is a CARRY on Linux and macOS parity. That CARRY is discharged by the same test.
@task_item n=4 part=touches
The rule of dim SGR on stale rows and bold SGR only on NAME values, always beside the word (section 6 CLI equivalent, visual-sgr), now has an integration-tier witness. It asserts the stream's escape sequences exactly: DIM, BOLD, RESET, DIM, RESET. The stale row is dim from its first byte to its last, NAME is dim and bold, BAY, the caption and the separator carry no SGR, and the word stale is shown.
@task_item n=5 part=touches
The OS-leg qualifiers that P6 and the per-surface tool reach give the portable-pty outer-PTY SGR checks (all three OS legs) need an exception for the stale-row dim witness. It is cfg(unix): it runs on ubuntu-latest and macos-latest only, not on windows-2025.
@task_item n=6 part=touches
The tier wording in path-p6-board and aria-patterns-roles, unit tier against outer-PTY or integration tier for dim only on stale rows, moves from unit-only to unit plus integration tier on the Unix legs.
@task_item n=7 part=touches
Indirect, through the census in tests/contract_os_gated_tests.rs: in cognitive-errors, the Verification qualifier says the frame verbs' server-verify peer pid witness runs on windows-2025 and ubuntu-latest only because macOS reports no peer pid. That gate now carries a named NO_PEER_PID reason in GATES, and the per-OS reading counts 43 ubuntu-not-macos cases under that carve-out. On re-read the claim is unmoved.
@task_item n=8 part=touches
Indirect, through the census: in cognitive-errors, the qualifier that the socket-directory refusal runs on ubuntu-latest and macos-latest names a Unix-gated case. Every such gate now stands as a GATES row with a reason. On re-read the claim is unmoved.
@task_item n=9 part=touches
Indirect, through the new reason lines: the Unix-mode refusal cases (list_refuses_an_instance_another_user_could_write in tests/cli_list.rs, verify_a_batch_script_refuses in tests/cli_verify.rs, plugin_install_writes_owner_only_modes in tests/cli_plugin_install.rs, strict_modes_on_the_board_is_exit_21_and_opens_no_log and strict_modes_on_plugin_install_is_exit_21_and_creates_nothing in tests/cross_exit_causes.rs) each now carry a one-line OS-gate reason. Any refusal-inventory entity in cognitive-errors that cites these cases or their OS legs is reached, though the refusal behaviour itself is unchanged.
@task_item n=10 part=touches
The section 2 CI-integration no-new-suite rule and the section 9 rule of no second driver and no new suite enum value: the new contract binary contract_os_gated_tests runs inside nextest-e2e through run --e2e. On re-read the rule is unmoved and still holds.
@task_item n=11 part=touches
The section 9 artifact-retention rule and the CI matrix [windows-2025, macos-latest, ubuntu-latest]: scripts/junit-os-sets.py downloads the per-OS JUnit artifacts of a run for the operator pass. It is never a CI step and writes nothing in the tree. On re-read the claim that 0.1.0 uploads no a11y artifact of its own is unmoved.
@task_fact n=1 slug=aria-patterns-roles
fact: The outer-PTY witness of dim only on stale rows is cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal (tests/cross_list_rows.rs, cfg(unix), on booted_wrapper): the wrapper is frozen through Frozen, its heartbeat is back-dated 60 s, and viola list runs under the outer PTY with TERM=xterm-256color. It passes on ubuntu-latest and macos-latest and does not run on windows-2025.
@task_fact n=2 slug=visual-design-verification
fact: On a terminal a stale viola list row emits exactly the SGR sequence DIM, BOLD, RESET, DIM, RESET: the row is dim from its first byte to its last, NAME is dim and bold, BAY, the caption and the separator carry no SGR, and the word stale is shown beside the dim.
@task_item n=1 part=untouched
Section 1 scope: the Standard tier, the entity list, the not-assertable plugin/IPC and internal crates, and the compliance cross-check. The chunk adds no product surface, verb or UI element.
@task_item n=2 part=untouched
P4, the wheel and the keyboard boundary (human keystrokes never blocked, and focus, mouse and resize sequences not moving the wheel, with F-W3 on Windows). run_child_ends_when_its_wrapper_is_terminated now running on every OS concerns child lifetime after a killed wrapper, not keystroke delivery.
@task_item n=3 part=untouched
Section 8 timeout extensions, the plain-language copy set and the CAPTION literal order NAME, LIVE, STATUS, WHEEL, DIALOG, CLI. No CLI string, hint, caption or exit code changed.
@task_item n=4 part=untouched
Section 7 screen-reader support and the manual NVDA, VoiceOver and Orca passes on P4 and P6. They are founder-owned and none ran.
@task_item n=5 part=untouched
Section 10 SLO invariants, the gate --require coverage,doctest failure conditions and the zero-flakiness parallel. No gate or invariant changed.
@task_item n=6 part=untouched
Section 11 anti-patterns and section 12 Decisions Log. The chunk makes no a11y decision and lifts no ban.
@task_item n=7 part=untouched
The fake agent's SIGHUP-ignoring exit and its Linux-only hangup case, and the ci profile's 20 s kill line at 27 binaries. a11y-plan states neither.
