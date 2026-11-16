@task_prose part=chunk
The Claude adapter reads the Stop hook payload's session_crons as a count only, its elements never kept or typed, and carries that one number beside background_tasks on the normalised turn-ended event.
The count reaches every hop background_tasks reaches today: the event's data in events.ndjson, CLI wait --json, MCP wait, and whatever further hop P3 measures (last, list).
A session with no scheduled wake reads none, by background_tasks's existing absent/null/non-array rule; no wake's content reaches any event, log line, snapshot, reply or fixture; wait's wake rule is unchanged.
P3 re-measures every hop background_tasks takes and the session_crons shape in the recorded Stop payloads, and closes one read-only block on whether the count lets a scheduled wake's prompt be told from a typed one.
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the Claude adapter's tolerant hook payload parse and its map from a Stop payload to the normalised turn-ended event — edits: the Stop payload's read of background_tasks, joined by session_crons as a count
@task_file n=2 path=crates/viola-core/src/lib.rs mark=edits
what: the normalised event kinds and their per-kind data, agent-neutral — edits: the turn-ended event data, beside background_tasks
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the run command's wrapper, where a hook's normalised turn-ended event is carried onto the appended event line
@task_file n=4 path=src/run/send/slot.rs mark=edits
what: the wrapper's send slot, which carries the turn-ended data a driver's send waits on
@task_file n=5 path=src/run/wait.rs mark=edits
what: the channel wait method and its result, the turn-ended data a woken wait returns
@task_file n=6 path=tests/cli_wait_last.rs mark=edits
what: the CLI wait and last checks over a turn end's data
@task_file n=7 path=tests/cli_wheel.rs mark=edits
what: the wheel checks over turn ends, which assert background_tasks today
@task_file n=8 path=tests/hook_events.rs mark=edits
what: the hook-to-kind map checks, a hook payload to the normalised event line
@task_file n=9 path=crates/viola-mcp/tests/handlers.rs mark=edits
what: the MCP tool handler checks, the wait tool's result among them
@task_file n=10 path=crates/viola-state/tests/state_events.rs mark=edits
what: the event-line read and snapshot replay checks over turn-ended data
@task_file n=11 path=fixtures/claude/2.1.283/Stop.default.json mark=re-reads
what: the recorded Stop hook payload whose session_crons shape P3 measures
@task_file n=12 path=fixtures/claude/2.1.287/Stop.default.json mark=re-reads
what: the recorded Stop hook payload whose session_crons shape P3 measures
@task_file n=13 path=fixtures/claude/2.1.288/Stop.default.json mark=re-reads
what: the recorded Stop hook payload whose session_crons shape P3 measures
@task_file n=14 path=fixtures/claude/2.1.302/Stop.default.json mark=re-reads
what: the recorded Stop hook payload whose session_crons shape P3 measures
@task_item n=1 part=needs
The turn-ended event's per-kind data gains one number, the count of scheduled wakes, beside background_tasks: the turn-ended kind, its data, and the Stop row of the hook-to-kind map.
@task_item n=2 part=needs
The Stop payload is an external payload parsed tolerantly: session_crons read as a count with each element ignored unread, an unparseable or non-array value read as unknown, never failing the hook.
@task_item n=3 part=needs
Claude-specific shapes stay in the agent crate: no Claude field name crosses into viola-core; the new data is added only on the normalised event in viola-core under its agent-neutral naming rule, and the root bin consumes only the normalised event.
@task_item n=4 part=needs
The normalised field's name and wire form follow the naming conventions: JSON field names in snake_case, standard Rust naming, absent optional fields omitted on write.
@task_item n=5 part=needs
A session with no scheduled wake reads none exactly as background_tasks reads none today: missing and null read the same, and the null-versus-0 reading follows background_tasks's existing rule.
@task_item n=6 part=needs
No wake's content (prompt, cron expression, id, schedule, any string) reaches an event, snapshot, process or diagnostics log line, reply or fixture: upstream text is content and untrusted, process logs hold codes only.
@task_item n=7 part=needs
The count reaches every hop background_tasks reaches: the appended event line in events.ndjson, the channel wait result, CLI wait --json, the MCP wait tool's result, and last or list where P3 measures background_tasks reaching them.
@task_item n=8 part=needs
wait's wake rule does not change: no wake kind, driver-relevant event, pending-dialog hold or completion decision of the channel wait method reads the new count.
@task_item n=9 part=needs
Mixed-version tolerance: viola's own formats stay versioned-tolerant, every reader skips the unknown field, an older binary reading the new event line or wait result keeps working; whether the added field moves the protocol version follows the protocol version bump rule.
@task_item n=10 part=needs
Whether reading the undocumented session_crons field needs a capability-ledger row with a verify probe, or is read-only telemetry that fails to none with no dependence, follows how background_tasks stands under the per-behaviour CLI capability ledger and the rule of no probe for a shape no send relies on.
@task_item n=11 part=needs
Snapshot replay recovers only log-derived fields: where the instance snapshot or replay carries turn-ended data today, the count follows background_tasks there and nowhere else.
@task_item n=12 part=needs
The directive's adjacent fact is placed, not built: P3 reads, without changing, how a submitted prompt's origin is classified (prompt-submitted origin, harness prompts classified by raw prefix, a human prompt confirming a wheel take, the human-typing refusal) and closes one block on what this count covers of a scheduled wake's prompt, what it leaves and the placement options.
@task_item n=13 part=needs
Proof runs on the fake agent and the recorded payloads only: no live claude start, no real CLI probe in CI; a claim needing a live reading gets its probe shaped and its count named at P3's end.
@task_item n=14 part=needs
New tests owe the directive's bounds: a test waiting on a channel or thread takes a bound, test sockets sit under the short /tmp root, a stamped home comes from the shared stamp's copy, each test runs in its own viola home.
@task_item n=15 part=needs
Each control names its own timeout on an entry filtered to its own cases with a control id of at most 16 characters, and holds only when those cases ran and failed with no timeout line; no mutation run.
@task_item n=16 part=needs
Acceptance of capability v2-35: a driver reading the turn end of a session with scheduled wakes sees how many, a session with none reads none, no wake's content reaches an event or log line, and wait wakes exactly as before.
@task_standing n=1
convention: each new test runs against the fake agent in CI on ubuntu, never a live claude
home: ci-cd-approach
@task_standing n=2
convention: each new test runs in its own isolated viola home, cleaned by its owner record
home: established-decisions · occupied-resources
@task_standing n=3
convention: each new test's channel socket sits under the short /tmp root, outside the product namespace, and each root test wait shares one bound
home: occupied-resources
@task_standing n=4
convention: each new recorded or hand-made fixture is scrubbed and hygiene-walked, and recorded only from a clean verify
home: occupied-resources · established-decisions
@task_standing n=5
convention: each new snapshot assertion runs in insta check mode, and an end-to-end run leaves zero panic lines
home: project-directory-structure
@task_standing n=6
convention: each new code path stays under the coverage floors and the formatting and clippy gate with print macros banned
home: ci-cd-approach · build-system
@task_item n=1 part=not_needed
The send path: bracketed-paste send, send confirmation, confirm windows, local-command echo, line-ending normalisation and send refusals; the chunk adds no send behaviour.
@task_item n=2 part=not_needed
Dialog holds and answers: permission, plan and question dialogs, the answer method, dialog expiry and the dialog hook's fail-open; a turn end's count holds no dialog.
@task_item n=3 part=not_needed
Start and revive: run start order, pinned exe copy, child program resolution, child argv and environment strip, revive refusals and their exit codes.
@task_item n=4 part=not_needed
Link, unlink, pause and release verbs and their link-set recording; the chunk touches none.
@task_item n=5 part=not_needed
The Hyprland compositor seam, window features and the planned terminal fleet console.
@task_item n=6 part=not_needed
Plugin install, the marketplace tree and the plugin's .mcp.json; no plugin content changes.
@task_item n=7 part=not_needed
The turn-end cause set and its words: the count is a separate field, the cause stays as it is.
@task_item n=8 part=not_needed
Session liveness, heartbeat and process liveness signals; the count says nothing of whether a session is alive.
@task_item n=9 part=not_needed
CLI and MCP exit codes, fault documents, instance-unreachable details and refusal-reason sets; the chunk adds no error path.
@task_item n=10 part=not_needed
config.json keys, configuration precedence and the colour environment; no setting governs the count.
@task_item n=11 part=not_needed
Release tooling, dist, licences and crates.io publishing; nothing is released by this chunk.
@task_item n=12 part=not_needed
The usage-budget absence and model API calls; a scheduled wake is counted, never gated on.
