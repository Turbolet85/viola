@task_prose part=session
The session ties every long-lived viola child a root test spawns to the test, so a wrapper and its fake agent end with their test, killed or not: a pre_exec sets a parent-death SIGKILL and exits when the recorded parent is gone.
Every direct root test spawn of viola now goes through one tied builder; a census test finds 0 untied spawn sites, down from 44 by the plan's pattern plus 1 by a wider spelling the plan missed.
The test harness boot options gain a tied flag and two constructors: an in-test harness session ties its supervisor and its stamping viola verify, while the CLI boot stays untied and unchanged in behaviour.
The shared verify stamp of the test homes counts only when its completion file and its stamps file stand; a hollow stamp restored without its files is left as found and a next-generation stamp dir is built beside it, nothing removed.
New witnesses SIGKILL or panic a victim test and assert no wrapper, agent or harness supervisor outlives it within a bounded wait; integration selection moved 505 to 509 and E2E 348 to 369, all green.
After implement the CI run read test green on an exact cache restore, and supply-chain red on advisory RUSTSEC-2026-0543 (chrono unmaintained), not mended here: no ignore, no re-run, its remedy asked of the overseer at route-resolve.
@task_file n=1 path=tests/support/tied.rs
what: new root test support module, the tie of a spawned viola child to its test (parent-death signal), the tied viola command builder, and the process-tree record and bounded survivor wait the witnesses use
@task_file n=2 path=tests/cross_killed_test.rs
what: new root E2E test binary, the killed-test and panicked-test witnesses for piped, direct, outer-PTY and frozen wrappers, and the census that no root test spawns viola untied
@task_file n=3 path=crates/viola-e2e/tests/harness_tied.rs
what: new harness integration test binary, the witness that a SIGKILLed test leaves no harness session, supervisor or agent running
@task_file n=4 path=tests/support/home.rs
what: root test support for test homes, the shared stamped home and its per-build verify stamp dir, now counted by a completion file and rebuilt as a next generation when hollow; the revive command built tied
@task_file n=5 path=tests/support/mod.rs
what: the root test support module list, now including the tie module
@task_file n=6 path=tests/support/piped.rs
what: root test support for a piped wrapper child, now tying the command it spawns
@task_file n=7 path=tests/support/cli.rs
what: root test support starting a wrapped instance through the CLI, now built tied
@task_file n=8 path=tests/support/start.rs
what: root test support for a refused start, now built tied
@task_file n=9 path=tests/support/verify.rs
what: root test support driving viola verify, its two spawn sites now built tied
@task_file n=10 path=tests/cli_verify.rs
what: viola verify integration tests, gaining the hollow-stamp rebuild and next-generation reuse witnesses
@task_file n=11 path=crates/viola-e2e/src/harness/boot.rs
what: the test harness boot, its boot options gaining a tied flag and session and CLI constructors, the supervisor spawn and the stamping viola verify tied when asked
@task_file n=12 path=crates/viola-e2e/src/harness/session.rs
what: the in-test harness session, now booting through the tied session constructor
@task_file n=13 path=crates/viola-e2e/src/bin/viola-harness.rs
what: the harness CLI binary (agent-run boot, status, logs, cleanup), its boot options built through the untied CLI constructor
@task_file n=14 path=.config/nextest.toml
what: the nextest CI profile, its stamped-home kill-timeout override now also selecting the new killed-test binary
@task_file n=15 path=crates/viola-e2e/src/harness/run/perf.rs
what: the harness perf run (hook benchmark boot), its boot options literal now naming the tied flag
@task_file n=16 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: harness lifecycle integration tests, their in-test boot options naming tied false (left untied)
@task_file n=17 path=crates/viola-e2e/tests/cli.rs
what: harness CLI integration tests, their in-test boot naming tied false (left untied)
@task_file n=18 path=tests/chaos_feed_panic.rs
what: chaos test of a vt100 feed panic in run, its viola spawns moved to the tied builder
@task_file n=19 path=tests/chaos_snapshot_replay.rs
what: chaos test of snapshot replay, its viola spawns moved to the tied builder
@task_file n=20 path=tests/chaos_wait_vanish.rs
what: chaos test of a wrapper vanishing during wait, its viola spawns moved to the tied builder
@task_file n=21 path=tests/cli_answer.rs
what: viola answer integration tests, their viola spawns moved to the tied builder
@task_file n=22 path=tests/cli_controls_not_disableable.rs
what: tests that redaction and controls cannot be disabled, their viola spawns moved to the tied builder
@task_file n=23 path=tests/cli_error_bodies.rs
what: CLI error body and fixed stderr message tests, their viola spawns moved to the tied builder
@task_file n=24 path=tests/cli_fake_agent.rs
what: fake-agent CLI tests, their viola spawns moved to the tied builder
@task_file n=25 path=tests/cli_instance_state.rs
what: instance state and liveness CLI tests, their viola spawns moved to the tied builder
@task_file n=26 path=tests/cli_machine_contract.rs
what: CLI --json machine contract tests, their viola spawns moved to the tied builder
@task_file n=27 path=tests/cli_output_plain.rs
what: plain CLI output tests, their viola spawns moved to the tied builder
@task_file n=28 path=tests/cli_plugin_install.rs
what: plugin install CLI tests, their viola spawn moved to the tied builder
@task_file n=29 path=tests/cli_send.rs
what: viola send integration tests, their viola spawns moved to the tied builder
@task_file n=30 path=tests/cli_version_gate.rs
what: run version gate tests, their viola spawn moved to the tied builder
@task_file n=31 path=tests/contract_diag_schema.rs
what: diagnostic line schema conformance tests over real process output, their viola spawns moved to the tied builder
@task_file n=32 path=tests/cross_exit_causes.rs
what: exit cause and detail code tests, the one spawn site of the wider spelling moved to the tied builder
@task_file n=33 path=tests/cross_list_rows.rs
what: viola list row tests, their viola spawns moved to the tied builder
@task_file n=34 path=tests/hook_events.rs
what: viola hook event tests, their viola spawns moved to the tied builder
@task_file n=35 path=tests/hook_fail_open.rs
what: hook fail-open tests, their viola spawns moved to the tied builder
@task_file n=36 path=tests/run_cli.rs
what: viola run CLI tests, their viola spawns moved to the tied builder
@task_file n=37 path=tests/security_negatives_socket_dir.rs
what: socket-directory refusal negative tests, their viola spawns moved to the tied builder
@task_file n=38 path=tests/security_negatives_strict_modes.rs
what: strict-modes refusal negative tests, their viola spawns moved to the tied builder
@task_item n=1 part=touches
decision/obs-deps-pass-cargo-deny, decision/cargo-deny-failure-fails and decision/ci-gates-fail-closed: the supply-chain job is red as-left on RUSTSEC-2026-0543, so an obs dependency no longer passes cargo deny; the session does not mend it and asks its remedy at route-resolve.
@task_item n=2 part=touches
decision/direct-chrono-dependency: chrono, the direct timestamp dependency, is the crate the advisory names unmaintained; it stands four ways in the graph, so replacing its direct use would not remove it.
@task_item n=3 part=touches
decision/rerun-no-substitute: the operator pass ruled out a re-run of the red supply-chain job as no remedy, the same reading the row holds for diagnostics.
@task_item n=4 part=touches
decision/test-homes-under-e2e-home and decision/test-homes-under-scan-root: a stamped test home's base may now stand as a next-generation stamp dir with a completion file, and the killed-test victims write their pid report under the witness's own test home.
@task_item n=5 part=touches
decision/homes-kept-until-gates-run and decision/test-homes-kept-until-gates: a hollow stamp is left as found and nothing is removed; a killed test no longer leaves a running wrapper writing into a kept home after the test ends.
@task_item n=6 part=touches
decision/verify-sole-stamps-writer and decision/unreadable-ledger-means-unverified: the shared stamp now counts only when the stamps file under the home ledger stands with a completion file written by test support after verify returns; the stamps file itself is still written by viola verify alone.
@task_item n=7 part=touches
decision/ci-verify-fake-agent-only and decision/real-cli-verify-never-in-ci: the stamping viola verify of an in-test harness session is now tied to the test, still against the fake agent.
@task_item n=8 part=touches
decision/no-sleep-based-log-waits: the new witnesses wait for the victim's report and for the recorded processes to be gone within a bounded deadline, then SIGKILL survivors and fail.
@task_item n=9 part=touches
decision/os-gated-test-census and decision/linux-only-os-target: the tie rests on the Linux parent-death signal, and the new witness binaries depend on it.
@task_item n=10 part=touches
decision/env-vars-not-config-channel and decision/env-vars-not-configuration: two test-only self-spawn markers, ROOT_TIED_TEST_CHILD and HARNESS_TIED_TEST_CHILD, outside VIOLA_*, each read by its own test binary and by no viola build.
@task_item n=11 part=touches
decision/gauges-from-harness-snapshots, decision/perf-job-runs-g2 and decision/hook-benchmark-on-passing-instance: the harness boot options the perf run and harness sessions build gained a tied flag; the perf boot names it with no change of behaviour.
@task_item n=12 part=touches
contract/harness-logs/*, contract/agent-run-status/* and contract/agent-run-logs/*: the harness CLI boot now builds through an untied CLI constructor, unchanged in behaviour (smoke: boot ok, status ready, cleanup processes_gone and endpoint_gone).
@task_item n=13 part=touches
decision/harness-logs-captured-before-scan: an in-test harness session's supervisor now ends when its test is killed, so a killed test leaves no harness session running past the log capture.
@task_item n=14 part=touches
decision/diag-lines-schema-checked-in-unit-tests and decision/schema-conformance-gate: the tests that check emitted lines on real process output now spawn viola through the tied builder; their assertions are unchanged.
@task_item n=15 part=touches
decision/zero-panics-gate and decision/panic-count-home-level-only: the witnesses' survivor panic and the panicked-victim case are test-harness panics, not product panic lines; a pick should confirm no product panic line lands in a counted home-level file.
@task_item n=16 part=touches
decision/heartbeat-liveness and contract/list-liveness/stale-threshold: a wrapper of a killed test now dies with it instead of standing with a stale heartbeat.
@task_item n=1 part=untouched
The diagnostics line schema, its required fields, the event enum, corr and conn correlation and every detail code: no event, field, detail word or exit code was added or changed.
@task_item n=2 part=untouched
Span and trace coverage, span naming, span attributes and span kinds: no span was added or renamed.
@task_item n=3 part=untouched
The MCP server, the channel JSON-RPC methods, the hook stdout and stderr contracts and the send, wait, dialog and wheel log lines: no IPC method, endpoint, event, socket or port was added.
@task_item n=4 part=untouched
PII scrubbing, the NEVER-log floor, redaction, veil and the secret scan: no logged field or content path moved.
@task_item n=5 part=untouched
Error capture and reporting, the panic hook, the panic line and panic outcome by role in product binaries: product panic handling is unchanged.
@task_item n=6 part=untouched
Metric coverage, the hook and session-end perf budgets and their hyperfine thresholds: no budget, threshold or derived metric moved; the nextest selection counts and spawn-site counts that moved are held by no obs-plan row.
@task_item n=7 part=untouched
Log file location, home resolution order, service identity, config.json keys and log levels: no product logging path or level changed.
@task_item n=8 part=untouched
The banned telemetry dependencies, OTel, exporters and network egress absences: no dependency was added.
