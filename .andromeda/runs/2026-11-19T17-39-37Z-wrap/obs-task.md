@task_prose part=session
The session recorded one contained live StopFailure payload on claude 2.1.328 (the start SF, 2026-11-19) and settled the StopFailure exception on it as outcome B, kept on the founder's word, no longer provisional; no ledger row and no verify probe are opened.
All change is test-only: the live rig (viola-e2e) gains an SF arm, its network-closed bwrap sandbox with a refusal before the ledger line, a StopFailure capture line and cell-attribute reads of the box row; the fake agent gains a turn-fail option and a settings-hooks option.
The rig's StopFailure capture keeps key names, the error word only from the CLI's closed thirteen (else other) and the byte lengths of two text fields, never a text; the box row is read for cell attributes only, never its text.
The live reading: turn-end stop-failure, cause server-error, captured word server_error, unknown-value lines 0; one connection-lost StopFailure on one version, nothing claimed for the other twelve words or another version.
The rig's kept record dir is keyed by CLI version first; the sandbox probe refuses no-runtime-dir unless XDG_RUNTIME_DIR is absolute and outside the temp root.
No product crate, product schema, config key, env var, log line, event, detail code or obs gate changed.
@task_file n=1 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolation of its child — the scratch home, the kept record dir (now keyed by CLI version, refusing a standing version), the bwrap sandbox argv, the runtime-dir check and the seeding of the StopFailure capture hook
@task_file n=2 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig — its arm set (gains SF, labels SF and SF-2), the sandbox probe, the SF arm's summary words (turn-end, cause, unknown-value-lines, captured-word, cause-matches, net-ns, hint-attr, typed-attr) and its count of unknown-value lines in the role file
@task_file n=3 path=crates/viola-e2e/src/rig/capture.rs
what: the rig's closed prompt capture — gains the StopFailure line holding key names, the error word of the CLI's thirteen else other, and the lengths of error_details and last_assistant_message, read within MAX_FRAME plus one, owner-only append
@task_file n=4 path=crates/viola-e2e/src/rig/screen.rs
what: the rig's screen model — gains reads of a cell's attributes and the box row's attributes as closed words, never a cell's text
@task_file n=5 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig's command line — rehearse sf with a contain option, live SF and SF-2, every live SF contained, refused sandbox exit 3 before any ledger line
@task_file n=6 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig rehearsals over the fake agent run in CI — gains four sf rehearsals (the stop-failure turn end and cause, an unknown error word read as api-error with no needle kept, the contained network namespace, the sandbox refusal)
@task_file n=7 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent — gains an argv option ending a turn with a recorded StopFailure variant and one running a Claude settings.json's hooks beside the plugin's, no shell
@task_item n=1 part=touches
The StopFailure exception: rows whose cell says StopFailure is registered with no recorded or live payload, or calls it provisional or owned by a later live payload, are stale — it is now settled as outcome B on the founder's word, citing the live reading (claude 2.1.328, 2026-11-19, the start SF, the word server_error).
@task_item n=2 part=touches
The unknown-value rows (hook-stdin parse-rejected with detail unknown-value, the unknown-value detail word, parse failures as unknown plus a counter): true as worded, but the rig now counts hook-stdin unknown-value lines in the role file and a planted unknown error word read live as cause api-error with one unknown-value line; a picker should check each still reads true beside that.
@task_item n=3 part=touches
last_assistant_message never logged: the rig's StopFailure capture keeps its byte length only, never its text — a test-only record beside the rule, not a log line; check the rows on the assistant message.
@task_item n=4 part=touches
Fixtures and test records are no telemetry source: the rig's StopFailure capture and its kept record dir per CLI version are test-only records of the same kind.
@task_item n=5 part=touches
No telemetry from screen content: the rig now reads the box row's cell attributes (never text) after the turn end — a test reading, not telemetry; the rule stands, a picker may note the test-only reading beside it.
@task_item n=6 part=touches
Real-CLI verify never in CI and CI against the fake agent only: the live SF arm runs only locally, contained; CI runs the unwrapped sf rehearsals over the fake agent; whether CI ran or skipped the contained rehearsal is unread.
@task_item n=7 part=touches
MAX_FRAME on stdin reads: the rig's StopFailure capture reads through MAX_FRAME plus one and reads an oversize or non-object payload as unread — the same bound, test-only.
@task_item n=8 part=touches
Content canary never in home-level files and needles: the unknown-error-word rehearsal asserts its needles and the canary absent from the rig's stdout, ledger and out dir.
@task_item n=9 part=touches
Network egress of any kind: the live SF start ran inside a network-closed bwrap sandbox (net-ns own), its request unable to leave; this is a test-only host dependency of the rig and claims no egress.
@task_item n=10 part=touches
Product readers read the snapshot through replay: the rig reads pid and child_pid under data in snapshot.json directly — a test reading, no product reader; the rule stands.
@task_item n=11 part=touches
Indirect, the instance-dead cause word and the socket directory: under an env without XDG_RUNTIME_DIR viola placed its socket dir under the temp root, which the sandbox hid, and send read instance-unreachable instance-dead; the rig now refuses no-runtime-dir instead — the product cause words are true as worded.
@task_fact n=1 slug=span-trace-coverage
fact: The live rig's StopFailure capture is a test-only record, never telemetry: it keeps the payload's key names, the error word only from the CLI's closed thirteen (any other value as other) and the byte lengths of error_details and last_assistant_message, never a text.
@task_item n=1 part=untouched
The process log line, the diag-line schema, the event enum, the detail codes, the corr and conn fields and the event stream: no product line, event or field changed.
@task_item n=2 part=untouched
The panic capture and error-reporting rows, and the absences of a reporter, an OTel SDK or an exporter: nothing added or removed.
@task_item n=3 part=untouched
The CI gates (bare instrument, zero panics, schema conformance, secret scan, artifact retention) and the hook perf budgets and their thresholds: no gate, budget or bound moved.
@task_item n=4 part=untouched
The MCP server, the channel, the wheel, dialog and send paths and their spans: no product path changed.
@task_item n=5 part=untouched
The logging stack, the subscriber, log levels, diagnostics file modes and home resolution: unchanged.
