@task_prose part=session
The fleet console now acts: every verb by keyboard on a selected row (last, pause, release, focus, revive, send, switch, open, answer), each run as a child of viola's own executable with --home, no shell, VIOLA_NAME and VIOLA_DIR removed, one child at a time, its output read bounded at MAX_FRAME a stream and escaped line by line.
A new closed event console-action (fields verb, a closed enum of nine CLI verb words, and exit_code, both required) is written once per finished child; the closed event list grew from 18 to 19 in the event enum, the log format list and both schemas.
The console-key line grew: its key word set went from 3 to 14 words (question added), it gained a count field required when key is text, and on the new editing line one read's keys fold to at most one line a class (text, backspace, unbound) with a count; no typed byte, text, name, argument, dialog text or child output reaches any line.
New console modes: a selection with scroll, a y/n confirm, one editing line (send, switch, open child) and an answer prompt for a pending permission or plan; a question draws one line and runs nothing; every mode is left by one key and Ctrl-C ends each.
The board item gained a dialog_id kept out of serialisation, so list, list --json and the MCP list item are unchanged; unicode-width was added as a root dependency, cargo deny green.
Measured: at 80 columns the key line is cut after m swi, so a answer and o open do not show; a 200-byte paste reached the console in at most four reads; matrix v2-15 stays GATED on the operator live reading.
@task_file n=1 path=src/console/action.rs
what: the fleet console's action runner, new: the verbs it runs as children of its own executable, their argv and stdin, the stripped instance environment and the bounded, escaped child output
@task_file n=2 path=src/console/keys.rs
what: the console's raw-stdin key reader: the decoder, its escape wait, its per-read cap and its closed key classes
@task_file n=3 path=src/console/mod.rs
what: the fleet console's loop and state: its modes (list, confirm, editing line, answer prompt), its key handling and the console-key and console-action diagnostic lines it writes
@task_file n=4 path=src/console/view.rs
what: the console frame's view: the row selection, the scroll window with its above and more lines, and the fitted key line
@task_file n=5 path=src/human/console.rs
what: the console's fixed human copy: the key line, the confirm, editing and answer prompts, the question line, the running line and the editing line's width rule
@task_file n=6 path=src/human/fleet.rs
what: the fleet strip's rows: each wrapped row's name, selection gutter and pending dialog kind and id
@task_file n=7 path=src/human.rs
what: the human-output module's re-exports
@task_file n=8 path=src/cmd/console.rs
what: the viola console verb's entry: it builds the action runner over its own executable and home
@task_file n=9 path=src/cmd/list.rs
what: the viola list verb (a board item literal gains the new field)
@task_file n=10 path=src/human/board/test_support.rs
what: the board rendering's test support (a board item literal)
@task_file n=11 path=crates/viola-core/src/obs.rs
what: the closed obs event enum and the log format event list, with the event doc of its fields
@task_file n=12 path=crates/viola-state/src/board.rs
what: the board read model: its item per instance, read from the snapshot or by replay
@task_file n=13 path=schemas/diag-line.v1.json
what: the diag-line v1 schema: the event enum and each event's message and field arms
@task_file n=14 path=schemas/diag-detail.v1.json
what: the detail-line schema: its event enum
@task_file n=15 path=tests/contract_diag_schema.rs
what: the diagnostic schema contract test: the closed event list matched against both schemas
@task_file n=16 path=tests/tui_console.rs
what: the console's end-to-end tests under an outer PTY over the fake agent
@task_file n=17 path=Cargo.toml
what: the workspace and root binary dependency lists
@task_file n=18 path=Cargo.lock
what: the dependency lock
@task_item n=1 part=touches
contract/diag-line-fields/console-key is stale: its key words are no longer quit, interrupt or unbound but a closed set of 14 (question added), the field arm's keys are now count and key, count required when key is text, and the editing line folds a read to at most one line a class.
@task_item n=2 part=touches
decision/no-interactive-terminal-interface: the console now takes a selection, a confirm, an answer prompt and one editing line of typed text.
@task_item n=3 part=touches
The closed event list grows from 18 to 19 with console-action: rows on the closed event enum, its match with the schemas and the adopted tests event list (decision/event-values-closed, decision/closed-event-and-corr-on-every-line, decision/diag-schema-matches-event-enum, decision/tests-event-list-adopted).
@task_item n=4 part=touches
Additive-only schema rules (decision/diag-schema-additive-only, decision/log-schema-additive-only, decision/log-fields-additive-only, decision/additive-field-naming): a new event, a new count field and new key words are added; nothing removed or renamed.
@task_item n=5 part=touches
Typed keys never logged (decision/typed-keys-never-logged, decision/no-typed-keys-in-telemetry): the editing line logs a count of keys a class per read, never a key byte or the typed text.
@task_item n=6 part=touches
Sent text never logged (decision/send-text-never-logged, decision/sent-text-never-logged, decision/no-text-in-telemetry, decision/send-text-confined): the console hands send text to the send verb on stdin and logs none of it.
@task_item n=7 part=touches
Dialog content never logged (decision/dialog-text-never-logged, decision/dialog-content-never-logged, decision/no-dialog-content-in-logs): the answer prompt reads only the board's kind word and dialog id; a question logs console-key question with no question or option text.
@task_item n=8 part=touches
contract/diagnostics-files/console: the console's diagnostics file now holds console-action lines and the console-key question and count lines.
@task_item n=9 part=touches
Control characters escaped in terminal rendering (decision/escape-controls-in-terminal, decision/controls-escaped-in-terminal-output): each child output line and the editing echo pass the same escape.
@task_item n=10 part=touches
contract/console-output/stdout: the console's stdout now draws the selection gutter, the confirm, answer and editing prompts, the question line and the action child's output lines.
@task_item n=11 part=touches
decision/no-telemetry-from-screen and decision/no-telemetry-from-screen-content: the child's output is drawn on the console, never logged.
@task_item n=12 part=touches
decision/exit-code-log-field: console-action carries the child's exit_code; a child that fails to spawn reads -1.
@task_item n=13 part=touches
Stripped environment and instance naming (decision/stripped-env-never-logged, decision/instance-never-from-working-directory, decision/cli-log-only-under-viola-name): the console's children run with VIOLA_NAME and VIOLA_DIR removed, the name a positional, and log in their own cli process log.
@task_item n=14 part=touches
decision/send-refused-by-each-deciding-side: a send from the console that the verb refuses (empty line exit 13) is logged by the child's own side; the console logs only its exit code.
@task_item n=15 part=touches
contract/child-spawn-log/subject and decision/uninstrumented-components: the console's action child gets no spawn line of the console's own, and the editing echo writes no line per character.
@task_item n=16 part=touches
decision/no-hot-path-instrumentation and decision/no-info-in-hot-loops: a paste on the editing line writes at most one line a class per read, not one a key.
@task_item n=17 part=touches
decision/metric-grouping-closed-enums and decision/derived-metrics-by-closed-enums: console-action's verb is a closed nine-word enum and console-key's word set a closed fourteen.
@task_item n=18 part=touches
decision/viola-core-emits-no-telemetry: viola-core's event enum gains the console-action variant, with no emission of its own.
@task_item n=19 part=touches
decision/schema-top-level-unevaluated-properties and decision/detail-schema-self-contained: both schemas gain console-action in their event enum; the line schema gains its field arm and a conditional required count.
@task_item n=20 part=touches
decision/obs-deps-pass-cargo-deny: a dependency (unicode-width) was added to the root binary, no crate added to the lock, cargo deny green.
@task_item n=21 part=touches
contract/panic-outcome-by-role/console: the console now spawns verb children; a child's fault or panic is its own cli role's, read by the console only as an exit code.
@task_fact n=1 slug=6-log-coverage
fact: contract/diag-line-fields/console-action: one console-action line per finished action child, fields verb (closed enum of the nine CLI verb words the console runs) and exit_code (integer), both required; no name, argument or output in the line.
@task_fact n=2 slug=6-log-coverage
fact: On the console's editing line, one read's keys write at most one console-key line a class (text, backspace, unbound) with a count, the count required for text; no typed byte or text reaches any line.
@task_fact n=3 slug=span-trace-coverage
fact: Console actions are observed by the console's console-action line and the child verb's own cli process log; the console runs each verb as a child of its own executable, one at a time, with no span of its own over it.
@task_item n=1 part=untouched
The viola run start path, its spans, span attributes and start exit detail codes: no start path changed.
@task_item n=2 part=untouched
Hook lines, hook latency budgets and hook perf gates: no hook path changed.
@task_item n=3 part=untouched
The MCP server, its tools, its stdout and its refusal results.
@task_item n=4 part=untouched
Channel JSON-RPC lines, params and errors, and the send and wait wrapper-side lines.
@task_item n=5 part=untouched
Error capture and panic handling, and every future error reporter decision.
@task_item n=6 part=untouched
CI integration gates, the secret scan, artifact uploads and retention: no harness or gate surface changed.
@task_item n=7 part=untouched
OTel, exporter, network egress and receiver absences.
@task_item n=8 part=untouched
Heartbeat, liveness and staleness rows, and the list --json output, unchanged.
@task_item n=9 part=untouched
viola verify, its spawn subjects and its stdout.
