# Classified — obs-plan · slo-invariants-telemetry-budgets.b1

## Answers
slo-invariants-telemetry-budgets#1.1 · no · the new socket-dir refusals write their lines after the wrapper or cli log opens, so no pre-init or refused-check exemption moves
slo-invariants-telemetry-budgets#2.1 · no · the session-end row's report-only percentiles and its max under 1.0 s budget still hold on this run's highest max of 2.8 ms
slo-invariants-telemetry-budgets#2.2 · yes · stale · the timed path now also holds the socket-dir check and the euid decision before verify_server, and macOS gains an euid comparison its "no comparison" reading omits; the 2.4–3.3 ms band still holds
slo-invariants-telemetry-budgets#2.3 · no · the session moves viola list's home scan rule, not the heartbeat thresholds or the liveness reading
slo-invariants-telemetry-budgets#2.4 · no · the new socket-dir check on the hook runs after hook-invoked, so logger init's one open and its order after the metadata reads still hold
