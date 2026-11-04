@answer n=1 entity=slo-invariants-telemetry-budgets#1.1 v=no
why: the session changed only test files and no panic path, role-file write or G2 counting, so the one-line panic invariant holds on the changed files
@answer n=2 entity=slo-invariants-telemetry-budgets#2.1 v=no
why: the session changed no hyperfine perf row, perf harness or gate --require perf, so the session-end max budget holds as before
@answer n=3 entity=slo-invariants-telemetry-budgets#2.2 v=no
why: the session changed no hook timed path, socket-directory check, euid decision or server verify, and the macOS no-pid gap it describes stands unchanged
@answer n=4 entity=slo-invariants-telemetry-budgets#2.3 v=no
why: the new witness back-dates a heartbeat 60 s and moves neither the 4.9 s live nor the 5.1 s stale flip, so the flip thresholds hold on the changed test files
@answer n=5 entity=slo-invariants-telemetry-budgets#2.4 v=no
why: the session changed no hook logger init, timed instance strict-modes check or perf-job G2 run, so the overhead row holds unchanged
