@answer n=1 entity=slo-invariants-telemetry-budgets#1.1 v=yes
why: the zero-panic rule and its counting are what gate G2 and the zero error budget (0 panic lines) count, both named in the chunk's scope and build-failure items.
@answer n=2 entity=slo-invariants-telemetry-budgets#2.1 v=yes
why: the chunk edits run/perf.rs (EXE_SUFFIX join), the `run --perf` harness whose hyperfine rows gate --require perf judges, inside the chunk's every-gate-green scope.
@answer n=3 entity=slo-invariants-telemetry-budgets#2.2 v=yes
why: the spine and pre-tool-use perf rows are re-measured by the edited run/perf.rs and judged by gate --require perf, so the timed hook path's max sample is part of the chunk's gate run.
@answer n=4 entity=slo-invariants-telemetry-budgets#2.3 v=no
why: heartbeat staleness flip is untouched; the chunk's not_needed item 4 says nothing changes the heartbeat file or its stale/gone flip.
@answer n=5 entity=slo-invariants-telemetry-budgets#2.4 v=yes
why: run/perf.rs must give each hyperfine hook command VIOLA_NAME and a home under target/e2e-home/ with a strict-modes-passing instance, and the perf job runs G2 over that home.
