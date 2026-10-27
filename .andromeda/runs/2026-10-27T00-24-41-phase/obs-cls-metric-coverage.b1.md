# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · yes · the Counter groups every event and detail value over home-level role files, so the typed-error WARN or ERROR closed code and the absent-copy exit-1 detail code (if the fix adds one) are counted here
metric-coverage#1.2 · no · a histogram of raw duration_ms samples, and the chunk adds no duration_ms line (its witness waits on hook events and events.ndjson offsets)
metric-coverage#1.3 · no · the gauge reads budget.read_at and heartbeat liveness, both Not needed, while the witness reads only status's state and last_error
metric-coverage#2.1 · no · hook latency comes from hook-decision lines on the hook path, which the chunk does not change; the pin write runs at start, not in a hook spawn
metric-coverage#2.2 · no · no channel method is added or changed, so no channel-response duration is reached
metric-coverage#2.3 · no · no send path changes and the witness waits on hook events, not a send readback, so no send-confirmed duration is reached
metric-coverage#2.4 · no · no send path changes, and the pinned-exe refusal is not a send line
metric-coverage#2.5 · no · no frame client changes, so no wait channel-response outcome is reached
metric-coverage#2.6 · no · no state reader or appender changes, so the pin write reaches no state-recovered line
metric-coverage#2.7 · no · the events.ndjson reader and its skip counts are unchanged, so list --json skipped is not reached
metric-coverage#2.8 · no · the chunk parses no config, stamp or ledger input, so no parse-rejected line is reached
metric-coverage#2.9 · no · no send or channel refusal is added, so the compat counters are not reached
metric-coverage#2.10 · no · it counts child and probe exits by child_exit_status and exit_source; the new absent-copy exit is the run's own self exit, and no child or probe exit changes
metric-coverage#2.11 · no · standing S5 · new test output counts toward the zero budgets: 0 unlogged panics (G2)
metric-coverage#3.1 · no · the CLI server-verify refusal is a frame-client line, and no frame client changes
metric-coverage#3.2 · no · a CLI wait refused by the instance check writes no line, and no frame client changes; the pin refusal is a different path
metric-coverage#4.0 · no · a report-only quantile snippet over hook-decision session-end durations, and the chunk adds no hook-decision line
