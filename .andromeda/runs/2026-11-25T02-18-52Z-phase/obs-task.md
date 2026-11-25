@task_prose part=chunk
Changes the form of the wrapper's window move so a placed window reaches its workspace while the compositor's active workspace stays unchanged (the operator's F4 «no focus change», v2-07).
The form is measured from the compositor's own help and sent through the one Hyprland seam's closed request set; every caller of the move (viola open, viola up, viola revive under --origin) takes the same answer or a named difference.
The seam's tap, fake and recorded move fixture follow the form; the fixture is re-recorded only in one operator-granted, operator-fired live leg.
Adds to v2-07's acceptance a predicate a gate entry reads: the active workspace read before a placing open equals the one read after; v2-07 is claimed and proved again, its live half gated.
Folded item: says where the automatic verify of a new CLI runs at a start and what it stamps when its own preconditions fail (an untrusted folder, a modal, a red it caused), fixes that by a forked form, with its own acceptance and a witness that cannot pass vacuously.
@task_file n=1 path=src/run/window.rs mark=edits
what: the wrapper's window step that places an opened window on its workspace through the Hyprland seam — edits: the move-to-workspace request it sends
@task_file n=2 path=crates/viola-hyprland mark=edits
what: the Hyprland seam, the one compositor boundary with its closed request set, its hyprctl probe, its tap and its fake — edits: the window move request form, the fake's reply set
@task_file n=3 path=fixtures/hyprland mark=edits
what: the recorded Hyprland fixtures per compositor version — edits: the move fixture, re-recorded through the operator's grant
@task_file n=4 path=src/run/auto_verify.rs mark=edits
what: the automatic verify of a new CLI run at a start, and the capability stamp it leaves — edits: where it runs and what it stamps when its own preconditions fail
@task_file n=5 path=src/cmd/run.rs mark=edits
what: the viola run start, the wrapper process that gates the CLI version and calls the automatic verify and the window step — edits: the start's call of the automatic verify
@task_item n=1 part=needs
The window step of a start (the run.window span and the window line of the run start events) sends a move whose form leaves the compositor's active workspace where it was; what the run start span attributes and the window line record of it.
@task_item n=2 part=needs
The Hyprland seam's closed request set gains or changes one move form, never a second compositor seam; the hyprctl probe child it spawns and the hyprctl-json parser's parse-rejected line keep their subject and detail words.
@task_item n=3 part=needs
The callers of the move, viola open, viola up and viola revive under --origin: their CLI process-log files, their process-exit detail on exit 1, and the revive start that logs as run, all reading the same placement answer.
@task_item n=4 part=needs
The seam's tap, fake and recorded fixture follow the new form: the fake's reply set kept to what the real compositor answers, the recorded fixture no telemetry source and holding no content beyond the compositor's replies.
@task_item n=5 part=needs
The active-workspace clause as an acceptance predicate a gate entry reads: workspace read before and after a placing open, compared by a machine-asserted verdict, not prose.
@task_item n=6 part=needs
The live legs (the fixture re-record, the v2-07 live reading) are operator-fired and gated; the phase and CI dispatch nothing to the compositor, and any compositor query the plan names is a read-only one.
@task_item n=7 part=needs
The automatic verify at a start: where it runs, the auto-verify subject of its child spawn lines, the run process log's line for a stop during the automatic verify, and the run log line's cli_verified and cli_version values.
@task_item n=8 part=needs
What the automatic verify stamps when its own preconditions fail: verify remains the only writer of capability stamps, a stamp write refusal and an unreadable ledger read as an unverified CLI, and the ledger's path and stamps are never logged.
@task_item n=9 part=needs
The folded item's witness cannot pass vacuously, and the real CLI's verify never runs in CI: CI verifies against the fake agent only.
@task_item n=10 part=needs
Indirect, through the window step: the window line has no trace of its own, start spans carry no endpoint, path or env values, and instrument attributes skip all arguments.
@task_item n=11 part=needs
Indirect, through any new failure cause the move or the automatic verify adds: a closed detail code carrying no path or pid, fixed stderr messages that interpolate nothing, and a distinct detail code per exit cause.
@task_item n=12 part=needs
Indirect, through the crates touched: synchronous instrumentation without Tokio, every line written through the one obs event macro with a closed event value, and the run stays silent on the terminal.
@task_standing n=1
convention: each new integration or E2E test's emitted diagnostics lines are schema-checked, and its test home lives under the one scanned test-home root, kept until the gate steps run
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: each new log line goes through the obs_event macro with a closed event value and corr where correlated, no raw tracing or print macros, no bare instrument attribute
home: 11-obs-anti-patterns-never-do-these
@task_standing n=3
convention: each new event or detail value a diagnostics line can carry is added to the line schema additively, never renaming a field
home: log-coverage · 6-log-coverage
@task_item n=1 part=not_needed
The MCP stdio server, its tools, frames and refusals: the chunk reaches no MCP path.
@task_item n=2 part=not_needed
The dialog hook, hook stdout and stderr, hook latency budgets and hook perf gates: no hook path changes.
@task_item n=3 part=not_needed
The channel's JSON-RPC lines, send, wait, last, wheel, pause and release paths and their spans: the move and the automatic verify send no channel frame.
@task_item n=4 part=not_needed
Link and unlink verbs, lineage names and the events log's link records: the chunk adds no link.
@task_item n=5 part=not_needed
Heartbeat liveness, list liveness and staleness thresholds: unchanged by a window move.
@task_item n=6 part=not_needed
Snapshot replay and state-recovered lines, torn-line healing of the event stream: the chunk writes no state record.
@task_item n=7 part=not_needed
The future error reporter, OTel exporters, network egress and the absences of hosted deployment, database and gRPC: nothing here adds an exporter or a network path.
@task_item n=8 part=not_needed
Metric coverage, counters, gauges and histograms derived by jq: no metric is added.
@task_item n=9 part=not_needed
Panic outcome by role for mcp and hook, and the mcp main-thread and task panic rows: the chunk touches no such role.
