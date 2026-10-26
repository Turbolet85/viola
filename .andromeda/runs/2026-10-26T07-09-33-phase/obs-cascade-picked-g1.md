# obs · picked accounting · group 1 of 7

## Picked
obs-scope-summary#1.4 · not in scope · a count of the plan's telemetry triggers; the chunk adds no trigger, and the triggers it meets ride as body lines under their entities
obs-scope-summary#2.1 · not in scope · the diagnostics root reconciliation; the chunk opens no new log file, its lines land in the existing role files
obs-scope-summary#2.2 · carried · the MCP tools' new `instance-unreachable` line lands in the `mcp.ndjson` sink this rule fixes
obs-scope-summary#2.4 · carried · the scope's dialog hook client fails open on a mismatch, so `hook-invoked`'s timing after the `hook.dialog` response is the chunk's own path
obs-scope-summary#3.2 · carried · the root bin's client verbs and hook are edited, and their new verification outcome is instrumented there
obs-scope-summary#6.2 · carried · the chunk edits `viola-channel`'s sync and Tokio client connect and its endpoint pid read
obs-scope-summary#6.3 · carried · the server identity read added to `viola-channel` must stay sync outside the `tokio` feature
obs-scope-summary#7.2 · carried · the chunk edits `viola-state`'s `live_endpoint`, the pre-check the verification joins
obs-scope-summary#7.3 · carried · the start-time comparison joined into `viola-state` must stay sync
obs-scope-summary#9.2 · carried · the chunk edits the `send` / `wait` / `last` / `answer` tools' pre-check in `viola-mcp`
obs-scope-summary#11.2 · carried · the hook client's fail-open-on-mismatch path is a hook invocation the chunk edits
obs-scope-summary#11.3 · carried · the chunk instruments `hook-decision` and the MCP tool-call handling on a failed verification
obs-scope-summary#12.2 · not in scope · the `claude agents --json` read, which the chunk does not reach
obs-scope-summary#14.2 · carried · the new peer-credential, pipe server pid and sysinfo start-time reads are OS API call sites the chunk adds
obs-scope-summary#14.3 · carried · the chunk's verification is instrumented at viola's connect call site, never the OS internals
obs-scope-summary#15.2 · carried · the clients read `snapshot.json`'s `pid`, `started_at` and `endpoint` through `viola-state`
obs-scope-summary#15.3 · carried · the `endpoint` path the clients read is scrubbed from external output, and the `snapshot.json` read is the scope's own
obs-scope-summary#18.1 · not in scope · standing S2 · every new span is `#[instrument(skip_all, name = ..., fields(...))]` with an explicit allow-list
obs-scope-summary#18.3 · carried · the CLI verbs' exit-21 mismatch line goes to the cli file sink only when an instance resolves
obs-scope-summary#18.5 · carried · the failed verification is a new cause behind exit 21 and needs its own distinct `detail` code
obs-scope-summary#19.1 · not in scope · the wrapper's server-side spans and start sequence, unchanged; `run`'s collision check is only re-read
