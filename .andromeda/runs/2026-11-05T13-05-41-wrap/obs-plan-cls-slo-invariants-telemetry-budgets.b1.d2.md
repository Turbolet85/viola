@answer n=1 entity=slo-invariants-telemetry-budgets#1.1 v=yes mode=stale
why: the panic-record rule and its refused-check exemptions live in reached src/panic_frames.rs and src/cmd files, and the Windows owner/DACL strict-modes arm they cite is removed
@answer n=2 entity=slo-invariants-telemetry-budgets#2.1 v=no
why: the session-end hyperfine max budget and its perf gate are untouched, the task keeping the perf budgets and hyperfine max unchanged
@answer n=3 entity=slo-invariants-telemetry-budgets#2.2 v=yes mode=contradict
why: the spine-hook timed path's macOS recorded gap with no pid comparison is retired, the macOS no-peer-pid carve-out collapsing to Linux
@answer n=4 entity=slo-invariants-telemetry-budgets#2.3 v=no
why: the heartbeat liveness 4.9 s live and 5.1 s stale flip is untouched, the task keeping the 5 s flip unchanged
@answer n=5 entity=slo-invariants-telemetry-budgets#2.4 v=no
why: the obs-overhead row sits inside the perf budgets and hyperfine max the session leaves unchanged, and its Linux strict-modes instance check is unchanged
