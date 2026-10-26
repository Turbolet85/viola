# Classified — obs · slo-invariants-telemetry-budgets.b1

## Answers
slo-invariants-telemetry-budgets#1.1 · yes · the new listener tests and run's start must each leave exactly one single-line event:"panic" record in their home-level role file (G2 on the Windows leg), with retries = 0 and no retry-once policy
slo-invariants-telemetry-budgets#2.1 · no · the session-end hook's hyperfine max budget; the chunk reaches no hook role and its Not-needed section excludes hook latency budgets
slo-invariants-telemetry-budgets#2.2 · no · the spine-hook and pre-tool-use hyperfine budgets; the chunk reaches no hook role and its Not-needed section excludes hook latency budgets
slo-invariants-telemetry-budgets#2.3 · no · the heartbeat staleness flip read as list --json liveness; the Not-needed section excludes heartbeat liveness transitions and the chunk re-measures none
slo-invariants-telemetry-budgets#2.4 · no · the hook obs-overhead gate times hyperfine hook commands; the chunk touches no hook path, and its own target/e2e-home/ test homes come from its Needs, not from this value
