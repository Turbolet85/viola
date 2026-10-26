# Classified — obs · obs-scope-summary.b5

## Answers
obs-scope-summary#31.3 · no · the wheel take-over's required log fields on a must-trace path the task names unchanged, and no wheel code is reached
obs-scope-summary#31.4 · no · a provenance line (tests excerpt, Critical Path 5) that states no rule of its own
obs-scope-summary#32.1 · no · the budget governor's surfaces on a must-trace path the task names unchanged, with no tokio code on it
obs-scope-summary#32.2 · no · the budget governor's must-trace spans and budget-paused events on a path the task names unchanged
obs-scope-summary#32.3 · no · the budget governor's required fields and exit 11 on a path the task names unchanged
obs-scope-summary#32.4 · no · a provenance line (tests excerpt, Critical Path 6) that states no rule of its own
obs-scope-summary#33.1 · no · the channel request and response scrub rule, an IPC trigger of security Vectors 1 to 8 that the task names not needed; no channel log code is edited
obs-scope-summary#34.1 · no · the send-text never-log rule, a paste trigger of security Vectors 1 to 8 that the task names not needed; no paste path is edited
obs-scope-summary#35.1 · no · the fail-open hook logging and empty stderr, a hook stdin trigger of security Vectors 1 to 8 that the task names not needed; no hook code is edited
obs-scope-summary#36.1 · no · the MCP rejection logs with codes only, an MCP rejects trigger of security Vectors 1 to 8 that the task names not needed; the rmcp error path is not edited
obs-scope-summary#37.1 · no · the messaging-token never-log floor and the R8 strip, a CLI and env trigger of security Vectors 1 to 8 that the task names not needed; the bump adds no env, flag or config, and the veil toggle ban is a cargo deny check that obs-scope-summary#40.1 covers
obs-scope-summary#38.1 · no · the diagnostics 0600 and 0700 rule and its refusal telemetry, a filesystem trigger of security Vectors 1 to 8 that the task names not needed; the named secret-scan test is part of the generic workspace gate
obs-scope-summary#39.1 · no · process-start and process-exit for the claude child and its probes, a child-spawn trigger of security Vectors 1 to 8 that the task names not needed; the chunk spawns nothing new
obs-scope-summary#40.1 · yes · obs dependencies stay Tokio-free in the sync crates, add no C-building crate and pass cargo deny on all three targets, the Vector 9 supply-chain trigger the lock and pin bump must keep true
obs-scope-summary#41.1 · no · corr joins for send and dialog on must-trace paths whose events the task keeps unchanged
obs-scope-summary#42.1 · no · duration_ms and the session-end and spine budgets on the hook path, named not needed, with no hook, run or heartbeat code changing
obs-scope-summary#43.1 · no · fault telemetry classes that need event values or an additive detail field the chunk does not add, and its heartbeat class is named not needed
obs-scope-summary#44.1 · no · one file sink and line schema on every OS, and the bump moves no sink, span or platform log path
obs-scope-summary#45.1 · no · the zero-unlogged-panics invariant, named not needed, and the panic hook runs before tokio's spawned-task containment with no hook code changing
obs-scope-summary#46.1 · no · no rotation of events.ndjson or diagnostics files in v1, and the chunk adds no rotation or file
obs-scope-summary#47.1 · no · structured JSON logs and the event list adopted verbatim, a log catalog the task keeps unchanged whose send events sit on a path named unchanged
obs-scope-summary#48.1 · no · a distinct detail code per exit cause, the detail code catalog the task keeps unchanged, and the chunk adds no code
obs-scope-summary#49.1 · no · counters for unknown kinds, torn lines and per-parser failures, none of which the Cargo and script edits reach
obs-scope-summary#50.1 · no · deterministic events keyed on offsets with no log-based sleeps, on paths whose events the task keeps unchanged
