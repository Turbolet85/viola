# Classified — obs-plan · obs-scope-summary.b3

## Answers
obs-scope-summary#17.1 · no · the CI/CD source excerpt stands because no scripts/ or .github/ file changed
obs-scope-summary#17.2 · no · CI/CD stays not-instrumentable because no scripts/ or .github/ file changed
obs-scope-summary#17.3 · no · CI/CD is still not product runtime and no fixtures/ file is among the reached files
obs-scope-summary#18.1 · no · the check runs inside the existing dispatch spans and no span name or area was added or renamed, so the spans and the panic hook stand
obs-scope-summary#18.2 · no · no frontend is in the session, so N/A stands
obs-scope-summary#18.3 · no · the sink configuration and diagnostics/ locations are untouched and the new lines land in the existing cli-<name>.ndjson and hook-<name>.ndjson files
obs-scope-summary#18.4 · no · service identity is unchanged with no event value, field or schema edit
obs-scope-summary#18.5 · no · the server-verify refusal stays a fixed string with no pid, start time, path or endpoint (asserted by security_negatives_server_identity), exit codes are unchanged, and the new exit-21 cause has its own distinct detail code
obs-scope-summary#19.1 · no · the run's spans and the channel server's spans are unchanged and no Tokio enters a sync crate
obs-scope-summary#19.2 · no · no frontend is in the session, so N/A stands
obs-scope-summary#19.3 · no · the run's sink and diagnostics/run-<name>.ndjson are untouched and the run's start sequence still verifies no server
obs-scope-summary#19.4 · no · run's service identity is unchanged with no event value, field or schema edit
obs-scope-summary#19.5 · no · the panic hook replacement and the 1 s heartbeat are untouched and liveness gains a server verdict beside the existing heartbeat read
obs-scope-summary#20.1 · no · the spans around stdin parse, channel call and decision emit still enclose the check, hook-invoked and hook-decision are still written (a mismatch writes hook-decision), and the catch sites are not reached
obs-scope-summary#20.2 · no · no frontend is in the session, so N/A stands
obs-scope-summary#20.3 · no · the hook sink and diagnostics/hook-<name>.ndjson are untouched, a mismatch prints nothing and writes no stderr, and the capture arms sit off the shared connect
obs-scope-summary#20.4 · no · process="hook" identity and the E1 unwrapped no-op are untouched, and the new Vector 4 fail-open cause is a different cause that leaves the missing-VIOLA_NAME no-op standing
obs-scope-summary#20.5 · no · the 1.0 s spine bound stands at a measured 2.2–2.4 ms with the check, and the sink and the statusline path are untouched
obs-scope-summary#21.1 · no · server_pid reads write and log nothing and no span name or area changed, so the client and server spans and the corr propagation stand
obs-scope-summary#21.2 · no · no frontend is in the session, so N/A stands
obs-scope-summary#21.3 · no · the sink configuration and diagnostics/ locations are untouched and the check only adds lines to the per-process files that already exist
obs-scope-summary#21.4 · no · each side still logs its own process and the process field and closed event enum are untouched
obs-scope-summary#21.5 · yes · add-to · F8, on macOS the peer pid is not reported (SERVER_PID_REPORTED false), so the server check returns Matches with no comparison, a recorded gap owed to Unix endpoint and home hardening
obs-scope-summary#22.1 · no · the check runs inside mcp.tool_call after TokioClient::connect, so each frame tool's span still wraps the Tokio client and no span name changed
obs-scope-summary#22.2 · no · no frontend is in the session, so N/A stands
