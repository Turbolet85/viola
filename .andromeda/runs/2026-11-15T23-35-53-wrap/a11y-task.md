@task_prose part=session
The session registers the CLI's StopFailure hook as a tenth hook event (stop-failure), so a turn the CLI ends on an API error is logged as turn-ended rather than left running.
That turn-ended event's data gains a cause key holding one closed code word, api-error; the CLI's error text and error details are never copied into an event, snapshot, diagnostics line or output.
The wrapper re-validates the cause: a turn-ended with a cause outside the closed set is not appended; the running turn clears with no pause or release, a parked wait returns turn-ended exit 0, the next send is accepted and the board reads idle.
The CLI's error words are classed into nine classes inside the Claude adapter crate only; no class reaches data, an event or a log line.
Registered hook events move 9 to 10 (hooks.json, the diagnostics hook_event enum, the fixture and fake-script schemas, the fake agent); no new CLI verb, output wording, exit code, env var or event kind.
CI on the pushed HEAD read red on one timeout in a run-start test outside the change, ruled not this chunk's and left open; the failed job was re-run once for this chunk's verdict only.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: edited — the shared core crate's closed code-word sets and event kinds: gains the closed turn-end cause set (api-error) and the turn-ended event's third data key
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: edited — the Claude adapter's hook-event normaliser: the stop-failure hook maps to turn-ended with the api-error cause, drops the vendor error text, and classes the error word inside the crate
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: edited — the Claude adapter's capability ledger: the hook event name list gains StopFailure (no ledger row)
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited — the capability ledger's stamp test cases, one line for the new event
@task_file n=5 path=crates/viola-agent-claude/src/lib.rs
what: edited — the Claude adapter crate's normalise entry and its unit tests
@task_file n=6 path=plugin/hooks/hooks.json
what: edited — the plugin's hook registration: gains StopFailure calling viola hook stop-failure with Stop's 5 s timeout, ten entries
@task_file n=7 path=src/cmd/run.rs
what: edited — the wrapper's hook-event channel input: re-validates the turn-ended cause against the closed set before appending to the event line
@task_file n=8 path=src/run/send/slot.rs
what: edited — the send slot's turn rung tests: a turn end lets the next send past
@task_file n=9 path=src/bin/viola-fake-agent.rs
what: edited — the fake agent test double: its registered events gain StopFailure and the instance goes idle on it
@task_file n=10 path=schemas/diag-line.v1.json
what: edited — the diagnostics line schema: the closed hook_event value list gains stop-failure (ten values)
@task_file n=11 path=schemas/claude-fixture.v1.json
what: edited — the recorded Claude fixture schema: the event enum gains StopFailure
@task_file n=12 path=schemas/fake-script.v1.json
what: edited — the fake agent script schema: the event enum gains StopFailure
@task_file n=13 path=tests/hook_events.rs
what: edited — the every-event hook witness suite: gains the absence probe that the vendor error text reaches no event line, snapshot or diagnostics file
@task_file n=14 path=tests/cli_wheel.rs
what: edited — the wheel and send critical-path suite: an API-error turn end clears the turn with no pause or release, a parked wait returns, the next send is accepted
@task_file n=15 path=tests/cli_list.rs
what: edited — the viola list board suite: the STATUS word reads idle after an API-error turn end and a second send is accepted
@task_file n=16 path=tests/hook_fail_open.rs
what: edited — the hook fail-open suite: gains a stop-failure unreachable row; the shared case reads its own event name
@task_file n=17 path=tests/security_negatives_strict_modes.rs
what: edited — the strict-modes hook-call refusal suite: the hook list gains stop-failure (ten)
@task_file n=18 path=crates/viola-state/tests/state_events.rs
what: edited — the state crate's event-line tests: turn-ended's unknown-field count with the cause key
@task_item n=1 part=touches
Any row counting the registered hook events as nine, or the diagnostics hook_event value list as one of nine: the count is now ten (stop-failure added).
@task_item n=2 part=touches
The viola hook event and dialog call contracts (server not the recorded wrapper, socket directory failure, strict-modes failure): a new hook call, viola hook stop-failure, now falls under each, fail-open within its bound.
@task_item n=3 part=touches
The single vendor-content boundary and the rule that vendor content never crosses: the CLI's API-error text is dropped at the parse and reaches no event line, snapshot, diagnostics file, wait output or terminal byte.
@task_item n=4 part=touches
Peer replies screened to code words and the code-word format: the new cause value api-error is a closed kebab-case code word re-validated by the wrapper; a value outside the closed set is not appended.
@task_item n=5 part=touches
No stack trace or bare status as error text: an API error ending a turn surfaces only as the code word api-error, never the vendor's error string.
@task_item n=6 part=touches
Critical path confirmed send and readback, and the viola send refusal input-not-ready: after an API-error turn end the next send is accepted instead of refused as turn-running.
@task_item n=7 part=touches
Critical path human takes the wheel: an API-error turn end clears the running turn with no pause or release; the wheel holds only its start record.
@task_item n=8 part=touches
Critical path CLI board, wait/last output and the viola wait contracts (waiting line, timed_out): a parked wait now wakes on an API-error turn end with turn-ended, exit 0, its --json data holding the cause word.
@task_item n=9 part=touches
The viola list board and state carried by the printed word: the STATUS word reads idle after an API-error turn end (no new wording or colour).
@task_item n=10 part=touches
Run diagnostics never reach the terminal: the new hook writes hook-invoked and hook-decision diagnostics lines with hook_event stop-failure, off the terminal.
@task_item n=11 part=touches
Plugin and IPC channel not assertable: the plugin's hook registration gains an entry and the hook-event channel gains a re-validated field, both held by integration tests, not a11y assertions.
@task_item n=12 part=touches
A11y assertions inside existing suites and a11y cases inside the coverage suite: the new cases (board idle, absence probe, wheel clear) were added to the existing suites, no new suite or driver.
@task_item n=13 part=touches
Zero flakiness, no retries, and gate breach fails the build: CI on the pushed HEAD read red on one timed-out run-start test outside the change, left open, and the failed job was re-run once for this chunk's verdict only.
@task_item n=14 part=touches
Time bounds fall on the driver only: the new hook carries Stop's 5 s timeout and fails open within 1.0 s; no bound falls on the human.
@task_fact n=1 slug=cognitive-accessibility
fact: A turn the agent CLI ends on an API error is reported as turn-ended carrying one closed cause code word, api-error; the vendor's error text never reaches any output, and finer error classes stay inside the adapter (provisional, founder fork F2).
@task_item n=1 part=untouched
TUI passthrough rows on human keystrokes, focus, mouse and resize sequences, vt100 feed panic and viola terminal bytes: the session adds no terminal byte and no stdin handling.
@task_item n=2 part=untouched
Colour and SGR rows (amber DIALOG, bold NAME, dim stale rows, failure words never coloured, no SGR off a terminal): no output wording or styling changed.
@task_item n=3 part=untouched
CLI exit code rows, refusal layout (unable, hint, stderr layout, --json failure documents) and the one-hint-per-code rules: no new refusal, exit code or hint.
@task_item n=4 part=untouched
viola revive, link, unlink, plugin install, verify and --help contracts: none of those verbs or their outputs changed.
@task_item n=5 part=untouched
Dialog rows (dialog hold, dialog-expired, critical path dialog answer): the new hook event is not a dialog and no dialog bound moved.
@task_item n=6 part=untouched
Screen-reader setup and manual keyboard and Orca pass rows, the tier, the absences (i18n, touch, web or graphical frontend, rendered page) and the structured violation JSON: none is reached.
