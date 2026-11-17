@task_prose part=session
The session changed test infrastructure only: a wrapper or fake agent that a test boots now ends with its test, even when the test is killed or panics.
Every root test now spawns viola through one tied builder. It sets a parent-death SIGKILL, and the child exits 1 before exec if its parent is already gone. A census test confirms that no root test spawns viola untied: 45 sites moved, 0 remain.
An in-test harness session ties its supervisor and its stamping viola verify to the test. The viola-harness CLI boot is unchanged in behaviour, and its supervisor stays untied.
The shared verify stamp now counts only with a completion file and a stamps file. A hollow stamp is rebuilt as a next generation directory and nothing is removed.
No verb, flag, output line, exit code, colour, glyph, IPC method, socket or port was added or changed. No spec master was edited and nothing was retired.
@task_file n=1 path=tests/support/tied.rs
what: new root test support that ties a spawned viola child to its test (parent-death kill) and checks that a recorded process tree is gone within a bound
@task_file n=2 path=tests/cross_killed_test.rs
what: new root E2E test binary proving that a killed or panicked test leaves no wrapper or fake agent running, plus the census of untied viola spawns
@task_file n=3 path=crates/viola-e2e/tests/harness_tied.rs
what: new harness integration test proving that a killed test leaves no harness session supervisor or instance running
@task_file n=4 path=tests/support/home.rs
what: root test home support, the shared verify stamp per build (stamp_dir, stamp_dir_in, stamp_counts, stamped_home); edited, the stamp counts only by its completion file and stamps file and is rebuilt as a generation dir
@task_file n=5 path=tests/support/mod.rs
what: root test support module list; edited to add the tied module
@task_file n=6 path=tests/support/piped.rs
what: root test support for a piped wrapper child; edited, the command it spawns is tied
@task_file n=7 path=tests/support/cli.rs
what: root test support starting viola run for a test; edited, built through the tied builder
@task_file n=8 path=tests/support/start.rs
what: root test support for run start refusals; edited, built through the tied builder
@task_file n=9 path=tests/support/verify.rs
what: root test support driving viola verify in tests; edited, two spawn sites built through the tied builder
@task_file n=10 path=tests/cli_verify.rs
what: viola verify integration tests; edited, two witnesses added that a hollow stamp is rebuilt and a rebuilt generation is reused
@task_file n=11 path=crates/viola-e2e/src/harness/boot.rs
what: the harness boot of a session supervisor (BootOptions); edited, a tied field and the for_session and for_cli constructors added, with the supervisor and the stamping verify tied only when asked
@task_file n=12 path=crates/viola-e2e/src/harness/session.rs
what: the in-test harness session entry; edited to boot through the tied for_session options
@task_file n=13 path=crates/viola-e2e/src/bin/viola-harness.rs
what: the viola-harness CLI binary (boot, status, cleanup); edited to boot through for_cli, untied, with no change in behaviour
@task_file n=14 path=.config/nextest.toml
what: nextest profiles; edited, the CI stamped-home 20 s kill override also covers the cross_killed_test binary
@task_file n=15 path=crates/viola-e2e/src/harness/run/perf.rs
what: the harness perf run; edited, one BootOptions literal names the new tied field
@task_file n=16 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: harness lifecycle integration tests; edited, its BootOptions names tied false
@task_file n=17 path=crates/viola-e2e/tests/cli.rs
what: viola-harness CLI integration tests; edited, its BootOptions names tied false
@task_file n=18 path=tests/chaos_feed_panic.rs
what: root chaos test of a feed panic; edited, its viola spawns go through the tied builder
@task_file n=19 path=tests/chaos_snapshot_replay.rs
what: root chaos test of snapshot replay; edited, its viola spawns go through the tied builder
@task_file n=20 path=tests/chaos_wait_vanish.rs
what: root chaos test of viola wait when the instance vanishes; edited, its viola spawns go through the tied builder
@task_file n=21 path=tests/cli_answer.rs
what: viola answer integration tests; edited, its viola spawns go through the tied builder
@task_file n=22 path=tests/cli_controls_not_disableable.rs
what: integration tests that controls cannot be disabled; edited, its viola spawns go through the tied builder
@task_file n=23 path=tests/cli_error_bodies.rs
what: integration tests of CLI error and refusal bodies; edited, its viola spawns go through the tied builder
@task_file n=24 path=tests/cli_fake_agent.rs
what: integration tests driving the fake agent through viola; edited, its viola spawns go through the tied builder
@task_file n=25 path=tests/cli_instance_state.rs
what: integration tests of instance state reads; edited, its viola spawns go through the tied builder
@task_file n=26 path=tests/cli_machine_contract.rs
what: integration tests of the --json machine contract; edited, its viola spawns go through the tied builder
@task_file n=27 path=tests/cli_output_plain.rs
what: integration tests of plain CLI output; edited, its viola spawns go through the tied builder
@task_file n=28 path=tests/cli_plugin_install.rs
what: viola plugin install integration tests; edited, its viola spawn goes through the tied builder
@task_file n=29 path=tests/cli_send.rs
what: viola send integration tests; edited, its viola spawns go through the tied builder
@task_file n=30 path=tests/cli_version_gate.rs
what: integration tests of the CLI version gate; edited, its viola spawn goes through the tied builder
@task_file n=31 path=tests/contract_diag_schema.rs
what: contract tests of the diagnostic schema; edited, its viola spawns go through the tied builder
@task_file n=32 path=tests/cross_exit_causes.rs
what: cross tests of exit causes; edited, the 45th spawn site, outside the planned census pattern, now goes through the tied builder
@task_file n=33 path=tests/cross_list_rows.rs
what: cross tests of viola list rows; edited, its viola spawns go through the tied builder
@task_file n=34 path=tests/hook_events.rs
what: viola hook event integration tests; edited, its viola spawns go through the tied builder
@task_file n=35 path=tests/hook_fail_open.rs
what: viola hook fail-open integration tests; edited, its viola spawns go through the tied builder
@task_file n=36 path=tests/run_cli.rs
what: viola run integration tests; edited, its viola spawns go through the tied builder
@task_file n=37 path=tests/security_negatives_socket_dir.rs
what: security negative tests of the socket directory refusal; edited, its viola spawns go through the tied builder
@task_file n=38 path=tests/security_negatives_strict_modes.rs
what: security negative tests of the strict-modes refusal; edited, its viola spawns go through the tied builder
@task_item n=1 part=touches
Indirect, through the tied builder: a viola child whose test dies is now SIGKILLed by its parent-death signal and prints nothing. Check whether any CLI exit-code or stream row says a wrapped run always ends with one of its listed codes. The killed case is test-only and adds no code.
@task_item n=2 part=touches
Indirect: a tied child whose parent is already gone exits 1 before it execs viola. That is the test harness's exit, not viola's. Check that no exit-code row reads it as a new meaning of CLI exit code 1.
@task_item n=3 part=touches
Indirect, through the stamp: the shared test stamp is a home where viola verify ran, and it now counts only when it holds home/ledger/stamps.json and a completion file. Check the viola verify output and refusal rows about the stamps.json update for any claim this test-side rule contradicts. The verify output itself is unchanged.
@task_item n=4 part=touches
Indirect: in an in-test harness session, the stamping viola verify and the session supervisor are now tied to the test, and the viola-harness CLI boot is not. Check any row about how viola verify or viola run behave when their caller dies.
@task_item n=5 part=touches
Names: two test-only self-spawn environment markers, ROOT_TIED_TEST_CHILD and HARNESS_TIED_TEST_CHILD, are now occupied outside VIOLA_*. No viola build reads them. design-system's name rows are command verbs only, and no verb was taken or freed.
@task_item n=6 part=touches
Callers: an LLM driver session or a test that boots viola through the harness now owns its children's lifetime. Check the CLI callers rows for any claim about who ends a wrapper.
@task_item n=1 part=untouched
Brand identity and strip expression: the flight-progress strip, wheel, dialog-pending cocked strip, stale dimming and stable slots. No output a human sees changed.
@task_item n=2 part=untouched
Colour palette, accent amber, colour tokens, colour depth and the colour decision order. No styling or colour logic changed.
@task_item n=3 part=untouched
Typography, spacing and iconography: ASCII-only output, glyphs, arrows, separators and column widths.
@task_item n=4 part=untouched
The help output verb groups, the flat verbs, and the link, unlink, last and wait output contracts. No verb or output line changed.
@task_item n=5 part=untouched
The web-spa surface absence, the rendered-page absence and the self-validation protocol checks (squint, swap, signature).
