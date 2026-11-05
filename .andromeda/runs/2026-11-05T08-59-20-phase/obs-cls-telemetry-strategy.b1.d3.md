@answer n=1 entity=telemetry-strategy#1.1 v=no
why: tracing spans are declared in code but never serialized, and no span on the Linux seams is named by the scope.

@answer n=2 entity=telemetry-strategy#1.2 v=yes
why: the closed event enum and every Linux boundary, state and process-lifecycle log line are kept fixed by the scope's Linux log contract.

@answer n=3 entity=telemetry-strategy#1.3 v=yes
why: the panic record, typed errors, process-exit detail codes and catch_unwind sites on the PTY pump and handle-wait threads are named by the scope's exit set and zero-unlogged-panics invariant.

@answer n=4 entity=telemetry-strategy#1.4 v=yes
why: the hyperfine --export-json perf gates are the measurement the scope keeps on the remaining perf job.

@answer n=5 entity=telemetry-strategy#1.5 v=no
why: events.ndjson rotation and retention are the planner's not-needed item, and the chunk writes no audit-stream change.

@answer n=6 entity=telemetry-strategy#2.1 v=yes
why: the zero-unlogged-panics invariant and the one-line panic record stand as they are, and the scope names that invariant.

@answer n=7 entity=telemetry-strategy#3.1 v=no
why: the channel IPC vector covers the channel wire, which the removal leaves unchanged.

@answer n=8 entity=telemetry-strategy#3.2 v=no
why: the paste-text send-refused path is not among the Windows-only or Linux-changed paths the scope names.

@answer n=9 entity=telemetry-strategy#3.3 v=yes
why: the hook contract, the silent no-op without VIOLA_NAME and the Linux server-verify-failed WARN are named by the scope, and the macOS euid-only carve-out leaves with its arm.

@answer n=10 entity=telemetry-strategy#3.4 v=yes
why: the MCP server-verification paths and their Linux details are named by the scope, and the macOS carve-out leaves with its arm.

@answer n=11 entity=telemetry-strategy#3.5 v=yes
why: the fixed CLI messages and env-stripped scenario carry the run-start refusal pair the scope keeps, and the .cmd/.bat refusal text leaves.

@answer n=12 entity=telemetry-strategy#3.6 v=yes
why: the strict-modes refusals, the 0600/0700 owner-only checks, and the process-start sideload_fallback companion are named by the scope, which removes the conpty/ sideload.

@answer n=13 entity=telemetry-strategy#3.7 v=yes
why: the child-spawn subject values include the conpty-sideload and conpty closed words the scope removes, and the claude child's process-start stays fixed.

@answer n=14 entity=telemetry-strategy#3.8 v=yes
why: the cargo-deny gates run after the Windows target dependencies and their deny.toml entries leave.

@answer n=15 entity=telemetry-strategy#3.9 v=no
why: the planner's not-needed item holds, since the removal leaves the channel wire and its corr and conn join key unchanged.

@answer n=16 entity=telemetry-strategy#3.10 v=yes
why: the perf budgets are named by the scope, which keeps the five hook-row run --perf timing under the 1.0 s budgets.

@answer n=17 entity=telemetry-strategy#3.11 v=yes
why: the process-exit exit_source kill-fallback on the PTY seam is named by the scope as unchanged once the Windows TerminateProcess fallback leaves.

@answer n=18 entity=telemetry-strategy#3.12 v=yes
why: the scope names the multi-platform-exporter-compat trigger, and the concurrent-append check and per-OS artifacts leave with the Windows and macOS jobs.

@answer n=19 entity=telemetry-strategy#3.13 v=yes
why: the error budget (zero panic lines, zero schema failures, zero secret-scan hits) is named by the scope's error-budget measurement.

@answer n=20 entity=telemetry-strategy#3.14 v=no
why: log rotation and the absence of it for diagnostics and events files is the planner's not-needed item, and the chunk changes none of it.

@answer n=21 entity=telemetry-strategy#3.15 v=yes
why: the diag-line and diag-detail schemas keep every Linux value, which the scope names.

@answer n=22 entity=telemetry-strategy#3.16 v=yes
why: the strict-modes-failed, socket-dir-refused and instance-unreachable details and exit 21 are named by the scope as unchanged on Linux.

@answer n=23 entity=telemetry-strategy#3.17 v=yes
why: the installed-CLI measured counters are the perf job's measurements that the scope keeps on the Linux leg.

@answer n=24 entity=telemetry-strategy#3.18 v=no
why: the vt100 gate outcome on the send scenario is not named by the scope, and the chunk changes no screen read on Linux.
