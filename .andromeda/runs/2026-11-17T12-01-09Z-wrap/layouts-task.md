@task_prose part=session
The session changed test infrastructure only: no CLI verb, output line, refusal, hint, exit code, json document, MCP tool or styling changed.
Every viola process a root test spawns is now tied to the test through a parent-death signal, so a killed or panicked test leaves no wrapper or fake agent running.
An in-test harness session's supervisor and its stamping viola verify are tied the same way; the CLI harness boot keeps its supervisor untied, unchanged in behaviour.
The shared verify stamp used by test homes now counts only when its completion file and its ledger stamps file are present; a hollow stamp is restamped into a next-generation directory, nothing removed.
A census test now proves no root test spawns viola untied; it measured 45 direct spawn sites (one more than the plan's 44) and moved all of them to the tied builder.
Nextest counts moved: integration 505 to 509, E2E 348 to 369; no production code, dependency or master spec changed.
@task_file n=1 path=tests/support/tied.rs
what: new root test support module: the tie that kills a test's viola child when the test dies, the tied viola command builder, and the process-tree survivor check
@task_file n=2 path=tests/cross_killed_test.rs
what: new E2E test binary: witnesses that a killed or panicked test leaves no piped, direct, outer-pty or frozen wrapper, plus the census of untied viola spawns
@task_file n=3 path=crates/viola-e2e/tests/harness_tied.rs
what: new harness integration test: a killed test leaves no harness session supervisor or instance running
@task_file n=4 path=tests/support/home.rs
what: root test home support: the shared verify stamp directory, its completion check and next-generation rebuild, and the revive command now built tied
@task_file n=5 path=tests/support/mod.rs
what: root test support module list, now including the tied module
@task_file n=6 path=tests/support/piped.rs
what: root test support for a piped wrapper spawn, now tying the command it is handed
@task_file n=7 path=tests/support/cli.rs
what: root test support starting a wrapped instance from the CLI, now through the tied builder
@task_file n=8 path=tests/support/start.rs
what: root test support for refused starts, now through the tied builder
@task_file n=9 path=tests/support/verify.rs
what: root test support driving verify runs in tests, now through the tied builder
@task_file n=10 path=tests/cli_verify.rs
what: verify integration tests, gaining the hollow-stamp rebuild and generation-reuse witnesses
@task_file n=11 path=crates/viola-e2e/src/harness/boot.rs
what: the test harness boot: its options gain a tied flag and two constructors, one for an in-test session (tied) and one for the CLI harness (untied)
@task_file n=12 path=crates/viola-e2e/src/harness/session.rs
what: the in-test harness session, now booting tied
@task_file n=13 path=crates/viola-e2e/src/bin/viola-harness.rs
what: the harness CLI binary's boot options, now through the untied CLI constructor
@task_file n=14 path=.config/nextest.toml
what: nextest profile config: the CI stamped-home kill override now also selects the killed-test binary
@task_file n=15 path=crates/viola-e2e/src/harness/run/perf.rs
what: harness perf run: its boot options name the new tied flag
@task_file n=16 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: harness lifecycle tests: their boot options name the tied flag, left untied
@task_file n=17 path=crates/viola-e2e/tests/cli.rs
what: harness CLI tests: their boot options name the tied flag, left untied
@task_file n=18 path=tests/chaos_feed_panic.rs
what: chaos test for a feed panic, its viola spawns moved to the tied builder
@task_file n=19 path=tests/chaos_snapshot_replay.rs
what: chaos test for snapshot replay, its viola spawns moved to the tied builder
@task_file n=20 path=tests/chaos_wait_vanish.rs
what: chaos test for a wrapper vanishing during wait, its viola spawns moved to the tied builder
@task_file n=21 path=tests/cli_answer.rs
what: answer verb integration tests, their viola spawns moved to the tied builder
@task_file n=22 path=tests/cli_controls_not_disableable.rs
what: integration tests that controls cannot be disabled, their viola spawns moved to the tied builder
@task_file n=23 path=tests/cli_error_bodies.rs
what: integration tests of CLI error bodies, their viola spawns moved to the tied builder
@task_file n=24 path=tests/cli_fake_agent.rs
what: integration tests against the fake agent, their viola spawns moved to the tied builder
@task_file n=25 path=tests/cli_instance_state.rs
what: instance state integration tests, their viola spawns moved to the tied builder
@task_file n=26 path=tests/cli_machine_contract.rs
what: machine-contract integration tests, their viola spawns moved to the tied builder
@task_file n=27 path=tests/cli_output_plain.rs
what: plain output mode integration tests, their viola spawns moved to the tied builder
@task_file n=28 path=tests/cli_plugin_install.rs
what: plugin install integration tests, their viola spawn moved to the tied builder
@task_file n=29 path=tests/cli_send.rs
what: send verb integration tests, their viola spawns moved to the tied builder
@task_file n=30 path=tests/cli_version_gate.rs
what: CLI version gate integration tests, their viola spawn moved to the tied builder
@task_file n=31 path=tests/contract_diag_schema.rs
what: diagnostics schema contract tests, their viola spawns moved to the tied builder
@task_file n=32 path=tests/cross_exit_causes.rs
what: E2E exit-cause tests, the 45th spawn site (outside the plan's pattern) moved to the tied builder
@task_file n=33 path=tests/cross_list_rows.rs
what: E2E list row tests, their viola spawns moved to the tied builder
@task_file n=34 path=tests/hook_events.rs
what: hook event tests, their viola spawns moved to the tied builder
@task_file n=35 path=tests/hook_fail_open.rs
what: hook fail-open tests, their viola spawns moved to the tied builder
@task_file n=36 path=tests/run_cli.rs
what: run verb integration tests, their viola spawns moved to the tied builder
@task_file n=37 path=tests/security_negatives_socket_dir.rs
what: socket directory security negative tests, their viola spawns moved to the tied builder
@task_file n=38 path=tests/security_negatives_strict_modes.rs
what: strict-modes security negative tests, their viola spawns moved to the tied builder
@task_item n=1 part=touches
Indirect, through the verify stamp: a test stamp now counts only when the verify run's ledger stamps file is present beside a completion file; a viola verify --record row that says what the record writes under the home ledger is the file the stamp check now relies on, unchanged in wording.
@task_item n=2 part=touches
Indirect, through the verify stamp: the stamping viola verify run inside an in-test harness session is now tied to the test; verify's output, step lines, summary and refusals are unchanged.
@task_item n=3 part=touches
Indirect, through the run and revive start tests: every viola run and viola revive a root test starts is now tied by a parent-death signal; the start output, start refusals and already-live checks of those verbs are unchanged.
@task_item n=4 part=touches
Indirect, through the global home flag: tests point --home at a copy of a stamped home that may now stand in a next-generation directory; the flag's contract is unchanged.
@task_item n=5 part=touches
Indirect, through the tie: a tied child whose parent is already gone exits 1 before exec; this is test-side only and is no viola exit code, so the CLI exit code rows hold.
@task_item n=6 part=touches
Indirect, through the wrapper tests: a killed test now also kills its piped, direct, outer-pty or frozen wrapper; the wrapper's repeated send refusals and send output rows are unchanged.
@task_item n=1 part=untouched
CLI output wording and layout: the board, the BAY line, columns, truncation, separators, readback mirror and send, wait, answer, last and link output rows; no output line changed.
@task_item n=2 part=untouched
CLI token rendering and styling: colour, SGR, TTY detection, spacing, radius and motion rows; nothing rendered changed.
@task_item n=3 part=untouched
Hints and refusal lines: the hint wording, hint placement and the refusal word column; no hint or refusal was added or reworded.
@task_item n=4 part=untouched
Exit codes and json documents: no exit code, json form, exit-21 detail code or internal-error document changed.
@task_item n=5 part=untouched
The MCP server and its tools and tool results, and the hidden mcp verb; nothing in the MCP surface changed.
@task_item n=6 part=untouched
The verb set and parsing: flat verbs, clap parsing, help output and per-verb usage; no verb or flag was added, hidden or removed.
@task_item n=7 part=untouched
The web surface absence and the CLI deep-links absence; no web or link capability was added.
