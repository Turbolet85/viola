# Classified — obs-plan · slo-invariants-telemetry-budgets.b1

## Answers
slo-invariants-telemetry-budgets#1.1 · no · the oversize cut panics nothing and writes a WARN parse-rejected line, not a panic line, so the zero-unlogged-panics rule and its exemptions hold on the reached files and g2 reads clean
slo-invariants-telemetry-budgets#2.1 · no · no hook path, perf row or gate command changed and fs.rs's change is replace_private_shared's compare read, so the session-end budget still holds
slo-invariants-telemetry-budgets#2.2 · no · no hook path, perf row, gate command or state-recovery reader changed and fs.rs's change is replace_private_shared's compare read, so the spine budget and its timed-path checks still hold
slo-invariants-telemetry-budgets#2.3 · no · no state-recovery path, reader or line changed, so the heartbeat staleness flip and its list liveness reading still hold
slo-invariants-telemetry-budgets#2.4 · no · no hook path changed and fs.rs's change is replace_private_shared's compare read, so logger init's one open and the instance check's metadata reads still hold
