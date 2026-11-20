@task_prose part=session
viola run now verifies an unstamped claude version itself when the new config.json key verify_new_cli is true (default false), so no hand-run viola verify is needed.
The start spawns the pinned viola verify as a child after the channel bind, under a new owner-only attempt lock ledger/verify.lock, then takes the gate's verdict again; the trust record is now written on that final verdict, after the verify.
The verify child is logged as a process-start / process-exit pair with the new diag-line subject auto-verify (child_exit_status, duration_ms) on run.start's span, with no span of its own; the subject enum moves 7 to 8 values and the known config keys 4 to 5.
The start writes one of five fixed ASCII stderr lines (verifying, did not verify, earlier red, verifying elsewhere, no stamp) and none under revive --json; an Attempted unverified entry never verifies again, so a red verify does not loop.
A SIGINT or SIGTERM during that verify TERMs the child, waits 30 s then KILLs it, writes the exit line and re-raises the signal: the start ends with no stamp, no snapshot and no session.
Nothing retired; measured green at every layer, a live round with real claude 2.1.329 in a scratch home, and CI 7/7.
@task_file n=1 path=src/run/auto_verify.rs
what: the start's automatic verify step: reads the verify_new_cli config key, decides whether to verify, spawns the verify child, waits, stops it on a signal
@task_file n=2 path=src/human/auto_verify.rs
what: the start's five fixed stderr lines for the automatic verify, one ASCII write each with no SGR
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger reader: the relied rows and the stamps verdict, plus a new attempted reading of a version's entry
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit tests of the capability ledger's stamps reading
@task_file n=5 path=crates/viola-state/src/stamps.rs
what: the stamps file store and its writer lock, plus the new owner-only attempt lock ledger/verify.lock, taken after the strict-modes check
@task_file n=6 path=src/obs.rs
what: the obs module: the diagnostics line writer, the config.json reader and its known config keys list
@task_file n=7 path=schemas/config.v1.json
what: the config.json schema, its known keys
@task_file n=8 path=schemas/diag-line.v1.json
what: the diagnostic line schema, its closed subject enum of spawn subjects
@task_file n=9 path=src/run/version_gate.rs
what: the start's version gate: the version probe, the strict stamps read and the cli_verified verdict, run.version_gate span
@task_file n=10 path=src/run/mod.rs
what: the viola run module list
@task_file n=11 path=src/cmd/run.rs
what: the viola run and revive start order: collision check, pin copy, version gate, bind, then the automatic verify, then the trust record
@task_file n=12 path=src/human.rs
what: the human-facing stderr text module list
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent that stands in for claude, now reading extra arguments from a file beside it
@task_file n=14 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: unit tests of the fake agent's options
@task_file n=15 path=tests/support/fake.rs
what: integration test support that writes the fake agent's files
@task_file n=16 path=tests/support/home.rs
what: integration test support: test homes and a booted wrapper, now a starting wrapper read before ready
@task_file n=17 path=tests/support/outer_pty.rs
what: integration test support: the outer PTY that drives a start, now giving its child pid
@task_file n=18 path=tests/cli_version_gate.rs
what: integration tests of the unverified-CLI gate, now the automatic verify cases
@task_file n=19 path=tests/cli_controls_not_disableable.rs
what: integration tests that no setting lifts a guard or a bound, now the verify_new_cli key
@task_file n=20 path=tests/contract_bounded_readers.rs
what: the bounded readers list check: every reader of an outside input capped at MAX_FRAME
@task_file n=21 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig that drives real claude locally, its arms and summary lines, now the AV arm
@task_file n=22 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's scratch home seeding, now the verify_new_cli key for the AV arm
@task_file n=23 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig's command entry and its labels
@task_file n=24 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig rehearsal tests against the fake agent
@task_item n=1 part=touches
contract/config-json/known-keys states config.json has four known keys: there are now five, verify_new_cli added (a JSON boolean, default false).
@task_item n=2 part=touches
The diag-line subject enum rows (contract/diag-subject/*) hold seven closed spawn subjects: it now holds eight, auto-verify added; contract/child-spawn-log/subject and contract/process-log/child-and-probe-lifecycle gain that child.
@task_item n=3 part=touches
decision/run-silent-terminal and decision/run-silent-on-terminal, and contract/run-terminal-output/child-output: viola run now writes one fixed stderr line of its own before the child when the key is on, and the verify child's own stdout and stderr reach the terminal before the claude child (null under revive --json).
@task_item n=4 part=touches
decision/verify-sole-stamps-writer and the stamps-writer rows: verify stays the only stamps writer, but a start now starts it automatically.
@task_item n=5 part=touches
contract/run-start-events/order, contract/run-start-spans/span-chain and the run.start span rows: the start order gains a step after channel.bind, and the trust step moves after it (it came before the bind); the step opens no span, its lines sit on run.start's span.
@task_item n=6 part=touches
contract/run-span-attributes/run-version-gate: run.version_gate and the version-probe pair are unchanged, but the strict stamps read runs again under the attempt lock and after the verify, the gate's verdict then taken again.
@task_item n=7 part=touches
contract/run-log/cli-verified and contract/run-process-log rows: cli_verified now records the final verdict, after the automatic verify, not the first gate read.
@task_item n=8 part=touches
contract/start-trust-log/trust-record and decision/trust-record-write-uninstrumented: the trust record is written on the final verdict, after the bind and the automatic verify.
@task_item n=9 part=touches
contract/run-log-parse-rejected/ledger-stamps, decision/unreadable-ledger-is-unverified and decision/unreadable-ledger-means-unverified: the strict stamps read keeps its one parse-rejected ledger-stamps line; malformed stamps still read unverified, and the new attempted reading refuses them the same way.
@task_item n=10 part=touches
contract/diagnostics-events/config-parse-rejected, contract/parse-rejected-detail/malformed and contract/config-json/v: a verify_new_cli value that is not a boolean reads false with one parse-rejected config-json malformed line; v must be 1, and a v 2 document gets only the diagnostics-level read's one line.
@task_item n=11 part=touches
decision/config-observed-at-load, decision/env-vars-not-config-channel and decision/env-vars-not-configuration: a config key now switches a behaviour on, read once at the start; no environment variable sets it.
@task_item n=12 part=touches
decision/real-cli-verify-never-in-ci and decision/ci-verify-fake-agent-only: the key is off by default in every CI home; the automatic verify is tested only against the fake agent, and real claude runs only in the local live rig.
@task_item n=13 part=touches
contract/verify-process-exit/child-exit-status-and-duration and the verify-spawn-subject rows: the start's verify pair carries child_exit_status and duration_ms under the subject auto-verify, beside verify's own probe subjects.
@task_item n=14 part=touches
contract/viola-verify-refusal/stamp-write-refused and contract/viola-verify-refusal/home-tree-refused: same words, a new caller (the start); a start whose verify writes no stamp shows the no-stamp line with unstamped.
@task_item n=15 part=touches
decision/verify-logs-only-with-instance and decision/verify-lines-carry-no-content: the start's verify child runs with VIOLA_NAME, VIOLA_DIR and VIOLA_BIN removed, cwd the start's own, stdin null.
@task_item n=16 part=touches
decision/claude-env-strip, decision/stripped-env-never-logged and decision/stripped-claude-values-never-logged: the start's CLAUDE strip plan is applied to the verify child too.
@task_item n=17 part=touches
contract/verify-process-exit/interrupted, contract/process-exit-detail/interrupted and contract/verify-stop-stderr/unable-hint-pair: a SIGINT or SIGTERM during the start's verify TERMs the verify child (verify's stop pair shows), waits 30 s then KILLs it, then the start re-raises the signal and writes no process-exit of its own.
@task_item n=18 part=touches
decision/home-tree-check-before-start-log, decision/strict-modes-before-diagnostics and decision/home-tree-check-before-log-init: the start's home-tree check still runs first; the attempt lock runs the strict stamps-modes check before opening ledger/verify.lock, and refuses a widened ledger directory.
@task_item n=19 part=touches
decision/diagnostics-owner-only-permissions and decision/private-diagnostics-files: a new on-disk owner-only file, ledger/verify.lock (0600), sits beside the stamps; it writes no log line.
@task_item n=20 part=touches
decision/ledger-contents-never-logged: the five stderr lines and the auto-verify pair carry only the parsed version and a closed word, never the ledger path or the stamps.
@task_item n=21 part=touches
decision/stderr-fixed-strings and decision/fixed-stderr-messages: the five automatic-verify lines are fixed ASCII text with no SGR, written in one write each.
@task_item n=22 part=touches
contract/dialog-log-line/unverified-cli and contract/answer-refusal/unverified-cli: an automatic green stamp now lifts the unverified-CLI gate at start; the rule and its code are unchanged.
@task_item n=23 part=touches
decision/revive-start-logs-as-run and contract/cli-log-file/revive-start: revive's start reaches the automatic verify through the same start; under revive --json the stderr lines are silent and verify's output is null.
@task_item n=24 part=touches
decision/diag-schema-additive-only, decision/log-schema-additive-only and decision/g4-schema-conformance-gate: the diag-line schema gains an enum value additively, and the auto-verify lines must conform to it.
@task_item n=25 part=touches
contract/diag-line-fields/process-start and contract/diag-line-fields/process-exit: those lines now carry the subject auto-verify, with child_exit_status and duration_ms on the exit.
@task_item n=26 part=touches
decision/uninstrumented-components: the attempt lock and the five stderr lines are left uninstrumented by design; only the verify child's pair is logged.
@task_item n=27 part=touches
decision/no-obs-gate-on-verify-lines: the integration and rig checks now read the auto-verify pair from the run log.
@task_fact n=1 slug=log-coverage
fact: The diag-line subject enum holds auto-verify: the process-start and process-exit pair of the viola verify child that a viola run start spawns, the exit carrying child_exit_status and duration_ms, written to the run log.
@task_fact n=2 slug=otel-sdk-init
fact: config.json has the key verify_new_cli, a JSON boolean, default false; any other value reads false with one parse-rejected config-json malformed line, and the key is read only when v is 1.
@task_fact n=3 slug=3-observability-harness-contract
fact: When verify_new_cli is on, viola run writes one fixed ASCII stderr line per start (verifying, did not verify, earlier red, verifying elsewhere, or no stamp with spawn-failed, unstamped or lock-refused), one write with no SGR, carrying only the parsed version and a closed word, and none under revive --json: the named exception to run's silence on the terminal.
@task_fact n=4 slug=span-trace-coverage
fact: The start's automatic verify opens no span of its own: its auto-verify lines sit on run.start's span, after channel.bind and before the trust record.
@task_fact n=5 slug=span-trace-coverage
fact: A SIGINT or SIGTERM during the start's automatic verify TERMs the verify child, KILLs it after a 30 s grace, writes the auto-verify process-exit, then re-raises the signal: the start ends with no process-exit of its own, no stamp, no snapshot and no session.
@task_fact n=6 slug=span-trace-coverage
fact: The automatic verify's attempt lock ledger/verify.lock is an owner-only 0600 file taken after the strict stamps-modes check and separate from the stamps writer's lock; a held lock reads as verifying elsewhere and writes no line.
@task_fact n=7 slug=span-trace-coverage
fact: An entry already Attempted for this CLI version (every relied row has a word, whatever it reads) is never verified again automatically: the start shows the earlier-red line and runs unverified, so a red verify cannot loop.
@task_item n=1 part=untouched
The viola mcp server, its tools, its stdin frame bounds and its refusal results: the session does not reach the MCP role.
@task_item n=2 part=untouched
The channel JSON-RPC lines, send, wait, last, dialog and hook paths, and the wheel pause and release: none of these changed.
@task_item n=3 part=untouched
Panic capture, catch sites, panic lines and the detail-file error chain: no panic path changed.
@task_item n=4 part=untouched
PII scrubbing, the secret scan, artifact uploads and the redaction rows: no logged field beyond the closed subject and existing fields was added.
@task_item n=5 part=untouched
SLO and performance budgets, hook latency, the heartbeat file and liveness: not reached.
@task_item n=6 part=untouched
The common log line fields, timestamps, corr and conn propagation, and service identity: unchanged.
@task_item n=7 part=untouched
The absences (OTel, exporters, network egress, error reporter, database, hosted deployment): no capability was added there.
@task_item n=8 part=untouched
viola list, revive --list, link and unlink outputs and their refusal pairs, and the exit 1 and exit 21 detail codes: none changed.
