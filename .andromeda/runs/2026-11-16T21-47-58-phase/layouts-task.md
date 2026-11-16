@task_prose part=chunk
The Claude adapter reads the Stop hook payload's session_crons as a count only, no element kept or typed, and carries it as one number beside background_tasks on the normalised turn-ended event.
That count reaches every hop background_tasks reaches today: the turn-ended event data in the events log, viola wait --json, the MCP wait tool, and any further hop P3 measures background_tasks taking (viola last, viola list).
A session with no scheduled wake reads none, mapped as background_tasks's absent, null or non-array shape is today; no wake's content reaches any output.
The wait wake rule is unchanged; P3 re-measures every hop of background_tasks and reads whether the count lets a scheduled wake's prompt be told from a typed one.
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the Claude adapter's Stop hook payload parsing and its normalisation into the turn-ended event — edits: the background_tasks count read, joined by a session_crons count
@task_file n=2 path=crates/viola-core/src/lib.rs mark=edits
what: the normalised agent event kinds shared across adapters — edits: the turn-ended event's data, a new normalised count field beside background_tasks
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the viola run wrapper loop that records turn-ended events to the events log — edits: the background_tasks hop
@task_file n=4 path=src/run/send/slot.rs mark=edits
what: the send slot that carries a turn end's data to a waiting driver — edits: the background_tasks hop
@task_file n=5 path=src/run/wait.rs mark=edits
what: the wait verb's turn-ended outcome and its --json document — edits: the background_tasks hop; the wake rule itself unchanged
@task_file n=6 path=tests/cli_wait_last.rs mark=edits
what: the CLI tests of viola wait and viola last output — edits: the background_tasks assertions
@task_file n=7 path=tests/cli_wheel.rs mark=edits
what: the CLI tests of the wheel and wait interplay — edits: the background_tasks assertions
@task_file n=8 path=tests/hook_events.rs mark=edits
what: the tests of hook payloads normalised to events — edits: the background_tasks assertions
@task_file n=9 path=crates/viola-mcp/tests/handlers.rs mark=edits
what: the tests of the viola mcp tool handlers and their results — edits: the background_tasks assertions
@task_file n=10 path=crates/viola-state/tests/state_events.rs mark=edits
what: the tests of the events log records and their readers — edits: the background_tasks assertions
@task_file n=11 path=fixtures/claude/2.1.283/Stop.default.json mark=re-reads
what: a recorded Stop hook payload, read for the session_crons field's shape
@task_file n=12 path=fixtures/claude/2.1.287/Stop.default.json mark=re-reads
what: a recorded Stop hook payload, read for the session_crons field's shape
@task_file n=13 path=fixtures/claude/2.1.288/Stop.default.json mark=re-reads
what: a recorded Stop hook payload, read for the session_crons field's shape
@task_file n=14 path=fixtures/claude/2.1.302/Stop.default.json mark=re-reads
what: a recorded Stop hook payload, read for the session_crons field's shape
@task_item n=1 part=needs
The viola wait <target> verb: its turn-ended outcome gains the scheduled-wake count beside background_tasks; when it wakes, holds or completes does not change.
@task_item n=2 part=needs
The viola wait --json document for a turn end: one added number field, a session with no scheduled wake reading none as background_tasks does today.
@task_item n=3 part=needs
The viola mcp wait tool and its success result: the turn-ended data carries the added count as background_tasks is carried.
@task_item n=4 part=needs
The viola last verb and the viola mcp last tool, where P3 measures background_tasks reaching them: the count follows the same hop.
@task_item n=5 part=needs
The viola list --json document and the viola mcp list tool, where P3 measures background_tasks reaching them: the count follows the same hop; the human board gains no column.
@task_item n=6 part=needs
The viola wait human output for turn-ended: human output is a contract, so whether the turn-ended line shows the count is a contract decision, held to the existing line's form and vocabulary if it does.
@task_item n=7 part=needs
turn-ended is not a send outcome: the count rides the turn-ended event and wait's turn-ended outcome only, never a send result or a readback word.
@task_item n=8 part=needs
--json on every data verb and one document per --json run: the count is a field inside the existing document, never a second document or a stderr line.
@task_item n=9 part=needs
No wake content in any output: no prompt, cron expression, id, schedule or string of a scheduled wake reaches a --json document, an MCP tool result, a human line, a hint or a log line.
@task_item n=10 part=needs
The field's normalised name is set under the shared core's naming rule; no Claude-specific name appears in a CLI --json document or an MCP tool result.
@task_item n=11 part=needs
Readers of viola's own formats skip unknown fields: an older reader given the added field in a wait --json document, an MCP result or the events log still reads it.
@task_item n=12 part=needs
The MCP tool results keep their success, refusal and invalid-params shapes; the count adds no refusal cause, exit code or hint.
@task_item n=13 part=needs
viola wait timed-out, dialog, dialog-expired and dialog unknown outputs stay as they are: the count is carried only on a turn end.
@task_item n=14 part=needs
Tests prove the count on the fake agent and the recorded Stop payloads, no live claude start: a session with wakes reads their number, one with none reads none, no wake content reaches an output.
@task_standing n=1
convention: every --json output is one document on stdout and every new field on it is a member of that verb's typed document
home: surface-cli
@task_standing n=2
convention: human output is a contract: a changed or added human line is asserted by its exact text in a CLI test
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page, and the count reaches no page.
@task_item n=2 part=not_needed
The viola list human board's rows, columns, widths, truncation, stale rows, colour depth and BAY line: the count adds no column to the board.
@task_item n=3 part=not_needed
CLI token rendering, SGR styling, TTY detection, colour roles and amber: the count is a plain number and adds no styled cell.
@task_item n=4 part=not_needed
viola send output, its readback mirror, readback vocabulary, word column and send refusals: a send's outcome does not change.
@task_item n=5 part=not_needed
viola revive and viola run start outputs, their --json refusal codes and start preflight: no start path changes.
@task_item n=6 part=not_needed
viola verify output and refusals: the chunk adds no verify step.
@task_item n=7 part=not_needed
viola link and unlink output and viola plugin install output: untouched.
@task_item n=8 part=not_needed
viola answer output and the dialog verbs: the count has no bearing on a dialog.
@task_item n=9 part=not_needed
Wheel verb output (pause, release) and the wheel and handoff spoken phraseology: the chunk changes no wheel state; the human-origin finding of a scheduled wake is placed as a block, not built.
@task_item n=10 part=not_needed
viola --help and per-verb usage lines, global flags and argument order: no verb, flag or argument is added.
@task_item n=11 part=not_needed
Frame verb exit-21 hints, instance-unreachable details and strict-modes refusals: the chunk adds no unreachable case.
@task_item n=12 part=not_needed
The no-banner, no-spinner, no-progress-glyphs, no-pager and no-footer rules: the chunk adds no live display.
