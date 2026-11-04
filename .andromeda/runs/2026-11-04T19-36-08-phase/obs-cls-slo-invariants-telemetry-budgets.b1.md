@answer n=1 entity=slo-invariants-telemetry-budgets#1.1 v=yes
why: the zero-unlogged-panics invariant and its G2 count rule bind through the panic hook, the catch sites and the G2 step on each OS (task items 10 to 12).
@answer n=2 entity=slo-invariants-telemetry-budgets#2.1 v=yes
why: the hyperfine max gate on hook session-end is one of the five timed hook rows the per-OS perf job gates through gate --require perf (task item 16).
@answer n=3 entity=slo-invariants-telemetry-budgets#2.2 v=yes
why: the spine and pre-tool-use timed path holds the Unix socket-dir, euid and server checks with their socket-dir-refused and server-verify-failed details, which the Unix endpoint telemetry and the perf job must honour (task items 5 to 7 and 16).
@answer n=4 entity=slo-invariants-telemetry-budgets#2.3 v=yes
why: the 4.9 s live and 5.1 s stale flip read through list --json is the liveness signal behind the v1-40 witness (task item 18).
@answer n=5 entity=slo-invariants-telemetry-budgets#2.4 v=yes
why: obs overhead on hook holds only when every hook runs with VIOLA_NAME set and a home under target/e2e-home/, the condition the per-OS perf job must meet (task item 16).
