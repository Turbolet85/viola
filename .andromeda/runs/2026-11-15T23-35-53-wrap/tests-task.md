@task_prose part=session
The session registers the agent CLI's StopFailure hook as a tenth hook event, stop-failure, which logs a turn-ended event when the CLI ends a turn on an API error.
That turn-ended data is exactly last_assistant_message null, background_tasks null, cause api-error. cause is a new closed one-member turn-end cause set in the core crate, and it is absent on a Stop end.
The API error's text (the payload's error, error_details and last_assistant_message) is never copied. It reaches no event line, snapshot, log line or detail file, and an absence probe with a planted value holds this.
The wrapper appends a hook turn-ended whose cause is absent or a word of the closed set; any other value is not appended. After the end the instance reads idle, a parked wait wakes on turn-ended and the next send is accepted, with no pause or release.
Registered hook events, hooks.json entries, the diag-line hook_event enum and the fake agent's registered events move from 9 to 10. turn-ended data keys move from 2 to 3. No recorded fixture, ledger row or verify probe was added, and no mutation run or live claude start was made.
Measured after implement: the CI red on the pushed head is a 20 s kill timeout of a stamped-home run-start test outside the change. The time sits in the stamped-home fixture's viola verify under host load. It stays open.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: edited — the shared event kinds and their data keys (turn-ended gains cause) and the new closed turn-end cause set, beside the instance-unreachable causes
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: edited — the hook event set and each hook payload's mapping to an event kind and its data; gains stop-failure and the API-error word classes held inside the adapter
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: edited — the capability ledger's hook event names
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited — the ledger and stamp unit cases
@task_file n=5 path=crates/viola-agent-claude/src/lib.rs
what: edited — the agent adapter's normalisation of hook payloads into events and the payload drift report
@task_file n=6 path=plugin/hooks/hooks.json
what: edited — the generated plugin's hook commands, exec form on the pinned path with per-hook timeouts; ten entries
@task_file n=7 path=src/cmd/run.rs
what: edited — the wrapper's hook.event channel input and its re-validation before a hook event line is appended
@task_file n=8 path=src/run/send/slot.rs
what: edited — the send refusal rung for a running turn (turn-running) and its clearing on a turn end
@task_file n=9 path=src/bin/viola-fake-agent.rs
what: edited — the fake agent, its registered hook events and the registry row status it sets per event
@task_file n=10 path=schemas/diag-line.v1.json
what: edited — the diagnostics line schema and its closed hook_event values
@task_file n=11 path=schemas/claude-fixture.v1.json
what: edited — the recorded Claude fixture schema and its hook event enum
@task_file n=12 path=schemas/fake-script.v1.json
what: edited — the fake agent turn script schema and its hook event enum
@task_file n=13 path=tests/hook_events.rs
what: edited — the every-hook-event integration witness and the API-error text absence probe
@task_file n=14 path=tests/cli_wheel.rs
what: edited — the wheel and send critical-path E2E (an API-error turn end clears the turn with no pause or release)
@task_file n=15 path=tests/cli_list.rs
what: edited — the viola list board E2E (status reads idle after an API-error turn end)
@task_file n=16 path=tests/hook_fail_open.rs
what: edited — the hook fail-open cases and the spine-bound case table
@task_file n=17 path=tests/security_negatives_strict_modes.rs
what: edited — the strict-modes security negatives over every registered hook
@task_file n=18 path=crates/viola-state/tests/state_events.rs
what: edited — the tolerant event log reader's tests, its unknown-field count
@task_item n=1 part=touches
contract/event-data/turn-ended: turn-ended data keys go from two to three. cause (api-error) is present on an API-error end and absent on a Stop end, so a row listing the keys last_assistant_message and background_tasks alone is stale.
@task_item n=2 part=touches
The row stating a check over the nine registered hook events is stale. Registered hook events, hooks.json entries and the fake agent's registered events are now ten, StopFailure joining them.
@task_item n=3 part=touches
contract/generated-plugin/hook-commands and contract/viola-run-start/plugin-hooks: the generated hooks.json gains StopFailure, running viola hook stop-failure in exec form on the pinned path with Stop's timeout of 5.
@task_item n=4 part=touches
contract/turn-ended/background-tasks and contract/dialog-data/null-details: background_tasks and last_assistant_message are null on the API-error turn end.
@task_item n=5 part=touches
contract/viola-wait/turn-ended, contract/viola-wait/wake, contract/channel-wait/wake-kinds, contract/wait/wake-kinds: a parked wait now also wakes on the API-error turn-ended (exit 0). The wake kind set is unchanged.
@task_item n=6 part=touches
contract/viola-send/turn-running and decision/send-refusal-order: the turn-running refusal clears on an API-error turn end, and the next send is accepted with no pause or release.
@task_item n=7 part=touches
contract/list-human/columns, contract/viola-list-json/live-instance, contract/agent-run-status-polled-fields/status: the board status reads idle after an API-error turn end.
@task_item n=8 part=touches
decision/ledger-row-verify-probe, decision/version-gate-needs-all-relied-rows: StopFailure is registered with no ledger row and no verify probe. This is a named exception to the capability ledger gate, PROVISIONAL on the founder's fork F1.
@task_item n=9 part=touches
decision/fixture-named-by-hook-event, decision/fixtures-per-cli-version, decision/replay-every-cli-version-fixture, decision/recorded-hook-payloads-walked-at-run-time, decision/fake-agent-matches-recordings: StopFailure has no recorded fixture. Its tests plant a synthetic StopFailure payload beside a copy of the recorded set, and the recorded fixtures are unchanged.
@task_item n=10 part=touches
decision/closed-enums-need-decisions-log and decision/event-values-closed style rows: the hook_event enum gains stop-failure, and a new closed turn-end cause set appears.
@task_item n=11 part=touches
decision/diagnostics-schema-conformance, decision/fixtures-schema-checked, decision/fixtures-match-tolerant-schema: the diag-line, claude-fixture and fake-script schemas gain stop-failure / StopFailure (10 values).
@task_item n=12 part=touches
decision/hooks-fail-open, decision/hook-fails-open, decision/fail-open-spine-bound-at-unit-speed: stop-failure joins the fail-open cases (within 1.0 s, nothing written on a refused instance). The spine-bound case reads each row's own event name.
@task_item n=13 part=touches
decision/error-sanitization, decision/logs-carry-no-secrets, decision/no-secrets-in-logs, decision/secrets-never-in-logs: the API error text crosses into no event data, event line, snapshot, log line or detail file. Only the closed word api-error crosses into events.ndjson, wait --json and MCP wait (PROVISIONAL, F2).
@task_item n=14 part=touches
decision/hook-unknown-fields-tolerated: the StopFailure payload's error_details is never declared. A non-string error drifts as path error, expected string, in the drift report only.
@task_item n=15 part=touches
decision/unknown-events-counted-skipped, decision/tolerant-event-reader, decision/defensive-log-reader: the event log reader's unknown-field count reads the turn-ended data keys, so cause is a known key.
@task_item n=16 part=touches
contract/fake-agent-events/session-start and contract/fake-agent-hooks/command-spawn, contract/fake-agent-hooks/stdin-payload: the fake agent registers StopFailure. It sets the registry row idle on StopFailure as on Stop and has no new option or env read.
@task_item n=17 part=touches
decision/one-negative-per-security-control, decision/bounded-remove-the-guard-witness: each new guard (registration, text drop, cause re-validation) has a one-shot remove-the-guard control. All five hold.
@task_item n=18 part=touches
contract/perf-budgets/spine-hooks: stop-failure is a new hook verb on Stop's timeout tier. Whether it falls under a perf budget is for the picker to read.
@task_item n=19 part=touches
decision/ci-stamped-home-kill-line, decision/verify-driven-tests-20s-kill, decision/root-wait-bound: measured that a stamped-home run-start test reaches the 20 s CI kill. Its extra time sits in the stamped-home fixture's viola verify, which has an unbounded wait inside the test's measured time. The red is open and nothing changed.
@task_item n=20 part=touches
decision/rstest-case-tables: a shared case table that pinned one event's name for every row broke when another event's row joined it. Each row now reads its name from its own arguments.
@task_fact n=1 slug=integration-test-strategy
fact: The turn-ended cause is a closed set, today api-error alone. The wrapper does not append a hook turn-ended whose cause is outside it. A turn-ended with cause absent is appended as before.
@task_fact n=2 slug=e2e-test-strategy
fact: An API-error turn end's error text reaches no events.ndjson line, snapshot.json or diagnostics file, held by an absence probe that plants a value in the hook payload.
@task_fact n=3 slug=6-e2e-test-strategy
fact: An API-error turn end clears the running turn with no pause or release. A parked wait returns turn-ended, the next send exits 0, and the board reads idle.
@task_fact n=4 slug=test-data-fixtures
fact: StopFailure is registered with no recorded fixture, no ledger row and no verify probe, PROVISIONAL (F1). The 2.1.325 recorded fixture set owns this exception, and until then its tests use a synthetic StopFailure payload.
@task_fact n=5 slug=test-data-fixtures
fact: A fake-agent fixtures root for a case that sends after the first turn starts as a copy of the recorded set. Synthetic payloads alone hold no screen rows, so the send reads input-not-ready.
@task_fact n=6 slug=unit-test-strategy
fact: The CLI's API error words are classified into nine classes inside the agent adapter alone. No class reaches event data, an event, a snapshot or a log line, and a unit table maps every CLI error word.
@task_item n=1 part=untouched
MCP tool set, tools/list, MCP invalid-params fields and MCP reply screens: no MCP tool, parameter or result shape changed beyond turn-ended data passing through.
@task_item n=2 part=untouched
instance-unreachable causes, exit 21 details and the unreachable log cause: that closed set is unchanged. It only shares the word cause with the new turn-end cause.
@task_item n=3 part=untouched
revive, run start refusals, start exit 1 causes, heartbeat staleness and the instance snapshot fields: no start or revive path changed.
@task_item n=4 part=untouched
dialog decision bodies, dialog holds, answer refusal order and link or unlink methods: StopFailure is not a dialog and records no link.
@task_item n=5 part=untouched
harness CLI, agent-run status, boot, cleanup, gate, pre-push and the CI workflow: no harness verb, option, gate or workflow changed.
@task_item n=6 part=untouched
coverage floors, mutation gate thresholds, perf budgets' numbers and supply-chain gates: no threshold, floor or tool pin moved, and no mutation run was made.
@task_item n=7 part=untouched
PTY passthrough, screen signatures, socket directory and strict-modes checks, and peer-credential checks: none was reached.
