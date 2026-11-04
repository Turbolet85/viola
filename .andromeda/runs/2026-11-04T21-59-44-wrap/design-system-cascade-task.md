@task_prose part=session
Test-only chunk, Linux and macOS parity: no product source, schema, plugin, fixture, CI workflow or Cargo file moved, and no product symbol, CLI output, colour, word or exit code changed.
It added an integration-tier outer-PTY witness of the CLI stale-row dim, cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal (Unix). It runs viola list under a PTY with TERM=xterm-256color, and the stale row's escape sequences read exactly DIM, BOLD, RESET, DIM, RESET, with stale shown. It passes on ubuntu-latest and macos-latest.
It moved the Frozen freeze guard out of tests/cli_instance_state.rs into tests/support/frozen.rs. It ungated run_child_ends_when_its_wrapper_is_terminated to every OS (SIGKILL on Unix). It switched the fake agent's hangup case reader from /proc SigIgn to ps -o sigignore=, and the case stays Linux only because macOS ps has no ignored-signal keyword (measured).
It added a census of OS-gated tests (tests/contract_os_gated_tests.rs: 52 functions and 7 files, 59 GATES rows, each with a reason) and an operator-only per-OS JUnit reader, scripts/junit-os-sets.py. It also added a reason line to ten OS-gated tests and binary(run_cli) to the nextest ci 20 s kill line (26 to 27 binaries).
CI ci#41918694243 on cbdd3874 is green, 15/15 checks.
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
The CLI colour tokens: SGR dim on a whole stale row, bold for NAME values, a reset after every styled span. These now have an end-to-end terminal witness on the Unix legs. Any text saying the dim-on-stale-rows proof is owed, unit tier only, or a CARRY on Linux and macOS parity is now stale, because the witness landed.
@task_item n=2 part=touches
The viola list strip table pattern: NAME bold and the stale scratch row dim. The witness reads the stale row dim from its first byte to its last, NAME dim and bold, at 256-colour depth (TERM=xterm-256color), and the stale word is printed. This confirms the pattern as built.
@task_item n=3 part=touches
The cli per-surface ban that colours nothing except DIALOG (amber) and stale rows (dim). The witness asserts no SGR on the BAY line, the unwrapped caption or the separator, and so confirms this ban on a real terminal.
@task_item n=4 part=touches
The colour decision order's depth rule (256-colour when TERM contains 256color). The new witness exercises this path under an outer PTY on Linux and macOS. It is a confirmation, not a change.
@task_item n=5 part=touches
The cli platform line naming macOS and Linux terminals. The full fake-agent suite and the CI test, perf and lint gates are green on Windows, macOS and Linux. Any claim that Linux or macOS terminal behaviour is unverified or owed may now be stale.
@task_item n=6 part=touches
The revive --json stale-heartbeat refusal and the run stale-name refusal (start refusal codes). Their tests use the Frozen guard, which now lives in tests/support/frozen.rs, not tests/cli_instance_state.rs. Any citation of the guard's file is a moved path.
@task_item n=7 part=touches
The exit-21 strict-modes refusals of the viola list board and viola plugin install. Their tests in tests/cross_exit_causes.rs (and plugin_install_writes_owner_only_modes, list_refuses_an_instance_another_user_could_write) now carry a reason line for their Unix gate, the mode-bit widening each one uses. The behaviour and printed pair are unchanged.
@task_item n=8 part=touches
The viola run passthrough (prints nothing once the child starts; the child owns the terminal). Its companion test run_child_ends_when_its_wrapper_is_terminated now runs on every OS, not on Windows only. Any statement scoping that wrapper-termination behaviour to Windows is stale.
@task_item n=9 part=touches
Line citations into tests/cross_list_rows.rs, tests/run_cli.rs, tests/cli_fake_agent.rs, tests/cli_instance_state.rs, tests/cross_exit_causes.rs, tests/cli_list.rs, tests/cli_plugin_install.rs, tests/cli_verify.rs and .config/nextest.toml may be offset, because lines were added in each.
@task_item n=10 part=touches
The test inventory: the new root contract binary contract_os_gated_tests (E2E layer, run --e2e). It is a census requiring every OS-gated test function or file to be named with a reason. Any count or list of contract binaries or OS-gated tests it holds moves.
@task_item n=11 part=touches
The nextest ci profile's 20 s kill line now names binary(run_cli): 26 to 27 binaries. This touches any count of that line.
@task_fact n=1 slug=surface
fact: The CLI stale-row dim is witnessed at the integration tier by cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal (Unix, ubuntu-latest and macos-latest). Under an outer PTY with TERM=xterm-256color, viola list's stream carries exactly DIM, BOLD, RESET, DIM, RESET for a stale row (dim first byte to last, NAME dim and bold), with no SGR on the BAY line, caption or separator.
@task_item n=1 part=untouched
Color Palette (core colors, surface scale, text hierarchy, semantic and domain status colors, border progression, contrast table). No hex, role or contrast value moved, and the frontend is deferred.
@task_item n=2 part=untouched
Typography, including the per-OS fallback stacks and the Linux fallback assertions. These are a 0.2.0 page-render concern. The session's Linux and macOS parity is test-suite parity, not font rendering.
@task_item n=3 part=untouched
Spacing, Depth Strategy, Border Radius, Motion and Iconography tokens. These are frontend design inputs, and no token was touched.
@task_item n=4 part=untouched
Brand Identity: the personality, the domain anchors, the readback box signature and its three states, and the expression level 0.2. No word, state or value changed.
@task_item n=5 part=untouched
The viola send readback mirror, the wait and last result lines, and the message-mode escaper. No output form changed.
@task_item n=6 part=untouched
Exit-code phraseology and the closed cause word lists for exit 21 and revive's --json codes. No code, cause word or document shape changed. Only test reason comments were added.
@task_item n=7 part=untouched
The wheel, link, gate and plugin install verb outputs, the verify step counter, the navigation pattern and the viola --help grouping. No verb or line changed.
@task_item n=8 part=untouched
Platform-Specific Notes for Git Bash, conhost without VT, the silent hook and mcp, and the stack-trace ban. Nothing Windows-console or hook-related moved.
@task_item n=9 part=untouched
The Surface web-spa section and the Self-Validation Protocol tests (Swap, Squint, Signature, Token, Sameness). The frontend is still deferred, and the signature's one 0.1.0 place is unchanged.
@task_item n=10 part=untouched
The clap, IsTerminal, windows-sys and hand-written SGR module toolkit line. No dependency was added or bumped, and Cargo.toml and Cargo.lock are unchanged.
