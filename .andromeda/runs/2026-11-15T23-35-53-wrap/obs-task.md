@task_prose part=session
The session registered the CLI's StopFailure hook as the tenth hook event, stop-failure, mapped to the turn-ended event kind; a turn the CLI ends on an API error now clears the running turn, the instance reads idle and the next send is accepted, with no pause or release.
The turn-ended event's data gains a third key, cause, holding the closed word api-error on an API-error end only and absent on a Stop; the cause set is a new closed one-member set in viola-core beside the exit-21 cause set.
The StopFailure payload's error text, error details and assistant message are never copied into data, an event line, the snapshot, a process log line or a detail file; a non-string error is a drift, written to the drift report only.
The CLI's thirteen error words are classified into nine classes inside the Claude agent crate and no class reaches data, an event, a snapshot or a log line (founder fork F2, PROVISIONAL); StopFailure has no capability-ledger row and no verify probe (fork F1, PROVISIONAL).
The wrapper re-validates a hook turn-ended: appended only when cause is absent or a word of the closed set; the diag-line schema's hook_event enum, the fixture and fake-script event enums and the fake agent's registered events each grow from 9 to 10.
Measured: every gate entry green locally; CI on the pushed head read red on one start-path test timing out at the 20 s kill, ruled not this chunk's and left open on another owner.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: viola-core, the shared pure core: the closed event kinds and each kind's data keys, the closed cause-word sets
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the Claude agent crate's hook normaliser: the closed set of hook events, the hook-to-kind map, the payload parse that builds an event's data and its drift report
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger of the Claude agent crate: the hook event names the ledger reads and stamps
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the capability ledger's stamp test cases
@task_file n=5 path=crates/viola-agent-claude/src/lib.rs
what: the Claude agent crate's root: its normalise boundary and unit tests
@task_file n=6 path=plugin/hooks/hooks.json
what: the hook registration of the viola plugin: each registered claude hook event mapped to its viola hook command with its timeout
@task_file n=7 path=src/cmd/run.rs
what: viola run, the wrapper: the hook.event channel method that re-validates a hook event and appends it to the event stream
@task_file n=8 path=src/run/send/slot.rs
what: the wrapper's send slot: the running-turn gate that refuses a send with turn-running until the turn ends
@task_file n=9 path=src/bin/viola-fake-agent.rs
what: the fake agent: its registered hook events and the registry row it marks idle at a turn end
@task_file n=10 path=schemas/diag-line.v1.json
what: the diag-line v1 schema: the hook_event enum of the hook-invoked and hook-decision lines
@task_file n=11 path=schemas/claude-fixture.v1.json
what: the recorded Claude fixture schema: its hook event enum
@task_file n=12 path=schemas/fake-script.v1.json
what: the fake agent's script schema: its hook event enum
@task_file n=13 path=tests/hook_events.rs
what: the every-event hook witness and the absence probe that the error text reaches no event line, snapshot or diagnostics file
@task_file n=14 path=tests/cli_wheel.rs
what: the wheel and wait end-to-end tests: a turn end clears the turn with no pause or release, a parked wait returns turn-ended
@task_file n=15 path=tests/cli_list.rs
what: the viola list tests: the board's STATUS reads idle after a turn end
@task_file n=16 path=tests/hook_fail_open.rs
what: the hook fail-open tests: each hook event exits 0 within its budget and writes nothing on a refused instance
@task_file n=17 path=tests/security_negatives_strict_modes.rs
what: the strict-modes security negatives over every registered hook command
@task_file n=18 path=crates/viola-state/tests/state_events.rs
what: the state crate's event-log tests: the unknown-field count over each kind's data keys
@task_item n=1 part=touches
The hook-invoked and hook-decision additive fields contract: hook_event is one of nine closed hook names, now ten with stop-failure.
@task_item n=2 part=touches
The closed event values decision: a new closed value stop-failure joins hook_event and needs its Decisions Log record.
@task_item n=3 part=touches
The diag-line schema additive-only rules and the schema-matches-event-enum rule: the hook_event enum gained one value, additively.
@task_item n=4 part=touches
The event stream line contract and the event line shape: the turn-ended event's data holds three keys now, cause added on an API-error end only.
@task_item n=5 part=touches
The rules that the assistant message and last_assistant_message are never logged: the StopFailure payload's assistant message, error text and error details are dropped at the parse and reach no line.
@task_item n=6 part=touches
The rules that event payloads stay out of process logs and content stays only in the instance detail file: the API-error text reaches neither a process log nor a detail file.
@task_item n=7 part=touches
The drift-report rules (drift reports go to diagnostics only, in the detail file only): a non-string error in a StopFailure payload is reported as a drift there and nowhere else.
@task_item n=8 part=touches
The hook fail-open rules (hook fails open silently, exits 0, silent without a trusted instance, never writes stderr): they now rule over the stop-failure hook too, measured to fail open within 1.0 s and write nothing on a refused instance.
@task_item n=9 part=touches
The spine hook budget and its perf gate: the stop-failure hook is registered at the Stop hook's tier with timeout 5, and joins the hooks those budgets cover.
@task_item n=10 part=touches
The hook stdout contract for viola hook and the stdout-by-role hook row: a new hook command, viola hook stop-failure.
@task_item n=11 part=touches
The corr-by-event and process-log corr rows for hook-invoked and hook-decision, and the dialog log fields hook-lines row: their lines now also carry hook_event stop-failure.
@task_item n=12 part=touches
The closed protocol codes and peer replies screened as code words rules: the wrapper re-validates a hook turn-ended's cause against a closed set and drops any other value.
@task_item n=13 part=touches
The rule that viola-core emits no telemetry: viola-core gained a closed cause set, with no logging.
@task_item n=14 part=touches
The derived metrics grouped by closed enums rules: the hook_event grouping enum has one more value.
@task_item n=15 part=touches
The wrapper send-refused detail turn-running contract and the wheel path pause-and-release events: an API-error end clears the turn with no pause or release, and a send before the end reads turn-running.
@task_item n=16 part=touches
The wait log fields channel-response outcome and the MCP wait tool: a parked wait returns turn-ended, its data now possibly holding the cause word, which crosses into wait --json and MCP wait (PROVISIONAL, fork F2).
@task_item n=17 part=touches
The rule that wrapped claude internals are not a contract and the fixtures-not-telemetry rules: the StopFailure name, error words and payload keys are a static read of installed CLI bundles, no recorded fixture added or changed.
@task_item n=18 part=touches
The capability ledger rules (ledger contents never logged, unreadable ledger means unverified): StopFailure is registered with no ledger row and no verify probe, a PROVISIONAL exception owned elsewhere.
@task_item n=19 part=touches
The CI verify against the fake agent only rule: the fake agent's registered events gained StopFailure and it marks the instance idle on it as on Stop.
@task_fact n=1 slug=log-format-json-schema
fact: The turn-ended event line's data holds a cause key only on an API-error end, a closed code word (api-error), and no cause key on a Stop.
@task_fact n=2 slug=8-pii-scrubbing-compliance
fact: An API-error turn end's error text, error details and assistant message never reach an event line, the snapshot, a process log line or a detail file; only the closed cause word crosses.
@task_fact n=3 slug=pii-scrubbing-compliance
fact: The CLI's API-error class is computed inside the Claude agent crate and no class word reaches event data, an event, a snapshot or a log line (PROVISIONAL, fork F2).
@task_fact n=4 slug=error-capture-reporting
fact: The wrapper appends a hook turn-ended only when its cause is absent or a word of the closed turn-end cause set; any other value is not appended.
@task_item n=1 part=untouched
The exit-21 unreachable cause words and their detail codes: the new turn-end cause set sits beside them and shares none of their words.
@task_item n=2 part=untouched
Telemetry export, OTel, error reporters, network egress and their absences: nothing exported or reported.
@task_item n=3 part=untouched
Panic capture, panic lines and panic outcome by role: no panic path changed.
@task_item n=4 part=untouched
MCP stdin framing, MAX_FRAME, MCP frame tool refusals and the MCP server identity.
@task_item n=5 part=untouched
Span names, span kinds and span attributes for run start, send, wait and wheel: no span added or renamed.
@task_item n=6 part=untouched
Diagnostics file locations, modes, home resolution, log init order and config.json keys: no config key added.
@task_item n=7 part=untouched
Heartbeat liveness, staleness thresholds, service identity and version constants.
@task_item n=8 part=untouched
Revive, link and unlink, verify stdout and spawn subjects, socket-directory and strict-modes refusal codes.
@task_item n=9 part=untouched
CI artifact retention, secret scan and upload rules: the CI red was a start-path timing outside this change, left open on another owner, and moved no gate threshold.
