@answer n=1 entity=metric-coverage#1.1 v=no
why: generic Counter over event, detail and refusal grouping of home-level diagnostics; it names no Windows-only or changed value, and the chunk changes no log line it reads
@answer n=2 entity=metric-coverage#1.2 v=no
why: histogram of raw duration_ms per event; names no event or line the chunk changes
@answer n=3 entity=metric-coverage#1.3 v=no
why: gauge from harness snapshots of budget.read_at and liveness; the heartbeat stays as it is (scope not_needed item 7) and the sh status path stays
@answer n=4 entity=metric-coverage#2.1 v=yes
why: hook-decision duration_ms per hook_event with gate = hyperfine max; the chunk's perf job re-measures the five hook rows against the 1.0 s budgets (scope item 5)
@answer n=5 entity=metric-coverage#2.2 v=no
why: channel-response latency per method; the chunk changes no channel wire or channel-response line
@answer n=6 entity=metric-coverage#2.3 v=no
why: send-confirmed duration_ms and the readback window in src/run/send.rs; the send path is not reached and no send value is named by the scope
@answer n=7 entity=metric-coverage#2.4 v=no
why: send terminal-state counter over send-confirmed and send-refused; the send path is not reached and the scope names none of these values
@answer n=8 entity=metric-coverage#2.5 v=no
why: wait outcome counter over channel-response of method wait; the chunk removes only the macOS carve-out on wait refusals and names no Linux wait outcome value
@answer n=9 entity=metric-coverage#2.6 v=no
why: state-recovered counter for chaos heal; the heal path is untouched and the scope excludes new state-recovered values (not_needed items 8 and 12)
@answer n=10 entity=metric-coverage#2.7 v=no
why: parse.skipped gauge from list --json skipped; no list or parse path is reached
@answer n=11 entity=metric-coverage#2.8 v=no
why: parse-rejected counter per parser; no parser path is reached and no Windows-only parser value is named
@answer n=12 entity=metric-coverage#2.9 v=no
why: unknown and v_rejected counters over send-refused and channel-response details; the chunk changes no channel-response, send or release path and the scope names none of these details
@answer n=13 entity=metric-coverage#2.10 v=yes
why: child and probe exits counter over process-exit fields subject, child_exit_status and exit_source; the scope keeps process-exit from the handle-wait thread and exit_source kill-fallback unchanged once the TerminateProcess fallback leaves viola-pty (scope item 12)
@answer n=14 entity=metric-coverage#2.11 v=yes
why: viola.panic counter must read zero; the scope's error budget is zero panic lines by G2 and the zero-unlogged-panics invariant stands (scope items 6 and 14)
@answer n=15 entity=metric-coverage#3.1 v=yes
why: the Linux server-verify-failed and socket-dir-refused process-exit lines at WARN in cli-<name>.ndjson; the scope keeps those details and exit 21 unchanged and removes only the macOS euid-only arm (scope item 10)
@answer n=16 entity=metric-coverage#3.2 v=yes
why: a CLI wait refused by the strict-modes check writes no line; the scope keeps exit 21, instance-unreachable and strict-modes-failed unchanged, so the chunk must add no line on that path (scope item 10)
@answer n=17 entity=metric-coverage#4.0 v=yes
why: the hook-decision quantile snippet states that the gates use max; the chunk's perf job keeps the hyperfine max gate under the 1.0 s budgets (scope item 5), so the stated gate value binds
