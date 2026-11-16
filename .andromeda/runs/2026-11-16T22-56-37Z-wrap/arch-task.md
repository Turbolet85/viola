@task_prose part=session
The turn-ended event data gains a fourth key, scheduled_wakes: a count of the session's scheduled wakes beside background_tasks, one number and no content.
The count is the length of the Stop hook payload's session_crons array; null when the key is absent, null or not an array, and a non-array writes a drift entry to the instance detail file only, as background_tasks does.
An API-error (StopFailure) turn end writes scheduled_wakes null beside its null last_assistant_message and background_tasks and its cause.
The key reaches exactly the hops background_tasks reaches: the turn-ended line in events.ndjson, CLI wait --json and MCP wait structuredContent unchanged; not last, list, the snapshot or the replay.
wait wakes on a turn end as before, whatever its scheduled_wakes; no new IPC method, event kind, endpoint, env var, crate or protocol version bump (v unchanged, an additive field).
Measured: the recorded Stop fixtures carry an empty session_crons and now read 0; a live session reading 1 with a wake scheduled is unmeasured and carried to the standing live rig entry.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: edited — the agent-neutral event kind table in viola-core: each normalised event kind and its closed set of data keys, the TurnEnded row now four keys
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: edited — the Claude hook payload normalisation in viola-agent-claude: the tolerant parse of Stop and StopFailure payloads into turn-ended data, its drift entries, the one site where session_crons becomes scheduled_wakes
@task_file n=3 path=src/run/wait.rs
what: edited — the wrapper's wait method in the root bin: which event kinds wake a waiting driver; a test added that a turn end wakes whatever its scheduled_wakes
@task_file n=4 path=crates/viola-state/tests/state_events.rs
what: edited — the state crate's event reader tests: a written key the reader counts as known, not an unknown field
@task_file n=5 path=crates/viola-mcp/tests/handlers.rs
what: edited — the MCP server's tool handler tests: MCP wait carries the turn-ended data in structuredContent unchanged
@task_file n=6 path=tests/hook_events.rs
what: edited — end-to-end hook event tests against the fake agent: a booted Stop carrying scheduled wakes writes their count and no wake content to any home file
@task_file n=7 path=tests/cli_wait_last.rs
what: edited — end-to-end tests of the CLI wait and last verbs: wait --json carries the scheduled-wake count of a turn end
@task_file n=8 path=tests/cli_wheel.rs
what: edited — end-to-end tests of the wheel through the CLI: one line, the turn-ended data pin now holding the fourth key
@task_item n=1 part=touches
The contract row event data turn-ended last_assistant_message: it says the data carries no scheduled-wake count and lists data as last_assistant_message, background_tasks, cause?; now false and stale, the data is last_assistant_message, background_tasks, scheduled_wakes, cause?.
@task_item n=2 part=touches
The contract row event data turn-ended background_tasks: a sibling item for scheduled_wakes is owed beside it (same hops, same null rules, same drift entry).
@task_item n=3 part=touches
The contract row channel methods wait wake kinds (and channel wait method wake kinds): a turn end wakes whatever its background_tasks or scheduled_wakes; the wake rule itself is unchanged but the row's wording names one key only.
@task_item n=4 part=touches
The contract row Hook to kind map StopFailure: its enumeration of the null data on an API-error turn end gains scheduled_wakes null.
@task_item n=5 part=touches
The contract row Hook to kind map Stop: the Stop payload now also reads session_crons, by length alone, into turn-ended data.
@task_item n=6 part=touches
The contract row normalised event kinds turn-ended: its data key set moved from three keys to four, no new kind.
@task_item n=7 part=touches
Rules over the agent-neutral event kind table and its data keys (event kinds added only in viola-core, the unknown-field count of the event reader): a key was added to an existing kind there, not a new kind.
@task_item n=8 part=touches
Rules over the Claude hook payload parse: external payloads parsed tolerantly, unparseable external values become unknown, missing and null read the same, Claude payload shapes in one crate, upstream text untrusted and content-bearing detail only in the instance; the new session_crons field follows each (null when absent or not an array, a drift entry in the instance detail file, no element read).
@task_item n=9 part=touches
Rules over what reaches the MCP wait and CLI wait --json documents, and the MCP tools wait contract item: the turn-ended data they pass through now holds scheduled_wakes.
@task_item n=10 part=touches
The protocol version bump rule and versioned-tolerant own formats: the change is an additive data field with v unchanged, which those rows must still admit.
@task_item n=11 part=touches
JSON field names in snake_case: scheduled_wakes is a new wire key under that convention, the CLI's session_crons name kept out of viola's own data.
@task_fact n=1 slug=standard-contracts
fact: turn-ended data carries scheduled_wakes, the count of the session's scheduled wakes (the Stop payload's session_crons length alone, no element read), null when the key is absent, null or not an array, a non-array writing a drift entry to the instance detail file only, null on an API-error turn end; it reaches the events.ndjson line, CLI wait --json and MCP wait only, not last, list, the snapshot or the replay.
@task_fact n=2 slug=standard-contracts
fact: A turn end wakes wait whatever its scheduled_wakes, as whatever its background_tasks; a scheduled wake is counted, never a wake condition.
@task_item n=1 part=untouched
The wheel: claim, release, human take, interrupt and the byte-source classifier are unchanged.
@task_item n=2 part=untouched
Send and its confirmation: bracketed paste, readiness gate, refusals, the not-delivered details and confirm windows are unchanged.
@task_item n=3 part=untouched
Dialogs: the dialog hold, answer, permission, plan and question data and the pending-dialog rules are unchanged.
@task_item n=4 part=untouched
The turn-end cause set and the unreachable cause set: no cause word added or removed.
@task_item n=5 part=untouched
CLI exit codes, refusal reasons, the fault document and the human output lines: no code, detail word or human line changed.
@task_item n=6 part=untouched
The board and list payload, the instance snapshot fields and last: the new key does not reach them.
@task_item n=7 part=untouched
revive, verify, the capability ledger and its stamps: no ledger row added; how a harness prompt is told from a typed one is not changed and its classification step is not placed.
@task_item n=8 part=untouched
Build system, CI/CD, supply chain, crate dependency direction and deployment: no dependency, crate, gate, workflow or floor moved.
@task_item n=9 part=untouched
Occupied resources and names: no binary, command, crate, env var, socket, port or MCP tool name taken or freed.
