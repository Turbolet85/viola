# Classified — obs · slo-invariants-telemetry-budgets.b1

## Answers
slo-invariants-telemetry-budgets#1.1 · yes · the chunk's new verification code in cli, hook and mcp runs after init step 4, so a panic there still owes one ERROR panic line in its role file and hook exit 0; a check placed before step 4 would fall in exemption 1
slo-invariants-telemetry-budgets#2.1 · yes · the added peer-credential and start-time reads sit on session-end's hook.event path, so its max must stay under the 1.0 s gate that gate --require perf judges
slo-invariants-telemetry-budgets#2.2 · yes · the same reads sit on the hook.event path of session-start, user-prompt-submit, stop and pre-tool-use, so their max must stay under the spine deadline gate
slo-invariants-telemetry-budgets#2.3 · yes · the edited live_endpoint is the liveness-only pre-check the verification joins, and its heartbeat-live verdict is set by this 4.9 s live / 5.1 s stale flip, which must hold after the join
slo-invariants-telemetry-budgets#2.4 · yes · the chunk adds reads and client log lines (channel-request, fail-open hook-decision) to the hook.event path it edits, so hook's obs overhead must stay inside the same max < 1.0 s gate
