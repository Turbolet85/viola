@task_prose part=session
The session measured whether real claude on Linux loses a key typed right after a resize of its own pty, and found no loss: 2.1.330 and 2.1.328 each kept 12 of 12 keys typed 0, 20 or 100 ms after a resize, the CLI read unwrapped.
It added a KR arm to the test-only live rig (labels KR, KR-2, KR8, KR8-2) that resizes its own pty, types up to 13 x and 13 DEL, and reads its box row's cell attributes, never text.
It mended the rig's settle for KR alone: quiet counted from a byte drained after the key, a late redraw told apart from a dirty box, under a rehearse-only quiet window of 100 to 400 ms and a rehearse-only KR seam that a live start refuses with exit 2.
The KR summary and ledger hold closed words, counts, delays and settle times only; the kept frames stay owner-only under the gitignored rig output directory and are never read into evidence.
No product crate, no viola-pty file, no fake-agent file, no log line, no event, no detail code, no config key and no env var changed; the fix half was not needed.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig's arms, session pump, settle and summary tally — the standing live rig that drives a real or fake claude in its own pty, outside every product process role
@task_file n=2 path=crates/viola-e2e/src/rig/keys.rs
what: the test-only live rig's key grant — which keys and actions a start may type into its own pty
@task_file n=3 path=crates/viola-e2e/src/rig/screen.rs
what: the test-only live rig's screen model of the child's pty, read by cell attributes for the box row
@task_file n=4 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the test-only live rig binary — its live, rehearse and census verbs and their usage refusals
@task_file n=5 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig rehearsals — integration tests that run the live rig over the fake agent
@task_item n=1 part=touches
Typed keys never logged and never in telemetry: the KR arm types x and DEL into its own pty; its summary and ledger record only key counts (keys x 13, del 13) and closed words, never a key stream, in the test-only rig, not in any product log.
@task_item n=2 part=touches
No telemetry from screen content: the KR arm reads its box row's cell attributes after a resize and writes closed words (clean, late-redraw, dirty) and counts, never screen text, into the rig's summary, which is no product telemetry.
@task_item n=3 part=touches
PTY byte stream never logged: the kept KR frames are written owner-only under the gitignored rig output directory and never copied into evidence, a message or the tree; the rig's settle reads the pty byte stream only to time quiet.
@task_item n=4 part=touches
Real-CLI verify never in CI and CI verify against the fake agent only: the live KR start is refused under CI; CI runs only the five KR rehearsals over the fake agent.
@task_item n=5 part=touches
Local run outputs never uploaded and the secret-scan scope: the rig's out dirs and kept frames are local, gitignored outputs that no CI upload carries.
@task_item n=6 part=touches
No sleep-based log waits: the rig's KR settle waits on a quiet window of pty bytes counted from a mark after the key (100 to 400 ms, default 400), not on any log line; check that no row reads this as a log wait.
@task_item n=7 part=touches
No retry policies and no retry-once: the spare start KR8-2 after the cut KR8 is one operator-granted start under a cap, not a retry policy in product code; check that no row reads it as one.
@task_item n=8 part=touches
Env names only, never values: the session printed no env value and added no env var; the rig's options are rehearse-only flags.
@task_item n=9 part=touches
Fixtures not a telemetry source: the rig's ledger lines and summaries are test evidence of closed words, counts and settle times, not a source of product telemetry.
@task_item n=1 part=untouched
Every process-log, detail-file, event-stream and diag-line contract (fields, event enum, detail codes, exit codes, corr and conn): no product process writes a new or changed line.
@task_item n=2 part=untouched
Span and trace coverage, span attributes and span naming: no span was declared, renamed or removed.
@task_item n=3 part=untouched
The PTY seam log events, the paste log event and the paste write's time bound: no PTY seam or paste code changed; the paste bound is a route entry for a later chunk.
@task_item n=4 part=untouched
Metric coverage, SLO invariants, hook and perf budgets, heartbeat thresholds and the panic and schema CI gates: no budget, threshold or gate moved.
@task_item n=5 part=untouched
Error capture and reporting, panic capture by role, redaction, veil and the allow-list: no product error path or redaction changed.
@task_item n=6 part=untouched
MCP, channel, hook, send, wait, dialog and wheel telemetry: none of those paths changed.
@task_item n=7 part=untouched
Service identity, config.json keys, diagnostics file locations and modes, and the logging stack: unchanged.
