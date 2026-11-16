@task_prose part=session
The session added a tenth registered hook event, StopFailure (hook name stop-failure), which maps to the turn-ended kind: a turn the claude CLI ends on an API error is now logged as turn-ended instead of leaving the turn running.
The turn-ended event's data gains a third key, cause, holding the closed word api-error on an API-error end only; it is absent on an ordinary Stop end. The CLI's error text and error details are never copied anywhere.
A new closed cause set, TurnEndCause (one member, api-error), lives in viola-core beside UnreachableCause. The wrapper appends a turn-ended line only when its cause is absent or is a word in that set.
After an API-error end the running turn clears with no pause or release, a parked viola wait returns turn-ended with exit 0, viola list shows the instance as idle, and the next viola send is accepted. No output wording is new.
The CLI error word is sorted into nine classes inside the Claude adapter, but no class reaches data, an event, a snapshot or any output. The cause word reaching events.ndjson, wait --json and MCP wait is PROVISIONAL (founder fork F2).
Counts that moved: registered hook events 9 to 10, and turn-ended data keys 2 to 3. Nothing was retired. No new verb, flag, exit code, refusal or MCP tool was added.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the shared core vocabulary: the event kinds and their data keys, and the closed cause sets (UnreachableCause, now also TurnEndCause)
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the Claude hook adapter: the hook event list, the hook-to-kind map and the normalisation of hook payloads into event data
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger's hook event name map
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the capability ledger's stamp test cases
@task_file n=5 path=crates/viola-agent-claude/src/lib.rs
what: the Claude adapter crate's entry, holding its hook event name tests
@task_file n=6 path=plugin/hooks/hooks.json
what: the viola plugin's hook registration list, mapping each CLI hook event to a viola hook command
@task_file n=7 path=src/cmd/run.rs
what: the viola run wrapper: its revalidation of hook events from the channel before it appends them to the event log
@task_file n=8 path=src/run/send/slot.rs
what: the wrapper's send slot, which a turn end clears so the next send goes through
@task_file n=9 path=src/bin/viola-fake-agent.rs
what: the fake agent test double and its registered hook event set
@task_file n=10 path=schemas/diag-line.v1.json
what: the diagnostic line schema and its closed set of hook event names
@task_file n=11 path=schemas/claude-fixture.v1.json
what: the recorded Claude fixture schema and its event name enum
@task_file n=12 path=schemas/fake-script.v1.json
what: the fake agent script schema and its event name enum
@task_file n=13 path=tests/hook_events.rs
what: the every-hook-event witness test and the error-text absence probe
@task_file n=14 path=tests/cli_wheel.rs
what: the wheel and turn end-to-end test: a turn cleared with no pause or release, a parked wait, and the next send
@task_file n=15 path=tests/cli_list.rs
what: the viola list board tests: STATUS idle after an API-error turn end
@task_file n=16 path=tests/hook_fail_open.rs
what: the hook fail-open tests across hook events
@task_file n=17 path=tests/security_negatives_strict_modes.rs
what: the strict-modes security negative tests over every registered hook
@task_file n=18 path=crates/viola-state/tests/state_events.rs
what: the state crate's event log tests, including the count of unknown data fields
@task_item n=1 part=touches
viola wait output for turn-ended: a parked wait now also returns turn-ended, exit 0, when the turn ends on an API error. The --json document's event data may carry cause api-error. Check whether the turn-ended wait contract row says what the data holds or lists its keys.
@task_item n=2 part=touches
viola mcp tools, wait: the MCP wait result's turn-ended data can now carry the cause word api-error (PROVISIONAL, F2). Check the wait tool row and the MCP success result row against a fixed key list.
@task_item n=3 part=touches
contract/cli-verbs/wait: a turn the CLI ends on an API error now ends a wait as turn-ended. A row that names only a Stop end as a turn end is now incomplete.
@task_item n=4 part=touches
decision/turn-ended-not-send-outcome: turn-ended now also covers an API-error end. It is still not a send outcome, and no send outcome word was added. Check that the row's reasoning still holds.
@task_item n=5 part=touches
viola list board STATUS join (contract/viola-list-board/status-join): after an API-error turn end the board reads idle, not a running turn. No new STATUS word was added. Check any row that ties idle to a Stop end only.
@task_item n=6 part=touches
viola send after a turn end: once an API-error end clears the running turn, the next send is accepted with no pause or release. Before the end, the one refusal is turn-running. Check the send output and refusal rows that describe when a send is refused as turn-running.
@task_item n=7 part=touches
wheel verb output (pause, release): an API-error end clears the turn with no wheel action, and the wheel records only its start. Check any row that says a running turn clears only through Stop or a wheel verb.
@task_item n=8 part=touches
Hints never quote text (decision/hints-never-quote-or-locate) and the --json documents: the CLI's API error text never reaches any output, and only the closed word api-error does. This confirms the rule rather than breaking it. Flag only if a row lists the closed words a turn-ended document may hold.
@task_item n=9 part=touches
Readback vocabulary and words-carry-every-state: a new closed word, api-error, now exists as a turn-end cause. It is not a readback state word, and no row should take it as one unless the row lists every closed word that can appear in output.
@task_fact n=1 slug=surface-cli
fact: On a turn the CLI ends on an API error, viola wait (human and --json) and the MCP wait tool return turn-ended with exit 0, like a Stop end. The --json and MCP data carries cause api-error, a closed word, and no error text or error class ever reaches output (PROVISIONAL, founder fork F2).
@task_fact n=2 slug=surface-cli
fact: After an API-error turn end, viola list STATUS reads idle and the next viola send is accepted with no pause or release. No new board or send wording exists for this end.
@task_item n=1 part=untouched
The web surface absence and the absence of deep links: no page, URL or link was added.
@task_item n=2 part=untouched
CLI token rendering (colours, roles, spacing, radius, motion) and the styling decisions (SGR, amber for dialog, no spinner, no banner, no footer): no styling changed.
@task_item n=3 part=untouched
CLI exit codes 10 to 21, the exit-21 hints and the --json details for an unreachable instance: no exit code, refusal cause or hint was added or moved.
@task_item n=4 part=untouched
viola revive output, refusals and --json error codes, and the viola run and viola revive start outputs and start refusals: not reached.
@task_item n=5 part=untouched
viola link and unlink output, viola last output, viola answer output, and the --help and per-verb usage output: no verb, flag or usage line changed.
@task_item n=6 part=untouched
The BAY line, board column widths, name truncation, ordering and the board's escaping: the board layout is unchanged; only which existing STATUS word appears after an API-error end.
