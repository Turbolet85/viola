@task_prose part=session
A test-only parity chunk: no file under src/, crates/*/src/, schemas/, plugin/, fixtures/, .github/, Cargo.toml or Cargo.lock moved (the preservation guard read clean, 13 files changed, all allowed), and no product symbol, IPC method, env var, socket or CI step changed.
It added tests/support/frozen.rs (the shared Frozen guard, moved out of tests/cli_instance_state.rs), tests/contract_os_gated_tests.rs (a census that names every OS-gated test with its reason: 52 functions and 7 files, 59 GATES rows, reason constants UNIX_MODES, UNIX_STATUSLINE and NO_PEER_PID), and scripts/junit-os-sets.py (an operator-only per-OS JUnit reader that uses python stdlib and the host gh, never a CI step).
It ungated run_child_ends_when_its_wrapper_is_terminated to every OS, landed the Unix outer-PTY witness of the dim stale row in viola list, and added one reason line to each of ten OS-gated tests that had none, several of them owner-only mode and strict-modes tests.
It measured that macOS ps has no keyword for the ignored-signal mask, so the fake-agent hangup case stays Linux-only and reads ps -o sigignore= in place of /proc SigIgn; the fake agent's SIGHUP-ignoring exit itself is unchanged.
CI on the final sha (ci#41918694243) was green, 15/15 checks across the three OS legs, secret scan included. Per-OS JUnit: 48 cases run on ubuntu-latest and not on macos-latest, 43 of them through the macOS no-peer-pid carve-out and 5 Linux-only for a named reason.
The ci nextest profile's 20 s kill line went from 26 to 27 binaries. That is a test-runner setting, not a security control.
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
The macOS client-side server-verification carve-out (no peer pid or start-time check on macOS, owned by no route entry, its euid and socket-directory checks standing) is now a named gate reason, NO_PEER_PID, in the census of tests/contract_os_gated_tests.rs. 43 of the 48 cases that run on ubuntu-latest and not on macos-latest skip macOS through it (both ci#41339639367 and ci#41918694243). The carve-out itself, its scope and its lack of a route entry are unchanged.
@task_item n=2 part=touches
The ~/.viola/ owner-only modes and the strict-modes check (Unix 0700 directories and 0600 files, a refusal of a path another user can write or read) are now covered by gate-reason lines on their Unix-only tests: list_refuses_an_instance_another_user_could_write, plugin_install_writes_owner_only_modes, run_creates_home_0700_and_role_file_0600, strict_modes_on_the_board_is_exit_21_and_opens_no_log and strict_modes_on_plugin_install_is_exit_21_and_creates_nothing. The census names them through its UNIX_MODES reason. The controls are unchanged; the report says no security-plan text states a reason for the two cross_exit_causes gates.
@task_item n=3 part=touches
The anti-pattern on test-side removal under target/e2e-home (two keepers, an owner-only 0700 target, nothing removed) covers three tests that each gained a reason line in tests/cli_instance_state.rs: home_base_backing_owner_only_target_is_kept_untouched, its group_or_other_bit_is_refused case and its target_that_is_a_link_is_refused case. The rule is unchanged.
@task_item n=4 part=touches
The Windows .cmd/.bat child refusal (child executable resolution, and the ban on spawning a .cmd/.bat PTY child) covers verify_a_batch_script_refuses in tests/cli_verify.rs, which gained a reason line and is named in the census. The refusal is unchanged.
@task_item n=5 part=touches
The Unix-only statusline shell-out (hook statusline running the recorded command as /bin/sh -c, never on Windows) is now a named gate reason, UNIX_STATUSLINE, in the census. The rule is unchanged.
@task_item n=6 part=touches
The terminal colour facts row (TERM read only to decide SGR output in viola list human mode) and the list-row escaping floor now have an integration witness. cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal (Unix) runs viola list under an outer PTY with TERM=xterm-256color and asserts the exact escape sequences DIM, BOLD, RESET, DIM, RESET with stale shown. No escaping, check or colour read changed.
@task_item n=7 part=touches
The test-only fake-agent feature and its seams: the fake agent's SIGHUP-ignoring exit (src/bin/viola-fake-agent.rs, shared Unix code) is unchanged, and no seam or env read was added. Only its test reader moved from /proc SigIgn to ps -o sigignore=, and that test is Linux-only.
@task_item n=8 part=touches
The CI/CD statement (ci.yml on a windows-2025, macos-latest and ubuntu-latest matrix) and the artifact canary secret-scan gate: no workflow, permission, action pin, upload or step changed. CI on the final sha (ci#41918694243) was green, 15/15 checks, with the secret scan inside the test and perf jobs.
@task_item n=9 part=touches
The Development environment rule (pre-push launches children under env -i with only HOME and PATH, and any host-valued assignment is a boundary widening): pre-push re-ran green with no change. The new scripts/junit-os-sets.py reads no environment variable, writes nothing in the tree, and downloads into a temp dir it removes. It runs only in the operator pass and never as a CI step or harness stage.
@task_item n=10 part=touches
Dependency pinning and tool floors: Cargo.toml and Cargo.lock are unchanged, and no crate was added or bumped. The new script uses only the python standard library and the host gh, so no lockfile or deny.toml entry changes.
@task_item n=11 part=touches
The child-gone-after-wrapper-kill behaviour: run_child_ends_when_its_wrapper_is_terminated now runs on every OS (TerminateProcess on Windows, kill -KILL on Unix), with the child confirmed gone by pid and start time. This reads alongside the rule that the child gets no inheritable channel handle. No product code moved.
@task_item n=12 part=touches
The guard that freezes a wrapper in tests (Frozen: kill -STOP and kill -CONT on Unix, a debugger attach on Windows) now lives in tests/support/frozen.rs only, aimed at the test's own wrapper. Its callers include the stale-holder refusals run_refuses_a_stale_name and run_refuses_a_frozen_wrapper_as_stale_on_windows. The product's stale-holder logic is unchanged.
@task_item n=1 part=untouched
IPC access control: the Windows protected DACL, the Unix 0700 socket directory with chmod 0600 after bind, the server-side peer euid check, the SQOS client open and the release-from-driver guard. No channel code moved.
@task_item n=2 part=untouched
Input validation rows: validate_paste_text, the MAX_FRAME cap and its consumers, channel framing, hook stdin, MCP inputs and MCP stdin, config.json, tailed paths, the vt100 feed and the session-id argv. No parser or cap changed.
@task_item n=3 part=untouched
Dependency security: deny.toml advisories, sources, licences and feature bans, the ConPTY vendor pins and verification, release-check.sh, zizmor, the nightly advisory run and the fuzz exemption. No workflow or lockfile moved.
@task_item n=4 part=untouched
Secret management and the NEVER-log floor: the stripped CLAUDE* names, the canary classes and the diagnostics 0600 rule. No logging, diagnostics or redaction code changed.
@task_item n=5 part=untouched
Error handling: the fixed Display impls, the MCP isError codes, the channel -32603 shape and hook failing open. No error type or refusal word changed.
@task_item n=6 part=untouched
Threat model tier, data classification and attack-surface vectors: the chunk adds no product surface, listener, channel method or data class.
@task_item n=7 part=untouched
Bootstrap phases and the Security Decisions Log: the operator ruling to gate the hangup case back to Linux is a decision about a test reader, not a security ratification, and no new seam, exemption or G2 exemption was recorded.
@task_item n=8 part=untouched
The data-protection rows on repository fixtures, verify's probe captures and the accepted risks of verify's trusted runs: there were no live sessions (fake agent only) and fixtures/ is unchanged.
