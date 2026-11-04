@answer n=1 entity=metric-coverage#1.1 v=yes
why: the counter reads home-level role files only and skips a torn line as G2 does, a G2 reading item 12 leaves to the G2 step
@answer n=2 entity=metric-coverage#1.2 v=no
why: a derived histogram definition in the metric catalog, which the scope drops as not needed; it states no gate, line or value the chunk must hold
@answer n=3 entity=metric-coverage#1.3 v=no
why: a derived gauge row naming agent-run status as the liveness reader, not the list --json reader item 18 names; it states no value the chunk must hold
@answer n=4 entity=metric-coverage#2.1 v=yes
why: states the hook latency gate as hyperfine max, the per-OS perf job's gate over the timed hook rows (item 16)
@answer n=5 entity=metric-coverage#2.2 v=no
why: a derived channel-latency counter over channel-response duration; it states no gate or line the chunk must emit or test
@answer n=6 entity=metric-coverage#2.3 v=no
why: a send-path confirm-latency derivation; the chunk touches no send path and the scope names no send line
@answer n=7 entity=metric-coverage#2.4 v=no
why: a derived send-outcome counter over send lines the scope does not name
@answer n=8 entity=metric-coverage#2.5 v=no
why: a derived wait-outcome counter; the wait refusal lines it depends on bind through metric-coverage#3.1 and #3.2
@answer n=9 entity=metric-coverage#2.6 v=no
why: a derived state-recovered counter; the same detail words on every OS are governed by item 21 directly, not by this row
@answer n=10 entity=metric-coverage#2.7 v=no
why: a derived skipped gauge over a list --json field; --json field parity is governed by items 5 and 21 directly
@answer n=11 entity=metric-coverage#2.8 v=no
why: a derived parse-rejected counter over a line the scope does not name
@answer n=12 entity=metric-coverage#2.9 v=no
why: a derived compat counter; its closed detail words are governed by item 5 directly and the row states no Unix-specific value
@answer n=13 entity=metric-coverage#2.10 v=no
why: a derived child-exit counter; exit_source and kill-fallback are governed by item 2 directly, not by this row
@answer n=14 entity=metric-coverage#2.11 v=yes
why: states the panic value must be 0, the zero-unlogged-panics invariant that G2 counts and the in-job gate measures (item 11)
@answer n=15 entity=metric-coverage#3.1 v=yes
why: states the CLI server-verify-failed and socket-dir-refused process-exit lines (exit 21, during connect, WARN) that the macOS carve-out and the socket-directory refusal require the chunk to emit (items 6 and 7)
@answer n=16 entity=metric-coverage#3.2 v=yes
why: states that a CLI or MCP wait refused by the strict-modes check writes no line, a no-log rule the Unix refusal paths must keep (items 5 and 6)
@answer n=17 entity=metric-coverage#4.0 v=yes
why: states the perf gates read max and that hook-*.ndjson is read with torn-line skipping as G2 does, a reading the per-OS perf job and G2 must honour (items 12 and 16)
