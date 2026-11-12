@row_none n=1 rec=1 type=S
@row_none n=2 rec=2 type=0
what: a note on when the document may be edited
@row_add n=1 row=decision/agent-readable-telemetry area=obs-plan-viola rec=3
name: Agent-readable telemetry only
rule: Every telemetry signal must be readable by an agent from a snapshot or pasted log excerpt: no human-only dashboards, no proprietary APM lock-in, no human-review-gated log analysis.
except: A surface that cannot be agent-readable is flagged not-instrumentable and handled through boundary instrumentation only.
@row_none n=3 rec=4 type=S
@row_none n=4 rec=5 type=S
@row_add n=2 row=decision/obs-tier-standard area=1-obs-scope-summary rec=6,9
name: Observability tier Standard
rule: The observability tier is Standard (1), with Minimal-tier exporter carve-outs.
why: The tests tier is Comprehensive (2) and obs sits one step below it within the plus-or-minus-one band; the Minimal security tier gives no Comprehensive driver.
@row_none n=5 rec=7 type=S
@row_none n=6 rec=8 type=S
@row_add n=3 row=absence/audit-log-compliance-trail area=1-obs-scope-summary rec=9
capability: An audit-log compliance trail
@row_add n=4 row=absence/pii-scrubbing-regime area=1-obs-scope-summary rec=9
capability: A PII-scrubbing regime beyond the NEVER-log floor
@row_add n=5 row=decision/no-network-otlp-exporter area=1-obs-scope-summary rec=11,10,49
name: No network OTLP exporter
rule: Telemetry has no network OTLP exporter, loopback included; the Standard default is replaced by a Minimal-tier shape.
why: Upstream bans outbound network calls and new listeners without a Security Decisions Log entry; viola makes no outbound network calls.
@row_add n=6 row=decision/no-error-reporter area=1-obs-scope-summary rec=11,10
name: No opt-in error reporter
rule: The product has no opt-in error reporter; the Standard default is replaced by a Minimal-tier shape.
why: Upstream bans outbound network calls, reserves stdout on 3 of 4 roles, and bans Tokio and C crates in the sync graph.
@row_add n=7 row=decision/no-env-log-levels area=1-obs-scope-summary rec=12,10
name: No per-module log levels from the environment
rule: Per-module log levels are never read from environment variables.
why: Environment variables are not a configuration channel.
@row_add n=8 row=decision/tracing-json-line-file-pipeline area=1-obs-scope-summary rec=13
name: tracing JSON-per-line file pipeline
rule: Telemetry is tracing plus tracing-subscriber writing JSON-per-line files under diagnostics/, correlated by corr and consumed through agent-run logs and status.
why: The pipeline is the one the tests plan binds.
@row_none n=7 rec=14 type=9
@row_none n=8 rec=15 type=S
@row_add n=9 row=decision/error-chain-fixed-stderr-message area=1-obs-scope-summary rec=17,16
name: Error chains reach stderr as fixed messages
rule: In the viola root binary, the only crate that uses anyhow, an error chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to the diagnostics logs only.
@row_add n=10 row=decision/viola-core-emits-no-telemetry area=1-obs-scope-summary rec=19,18
name: viola-core emits no telemetry
rule: The viola-core crate supplies log-field values but emits nothing and is not instrumented.
why: A pure type crate with no I/O.
@row_add n=11 row=decision/pty-byte-stream-never-logged area=1-obs-scope-summary rec=21,20
name: PTY byte stream never logged
rule: The viola-pty seam is instrumented, but the PTY byte stream is never logged, and the openpty OS API behind it is boundary-only.
why: The PTY byte stream is user content.
@row_add n=12 row=decision/channel-json-rpc-over-ndjson area=1-obs-scope-summary rec=22,54
name: Channel is JSON-RPC 2.0 over ndjson
rule: The viola-channel crate speaks JSON-RPC 2.0 over ndjson on local Unix sockets, one endpoint per viola run, with a sync client and server and a Tokio client behind the tokio feature.
@row_add n=13 row=decision/channel-sync-instrumentation area=1-obs-scope-summary rec=23,22
name: Sync instrumentation in viola-channel
rule: Instrumentation in viola-channel must be sync, because Tokio is forbidden in that crate's graph.
except: The tokio feature, when enabled.
@row_add n=14 row=decision/state-sync-instrumentation area=1-obs-scope-summary rec=25,24
name: Sync instrumentation in viola-state
rule: Instrumentation in viola-state must be sync.
why: The Stack bans Tokio in viola-state.
@row_add n=15 row=decision/drift-reports-diagnostics-only area=1-obs-scope-summary rec=27,26
name: Drift reports to diagnostics only
rule: Drift reports of viola-agent-claude go to the diagnostics logs only.
why: Its outputs are the most sensitive in the product.
@row_add n=16 row=decision/stripped-claude-env-never-logged area=1-obs-scope-summary rec=27,26
name: R8-stripped CLAUDE values never logged
rule: R8-stripped CLAUDE* environment values are never logged anywhere.
why: The viola-agent-claude outputs are the most sensitive in the product.
@row_add n=17 row=decision/rmcp-stdio-mcp-server area=1-obs-scope-summary rec=28
name: rmcp stdio MCP server
rule: The viola-mcp crate is an rmcp stdio MCP server.
@row_add n=18 row=contract/viola-mcp-tools/send area=1-obs-scope-summary rec=28
boundary: viola-mcp stdio server tools
item: send
promise: The viola-mcp stdio server offers a send tool.
@row_add n=19 row=contract/viola-mcp-tools/wait area=1-obs-scope-summary rec=28
boundary: viola-mcp stdio server tools
item: wait
promise: The viola-mcp stdio server offers a wait tool.
@row_add n=20 row=contract/viola-mcp-tools/last area=1-obs-scope-summary rec=28
boundary: viola-mcp stdio server tools
item: last
promise: The viola-mcp stdio server offers a last tool.
@row_add n=21 row=contract/viola-mcp-tools/answer area=1-obs-scope-summary rec=28
boundary: viola-mcp stdio server tools
item: answer
promise: The viola-mcp stdio server offers an answer tool.
@row_add n=22 row=contract/viola-mcp-tools/list area=1-obs-scope-summary rec=28
boundary: viola-mcp stdio server tools
item: list
promise: The viola-mcp stdio server offers a list tool.
@row_add n=23 row=decision/mcp-stdout-frames-only area=1-obs-scope-summary rec=29,28
name: MCP stdout carries frames only
rule: No telemetry from the viola-mcp server may go to stdout.
why: Stdout is reserved for MCP frames.
@row_none n=9 rec=30 type=5
@row_add n=24 row=decision/child-exit-from-process-handle area=1-obs-scope-summary rec=31,51
name: Child exit from the process handle
rule: The wrapped child's exit and its process-exit event come from the wait on the process handle, never from reader EOF.
@row_add n=25 row=decision/claude-internals-not-a-contract area=1-obs-scope-summary rec=31,30
name: Wrapped claude internals are not a contract
rule: The wrapped claude CLI is a third-party binary run unmodified and observed at its boundary only: spawn, exit, paste writes and the vt100 readiness-gate outcome; its internal state and transcripts are never read as a contract.
@row_none n=10 rec=32 type=5
@row_none n=11 rec=33 type=0
what: a description of what viola sees at the hook boundary
@row_add n=26 row=decision/agents-probe-parse-failure-unknown area=1-obs-scope-summary rec=35,34
name: Agents probe parse failure becomes unknown
rule: A parse failure of the claude agents --json or --version probe becomes "unknown" plus a counter.
@row_add n=27 row=decision/agents-probe-output-untrusted area=1-obs-scope-summary rec=35,34
name: Agents probe output untrusted
rule: The output rows of the claude agents --json probe are untrusted.
@row_add n=28 row=decision/statusline-command-duration-exit-only area=1-obs-scope-summary rec=37,36
name: Statusline command observed by duration and exit
rule: The user's statusline_command shell-out is observed by its duration and exit status only.
why: A user-written shell command.
@row_add n=29 row=decision/statusline-command-string-diagnostics-only area=1-obs-scope-summary rec=37,36
name: Statusline command string kept in diagnostics
rule: The user's statusline_command string is never logged outside the diagnostics logs.
@row_add n=30 row=decision/os-apis-at-call-sites area=1-obs-scope-summary rec=39,38
name: OS APIs observed at call sites
rule: OS APIs (openpty, Unix socket, filesystem locks and atomic rename, sysinfo) are instrumented at viola's call sites only, never in the OS internals.
@row_none n=12 rec=40 type=5
@row_add n=31 row=decision/event-payload-not-in-process-logs area=1-obs-scope-summary rec=41
name: Event payloads stay out of process logs
rule: Payload content of events.ndjson is never copied into process-log lines.
@row_add n=32 row=decision/absolute-paths-scrubbed-externally area=1-obs-scope-summary rec=41
name: Absolute paths scrubbed from external output
rule: Absolute paths in snapshot.json and config values are scrubbed from external output.
@row_none n=13 rec=42 type=5
@row_none n=14 rec=43 type=0
what: an instrumentability note on static configuration files
@row_add n=33 row=decision/claude-fixtures-scrub-required area=1-obs-scope-summary rec=45,44
name: Claude fixtures are scrub-required
rule: Build and CI infrastructure is not instrumented, and the claude fixtures are scrub-required data, never a telemetry source.
why: Not product runtime.
@row_none n=15 rec=46 type=S
@row_add n=34 row=contract/log-line/process area=1-obs-scope-summary rec=47
boundary: the log line
item: process
promise: Every log line carries a process key naming its role: run, hook, mcp or cli.
@row_none n=16 rec=48 type=5
@row_add n=35 row=decision/cli-file-sink-only area=1-obs-scope-summary rec=49,48
name: Short-lived CLI logs to a file sink only
rule: Short-lived CLI verbs log only to a JSON-per-line file sink in diagnostics when an instance resolves, never to stdout.
why: Stdout carries the --json results.
@row_add n=36 row=decision/instance-never-from-working-directory area=1-obs-scope-summary rec=49
name: Instance never from the working directory
rule: The instance comes from VIOLA_NAME or an argument, never from the working directory.
@row_add n=37 row=decision/stderr-fixed-strings area=1-obs-scope-summary rec=49
name: Fixed strings on stderr
rule: Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
@row_none n=17 rec=50 type=5
@row_add n=38 row=contract/run-log-file/path area=1-obs-scope-summary rec=51,50
boundary: the viola run log file
item: path
promise: viola run logs only to a file sink at diagnostics/run-<name>.ndjson.
@row_add n=39 row=decision/run-silent-terminal area=1-obs-scope-summary rec=51,50
name: viola run silent on the terminal
rule: viola run writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns.
@row_add n=40 row=decision/run-sync-instrumentation area=1-obs-scope-summary rec=51,50
name: Sync instrumentation in viola run
rule: Instrumentation in viola run is sync, with no Tokio.
@row_none n=18 rec=52 type=5
@row_add n=41 row=contract/hook-log-file/path area=1-obs-scope-summary rec=53,52
boundary: the viola hook log file
item: path
promise: viola hook logs only to a file sink at diagnostics/hook-<name>.ndjson.
@row_add n=42 row=decision/hook-stderr-forbidden area=1-obs-scope-summary rec=53,52
name: No stderr from hooks
rule: viola hook writes nothing to stderr.
@row_add n=43 row=decision/hook-stdout-decision-body-only area=1-obs-scope-summary rec=53,52
name: Hook stdout carries the decision body only
rule: viola hook stdout carries only the decision body; for viola hook statusline, the output of the user's status-line command.
@row_add n=44 row=decision/hook-log-write-failure-swallowed area=1-obs-scope-summary rec=53,52
name: Failed hook log write swallowed
rule: A failed log write in viola hook is swallowed and the hook still exits 0.
@row_add n=45 row=decision/hook-silent-no-op-without-instance area=1-obs-scope-summary rec=53,52
name: Hook silent no-op without an instance
rule: Without VIOLA_NAME, on an instance the instance check refuses, or on a VIOLA_DIR naming an instance directory that does not exist, viola hook is a silent no-op: exit 0, no file opened, no line, no frame.
@row_add n=46 row=decision/hook-log-line-single-append-write area=1-obs-scope-summary rec=53,52
name: One write per hook log line
rule: Each viola hook log line is one write on an append-mode handle.
@row_add n=47 row=decision/hook-logger-init-no-blocking-io area=1-obs-scope-summary rec=53,52
name: Hook logger init adds no blocking I/O
rule: Logger initialisation in viola hook adds no blocking I/O beyond one file open and append.
@row_add n=48 row=decision/corr-field-propagation area=1-obs-scope-summary rec=55,54
name: Propagation through the corr field
rule: Correlation across the viola-channel is carried by the corr field in log lines of the process on each side.
why: Channel params declare no trace-context field.
@row_add n=49 row=contract/json-rpc-errors/32603 area=1-obs-scope-summary rec=55,54
boundary: viola-channel JSON-RPC errors
item: -32603
promise: Error -32603 is "internal error" with data:null.
@row_add n=50 row=decision/self-reported-from-not-identity area=1-obs-scope-summary rec=55,54
name: Self-reported from is not an identity
rule: The self-reported from on the viola-channel is never logged as an authenticated identity.
