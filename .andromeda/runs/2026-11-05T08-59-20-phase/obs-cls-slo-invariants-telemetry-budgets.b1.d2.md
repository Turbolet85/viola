@answer n=1 entity=slo-invariants-telemetry-budgets#1.1 v=yes
why: the G2 panic count and its single seam exemption at src/cmd/hook/seam.rs are named by the chunk's error budget and G2 scope, so the counting rule binds
@answer n=2 entity=slo-invariants-telemetry-budgets#2.1 v=yes
why: the hook session-end 1.0 s max budget judged by gate --require perf is named by the chunk's perf scope over the five hook rows
@answer n=3 entity=slo-invariants-telemetry-budgets#2.2 v=yes
why: the spine and pre-tool-use rows gated by gate --require perf are named by the chunk's perf scope, and the Unix peer euid check on the timed path stays on Linux
@answer n=4 entity=slo-invariants-telemetry-budgets#2.3 v=no
why: the heartbeat liveness flip is shared Linux code the chunk leaves unchanged, and no scope item names it
@answer n=5 entity=slo-invariants-telemetry-budgets#2.4 v=yes
why: the perf job's hyperfine hook commands must run with VIOLA_NAME set and a home under target/e2e-home/, which the chunk's perf scope names
