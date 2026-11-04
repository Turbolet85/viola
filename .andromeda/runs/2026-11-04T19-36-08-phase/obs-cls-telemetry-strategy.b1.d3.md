@answer n=1 entity=telemetry-strategy#1.1 v=yes
why: the scope names the openpty seam (spawn, write, wait, kill) and says it is instrumented at the seam only, so these seam spans bind
@answer n=2 entity=telemetry-strategy#1.2 v=yes
why: process-log lines and the closed event enum are the one diagnostics schema the scope re-asserts on every OS
@answer n=3 entity=telemetry-strategy#1.3 v=yes
why: the panic record and catch_unwind on the PTY pump and handle-wait threads are scope item 10
@answer n=4 entity=telemetry-strategy#1.4 v=yes
why: the per-OS perf job with hyperfine export-json gates is re-read and must be green on Unix
@answer n=5 entity=telemetry-strategy#1.5 v=yes
why: events.ndjson is named in the one-schema rule and must carry the same fields on every OS
@answer n=6 entity=telemetry-strategy#2.1 v=yes
why: the scope's panic rule (one single-line event panic record in the home-level role file, none on stderr) is this record's rule
@answer n=7 entity=telemetry-strategy#3.1 v=yes
why: the Unix channel endpoint telemetry (channel-request and channel-response) is IPC logging under this trigger's field allow-list
@answer n=8 entity=telemetry-strategy#3.2 v=yes
why: scope item 2 logs paste as text_bytes only, which is this vector's rule
@answer n=9 entity=telemetry-strategy#3.3 v=yes
why: the hook contract, the silent no-op without VIOLA_NAME, and the socket-dir-refused and server-verify-failed hook-decision details are scope items 5, 6 and 19
@answer n=10 entity=telemetry-strategy#3.4 v=yes
why: scope item 6 gives MCP's macOS instance-unreachable return with no line
@answer n=11 entity=telemetry-strategy#3.5 v=yes
why: run's pre-child fixed two-line stderr pair (scope item 8) is this CLI and env vector's fixed-message rule
@answer n=12 entity=telemetry-strategy#3.6 v=yes
why: scope items 8, 15, 20 and 21 name the file modes, home-tree refusals, state-recovered detail words and the secret scan over diagnostics files
@answer n=13 entity=telemetry-strategy#3.7 v=yes
why: process-start with subject claude-child is scope item 2
@answer n=14 entity=telemetry-strategy#3.8 v=yes
why: the cargo-deny gates (no Tokio in the sync crates, no C-building crates) are scope item 17
@answer n=15 entity=telemetry-strategy#3.9 v=yes
why: channel-request and channel-response must carry corr and conn on both sides, scope item 7
@answer n=16 entity=telemetry-strategy#3.10 v=yes
why: the hyperfine perf gates over the five hook rows, with the VIOLA_NAME and e2e-home condition, are scope item 16
@answer n=17 entity=telemetry-strategy#3.11 v=yes
why: process-exit from the handle-wait thread and exit_source kill-fallback are scope item 2
@answer n=18 entity=telemetry-strategy#3.12 v=yes
why: scope item 3 names the same file sink and line schema on every OS
@answer n=19 entity=telemetry-strategy#3.13 v=yes
why: zero unlogged panics, zero secret-scan hits and zero schema failures are scope item 11
@answer n=20 entity=telemetry-strategy#3.14 v=no
why: the scope's not_needed item 8 drops rotation and retention for v1 and the chunk adds no rotation, so no code or test acts on it
@answer n=21 entity=telemetry-strategy#3.15 v=yes
why: the section 3 schema verbatim is the one diagnostics schema the scope re-asserts on every OS, scope item 5
@answer n=22 entity=telemetry-strategy#3.16 v=yes
why: the closed detail catalog and the strict-modes and socket-dir-refused refusals with their exit codes are scope items 5, 8, 9 and 21
@answer n=23 entity=telemetry-strategy#3.17 v=yes
why: the per-OS perf job re-measures the installed CLI (scope item 16), and this trigger states the basis of the counters it reads
@answer n=24 entity=telemetry-strategy#3.18 v=yes
why: the PTY byte stream is never logged and the run pump feeds vt100, so no screen content may reach telemetry, scope item 1
