@answer n=1 entity=slo-invariants-telemetry-budgets#1.1 v=yes
why: the zero-unlogged-panics invariant, G2's counting rule and the bounded exemptions bind the panic hook, the catch sites, the run and revive refusals and G2's single seam exemption; its retry-once line restates S6 but is not what binds it
@answer n=2 entity=slo-invariants-telemetry-budgets#2.1 v=yes
why: the hyperfine max under 1.0 s gate for hook session-end, judged by gate --require perf, binds the per-OS perf job over the five timed hook rows
@answer n=3 entity=slo-invariants-telemetry-budgets#2.2 v=yes
why: the spine and pre-tool-use deadline gate binds the timed hook path on Unix, its socket-dir check, euid decision and server check, and the socket-dir-refused and server-verify-failed fail-open details
@answer n=4 entity=slo-invariants-telemetry-budgets#2.3 v=yes
why: the 4.9 s live and 5.1 s stale heartbeat flip is the liveness signal behind the v1-40 witness, read through list --json
@answer n=5 entity=slo-invariants-telemetry-budgets#2.4 v=yes
why: obs overhead on hook holds only with VIOLA_NAME set and a home under target/e2e-home/, which the perf job's scope names
