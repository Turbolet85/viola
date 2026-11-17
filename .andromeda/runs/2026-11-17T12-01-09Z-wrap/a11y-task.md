@task_prose part=session
Every long-lived viola child a root test spawns is now tied to the test with a parent-death SIGKILL, so a killed or panicked test leaves no wrapper or fake agent running; a census test fails on any untied viola spawn under the root tests (45 sites moved, 0 left).
An in-test harness session's supervisor and its stamping viola verify are tied the same way; the CLI boot of the harness stays untied and unchanged in behaviour.
The shared verify stamp of the test home counts only with its completion file and its stamps file; a hollow stamp restored without them is rebuilt as a next generation dir, nothing removed.
The CI kill override for stamped-home tests now covers the new killed-test binary; integration selection moved 505 to 509 and E2E 348 to 369, all green; no printed CLI output, exit code, refusal or --json document changed.
@task_file n=1 path=tests/support/tied.rs
what: new root test support that ties a spawned viola child to its test and waits for a recorded process tree to be gone within a bound
@task_file n=2 path=tests/cross_killed_test.rs
what: new E2E-layer test binary: killed and panicked test witnesses for piped, direct, outer-PTY and frozen wrappers, and the census of untied viola spawns
@task_file n=3 path=crates/viola-e2e/tests/harness_tied.rs
what: new harness integration test: a killed test leaves no harness session (supervisor, wrapper, fake agent)
@task_file n=4 path=tests/support/home.rs
what: root test support for the test home, the shared verify stamp per build and its reuse, and the revive command
@task_file n=5 path=tests/support/mod.rs
what: root test support module list
@task_file n=6 path=tests/support/piped.rs
what: root test support spawning a wrapper on pipes (the piped run driver)
@task_file n=7 path=tests/support/cli.rs
what: root test support starting viola for the CLI suites
@task_file n=8 path=tests/support/start.rs
what: root test support for refused run and revive starts
@task_file n=9 path=tests/support/verify.rs
what: root test support running viola verify
@task_file n=10 path=tests/cli_verify.rs
what: viola verify suite, its step-line and stamp witnesses
@task_file n=11 path=crates/viola-e2e/src/harness/boot.rs
what: the harness boot of a session: its supervisor spawn, its stamping verify, its boot options
@task_file n=12 path=crates/viola-e2e/src/harness/session.rs
what: the in-test harness session a test boots
@task_file n=13 path=crates/viola-e2e/src/bin/viola-harness.rs
what: the harness CLI (boot, status, cleanup) the agent runner drives
@task_file n=14 path=.config/nextest.toml
what: nextest profiles: the CI profile's per-binary kill timeout overrides
@task_file n=15 path=crates/viola-e2e/src/harness/run/perf.rs
what: the harness perf run boot
@task_file n=16 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: harness lifecycle integration tests (boot, status, cleanup)
@task_file n=17 path=crates/viola-e2e/tests/cli.rs
what: harness CLI integration tests
@task_file n=18 path=tests/chaos_feed_panic.rs
what: chaos suite: a contained vt100 feed panic in the TUI passthrough
@task_file n=19 path=tests/chaos_snapshot_replay.rs
what: chaos suite: screen snapshot replay
@task_file n=20 path=tests/chaos_wait_vanish.rs
what: chaos suite: viola wait when the instance vanishes
@task_file n=21 path=tests/cli_answer.rs
what: viola answer suite: the dialog answer critical path
@task_file n=22 path=tests/cli_controls_not_disableable.rs
what: suite holding that the safety controls cannot be disabled
@task_file n=23 path=tests/cli_error_bodies.rs
what: error bodies suite: each error body's writer and witness
@task_file n=24 path=tests/cli_fake_agent.rs
what: CLI suite over a fake agent instance
@task_file n=25 path=tests/cli_instance_state.rs
what: instance state suite (list board rows, stale rows)
@task_file n=26 path=tests/cli_machine_contract.rs
what: machine contract suite: --json documents and exit codes
@task_file n=27 path=tests/cli_output_plain.rs
what: plain-mode output-discipline suite (no SGR off a terminal)
@task_file n=28 path=tests/cli_plugin_install.rs
what: viola plugin install suite
@task_file n=29 path=tests/cli_send.rs
what: viola send suite: the confirmed send and readback critical path
@task_file n=30 path=tests/cli_version_gate.rs
what: CLI version gate suite
@task_file n=31 path=tests/contract_diag_schema.rs
what: run diagnostics schema contract suite
@task_file n=32 path=tests/cross_exit_causes.rs
what: cross suite over wrapper exit causes
@task_file n=33 path=tests/cross_list_rows.rs
what: cross suite over viola list board rows
@task_file n=34 path=tests/hook_events.rs
what: viola hook event and dialog calls suite
@task_file n=35 path=tests/hook_fail_open.rs
what: hook fail-open suite
@task_file n=36 path=tests/run_cli.rs
what: viola run suite: TUI passthrough entry and outer-PTY runs
@task_file n=37 path=tests/security_negatives_socket_dir.rs
what: security negatives: socket-directory refusals
@task_file n=38 path=tests/security_negatives_strict_modes.rs
what: security negatives: strict-modes refusals
@task_item n=1 part=touches
The a11y assertions that run inside the existing test suites: every suite that holds them now spawns viola through the tied support, and integration and E2E stayed green with the tie in place.
@task_item n=2 part=touches
The rule that a11y failures are nextest failures and a11y cases run in the coverage suite: the selection counts moved (integration 505 to 509, E2E 348 to 369) by the new killed-test and stamp witnesses, none of them an a11y case.
@task_item n=3 part=touches
Zero flakiness with no retries: a CI profile kill timeout override (the stamped-home 20 s kill) now also covers the new killed-test binary; a row holding that bound or that override list is touched.
@task_item n=4 part=touches
The CI test gate that allows no skips and requires coverage: new test binaries joined the gated suites, none skipped.
@task_item n=5 part=touches
The TUI boundary driver and the outer-PTY boundary tests: an outer-PTY wrapper a test spawns is now tied and witnessed gone when its test is killed; the boundary assertions themselves are unchanged.
@task_item n=6 part=touches
Boundary tests sequenced on records: the new witnesses read a pid report the victim writes, not screen content.
@task_item n=7 part=touches
Time bounds fall on the driver only: the new gone-within wait bounds the test driver, never the human.
@task_item n=8 part=touches
No separate a11y driver or suite value: the new binaries are process-hygiene witnesses inside the existing integration and E2E suites, not an a11y driver.
@task_item n=9 part=touches
Live readings never gate: no live claude start was made; the harness session tie and smoke ran against the fake agent.
@task_item n=10 part=touches
The plain-mode and refusal literal pins (fixed CLI strings pinned by literal, refusal layout held by literal pins): the suites holding them had only their viola spawn moved to the tied form; no literal changed.
@task_item n=1 part=untouched
The CLI output contracts themselves: no step line, waiting line, refusal unable/hint pair, exit code, --json document, fault body or code word changed.
@task_item n=2 part=untouched
Colour and SGR decisions (word beside every SGR, amber, bold, dim on stale rows, no colour on failure words) and NAME truncation: no output rendering changed.
@task_item n=3 part=untouched
Screen-reader targets, setups and the manual Orca and keyboard passes: nothing changed in them.
@task_item n=4 part=untouched
The absences (web or graphical frontend, touch surface, rendered page, internationalization, user time limit): no capability added or removed.
@task_item n=5 part=untouched
The CI supply-chain red (a chrono advisory) is not an a11y gate and is not mended in this session; the a11y GitHub Actions workflow rows are not moved by it.
@task_item n=6 part=untouched
TUI passthrough contracts (human keystrokes, viola terminal bytes, focus/mouse/resize, vendor content, feed panic) and the wheel decisions: no runtime behaviour of the wrapper changed.
