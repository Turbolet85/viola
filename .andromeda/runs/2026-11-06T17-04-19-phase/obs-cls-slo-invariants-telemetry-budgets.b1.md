@answer n=1 entity=slo-invariants-telemetry-budgets#1.1 v=yes
why: the G2 zero-panic gate and the zero error budget the done-check clears count panic lines by this rule, and the live-session evidence reads it through agent-run logs
@answer n=2 entity=slo-invariants-telemetry-budgets#2.1 v=yes
why: the session-end hook max < 1.0 s row is judged by `gate --require perf` in the perf job that the green-CI check covers, and perf.rs's EXE_SUFFIX join builds its hook commands
@answer n=3 entity=slo-invariants-telemetry-budgets#2.2 v=yes
why: the spine and pre-tool-use perf rows are required by name in `gate --require perf`, which the green-CI run covers, and perf.rs's EXE_SUFFIX join builds these hook commands
@answer n=4 entity=slo-invariants-telemetry-budgets#2.3 v=no
why: the heartbeat stale/gone flip is untouched (scope not-needed item 4) and no gate the chunk re-runs names it beyond the general test run
@answer n=5 entity=slo-invariants-telemetry-budgets#2.4 v=yes
why: the obs-overhead row is judged by the same perf gate the green-CI check covers and its hook commands are built by perf.rs's EXE_SUFFIX joins
