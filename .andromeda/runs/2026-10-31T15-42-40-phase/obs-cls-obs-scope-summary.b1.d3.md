# Classified — obs · obs-scope-summary.b1

## Answers
obs-scope-summary#1.1 · no · a count of telemetry surface entries across process roles; it states no rule the chunk's code or tests honour
obs-scope-summary#1.2 · no · a count of instrumentable crates; it states no rule the chunk's code or tests honour
obs-scope-summary#1.3 · no · a count of must-trace paths and edge flows; it states no rule the chunk's code or tests honour
obs-scope-summary#1.4 · no · a count of telemetry triggers; the chunk's own trigger (Founder Direction 4) is not stated in this entity
obs-scope-summary#2.1 · yes · the pending fork over the diagnostics root decides where the new process-exit lines land and where the tests read them (`<home>/diagnostics/` vs `instances/<name>/diagnostics/`)
obs-scope-summary#2.2 · yes · mcp writes stderr JSON only when mcp.ndjson cannot open, which binds the mcp refused home check (exit 1, nothing on stdout or stderr) and the rejected-call lines
obs-scope-summary#2.3 · no · the event enum extensions are release-from-driver and chaos signals; the chunk moves process-exit detail values, not event names
obs-scope-summary#2.4 · no · hook-invoked corr timing relative to dialog_id is excluded by the task's Not needed list
obs-scope-summary#3.1 · no · source citation only; the rule it cites is #3.3
obs-scope-summary#3.2 · yes · the CLI crate is instrumentable, so the new cause paths in src/cmd and the obs and human modules must be logged per Section 6, outside the pre-log refusals
obs-scope-summary#3.3 · yes · the anyhow error-capture boundary: a serde-sourced chain must become a fixed message before stderr and the full chain goes only to diagnostics/, which binds the new fixed Display strings and the replaced chain
obs-scope-summary#4.1 · no · source citation only; the rule it cites is #4.3
obs-scope-summary#4.2 · yes · viola-core is reached whole and hosts the closed codes read through is_code_word, so the new codes added to its lib.rs must add no span or log call
obs-scope-summary#4.3 · yes · viola-core emits nothing and supplies the kebab-case values used as log fields, so the new codes are values there, not emissions
obs-scope-summary#5.1 · no · source citation for the PTY seam, which the chunk does not reach
obs-scope-summary#5.2 · no · the PTY seam is excluded by the task's Not needed list; no exit-cause code arises there
obs-scope-summary#5.3 · no · the PTY byte stream is user content and the OS PTY APIs are boundary-only; the chunk does not reach either
obs-scope-summary#6.1 · no · source citation for the IPC seam; the rules it sources are #6.2 and #6.3
obs-scope-summary#6.2 · yes · the IPC seam carries the frame verbs, and the vanished-server and endpoint-lost reads in viola-channel client.rs must log as instrumentable
obs-scope-summary#6.3 · yes · sync-only instrumentation binds the new client-side log calls in viola-channel client.rs, which must avoid Tokio
obs-scope-summary#7.1 · no · source citation only; the rules it sources are #7.2 and #7.3
obs-scope-summary#7.2 · yes · the chunk's new process-exit and endpoint-lost lines are written through this ndjson pipeline, so its instrumentable status binds them
obs-scope-summary#7.3 · yes · sync-only instrumentation binds the new obs_event! lines written through this pipeline
obs-scope-summary#8.1 · no · serde_path_to_error drift reports; claude agents --json parsing is excluded by the task's Not needed list
obs-scope-summary#8.2 · no · the drift-report crate is not reached by the chunk and no exit-21 or exit-1 code sits there
