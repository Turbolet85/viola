@task_prose part=chunk
Every session viola starts names its model and its effort: a start (viola run, and the start arm of viola revive) hands its claude child a model and an effort on every spawn, the pair the operator gave, else viola's configured default pair.
The pair is recorded per instance as a closed set of values, beside the lineage record (the events log, the snapshot, the replay), and viola revive replays the recorded pair on its resume.
The CLI channel that carries the pair to claude (flag, environment name or settings key), where and in what shape the pair is recorded, and what a resume does with model and effort today are measured first at P3; an undocumented claude behaviour becomes a capability ledger row with a verify probe.
Inheritance of the pair by open, up and adopt is not built here, switching a running session's pair is not this chunk, and nothing is written to the user's own claude settings.
@task_item n=1 part=needs
The viola run and viola revive start arm argument surface: a model and an effort may be given at launch, viola's configured default pair used where none is given.
@task_item n=2 part=needs
The claude child spawn: its argv and environment carry the pair on every spawn, through the child resolution and flag literals of the claude agent adapter.
@task_item n=3 part=needs
The process-start line of the claude child and the start spans over the spawn: environment names only, never values, no endpoint, path or env values on start spans, and the child settings argument never logged; a pair carried by flag or env obeys the same.
@task_item n=4 part=needs
The CLAUDE environment strip over the child's environment: the identity floor stays stripped and stripped CLAUDE values are never logged, whatever channel the pair takes.
@task_item n=5 part=needs
config.json as the home of viola's configured default pair, if the operator's fork lands it there: its known keys and version, configuration observed only at load, an unknown key or value read as a parse-rejected config line.
@task_item n=6 part=needs
Environment variables are not a configuration channel for viola: the default pair is never read from an environment variable of viola's own.
@task_item n=7 part=needs
The events log and the snapshot as the record of the pair: one events line of the event-line shape, deterministic emission, product readers reading the snapshot through replay, and the start's product order read from the events log.
@task_item n=8 part=needs
The pair as a closed set of values: model and effort words are closed enums, never free text, wherever a line or record carries them.
@task_item n=9 part=needs
The resume: viola revive replays the pair its instance recorded; the revive start arm logs as run, and its preflight order and detail codes stay as they are.
@task_item n=10 part=needs
viola list, list --json and the MCP list tool, should the operator's fork show the pair there: any field added is additive to what those surfaces carry.
@task_item n=11 part=needs
The capability ledger: a row for an undocumented claude behaviour of the pair's CLI channel; ledger path and stamps are never logged and an unreadable ledger counts as an unverified CLI.
@task_item n=12 part=needs
A diagnostics field added for the pair is additive only, typed, on the default-deny field allow-list and in the diag-line schema the conformance gate checks.
@task_item n=13 part=needs
Proof without a live start: no rig start of a real claude and no real-CLI verify in CI; tests run against the fake agent only, and nothing of the operator's real claude home is read.
@task_standing n=1
convention: every line a new test makes a process emit is schema-checked against the diag-line schema in integration and E2E tests
home: ci-integration
@task_standing n=2
convention: every new log line goes through the single obs_event macro, never a raw tracing or print macro, with spans skipping all arguments
home: 11-obs-anti-patterns-never-do-these
@task_standing n=3
convention: every new integration-test home lives under the one scanned test-home root and is kept until the gates run
home: 9-ci-integration
@task_standing n=4
convention: every recorded claude fixture is scrub-required and is no telemetry source
home: 1-obs-scope-summary
@task_standing n=5
convention: new obs code is mutation-tested like product code
home: ci-integration
@task_item n=1 part=not_needed
The dialog path, its hook lines, hook decision codes and hook latency budgets: the chunk raises and answers no dialog and changes no hook.
@task_item n=2 part=not_needed
The send path, the wheel, pause and release, and the confirmation window: the chunk sends no text and types nothing into a session.
@task_item n=3 part=not_needed
The viola-channel JSON-RPC lines, their corr and conn join keys and the -32603 error: the pair travels at spawn, not over the channel.
@task_item n=4 part=not_needed
viola mcp stdin framing, MAX_FRAME and the frame-tool refusals for wait and last: the chunk adds no MCP framing or frame tool.
@task_item n=5 part=not_needed
Link and unlink, their events and the implicit link on send and answer: the chunk records no link.
@task_item n=6 part=not_needed
Panic capture, panic outcomes by role and the detail-file panic line: the chunk adds no catch site and changes no panic handling.
@task_item n=7 part=not_needed
Socket-directory and server-verification refusals and their exit 21 cause words: the chunk adds no connect path.
@task_item n=8 part=not_needed
The future error reporter, OTel, network egress and telemetry exporter rows: the chunk adds no reporter, exporter or endpoint.
@task_item n=9 part=not_needed
Heartbeat ticks, liveness and the stale threshold: the chunk changes no liveness signal.
@task_item n=10 part=not_needed
Artifact upload, secret-scan hit reports and retention: the chunk adds no CI upload.
