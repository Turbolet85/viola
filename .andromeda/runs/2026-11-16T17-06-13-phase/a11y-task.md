@task_prose part=chunk
An API-error turn end now gives its cause as one of nine closed words, not the single word api-error: rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error.
The claude adapter classifies the CLI's StopFailure error value as it does today, and only the resulting word goes into the turn-ended event's cause field, events.ndjson, viola wait --json and the MCP wait tool.
A value the map does not hold, an absent error or the CLI's unknown all read api-error. The CLI's error text still reaches no event, snapshot, log line, reply or diagnostics line.
Every consumer that accepts exactly api-error today accepts the nine and still refuses any other text. P3 re-measures where the word is produced, every consumer, and each known CLI error value (measured or read-from-source).
Proof uses the fake agent and recorded payloads only: no live claude start and no mutation run.
@task_file n=1 path=src/cmd/run.rs mark=edits
what: the wrapper's check on each line appended to the event log, which admits a turn-ended line only when cause is absent or a closed cause word — edits: the appended-line check
@task_item n=1 part=needs
The turn-ended event's cause field: its closed value set grows from one word to nine kebab-case words. It is an existing field of an existing event kind, so no new event kind is added.
@task_item n=2 part=needs
The vendor-content boundary: the CLI's error text (last assistant message, error details, the raw error value) is classified inside the claude adapter, and only one of the nine closed words crosses into viola's own output.
@task_item n=3 part=needs
No vendor text in any event line, snapshot, log line, reply or run diagnostics line; a turn end's cause is a code word and never bare error text.
@task_item n=4 part=needs
Fallback: an unknown, absent or unmapped CLI error value reads api-error. Any later notice of a new CLI value is a count or a closed log word, never its text.
@task_item n=5 part=needs
viola wait --json output and the MCP wait tool: the cause they carry for a turn end can now be any of the nine words, and they still refuse any other text.
@task_item n=6 part=needs
The viola list board and other readers of the event log accept the nine cause words and refuse any other text. The state is carried by its printed word, never by colour alone.
@task_item n=7 part=needs
The wrapper's appended-line check on the event log admits a turn-ended line whose cause is absent or one of the nine words, and refuses any other value.
@task_item n=8 part=needs
The code word format: each of the nine cause words is a lowercase kebab-case code word with nothing interpolated beside it.
@task_item n=9 part=needs
Fixed strings pinned by literal: each of the nine words and the fallback are asserted by their literal text in the contract tests.
@task_item n=10 part=needs
Peer replies and readings are screened to closed code words: an incoming cause that is not one of the nine is refused, not echoed as text.
@task_item n=11 part=needs
Live readings never gate: proof rests on the fake agent and recorded payloads, and any mapping that needs a live reading is named at P3 with its probe shape and kept out of the gate.
@task_item n=12 part=needs
Control characters: no cause value carries raw bytes from the CLI, so no C0/C1 byte of vendor text can reach CLI output through the cause field.
@task_standing n=1
convention: each new a11y or output-discipline assertion for the nine cause words runs inside the existing test suites, with no separate a11y suite or driver
home: 1-a11y-scope-summary
@task_standing n=2
convention: each fixed string a new test asserts is pinned by its literal
home: 8-cognitive-accessibility · cognitive-accessibility
@task_standing n=3
convention: each new test is deterministic and bounded, with no retry and no skip
home: 10-slo-invariants-a11y-budgets
@task_standing n=4
convention: each new a11y check fails as an ordinary nextest failure inside the existing suite
home: bootstrap-phases-derive-for-route-setup-project
@task_item n=1 part=not_needed
TUI passthrough of human keystrokes, focus, mouse and resize sequences, and vt100 feed panic handling: the chunk changes no terminal bytes and no keystroke path.
@task_item n=2 part=not_needed
CLI refusal output (unable and hint lines, refusal documents, exit codes) and the revive, link, unlink, plugin install and start refusals: the chunk adds no refusal and changes no exit code.
@task_item n=3 part=not_needed
Socket-directory, strict-modes and server-verify refusals on frame verbs, hook calls and MCP frame tools: the chunk changes no transport or trust check.
@task_item n=4 part=not_needed
Dialog-hold bounds, dialog-expired events and send refusals while the human types: the chunk changes no dialog or send path.
@task_item n=5 part=not_needed
viola --help rows, group labels and usage line: the chunk adds no verb or flag.
@task_item n=6 part=not_needed
Absences of a web or graphical frontend, a touch surface, internationalization or a user time limit, and the manual Orca and keyboard passes: the chunk adds no surface they rule over.
@task_item n=7 part=not_needed
Name truncation and Bold or Dim styling of NAME values and stale rows: the chunk changes no name or staleness column.
