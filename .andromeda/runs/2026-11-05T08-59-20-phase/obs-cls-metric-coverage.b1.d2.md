@answer n=1 entity=metric-coverage#1.1 v=no
why: counter over diagnostics lines the chunk does not change; no scope item names this derivation
@answer n=2 entity=metric-coverage#1.2 v=no
why: histogram of duration_ms samples; the chunk changes no duration line and no scope item names it
@answer n=3 entity=metric-coverage#1.3 v=yes
why: the gauge reads the harness status command (agent-run status) that scope item 17 names and keeps on the sh path
@answer n=4 entity=metric-coverage#2.1 v=yes
why: hook-decision latency gated by hyperfine max, the perf budgets scope item 5 keeps for the five hook rows
@answer n=5 entity=metric-coverage#2.2 v=no
why: channel-response latency; the channel wire stays unchanged and no scope item names this value
@answer n=6 entity=metric-coverage#2.3 v=no
why: send readback latency over the send path, which the chunk does not reach and no scope item names
@answer n=7 entity=metric-coverage#2.4 v=no
why: send outcome counter; the chunk adds and changes no send line or send closed word
@answer n=8 entity=metric-coverage#2.5 v=no
why: wait outcome counter reads the channel-response outcome, which the chunk leaves unchanged; scope names the refusal details and exit 21, not this value
@answer n=9 entity=metric-coverage#2.6 v=no
why: state-recovered counter; no reached file or scope item touches heal or recovery
@answer n=10 entity=metric-coverage#2.7 v=no
why: parse skipped gauge from list --json; the chunk does not touch the list path
@answer n=11 entity=metric-coverage#2.8 v=no
why: parse-rejected counter per parser; the chunk does not touch the parsers
@answer n=12 entity=metric-coverage#2.9 v=no
why: compat counters over refusal and detail values the chunk does not change; scope names none of these values
@answer n=13 entity=metric-coverage#2.10 v=yes
why: counts process-exit lines with exit_source, the PTY seam values scope item 12 names as unchanged (handle-wait process-exit and the kill-fallback exit_source)
@answer n=14 entity=metric-coverage#2.11 v=yes
why: panic counter whose target is zero, the zero-panic-lines error budget scope item 6 keeps by G2 and the zero-unlogged-panics invariant of scope item 14
@answer n=15 entity=metric-coverage#3.1 v=yes
why: names the Linux exit 21 with server-verify-failed and socket-dir-refused, which scope item 10 keeps, and the macOS euid carve-out the chunk removes
@answer n=16 entity=metric-coverage#3.2 v=yes
why: names exit 21, instance-unreachable and strict-modes-failed on the strict-modes refusal, which scope item 10 keeps unchanged
@answer n=17 entity=metric-coverage#4.0 v=no
why: report-only quantile snippet over hook-decision lines the chunk does not change; the gate the chunk keeps uses max, which entity 2.1 carries
