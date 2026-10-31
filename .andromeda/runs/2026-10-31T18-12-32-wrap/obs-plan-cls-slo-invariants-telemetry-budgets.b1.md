# Classified — obs-plan · slo-invariants-telemetry-budgets.b1

## Answers
slo-invariants-telemetry-budgets#1.1 · yes · stale · under `revive --json` the start's home-tree refusal now prints an exit-1 document (every refusal of the start does), so the exemption-8 reading "exit 1, the home or the instance pair" no longer gives the whole output; no line is written, so the exemption itself holds
slo-invariants-telemetry-budgets#2.1 · no · the `hook session-end` perf row lives in the hook and the perf job, neither reached; `src/cmd/hook.rs` is not in the diff and no perf row or threshold moved
slo-invariants-telemetry-budgets#2.2 · no · the hook's timed path, its server check and its snapshot read live in hook and channel code the session did not reach; the `hook-decision` words and the spine perf rows stand
slo-invariants-telemetry-budgets#2.3 · no · the session adds `without_endpoint` to `liveness.rs` but moves no heartbeat threshold or liveness value, so the 4.9 s live and 5.1 s stale or gone flip still reads the same in `list --json`
slo-invariants-telemetry-budgets#2.4 · no · the hook's logger init and its instance and diagnostics checks are in hook and strict-mode code the session did not change, and the perf budget did not move
