@task_prose part=session
The session ties every long-lived viola child a root test spawns to its test: a pre_exec sets the parent-death signal to SIGKILL and exits when the recorded parent is gone, so a booted wrapper and its fake agent end with their test, killed or panicked.
An in-test harness session's supervisor and its stamping viola verify are tied the same way; the CLI boot of the test harness stays untied, unchanged in behaviour.
The shared verify stamp of the test homes counts only with a completion file and its stamps file; a hollow stamp restored without its files is left standing as found, nothing removed, and a next-generation stamp dir is built beside it.
A census test now finds no untied direct viola spawn under the root tests; it measured 45 sites, one more than research said, all moved behind the tie.
A test-side witness records the victim's process tree by pid and start time and, at its deadline, SIGKILLs only the recorded survivors whose pid and start time still match.
CI on the pushed commit read the test job green on an exact rust-cache restore with no cache deleted; the supply-chain job is red on a new unmaintained-crate advisory for chrono, not mended here, no ignore added and no re-run, its remedy asked of the overseer.
@task_file n=1 path=tests/support/tied.rs
what: new root test support that ties a spawned viola child to its test (parent-death SIGKILL), builds the tied viola command, records a process tree and kills recorded survivors at a deadline
@task_file n=2 path=tests/cross_killed_test.rs
what: new root E2E test binary: the census that no root test spawns viola untied, and the killed-test and panicked-test witnesses that no wrapper or fake agent outlives its test
@task_file n=3 path=crates/viola-e2e/tests/harness_tied.rs
what: new test-harness integration binary witnessing that a killed test leaves no harness session, its supervisor or its instances running
@task_file n=4 path=tests/support/home.rs
what: root test support for test homes: the shared verify stamp dir, its completion check and next-generation rebuild, the stamped home and the revive command
@task_file n=5 path=tests/support/mod.rs
what: root test support module list, now holding the tie module
@task_file n=6 path=tests/support/piped.rs
what: root test support that spawns a piped wrapper, now tied
@task_file n=7 path=tests/support/cli.rs
what: root test support that starts viola from the CLI, now through the tied command
@task_file n=8 path=tests/support/start.rs
what: root test support for refused starts, now through the tied command
@task_file n=9 path=tests/support/verify.rs
what: root test support that runs viola verify for tests, now through the tied command
@task_file n=10 path=tests/cli_verify.rs
what: viola verify integration tests, gaining the hollow-stamp rebuild and generation-reuse cases
@task_file n=11 path=crates/viola-e2e/src/harness/boot.rs
what: the test harness boot: boot options with the new tied field and session and CLI constructors, the supervisor spawn and the stamping viola verify
@task_file n=12 path=crates/viola-e2e/src/harness/session.rs
what: the test harness in-test session, booting through the tied session options
@task_file n=13 path=crates/viola-e2e/src/bin/viola-harness.rs
what: the viola-harness binary's CLI boot options, untied
@task_file n=14 path=.config/nextest.toml
what: nextest CI profile, the stamped-home kill override now covering the killed-test binary
@task_file n=15 path=crates/viola-e2e/src/harness/run/perf.rs
what: the test harness perf run, naming the tied field on its boot options
@task_file n=16 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: test harness lifecycle tests, their in-test boot left untied
@task_file n=17 path=crates/viola-e2e/tests/cli.rs
what: test harness CLI tests, their in-test boot left untied
@task_file n=18 path=tests/chaos_feed_panic.rs
what: chaos test of a screen feed panic, its viola spawns moved behind the tie
@task_file n=19 path=tests/chaos_snapshot_replay.rs
what: chaos test of a replayed snapshot, its viola spawns moved behind the tie
@task_file n=20 path=tests/chaos_wait_vanish.rs
what: chaos test of a vanishing wait, its viola spawns moved behind the tie
@task_file n=21 path=tests/cli_answer.rs
what: viola answer CLI tests, their viola spawns moved behind the tie
@task_file n=22 path=tests/cli_controls_not_disableable.rs
what: tests that no config, env var or flag switches off a control, their viola spawns moved behind the tie
@task_file n=23 path=tests/cli_error_bodies.rs
what: CLI error output tests, their viola spawns moved behind the tie
@task_file n=24 path=tests/cli_fake_agent.rs
what: CLI tests over the fake agent, their viola spawns moved behind the tie
@task_file n=25 path=tests/cli_instance_state.rs
what: CLI instance state tests, their viola spawns moved behind the tie
@task_file n=26 path=tests/cli_machine_contract.rs
what: CLI machine-readable contract tests, their viola spawns moved behind the tie
@task_file n=27 path=tests/cli_output_plain.rs
what: CLI plain human output tests, their viola spawns moved behind the tie
@task_file n=28 path=tests/cli_plugin_install.rs
what: viola plugin install tests, their viola spawns moved behind the tie
@task_file n=29 path=tests/cli_send.rs
what: viola send CLI tests, their viola spawns moved behind the tie
@task_file n=30 path=tests/cli_version_gate.rs
what: viola run version gate tests, their viola spawns moved behind the tie
@task_file n=31 path=tests/contract_diag_schema.rs
what: diagnostics schema contract tests, their viola spawns moved behind the tie
@task_file n=32 path=tests/cross_exit_causes.rs
what: cross-cutting exit cause tests, including the one viola spawn site research missed, moved behind the tie
@task_file n=33 path=tests/cross_list_rows.rs
what: viola list row tests, their viola spawns moved behind the tie
@task_file n=34 path=tests/hook_events.rs
what: viola hook event tests, their viola spawns moved behind the tie
@task_file n=35 path=tests/hook_fail_open.rs
what: tests that the hook fails open, their viola spawns moved behind the tie
@task_file n=36 path=tests/run_cli.rs
what: viola run CLI tests, their viola spawns moved behind the tie
@task_file n=37 path=tests/security_negatives_socket_dir.rs
what: security negative tests of the socket-directory check, their viola spawns moved behind the tie
@task_file n=38 path=tests/security_negatives_strict_modes.rs
what: security negative tests of the strict-modes check, their viola spawns moved behind the tie
@task_item n=1 part=touches
Test-side removal only of test-created dirs: a hollow stamp dir is left standing as found and a next-generation dir is built beside it, nothing removed; check the rule still reads true and whole.
@task_item n=2 part=touches
Tests use their own home: the stamped test homes now copy from a stamp that counts only with its completion file and stamps file, and the witnesses write their victim reports under their own test home.
@task_item n=3 part=touches
Only viola verify writes the stamps file and stamps are touched only after a strict-modes check: the test-side stamp now also writes a completion file beside the stamped home after the stamper returns; the stamps file itself is still viola verify's.
@task_item n=4 part=touches
Dialog answers need a verify stamp and the version gate reads stamps strictly: the test homes' stamp reuse changed, the product's stamp reads did not.
@task_item n=5 part=touches
New seam or environment read needs a log entry: two test-only self-spawn markers, ROOT_TIED_TEST_CHILD and HARNESS_TIED_TEST_CHILD, outside VIOLA_*, each read by its own test binary alone and by no viola build.
@task_item n=6 part=touches
Test seams only under the fake-agent feature and compiled out of release: the two new markers live in test binaries, not in viola; check no row reads them as viola seams.
@task_item n=7 part=touches
sysinfo for process identity: the test-side tree record and the deadline kill identify a process by pid and start time read through sysinfo.
@task_item n=8 part=touches
Owner-only modes: the stamp completion file and the victims' report files are written 0600, the report renamed into place.
@task_item n=9 part=touches
Release build refuses test-only binaries and stays free of test-only features: two new test binaries and the test harness boot change are test-side only.
@task_item n=10 part=touches
The cargo-deny gate in CI, the weekly advisory run and the critical CVE response rules: the supply-chain job is red at the pushed commit on RUSTSEC-2026-0543, chrono unmaintained, reached through viola, viola-state, rmcp and schemars; no ignore was added because chrono is an enabled dependency, no re-run, its remedy asked of the overseer.
@task_item n=11 part=touches
Reasoned advisory ignores for disabled rmcp features: the ruling that an ignore is admitted only for an advisory confined to a disabled feature was applied to the chrono advisory and refused one.
@task_item n=12 part=touches
Nothing launched as root and the hermetic pre-push child environment: the pre-push gate ran green with the tie in place; the tie sets only the parent-death signal on the child.
@task_item n=13 part=touches
No rust-cache in the release workflow: CI's test job restored the saved rust-cache key exactly, no cache deleted and no new key saved; the release workflow was not changed.
@task_fact n=1 slug=security-anti-patterns-never-do-these
fact: Test-side code signals only processes it recorded: a witness kills a survivor only when its pid and its start time both still match the tree it recorded, never by pid alone or by name.
@task_fact n=2 slug=security-anti-patterns-never-do-these
fact: Every long-lived viola child a root test spawns, and an in-test harness session's supervisor, is tied to its test by a parent-death SIGKILL, so a killed or panicked test leaves no wrapper or fake agent running; the test harness CLI boot is the named exception, untied.
@task_item n=1 part=untouched
Viola's own IPC, channel methods, MCP tools, socket directory, peer euid and server verification: no method, endpoint, event, socket or port added or changed.
@task_item n=2 part=untouched
Input validation of paste text, hook payloads, MCP input and channel frames: no product parser or validator changed.
@task_item n=3 part=untouched
Verify's own probe directories removed by drop guards and events emitted from a Drop guard: they only share the word drop guard with the test wrapper's lifecycle.
@task_item n=4 part=untouched
Dependency pins, crate sources, licences, C-building crates, the MSRV and toolchain pins: no dependency added or moved, libc and sysinfo already present.
@task_item n=5 part=untouched
Workflows, action SHA pins, workflow permissions, zizmor and the artifact canary scan: no workflow changed.
@task_item n=6 part=untouched
Secret management, the CLAUDE credential strip, the live rig and its credentials copy: no live claude start in this session.
@task_item n=7 part=untouched
Data protection of the event log, diagnostics, pinned binary and plugin files and fixtures: no product writer changed.
@task_item n=8 part=untouched
Error handling, refusal codes and CLI exit codes: no error contract moved.
