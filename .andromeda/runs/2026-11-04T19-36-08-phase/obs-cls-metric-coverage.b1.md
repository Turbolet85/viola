@answer n=1 entity=metric-coverage#1.1 v=no
why: a derived Counter row of the metric catalog, which the task drops as not needed (item 3); its home-level-only scope restates item 12 and owns no rule the chunk must act on
@answer n=2 entity=metric-coverage#1.2 v=no
why: a derived Histogram row of the metric catalog (raw duration samples, no bucketing), which the task drops as not needed (item 3)
@answer n=3 entity=metric-coverage#1.3 v=yes
why: its gauge names liveness read through agent-run status, the liveness signal the v1-40 stale-row witness rests on (item 18)
@answer n=4 entity=metric-coverage#2.1 v=yes
why: the hook-decision duration gated by hyperfine max is the statistic the per-OS perf job's gate over the five timed hook rows must use (item 16)
@answer n=5 entity=metric-coverage#2.2 v=no
why: a derived channel-latency metric row with no gate and no emitted-line rule the chunk's code must meet (item 3)
@answer n=6 entity=metric-coverage#2.3 v=no
why: a derived send-confirm metric row over send.rs, which the chunk does not change and no gate names (item 3)
@answer n=7 entity=metric-coverage#2.4 v=no
why: a derived send-outcome counter row with no gate and no emitted-line rule for the chunk (item 3)
@answer n=8 entity=metric-coverage#2.5 v=no
why: a derived wait-outcome counter row; the CLI wait refusal lines it depends on are bound by entity metric-coverage#3.1, not by this row (item 3)
@answer n=9 entity=metric-coverage#2.6 v=no
why: a derived heal counter row naming state-recovered.detail, with no value stated that the chunk re-asserts (item 3)
@answer n=10 entity=metric-coverage#2.7 v=no
why: a derived skipped-records gauge row over list --json, with no value stated that the chunk re-asserts (item 3)
@answer n=11 entity=metric-coverage#2.8 v=no
why: a derived parse-rejection counter row for parsers the chunk does not touch (item 3)
@answer n=12 entity=metric-coverage#2.9 v=no
why: a derived compat counter row naming viola.compat.v_rejected, which task item 3 names explicitly as not needed
@answer n=13 entity=metric-coverage#2.10 v=no
why: a derived child-exit counter row grouping process-exit fields; the process-exit rules are bound by task items 2 and 4, not by this row (item 3)
@answer n=14 entity=metric-coverage#2.11 v=yes
why: the panic SLO that must be 0 is the zero-unlogged-panics invariant G2 counts and the per-OS gate exit codes measure (item 11)
@answer n=15 entity=metric-coverage#3.1 v=yes
why: it states the process-exit server-verify-failed and socket-dir-refused lines the Unix CLI path must emit on each OS (items 5, 6, 7)
@answer n=16 entity=metric-coverage#3.2 v=yes
why: a CLI wait refused by the strict-modes check writes no line, so the chunk's Unix client path must not add one there (item 5, strict-modes-failed)
@answer n=17 entity=metric-coverage#4.0 v=yes
why: it states the gates use max, the statistic the per-OS hyperfine perf gate must hold to (item 16); the quantile snippet itself is report-only
