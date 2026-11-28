@task_prose part=chunk
The viola console gains actions over the fleet view: a selection of one row of the board, and verbs reached from it by keyboard alone.
Each action runs through the CLI verb it names (open a child, focus, send, answer, last, pause, release, revive, and switch's console leg), with the same outcome and refusal as at a shell and no second code path.
A send shows its readback as the frame verb's own reply; a refused action shows as the verb's unable line on the console's own surface.
A key decoder of key sequences replaces the scaffold's one-class-per-read decoder, and every mode the actions add is left by keyboard alone (no key trap).
Which verbs, keys, selection model, send text entry, confirm and scroll are operator forks; capability clauses needing a live reading are claimed gated.
@task_item n=1 part=needs
The console role as a process: its own diagnostics file, its console-key and console-frame diagnostic line fields, its stdout carrying the frame only, and its panic outcome by role, now with actions added.
@task_item n=2 part=needs
Typed keys never logged and never in telemetry: the new key-sequence decoder and the send's text entry read keystrokes the console's own lines must not carry; a console-key line carries a class, never the bytes.
@task_item n=3 part=needs
Sent text never logged and confined to instance diagnostics: a send from the console carries the human's text through the send verb; no console line, span field or home-level file may hold it.
@task_item n=4 part=needs
The send readback: the send frame verb's reply (ok or a refusal code) shown on the console, with the send path's log events, send-issued / send-confirmed / send-refused lines and their corr, and the client-side send validation refusals (empty-text, control-character) and wrapper send-refused detail words as the readback's possible values.
@task_item n=5 part=needs
A refusal shown as the verb's unable line: the CLI's unable and hint pair, its fixed stderr refusal and fault strings, the exit 1 / exit 21 detail and cause words, and the internal-error fixed message, now rendered on the console's surface rather than a shell's stderr.
@task_item n=6 part=needs
Control characters escaped in terminal rendering: a verb's reply or refusal line drawn on the console is escaped before it reaches the terminal, and peer replies are screened to code words.
@task_item n=7 part=needs
Actions through the CLI verbs: each verb the console reaches keeps its own CLI process-log lines (process-start, process-exit with detail and exit code) as at a shell, whether called in-process or run as a child; a child run is logged by the child and shell spawn log events with its subject.
@task_item n=8 part=needs
The switch verb's console leg: viola switch stays CLI-only and the console reaches it through that verb, with the CLI process-log file entry for viola switch.
@task_item n=9 part=needs
Release from the console: release is a human verb, and a release from a driver is refused with release-from-driver; the console, driven by the human, must not trip that refusal, and pause and release keep the wheel path log events.
@task_item n=10 part=needs
Answer from the console: dialog text is never logged on an answer, and the answer verb's driver-side refusals (not-delivered, unverified-cli) are what the console shows.
@task_item n=11 part=needs
The focus, open, last and revive verbs reached from the console: their CLI process-log entries, the focus exit-1 detail, the wait and last connect refusals and the revive start logging as run, as each verb owns them.
@task_item n=12 part=needs
A selection names its row by the board's item, never by the drawn text; no telemetry is read from screen content, and the console's test driver reads state, never the screen.
@task_item n=13 part=needs
STATUS stays from the agents thread (at most every 5 s, unknown after a failed read): agents probe output untrusted, parse failure reads unknown, and the console's agents-probe log volume as accepted; a send's readback never comes from STATUS.
@task_item n=14 part=needs
The terminal restore check: line mode raw while the frame stands and cooked after the quit, read on the console's own terminal, kept true with the new decoder and modes.
@task_item n=15 part=needs
Panics in the console's new code: caught at the role's catch site, one single-line panic record, exit decided at the catch site, vt100 feed panics contained.
@task_item n=16 part=needs
New console tests run against the fake agent only, keep the test-scoped PATH and never read host sessions or the real claude, ~/.claude or ~/.viola; no live start and no window.
@task_item n=17 part=needs
Logging from the new console code goes through the obs_event macro only: no raw tracing macros, no print macros in product paths, no bare instrument, closed event values, corr passed explicitly.
@task_standing n=1
convention: Every emitted diagnostics line a new test drives is schema-checked against the diagnostic line schema in integration and E2E tests.
home: ci-integration
@task_standing n=2
convention: Every new integration-test home lives under the one scanned test-home root and is kept until the gate steps run.
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: Every new test waits on log lines without sleeps, and the zero-panics gate holds over its homes.
home: 10-slo-invariants-telemetry-budgets
@task_standing n=4
convention: Every new log line or field is additive to the log schema, follows additive field naming, and carries no content in home-level lines.
home: 6-log-coverage · log-coverage
@task_item n=1 part=not_needed
The viola mcp stdio server, its tools, frame limits and MCP refusal results: the console reaches verbs through the CLI, never through MCP.
@task_item n=2 part=not_needed
The viola hook role: its stdout decision body, fail-open, latency budgets and hook perf gates; the console adds no hook behaviour.
@task_item n=3 part=not_needed
viola verify and plugin install: their stdout step and summary lines, spawn subjects, stamps and ledger rows are no verb the console offers.
@task_item n=4 part=not_needed
Any future error reporter, OTel SDK or exporter, OTLP receiver, network egress and Sentry rows: the chunk adds no telemetry transport.
@task_item n=5 part=not_needed
The wrapper's run start internals: pin copy, version gate, collision check, window line and run start span attributes; the console only calls verbs and does not change how a start runs.
@task_item n=6 part=not_needed
State recovery and snapshot replay: torn-line healing, state-recovered lines and replayed snapshots are untouched by a selection or an action.
@task_item n=7 part=not_needed
CI artifact upload rules, secret-scan hit report fields, mutation output and fuzz artifact rules: the chunk adds no CI job or upload.
@task_item n=8 part=not_needed
Heartbeat file and liveness staleness thresholds: the view's liveness reading is unchanged by this chunk.
