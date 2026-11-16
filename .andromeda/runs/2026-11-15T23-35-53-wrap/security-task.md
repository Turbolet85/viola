@task_prose part=session
The session registered the Claude CLI's StopFailure hook as a tenth hook event, stop-failure, mapped to the turn-ended event kind; registered hook events moved from nine to ten.
A turn the CLI ends on an API error now logs turn-ended with data exactly last_assistant_message null, background_tasks null and cause api-error; the instance reads idle and the next send is accepted, with no pause or release.
The payload's error text, error_details and last_assistant_message are never copied into event data, a log line, the snapshot or a diagnostics file; a non-string error reaches the drift report only.
viola-core gained a closed one-member turn-end cause set (api-error); the wrapper's hook.event append re-checks a turn-ended cause against it, appending an absent cause as before and refusing any other value.
The CLI error word is classified into nine closed classes inside the Claude adapter crate only; no class is written anywhere outside it.
No new IPC method, endpoint, socket, port, environment variable, crate, dependency or event kind; turn-ended data keys moved from two to three.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the core crate's shared closed types and validators, the event kinds and their data keys, now with the closed turn-end cause set beside the unreachable causes
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the Claude adapter's hook payload parsing, the closed set of registered hook events and their mapping to event kinds and event data, now with stop-failure and the API-error class table
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: the Claude adapter's capability ledger, the hook event names it maps
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the capability ledger's stamp test cases
@task_file n=5 path=crates/viola-agent-claude/src/lib.rs
what: the Claude adapter crate's normalisation of hook payloads into events, and its unit tests
@task_file n=6 path=plugin/hooks/hooks.json
what: the plugin's hook registration, each hook event an exec-form command on the absolute pinned viola path with its timeout, now ten entries
@task_file n=7 path=src/cmd/run.rs
what: the wrapper's channel method handlers, hook.event's server-side re-validation before an event line is appended
@task_file n=8 path=src/run/send/slot.rs
what: the wrapper's running-turn slot that a turn end clears so the next send is accepted
@task_file n=9 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary under the fake-agent feature, its closed list of registered hook events and its registry status marking
@task_file n=10 path=schemas/diag-line.v1.json
what: the diagnostics line schema, the closed hook_event values of hook-invoked and hook-decision lines
@task_file n=11 path=schemas/claude-fixture.v1.json
what: the recorded Claude fixture schema, its closed hook event enum
@task_file n=12 path=schemas/fake-script.v1.json
what: the fake agent script schema, its closed hook event enum
@task_file n=13 path=tests/hook_events.rs
what: the every-event hook witness and the absence probe that the payload's error text reaches no event line, snapshot or diagnostics file
@task_file n=14 path=tests/cli_wheel.rs
what: the wheel and send end-to-end tests, a turn cleared with no pause or release and the next send accepted
@task_file n=15 path=tests/cli_list.rs
what: the viola list board tests, an instance status after a turn end
@task_file n=16 path=tests/hook_fail_open.rs
what: the hook fail-open tests, a refused or unreachable hook exiting silently within its bound
@task_file n=17 path=tests/security_negatives_strict_modes.rs
what: the strict-modes security negative tests over every registered hook event
@task_file n=18 path=crates/viola-state/tests/state_events.rs
what: the event log reader tests, the unknown-field count over each event kind's data keys
@task_item n=1 part=touches
decision/hook-unknown-event-silent-exit says a closed set of nine hook events; the set is now ten, with stop-failure.
@task_item n=2 part=touches
contract/channel-frames/hook-event-revalidation: the wrapper's hook.event re-validation now also re-checks a turn-ended cause against the closed turn-end cause set; absent is appended, any other value is not appended.
@task_item n=3 part=touches
contract/wrapper-channel-methods/hook-event: hook.event now carries turn-ended with a cause word on an API-error end; no new method.
@task_item n=4 part=touches
decision/server-side-revalidation and decision/wrapper-checks-authoritatively: the wrapper re-validates one more field, the turn-ended cause.
@task_item n=5 part=touches
decision/hook-security-fields-closed, decision/hook-security-fields-closed-enums and decision/closed-types-for-protocol-fields: the turn-end cause is a new closed enum in the core crate, and the hook event set gains a member.
@task_item n=6 part=touches
decision/shared-validators-in-core: the turn-end cause set and its from_name check live in the core crate and are read by both the adapter and the wrapper.
@task_item n=7 part=touches
decision/hook-payload-is-content, decision/upstream-text-is-content and decision/upstream-text-is-never-a-command: the StopFailure payload's error text, error_details and last assistant message are dropped at the parse; only a closed word crosses.
@task_item n=8 part=touches
decision/tolerant-hook-parsing and decision/drift-reports-diagnostics-only: a non-string error field drifts to the drift report only, the hook still yields its event.
@task_item n=9 part=touches
decision/data-classes and decision/user-content-only-to-diagnostics: a new value, the cause word api-error, crosses into events.ndjson, wait --json and MCP wait (boundary widening, PROVISIONAL on founder fork F2); no user content or error text crosses, not even to diagnostics.
@task_item n=10 part=touches
decision/stop-background-tasks-count-only and decision/question-option-text-crosses-boundary: the turn-ended data shape these rules govern gains a third key, cause, null for the other two on an API-error end.
@task_item n=11 part=touches
contract/mcp-tools/wait and contract/wrapper-channel-methods/wait: a parked wait now returns turn-ended exit 0 on an API-error end, its event data holding the cause word.
@task_item n=12 part=touches
decision/hook-fails-open, decision/hooks-fail-open, decision/hook-silent-exit-zero and decision/hook-silent-and-exit-zero: the new stop-failure hook joins the fail-open, silent, exit-zero path within its 1.0 s bound, nothing written on a refused instance.
@task_item n=13 part=touches
decision/hook-strict-check-first and decision/hook-instance-check-first: the stop-failure hook runs the same strict-modes and instance checks first; the strict-modes negatives now cover ten hook events.
@task_item n=14 part=touches
decision/absolute-viola-path-in-commands, decision/exec-commands-absolute-pinned-path, decision/rewrite-plugin-files and decision/plugin-files-rewritten-each-start: a new hook registration entry, exec form on the pinned path with timeout 5, is written into the plugin files.
@task_item n=15 part=touches
contract/hook-stdin/oversize and contract/hook-stdin/not-one-object: the stop-failure payload goes through the same bounded stdin read and object check.
@task_item n=16 part=touches
decision/reads-capped-at-max-frame and decision/max-frame-cap: the new hook stdin read keeps the MAX_FRAME cap.
@task_item n=17 part=touches
decision/unverified-cli-gate: StopFailure is registered with no capability ledger row and no verify probe, a named exception (PROVISIONAL on founder fork F1), owned by the 2.1.325 recorded fixture set.
@task_item n=18 part=touches
decision/test-seams-fake-agent-only and decision/test-seams-compiled-out: the fake agent's registered event list grows to ten; no new option, mode or environment read.
@task_item n=19 part=touches
decision/new-env-read-needs-log-entry and decision/new-listener-needs-log-entry: no new environment read, listener or channel method was added, so no log entry is owed.
@task_item n=20 part=touches
contract/hook-decision-detail rows and the hook-invoked and hook-decision diagnostics lines: hook_event gains the closed value stop-failure, ten values.
@task_fact n=1 slug=input-validation
fact: The wrapper appends a hook turn-ended event only when its cause is absent or a word of the closed turn-end cause set (api-error alone); any other cause value is not appended.
@task_fact n=2 slug=data-protection
fact: On an API-error turn end, the StopFailure payload's error text, error_details and last_assistant_message are dropped at the parse; turn-ended data holds only null for last_assistant_message and background_tasks and the closed word api-error as cause, which crosses into events.ndjson, wait --json and MCP wait (boundary widening, PROVISIONAL on founder fork F2).
@task_fact n=3 slug=data-protection
fact: The CLI's StopFailure error word is classified into nine closed API-error classes inside the Claude adapter crate only; no class is written to event data, an event line, the snapshot or a log line (PROVISIONAL on founder fork F2).
@task_fact n=4 slug=authentication-authorization
fact: StopFailure is the named exception to the unverified-CLI capability gate: registered with no capability ledger row and no verify probe, owned by the 2.1.325 recorded fixture set, PROVISIONAL on founder fork F1.
@task_item n=1 part=untouched
Dependency security: no crate, dependency, feature, toolchain pin, licence or advisory changed; rmcp and serde_json rules stand.
@task_item n=2 part=untouched
Secret management: no credential, environment variable strip, identity floor or canary scan class changed.
@task_item n=3 part=untouched
Socket, peer euid, server verification and home strict-modes refusal contracts: no new socket, endpoint, channel method or refusal code.
@task_item n=4 part=untouched
Error handling exit codes and the instance-unreachable cause set: the unreachable causes and exit 21 are unchanged; the new closed cause set sits beside them and shares only the word cause.
@task_item n=5 part=untouched
MCP stdin, send, answer, pause, release, link and unlink frames: no parameter, refusal word or paste text rule changed.
@task_item n=6 part=untouched
Verify, fixture recording and scrubbing rows: no recorded fixture was added or changed and no verify probe was added.
@task_item n=7 part=untouched
CI workflows, pre-push gate, release build and mutation testing rows: no workflow, gate, harness verb or release stage changed.
@task_item n=8 part=untouched
Absences: no network call, public listener, database, secret store or account surface was added.
