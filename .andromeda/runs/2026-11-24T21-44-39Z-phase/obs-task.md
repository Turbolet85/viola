@task_prose part=chunk
Changes where `viola adopt` starts its resumed claude child, so the CLI finds the adopted session: today the spawn folder of the adopt start is the caller's current directory.
Adds what `viola adopt` says and records when the resumed child ends at once after the start: a closed detail word, an exit, a stderr unable/hint pair, an event and a process-log line, and whether the instance directory and name are released after it.
If the fix moves where the R5 live-reading rig must type `viola adopt`, the rig script changes with it and its rehearse form reaches the changed path with no real claude.
Re-reads, as P3 measurements and with no live start: what was recorded of the two red children (exit code, time to exit), what revive and up use as their resume folder (the snapshot's recorded cwd) and that adopt has no such record, and what the capability ledger holds about a resume from another folder.
@task_file n=1 path=src/cmd/adopt.rs mark=edits
what: the viola adopt verb, whose start logs as run and spawns the resumed claude child — edits: the adopt start's spawn folder for the claude child, the handling of a child that exits at once after the start (its exit, detail word, stderr pair, event and log line, and the release of the instance directory and name)
@task_file n=2 path=scripts/r5-adopt.sh mark=edits
what: the R5 live-reading rig for viola adopt, with a live arm and a rehearse arm that runs no claude — edits: the folder the live arm types `viola adopt` in, and the rehearse form reaching the changed path
@task_item n=1 part=needs
The adopt start's resumed claude child gets a new spawn folder; the adopt start keeps logging as run, with the claude-child process-start and process-exit lines in the run process log.
@task_item n=2 part=needs
A new early-exit cause of viola adopt gets its own closed detail code, distinct from every other exit-1 cause of adopt, carrying no path or pid: one more entry in the process-exit detail codes for exit 1 from viola adopt.
@task_item n=3 part=needs
The early child exit is read from the process handle, never from screen content; its exit code and duration ride the claude-child process-exit line as typed fields.
@task_item n=4 part=needs
The early exit reaches the human as a fixed, pathless stderr unable/hint pair beside the existing refusal pairs; the error message interpolates nothing, no session id and no folder.
@task_item n=5 part=needs
The early exit is recorded as a product event in the events log: a closed event value, the event line shape kept, emission deterministic, and no event payload copied into the process log.
@task_item n=6 part=needs
Any new event word, detail word or field added to the diagnostics line is additive only and joins the diagnostic line schema and its closed enums, so the schema matches the event enum.
@task_item n=7 part=needs
Adopt's typed session id is never logged; the resume folder path appears in no home-level line or detail code, at most in the instance detail file.
@task_item n=8 part=needs
The instance is resolved from its name, never from the working directory: moving the child's spawn folder must not change how adopt finds or names its instance, nor which diagnostics files it writes.
@task_item n=9 part=needs
Releasing or keeping the instance directory and the name after the early exit follows the rule that no process log is written without an instance and that a refused check opens no process log.
@task_item n=10 part=needs
The CLAUDE environment strip and identity floor still apply to the resumed child at its new folder; stripped values are never logged, names only.
@task_item n=11 part=needs
Wrapped claude internals and Claude Code transcripts are not a contract: the product must not decide the resume folder by reading the CLI's transcript store.
@task_item n=12 part=needs
The capability ledger is read for a row on resume from another folder (none means UNMEASURED); an unreadable ledger counts as unverified, and the ledger's path and stamps are never logged.
@task_item n=13 part=needs
Every new log line goes through the single obs_event macro with explicit corr and instance; no raw tracing macros, no print macros, and an instrument attribute skips all arguments.
@task_item n=14 part=needs
The early-exit and spawn-folder behaviour is proved against the fake agent only; the real CLI never runs in CI or in this phase, and the rig's rehearse arm spawns no claude.
@task_item n=15 part=needs
The revive and up starts' resume folder (the snapshot's recorded cwd, with the cwd-missing exit-1 detail) is read as the model for adopt's folder, never changed.
@task_standing n=1
convention: each new log line is a single-line obs_event record with a closed event value, codes only, and is schema-checked in integration and E2E tests
home: log-coverage
@task_standing n=2
convention: each new detail code is a closed word, distinct per exit cause, carrying no path or pid
home: 1-obs-scope-summary · obs-scope-summary
@task_standing n=3
convention: each new stderr refusal or fault line is a fixed, pathless unable/hint pair, with serde and error chains reduced to a fixed message
home: error-capture-reporting
@task_standing n=4
convention: each new test that spawns a child uses the fake agent, never the real claude
home: 4-span-trace-coverage
@task_standing n=5
convention: each new test home lives under the one scanned integration-test root and is kept until the gates run
home: 9-ci-integration · ci-integration
@task_standing n=6
convention: each new fixture is synthetic or scrubbed, and no fixture is a telemetry source
home: pii-scrubbing-compliance
@task_item n=1 part=not_needed
The viola mcp stdio server, its tools, frame refusals and stdin limits: adopt is a cli verb whose start logs as run, and no MCP surface changes.
@task_item n=2 part=not_needed
Hook roles, hook stdout and stderr, hook fail-open, and the spine and session-end hook latency budgets and their perf gates.
@task_item n=3 part=not_needed
The channel JSON-RPC lines, send, wait, last, dialog and wheel paths, their spans, log fields and refusal details.
@task_item n=4 part=not_needed
Heartbeat liveness, staleness thresholds and list --json liveness.
@task_item n=5 part=not_needed
Link and unlink, lineage, launch and window lines and their events.
@task_item n=6 part=not_needed
Absences of a future error reporter, OTel SDK or exporter, Sentry, network egress, hosted deployment, database or gRPC: nothing here adds telemetry export.
@task_item n=7 part=not_needed
State recovery from snapshot replay and torn-line healing in the events log: the chunk adds no replay or healing.
@task_item n=8 part=not_needed
The secret-scan hit report fields and artifact retention rules: no new CI job or upload is added.
