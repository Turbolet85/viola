@answer n=1 entity=slo-invariants-telemetry-budgets#1.1 v=yes
why: the scope's zero error budget (task_item n=8, "0 panic lines") and G2 zero panics rule over the one-line-per-panic invariant, so the chunk's done-check must clear it
@answer n=2 entity=slo-invariants-telemetry-budgets#2.1 v=yes
why: the chunk's CI-green wrap covers the perf job, whose `gate --require perf` asserts the hook session-end `max < 1.0 s` value this row states
@answer n=3 entity=slo-invariants-telemetry-budgets#2.2 v=yes
why: the scope's socket-directory refusal wording (task_item n=14) names the `socket-dir-refused` detail code this spine and pre-tool-use row states, and the perf gate witnesses the timed path
@answer n=4 entity=slo-invariants-telemetry-budgets#2.3 v=no
why: the scope states nothing changes the heartbeat file or its stale/gone flip (task_item n=4 not_needed), and no named check reads this liveness value
@answer n=5 entity=slo-invariants-telemetry-budgets#2.4 v=yes
why: the same perf gate `max < 1.0 s` over the hook timed path is the CI check the chunk must keep green, and the row names the perf job's G2 run over that home
