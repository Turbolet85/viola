@answer n=1 entity=metric-coverage#1.1 v=no
why: the counter is a report-only jq query over diagnostics files; the chunk changes no line it reads and the scope names no torn-line behaviour
@answer n=2 entity=metric-coverage#1.2 v=no
why: a histogram of duration_ms samples; the chunk changes no duration_ms line and no gate reads it
@answer n=3 entity=metric-coverage#1.3 v=no
why: the gauge reads budget.read_at and liveness from agent-run status; the chunk leaves the sh status path and those fields unchanged and the scope names neither
@answer n=4 entity=metric-coverage#2.1 v=yes
why: the scope keeps the hyperfine max under the 1.0 s hook budgets on the perf job, which is the gate this row names
@answer n=5 entity=metric-coverage#2.2 v=no
why: channel call latency per method reads channel-response durations; the scope names no channel latency value
@answer n=6 entity=metric-coverage#2.3 v=no
why: send readback latency reads send-confirmed lines; send is neither a reached file nor named by the scope
@answer n=7 entity=metric-coverage#2.4 v=no
why: the send terminal-state counter reads send-confirmed and send-refused values the scope does not name
@answer n=8 entity=metric-coverage#2.5 v=no
why: the wait outcome counter reads the channel-response outcome field, which the scope does not name; its refusal details bind through row 3.1
@answer n=9 entity=metric-coverage#2.6 v=no
why: the state-recovered counter reads the chaos heal line, which the chunk neither changes nor the scope names
@answer n=10 entity=metric-coverage#2.7 v=no
why: the parse.skipped gauge reads list --json output the chunk does not touch
@answer n=11 entity=metric-coverage#2.8 v=no
why: parse-rejected counts per parser; the chunk changes no parser and the scope names no parse-rejected line
@answer n=12 entity=metric-coverage#2.9 v=no
why: the compat counters read send-refused unknown and version-rejection values that the chunk neither changes nor the scope names
@answer n=13 entity=metric-coverage#2.10 v=yes
why: the scope names the process-exit line and exit_source kill-fallback on the PTY seam, which must stay unchanged once the viola-pty Windows fallback leaves
@answer n=14 entity=metric-coverage#2.11 v=yes
why: the scope names the zero-panic error budget measured by G2, whose one exemption is the fake-agent forced panic, so the new tests must keep panic lines at zero
@answer n=15 entity=metric-coverage#3.1 v=yes
why: the scope names server-verify-failed, socket-dir-refused and exit 21 on the wait and MCP paths as unchanged on Linux; only the macOS euid-only clause leaves
@answer n=16 entity=metric-coverage#3.2 v=yes
why: the scope names exit 21, instance-unreachable and strict-modes-failed on the wait path as unchanged, which is the refusal this row describes
@answer n=17 entity=metric-coverage#4.0 v=no
why: a report-only quantile snippet over hook-decision samples; the chunk changes no hook-decision line and the gates read max
