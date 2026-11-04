@task_prose part=session
Chunk 2026-11-04-linux-and-macos-parity is test-only: no file under src/, crates/*/src/, schemas/, plugin/, fixtures/, .github/, Cargo.toml or Cargo.lock moved; no product symbol, event, detail code, log field, IPC method, env var or config key changed.
It moved the Frozen guard whole out of tests/cli_instance_state.rs into the new tests/support/frozen.rs (Unix SIGSTOP via kill -STOP, Windows debugger attach), its four callers in two files.
It ungated run_child_ends_when_its_wrapper_is_terminated to every OS (Unix kill -KILL of the wrapper, the child gone by pid and start time), and landed the v1-40 outer-PTY witness cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal on the Unix legs (wrapper frozen, heartbeat back-dated 60 s, viola list under TERM=xterm-256color).
It added the census binary tests/contract_os_gated_tests.rs (59 GATES rows: 52 OS-gated test functions and 7 gated files, each with a reason; E2E layer, run --e2e) and the operator-pass instrument scripts/junit-os-sets.py, which downloads the junit-windows-2025, junit-macos-latest and junit-ubuntu-latest artifacts with gh and prints per-OS counts and set differences; never a CI step.
The ci profile's 20 s kill filter in .config/nextest.toml gains binary(run_cli): 26 to 27 binaries. The fake agent hangup case stays Linux-only, its reader now ps -o sigignore= in place of /proc SigIgn, since macOS ps has no ignored-signal keyword (measured, ci#41915621814).
Final CI ci#41918694243 on cbdd3874 green 15/15 (test, perf, lint on three OSes, so G2, G4 and the secret scan); JUnit windows-2025 3386, macos-latest 3412, ubuntu-latest 3458, 0 failed; ubuntu-not-macos 48 = 43 of the macOS no-peer-pid carve-out + 5 Linux-only.
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
Any CI-integration text naming the nextest ci profile's 20 s kill line (slow-timeout period 10s, terminate-after 2) or its binary set: it now names binary(run_cli) after binary(path1_run_start), 27 binaries where it read 26.
@task_item n=2 part=touches
Any text on the per-OS junit-<os> uploads (the nextest JUnit alone) or on CI producing nextest JUnit XML as a machine-parseable signal: the new scripts/junit-os-sets.py consumes exactly those three artifacts (junit-windows-2025, junit-macos-latest, junit-ubuntu-latest) through gh run download, an operator-pass reading, not a CI step, and writes nothing in the tree.
@task_item n=3 part=touches
The multi-platform-exporter-compat trigger and any claim that the same tests and gates run on windows-2025, macos-latest and ubuntu-latest: the session widened Unix coverage (the wrapper-terminated case on every OS, the stale-row witness on both Unix runners) and measured the per-OS JUnit sets, ubuntu-not-macos 48 = 43 no-peer-pid carve-out cases + 5 Linux-only cases.
@task_item n=4 part=touches
The macOS no-peer-pid carve-out statements (wait/last server verification and MCP instance-unreachable on macOS by euid mismatch alone): unchanged in behaviour, now counted as 43 cases that run on ubuntu-latest and not on macos-latest.
@task_item n=5 part=touches
The wrapped claude child boundary (exit detected on the process handle, the handle-wait thread, kill-fallback exit_source): run_child_ends_when_its_wrapper_is_terminated now proves on Unix too that the child ends when its wrapper is SIGKILLed, read through snapshot_data pids and start time; product code unchanged.
@task_item n=6 part=touches
The liveness signal and the chaos class stale heartbeat vs live pid (heartbeat flip at more than 5 s, list liveness): the new integration witness freezes a live wrapper through Frozen and back-dates its heartbeat 60 s to show a stale row end to end under a terminal; no liveness rule changed.
@task_item n=7 part=touches
Any citation of the Frozen guard's home as tests/cli_instance_state.rs: it now lives only in tests/support/frozen.rs (pub mod frozen in tests/support/mod.rs), on Unix kill -STOP polled to ps state T and kill -CONT on drop, on Windows a debugger attach and detach on drop.
@task_item n=8 part=touches
The list colour layer (list adds no stderr writer, its unable/hint pair carries no SGR): the stale-row witness asserts list's terminal stream escape sequences are exactly DIM, BOLD, RESET, DIM, RESET with stale shown, no SGR on BAY, caption or separator.
@task_item n=9 part=touches
Any test inventory or contract-binary list (contract_diag_schema, contract_bounded_readers and kin): a new root contract binary contract_os_gated_tests joins the E2E layer (contract_ prefix, run --e2e, suite nextest-e2e), its three tests a census of OS-gated tests in tests/ and crates/*/tests/ with a reason each.
@task_item n=10 part=touches
The fake agent references (the fake-agent seam, print-lint exceptions, CI viola verify against the fake agent): the fake agent's exit ignoring SIGHUP is unchanged shared Unix code; only its test's reader moved from /proc SigIgn to ps -o sigignore=, the case Linux-only because macOS ps cannot print the ignored mask.
@task_item n=11 part=touches
Gate verdict statements for G1, G2, G4, the secret scan, lint-probes and pre-push: all green on the final pushed sha cbdd3874 (ci#41918694243, checks 15/15) and locally; the smoke cleanup read processes_gone:true and endpoint_gone:true; no gate command or step changed.
@task_fact n=1 slug=ci-integration
fact: scripts/junit-os-sets.py (python 3 stdlib, --run <id>) downloads the junit-windows-2025, junit-macos-latest and junit-ubuntu-latest artifacts with gh run download into a temp dir removed on exit, prints per-OS cases, failed and skipped, then the sorted case lists for ubuntu-not-macos, macos-not-ubuntu, windows-not-ubuntu and ubuntu-not-windows; exit 0, 1 for a missing or unparsable report, 2 for usage; an operator-pass instrument, never a CI step.
@task_fact n=2 slug=ci-integration
fact: The nextest ci profile's 20 s kill line (.config/nextest.toml:24, slow-timeout period 10s, terminate-after 2) names 27 binaries, run_cli among them, the count's rule the binary(...) names on that filter line.
@task_fact n=3 slug=ci-integration
fact: tests/contract_os_gated_tests.rs, a contract binary in the E2E layer (run --e2e), holds every OS-gated test function and gated file under tests/ and crates/*/tests/ as a GATES row with a non-empty reason (59 rows: 52 functions, 7 files), fails on an unnamed gate or a row that no longer stands, and self-checks on eight planted cases.
@task_item n=1 part=untouched
The log line schema, required fields, closed event enum, detail code catalog and log level mapping (§3, §6): no product source, schemas/diag-line.v1.json or schemas/diag-detail.v1.json moved.
@task_item n=2 part=untouched
Error capture and panic hooks (§7): viola_panic_hook, catch_unwind sites, internal-error exit lines and the anyhow chain rules are untouched; no src file changed.
@task_item n=3 part=untouched
PII scrubbing and default-deny (§8): veil, the source allow-list, sink separation and the secret-scan patterns are unchanged.
@task_item n=4 part=untouched
Metric coverage (§5): derived jq metrics, naming and cardinality discipline are unchanged.
@task_item n=5 part=untouched
The six must-trace span scenarios and edge flows (§4) and span naming: no span, field or emitting call site changed.
@task_item n=6 part=untouched
The ci.yml step order, G1 to G4 gate commands, uploads gated on the secret-scan outcome, artifact retention and nightly.yml (§9): .github/ was not touched.
@task_item n=7 part=untouched
SLO invariants and perf budgets (§10): zero unlogged panics, hyperfine max budgets and the error budget are unchanged.
@task_item n=8 part=untouched
Obs tier, scope entities, telemetry surfaces and exporters (§1, §2) and the obs anti-patterns (§11): no surface, exporter, sink or ban changed.
@task_item n=9 part=untouched
The Obs Decisions Log (§12): no decision was made or amended for obs in this chunk.
