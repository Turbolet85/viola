@task_prose part=session
An API-error turn end now names its cause from a closed list of nine words: rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error.
The adapter maps the CLI's 13 StopFailure error words onto those nine; an absent word reads api-error, an outside word reads api-error and is logged once as parse-rejected with detail unknown-value, code and count only.
The error text still crosses nothing; the cause word reaches the turn-ended event, viola wait --json and the MCP wait tool only; no human output of any verb changed.
Every cause word clears the running turn the same way: STATUS reads idle after it, with no pause or release.
The adapter-internal API-error class list was retired; the classifier stays in the adapter and only the cause word crosses.
The 13 CLI error words were read from the installed CLI bundles, not measured in a live run.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the shared core types, holding the closed turn-end cause word set (now nine words) and its name lookup
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the Claude adapter's hook normaliser and hook-to-kind map, which classifies the StopFailure error word into the turn-end cause word and drops the error text
@task_file n=3 path=src/cmd/hook.rs
what: the viola hook verb's handler, reading hook stdin and writing the parse-rejected diagnostic line for an unknown value before delivering the event
@task_file n=4 path=schemas/diag-line.v1.json
what: the diagnostic log line schema, whose parse-rejected detail enum gains unknown-value
@task_file n=5 path=src/cmd/run.rs
what: the wrapper's viola run handler, re-checking a hook event's turn-end cause against the closed word set before appending it (tests added only)
@task_file n=6 path=src/run/send/slot.rs
what: the wrapper's send slot that holds the running turn and is cleared by a turn end (a test row added only)
@task_file n=7 path=crates/viola-state/tests/state_events.rs
what: the state events test suite for the turn-ended event data and its cause field
@task_file n=8 path=tests/hook_events.rs
what: the booted hook events suite, now driving every CLI error word through a wrapper to the turn-ended cause
@task_file n=9 path=tests/cli_wheel.rs
what: the wheel verbs suite (pause and release), with a rate-limited turn end letting the next send through
@task_file n=10 path=tests/cli_list.rs
what: the viola list board suite, with STATUS reading idle after an API-error turn end
@task_file n=11 path=crates/viola-mcp/tests/handlers.rs
what: the MCP tool handlers suite, now checking the wait tool carries a turn-end cause word unchanged in structuredContent
@task_item n=1 part=touches
contract/viola-wait-output/turn-ended: the turn-ended outcome's cause is now one of nine closed words in the --json form; the human wait line is unchanged and names no cause or error text.
@task_item n=2 part=touches
contract/mcp-tools/wait: the wait tool's turn-ended result carries the cause word unchanged in structuredContent, one of nine, isError false.
@task_item n=3 part=touches
contract/mcp-tool-results/success: a turn end with an API-error cause is still a success result, not a refusal.
@task_item n=4 part=touches
decision/turn-ended-not-send-outcome: every one of the nine cause words is a turn end, never a send outcome or refusal; check the row names no single cause word.
@task_item n=5 part=touches
contract/viola-list-board/status-join: after any of the nine cause words the running turn clears and STATUS reads idle; check any wording that names api-error alone.
@task_item n=6 part=touches
decision/human-output-is-contract: the cause word reaches --json and MCP only; no human output line changed, so the contract holds unchanged.
@task_item n=7 part=touches
decision/hints-never-quote-or-locate: the CLI's API error text is still dropped at the parse and never shown; only a closed word crosses.
@task_item n=8 part=touches
decision/words-carry-every-state: the turn-end cause is now carried by nine closed words rather than one.
@task_item n=9 part=touches
contract/cli-json-mode/one-document: viola wait --json still prints one document; its turn-ended cause value set grew additively with no version bump.
@task_item n=10 part=touches
contract/viola-wait-output/dialog-unknown: a sibling wait outcome; check it states no single api-error cause word in a shared clause.
@task_item n=11 part=touches
contract/cli-verbs/wait: the wait verb's turn-ended result may now carry any of the nine cause words; check any wording naming api-error alone.
@task_fact n=1 slug=surface-cli
fact: An API-error turn end is shown as a turn end on every surface; its cause is one closed word of nine (rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error), carried only by viola wait --json and the MCP wait tool, never in human output and never with the CLI's error text.
@task_item n=1 part=untouched
The web-spa surface and its absence row: viola still renders no page.
@task_item n=2 part=untouched
CLI exit codes and the exit-21 instance-unreachable details and hints: no exit code, detail word or hint changed; their closed cause words are another set.
@task_item n=3 part=untouched
The CLI token rendering rows, colours, styling, glyphs, banner, spinner, footer and board layout rows: no styling or layout moved.
@task_item n=4 part=untouched
viola send output, the readback mirror, send refusals and the send JSON document: no send line or word changed.
@task_item n=5 part=untouched
viola revive, viola run start, viola link and unlink, viola plugin install, viola verify, viola last and viola answer outputs and refusals: none reached.
@task_item n=6 part=untouched
viola --help and per-verb usage, global flags and the MCP hidden-verb rows: no verb, flag or tool was added or removed.
