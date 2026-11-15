@task_prose part=session
The session removed the status-line interception whole: no viola hook statusline arm, no /bin/sh -c shell-out and its 5 s deadline, no per-session settings.json override, no --settings handed to the child by run, no status-line source read, no statusline_command in the instance snapshot.
The two ledger rows statusline-override and statusline-shell are retired with ShellVerdict: the relied set is 21 rows (was 23); verify prints 21 step lines and stamped <ver>  21 pass  0 fail; --settings stays an agent-crate flag used by verify's Runs C and D alone.
The fake agent lost its statusline-echo mode word (agents is the one left), --statusline-stdin and its statusline receipt kind; --settings stays accepted, its value consumed and ignored; the harness boot lost --statusline-echo.
The StatusLine.default-1 fixtures of both CLI versions, their fixture schema and two hook_stdin fuzz seeds (12 to 10) are deleted; the diag-line schema loses subject statusline-shell, hook_event statusline and process-exit's shell_exit_status; the hook keeps one capture arm, hook <event> --capture.
The word statusline now maps to no hook event: unknown-event exit 0, nothing read or written; new tests pin that, a start passing no --settings and writing no settings file, a 0.1.0 snapshot holding the retired key read whole, and stamps holding the two retired row ids read on the relied rows alone; snapshot and stamps envelope v stays 1.
@task_file n=1 path=crates/viola-agent-claude/src/statusline.rs
what: the agent crate's status-line module: the settings override document, the probe document and the /bin/sh -c shell argv of the status-line command (deleted)
@task_file n=2 path=src/cmd/hook/statusline.rs
what: the viola hook statusline entry point, its shell-out under the status-line deadline and its hidden --capture --shell probe arm (deleted)
@task_file n=3 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's statusline-echo mode and its status-line command execution with --statusline-stdin (deleted)
@task_file n=4 path=fixtures/claude/2.1.287/StatusLine.default-1.json
what: the recorded StatusLine fixture variant of CLI version 2.1.287, the one holding workspace.repo (deleted)
@task_file n=5 path=fixtures/claude/2.1.302/StatusLine.default-1.json
what: the recorded StatusLine fixture variant of CLI version 2.1.302 (deleted)
@task_file n=6 path=fuzz/corpus/hook_stdin/statusline-absent
what: a hook_stdin fuzz corpus seed for the status-line payload (deleted)
@task_file n=7 path=fuzz/corpus/hook_stdin/statusline-epoch
what: a hook_stdin fuzz corpus seed for the status-line payload (deleted)
@task_file n=8 path=schemas/claude-statusline-fixture.v1.json
what: the tolerant schema the recorded StatusLine fixtures were checked against (deleted)
@task_file n=9 path=tests/hook_statusline.rs
what: the integration suite of viola hook statusline: passthrough, strict-home gate, shell bound and frame cap (deleted)
@task_file n=10 path=tests/support/statusline.rs
what: the shared test helper planting the status-line source and reading the override (deleted)
@task_file n=11 path=crates/viola-agent-claude/src/ledger.rs
what: the ledger rows and the relied set the version gate and verify probes read, now 21 rows
@task_file n=12 path=crates/viola-agent-claude/src/lib.rs
what: the agent crate's claude flag literals, --plugin-dir and --settings beside it
@task_file n=13 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit cases of the stamps: a widened ledger refused, a stamp holding retired row ids read on the relied rows alone
@task_file n=14 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases pinning the relied-row verdict vector of the ledger checks
@task_file n=15 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: unit cases of verify's typed runs over the ledger rows
@task_file n=16 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot writer and reader, its fields and the skipping of unknown keys
@task_file n=17 path=crates/viola-e2e/src/bin/viola-harness.rs
what: the test harness command line: boot and its options
@task_file n=18 path=crates/viola-e2e/src/harness/boot.rs
what: the harness boot command: home, stamps and wrapper start
@task_file n=19 path=crates/viola-e2e/src/harness/run.rs
what: the harness run command and the local-live suite's expected verify lines and ledger rows
@task_file n=20 path=crates/viola-e2e/src/harness/session.rs
what: the harness session options and their defaults
@task_file n=21 path=schemas/diag-line.v1.json
what: the diagnostics line schema: the subject and hook_event enums and the process-exit fields
@task_file n=22 path=fixtures/viola-0.1.0/retired-words.json
what: the committed retired-words fixture, now holding the retired snapshot key, ledger row ids, hook word, source file and diag subject
@task_file n=23 path=fixtures/viola-0.1.0/snapshot.json
what: the committed 0.1.0 instance snapshot fixture, read by the compat cases
@task_file n=24 path=src/bin/viola-fake-agent.rs
what: the fake agent's command line: mode words and argv-only options
@task_file n=25 path=src/cmd/hook.rs
what: viola hook's dispatch of hook event words, the unknown-event exit and the capture arm
@task_file n=26 path=src/cmd/run.rs
what: viola run's start: the checks before the child, the instance files written and the snapshot record
@task_file n=27 path=src/cmd/verify.rs
what: viola verify: the typed runs, fixture recording and the stamped line
@task_file n=28 path=src/cmd/verify/typed/runs.rs
what: verify's typed Runs B, C and D and the settings passed to them
@task_file n=29 path=src/run/mod.rs
what: the wrapper's child launch: the claude child's argv
@task_file n=30 path=tests/hook_fail_open.rs
what: the hook fail-open integration suite, now with the retired hook word read as an unknown event writing nothing
@task_file n=31 path=tests/cli_instance_state.rs
what: the integration suite of the instance dir and snapshot a run leaves, and of the user settings left as found
@task_file n=32 path=tests/cli_fake_agent.rs
what: the fake agent's command-line and exit integration suite
@task_file n=33 path=tests/cli_verify/runs.rs
what: the integration cases of verify's runs and their summary counts
@task_file n=34 path=tests/contract_bounded_readers.rs
what: the contract check that every external reader is bounded, listing the readers
@task_file n=35 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene run-time walk and its scrub classes
@task_file n=36 path=tests/contract_os_gated_tests.rs
what: the register of OS-gated tests, each named with a reason
@task_file n=37 path=tests/tui_passthrough.rs
what: the TUI passthrough suite through an outer PTY
@task_file n=38 path=.config/nextest.toml
what: the nextest profiles, the CI kill-line override filter naming the stamped-home test binaries
@task_file n=39 path=Cargo.toml
what: the root package's integration test target list
@task_file n=40 path=tests/contract_fake_agent_drift.rs
what: the fake-agent drift check comparing fake transcripts to recorded fixtures byte for byte
@task_item n=1 part=touches
The status-line decisions retire whole with their subject: statusline-command-needs-strict-home, statusline-passthrough, statusline-shell-out-frame-cap, statusline-command-after-strict-modes, statusline-command-never-runs-unsafe, statusline-command-shell-and-bound, statusline-source-read-only, statusline-source-written-by-test.
@task_item n=2 part=touches
The status-line contracts retire: viola-hook-statusline/passthrough, hook-statusline/passthrough, statusline-hook/stdout, statusline-settings-source/source-file, instance-settings/statusline-override, viola-run-start/settings-override.
@task_item n=3 part=touches
Fake agent contracts: fake-agent-hooks/status-line-command and fake-agent-receipt/statusline retire; fake-agent-receipt/kind loses the statusline kind; fake-agent-cli/argv-only-options now has one mode word (agents), no --statusline-stdin, and --settings accepted with its value ignored.
@task_item n=4 part=touches
contract/harness-boot/statusline-echo retires: harness boot takes no --statusline-echo; harness session options lose statusline_echo.
@task_item n=5 part=touches
decision/plugin-rewritten-on-start names plugin and settings rewritten on start: a start now writes no settings.json, only the plugin.
@task_item n=6 part=touches
The workspace.repo exception clauses lose their subject (no fixture holds a workspace.repo once the StatusLine variants left): decision/fixture-hygiene-walk, decision/recorder-scrub, decision/no-production-data-in-tests; the except clauses retire, rows kept.
@task_item n=7 part=touches
Fixture rows listing a status line among the recorded fixture variant families (fixtures-per-cli-version, fixture-named-by-hook-event, fixtures-schema-checked, fixtures-match-tolerant-schema) and decision/fixtures-recorded-by-live-verify: the StatusLine family and its schema are gone.
@task_item n=8 part=touches
Relied ledger rows 23 to 21: rows naming statusline-override or statusline-shell as relied rows or verify probes (ledger-row-verify-probe, version-gate-needs-all-relied-rows, stamps-refuse-widened-ledger, ledger-probe-contract-suite, contract/viola-verify/stamped-line, verify-output step and summary lines) lose those ids.
@task_item n=9 part=touches
Capture arms two to one: any row naming the two --capture arms or hook statusline --capture now names one arm, hook <event> --capture (recorded-hook-payloads-walked-at-run-time, hook-payloads-recorded-by-verify, hook-payloads-recorded-locally).
@task_item n=10 part=touches
Hook event words: viola hook takes nine event words and no tenth; the word statusline is an unknown event, exit 0, nothing read or written (unknown-events-counted-skipped, hooks-fail-open, hook-fails-open, contract/viola-hook/*, contract/generated-plugin/hook-commands).
@task_item n=11 part=touches
Hook deadlines: the 5 s status-line deadline is gone; rows listing it beside the spine and dialog deadlines (hook-deadline-timing, performance-budget-tests-for-hook-deadlines, hook-deadline-driver) lose it.
@task_item n=12 part=touches
Bounded readers: the status-line shell-out output and source readers leave the list (bounded-readers, bounded-readers-keep-the-cap) and the frame cap's consumer list.
@task_item n=13 part=touches
Instance snapshot: statusline_command leaves the snapshot fields (contract/instance-snapshot/*, contract/snapshot/run-fields, snapshot-replay-fields, snapshot-read-through-replay where hook statusline was a replay reader: readers 6 to 5).
@task_item n=14 part=touches
Diagnostics schema: subject statusline-shell, hook_event statusline and process-exit's shell_exit_status removed (contract/log-event-corr/process-start, contract/log-event-corr/process-exit, diagnostics-schema-conformance, diag-line-schema-requires-corr); deadline and deadline_hit stay.
@task_item n=15 part=touches
decision/retired-words-as-fixtures: the retired-words fixture gains five retired_* keys (snapshot key, ledger rows, hook word, source file, diag subject); the 0.1.0 snapshot fixture holds the old statusline_command.
@task_item n=16 part=touches
Child launch: the claude child gets --plugin-dir <dir> then the caller's args, no --settings (claude-child-holds-nothing-of-viola, contract/viola-run-start/child-after-events, contract/process-start/claude-child).
@task_item n=17 part=touches
decision/ci-stamped-home-kill-line: the CI kill-line override filter no longer names the hook_statusline binary; the test target hook_statusline is gone.
@task_item n=18 part=touches
OS-gated and bounded-reader registers lose the rows naming the removed statusline tests (os-gated-tests-registered, os-gated-tests-each-named).
@task_item n=19 part=touches
TUI and drift: the TUI statusline passthrough case and the fake-agent drift statusline cases are removed (tui-passthrough-never-parses-screen, tui-outer-pty-driver, fake-agent-matches-recordings, fixture-drift-byte-compare, fake-transcripts-compared-to-recordings).
@task_item n=20 part=touches
hook_stdin fuzz seeds 12 to 10: rows naming the statusline seeds (fuzz-corpora-byte-exact, fuzz-reproducers-committed, hook-stdin-test-layers, hook-e2e-recorded-stdin) lose them; no row states the count.
@task_item n=21 part=touches
The local-live suite and harness expected lines read 21 rows: [21/21] local-command-compact and stamped 2.1.287  21 pass  0 fail (contract/local-live-suite/pass, contract/run-suite/local-live).
@task_fact n=1 slug=1-test-scope-summary
fact: Absence: the status-line interception, no settings override, no status-line source read, no status-line hook arm or shell-out, no status-line fixtures; a wrapped session's status line is exactly as unwrapped.
@task_fact n=2 slug=test-scope-summary
fact: viola runs no shell at all: no product path spawns /bin/sh.
@task_fact n=3 slug=integration-test-strategy
fact: The retired hook word statusline is an unknown hook event: exit 0, nothing read or written, the home's tree byte-equal, read over a home whose wrapper booted and then stopped.
@task_fact n=4 slug=6-e2e-test-strategy
fact: A start hands its child no --settings and writes no settings.json in the instance dir; the user settings are left byte- and mtime-equal and no default home is made under --home.
@task_fact n=5 slug=unit-test-strategy
fact: Compat: a snapshot holding the retired statusline_command reads whole skipping it, a written snapshot holds no retired key, and a stamp holding the two retired row ids reads on the relied rows alone; the envelope v stays 1.
@task_fact n=6 slug=7-test-data-fixtures
fact: The fake agent accepts --settings <file>, consuming and ignoring its value.
@task_item n=1 part=untouched
MCP tool contracts: tools/list, invalid-params fields, reply screen and tool result bodies.
@task_item n=2 part=untouched
Send, wait, answer, release and last contracts and the send-confirmation decisions.
@task_item n=3 part=untouched
Revive refusal codes, start exit causes and the exit-21 instance-unreachable details.
@task_item n=4 part=untouched
Channel link and unlink methods, JSON-RPC error codes and wrapper channel frames.
@task_item n=5 part=untouched
Coverage floors and thresholds, mutation gate rows and the mutants verdict contracts.
@task_item n=6 part=untouched
CI supply-chain, cargo-deny bans, zizmor, toolchain pins, artifacts and secret scan: no dependency or workflow changed.
@task_item n=7 part=untouched
Liveness, heartbeat staleness and the list board columns and JSON.
@task_item n=8 part=untouched
Dialog decision bodies, hook-decision records and the impostor-server and peer-check rows.
@task_item n=9 part=untouched
Rows where status line means another thing, as nextest padding a status line's duration.
@task_item n=10 part=untouched
Pre-push and gate exit codes, breach codes and their detail words.
