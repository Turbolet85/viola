@answer n=1 entity=slo-invariants-telemetry-budgets#1.1 v=no
why: the session leaves the zero-unlogged-panics invariant and its exemptions unchanged, so the one-line panic claim still holds on the reached hook and send files
@answer n=2 entity=slo-invariants-telemetry-budgets#2.1 v=no
why: run --perf and gate --require perf stay green with the session's budgets unchanged, so the hook session-end max under 1.0 s budget holds
@answer n=3 entity=slo-invariants-telemetry-budgets#2.2 v=no
why: the spine and pre-tool-use deadline and timed path stay as written, run --perf is green and the budgets are unchanged
@answer n=4 entity=slo-invariants-telemetry-budgets#2.3 v=no
why: the 4.9 s and 5.1 s heartbeat flip is a section 10 budget value the session left unchanged, with run --perf green
@answer n=5 entity=slo-invariants-telemetry-budgets#2.4 v=no
why: the hook logger-init overhead gate and its strict-modes check order hold on the changed code, since run --perf is green and the budgets are unchanged
