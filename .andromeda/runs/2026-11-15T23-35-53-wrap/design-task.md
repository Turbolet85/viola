@task_prose part=session
The session made a turn that the Claude CLI ends on an API error end like any other turn: the wrapper logs one turn-ended event, the running turn clears with no pause or release, the instance goes back to idle and the next send is accepted.
It registered one more CLI hook, StopFailure, run as viola hook stop-failure. That takes the registered hook events from nine to ten, and the hook stays fail-open like the others.
The turn-ended event's data gains a third key, cause, set to the closed code word api-error on an API-error end and absent on a normal Stop. The CLI's error text and error details are never copied into an event line, a snapshot, a log line or a detail file.
The wrapper appends a turn-ended event only when cause is absent or is a word from the closed set. The CLI's error words are sorted into nine classes inside the agent adapter, and no class reaches any output.
It measured that the one CI timeout on the pushed head came from a fixture's verify step and not from this change. That red stays open with a named owner, and no output wording or exit code changed.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the shared core vocabulary, meaning the event kinds with each kind's data keys and the closed cause sets. It gains the closed turn-end cause set (api-error), which sits beside the unreachable causes.
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the hook handler that maps each CLI hook to an event kind and builds its data. It gains stop-failure mapped to turn-ended, drops the error text, and sorts the CLI error word into nine internal classes.
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger's hook event names, which gain StopFailure's name.
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's stamps test cases, which gain the new event.
@task_file n=5 path=crates/viola-agent-claude/src/lib.rs
what: the Claude agent adapter's normalisation of hook events into event lines.
@task_file n=6 path=plugin/hooks/hooks.json
what: the plugin's hook registration list. It gains StopFailure, which runs viola hook stop-failure with Stop's timeout, for ten entries in all.
@task_file n=7 path=src/cmd/run.rs
what: the viola run wrapper's handling of hook.event frames. It checks the turn-ended cause again against the closed set before it appends the event.
@task_file n=8 path=src/run/send/slot.rs
what: the send slot, which clears the running turn on a turn end so the next send gets past the turn-running refusal.
@task_file n=9 path=src/bin/viola-fake-agent.rs
what: the fake agent used by tests. Its registered events gain StopFailure (ten in all), and it marks the instance idle on StopFailure as it does on Stop.
@task_file n=10 path=schemas/diag-line.v1.json
what: the diagnostics log line schema. Its closed hook_event values gain stop-failure, for ten values.
@task_file n=11 path=schemas/claude-fixture.v1.json
what: the recorded CLI fixture schema. Its event enum gains StopFailure.
@task_file n=12 path=schemas/fake-script.v1.json
what: the fake-agent script schema. Its event enum gains StopFailure.
@task_file n=13 path=tests/hook_events.rs
what: the every-event hook witness test plus a probe that checks the error text never appears in any event line, snapshot or detail file.
@task_file n=14 path=tests/cli_wheel.rs
what: the end-to-end test of the turn wheel. An API-error end clears the turn with no pause or release, a parked wait returns turn-ended, and the next send is accepted.
@task_file n=15 path=tests/cli_list.rs
what: the viola list board test. STATUS reads idle after an API-error turn end, and a second send is accepted.
@task_file n=16 path=tests/hook_fail_open.rs
what: the hook verb's fail-open test table, which gains a stop-failure row.
@task_file n=17 path=tests/security_negatives_strict_modes.rs
what: the strict-modes security negatives. Their list of hook verbs grows to ten.
@task_file n=18 path=crates/viola-state/tests/state_events.rs
what: the state crate's event log test. It now counts turn-ended's cause key as a known field.
@task_item n=1 part=touches
The rule that turn-ended never fills the readback box gains a second kind of turn end, the API-error end. That end also carries no message text (last_assistant_message is null), so nothing may enter the readback box.
@task_item n=2 part=touches
The board's STATUS contracts (the flight-progress strip's STATUS and viola list board STATUS) now cover an API-error turn end, which reads as idle with no separate status word, colour or mark.
@task_item n=3 part=touches
The viola wait event form returns turn-ended with exit 0 after an API-error end. Under --json, the turn-ended data can now hold the cause word api-error.
@task_item n=4 part=touches
The contracts of the hook verb (its exit code and stderr) and the hook row in the CLI colour decision order now cover one more hook, stop-failure. It is fail-open within one second and writes nothing on a refused instance.
@task_item n=5 part=touches
The name of the hook command now takes a tenth event argument, stop-failure.
@task_item n=6 part=touches
The not-delivered turn-running refusal hint covers a send made while a turn runs before an API-error end (one refusal). After that end the turn has cleared and the next send is accepted.
@task_item n=7 part=touches
The pause and release rule (pause and release as control transfer) now covers a turn end that clears the running turn without a pause or a release.
@task_item n=8 part=touches
The rules that no internals appear in errors and hints never quote text now also cover the CLI's API error text, which is dropped when the payload is parsed and never shown or logged. Only the closed word api-error crosses into output.
@task_item n=9 part=touches
The peer-reply screening and closed code-word conventions now have a counterpart in the wrapper: a turn-ended cause outside the closed set is not appended. That check is a code-word check on channel input.
@task_item n=10 part=touches
Any row whose cell holds the count of registered hook events, nine, now reads ten.
@task_fact n=1 slug=surface-cli
fact: A turn that the CLI ends on an API error shows on the board and in viola wait like any turn end. STATUS reads idle, wait reads turn-ended, and no distinct status, colour or hint marks it. The only trace is the closed cause word api-error in --json event data, and the CLI's error text is shown nowhere.
@task_item n=1 part=untouched
The exit 21 causes and their --json cause words share the word cause but are the unreachable-instance causes. The session changed none of them, and no exit code changed.
@task_item n=2 part=untouched
Colour, typography, spacing, iconography, glyph and separator rows: no output wording, colour span, glyph or column changed.
@task_item n=3 part=untouched
The revive, link, unlink, verify, last and plugin install output and refusal contracts: none of these verbs changed.
@task_item n=4 part=untouched
Brand identity and the self-validation checks: the session added no visual element and no motion.
@task_item n=5 part=untouched
The send readback box and send mirror wording: no send output form changed, only when a send is accepted after a turn end.
@task_item n=6 part=untouched
Web surface and rendered-page absences: they stand unchanged.
