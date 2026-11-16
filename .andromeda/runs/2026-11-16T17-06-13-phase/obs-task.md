@task_prose part=chunk
Grows the closed cause set of the turn-ended event from the one word api-error to nine: rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error.
The CLI's StopFailure error value is classified inside the Claude adapter as it already is; only the resulting word crosses into the turn-ended cause, events.ndjson, viola wait --json and the MCP wait tool.
api-error stays the word for the CLI's unknown, an absent error and any value outside the map; the plan says how a new CLI error value is noticed later (a count or a closed log word, never its text) or states that it is not.
Every consumer that accepts exactly api-error today accepts the nine and still refuses any other text; the CLI's error text still reaches no event, snapshot, log line, reply or diagnostics line.
Re-measures at P3 where the word is produced, every consumer that matches on it, and each known CLI error value as measured or read-from-source; no live claude start and no mutation run.
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the Claude adapter's hook payload reading, where a StopFailure's error value is classified into an API-error class and the turn-ended data is built — edits: data_of, the ApiErrorClass to cause mapping
@task_file n=2 path=crates/viola-core/src/lib.rs mark=edits
what: viola-core's closed event model, the TurnEndCause value set of the turn-ended event's cause field — edits: TurnEndCause and its ALL list
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the wrapper's appended-line check on events.ndjson, admitting a turn-ended line only with cause absent or a TurnEndCause word — edits: the appended-line check
@task_item n=1 part=needs
The turn-ended event line in events.ndjson: its cause field is a closed value set that grows from one word to nine; the event line shape stays as it is and no new event kind is minted.
@task_item n=2 part=needs
The CLI's error text (last_assistant_message, error_details, the raw error value) crosses no event, snapshot, process log line, detail line, reply or MCP result except as one of the nine closed words: last_assistant_message and assistant output are never logged.
@task_item n=3 part=needs
Wrapped claude internals and its hook payload values are untrusted and not a contract: an error value outside the map reads api-error, the unknown fallback, never echoed.
@task_item n=4 part=needs
How an unmapped new CLI error value is noticed later, if at all: only as a count or a closed log word on an existing diagnostics line (parse-failure or unknown counter style), never the value's text; any such line or field is additive to the line schema and joins the closed event and field allow-list.
@task_item n=5 part=needs
viola wait --json and the MCP wait tool carry the turn-ended cause word to the driving agent: both paths accept the nine words and refuse any other text.
@task_item n=6 part=needs
The board and list readers of the event log, and the agent-run status and logs readers that read events records, accept the nine cause words and refuse any other text.
@task_item n=7 part=needs
Closed enums and code words: the cause is a closed code like the other detail and cause words, interpolating nothing; any metric grouping by cause groups by the closed nine only.
@task_item n=8 part=needs
viola-core emits no telemetry: growing TurnEndCause adds no log line or instrumentation inside viola-core.
@task_item n=9 part=needs
Recorded Claude payload fixtures used as proof are scrub-required and synthetic, and are no telemetry source; proof rests on the fake agent and recorded payloads, never a live claude start in CI.
@task_item n=10 part=needs
Hook constraints the classifier code runs under: the hook fails open silently, writes no stderr, keeps exit 0, emits its decision body only on stdout and stays inside its latency budget.
@task_item n=11 part=needs
Wait log lines and the channel-response outcome on the wait path carry no event payload: the cause word is not copied from events.ndjson into process logs.
@task_item n=12 part=needs
Each of the nine classes is proven by a bounded test against the fake agent or a recorded payload, and a refusal test proves a word outside the nine is refused by the appended-line check and the readers.
@task_standing n=1
convention: every new or changed integration or E2E test's emitted diagnostics lines are schema-checked, carry no line content in gate output, and its home lives under the scanned test-home root kept until the gates run
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: every new log line or field goes through the obs_event macro with a closed event value, the default-deny field allow-list, and stays additive to the line schema
home: 11-obs-anti-patterns-never-do-these
@task_standing n=3
convention: every new fixture or test input is synthetic and scrubbed, never real conversation text, and never uploaded unscanned
home: 8-pii-scrubbing-compliance · pii-scrubbing-compliance
@task_item n=1 part=not_needed
Dialog, answer and send paths: their log lines, spans, corr rules and refusal detail words; the chunk changes only the turn-ended cause.
@task_item n=2 part=not_needed
Process-exit, exit-1 and exit-21 detail codes, unreachable cause words and strict-modes or socket-directory refusals: a turn end is no process exit or connect refusal.
@task_item n=3 part=not_needed
Panic capture, panic lines, panic outcome by role and the future error reporter: the chunk adds no panic path and no reporter.
@task_item n=4 part=not_needed
Span declarations, span kinds and span attributes of run start, wheel, send and dialog: the chunk adds no span.
@task_item n=5 part=not_needed
Heartbeat liveness, list liveness and staleness thresholds: the cause word changes no liveness.
@task_item n=6 part=not_needed
Revive, verify, plugin install, link and unlink verbs with their exit codes and stdout: none reads a turn-ended cause.
@task_item n=7 part=not_needed
Channel JSON-RPC frame, params, conn and srv_conn rows and peer-credential checks: the channel protocol is unchanged.
@task_item n=8 part=not_needed
Performance gates and hook benchmark budgets as measurements: no perf job is planned for this chunk.
@task_item n=9 part=not_needed
Log rotation, file modes, home resolution, config.json keys and service identity: no sink, path or config key changes.
@task_item n=10 part=not_needed
Network egress, OTel exporters, Sentry and other telemetry-receiver absences: nothing leaves the process.
