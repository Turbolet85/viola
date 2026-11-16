@task_prose part=chunk
Grows the turn-ended event's closed cause set from the one word api-error to nine: rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error.
The Claude adapter's existing API-error classifier now hands its word to the turn-ended cause; api-error stays the word for the CLI's unknown, an absent error and any value outside the map.
Every consumer that accepts exactly api-error today (the wrapper's appended-line check, event-log readers, the board, viola wait in CLI and MCP, the contract tests) accepts the nine and still refuses any other text.
The CLI's error text still crosses nothing; the plan says how a new CLI error value is noticed later (a count or a closed word, never its text) or states that it is not.
Re-measures at P3 where the word is produced, every consumer of it, and each known CLI error value as measured or read-from-source; no live claude start, no mutation run.
@task_file n=1 path=crates/viola-core/src/lib.rs mark=edits
what: the core's closed turn-end cause value set of the turn-ended event, owned by viola-core — edits: TurnEndCause (its variants and its ALL list)
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the Claude adapter's hook payload reading and its API-error classifier, the one place a StopFailure's error word is classified — edits: ApiErrorClass, api_error_class, data_of
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the wrapper's appended-line check, which admits a turn-ended line only with cause absent or a closed cause word — edits: the appended-line check
@task_item n=1 part=needs
The turn-ended event's cause field: a closed value set of nine words, still one word, never free text, never a new event kind.
@task_item n=2 part=needs
An API-error turn end now carries one of the nine cause words instead of the single api-error word; api-error remains for unknown, an absent error and any unmapped value.
@task_item n=3 part=needs
viola wait --json and MCP wait: the event form of a turn-ended event carries the granular cause word and refuses any other text.
@task_item n=4 part=needs
viola wait human output, event form: a turn-ended event line naming its cause shows the granular word.
@task_item n=5 part=needs
The board (viola list): an API-error turn end is still shown as any turn end, whatever its cause word; the STATUS and WHEEL fields gain no cause-specific state.
@task_item n=6 part=needs
The readback box and send readback: a turn-ended event, of any cause, still never fills the readback box nor reads as a send's confirmation.
@task_item n=7 part=needs
The CLI's error text (its last assistant message, error details, the raw error value) reaches no event, snapshot, log line, reply, hint or diagnostics line other than as one of the nine words; errors and hints carry no internals and never quote text.
@task_item n=8 part=needs
Any human line that prints the cause word stays ASCII-only, escapes control characters, adds no colour, glyph or icon for a cause, and gives no hint for a fault or unknown.
@task_item n=9 part=needs
Data on stdout, messages on stderr: a --json result stays one stdout document; the cause word joins it as a field value, with no extra line.
@task_item n=10 part=needs
The wrapper's appended-line check accepts a turn-ended line with cause absent or one of the nine words and refuses any other cause text.
@task_item n=11 part=needs
Fixed phraseology vocabulary: the nine cause words are fixed words of the product's vocabulary, never renamed, added to or dropped by this chunk.
@task_item n=12 part=needs
A new CLI error value outside the map is noticed, if at all, by a count or a closed word, never by its text.
@task_standing n=1
convention: every new or changed human output line keeps to ASCII-only text, escaped control characters, no decorative glyph and no colour beyond the existing tokens
home: surface-cli · surface
@task_standing n=2
convention: every new log line, reply or diagnostics line names a fault by a closed word and carries no internals or quoted foreign text
home: anti-patterns-never-do-these
@task_item n=1 part=not_needed
Style direction, expression level, rejected style presets and motion rows: the chunk adds no visual element.
@task_item n=2 part=not_needed
Colour palette, typography, spacing, depth and border radius: no colour, type, width or spacing changes.
@task_item n=3 part=not_needed
Iconography and CLI glyph or separator contracts: no new glyph, separator or column.
@task_item n=4 part=not_needed
The web surface absence: no page or web surface is involved.
@task_item n=5 part=not_needed
Exit codes and exit 21 causes, their --json causes and hints: a turn-ended cause is no exit cause and adds no exit code.
@task_item n=6 part=not_needed
Revive, verify, link and unlink, plugin install and run start refusals: none of those verbs or refusal tables change.
@task_item n=7 part=not_needed
Strict-modes refusals, socket-directory refusals and fixed refusal pairs: no home or socket check is touched.
@task_item n=8 part=not_needed
Send refusal hints (not-delivered reasons) and peer reply screening code words: the cause set is a separate closed set and does not grow those lists.
@task_item n=9 part=not_needed
Wrapper fault --json detail codes: JSON-RPC fault details are untouched.
@task_item n=10 part=not_needed
Self-validation protocol design tests (squint, swap, readback box signature check): no design element is introduced to test.
@task_item n=11 part=not_needed
Command name register: no verb is added or renamed.
