@answer n=1 entity=metric-coverage#1.1 v=no
why: Counter over event, detail and refusal values in diagnostics; the chunk adds no metric or grouping key and edits no event value
@answer n=2 entity=metric-coverage#1.2 v=no
why: Histogram of raw duration_ms per event; no chunk change touches latency samples
@answer n=3 entity=metric-coverage#1.3 v=no
why: Gauge over budget.read_at and liveness; the budget and heartbeat are untouched (not-needed items 2 and 4)
@answer n=4 entity=metric-coverage#2.1 v=no
why: hook latency gate sits in perf.rs, whose edit is the EXE_SUFFIX join only; the gate unit is a re-check every change owes
@answer n=5 entity=metric-coverage#2.2 v=no
why: channel-response latency per method; the chunk adds no channel method (not-needed item 3)
@answer n=6 entity=metric-coverage#2.3 v=no
why: send-confirmed latency is read in src/run/send.rs, which the chunk does not edit or name
@answer n=7 entity=metric-coverage#2.4 v=no
why: send terminal-state counter; no send path is touched by the chunk
@answer n=8 entity=metric-coverage#2.5 v=no
why: wait outcome counter from channel-response; the chunk adds no channel verb and touches no wait path
@answer n=9 entity=metric-coverage#2.6 v=no
why: state-recovered heal counter; the torn-append heal and state readers are untouched (not-needed item 5)
@answer n=10 entity=metric-coverage#2.7 v=no
why: list --json skipped gauge; no state-store reader is touched
@answer n=11 entity=metric-coverage#2.8 v=no
why: parse-rejected counter per parser; the chunk touches no parser
@answer n=12 entity=metric-coverage#2.9 v=no
why: unknown and v_rejected compat counters; no send, link, unlink or channel validation is touched
@answer n=13 entity=metric-coverage#2.10 v=yes
why: the scope names process-exit from the handle-wait thread with exit_source "kill-fallback" (item 6), the fields this counter groups by
@answer n=14 entity=metric-coverage#2.11 v=yes
why: the scope's zero error budget (0 panic lines, zero panics under G2) is the value this counter states
@answer n=15 entity=metric-coverage#3.1 v=yes
why: the scope names the socket-dir-refused pair and its detail code (item 14), which this rule counts as a process-exit at WARN
@answer n=16 entity=metric-coverage#3.2 v=no
why: strict-modes refusal in client::refuse_untrusted; the chunk does not change it and the scope does not name it
@answer n=17 entity=metric-coverage#4.0 v=no
why: report-only quantile snippet over hook-decision lines; the chunk re-pins no value it reads
