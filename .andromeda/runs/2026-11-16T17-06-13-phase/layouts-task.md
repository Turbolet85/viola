@task_prose part=chunk
The turn-ended event's closed cause set grows from the one word api-error to nine: rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error.
The Claude adapter classifies a StopFailure's error value as it already does; only the resulting word crosses into the event's cause and so into the event log, viola wait --json and the MCP wait tool.
Every consumer that today accepts exactly api-error (wrapper's appended-line check, event-log readers, board and list, wait on CLI and MCP, contract tests) accepts the nine and still refuses any other text.
An unmapped, unknown or absent error value reads api-error; the plan says how a new CLI error value is noticed (a count or closed log word, never its text) or states that it is not.
It re-measures at P3 where the word is produced, every consumer of the word, and each known CLI error value as measured or read-from-source; no live claude start, no mutation run.
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the Claude hook adapter that classifies the CLI's StopFailure error value into an API-error class and builds the turn-ended event data — edits: ApiErrorClass, api_error_class, data_of
@task_file n=2 path=crates/viola-core/src/lib.rs mark=edits
what: the core shared vocabulary that owns the turn-ended event's closed cause value set — edits: TurnEndCause and its ALL list
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the wrapper of viola run — its appended-line check that admits a turn-ended line only with cause absent or a closed cause word — edits: the appended-line check
@task_item n=1 part=needs
The turn-ended event's cause field: a closed set of nine words, each a fixed lowercase hyphenated token, api-error standing for unknown, absent and any value outside the map.
@task_item n=2 part=needs
viola wait output for a turn end: the human turn-ended line and the --json document carry the cause as one of the nine closed words and nothing beside it.
@task_item n=3 part=needs
The MCP wait tool's success result carries the cause as one of the nine closed words; any other text is refused, as on the CLI path.
@task_item n=4 part=needs
The board and viola list (human and --json) read a turn-ended cause from the event log and accept the nine words, still refusing any other text.
@task_item n=5 part=needs
The CLI's error text (last_assistant_message, error_details, the raw error value) reaches no output line, --json document, MCP result, refusal, hint or diagnostics line other than as one of the nine words; hints never quote text.
@task_item n=6 part=needs
turn-ended stays apart from a send outcome: the nine causes are turn-end causes and add no word to the send readback vocabulary or the send outcome lines.
@task_item n=7 part=needs
Human output is a contract: any changed turn-ended line of viola wait is pinned by a test against its exact text, with words carrying the state and no colour or glyph standing in for the cause.
@task_item n=8 part=needs
--json on the data verbs stays one document per call; the cause value in it is a closed word a consumer can match on.
@task_item n=9 part=needs
An unmapped new CLI error value is either noticed by a count or a closed log word, never its text, or the plan states plainly that it is not; nothing on any CLI or MCP surface carries the unmapped value.
@task_item n=10 part=needs
No new event kind is minted and no Claude-specific shape leaves the adapter: the change is to the closed value set of an existing field.
@task_item n=11 part=needs
Proof rests on the fake agent and recorded payloads, with bounded tests and named controls using --timeout S and ids of at most 16 characters; a mapping needing a live reading is named at P3 with its probe shape.
@task_standing n=1
convention: each new or changed CLI or MCP output line or --json field is a contract pinned by a test on its exact text or document shape
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page, and this chunk adds none.
@task_item n=2 part=not_needed
CLI token rendering rows (colour roles, spacing, radius, motion tokens) and SGR styling: the cause is a word, no colour or style is added for it.
@task_item n=3 part=not_needed
viola send output and the readback mirror (issue line, read back for slash commands, word column, unable, unconfirmable): a turn end's cause is not a send outcome.
@task_item n=4 part=not_needed
viola run and viola revive start output, their refusals and --json error codes: the chunk changes no start path.
@task_item n=5 part=not_needed
viola verify output and refusals, and viola plugin install output: neither reads the turn-ended cause.
@task_item n=6 part=not_needed
viola link and unlink output, and the wheel verbs pause and release output: the chunk touches neither.
@task_item n=7 part=not_needed
viola --help and per-verb usage output, and the help verb groups: no verb, flag or usage line is added.
@task_item n=8 part=not_needed
Frame-verb exit-21 hints and exit-21 --json detail rows: the chunk adds no unreachable-instance cause.
