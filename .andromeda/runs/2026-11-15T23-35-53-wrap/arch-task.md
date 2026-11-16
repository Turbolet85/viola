@task_prose part=session
The session registers the Claude Code StopFailure hook as a tenth hook event: viola hook stop-failure maps to the turn-ended kind, not a dialog, so a turn the CLI ends on an API error clears the running turn, the instance reads idle and the next send is accepted, with no pause or release.
The turn-ended event's data gains a third key, cause, written only on an API-error end and holding one word of a new closed set TurnEndCause in viola-core (one member, api-error); on a Stop end cause is absent and the other two keys are null on an API-error end.
The upstream error text, error_details and last_assistant_message of a StopFailure payload are never copied into data, an event line, the snapshot or a diagnostics file; the CLI error word is classified into nine classes inside viola-agent-claude only, and no class reaches data.
The wrapper re-validates hook.event: a turn-ended carrying a cause is appended only when cause is a word TurnEndCause accepts; absent cause appends as before; any other value is not appended.
Registered hook events, the hook-to-kind map, the plugin hooks list, the diag-line hook_event values and the fake agent's registered events all move 9 to 10; StopFailure is registered with no capability-ledger row and no verify probe (PROVISIONAL, founder fork F1); the cause word crossing into events.ndjson, wait --json and MCP wait is PROVISIONAL (fork F2).
No new IPC method, event kind, crate, dependency, env var, config key, socket or port; nothing retired. CI on the pushed head read red on one timeout in a test outside this change, ruled not this chunk's and left open.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: viola-core, the agent-neutral core: the normalised event kinds and their data keys, the closed cause sets (UnreachableCause, now TurnEndCause) — edited
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: viola-agent-claude's hook layer: the Claude hook events, the hook-to-kind map and the normalisation of a hook payload into event data — edited
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: viola-agent-claude's per-behaviour CLI capability ledger and its event names — edited
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the capability ledger's stamps test cases — edited
@task_file n=5 path=crates/viola-agent-claude/src/lib.rs
what: viola-agent-claude's crate root, the Claude-specific payload shapes and normalisation entry — edited
@task_file n=6 path=plugin/hooks/hooks.json
what: the hook registrations of the plugin compiled into the binary and loaded into wrapped sessions: exec-form command hooks with their tier timeouts — edited
@task_file n=7 path=src/cmd/run.rs
what: the wrapper (viola run): the channel hook.event method that re-validates and appends hook events to the log — edited
@task_file n=8 path=src/run/send/slot.rs
what: the wrapper's send slot: the turn-running rung that refuses a send mid-turn and clears on a turn end — edited
@task_file n=9 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary: its registered hook events and its registry status marks — edited
@task_file n=10 path=schemas/diag-line.v1.json
what: the diagnostics line schema, its closed hook_event values — edited
@task_file n=11 path=schemas/claude-fixture.v1.json
what: the recorded Claude fixture schema, its hook event enum — edited
@task_file n=12 path=schemas/fake-script.v1.json
what: the fake agent script schema, its hook event enum — edited
@task_file n=13 path=tests/hook_events.rs
what: the every-event hook witness test and the absence probe that no upstream text reaches an event line, snapshot or diagnostics file — edited
@task_file n=14 path=tests/cli_wheel.rs
what: the wheel and turn end-to-end tests over the CLI (send, wait, pause, release) — edited
@task_file n=15 path=tests/cli_list.rs
what: the board tests over viola list: status and liveness of each instance — edited
@task_file n=16 path=tests/hook_fail_open.rs
what: the hook fail-open tests: each hook's bound and fail-open behaviour on a refused or unreachable instance — edited
@task_file n=17 path=tests/security_negatives_strict_modes.rs
what: the security negative tests of strict modes over every registered hook — edited
@task_file n=18 path=crates/viola-state/tests/state_events.rs
what: viola-state's event reader tests: unknown-field counts per kind's data keys — edited
@task_item n=1 part=touches
The hook-to-kind map rows: a new hook StopFailure maps to turn-ended (not a dialog), joining Stop as a second source of turn-ended; no stop-failure row stands yet.
@task_item n=2 part=touches
contract/event-data/turn-ended-last-assistant-message and contract/event-data/turn-ended-background-tasks: turn-ended data now holds three keys, cause added on an API-error end, both other keys null there; absent cause on a Stop end.
@task_item n=3 part=touches
contract/normalised-event-kinds/turn-ended: the kind now also arises from an API-error turn end; no new event kind is added.
@task_item n=4 part=touches
contract/hook-event/appended-line and contract/hook-event/kind (and contract/channel-methods/hook-event): the wrapper's hook.event append gains a re-validation of cause against the closed TurnEndCause set; any other cause value is not appended.
@task_item n=5 part=touches
decision/capability-ledger-gate, decision/cli-capability-ledger and decision/capability-ledger: a named exception, StopFailure registered with no ledger row and no verify probe, owned by the 2.1.325 recorded fixture set entry, PROVISIONAL on founder fork F1.
@task_item n=6 part=touches
decision/plugin-precedence-no-ledger-row: a second thing standing with no ledger row now exists beside it (the StopFailure registration).
@task_item n=7 part=touches
decision/event-kinds-agent-neutral and decision/core-depends-on-no-viola-crate: the new closed cause set lives in viola-core, agent-neutral, no serde, no Claude name; its readers are viola-agent-claude and the wrapper.
@task_item n=8 part=touches
decision/claude-shapes-in-one-crate, decision/claude-shapes-isolated and decision/agent-claude-pure: the StopFailure payload shape and the classification of the CLI error word into nine API-error classes stay inside viola-agent-claude; no class leaves it.
@task_item n=9 part=touches
decision/untrusted-upstream-text, decision/upstream-text-is-content and decision/sanitised-error-surfaces: the StopFailure error text, error_details and last_assistant_message are dropped at parse and reach no event line, snapshot or diagnostics file.
@task_item n=10 part=touches
decision/tolerant-external-payloads and decision/unparseable-external-values-unknown: a non-string error field on StopFailure is reported as drift only, never failing the hook.
@task_item n=11 part=touches
contract/unreachable-cause/cause-set: a sibling closed cause set (TurnEndCause) now stands beside UnreachableCause; the unreachable set itself is unchanged.
@task_item n=12 part=touches
decision/wire-enum-values-kebab-case and contract/code-word/format: api-error is a new wire word in event data, held to the code-word format.
@task_item n=13 part=touches
Hook tier rows (contract/hook-tiers/sync-spine, contract/hook-tiers/ordering-spine, decision/spine-deadline, decision/spine-hook-loses-event-not-turn): StopFailure joins at Stop's tier, timeout 5, fail-open within the spine bound.
@task_item n=14 part=touches
decision/exec-form-command-hooks, decision/exec-form-hooks, decision/hooks-and-mcp-never-search-path and decision/hooks-fail-open: the new registration is exec form on the pinned path and fails open like the others.
@task_item n=15 part=touches
Any row holding the count of registered hook events (nine): it moves to ten; the plugin hooks list, the diag-line hook_event values and the fake agent's registered events all hold ten.
@task_item n=16 part=touches
decision/hook-without-trusted-instance-exits-silently and decision/hook-exits-on-untrusted-instance: they now rule over a tenth hook, stop-failure, with nothing written on a refused instance.
@task_item n=17 part=touches
decision/send-never-queued, decision/every-send-confirmed and contract/send-refusal/order: an API-error turn end now clears the turn-running state so the next send is accepted, where before such a turn left the instance running.
@task_item n=18 part=touches
contract/channel-wait/wake-kinds, contract/channel-methods/wait-wake-kinds, contract/mcp-tools/wait and contract/channel-wait/result: a parked wait now wakes on an API-error turn end as turn-ended and its result carries the cause word (PROVISIONAL, fork F2).
@task_item n=19 part=touches
contract/board/status, contract/list-item/status and contract/list-payload/status: the board reads idle after an API-error turn end.
@task_item n=20 part=touches
decision/human-always-wins-wheel, decision/wheel-returns-only-through-release and contract/event-data/wheel: an API-error turn end moves no wheel, writes no pause and no release.
@task_item n=21 part=touches
decision/fake-agent-test-seams and name/binary/viola-fake-agent: the fake agent's registered events gain StopFailure (ten) and it marks idle on StopFailure as on Stop; no new option or env read.
@task_item n=22 part=touches
decision/reader-counts-skipped-lines: the event reader's unknown-field count reads turn-ended's data keys, now three.
@task_item n=23 part=touches
decision/codes-only-role-logs, decision/process-logs-hold-codes-only and contract/diagnostics-role-files/hook-name: hook-invoked and hook-decision lines now carry hook_event stop-failure; no payload text.
@task_item n=24 part=touches
name/command/hook and the name/other/hook-* names: the hook verb takes a new event argument, stop-failure.
@task_fact n=1 slug=conventions
fact: Hook to kind map: StopFailure (viola hook stop-failure) normalises to turn-ended, not a dialog, its data exactly last_assistant_message null, background_tasks null, cause api-error.
@task_fact n=2 slug=conventions
fact: TurnEndCause is a closed set in viola-core beside UnreachableCause, one member api-error, with no Claude-specific name; a turn-ended cause is one of its words.
@task_fact n=3 slug=standard-contracts
fact: turn-ended event data gains cause, present only on an API-error turn end (api-error), absent on a Stop end; it crosses into events.ndjson, wait --json and MCP wait (PROVISIONAL, founder fork F2).
@task_fact n=4 slug=standard-contracts
fact: The wrapper's hook.event appends a turn-ended carrying cause only when cause is a TurnEndCause word; cause absent appends as before; any other value is not appended.
@task_fact n=5 slug=standard-contracts
fact: A StopFailure payload's error text, error_details and last_assistant_message are never copied into event data, an event line, the snapshot or a diagnostics file.
@task_fact n=6 slug=crate-dependency-direction
fact: The CLI's StopFailure error word is classified into nine API-error classes only inside viola-agent-claude; normalisation does not use it and no class reaches event data, a snapshot or a log line.
@task_fact n=7 slug=cross-cutting-patterns
fact: Exception to the capability ledger gate: StopFailure is registered with no ledger row and no verify probe, owned by the 2.1.325 recorded fixture set entry (PROVISIONAL, founder fork F1).
@task_fact n=8 slug=occupied-resources
fact: The hook event name hook stop-failure is occupied (the tenth registered hook event).
@task_fact n=9 slug=standard-contracts
fact: An API-error turn end clears the running turn with no pause or release: the instance reads idle and the next send is accepted.
@task_item n=1 part=untouched
CLI exit codes and exit-21 documents: no code or process-exit detail added or moved.
@task_item n=2 part=untouched
viola revive, viola verify and their refusal codes, stamps envelope and verify probe behaviour: no change (no verify probe added).
@task_item n=3 part=untouched
Configuration (config.json keys, precedence, diagnostics_level) and environment variables: no key and no variable added.
@task_item n=4 part=untouched
Stack, dependencies, toolchain, cargo-deny, CI workflows, release and fuzzing: no dependency, crate or workflow change.
@task_item n=5 part=untouched
Endpoints, sockets, socket directory checks, peer admission and server verification: no new endpoint, socket or port.
@task_item n=6 part=untouched
The PTY seam, the vt100 screen model, readiness gate and screen signatures: untouched.
@task_item n=7 part=untouched
Link and unlink, the link set and their event lines: untouched.
@task_item n=8 part=untouched
absence/usage-budget-gate and absence/model-api-calls still hold: no rate-limit or budget state is kept, and nothing pauses or refuses a send on an API error class.
@task_item n=9 part=untouched
Snapshot envelope, atomic writes, append discipline and the events log's rotation rules: no format change beyond the turn-ended data key.
@task_item n=10 part=untouched
Dialog hold, pending dialog and the answer path: StopFailure is not a dialog hook.
@task_item n=11 part=untouched
The installed MCP-only plugin and its .mcp.json, and the Hyprland seam: untouched.
