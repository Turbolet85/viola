@task_prose part=session
The wrapper's paste of a driver's send into the child's PTY now ends within a compiled bound, PASTE_BOUND 5 s, set by no config key, env var or flag.
The PTY writer moved to one dedicated writer thread, started under catch_unwind like the two copy threads, fed in arrival order by a bounded queue of 8 jobs; the shared input-writer lock and its one-write paste are retired.
A paste that cannot finish in time is cut: the bracket is closed with no Enter, and the send is refused not-delivered with the new closed detail word paste-cut, carrying the send's cursor after its send-issued.
The cut leaves an in-memory cut mark: later sends are refused paste-cut with no cursor until a human key moves the wheel's new key count or a prompt-submitted, session-start or session-end record clears it.
A human key typed during a paste waits behind at most one 4096-byte chunk and the closing marker, never refused or dropped; the send's longest delivered total moved from 48.5 s to 53.5 s.
No capability is claimed; no schema, config key, env var, IPC method, span or dependency was added.
@task_file n=1 path=crates/viola-pty/src/pump.rs
what: the PTY seam's pump: the copy threads, the paste writer thread, the bounded paste and its cut, the run PTY threads under catch_unwind, the pty.paste_write span
@task_file n=2 path=crates/viola-pty/src/lib.rs
what: the PTY seam crate's API and its typed error with fixed Display messages, the paste handle, the new PasteCut error
@task_file n=3 path=crates/viola-agent-claude/src/screen.rs
what: the claude agent's compiled screen and time constants, now holding the paste bound
@task_file n=4 path=crates/viola-core/src/lib.rs
what: viola-core's closed refusal reasons and their detail words, the not-delivered detail set
@task_file n=5 path=crates/viola-mcp/src/tools.rs
what: the viola-mcp frame tools and the test holding the MCP call deadline between the wrapper's bounds and the client read
@task_file n=6 path=src/human/hints.rs
what: the human-facing refusal hint lines shown on a refused send
@task_file n=7 path=src/human/mirror.rs
what: the terminal mirror's struck line for a refused send
@task_file n=8 path=src/run/send.rs
what: the wrapper's send handling in viola run: the send refusal order, the send-issued and send-refused records, the readiness gate
@task_file n=9 path=src/run/send/slot.rs
what: the wrapper's send slot and in-flight send state, now holding the cut mark and its clears on hook events
@task_file n=10 path=src/run/wheel.rs
what: the wheel: human typing, the typed mark, pause and release, now counting human key reads
@task_file n=11 path=src/cmd/run.rs
what: the viola run verb: the run's PTY pump wiring and the attached paste
@task_file n=12 path=src/cmd/verify/typed/run.rs
what: viola verify's typed run harness that pastes text into the child
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: the fake agent binary used in CI in place of claude, its argv options
@task_file n=14 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's stdin reading, now able to stop reading
@task_file n=15 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's argv option parse tests
@task_file n=16 path=tests/cli_send.rs
what: the cross-process integration tests of viola send against the fake agent
@task_item n=1 part=touches
decision/run-threads-inside-catch-unwind: the run's PTY threads under catch_unwind went from two copy threads to three, the new paste writer thread added.
@task_item n=2 part=touches
decision/run-worker-panic-ends-run: a panic of the new writer thread is a worker panic that ends the session with the child killed; the row's thread count moved.
@task_item n=3 part=touches
contract/send-refused-detail/refusal-reason-words: the RefusalReason detail words gain paste-cut under not-delivered, appended last.
@task_item n=4 part=touches
contract/pty-seam-log/paste: the PTY seam's paste event now ends either written whole or cut at its bound; the pty.paste_write span kept with skip_all, text_bytes and paste_mode only.
@task_item n=5 part=touches
contract/send-path-log-events/event-order and contract/send-line/wrapper-side: a cut send writes send-issued then send-refused paste-cut with its cursor and no send-confirmed; a send refused on the cut mark writes send-refused with no cursor and no send-issued.
@task_item n=6 part=touches
contract/corr-by-event/send and contract/diagnostics-line/corr-on-send-lines: the cut's send-refused carries the send's cursor corr; the mark's refusal carries none.
@task_item n=7 part=touches
contract/send-log-line/detail and contract/diag-line-fields/send-events: send-refused detail may now hold paste-cut; the schema field stays an open string, no schema edit.
@task_item n=8 part=touches
decision/send-refused-by-each-deciding-side: paste-cut is decided and logged on the wrapper side only.
@task_item n=9 part=touches
decision/wheel-typed-mark-uninstrumented: the wheel now counts human key reads, in memory, writing no line and no span; a hook's human_input move counts nothing.
@task_item n=10 part=touches
decision/typed-keys-never-logged, decision/no-typed-keys-in-telemetry, decision/pty-byte-stream-never-logged, decision/send-text-never-logged: the queued human keys and the paste chunks pass through the writer thread and are never logged; only the key count exists, unlogged.
@task_item n=11 part=touches
decision/span-carried-into-tasks-and-threads: the new writer thread carries no span.
@task_item n=12 part=touches
decision/obs-deps-tokio-free and decision/sync-instrumentation-no-tokio: the writer thread and its queue use std mpsc, thread and atomics only, no Tokio.
@task_item n=13 part=touches
decision/typed-errors-fixed-display and decision/error-messages-interpolate-nothing: a new PTY error PasteCut with a fixed Display and no source.
@task_item n=14 part=touches
decision/detail-codes-only, decision/closed-protocol-codes and decision/event-values-closed: a new closed detail code, paste-cut, path-free.
@task_item n=15 part=touches
contract/send-refused/human-typing and contract/send-refused-detail/turn-running: the send refusal order gains two paste-cut rungs, the mark after the second human-typing and turn-running reads, the cut after the paste; a send during a blocked paste waits up to the paste bound then reads turn-running.
@task_item n=16 part=touches
contract/send-refused-detail/input-not-ready: any paste failure other than a cut stays input-not-ready with its cursor.
@task_item n=17 part=touches
contract/cli-exit/20-wrapper-fault and contract/cli-stderr/refusal-pair: a send refused paste-cut exits 13 by reason with a new hint line; a client built before the word shows paste-cut as the detail with no hint line.
@task_item n=18 part=touches
decision/no-retry-policies and decision/no-retry-once: a cut paste is not retried; later sends are refused until a human acts.
@task_item n=19 part=touches
decision/ci-verify-fake-agent-only: the fake agent gains an argv-only stop-reading option to drive the cut in CI; no env seam.
@task_fact n=1 slug=span-trace-coverage
fact: The cut mark left by a cut paste is in memory only, opens no span and writes no line of its own; its set shows as the cut send's send-refused paste-cut record and its clears as their own prompt-submitted, session-start and session-end records.
@task_fact n=2 slug=span-trace-coverage
fact: Wrapper send-refused detail paste-cut: a paste cut at its bound refuses not-delivered paste-cut after send-issued, with the send's cursor; a send refused on a standing cut mark refuses paste-cut with no cursor and nothing issued.
@task_item n=1 part=untouched
The channel, dialog and hook log paths, their corr and their detail codes.
@task_item n=2 part=untouched
Heartbeat, liveness, state recovery and snapshot replay rows.
@task_item n=3 part=untouched
Log file locations, owner-only modes, service identity and the subscriber setup.
@task_item n=4 part=untouched
PII scrubbing, secret scan, artifact upload and the CI gate rows.
@task_item n=5 part=untouched
Metric coverage, perf budgets and hook latency budgets.
@task_item n=6 part=untouched
Revive, link and unlink, plugin install and the run start preflight detail codes.
@task_item n=7 part=untouched
The absences: no exporter, no error reporter, no network egress, no database.
