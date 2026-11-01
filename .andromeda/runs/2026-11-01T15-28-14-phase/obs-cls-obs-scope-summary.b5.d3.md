# Classified — obs · obs-scope-summary.b5

## Answers
obs-scope-summary#31.3 · no · the wheel `human-typing` refusal (exit 10) is not reached by the socket, peer-euid or home-check changes
obs-scope-summary#31.4 · no · a source note for the wheel scenario, not a rule over the chunk's code
obs-scope-summary#32.1 · no · the budget surfaces (`budget.json`, `budget-gate`) are not moved
obs-scope-summary#32.2 · no · the budget must-trace spans and events are outside the chunk
obs-scope-summary#32.3 · no · the budget-paused fields are outside the chunk
obs-scope-summary#32.4 · no · a source note for the budget governor, not a rule over the chunk's code
obs-scope-summary#33.1 · yes · the channel-request/response lines log `from` as unauthenticated, and a verified euid must not make it an authenticated identity in any line
obs-scope-summary#34.1 · no · the send-text never-log rule and the send-refused text codes are untouched, and the new refusals carry no text
obs-scope-summary#35.1 · yes · hook fail-open paths log only to `hook-<name>.ndjson` with exit 0 and empty stderr, which the hook connect refusal joins as a fail-open
obs-scope-summary#36.1 · yes · rejected MCP frame tools log codes only and write no line, which the client-side euid mismatch and socket-directory refusal map to `instance-unreachable`
obs-scope-summary#37.1 · no · the never-log floor for the CLAUDE_CODE_MESSAGING values and the R8 strip are not touched by the chunk
obs-scope-summary#38.1 · yes · the strict-modes refusals that leave no line (`check_home_tree`, `refuse_untrusted`, `viola list`'s scan) are where the home check and client refusals change
obs-scope-summary#39.1 · no · the claude child and probe process-start/exit lines are not reached
obs-scope-summary#40.1 · no · standing S11 · A new dependency stays Tokio-free in the sync crates, adds no C-building crate and passes `cargo deny`
obs-scope-summary#41.1 · no · the corr joins for send cursor and dialog_id are unchanged, and a wrapper-side refusal is keyed by `conn` with no request behind it
obs-scope-summary#42.1 · yes · a hook connect refusal writes a `hook-decision` fail-open line, which must carry `duration_ms` per hook event
obs-scope-summary#43.1 · no · endpoint-lost during call, torn lines, snapshot replay and stale heartbeat are mid-call faults the chunk does not change
obs-scope-summary#44.1 · yes · the macOS pid gap (no server-verify-failed line, verdict Matches with no comparison) is the named carve-out the scope records, and Unix 0600/0700 applies to the socket directory
obs-scope-summary#45.1 · no · the zero-unlogged-panics invariant and the check_diagnostics panic-sink gap are not changed
obs-scope-summary#46.1 · no · no rotation rule is touched
obs-scope-summary#47.1 · yes · the refusal traces in `hook-<name>.ndjson` and `run-<name>.ndjson` are process logs under `diagnostics/` that harness `logs` merges with `events.ndjson`
obs-scope-summary#48.1 · yes · each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid, which the two new refusals and the socket-directory cause must take
obs-scope-summary#49.1 · no · the parser and unknown-field counters and the exit-14 fallback are reader-side and not changed
obs-scope-summary#50.1 · no · standing S10 · A new test takes no retry-once policy and no sleep-based log wait, and asserts by exit code or a `jq` / harness verdict
