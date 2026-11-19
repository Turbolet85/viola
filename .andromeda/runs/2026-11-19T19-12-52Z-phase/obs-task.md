@task_prose part=chunk
Adds /context to the compiled list of local commands a send confirms by a post-condition rather than by a submitted prompt: one more local-command stdout tag in the session transcript after the send's own pre-paste end, confirmed the way /mcp is today.
Adds a capability-ledger row for that post-condition on /context, with its viola verify probe (a compiled-literal paste in an interactive verify run), the row stamped live on the installed claude CLI; whether the row is relied on is left to P3.
A send of /context then confirms by the post-condition, or answers ok with confirmed false and detail unconfirmable where the version has no measured post-condition.
The fake agent rehearses the new paste and post-condition so CI proves the path without a live start; the one-reading mechanism behind the post-condition is re-verified at P3/verify, never assumed.
Carries forward two open readings unchanged in kind: the placeholder hint's cell attributes (attributes only, never text) read on the next rig start, and the intermittent rig rehearsal red mended only at its next red, with no retry rule.
@task_item n=1 part=needs
The send path's confirmation of a local command: the confirmation window close for an ordinary send versus a post-condition-confirmed local command, and the send-confirmed line it ends in.
@task_item n=2 part=needs
The unconfirmable send: a send answering ok with confirmed false and detail unconfirmable when the version has no measured post-condition, and how that send's line carries it.
@task_item n=3 part=needs
Send log lines and their fields (send-issued, send-confirmed, send-refused), their corr, and the closed detail words a send may carry; no new detail word may be invented outside the closed set.
@task_item n=4 part=needs
Sent text is never logged: the /context paste is sent text and stays out of every process log, span field and verify line; only its byte count or closed codes may appear.
@task_item n=5 part=needs
The transcript read behind the post-condition: the transcript echo read stays uninstrumented, Claude Code transcripts and wrapped claude internals are not a contract, and the stdout-tag count is a product reading, never a telemetry source.
@task_item n=6 part=needs
No telemetry from screen content: the post-condition is read from the transcript, and the hint-attribute reading carried forward reads cell attributes only, never text, and feeds no log line.
@task_item n=7 part=needs
The capability ledger: its path and stamps are never logged, an unreadable ledger counts as an unverified CLI, and a parse rejection of the ledger stamps is logged as the parse-rejected line with its closed detail.
@task_item n=8 part=needs
viola verify as the only stamps writer: the new probe's verify lines carry no content, its step and summary lines on stdout, its exit codes, and its child spawn subject for an interactive PTY probe.
@task_item n=9 part=needs
Real-CLI verify never runs in CI and CI verify runs against the fake agent only: the live stamp is an operator step, and the CI proof is the fake agent's rehearsal.
@task_item n=10 part=needs
Recorded fixtures and the fake agent's rehearsal are no telemetry source, and claude fixtures are scrub-required.
@task_item n=11 part=needs
Confirmed send spans and their attributes: the span chain of a confirmed send, the readiness gate and confirmation window spans, the window_ms and text_bytes attributes, all declared with skip-all and an explicit field allow-list.
@task_item n=12 part=needs
Any new log line goes through the single obs_event macro with a closed event value; no raw tracing macros, print macros or bare instrument attributes in the new code.
@task_item n=13 part=needs
No retry policies and no retry-once: the intermittent rig rehearsal red is mended only at its next red, its cause named first, never by a retry or a loosened wait.
@task_item n=14 part=needs
No sleep-based log waits in the new tests and rehearsal: waits key on a condition, not a fixed sleep.
@task_item n=15 part=needs
Emitted diagnostics lines from the new tests are schema-checked by the conformance gate and stay within the log line schema, which is additive only.
@task_standing n=1
convention: Every integration or E2E test that runs a viola process has its emitted lines schema-checked against the diagnostics line schema.
home: ci-integration
@task_standing n=2
convention: Every integration-test home lives under the one scanned test root and is kept until the gate steps run.
home: 9-ci-integration
@task_standing n=3
convention: Every recorded claude fixture is scrub-required and is never a telemetry source.
home: 1-obs-scope-summary
@task_standing n=4
convention: Every new test waits on a condition, never a fixed sleep, and carries no retry rule.
home: 10-slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
Hook rows: hook latency budgets, hook fail-open, hook stdout and stderr, hook capture arms; the chunk changes no hook.
@task_item n=2 part=not_needed
Dialog rows: dialog-raised and dialog-answered lines, dialog hold, expired or second dialog; the chunk raises or answers no dialog.
@task_item n=3 part=not_needed
viola-channel link and unlink, pause and release, wheel transitions and the release-from-driver refusal; the chunk touches none of them.
@task_item n=4 part=not_needed
The MCP stdio server's frame and stdin rows (MAX_FRAME, oversize lines, MCP refusal texts); the chunk adds no MCP tool.
@task_item n=5 part=not_needed
Revive preflight, the agents probe of claude agents and its parse-failure counts; the chunk adds no revive or agents path.
@task_item n=6 part=not_needed
Heartbeat, liveness and staleness rows, list --json liveness; the chunk changes no liveness signal.
@task_item n=7 part=not_needed
State recovery and snapshot replay rows (torn-line heal, snapshot-replayed, replayed snapshot unreachable); the chunk touches no state store.
@task_item n=8 part=not_needed
Future error reporter, Sentry, OTel SDK and exporter, network egress and the absences of a database, gRPC, hosted deployment or web frontend; the chunk adds no dependency or sink.
@task_item n=9 part=not_needed
Socket-directory, strict-modes and server-verify refusal codes on CLI and MCP verbs; the chunk changes no refusal path.
@task_item n=10 part=not_needed
config.json keys and the diagnostics level setting; the chunk adds no configuration.
