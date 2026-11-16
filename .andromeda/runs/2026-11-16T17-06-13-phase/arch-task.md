@task_prose part=chunk
Grows the turn-ended event's closed cause set from the one word api-error to nine: rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error.
A StopFailure's error word stays classified inside viola-agent-claude as it already is; only the resulting word crosses into the turn-ended cause, the events log, viola wait --json and MCP wait.
api-error now stands for the CLI's unknown, an absent error and any value outside the map; the CLI's error text still reaches no event, snapshot, log line, reply or diagnostics line.
Every consumer that accepts exactly api-error today accepts the nine and still refuses any other text; the plan states how a new CLI error value is noticed later (a count or closed log word, never its text) or that it is not.
Re-measures at P3 where the word is produced, every consumer of it, and each known CLI error value as measured (recorded payload) or read-from-source; no live claude start, no mutation run.
@task_file n=1 path=src/cmd/run.rs mark=edits
what: the wrapper's run path in the root bin, holding the channel hook.event appended-line check that admits a turn-ended line only with cause absent or a turn-end cause word — edits: the appended-line check of the turn-ended cause
@task_item n=1 part=needs
The turn-end cause set contract and the turn-ended cause in event data per kind: the closed set grows from one word to exactly nine, owned by viola-core as a closed value set of an existing field, kebab-case on the wire.
@task_item n=2 part=needs
No new event kind is minted: event kinds are added only in viola-core and this change alters a value set of the existing turn-ended kind, not the normalised event kinds list.
@task_item n=3 part=needs
The StopFailure classifier in viola-agent-claude: its existing nine-class map becomes the source of the cause word written into the turn-ended event data, replacing the one fixed api-error word; Claude payload shapes stay in that one crate and it stays pure, with no new Claude shape outside it.
@task_item n=4 part=needs
The hook-to-kind map entry for StopFailure: it still yields turn-ended, now carrying the classified word as its cause.
@task_item n=5 part=needs
Unmapped external values: the CLI's unknown, an absent error and any value outside the map read api-error, under the rules that external payloads are parsed tolerantly and unparseable external values become the catch-all word.
@task_item n=6 part=needs
Error text crosses nothing: last_assistant_message, error_details and the raw error value reach no event, snapshot, process log, diagnostics line, reply or human line other than as one of the nine closed words, under the upstream-text-is-untrusted, sanitised error surfaces and content-only-in-the-instance rules.
@task_item n=7 part=needs
The wrapper's channel hook.event appended-line check: a turn-ended line is admitted only with cause absent or one of the nine words, and any other text is still refused.
@task_item n=8 part=needs
Readers of the events log and the snapshot replay: they accept the nine words in turn-ended cause and treat any other text as they treat an unknown value today; missing and null cause still read the same.
@task_item n=9 part=needs
The board (CLI list and MCP list) and the list payload: whatever status or field reads a turn-ended cause accepts the nine words and refuses other text.
@task_item n=10 part=needs
viola wait --json and MCP wait (channel wait result and wake kinds): the turn-ended event they return carries the classified word as cause, under the MCP wait cap and existing result shapes.
@task_item n=11 part=needs
API-error turn end clears the running turn: the rule holds for a turn end carrying any of the nine words, not only api-error.
@task_item n=12 part=needs
Noticing a new CLI error value: if the plan adds a signal, it is a count or a closed code word in a codes-only process log or diagnostics line emitted through the one event emitter, never the value's text; otherwise the plan states plainly that no signal exists.
@task_item n=13 part=needs
Mixed-version tolerance of the own formats and the protocol version bump rule: whether a wider closed value set in an existing field bumps the event or channel protocol version, and how an older reader meets a word it does not know.
@task_item n=14 part=needs
Proof rests on the fake agent and recorded StopFailure payloads only: no live claude in CI or in this plan, real CLI probes only locally, and no probe for a shape no send relies on; a mapping that needs a live reading is named at P3's end with its probe shape and count.
@task_item n=15 part=needs
The capability ledger's reading of the CLI's StopFailure error values: each matcher value (rate_limit, overloaded, the auth values, billing_error, invalid_request, model_not_found, server_error, max_output_tokens) re-marked as measured or read-from-source.
@task_item n=16 part=needs
The contract tests that pin the turn-ended cause today: updated to accept the nine words and to keep refusing any other text.
@task_item n=17 part=needs
New tests owe a bound: every wait in a new test sits under the shared root test wait bound, and every control is named with a timeout and a short id.
@task_standing n=1
convention: each new test runs in its own isolated viola home
home: established-decisions
@task_standing n=2
convention: each new test runs against the fake agent, never a live claude
home: ci-cd-approach
@task_standing n=3
convention: each recorded payload fixture added is scrubbed and hygiene-walked before it is committed
home: occupied-resources
@task_standing n=4
convention: each new test's wait shares the one root test wait bound and its evidence survives a killed test
home: occupied-resources
@task_standing n=5
convention: each new snapshot assertion runs in check mode only, and new end-to-end runs keep zero panic lines
home: project-directory-structure
@task_standing n=6
convention: each new log line is codes-only, emitted through the one event emitter, with no upstream text interpolated
home: cross-cutting-patterns · build-system
@task_item n=1 part=not_needed
The PTY seam, vt100 screen model, readiness gate and screen signatures: the change reads no screen and types nothing.
@task_item n=2 part=not_needed
The wheel, its byte-source claim, release and human-interrupt rows: no wheel state or move is touched.
@task_item n=3 part=not_needed
Dialog hold, pending dialog, answer and permission/plan/question answer rows: StopFailure is no dialog.
@task_item n=4 part=not_needed
Send flight, paste, bracketed send, send refusals and send confirmation windows: the change alters no send path.
@task_item n=5 part=not_needed
Link and unlink recording and their channel methods: no link state moves.
@task_item n=6 part=not_needed
Revive, run start refusals, pinned exe copy and child argv or environment rows: no start or revive behaviour changes.
@task_item n=7 part=not_needed
Hyprland window seam and terminal fleet console rows: no window or console feature is reached.
@task_item n=8 part=not_needed
viola verify exit codes, refusal codes and its probe directories, and plugin install exit codes: neither verb is changed.
@task_item n=9 part=not_needed
Socket directory checks, endpoint naming, peer euid admission and server verification order: no channel transport rule changes.
@task_item n=10 part=not_needed
config.json keys, config precedence and colour environment rows: no configuration is added.
@task_item n=11 part=not_needed
Release tooling, crates.io publishing, licence and deployment rows: nothing is released or deployed.
