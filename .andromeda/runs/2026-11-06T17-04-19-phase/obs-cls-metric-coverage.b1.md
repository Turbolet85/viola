@answer n=1 entity=metric-coverage#1.1 v=no
why: counts send-refused lines by side and detail, and the chunk names no send refusal or detail value
@answer n=2 entity=metric-coverage#1.2 v=no
why: raw duration_ms latency histogram per event, and the chunk adds or changes no timed event
@answer n=3 entity=metric-coverage#1.3 v=no
why: budget.read_at and liveness gauges, and the chunk leaves the budget governor and heartbeat unchanged
@answer n=4 entity=metric-coverage#2.1 v=no
why: hook-decision latency per hook_event, and the chunk touches no hook path
@answer n=5 entity=metric-coverage#2.2 v=no
why: channel-response latency per method, and the chunk adds no channel method
@answer n=6 entity=metric-coverage#2.3 v=no
why: send-confirmed readback window and src/run/send.rs gate timing, neither named by the scope
@answer n=7 entity=metric-coverage#2.4 v=no
why: send-confirmed and send-refused terminal-state counter, and the scope names no send outcome value
@answer n=8 entity=metric-coverage#2.5 v=no
why: wait outcome counter over channel-response method wait, and the chunk changes no channel method
@answer n=9 entity=metric-coverage#2.6 v=no
why: state-recovered heal counter, and no state-store reader or appender is touched
@answer n=10 entity=metric-coverage#2.7 v=no
why: list --json skipped gauge, and no state-store reader is touched
@answer n=11 entity=metric-coverage#2.8 v=no
why: parse-rejected counter per parser, and the scope names no parser
@answer n=12 entity=metric-coverage#2.9 v=no
why: unknown and unsupported-version compat counters, and the scope names none of those refusals
@answer n=13 entity=metric-coverage#2.10 v=yes
why: the child-exit counter reads process-exit subject, child_exit_status and exit_source, which item 6 names as the wrapper's handle-wait exit line and the kill-fallback value the portable-pty bump must keep emitting
@answer n=14 entity=metric-coverage#2.11 v=yes
why: the panic counter must read 0, the same zero error budget the done-check must clear as the chunk's own re-asserted value
@answer n=15 entity=metric-coverage#3.1 v=yes
why: names the socket-dir-refused process-exit line and its detail grouping, which the chunk re-words under its socket-directory refusal wording item
@answer n=16 entity=metric-coverage#3.2 v=no
why: a strict-modes wait refusal (client::refuse_untrusted, exit 21) with no line written, not the socket-directory refusal the scope names
@answer n=17 entity=metric-coverage#4.0 v=no
why: report-only hook session-end quantile snippet, and the chunk changes no hook latency or G2 rule
