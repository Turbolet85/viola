@task_prose part=session
The session moved the input-box readiness predicate: a modal label on the screen now refuses a send only where it stands as a dialog, either option-shaped or with no input-box row below it.
A modal label quoted mid-sentence in a transcript row above a ready input box no longer refuses a send on an idle session; the send goes through as send-issued then send-confirmed.
A standing modal still refuses at once with send-refused detail input-not-ready; the quiet period, the gate's maximum wait, the poisoned arm and the unverified arm are unchanged.
The same predicate serves viola run's pre-send readiness gate, verify's paste guard, the ledger's input-box-ready post-condition and the rig's box read; the start's first-screen modal arms and verify's settle end still read raw presence of a modal literal.
No log line, span, field, event, detail word, exit code, hint, config key, schema, dependency or count was added, moved or retired; CI on the pushed head is green.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the compiled screen signatures of the claude agent crate: the modal, input-box and footer literals and the input-box readiness predicate read over the vt100 screen, with the new standing-modal reading
@task_file n=2 path=src/run/gate.rs
what: viola run's pre-send readiness gate (the run.readiness_gate step of a confirmed send) that refuses with input-not-ready, and its unit tests
@task_file n=3 path=tests/support/fake.rs
what: the integration tests' fake-agent support helpers, here a new synthetic idle screen quoting a modal label above the input box
@task_file n=4 path=tests/cli_send.rs
what: the viola send integration tests over the fake agent and a stamped home, here a send past a quoted modal label
@task_item n=1 part=touches
The wrapper send-refused detail input-not-ready: it is now written only for a standing modal (option-shaped, or no input-box row below it) or an input box not up; a modal label merely quoted in a transcript row above the box no longer yields it. The word and its hint are unchanged.
@task_item n=2 part=touches
The send path log events and their order: a send over a screen quoting a modal label now runs send-issued then send-confirmed with no send-refused, where it was refused before.
@task_item n=3 part=touches
The manual span on the readiness gate and the confirmed-send span attribute run.readiness_gate: the gate they cover changed its modal refusal; no span, attribute or name changed.
@task_item n=4 part=touches
Send refusal logged by each deciding side: the wrapper side's input-not-ready decision now reads the standing-modal predicate; the side that logs it is unchanged.
@task_item n=5 part=touches
Browser-tools prompt reading uninstrumented: the report cites it as the rule the new standing-modal reading falls under; the new reading adds no line or span.
@task_item n=6 part=touches
No telemetry from screen content (both rows): the predicate reads more of the screen (a row's shape and the rows below it) and returns only a boolean; no row text is kept, logged or returned.
@task_item n=7 part=touches
Wrapped claude internals are not a contract, and the recorded fixtures being no telemetry source: the modal literals stay compiled screen signatures; the proof runs over the recorded trust screens and synthetic screens named as synthetic, no fixture changed.
@task_item n=8 part=touches
verify lines carry no content (both rows) and verify logging only inside an instance: verify's paste guard calls the moved predicate; its lines and settle-end reading are unchanged.
@task_item n=9 part=touches
The start's trust-record log line and the trust-record write being uninstrumented: the start's first-screen read keeps raw presence of a modal literal by design, so its line and order do not move.
@task_item n=10 part=touches
vt100 panics contained and the vt100 feed panic contained: the predicate reads the vt100 screen; the feed-panic chaos check ran green, nothing in the containment moved.
@task_item n=11 part=touches
Components left uninstrumented: the screen-signature component gained the standing-modal reading with no instrumentation, consistent with its standing.
@task_fact n=1 slug=4-span-trace-coverage
fact: The standing-modal reading on the readiness gate is uninstrumented: a modal label quoted above a ready input box writes no line and lets the send proceed, and a standing modal (option-shaped, or with no input-box row below it) keeps send-refused detail input-not-ready.
@task_item n=1 part=untouched
The log line schema and its fields, field names, levels, corr and conn correlation, and every diagnostics-line contract: no field, event value or line was added or changed.
@task_item n=2 part=untouched
The detail-word and exit-code sets (exit 1, 20, 21 detail codes, unreachable cause words, parse-rejected details): no word was added, retired or renamed.
@task_item n=3 part=untouched
Panic capture and error reporting, the panic line, catch sites and the error-chain detail file.
@task_item n=4 part=untouched
PII scrubbing and redaction, the allow-list, veil, the secret scan and artifact uploads.
@task_item n=5 part=untouched
CI gates, SLO invariants and telemetry budgets: no threshold, budget, floor or gate moved; the CI run was green on every job.
@task_item n=6 part=untouched
The hook, MCP and channel roles, their streams, their refusals and their log files.
@task_item n=7 part=untouched
Heartbeat and liveness, state recovery and replay, the events log, log file locations, modes and rotation.
@task_item n=8 part=untouched
The logging stack, subscriber setup, init order, service identity, config.json keys and the telemetry dependencies.
@task_item n=9 part=untouched
The claude child's process-start env fields, the R8 strip, the capability ledger contents and the verify spawn subjects and exit codes.
