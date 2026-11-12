@row_add n=1 row=contract/log-levels/warn area=6-log-coverage rec=1
boundary: log levels
item: warn
promise: WARN carries state-recovered; parse-rejected; hook-decision with a fail-open detail; process-exit with exit 21; channel-response with -32600 or -32602.
@row_add n=2 row=contract/log-levels/info area=6-log-coverage rec=2
boundary: log levels
item: info
promise: INFO carries all other boundary, lifecycle and state-transition events, including refusals, which are normal product outcomes.
@row_add n=3 row=contract/log-levels/debug area=6-log-coverage rec=3
boundary: log levels
item: debug
promise: DEBUG is reserved for per-frame channel framing detail and uses the same field allow-list.
@row_add n=4 row=decision/debug-level-by-config-only area=6-log-coverage rec=3
name: Debug level enabled by config only
rule: The debug log level is enabled only by diagnostics_level "debug" in config.json, never by RUST_LOG.
@row_add n=5 row=contract/log-levels/trace area=6-log-coverage rec=4
boundary: log levels
item: trace
promise: TRACE is never emitted; the level exists only in the tests' level enum.
@row_none n=1 rec=5 type=S
@row_none n=2 rec=6 type=S
@row_none n=3 rec=7 type=S
@row_add n=6 row=decision/viola-crates-log-at-info area=6-log-coverage rec=8,5
name: Viola crates log at INFO
rule: The per-module log filter sets the viola, viola_core, viola_pty, viola_channel, viola_state, viola_agent_claude and viola_mcp targets to INFO, or DEBUG via diagnostics_level.
@row_add n=7 row=decision/third-party-crate-logs-off area=6-log-coverage rec=9,5
name: Third-party crate logs off
rule: The per-module log filter turns logging OFF for the rmcp, portable_pty, sysinfo and interprocess targets.
why: Their failures are captured at the viola seam as viola events.
@row_add n=8 row=decision/unlisted-targets-log-off area=6-log-coverage rec=10,5
name: Unlisted log targets off
rule: The per-module log filter turns logging OFF for every target it does not list.
@row_none n=4 rec=11 type=S
@row_add n=9 row=decision/file-only-per-role-log-sinks area=6-log-coverage rec=12,11
name: File-only log sinks per role
rule: Diagnostics are written to files only, one sink per role, under the home's diagnostics directory.
@row_add n=10 row=decision/content-detail-in-instance-file area=6-log-coverage rec=12,11
name: Content-bearing detail in the instance detail file
rule: Content-bearing detail goes only to the per-process detail file inside the instance's diagnostics directory.
@row_add n=11 row=decision/no-log-rotation area=6-log-coverage rec=13
name: No log rotation
rule: Log files are not rotated in v1; retention is the session home's lifetime.
@row_add n=12 row=decision/json-to-file-only-log-mode area=6-log-coverage rec=14
name: JSON-to-file as the only log mode
rule: JSON to file is the only log mode: there is no agent-mode flag and no stdout pretty sink on any role.
@row_none n=5 rec=15 type=S
@row_add n=13 row=contract/ipc-invocation-log/fields area=6-log-coverage rec=16
boundary: IPC invocation log lines
item: fields
promise: Each channel-request and channel-response line carries method, corr, conn, result class, error code and sender.
@row_add n=14 row=decision/ipc-calls-logged-without-arguments area=6-log-coverage rec=16
name: IPC calls logged without arguments
rule: IPC params and result bodies are never recorded, not even as a hash or digest.
@row_add n=15 row=decision/euid-refused-peer-writes-no-line area=6-log-coverage rec=16
name: Euid-refused peer writes no line
rule: A channel peer refused by euid is dropped before its worker thread and before any frame, writes no log line, and the client reads end-of-file.
why: There is no frame to key a line on, and channel-response's detail is closed.
@row_add n=16 row=contract/pty-seam-log/spawn area=6-log-coverage rec=17
boundary: PTY seam log events
item: spawn
promise: A spawn gives process-start{subject:"claude-child"}.
@row_add n=17 row=contract/pty-seam-log/kill-fallback area=6-log-coverage rec=17
boundary: PTY seam log events
item: kill-fallback
promise: An exit through the kill fallback is logged with exit_source:"kill-fallback".
@row_add n=18 row=contract/pty-seam-log/paste area=6-log-coverage rec=17
boundary: PTY seam log events
item: paste
promise: A paste is logged with text_bytes only.
@row_add n=19 row=contract/child-spawn-log/subject area=6-log-coverage rec=18
boundary: child and shell spawn log events
item: subject
promise: process-start and process-exit lines for child and shell spawns carry a subject from a closed set.
@row_add n=20 row=contract/child-spawn-log/claude-agents-json-parse-failure area=6-log-coverage rec=18
boundary: child and shell spawn log events
item: claude agents --json parse failure
promise: A claude agents --json parse failure gives one parse-rejected{parser:"claude-agents-json"} line and the value "unknown".
@row_add n=21 row=absence/grpc area=6-log-coverage rec=19
capability: gRPC of any kind
@row_add n=22 row=absence/database area=6-log-coverage rec=19
capability: A database
@row_none n=6 rec=20 type=S
@row_none n=7 rec=21 type=S
@row_none n=8 rec=22 type=S
@row_add n=23 row=decision/local-error-capture-only area=7-error-capture-reporting rec=23,26
name: Local error capture only
rule: Errors are captured locally only; there is no external error-reporting platform and no reporter to opt into.
@row_add n=24 row=decision/typed-errors-fixed-display area=7-error-capture-reporting rec=23,33
name: Typed errors with fixed Display messages
rule: Every typed error has a fixed-message Display, written with thiserror, and holds no fields with paths or payloads.
except: PtyError's Display is hand-written, as viola-pty takes no thiserror.
@row_add n=25 row=decision/anyhow-at-dispatch-edge-only area=7-error-capture-reporting rec=23
name: anyhow at the dispatch edge only
rule: anyhow is used only at the root binary's dispatch edge and its catch-site reporter.
@row_add n=26 row=decision/hand-written-panic-capture area=7-error-capture-reporting rec=23
name: Hand-written panic capture
rule: Panic capture is a hand-written std panic hook plus std catch_unwind, with no panic-reporting crate.
@row_add n=27 row=decision/sentry-not-admissible area=7-error-capture-reporting rec=24
name: Sentry not admissible
rule: sentry and sentry-tracing are not admissible.
why: They break the egress ban, and their default reqwest, native-tls and tokio transport breaks the cargo-deny C-crate and Tokio-in-sync-crates bans.
@row_add n=28 row=decision/tracing-panic-rejected area=7-error-capture-reporting rec=25
name: tracing-panic rejected
rule: tracing-panic is not used.
why: The crate is stale.
@row_add n=29 row=decision/human-panic-rejected area=7-error-capture-reporting rec=25
name: human-panic rejected
rule: human-panic is not used.
why: It writes a human report to stderr.
@row_add n=30 row=decision/future-reporter-ureq-rustls area=7-error-capture-reporting rec=27,26
name: Future reporter uses ureq and rustls
rule: Any future error reporter must use the ureq and rustls transport.
@row_add n=31 row=decision/future-reporter-never-log-floor area=7-error-capture-reporting rec=28,26
name: Future reporter applies the NEVER-log floor
rule: Any future error reporter must apply the NEVER-log floor in before_send.
@row_add n=32 row=decision/future-reporter-off-by-default area=7-error-capture-reporting rec=29,26
name: Future reporter off by default
rule: Any future error reporter must be off by default.
@row_add n=33 row=decision/future-reporter-not-env-enabled area=7-error-capture-reporting rec=30,26
name: Future reporter never enabled by env var
rule: Any future error reporter must never be enabled by an environment variable.
@row_add n=34 row=decision/future-reporter-needs-egress-entry area=7-error-capture-reporting rec=31,26
name: Future reporter needs a security egress entry
rule: Any future error reporter also needs a security Decisions Log entry for its egress.
@row_add n=35 row=decision/scrubbing-at-source area=7-error-capture-reporting rec=32
name: Scrubbing at the source
rule: Error scrubbing sits at the source, not in a reporter's before_send hook.
why: No reporter exists.
@row_add n=36 row=decision/serde-errors-become-internal-error area=7-error-capture-reporting rec=34,32
name: Serde error chains replaced by internal error
rule: At dispatch, an error chain whose source is a serde_json or serde_path_to_error error is replaced by the fixed message "internal error" before it reaches stderr, --json, MCP isError or channel error.data.
why: A scrubbing layer at the source.
@row_add n=37 row=decision/full-error-chain-in-detail-file-only area=7-error-capture-reporting rec=35,32
name: Full error chain in the detail file only
rule: The full error chain is written as a one-line JSON array chain:[...] only to the instance's per-process detail file, and only when an instance resolves.
@row_none n=9 rec=36 type=S
@row_add n=38 row=decision/panic-hook-silent-on-stderr area=7-error-capture-reporting rec=37
name: Panic hook installed first, silent on stderr
rule: Viola's panic hook is installed by main's first statement, never calls the default hook and never writes to stderr.
why: The hook contract and the run terminal both forbid it.
@row_add n=39 row=decision/panic-line-bypasses-tracing area=7-error-capture-reporting rec=38
name: Panic line bypasses tracing
rule: The panic hook writes its one JSON line with a single write to a pre-opened file, not through the tracing subscriber.
why: No re-entrancy if the panic happened inside formatting.
@row_add n=40 row=contract/panic-line/keys area=7-error-capture-reporting rec=38
boundary: the panic line
item: keys
promise: The panic line holds timestamp, level "ERROR", target "viola::panic", message "panic", event "panic", process, instance, panic_location and thread.
@row_add n=41 row=contract/panic-line/corr area=7-error-capture-reporting rec=38
boundary: the panic line
item: corr
promise: corr is never written on the panic line.
@row_add n=42 row=contract/panic-line/instance area=7-error-capture-reporting rec=38
boundary: the panic line
item: instance
promise: instance is omitted when none resolved, the same key-absence null encoding obs_event! uses.
@row_add n=43 row=decision/panic-backtrace-in-detail-file area=7-error-capture-reporting rec=39
name: Panic backtrace in the detail file
rule: When an instance resolves, the panic hook also writes the payload and a raw, never-symbolised backtrace as one line to the process's detail file, not gated by RUST_BACKTRACE.
why: Env vars are not a configuration channel.
@row_add n=44 row=decision/panic-exit-decided-at-catch-site area=7-error-capture-reporting rec=40
name: Panic exit decided at a catch site
rule: The panic hook only writes; the exit is decided by one catch_unwind in the root binary's main around everything after the hook is set, so no role ends through an uncaught unwind.
@row_add n=45 row=decision/run-threads-inside-catch-unwind area=7-error-capture-reporting rec=40
name: Run threads inside catch_unwind
rule: The run PTY pump and handle-wait threads each run inside catch_unwind; a worker panic that leaves main running with no process-exit is forbidden.
@row_add n=46 row=decision/panic-unwind-in-all-profiles area=7-error-capture-reporting rec=40
name: panic = unwind in every profile
rule: Every Cargo profile keeps panic = "unwind".
guard: G3
@row_add n=47 row=contract/panic-outcome-by-role/hook area=7-error-capture-reporting rec=41
boundary: panic outcome by role
item: hook
promise: After a panic, hook exits 0 with empty stdout.
@row_add n=48 row=contract/panic-outcome-by-role/cli area=7-error-capture-reporting rec=41
boundary: panic outcome by role
item: cli
promise: After a panic, cli logs process-exit{subject:"self", exit_code:1, detail:"internal-error"} when an instance resolves, puts the chain in the detail file only, writes exactly "error: internal error\n" on stderr in one write with no hint, and exits 1.
@row_add n=49 row=contract/panic-outcome-by-role/cli-json area=7-error-capture-reporting rec=41
boundary: panic outcome by role
item: cli --json
promise: After a panic under --json, cli writes {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty, and exits 1.
@row_add n=50 row=contract/panic-outcome-by-role/run area=7-error-capture-reporting rec=41
boundary: panic outcome by role
item: run
promise: After a panic, run logs process-exit{subject:"self", exit_code:1, detail:"internal-error"} and exits 1, writing nothing on the terminal while the child runs.
@row_add n=51 row=contract/panic-outcome-by-role/mcp-task-panic area=7-error-capture-reporting rec=41
boundary: panic outcome by role
item: mcp spawned-task panic
promise: A panic inside an mcp spawned Tokio task is logged by the hook, contained at the task boundary, and the process continues.
@row_add n=52 row=contract/panic-outcome-by-role/mcp-main-thread-panic area=7-error-capture-reporting rec=41
boundary: panic outcome by role
item: mcp main-thread panic
promise: An mcp main-thread panic writes no process-exit line and nothing to stdout, and the process exits 1.
@row_none n=10 rec=42 type=4
relied: A panic before any role file is opened (clap parsing, home and instance checks) leaves no log record.
@row_none n=11 rec=43 type=S
@row_add n=53 row=decision/unhandled-panics-always-captured area=7-error-capture-reporting rec=44,43
name: Unhandled panics always captured
rule: Every unhandled panic is captured as the one-line event "panic".
@row_add n=54 row=decision/boundary-errors-end-in-logged-line area=7-error-capture-reporting rec=45,43
name: Crate-seam errors end in a logged line
rule: Every typed error that crosses a crate seam ends in a logged process-exit detail, channel-response error, hook-decision detail or send-refused line, carrying codes only at WARN or ERROR.
@row_add n=55 row=decision/vt100-panics-contained area=7-error-capture-reporting rec=46,43
name: vt100 panics contained
rule: vt100 panics are caught around each feed and resize on run's feed thread; each poisoning episode gives exactly one parse-rejected{parser:"vt100-feed"} at WARN, and run continues.
@row_add n=56 row=decision/mcp-rejects-codes-only area=7-error-capture-reporting rec=47
name: MCP rejects are codes only
rule: MCP rejects are isError codes only, and a logged reject's line holds codes only.
@row_add n=57 row=decision/instance-unreachable-not-logged area=7-error-capture-reporting rec=47
name: instance-unreachable logged by no line
rule: An MCP instance-unreachable reject, from any instance, liveness, socket-directory, euid or server-verification check, is logged by no line; the isError code is its only record.
@row_add n=58 row=decision/error-correlation-without-trace-ids area=7-error-capture-reporting rec=48
name: Error correlation without trace ids
rule: Errors correlate by corr, conn, process and instance, plus timestamp, not by trace_id or span_id.
@row_add n=59 row=contract/detail-file-line/correlation-keys area=7-error-capture-reporting rec=48
boundary: the detail-file error line
item: correlation keys
promise: The detail-file line for an error carries the same event, corr and conn as its home-level counterpart, so jq joins them.
@row_add n=60 row=decision/no-sentry-or-otel-bridge area=7-error-capture-reporting rec=48
name: No Sentry or OTel bridge
rule: Errors have no Sentry or OpenTelemetry bridge.
why: Both were deferred or rejected.
@row_none n=12 rec=49 type=S
@row_none n=13 rec=50 type=S
@row_none n=14 rec=51 type=S
@row_none n=15 rec=52 type=S
@row_add n=61 row=decision/secrets-never-logged area=8-pii-scrubbing-compliance rec=53
name: Secrets never logged
rule: The Cookie header, signing credentials and R8-stripped CLAUDE* values are never logged.
@row_add n=62 row=decision/ci-secrets-not-in-artifacts area=8-pii-scrubbing-compliance rec=53
name: CI secrets never in artifacts
rule: CI secrets are never echoed into artifacts.
@row_add n=63 row=decision/conversation-text-only-in-detail-file area=8-pii-scrubbing-compliance rec=53
name: Conversation text only in the instance detail file
rule: Prompt, assistant, tool-input, plan and dialog text never reach a home-level line, only an instance's per-process detail file inside a chain or drift report.
@row_add n=64 row=decision/statusline-command-out-of-home-logs area=8-pii-scrubbing-compliance rec=53
name: statusline_command out of home-level logs
rule: statusline_command stays out of home-level logs.
@row_add n=65 row=decision/home-level-lines-path-free area=8-pii-scrubbing-compliance rec=54
name: Home-level lines path-free
rule: Home-level log lines carry no paths.
@row_add n=66 row=decision/endpoint-and-pinned-bin-never-logged area=8-pii-scrubbing-compliance rec=54
name: endpoint and pinned_bin never logged
rule: The endpoint and pinned_bin values are never logged.
@row_add n=67 row=decision/claude-agents-rows-never-logged area=8-pii-scrubbing-compliance rec=54
name: claude agents rows never logged verbatim
rule: claude agents --json rows are never logged verbatim, only parse-rejected counts.
@row_add n=68 row=decision/low-class-metadata-as-typed-fields area=8-pii-scrubbing-compliance rec=54
name: Low-class metadata as typed fields
rule: Low-class metadata is logged as typed fields, never inside detail codes.
@row_none n=16 rec=55 type=S
