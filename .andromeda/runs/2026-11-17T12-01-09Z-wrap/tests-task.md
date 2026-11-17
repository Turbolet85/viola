@task_prose part=session
The session ties every long-lived viola child a root test spawns to the spawning test thread (parent-death SIGKILL, and an exit when the recorded parent is already gone), so a SIGKILLed or panicked test leaves no wrapper or fake agent running.
45 direct root spawn sites of viola were moved onto one tied spawn builder; a new census test (with a planted positive) holds that no root test spawns viola untied, matching any path that ends in the VIOLA constant. The prior count of 44 sites in 24 files was disproved by one more spelling.
The harness boot options gain a tied flag and two constructors: an in-test harness session ties its supervisor and its stamping viola verify, while the CLI boot keeps its supervisor untied and unchanged in behaviour. The in-test boots of the harness lifecycle and harness CLI tests stay untied.
The shared verify stamp now counts only when it holds a completion file and its stamps file. A hollow stamp, such as one a CI cache restored without its files, is never trusted and nothing is removed: the next generation dir (viola-stamp-key-gN) is stamped through a temp dir renamed into place.
New witnesses: killed-test cases for piped, direct, outer-PTY and frozen wrappers, a panicked-test case, and a killed harness-session case, plus two hollow-stamp cases. At its deadline a witness SIGKILLs every recorded survivor whose pid and start time still match, then fails naming it. The CI 20 s stamped-home kill line now covers the killed-test witness binary.
No product code, dependency, manifest or CI workflow changed. CI's test job went green on an exact rust-cache restore. The supply-chain red (a new advisory) is not this session's, and no ignore was added.
@task_file n=1 path=tests/support/tied.rs
what: new shared root test helper: the tied spawn builder for viola (parent-death kill), the recorded process tree, and the bounded gone-within witness that kills and names survivors by pid and start time
@task_file n=2 path=tests/cross_killed_test.rs
what: new root E2E-layer cross-surface test binary: killed-test and panicked-test witnesses that no wrapper or fake agent outlives its test, and the census that no root test spawns viola untied
@task_file n=3 path=crates/viola-e2e/tests/harness_tied.rs
what: new viola-e2e integration test: a killed test leaves no harness session (supervisor, wrappers, fake agents)
@task_file n=4 path=tests/support/home.rs
what: shared root test support for per-test homes and the shared verify stamp (stamped home): the stamp now counts only by its completion file and stamps file, and a hollow stamp gets a next-generation dir; its revive command builds through the tied builder
@task_file n=5 path=tests/support/mod.rs
what: root test support module list, which now declares the tied helper
@task_file n=6 path=tests/support/piped.rs
what: shared root helper that spawns a long-lived wrapper over pipes behind a Drop guard; it now ties the command it is handed
@task_file n=7 path=tests/support/cli.rs
what: shared root helper that starts viola for CLI tests; its start builds through the tied builder
@task_file n=8 path=tests/support/start.rs
what: shared root helper for run and revive start refusals; it builds through the tied builder
@task_file n=9 path=tests/support/verify.rs
what: shared root helper that runs viola verify for stamps and fixtures; both spawn sites build through the tied builder
@task_file n=10 path=tests/cli_verify.rs
what: root integration tests of viola verify and stamped homes; adds the hollow-stamp rebuild and generation-reuse cases
@task_file n=11 path=crates/viola-e2e/src/harness/boot.rs
what: the harness boot (build, stamp, start wrappers, spawn the supervisor); gains the tied boot option with an in-test session constructor (tied) and a CLI constructor (untied), and ties the supervisor and stamping verify when asked
@task_file n=12 path=crates/viola-e2e/src/harness/session.rs
what: the in-test harness session helper, which now boots tied through the session constructor
@task_file n=13 path=crates/viola-e2e/src/bin/viola-harness.rs
what: the viola-harness binary (agent-run boot, status, cleanup, run); its boot builds options through the CLI constructor, untied
@task_file n=14 path=.config/nextest.toml
what: nextest profiles and the CI per-test kill overrides; the 20 s stamped-home kill line adds the killed-test witness binary
@task_file n=15 path=crates/viola-e2e/src/harness/run/perf.rs
what: the harness run perf suite, which boots its own home; it names the new tied option (untied)
@task_file n=16 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: viola-e2e harness lifecycle integration tests (boot, status, cleanup); its in-test boot options name the tied option as untied
@task_file n=17 path=crates/viola-e2e/tests/cli.rs
what: viola-e2e harness CLI integration tests; its in-test boot names the tied option as untied
@task_file n=18 path=tests/chaos_feed_panic.rs
what: root chaos test binary (screen feed panic); only its viola spawn lines changed, onto the tied builder
@task_file n=19 path=tests/chaos_snapshot_replay.rs
what: root chaos test binary (snapshot replay); only its viola spawn lines changed, onto the tied builder
@task_file n=20 path=tests/chaos_wait_vanish.rs
what: root chaos test binary (wait on a vanished wrapper); only its viola spawn lines changed, onto the tied builder
@task_file n=21 path=tests/cli_answer.rs
what: root CLI integration test binary for viola answer; only its viola spawn lines changed, onto the tied builder
@task_file n=22 path=tests/cli_controls_not_disableable.rs
what: root CLI integration test binary that security controls are not disableable; only its viola spawn lines changed, onto the tied builder
@task_file n=23 path=tests/cli_error_bodies.rs
what: root CLI integration test binary for error bodies; only its viola spawn lines changed, onto the tied builder
@task_file n=24 path=tests/cli_fake_agent.rs
what: root CLI integration test binary over the fake agent; only its viola spawn lines changed, onto the tied builder
@task_file n=25 path=tests/cli_instance_state.rs
what: root CLI integration test binary for instance state; only its viola spawn lines changed, onto the tied builder
@task_file n=26 path=tests/cli_machine_contract.rs
what: root CLI integration test binary for the --json machine contract; only its viola spawn lines changed, onto the tied builder
@task_file n=27 path=tests/cli_output_plain.rs
what: root CLI integration test binary for plain output without a TTY; only its viola spawn lines changed, onto the tied builder
@task_file n=28 path=tests/cli_plugin_install.rs
what: root CLI integration test binary for plugin install; only its viola spawn line changed, onto the tied builder
@task_file n=29 path=tests/cli_send.rs
what: root CLI integration test binary for viola send; only its viola spawn lines changed, onto the tied builder
@task_file n=30 path=tests/cli_version_gate.rs
what: root CLI integration test binary for the CLI version gate; only its viola spawn line changed, onto the tied builder
@task_file n=31 path=tests/contract_diag_schema.rs
what: root contract test binary for diagnostics schema conformance; only its viola spawn lines changed, onto the tied builder
@task_file n=32 path=tests/cross_exit_causes.rs
what: root cross-surface test binary for exit causes; its one spawn site in a wider spelling, missed by the prior count, moved onto the tied builder
@task_file n=33 path=tests/cross_list_rows.rs
what: root cross-surface test binary for list rows; only its viola spawn lines changed, onto the tied builder
@task_file n=34 path=tests/hook_events.rs
what: root hook test binary for hook events; only its viola spawn lines changed, onto the tied builder
@task_file n=35 path=tests/hook_fail_open.rs
what: root hook test binary for hook fail-open; only its viola spawn lines changed, onto the tied builder
@task_file n=36 path=tests/run_cli.rs
what: root integration test binary for viola run; only its viola spawn lines changed, onto the tied builder
@task_file n=37 path=tests/security_negatives_socket_dir.rs
what: root security negative test binary for the socket directory check; only its viola spawn lines changed, onto the tied builder
@task_file n=38 path=tests/security_negatives_strict_modes.rs
what: root security negative test binary for strict modes; only its viola spawn lines changed, onto the tied builder
@task_item n=1 part=touches
decision/wrapper-drop-guard: test wrappers held in a Drop guard is now not the whole lifecycle. Every long-lived viola child a root test spawns is also tied by a parent-death kill, so a SIGKILLed test leaves none. The row is stale by omission.
@task_item n=2 part=touches
decision/wrapper-spawned-with-guard: a long-lived wrapper spawned with a drop guard now also needs the tied spawn builder. The drop guard alone no longer covers a killed test.
@task_item n=3 part=touches
decision/one-verify-stamp-per-build: a stamp counts only by its completion file and stamps file, and a hollow one gets a next-generation dir (-gN) with nothing removed. The row has no completion-file or generation clause.
@task_item n=4 part=touches
decision/rust-cache-only-cache and decision/rust-cache-only-in-ci: the stamp now survives whatever a rust-cache restore leaves (a hollow stamp is rebuilt). CI's test job read green on an exact restore of the saved key, with no cache deleted. The rust-cache rule itself is unchanged.
@task_item n=5 part=touches
decision/ci-stamped-home-kill-line: the 20 s CI kill line for stamped-home tests now also covers the new killed-test witness binary. The filter value that row holds moved.
@task_item n=6 part=touches
decision/verify-driven-tests-20s-kill: the per-test kill that bounds verify-driven tests reaches one more binary through the same override.
@task_item n=7 part=touches
decision/supervisor-ordinary-child and decision/supervisor-spawned-only-by-boot: an in-test harness session's supervisor is now tied to the test's thread, while the CLI boot's supervisor stays an untied ordinary child.
@task_item n=8 part=touches
decision/harness-boot-creates-stamps-and-starts and contract/test-harness/boot: boot gains a tied option and two constructors. The stamping viola verify is tied when the boot is tied, and CLI boot behaviour is unchanged.
@task_item n=9 part=touches
decision/cleanup-removes-home-record-and-supervisor and decision/e2e-boot-model-by-cleanup: a killed in-test session no longer relies on cleanup to end its supervisor, wrappers and fake agents. The tie ends them.
@task_item n=10 part=touches
decision/pid-start-time-verified-kill: the new gone-within witness SIGKILLs only survivors whose recorded pid and start time still match. It is a new user of that rule.
@task_item n=11 part=touches
decision/within-7s-wait-bound and decision/root-wait-bound: the killed-test witnesses wait within WITHIN. The victim pre-stamps so that only the stamp copy falls inside WITHIN.
@task_item n=12 part=touches
decision/one-shared-test-helper and decision/shared-e2e-helpers: a new shared root helper (the tied spawn builder and the process-tree witness) is the one copy every root spawn site and witness uses.
@task_item n=13 part=touches
decision/e2e-layer-by-binary-prefix and decision/integration-binary-surface-prefix: a new cross-prefixed root E2E-layer binary and a new viola-e2e integration binary were added. The selection counts moved (integration 505 to 509, E2E 348 to 369).
@task_item n=14 part=touches
decision/test-started-viola-drops-env and decision/test-viola-drops-session-vars: every test-started viola now builds through the tied builder. Any rule over how a test-started viola command is built applies to that builder.
@task_item n=15 part=touches
decision/stamps-written-only-by-verify, decision/stamps-only-from-verify and decision/verify-sole-stamp-writer: test support now writes a completion file into a stamp dir after verify returns and renames a temp dir into place. Verify still writes the stamps file.
@task_item n=16 part=touches
decision/single-writers and decision/single-writer-state-files: the stamp dir has a new completion file and generation naming. A row on stamp-file writers may need the completion file named.
@task_item n=17 part=touches
decision/guarded-paths-start-stamped, decision/fake-agent-replay-for-stamps and decision/stamping-verify-passes-transcript: these rows rule over the stamped-home chain, which keeps its signature and callers but now rebuilds a hollow stamp.
@task_item n=18 part=touches
decision/owner-record-removed-last and decision/test-home-owner-record-sweep: a hollow stamp is left as found and never removed. A stamp-dir generation stands beside it.
@task_item n=19 part=touches
decision/fake-agent-own-process-group and decision/fake-agent-children-own-process-group: the killed-test witnesses assert that the fake agent dies with its tied wrapper. These rows rule over the process tree the witnesses read.
@task_item n=20 part=touches
decision/killed-test-child-uninstrumented: the new victims are killed test children, the case this row rules over.
@task_item n=21 part=touches
decision/bounded-remove-the-guard-witness and decision/planted-hygiene-violations: the new census proves itself on a planted positive (reads 1 there, 0 elsewhere). Its controls (tied-pdeath, sup-tied, stamp-hollow) hold.
@task_item n=22 part=touches
decision/no-sleep-synchronization and decision/no-test-retries-or-sleeps: the new witnesses poll a bounded deadline on recorded processes. These rows rule over their waits.
@task_fact n=1 slug=integration-test-strategy
fact: Every long-lived viola child a root test spawns is tied to its spawning test thread by a parent-death SIGKILL (and exits when the recorded parent is already gone), so a SIGKILLed or panicked test leaves no wrapper or fake agent; the Drop guard still covers the normal end.
@task_fact n=2 slug=integration-test-strategy
fact: A census test holds that no root test spawns viola untied, matching any Command::new path equal to or ending in the VIOLA constant over every test source recursively, proven on a planted positive.
@task_fact n=3 slug=integration-test-strategy
fact: The parent-death signal follows the spawning thread, so a tied viola child is spawned only on a thread that outlives it.
@task_fact n=4 slug=test-data-bootstrap
fact: A verify stamp counts only when it holds its completion file and its stamps file; a hollow stamp is never trusted and never removed, and the next generation dir viola-stamp-key-gN is stamped instead through a temp dir renamed into place, then reused without stamping again.
@task_fact n=5 slug=5-command-implementation
fact: An in-test harness session ties its supervisor and its stamping viola verify to the test's thread; the agent-run CLI boot leaves its supervisor untied so it outlives boot; the in-test boots of the harness lifecycle and harness CLI tests are untied, a known gap.
@task_fact n=6 slug=pid-file
fact: A killed-test witness records the victim's process tree with start times, waits within WITHIN, then SIGKILLs every survivor whose pid and start time still match and fails naming each survivor by subject.
@task_item n=1 part=untouched
Product behaviour contracts (wrapper channel and JSON-RPC errors, MCP tools, CLI exit codes and --json documents, hook decision bodies, send, wait, list, revive and run refusals): no product code changed, and the moved test spawn sites assert the same contracts.
@task_item n=2 part=untouched
Supply-chain and advisory rows (cargo-deny gate, no advisory ignored, fail-closed supply-chain job, weekly advisories): no manifest, lockfile, deny config or ignore changed. The supply-chain red is a new upstream advisory and not this session's.
@task_item n=3 part=untouched
Coverage floors, the coverage threshold, mutation gate thresholds and cadence, and the mutants kill lines: no threshold, floor or mutants profile moved.
@task_item n=4 part=untouched
Process log format, corr fields and diagnostics schema rows: no log line, event or field was added or changed.
@task_item n=5 part=untouched
Fake agent command line, receipt, script and screen contracts, and the recorded fixtures and their hygiene: the fake agent and fixtures are unchanged.
@task_item n=6 part=untouched
CI workflow rows (artifacts, secret scan, SHA-pinned actions, permissions, zizmor, MSRV, pinned toolchain): no workflow or CI job changed.
@task_item n=7 part=untouched
Harness status, logs, gate and pre-push contracts and their exit codes, and the harness session record fields: their shapes and codes are unchanged.
@task_item n=8 part=untouched
Absences (database, outbound HTTP, browser suite, Windows and macOS, deploy stage): no capability added or removed.
