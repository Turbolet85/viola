@task_prose part=session
The session ties every long-lived viola child a root test spawns to the test's spawning thread: a parent-death SIGKILL, plus an exit when the recorded parent is already gone, so a killed or panicked test leaves no booted wrapper or fake agent running.
An in-test harness session's supervisor and its stamping viola verify are tied the same way; the viola-harness CLI boot keeps its supervisor untied and behaves as before. Two other in-test viola-e2e boots also stay untied, and that gap is left to the route.
The shared verify stamp used by test homes now counts only when it holds a completion file and the ledger stamps file. A hollow stamp, for example one a CI cache restore left without its files, is left as found, and a next-generation stamp dir is built beside it. Nothing is removed.
A census test now holds the count of root viola spawns outside the tie helper at 0. Before, there were 45 such sites, one more than research had counted.
Two test-only self-spawn environment markers are added outside the VIOLA_ namespace. No product crate, wire contract, dependency or CLI behaviour changed.
@task_file n=1 path=tests/support/tied.rs
what: new root test-support module that ties a spawned viola child to its parent by parent-death signal, records a process tree with start times, and waits within the shared bound before killing and naming any survivor
@task_file n=2 path=tests/cross_killed_test.rs
what: new root end-to-end test binary that kills or panics a victim test and witnesses that no piped, direct, outer-pty or frozen wrapper or fake agent survives, plus the census that no root test spawns viola untied
@task_file n=3 path=crates/viola-e2e/tests/harness_tied.rs
what: new viola-e2e integration test binary that kills a test holding a harness session and witnesses that its supervisor, wrappers and fake agents are gone
@task_file n=4 path=tests/support/home.rs
what: root test-support home helpers: the isolated test home, the shared once-per-build verify stamp dir and its reuse rule, and the revive command
@task_file n=5 path=tests/support/mod.rs
what: root test-support module list
@task_file n=6 path=tests/support/piped.rs
what: root test-support piped wrapper spawner
@task_file n=7 path=tests/support/cli.rs
what: root test-support CLI start helper
@task_file n=8 path=tests/support/start.rs
what: root test-support start-refusal helper
@task_file n=9 path=tests/support/verify.rs
what: root test-support viola verify runners
@task_file n=10 path=tests/cli_verify.rs
what: root integration tests of viola verify and the stamped test home, now witnessing that a hollow stamp is rebuilt and a rebuilt generation reused
@task_file n=11 path=crates/viola-e2e/src/harness/boot.rs
what: the test harness boot of a session: its options, the supervisor spawn and the stamping verify, now tied only when asked
@task_file n=12 path=crates/viola-e2e/src/harness/session.rs
what: the in-test harness session a viola-e2e test boots
@task_file n=13 path=crates/viola-e2e/src/bin/viola-harness.rs
what: the viola-harness test-only binary and its CLI boot
@task_file n=14 path=.config/nextest.toml
what: the nextest profiles, whose CI override kills a stamped-home test at 20 s
@task_file n=15 path=crates/viola-e2e/src/harness/run/perf.rs
what: the harness perf run that boots a session
@task_file n=16 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: viola-e2e integration tests of the harness session lifecycle
@task_file n=17 path=crates/viola-e2e/tests/cli.rs
what: viola-e2e integration tests of the viola-harness CLI
@task_file n=18 path=tests/chaos_feed_panic.rs
what: root chaos test of a screen feed panic, its viola spawns now tied
@task_file n=19 path=tests/chaos_snapshot_replay.rs
what: root chaos test of snapshot replay, its viola spawns now tied
@task_file n=20 path=tests/chaos_wait_vanish.rs
what: root chaos test of a wait whose wrapper vanishes, its viola spawns now tied
@task_file n=21 path=tests/cli_answer.rs
what: root integration tests of the answer verb, their viola spawns now tied
@task_file n=22 path=tests/cli_controls_not_disableable.rs
what: root integration tests that the human controls cannot be disabled, their viola spawns now tied
@task_file n=23 path=tests/cli_error_bodies.rs
what: root integration tests of CLI error bodies, their viola spawns now tied
@task_file n=24 path=tests/cli_fake_agent.rs
what: root integration tests against the fake agent, their viola spawns now tied
@task_file n=25 path=tests/cli_instance_state.rs
what: root integration tests of instance state, their viola spawns now tied
@task_file n=26 path=tests/cli_machine_contract.rs
what: root integration tests of the CLI machine-readable contract, their viola spawns now tied
@task_file n=27 path=tests/cli_output_plain.rs
what: root integration tests of plain human output, their viola spawns now tied
@task_file n=28 path=tests/cli_plugin_install.rs
what: root integration tests of plugin install, their viola spawn now tied
@task_file n=29 path=tests/cli_send.rs
what: root integration tests of the send verb, their viola spawns now tied
@task_file n=30 path=tests/cli_version_gate.rs
what: root integration tests of the CLI version gate, their viola spawn now tied
@task_file n=31 path=tests/contract_diag_schema.rs
what: root contract tests of the diagnostics schema, their viola spawns now tied
@task_file n=32 path=tests/cross_exit_causes.rs
what: root end-to-end tests of child exit causes, holding the one spawn site research missed, now tied
@task_file n=33 path=tests/cross_list_rows.rs
what: root end-to-end tests of list rows, their viola spawns now tied
@task_file n=34 path=tests/hook_events.rs
what: root integration tests of hook events, their viola spawns now tied
@task_file n=35 path=tests/hook_fail_open.rs
what: root integration tests that hooks fail open, their viola spawns now tied
@task_file n=36 path=tests/run_cli.rs
what: root integration tests of viola run, their viola spawns now tied
@task_file n=37 path=tests/security_negatives_socket_dir.rs
what: root security-negative tests of the socket directory check, their viola spawns now tied
@task_file n=38 path=tests/security_negatives_strict_modes.rs
what: root security-negative tests of the strict-modes check, their viola spawns now tied
@task_item n=1 part=touches
The occupied-resources rule over test-private self-spawn markers, which names MCP_SERVE_TEST_CHILD as its precedent, is stale by omission: ROOT_TIED_TEST_CHILD and HARNESS_TIED_TEST_CHILD are new markers of the same kind.
@task_item n=2 part=touches
Environment variables carry the VIOLA_ prefix, and the VIOLA_ namespace is product plumbing only. The two new markers stand outside VIOLA_ as test-only names that no viola build reads, the same exception the existing test-child marker holds.
@task_item n=3 part=touches
Harness variables are never read by viola, and test seams are not a configuration channel. Each new marker is read only by its own test binary.
@task_item n=4 part=touches
Root tests freeze a wrapper through one SIGSTOP guard. A frozen wrapper's survival of a killed test is now witnessed, and the frozen wrapper is tied like every other root spawn.
@task_item n=5 part=touches
Test evidence survives a killed test, and test homes are removed by their owner record only. A killed test now leaves no booted wrapper or fake agent, and its victim writes its report under the witness's own test home.
@task_item n=6 part=touches
Root test waits share one bound. The new wait that every recorded descendant is gone runs within that shared bound before it kills survivors.
@task_item n=7 part=touches
Each test runs in its own viola home under the once-per-build shared verify stamp. The stamp now counts only by its completion file and ledger stamps file, and a hollow one gets a next-generation dir with nothing removed.
@task_item n=8 part=touches
verify is the one stamps writer, and malformed or untrusted stamps read as unverified. The test stamp is still written only by a viola verify run, and that run is now tied when an in-test harness session stamps.
@task_item n=9 part=touches
viola-e2e is test-only, and so is the viola-harness binary. The harness boot gains a tied option and two constructors: an in-test session ties its supervisor, and the CLI boot does not.
@task_item n=10 part=touches
Test-only binaries are kept out of release builds, and the e2e crate's no-op fake-agent feature still holds. There are two new test binaries: a root cross_ end-to-end binary and a viola-e2e integration binary.
@task_item n=11 part=touches
Indirect, through the tie helper: libc is the one OS-binding crate. The parent-death signal is set through libc prctl in a pre_exec hook, and no new crate was added.
@task_item n=12 part=touches
Indirect, through the tie helper: process start time is read from the OS through sysinfo. The test process tree records each pid with its sysinfo start time, so only a matching survivor is killed.
@task_item n=13 part=touches
Linux only, and OS-gated tests name their reason. The tie uses the Linux parent-death signal.
@task_item n=14 part=touches
The zero-panics gate allows no panic line in end-to-end runs. The new panicked-test witness panics its victim on purpose, and the tree wait panics to name any survivor.
@task_item n=15 part=touches
The real claude CLI runs only locally, and tests run against the fake agent. Every new witness boots the fake agent; no live claude was started.
@task_item n=16 part=touches
Homes are kept after each job's end-to-end run. A stamp dir may now stand as viola-stamp-<key>-g<n> beside a hollow viola-stamp-<key> left as found.
@task_fact n=1 slug=occupied-resources
fact: ROOT_TIED_TEST_CHILD and HARNESS_TIED_TEST_CHILD are occupied names. Each is a test-only self-spawn marker outside VIOLA_, read only by its own test binary, the root one carrying a report path and a mode, the harness one a report path.
@task_fact n=2 slug=occupied-resources
fact: Every long-lived viola child a root test spawns is tied to its spawning thread by a parent-death SIGKILL, and a census test holds that no root test spawns viola untied. An in-test harness session's supervisor and stamping verify are tied the same way, but the viola-harness CLI boot's supervisor is not.
@task_fact n=3 slug=occupied-resources
fact: The shared test verify stamp counts only when it holds its completion file and ledger stamps file. A hollow stamp is left standing, and the next generation is built under viola-stamp-<key>-g<n> through a temporary dir renamed into place. No stamp dir is removed.
@task_item n=1 part=untouched
Wire contracts are untouched: channel methods, frames, fault codes, MCP tools and results, CLI verbs, exit codes, JSON documents and event kinds. No IPC method, event, socket or port was added.
@task_item n=2 part=untouched
Dependency policy and chrono timestamps are untouched. No dependency or lockfile changed. The supply-chain advisory red on chrono came from the advisory database, not from this session, and its remedy is asked of the overseer elsewhere.
@task_item n=3 part=untouched
CI workflow decisions are untouched: action pins, permissions, the weekly advisory audit, caching and release. The CI test job passed on an exact cache restore, and no workflow changed.
@task_item n=4 part=untouched
Product state and hooks are untouched: the event log, snapshots, ledger stamp envelope format, pinned copy, plugin and hook contracts. Only the test-side reuse rule for the shared stamp dir moved.
@task_item n=5 part=untouched
The product wrapper, its wheel, send, dialog and readiness decisions are untouched. The wrapper's own lifecycle code did not change; only how tests spawn it did.
@task_item n=6 part=untouched
Crate dependency direction and project directory structure are untouched. No crate was added, and no module boundary between product crates moved.
