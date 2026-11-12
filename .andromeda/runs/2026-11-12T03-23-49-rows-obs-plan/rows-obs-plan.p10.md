@row_none n=1 rec=1 type=S
@row_none n=2 rec=2 type=0
what: an instrumentability label naming no entity and no reason
@row_add n=1 row=decision/hook-mcp-instrumented-at-boundary area=obs-scope-summary rec=3
name: Hook and MCP instrumented at the boundary
rule: Hook and MCP traffic is instrumented only at viola's own boundary: hook-invoked and hook-decision, MCP tool-call handling, and the viola mcp stdin cut.
why: The caller is Claude Code, so viola sees only the inbound stdin payload and its own response, and no trace context arrives from Claude Code.
@row_add n=2 row=contract/mcp-stdin/oversize-line area=obs-scope-summary rec=3,44
boundary: viola mcp stdin
item: oversize line
promise: A line longer than MAX_FRAME is refused whole, never truncated or skipped, ends the session as a closed stdin does with stderr empty, and its one record is parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1} carrying no byte of the line.
@row_none n=3 rec=4 type=S
@row_none n=4 rec=5 type=0
what: an instrumentability label naming no entity and no reason
@row_add n=3 row=decision/agents-cli-instrumented-at-boundary area=obs-scope-summary rec=6,4
name: claude agents CLI instrumented at spawn, exit and parse
rule: The external `claude agents --json` CLI is instrumented only at its spawn, its exit and its parse outcome.
why: An external CLI.
@row_add n=4 row=decision/agents-cli-parse-failure-unknown area=obs-scope-summary rec=6,4
name: claude agents parse failure reads unknown
rule: A failed parse of `claude agents --json` output reads "unknown" plus a counter.
@row_add n=5 row=decision/agents-cli-output-untrusted area=obs-scope-summary rec=6,4
name: claude agents output untrusted
rule: The output rows of `claude agents --json` are treated as untrusted.
@row_none n=5 rec=7 type=S
@row_none n=6 rec=8 type=0
what: an instrumentability label naming no entity and no reason
@row_add n=6 row=decision/statusline-shell-out-instrumentation area=obs-scope-summary rec=9,7
name: Status-line shell-out instrumented by duration and exit
rule: The `viola hook statusline` shell-out of the user's command is instrumented by its duration and exit status only.
why: A user-written shell command.
@row_add n=7 row=decision/statusline-command-only-in-diagnostics area=obs-scope-summary rec=9
name: Status-line command string kept to diagnostics
rule: The statusline_command string is never logged outside diagnostics/.
@row_none n=7 rec=10 type=S
@row_none n=8 rec=11 type=0
what: an instrumentability label naming no entity and no reason
@row_add n=8 row=decision/instrument-call-sites-not-os-internals area=obs-scope-summary rec=12
name: OS operations instrumented at viola's call sites
rule: OS-level operations (bind, connect, lock, rename, kill fallback, the server check's peer reads, the strict-modes stat reads, the socket directory check) are instrumented at viola's own call sites, not inside the OS.
@row_add n=9 row=contract/exit-cause-detail/server-verify-failed area=obs-scope-summary rec=12
boundary: Exit cause detail codes
item: server-verify-failed
promise: A failed server check of peer euid, peer pid or start time is logged by the CLI and the hook as server-verify-failed; MCP writes no line for it.
@row_add n=10 row=contract/exit-cause-detail/socket-dir-refused area=obs-scope-summary rec=12,26,53
boundary: Exit cause detail codes
item: socket-dir-refused
promise: A refused socket directory has its own detail code socket-dir-refused and writes its line (MCP writes none); on run and on a start made by viola revive that line is one process-exit{subject:"self", exit_code:1} before any snapshot, child or bind.
@row_add n=11 row=decision/refused-peer-dropped-without-line area=obs-scope-summary rec=12,41
name: Refused channel peer dropped without a log line
rule: A channel stream whose peer euid is refused is dropped on the accept thread before its worker thread and before any frame: the peer reads end-of-file, and the wrapper writes no log line and nothing on the terminal.
why: There is no frame to key a line on, and channel-response's detail is closed.
@row_none n=9 rec=13 type=S
@row_none n=10 rec=14 type=0
what: an instrumentability label for the persistent stores
@row_add n=12 row=decision/event-payload-not-in-process-logs area=obs-scope-summary rec=15
name: Event payloads kept out of process logs
rule: The payload content of events.ndjson is never copied into process-log lines.
why: events.ndjson is itself the product audit stream.
@row_add n=13 row=decision/absolute-paths-scrubbed-externally area=obs-scope-summary rec=15
name: Absolute paths scrubbed from external output
rule: Absolute paths in snapshot.json (endpoint, pinned_bin) and in config values are scrubbed from external output and allowed in diagnostics/.
@row_add n=14 row=contract/replayed-snapshot/live-fields area=obs-scope-summary rec=15
boundary: A snapshot.json rebuilt by replay
item: endpoint, pid, started_at, statusline_command
promise: A snapshot.json rebuilt from events.ndjson carries none of these, so the endpoint reads unreachable, hook connect fails open with channel-unreachable, hook statusline prints nothing, and run's collision check reads the heartbeat alone.
@row_add n=15 row=decision/budget-metadata-loggable area=obs-scope-summary rec=15
name: Budget metadata loggable as fields
rule: Operational metadata in budget.json (percentages, resets_at) may be logged as structured fields.
@row_none n=11 rec=16 type=S
@row_none n=12 rec=17 type=0
what: an instrumentability label naming no entity and no reason
@row_add n=16 row=decision/config-observed-at-load area=obs-scope-summary rec=18
name: Configuration observed only at load
rule: Configuration is observed only at its parse at load time, logged as skipped-key counts.
why: Static configuration and data.
@row_none n=13 rec=19 type=S
@row_none n=14 rec=20 type=0
what: an instrumentability label naming no entity and no reason
@row_add n=17 row=decision/fixtures-not-telemetry area=obs-scope-summary rec=21
name: Recorded fixtures are no telemetry source
rule: Recorded fixtures are scrub-required data, not a telemetry source, and are excluded from the observability plan.
why: Not product runtime.
@row_add n=18 row=decision/tracing-json-logging-pipeline area=obs-scope-summary rec=22
name: tracing with a JSON subscriber for logging
rule: Logging and spans go through the Rust tracing facade with a tracing-subscriber JSON structured logger.
@row_add n=19 row=decision/panic-hook-replaced area=obs-scope-summary rec=22,31,47
name: Panic hook replaced by a one-line writer
rule: Each process replaces the default panic hook, as the first statement in main and before any child spawns, with one that writes one JSON line and nothing to stderr.
why: The default hook prints to stderr, which breaks the hook contract and is the child's terminal under run.
@row_add n=20 row=absence/interactive-terminal-interface area=obs-scope-summary rec=23
capability: An interactive terminal interface: prompt, TUI or pager
@row_add n=21 row=decision/process-logs-file-sink-only area=obs-scope-summary rec=24,48
name: Process logs to a file sink only
rule: Process logs are written only as JSON lines to one file per process role under diagnostics/; there is no network exporter and no stdout sink.
why: viola makes no outbound network calls, and stdout carries --json results.
@row_add n=22 row=decision/no-process-log-without-instance area=obs-scope-summary rec=24
name: No process log without an instance
rule: A short-lived CLI verb with no resolved instance writes no process-log file, and its human stderr carries only fixed-message lines.
@row_add n=23 row=decision/strict-modes-refusal-writes-no-line area=obs-scope-summary rec=24,26,47,53
name: Strict-modes refusal writes no line
rule: A strict-modes check refused before any log opens ends the process with nothing under the home opened and no log line; its cause is carried only by the fixed stderr pair or the --json document.
@row_add n=24 row=contract/cli-log-file/revive-list area=obs-scope-summary rec=24
boundary: CLI process-log file
item: viola revive --list
promise: Logs as cli into the caller's cli-<VIOLA_NAME>.ndjson once its strict-modes check has passed; no file with no VIOLA_NAME.
@row_add n=25 row=contract/cli-log-file/link-unlink area=obs-scope-summary rec=24
boundary: CLI process-log file
item: viola link / viola unlink
promise: Log as cli into the driven instance's cli-<driven>.ndjson, never the caller's VIOLA_NAME, once their instance check of it has passed.
@row_add n=26 row=contract/cli-log-file/revive-start area=obs-scope-summary rec=24
boundary: CLI process-log file
item: viola revive start arm
promise: Logs as the wrapper, process="run", into diagnostics/run-<name>.ndjson.
@row_add n=27 row=contract/cli-json-error/strict-modes-failed area=obs-scope-summary rec=24,26
boundary: CLI --json error document
item: instance-unreachable strict-modes-failed
promise: A refused instance check of send, wait, last, answer, pause, release, link or unlink exits 21 with the instance pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
@row_add n=28 row=contract/viola-verify/home-tree-refused area=obs-scope-summary rec=24
boundary: viola verify
item: refused home-tree check
promise: A refused home-tree check, or a refused update_stamps, exits 1 with the home pair, opens no log and writes no line.
@row_add n=29 row=contract/service-identity/service-name area=obs-scope-summary rec=25
boundary: Service identity
item: service.name
promise: Hardcoded "viola", the binary name.
@row_add n=30 row=contract/service-identity/service-version area=obs-scope-summary rec=25
boundary: Service identity
item: service.version
promise: The compile-time CARGO_PKG_VERSION, the same value used as sender and writer.
@row_add n=31 row=absence/deployment-environments area=obs-scope-summary rec=25
capability: Deployment environments or hosting
@row_add n=32 row=decision/instance-from-name-not-cwd area=obs-scope-summary rec=26,25
name: Instance from its name, never the working directory
rule: The instance comes from VIOLA_NAME or an explicit argument, never from the working directory.
@row_add n=33 row=decision/fixed-stderr-messages area=obs-scope-summary rec=26
name: Fixed stderr refusal and fault lines
rule: Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
@row_add n=34 row=contract/cli-internal-error/stderr-line area=obs-scope-summary rec=26
boundary: CLI internal error
item: stderr line
promise: Without --json a catch-site fault prints the fixed line `error: internal error` to stderr.
@row_add n=35 row=contract/cli-internal-error/json-document area=obs-scope-summary rec=26
boundary: CLI internal error
item: --json document
promise: Under --json, read among the arguments before a --, a catch-site fault leaves stderr empty and prints {"v":1,"error":"internal-error","detail":null} to stdout.
@row_add n=36 row=decision/exit-code-log-field area=obs-scope-summary rec=26
name: Exit code as a log field
rule: The exit code (0, 1, 2, 10–14, 20, 21), the machine-readable outcome, is carried as a log field.
@row_add n=37 row=decision/distinct-detail-code-per-exit-cause area=obs-scope-summary rec=26
name: A distinct detail code for each exit cause
rule: Each cause behind exit 21 and exit 1 gets a distinct detail code.
except: A strict-modes check refused before any log opens writes no line and has no code.
@row_add n=38 row=decision/sync-instrumentation-no-tokio area=obs-scope-summary rec=27,47
name: Sync instrumentation without Tokio
rule: Instrumentation in the sync crates runs on std threads with no Tokio.
why: Hybrid concurrency.
guard: cargo deny ban
@row_none n=15 rec=28 type=0
what: a not-applicable frontend line
@row_add n=39 row=decision/run-silent-on-terminal area=obs-scope-summary rec=29,28
name: run silent on the terminal
rule: run writes nothing to the terminal while the child runs; its process log goes only to diagnostics/run-<name>.ndjson.
why: The terminal carries the child's screen.
@row_add n=40 row=contract/process-log/process area=obs-scope-summary rec=30,35,45,49
boundary: Process log line
item: process
promise: The process role, one of run, hook, mcp, cli.
@row_add n=41 row=contract/heartbeat-file/tick area=obs-scope-summary rec=31
boundary: Heartbeat file
item: tick
promise: The heartbeat file is touched every 1 s as the liveness tick.
@row_add n=42 row=decision/child-exit-from-handle-wait area=obs-scope-summary rec=31
name: Child exit from the process handle
rule: The child's process-exit comes from the wait on its process handle, never from reader end-of-file.
why: Child exit is detected on the process handle.
@row_add n=43 row=decision/hook-panic-one-line area=obs-scope-summary rec=32,49
name: Hook panic logged as one line
rule: A panic in a hook is caught with catch_unwind and logged as one level:"ERROR", event:"panic" line.
@row_add n=44 row=decision/hook-lines-required area=obs-scope-summary rec=32
name: Hook invocation and decision lines required
rule: Every hook process writes its hook-invoked and hook-decision lines.
@row_none n=16 rec=33 type=0
what: a not-applicable frontend line
@row_add n=45 row=decision/hook-never-writes-stderr area=obs-scope-summary rec=34,49
name: Hook never writes stderr
rule: A hook process never writes to stderr.
@row_add n=46 row=contract/hook-stdout/hook-event area=obs-scope-summary rec=34
boundary: Hook stdout
item: viola hook <event>
promise: stdout carries only the decision body.
@row_add n=47 row=contract/hook-stdout/statusline area=obs-scope-summary rec=34
boundary: Hook stdout
item: viola hook statusline
promise: stdout carries only the user's status-line command's output.
@row_add n=48 row=contract/hook-stdout/capture-arms area=obs-scope-summary rec=34
boundary: Hook stdout
item: --capture arms
promise: The two hidden --capture arms write no log sink and print nothing, except --answers' probe decision body.
@row_add n=49 row=decision/hook-log-failure-swallowed area=obs-scope-summary rec=34,49
name: Hook log write failure swallowed
rule: A failed log write in a hook is swallowed and the hook still exits 0.
@row_add n=50 row=decision/hook-silent-without-trusted-instance area=obs-scope-summary rec=35,36
name: Hook silent without a trusted instance
rule: A hook with no VIOLA_NAME, or whose instance is absent or refused by its strict-modes check, exits 0 with no config read, no stdin read, no log line, no stderr, no frame and no SessionEnd append.
@row_add n=51 row=decision/hook-instance-check-before-obs-init area=obs-scope-summary rec=35
name: Hook instance check before logging starts
rule: The hook's event and dialog paths run the instance strict-modes check before reading the diagnostics level and initialising logging.
except: The two --capture arms run ahead of the check.
@row_add n=52 row=decision/one-write-per-log-line area=obs-scope-summary rec=36,48,49
name: One write call per log line
rule: Each formatted log line is emitted as a single write call on an append-mode handle.
why: Many hook processes append to the same file at once.
@row_add n=53 row=decision/hook-latency-budget area=obs-scope-summary rec=36
name: Hook latency budget
rule: Each hook runs within max < 1.0 s, and logger init adds no blocking I/O beyond one file open and append.
guard: run --perf hook rows under SPINE_DEADLINE
except: viola hook statusline is outside the budget.
@row_add n=54 row=decision/statusline-deadline area=obs-scope-summary rec=36
name: Status-line shell-out deadline
rule: The status-line shell-out of the user's command is bounded by STATUSLINE_DEADLINE, 5 s.
state: provisional
@row_add n=55 row=decision/correlation-in-corr-field area=obs-scope-summary rec=37
name: Correlation in the corr log field
rule: Correlation across a channel call travels in the corr log field; channel params declare no trace-context field.
@row_none n=17 rec=38 type=0
what: a not-applicable frontend line
@row_none n=18 rec=39 type=0
what: which log file each side of a channel call writes to
@row_add n=56 row=contract/channel-params/sender area=obs-scope-summary rec=40
boundary: Channel params
item: sender
promise: params.sender carries the peer's version, CARGO_PKG_VERSION.
@row_add n=57 row=contract/json-rpc-frame/id area=obs-scope-summary rec=41
boundary: JSON-RPC frame
item: id
promise: Ids are integers, monotonic per connection, and so not unique across connections or processes.
@row_add n=58 row=contract/json-rpc-error/32603 area=obs-scope-summary rec=41
boundary: JSON-RPC error
item: -32603
promise: The -32603 error carries the message "internal error" with data null.
@row_add n=59 row=decision/from-not-authenticated area=obs-scope-summary rec=41
name: from never logged as authenticated
rule: The self-reported from field is never logged as an authenticated identity.
@row_none n=19 rec=42 type=4
@row_none n=20 rec=43 type=0
what: a not-applicable frontend line
@row_add n=60 row=decision/mcp-log-sink area=obs-scope-summary rec=44,46,48,50
name: MCP log sink
rule: viola mcp always logs to <home>/diagnostics/mcp.ndjson once the home and its diagnostics/ pass strict-modes, with stderr JSON only when that file cannot be opened.
@row_add n=61 row=decision/subscriber-explicit-writer area=obs-scope-summary rec=44,47
name: Subscriber installed with an explicit writer
rule: The tracing subscriber is always installed with an explicit writer, never stdout.
why: tracing-subscriber fmt() writes to stdout unless a writer is configured, and stdout carries MCP frames.
@row_add n=62 row=contract/mcp-stdin/line-within-max-frame area=obs-scope-summary rec=44
boundary: viola mcp stdin
item: line within MAX_FRAME
promise: A line of at most MAX_FRAME bytes, its newline counted, passes whole.
@row_add n=63 row=contract/mcp-stdin/oversize-before-initialize area=obs-scope-summary rec=44
boundary: viola mcp stdin
item: oversize line before initialize
promise: An oversize line before initialize exits the process 1 with both streams empty and the one parse-rejected record.
@row_add n=64 row=contract/mcp-server/instance area=obs-scope-summary rec=45
boundary: MCP server identity
item: instance
promise: The server's own VIOLA_NAME when set, which is also the from it adds.
@row_add n=65 row=decision/mcp-results-codes-only area=obs-scope-summary rec=46
name: MCP rejections carry codes only
rule: MCP isError results and the log lines of rejected calls carry codes only.
@row_add n=66 row=contract/mcp-iserror/instance-unreachable area=obs-scope-summary rec=46
boundary: MCP isError result
item: instance-unreachable
promise: The instance-unreachable detail is one closed cause: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed or socket-dir-refused; the CLI's document less v, its text `refused: instance-unreachable`.
@row_add n=67 row=contract/mcp-iserror/invalid-params area=obs-scope-summary rec=46
boundary: MCP isError result
item: invalid-params
promise: The invalid-params detail is a closed field word.
@row_add n=68 row=contract/mcp-iserror/refusal area=obs-scope-summary rec=46
boundary: MCP isError result
item: refusal
promise: A peer refusal that is no code word reads unknown, and a string detail that is no code word reads null.
@row_add n=69 row=contract/mcp-iserror/wrapper-fault area=obs-scope-summary rec=46
boundary: MCP isError result
item: wrapper-fault
promise: The wrapper-fault detail is a closed sanitised object: the fixed message of the five codes with data null or closed values; a code outside the five reads message "wrapper fault", data null.
@row_add n=70 row=decision/no-otel-exporter area=obs-scope-summary rec=47
name: No OTel SDK exporter
rule: v1 initialises no OTel SDK exporter; the tracing facade plus the tracing-subscriber JSON formatter is the binding pipeline.
why: No outbound network calls, a new listener needs a security decision, stdout is reserved on hook, mcp and run, and cargo deny bans Tokio and C-building crates.
except: An optional OTel bridge layer may feed only an in-process or file sink and stays Tokio-free on the sync crates.
@row_add n=71 row=decision/process-init-order area=obs-scope-summary rec=47
name: Process logging init order
rule: Each process installs its panic hook, parses arguments, resolves home and instance and runs its strict-modes check, opens its diagnostics file append-mode 0600 in a 0700 directory, installs the subscriber, then emits process-start.
why: For run this completes before the start sequence so the collision check and version gate are logged; for mcp, before the stdio transport starts.
@row_add n=72 row=decision/tracing-subscriber-version-floor area=obs-scope-summary rec=48
name: tracing-subscriber version floor
rule: tracing-subscriber is held at version 0.3.20 or later.
why: The security floor for RUSTSEC-2025-0055.
@row_add n=73 row=decision/log-level-not-from-env area=obs-scope-summary rec=48
name: Log level never from the environment
rule: The log level filter is never read from RUST_LOG; any verbosity control comes from config.json or a CLI flag and never bypasses redaction.
why: Environment variables are not a configuration channel, and no env, flag or config value may disable a control.
@row_add n=74 row=decision/json-to-file-only-mode area=obs-scope-summary rec=48
name: JSON-to-file the only logging mode
rule: JSON-to-file is the only logging mode; there is no separate agent-mode flag.
@row_add n=75 row=contract/process-log/timestamp area=obs-scope-summary rec=48,49
boundary: Process log line
item: timestamp
promise: RFC 3339 UTC with milliseconds and Z.
@row_add n=76 row=contract/event-line/shape area=obs-scope-summary rec=49
boundary: events.ndjson event line
item: line shape
promise: Exactly {"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}.
@row_add n=77 row=contract/process-log/level area=obs-scope-summary rec=49
boundary: Process log line
item: level
promise: One of DEBUG, INFO, WARN, ERROR.
@row_add n=78 row=contract/process-log/target area=obs-scope-summary rec=49
boundary: Process log line
item: target
promise: A required field.
@row_add n=79 row=contract/process-log/message area=obs-scope-summary rec=49
boundary: Process log line
item: message
promise: A required field, as fields.message or the flattened message.
@row_add n=80 row=contract/process-log/event area=obs-scope-summary rec=49
boundary: Process log line
item: event
promise: A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
@row_add n=81 row=contract/process-log/instance area=obs-scope-summary rec=49
boundary: Process log line
item: instance
promise: A ViolaName or null.
@row_add n=82 row=contract/process-log/corr area=obs-scope-summary rec=49
boundary: Process log line
item: corr
promise: Copied unchanged as a JSON number or string, never renamed.
@row_add n=83 row=contract/process-log-corr/channel-request area=obs-scope-summary rec=49
boundary: Process log corr by event
item: channel-request
promise: corr is the JSON-RPC request id.
@row_add n=84 row=contract/process-log-corr/channel-response area=obs-scope-summary rec=49
boundary: Process log corr by event
item: channel-response
promise: corr is the JSON-RPC request id.
@row_add n=85 row=contract/process-log-corr/dialog-raised area=obs-scope-summary rec=49
boundary: Process log corr by event
item: dialog-raised
promise: corr is the dialog_id.
@row_add n=86 row=contract/process-log-corr/dialog-answered area=obs-scope-summary rec=49
boundary: Process log corr by event
item: dialog-answered
promise: corr is the dialog_id.
@row_add n=87 row=contract/process-log-corr/hook-invoked area=obs-scope-summary rec=49
boundary: Process log corr by event
item: hook-invoked
promise: corr is the dialog_id for dialog hooks, otherwise null.
@row_add n=88 row=contract/process-log-corr/hook-decision area=obs-scope-summary rec=49
boundary: Process log corr by event
item: hook-decision
promise: corr is the dialog_id for dialog hooks, otherwise null.
@row_add n=89 row=contract/process-log-corr/send-issued area=obs-scope-summary rec=49
boundary: Process log corr by event
item: send-issued
promise: corr is the send cursor.
@row_add n=90 row=contract/process-log-corr/send-confirmed area=obs-scope-summary rec=49
boundary: Process log corr by event
item: send-confirmed
promise: corr is the send cursor.
@row_add n=91 row=contract/process-log-corr/send-refused area=obs-scope-summary rec=49
boundary: Process log corr by event
item: send-refused
promise: corr is the send cursor.
@row_add n=92 row=contract/process-log-corr/process-start area=obs-scope-summary rec=49
boundary: Process log corr by event
item: process-start
promise: corr is null.
@row_add n=93 row=contract/process-log-corr/process-exit area=obs-scope-summary rec=49
boundary: Process log corr by event
item: process-exit
promise: corr is null.
@row_add n=94 row=contract/process-log-corr/panic area=obs-scope-summary rec=49
boundary: Process log corr by event
item: panic
promise: corr is null.
@row_add n=95 row=decision/log-fields-additive-only area=obs-scope-summary rec=49
name: Required log fields only added to
rule: Fields may be added to process-log lines, but the required ones are never renamed or removed.
why: The harness greps on them.
@row_add n=96 row=decision/no-secrets-in-log-lines area=obs-scope-summary rec=49
name: No secrets in log lines
rule: No log line contains a Cookie header or any stripped CLAUDE* value.
guard: secret-scan test
@row_add n=97 row=decision/single-line-log-records area=obs-scope-summary rec=49
name: Single-line log records
rule: Every log record is one JSON line parseable by jq; never a multi-line stack trace.
@row_add n=98 row=decision/diagnostics-file-modes area=obs-scope-summary rec=49,50
name: Diagnostics file and directory modes
rule: Diagnostics files are 0600 inside 0700 directories.
guard: secret-scan test
@row_add n=99 row=decision/log-file-basenames-follow-tests area=obs-scope-summary rec=50
name: Log file names follow tests
rule: Process-log files are <home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson, as tests require.
why: Tests is upstream; arch and security place diagnostics under the instance directory instead.
@row_add n=100 row=contract/logs-wrapper/record-envelope area=obs-scope-summary rec=50
boundary: logs wrapper
item: record envelope
promise: Each process-log line is wrapped as {"src":"diag","file":<basename>,"record":<line>}.
@row_add n=101 row=decision/home-level-mcp-log-no-user-content area=obs-scope-summary rec=50
name: Home-level MCP log holds no user content
rule: An mcp.ndjson at home level never carries user content.
@row_add n=102 row=decision/diagnostics-checked-at-open area=obs-scope-summary rec=50
name: Diagnostics checked before open
rule: Before a role or detail file is created or narrowed, the existing diagnostics/ folder and the file are checked, a symlink refused and no group or other bit allowed; a refusal is a failed open.
@row_add n=103 row=decision/no-log-rotation area=obs-scope-summary rec=50
name: No log rotation
rule: Process logs are retained for the lifetime of the session home, with no rotation.
@row_none n=21 rec=51 type=0
what: the list of surfaces a critical path crosses
@row_add n=104 row=contract/run-start-events/order area=obs-scope-summary rec=52
boundary: run start events in events.ndjson
item: order
promise: A run start writes wheel{cause:"start"}, then budget-gate, then session-start.
@row_add n=105 row=contract/run-log/cli-version area=obs-scope-summary rec=53
boundary: run log line
item: cli_version
promise: A required field on run's log lines.
@row_add n=106 row=contract/run-log/cli-verified area=obs-scope-summary rec=53
boundary: run log line
item: cli_verified
promise: A required field on run's log lines.
@row_add n=107 row=contract/run-log/exit-code area=obs-scope-summary rec=53
boundary: run log line
item: exit_code
promise: A required field on run's log lines.
@row_add n=108 row=contract/run-exit-1-detail/already-live area=obs-scope-summary rec=53
boundary: run exit-1 detail codes
item: already-live
promise: A live holder or a fresh replayed beat.
@row_add n=109 row=contract/run-exit-1-detail/stale-heartbeat area=obs-scope-summary rec=53
boundary: run exit-1 detail codes
item: stale-heartbeat
promise: A stale holder.
@row_add n=110 row=contract/run-exit-1-detail/squatted-name area=obs-scope-summary rec=53
boundary: run exit-1 detail codes
item: squatted-name
promise: A distinct detail code for a squatted name.
@row_add n=111 row=contract/run-exit-1-detail/sha-256-mismatch area=obs-scope-summary rec=53
boundary: run exit-1 detail codes
item: sha-256 mismatch
promise: A distinct detail code for a sha-256 mismatch.
@row_add n=112 row=contract/run-exit-1-detail/no-session area=obs-scope-summary rec=53
boundary: run exit-1 detail codes
item: no-session
promise: On a start made by viola revive, a distinct detail code.
@row_add n=113 row=contract/run-exit-1-detail/session-live area=obs-scope-summary rec=53
boundary: run exit-1 detail codes
item: session-live
promise: On a start made by viola revive, a distinct detail code.
@row_add n=114 row=contract/run-exit-1-detail/cwd-missing area=obs-scope-summary rec=53
boundary: run exit-1 detail codes
item: cwd-missing
promise: On a start made by viola revive, a distinct detail code.
@row_add n=115 row=decision/detail-codes-no-paths area=obs-scope-summary rec=53
name: Detail codes carry no path or pid
rule: No path or pid appears in a detail code.
@row_none n=22 rec=54 type=S
@row_none n=23 rec=55 type=0
what: the list of surfaces a critical path crosses
