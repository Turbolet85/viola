@answer n=1 entity=metric-coverage#1.1 v=no
why: the counter groups send-refused by side and detail, and the session changes neither field nor any send-refused value
@answer n=2 entity=metric-coverage#1.2 v=no
why: the histogram takes raw duration_ms samples per event, which the session does not change
@answer n=3 entity=metric-coverage#1.3 v=no
why: the gauge reads the budget.read_at snapshot and agent-run status liveness, and the session leaves the heartbeat and harness status alone
@answer n=4 entity=metric-coverage#2.1 v=no
why: hook latency samples and the hyperfine max gate are untouched, since the session keeps the perf budgets
@answer n=5 entity=metric-coverage#2.2 v=no
why: channel-response duration samples are unchanged, the session moves only the socket dir forms
@answer n=6 entity=metric-coverage#2.3 v=no
why: the readback latency cites src/run/send.rs and its GATE_MAX_WAIT, and the session does not reach send.rs
@answer n=7 entity=metric-coverage#2.4 v=no
why: the send-confirmed and send-refused terminal counts and their design words are not named or changed by the session
@answer n=8 entity=metric-coverage#2.5 v=no
why: the wait outcome field and its counter are unchanged, since only the server verdict inputs move
@answer n=9 entity=metric-coverage#2.6 v=no
why: the state-recovered detail and chaos heal are not named, and the viola-state change retires only the Windows owner and DACL arm
@answer n=10 entity=metric-coverage#2.7 v=no
why: the list --json skipped gauge is not named or changed by the session
@answer n=11 entity=metric-coverage#2.8 v=no
why: the parse-rejected counter per parser is not named or changed by the session
@answer n=12 entity=metric-coverage#2.9 v=no
why: the unknown, unsupported-version and invalid-params detail names are not named or changed by the session
@answer n=13 entity=metric-coverage#2.10 v=no
why: the child exit counter names no detail or kill-fallback value, and the session keeps kill-fallback on the handle wait
@answer n=14 entity=metric-coverage#2.11 v=no
why: the panic event name and its must-be-zero SLO are not named or changed by the session
@answer n=15 entity=metric-coverage#3.1 v=yes mode=contradict
why: the server-verification refusal is pinned to Linux, Windows and macOS with an euid-only reading, and the session retires Windows and the macOS euid-only carve-out
@answer n=16 entity=metric-coverage#3.2 v=yes mode=stale
why: the session moves the error-capture rule on MCP instance-unreachable in tools.rs and client.rs, which this no-line record of the check_instance refusal describes
@answer n=17 entity=metric-coverage#4.0 v=no
why: the canonical hook-decision quantile snippet reads only hook-decision duration samples and diagnostics file globs, which the session leaves unchanged
