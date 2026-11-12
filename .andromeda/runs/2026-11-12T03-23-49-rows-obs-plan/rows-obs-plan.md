@row_none n=1 rec=fct-puseu3vv@1 type=S
@row_none n=2 rec=fct-7rmyxbi7@1 type=0
what: a note on when the document may be edited
@row_add n=1 row=decision/agent-readable-telemetry area=obs-plan-viola rec=fct-gty7muwj@1
name: Agent-readable telemetry only
rule: Every telemetry signal must be readable by an agent from a snapshot or pasted log excerpt: no human-only dashboards, no proprietary APM lock-in, no human-review-gated log analysis.
except: A surface that cannot be agent-readable is flagged not-instrumentable and handled through boundary instrumentation only.
@row_none n=3 rec=fct-cybcxb5n@1 type=S
@row_none n=4 rec=fct-cnzlh5yi@1 type=S
@row_add n=2 row=decision/obs-tier-standard area=1-obs-scope-summary rec=fct-y4ogetzn@1,fct-6eqwgp2k@1
name: Observability tier Standard
rule: The observability tier is Standard (1), with Minimal-tier exporter carve-outs.
why: The tests tier is Comprehensive (2) and obs sits one step below it within the plus-or-minus-one band; the Minimal security tier gives no Comprehensive driver.
@row_none n=5 rec=fct-2cs5m4zb@1 type=S
@row_none n=6 rec=fct-n5cdr5q7@1 type=S
@row_add n=3 row=absence/audit-log-compliance-trail area=1-obs-scope-summary rec=fct-6eqwgp2k@1
capability: An audit-log compliance trail
@row_add n=4 row=absence/pii-scrubbing-regime area=1-obs-scope-summary rec=fct-6eqwgp2k@1
capability: A PII-scrubbing regime beyond the NEVER-log floor
@row_add n=5 row=decision/no-network-otlp-exporter area=1-obs-scope-summary rec=fct-4v6f5pir@1,fct-uv7mc5hr@1,fct-fxg4qazp@1
name: No network OTLP exporter
rule: Telemetry has no network OTLP exporter, loopback included; the Standard default is replaced by a Minimal-tier shape.
why: Upstream bans outbound network calls and new listeners without a Security Decisions Log entry; viola makes no outbound network calls.
@row_add n=6 row=decision/no-error-reporter area=1-obs-scope-summary rec=fct-4v6f5pir@1,fct-uv7mc5hr@1
name: No opt-in error reporter
rule: The product has no opt-in error reporter; the Standard default is replaced by a Minimal-tier shape.
why: Upstream bans outbound network calls, reserves stdout on 3 of 4 roles, and bans Tokio and C crates in the sync graph.
@row_add n=7 row=decision/no-env-log-levels area=1-obs-scope-summary rec=fct-ljftf7ri@1,fct-uv7mc5hr@1
name: No per-module log levels from the environment
rule: Per-module log levels are never read from environment variables.
why: Environment variables are not a configuration channel.
@row_add n=8 row=decision/tracing-json-line-file-pipeline area=1-obs-scope-summary rec=fct-sxvw7chq@1
name: tracing JSON-per-line file pipeline
rule: Telemetry is tracing plus tracing-subscriber writing JSON-per-line files under diagnostics/, correlated by corr and consumed through agent-run logs and status.
why: The pipeline is the one the tests plan binds.
@row_none n=7 rec=fct-mpt2356v@1 type=9
@row_none n=8 rec=fct-hp7myxef@1 type=S
@row_add n=9 row=decision/error-chain-fixed-stderr-message area=1-obs-scope-summary rec=fct-cjx2g7cf@1,fct-3yxzr6p7@1
name: Error chains reach stderr as fixed messages
rule: In the viola root binary, the only crate that uses anyhow, an error chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to the diagnostics logs only.
@row_add n=10 row=decision/viola-core-emits-no-telemetry area=1-obs-scope-summary rec=fct-gy7sj7m2@1,fct-yt4lnr2b@1
name: viola-core emits no telemetry
rule: The viola-core crate supplies log-field values but emits nothing and is not instrumented.
why: A pure type crate with no I/O.
@row_add n=11 row=decision/channel-json-rpc-over-ndjson area=1-obs-scope-summary rec=fct-lz2jkmkt@1,fct-p4bwzizt@2
name: Channel is JSON-RPC 2.0 over ndjson
rule: The viola-channel crate speaks JSON-RPC 2.0 over ndjson on local Unix sockets, one endpoint per viola run, with a sync client and server and a Tokio client behind the tokio feature.
@row_add n=12 row=decision/channel-sync-instrumentation area=1-obs-scope-summary rec=fct-hezc5s2v@1,fct-lz2jkmkt@1
name: Sync instrumentation in viola-channel
rule: Instrumentation in viola-channel must be sync, because Tokio is forbidden in that crate's graph.
except: The tokio feature, when enabled.
@row_add n=13 row=decision/state-sync-instrumentation area=1-obs-scope-summary rec=fct-ewk35na4@1,fct-4rqiexan@1
name: Sync instrumentation in viola-state
rule: Instrumentation in viola-state must be sync.
why: The Stack bans Tokio in viola-state.
@row_add n=14 row=decision/stripped-claude-env-never-logged area=1-obs-scope-summary rec=fct-gira4fst@1,fct-73ei4s3r@2
name: R8-stripped CLAUDE values never logged
rule: R8-stripped CLAUDE* environment values are never logged anywhere.
why: The viola-agent-claude outputs are the most sensitive in the product.
@row_add n=15 row=decision/rmcp-stdio-mcp-server area=1-obs-scope-summary rec=fct-zry4bqzw@1
name: rmcp stdio MCP server
rule: The viola-mcp crate is an rmcp stdio MCP server.
@row_add n=16 row=contract/viola-mcp-tools/send area=1-obs-scope-summary rec=fct-zry4bqzw@1
boundary: viola-mcp stdio server tools
item: send
promise: The viola-mcp stdio server offers a send tool.
@row_add n=17 row=contract/viola-mcp-tools/wait area=1-obs-scope-summary rec=fct-zry4bqzw@1
boundary: viola-mcp stdio server tools
item: wait
promise: The viola-mcp stdio server offers a wait tool.
@row_add n=18 row=contract/viola-mcp-tools/last area=1-obs-scope-summary rec=fct-zry4bqzw@1
boundary: viola-mcp stdio server tools
item: last
promise: The viola-mcp stdio server offers a last tool.
@row_add n=19 row=contract/viola-mcp-tools/answer area=1-obs-scope-summary rec=fct-zry4bqzw@1
boundary: viola-mcp stdio server tools
item: answer
promise: The viola-mcp stdio server offers an answer tool.
@row_add n=20 row=contract/viola-mcp-tools/list area=1-obs-scope-summary rec=fct-zry4bqzw@1
boundary: viola-mcp stdio server tools
item: list
promise: The viola-mcp stdio server offers a list tool.
@row_add n=21 row=decision/mcp-stdout-frames-only area=1-obs-scope-summary rec=fct-uulw22xw@1,fct-zry4bqzw@1
name: MCP stdout carries frames only
rule: No telemetry from the viola-mcp server may go to stdout.
why: Stdout is reserved for MCP frames.
@row_add n=22 row=decision/child-exit-from-process-handle area=1-obs-scope-summary rec=fct-avrouaxx@1,fct-ctz7espz@1
name: Child exit from the process handle
rule: The wrapped child's exit and its process-exit event come from the wait on the process handle, never from reader EOF.
@row_none n=9 rec=fct-bwsghifi@1 type=5
@row_none n=10 rec=fct-h5763i4j@1 type=0
what: a description of what viola sees at the hook boundary
@row_add n=23 row=decision/agents-probe-parse-failure-unknown area=1-obs-scope-summary rec=fct-lpshe4lx@1,fct-ubqrs7oo@1
name: Agents probe parse failure becomes unknown
rule: A parse failure of the claude agents --json or --version probe becomes "unknown" plus a counter.
@row_add n=24 row=decision/agents-probe-output-untrusted area=1-obs-scope-summary rec=fct-lpshe4lx@1,fct-ubqrs7oo@1
name: Agents probe output untrusted
rule: The output rows of the claude agents --json probe are untrusted.
@row_add n=25 row=decision/statusline-command-duration-exit-only area=1-obs-scope-summary rec=fct-hfqkop25@1,fct-4hfgtuu2@1
name: Statusline command observed by duration and exit
rule: The user's statusline_command shell-out is observed by its duration and exit status only.
why: A user-written shell command.
@row_add n=26 row=decision/statusline-command-string-diagnostics-only area=1-obs-scope-summary rec=fct-hfqkop25@1,fct-4hfgtuu2@1
name: Statusline command string kept in diagnostics
rule: The user's statusline_command string is never logged outside the diagnostics logs.
@row_add n=27 row=decision/os-apis-at-call-sites area=1-obs-scope-summary rec=fct-eokuiijb@1,fct-zce67g4u@2
name: OS APIs observed at call sites
rule: OS APIs (openpty, Unix socket, filesystem locks and atomic rename, sysinfo) are instrumented at viola's call sites only, never in the OS internals.
@row_none n=11 rec=fct-d2ucjvnc@1 type=5
@row_none n=12 rec=fct-4ereqqqf@1 type=5
@row_none n=13 rec=fct-any6tvdo@1 type=0
what: an instrumentability note on static configuration files
@row_add n=28 row=decision/claude-fixtures-scrub-required area=1-obs-scope-summary rec=fct-k7mp4nhp@1,fct-cnsk5zxg@1
name: Claude fixtures are scrub-required
rule: Build and CI infrastructure is not instrumented, and the claude fixtures are scrub-required data, never a telemetry source.
why: Not product runtime.
@row_none n=14 rec=fct-bui7ij2h@1 type=S
@row_add n=29 row=decision/cli-file-sink-only area=1-obs-scope-summary rec=fct-fxg4qazp@1,fct-wf4ruhe4@1
name: Short-lived CLI logs to a file sink only
rule: Short-lived CLI verbs log only to a JSON-per-line file sink in diagnostics when an instance resolves, never to stdout.
why: Stdout carries the --json results.
@row_add n=30 row=decision/instance-never-from-working-directory area=1-obs-scope-summary rec=fct-fxg4qazp@1
name: Instance never from the working directory
rule: The instance comes from VIOLA_NAME or an argument, never from the working directory.
@row_add n=31 row=decision/stderr-fixed-strings area=1-obs-scope-summary rec=fct-fxg4qazp@1
name: Fixed strings on stderr
rule: Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
@row_add n=32 row=contract/run-log-file/path area=1-obs-scope-summary rec=fct-ctz7espz@1,fct-bgcxnqje@1
boundary: the viola run log file
item: path
promise: viola run logs only to a file sink at diagnostics/run-<name>.ndjson.
@row_add n=33 row=decision/run-silent-terminal area=1-obs-scope-summary rec=fct-ctz7espz@1,fct-bgcxnqje@1
name: viola run silent on the terminal
rule: viola run writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns.
@row_add n=34 row=decision/run-sync-instrumentation area=1-obs-scope-summary rec=fct-ctz7espz@1,fct-bgcxnqje@1
name: Sync instrumentation in viola run
rule: Instrumentation in viola run is sync, with no Tokio.
@row_add n=35 row=contract/hook-log-file/path area=1-obs-scope-summary rec=fct-qlfp3c3h@1,fct-z66kg5vk@1
boundary: the viola hook log file
item: path
promise: viola hook logs only to a file sink at diagnostics/hook-<name>.ndjson.
@row_add n=36 row=decision/hook-stderr-forbidden area=1-obs-scope-summary rec=fct-qlfp3c3h@1,fct-z66kg5vk@1
name: No stderr from hooks
rule: viola hook writes nothing to stderr.
@row_add n=37 row=decision/hook-stdout-decision-body-only area=1-obs-scope-summary rec=fct-qlfp3c3h@1,fct-z66kg5vk@1
name: Hook stdout carries the decision body only
rule: viola hook stdout carries only the decision body; for viola hook statusline, the output of the user's status-line command.
@row_add n=38 row=decision/hook-silent-no-op-without-instance area=1-obs-scope-summary rec=fct-qlfp3c3h@1,fct-z66kg5vk@1
name: Hook silent no-op without an instance
rule: Without VIOLA_NAME, on an instance the instance check refuses, or on a VIOLA_DIR naming an instance directory that does not exist, viola hook is a silent no-op: exit 0, no file opened, no line, no frame.
@row_add n=39 row=decision/hook-log-line-single-append-write area=1-obs-scope-summary rec=fct-qlfp3c3h@1,fct-z66kg5vk@1
name: One write per hook log line
rule: Each viola hook log line is one write on an append-mode handle.
@row_add n=40 row=decision/hook-logger-init-no-blocking-io area=1-obs-scope-summary rec=fct-qlfp3c3h@1,fct-z66kg5vk@1
name: Hook logger init adds no blocking I/O
rule: Logger initialisation in viola hook adds no blocking I/O beyond one file open and append.
@row_add n=41 row=decision/corr-field-propagation area=1-obs-scope-summary rec=fct-m7cbk627@1,fct-p4bwzizt@2
name: Propagation through the corr field
rule: Correlation across the viola-channel is carried by the corr field in log lines of the process on each side.
why: Channel params declare no trace-context field.
@row_add n=42 row=contract/json-rpc-errors/32603 area=1-obs-scope-summary rec=fct-m7cbk627@1,fct-p4bwzizt@2
boundary: viola-channel JSON-RPC errors
item: -32603
promise: Error -32603 is "internal error" with data:null.
@row_add n=43 row=decision/self-reported-from-not-identity area=1-obs-scope-summary rec=fct-m7cbk627@1,fct-p4bwzizt@2
name: Self-reported from is not an identity
rule: The self-reported from on the viola-channel is never logged as an authenticated identity.
@row_add n=44 row=decision/mcp-stdio-server-tokio-allowed area=1-obs-scope-summary rec=fct-p7pvzia6@1
name: MCP server on stdio with Tokio allowed
rule: The `viola mcp` surface is an internal stdio MCP server, and Tokio is allowed in it.
@row_add n=45 row=contract/mcp-stdin/max-frame area=1-obs-scope-summary rec=fct-p7pvzia6@1
boundary: viola mcp stdin
item: MAX_FRAME
promise: Stdin is read in lines of at most `MAX_FRAME`; a longer line is refused whole and the session ends as at a closed stdin.
@row_add n=46 row=decision/mcp-log-file-sink area=1-obs-scope-summary rec=fct-mawfkjfn@1
name: MCP logs to a file sink, never stdout
rule: The `mcp` process always logs to the file sink `<home>/diagnostics/mcp.ndjson`, with stderr JSON only when that file cannot be opened, and the log writer is set explicitly.
why: stdout is forbidden because it carries MCP frames
@row_none n=15 rec=fct-q66n3gga@1 type=S
@row_add n=47 row=decision/no-otel-sdk-exporter area=1-obs-scope-summary rec=fct-hx3c7qre@1
name: No OTel SDK exporter
rule: No OTel SDK exporter is initialised; any OTel bridge may only feed an in-process or file sink and must be Tokio-free.
why: outbound network calls are banned, a new listener needs a Security Decisions Log entry, stdout is reserved on hook, mcp and run, and cargo deny bans Tokio in sync crates
except: an optional Tokio-free OTel bridge feeding an in-process or file sink
@row_add n=48 row=decision/tracing-json-pipeline area=1-obs-scope-summary rec=fct-hx3c7qre@1,fct-ccfaed7l@1
name: tracing with the tracing-subscriber JSON formatter
rule: The `tracing` facade plus the tracing-subscriber JSON formatter is the binding logging pipeline, and JSON-to-file is the only logging mode.
@row_add n=49 row=contract/diag-line/service-name area=1-obs-scope-summary rec=fct-uwfffcsu@1
boundary: diagnostic log line
item: service.name
promise: Hardcoded to "viola".
@row_add n=50 row=contract/diag-line/service-version area=1-obs-scope-summary rec=fct-uwfffcsu@1
boundary: diagnostic log line
item: service.version
promise: The crate version, fixed at compile time.
@row_add n=51 row=contract/diag-line/process area=1-obs-scope-summary rec=fct-uwfffcsu@1
boundary: diagnostic log line
item: process
promise: The role of the writing process, one of run, hook, mcp, cli; required on every line and matching `sender` / `writer`.
@row_add n=52 row=contract/diag-line/instance area=1-obs-scope-summary rec=fct-uwfffcsu@1
boundary: diagnostic log line
item: instance
promise: The instance's `ViolaName`, or null; required on every line.
@row_add n=53 row=decision/one-log-file-per-role area=1-obs-scope-summary rec=fct-ccfaed7l@1
name: One log file per process role
rule: Each process role writes its own file under `diagnostics/`, no role has a stdout sink, and stderr JSON is used only by `mcp` when its file cannot be opened.
@row_add n=54 row=decision/level-filter-not-from-env area=1-obs-scope-summary rec=fct-ccfaed7l@1
name: Level filter never read from RUST_LOG
rule: The log level filter is never read from `RUST_LOG`.
@row_add n=55 row=decision/redaction-not-disableable area=1-obs-scope-summary rec=fct-ccfaed7l@1,fct-xvmeqbmc@1
name: Redaction cannot be disabled
rule: No verbosity control, env, flag or config may disable or bypass redaction.
@row_add n=56 row=decision/diag-schema-additive-only area=1-obs-scope-summary rec=fct-zhb762yr@1
name: Diagnostic log schema is additive only
rule: The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove any field.
@row_add n=57 row=decision/diagnostics-owner-only-modes area=1-obs-scope-summary rec=fct-igz2gfrn@1,fct-wrd7y3z2@2
name: Diagnostics files are owner-only
rule: Every diagnostics file is mode 0600 inside a 0700 directory.
@row_add n=58 row=contract/harness-logs/merge area=1-obs-scope-summary rec=fct-q2zh72zo@2
boundary: harness logs command
item: merged sources
promise: Merges `events.ndjson` lines (`src:"events"`, with byte `offset`) with `diagnostics/*.ndjson` lines (`src:"diag"`).
@row_add n=59 row=contract/harness-logs/torn-line area=1-obs-scope-summary rec=fct-q2zh72zo@2
boundary: harness logs command
item: torn lines
promise: A torn line is emitted as `{"torn":true}`.
@row_add n=60 row=contract/harness-logs/filters area=1-obs-scope-summary rec=fct-q2zh72zo@2
boundary: harness logs command
item: filters
promise: Output can be filtered by `--instance`, `--kind`, `--process` and `--after`.
@row_add n=61 row=contract/harness-logs/output area=1-obs-scope-summary rec=fct-q2zh72zo@2
boundary: harness logs command
item: output
promise: Output is ndjson on stdout, ready for `jq` or pasting to an agent.
@row_add n=62 row=contract/agent-run-status/state-last-error area=1-obs-scope-summary rec=fct-q2zh72zo@2
boundary: agent-run status
item: state and last_error
promise: The aggregate health snapshot gives `state` and `last_error` as a closed enum.
@row_add n=63 row=decision/no-traceparent-on-local-socket area=1-obs-scope-summary rec=fct-pcotmuct@1
name: No W3C traceparent on the local socket
rule: Calls from CLI, MCP and hook to the wrapper carry no W3C `traceparent`; correlation uses `corr` (JSON-RPC `id`, `dialog_id`, send `cursor`) plus `process` and `instance`.
why: channel `params` declare no trace-context field
@row_add n=64 row=decision/hook-stdin-correlation area=1-obs-scope-summary rec=fct-pcotmuct@1
name: Hook stdin correlated by instance and dialog
rule: Hook stdin from Claude Code carries no inbound trace context, so it is correlated by `instance` and `dialog_id`.
@row_add n=65 row=decision/liveness-logged-on-change area=1-obs-scope-summary rec=fct-zdbw2hss@1
name: Liveness transitions logged on change only
rule: The `run` heartbeat file touched every second is the tick; liveness transitions (live, stale, gone) are logged on change only, never as a per-second line.
why: a 1 s log line would grow a never-rotated file without bound
@row_none n=16 rec=fct-r554tsmx@1 type=S
@row_add n=66 row=decision/spans-not-wire-contract area=1-obs-scope-summary rec=fct-o6ysgnna@1
name: Tracing spans are not part of the wire contract
rule: Span names are internal `tracing` spans in `<area>.<operation>` snake_case form; the agent-facing contract is the closed `event` enum plus `corr`.
why: the config sets `with_current_span(false)`, so spans do not reach the line
@row_add n=67 row=decision/exit-1-detail-codes area=1-obs-scope-summary rec=fct-i6rpx5jf@1,fct-qixjo2w7@1
name: Distinct detail codes for exit-1 causes
rule: On the `viola run` start sequence to child spawn, each exit-1 cause logs a distinct `detail` code with no path or pid in it.
@row_add n=68 row=decision/assistant-message-never-logged area=1-obs-scope-summary rec=fct-xsyteni6@1,fct-mutbrgup@1
name: Assistant message never logged on readback
rule: On `wait` / `last` readback, the `last_assistant_message` content never appears in log fields.
@row_add n=69 row=decision/dialog-text-never-logged area=1-obs-scope-summary rec=fct-t4iplhxy@1,fct-berta6ar@1
name: Dialog text never logged on answer
rule: On the dialog to `answer` path, the question, answer and tool `input` text never appear in log fields.
@row_add n=70 row=decision/statusline-command-confined area=1-obs-scope-summary rec=fct-z24xvxbu@1,fct-tlje6rwd@1
name: Statusline command confined to diagnostics
rule: On the budget governor path, the `statusline_command` string never appears outside `diagnostics/`.
@row_none n=17 rec=fct-nluqjh6q@1 type=9
@row_none n=18 rec=fct-ana74txu@1 type=S
@row_none n=19 rec=fct-c2yfcgzp@1 type=S
@row_none n=20 rec=fct-mlx62e77@1 type=S
@row_add n=71 row=decision/channel-lines-carry-no-bodies area=1-obs-scope-summary rec=fct-ernmap45@1
name: Channel log lines carry no bodies
rule: IPC channel log lines never carry `params` or result bodies.
@row_add n=72 row=decision/channel-from-unauthenticated area=1-obs-scope-summary rec=fct-ernmap45@1
name: Channel sender logged as unauthenticated
rule: The `from` of an IPC channel call is logged as unauthenticated.
@row_none n=21 rec=fct-q4jff4ss@1 type=S
@row_none n=22 rec=fct-ptquzqoz@1 type=S
@row_add n=73 row=decision/send-text-confined area=1-obs-scope-summary rec=fct-w5cnwd2z@1
name: Send text confined to instance diagnostics
rule: `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
@row_none n=23 rec=fct-xyisrhrm@1 type=S
@row_none n=24 rec=fct-wpzlres3@1 type=S
@row_add n=74 row=decision/hook-fail-open-silent area=1-obs-scope-summary rec=fct-s5xlshqe@1
name: Hook fails open silently
rule: Hook fail-open paths log only to `hook-<name>.ndjson` and exit 0 with stderr empty.
@row_add n=75 row=decision/drift-reports-to-diagnostics area=1-obs-scope-summary rec=fct-s5xlshqe@1
name: Drift reports go to diagnostics only
rule: Hook drift reports are written to `diagnostics/` only.
@row_none n=25 rec=fct-yrjpw262@1 type=S
@row_none n=26 rec=fct-f5gzcc3w@1 type=S
@row_none n=27 rec=fct-zsea7hp7@1 type=S
@row_none n=28 rec=fct-cyx6phfi@1 type=S
@row_add n=76 row=decision/stripped-env-never-logged area=1-obs-scope-summary rec=fct-xvmeqbmc@1
name: Stripped CLAUDE env values never logged
rule: No R8-stripped `CLAUDE*` value appears in any sink or fixture; the strip is logged as a count or variable names only.
@row_add n=77 row=decision/serde-errors-fixed-message area=1-obs-scope-summary rec=fct-xvmeqbmc@1
name: Serde error chains become a fixed message
rule: A serde-sourced anyhow error chain is replaced by a fixed message before it reaches stderr.
@row_none n=29 rec=fct-v7ii4ik3@1 type=S
@row_none n=30 rec=fct-32l5mpj3@1 type=S
@row_add n=78 row=decision/refusal-telemetry area=1-obs-scope-summary rec=fct-wrd7y3z2@2
name: Refusal telemetry for filesystem refusals
rule: Pinned-exe hash mismatches and ignored symlinks produce refusal telemetry.
@row_add n=79 row=decision/strict-modes-failure-lines area=1-obs-scope-summary rec=fct-wrd7y3z2@2
name: Where a strict-modes failure is logged
rule: A strict-modes failure has a log line only on the `ledger-stamps` read in the `run` version gate and on `verify`'s own `process-exit` for a refused stamp write.
@row_add n=80 row=decision/refused-diagnostics-drops-line area=1-obs-scope-summary rec=fct-wrd7y3z2@2
name: Refused diagnostics file drops its line
rule: A refused `diagnostics/` folder or file is treated as a failed open and its log line is dropped.
@row_add n=81 row=contract/process-exit-detail/strict-modes-failed area=1-obs-scope-summary rec=fct-wrd7y3z2@2
boundary: process-exit detail codes
item: strict-modes-failed
promise: Carried by `verify`'s `process-exit` for a refused stamp write, with exit 1.
@row_add n=82 row=contract/process-exit-detail/socket-dir-refused area=1-obs-scope-summary rec=fct-wrd7y3z2@2
boundary: process-exit detail codes
item: socket-dir-refused
promise: Carried by the line of a refused socket directory, where it has one; a refused socket directory is no strict-modes failure.
@row_none n=31 rec=fct-tbqtd6ka@1 type=S
@row_none n=32 rec=fct-djrxd23m@1 type=S
@row_add n=83 row=decision/parse-failures-unknown-counter area=1-obs-scope-summary rec=fct-6ciuuorn@1
name: Parse failures surface as unknown plus a counter
rule: A child-output parse failure surfaces as `unknown` plus a counter.
@row_add n=84 row=decision/escape-controls-in-terminal area=1-obs-scope-summary rec=fct-6ciuuorn@1
name: Control characters escaped in terminal rendering
rule: C0 and C1 control characters are escaped in any human terminal rendering of logged rows.
@row_none n=33 rec=fct-mhsc2vvi@1 type=S
@row_none n=34 rec=fct-6cgyhmtp@1 type=S
@row_none n=35 rec=fct-4qdca7zm@1 type=S
@row_none n=36 rec=fct-huapunm4@1 type=S
@row_none n=37 rec=fct-hd6w2yfv@1 type=S
@row_none n=38 rec=fct-gx6wpoyv@1 type=S
@row_none n=39 rec=fct-yxfyyyjr@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=40 rec=fct-zsuohxry@1 type=0
what: a source citation for a trigger
@row_none n=41 rec=fct-vox54xyt@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=42 rec=fct-mjdbrsjn@2 type=0
what: a source citation for a trigger
@row_add n=85 row=decision/single-file-sink-no-os-exporter area=1-obs-scope-summary rec=fct-dao4nifd@2
name: One file sink and one line schema
rule: Telemetry on Linux goes to one file sink with one line schema, with no OS-specific exporter.
@row_none n=43 rec=fct-as4ibtw3@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=44 rec=fct-coohkvlj@1 type=0
what: a source citation for a trigger
@row_none n=45 rec=fct-metpdpyi@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_add n=86 row=decision/event-log-never-rotated area=1-obs-scope-summary rec=fct-6xh2aqem@1
name: Event log never rotated
rule: The events.ndjson product event log is never rotated; its retention is the whole lifetime of the session home.
why: byte offsets into the log are cursors
@row_none n=46 rec=fct-izqaewfs@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_add n=87 row=decision/ndjson-logs-on-disk area=1-obs-scope-summary rec=fct-rjdprqrn@1
name: Bridge logs as ndjson on disk
rule: The bridge's logs are ndjson on disk, and the event log serves as the audit trail.
why: logs on disk survive restarts on either side
@row_none n=47 rec=fct-cywoqpsh@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=48 rec=fct-fzcmyye4@1 type=0
what: a source citation for a trigger
@row_add n=88 row=contract/process-log-event/release-from-driver area=1-obs-scope-summary rec=fct-y6qmqcu2@1
boundary: the process log event field
item: release-from-driver
promise: Release from the driver is logged as its own distinct event.
@row_none n=49 rec=fct-uzysxmnr@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=50 rec=fct-n5zh5jve@1 type=0
what: a source citation for a trigger
@row_none n=51 rec=fct-6ofy4zhs@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=52 rec=fct-ipp77q5u@1 type=0
what: a source citation for a trigger
@row_add n=89 row=decision/no-compliance-audit-trail area=1-obs-scope-summary rec=fct-vqjna3cn@1,fct-i24bcxka@1
name: No compliance audit trail
rule: Observability keeps no compliance audit trail; events.ndjson is the product audit stream, not a compliance trail.
why: the security tier is Minimal and no compliance triggers apply
@row_add n=90 row=decision/normal-mode-not-inverted area=1-obs-scope-summary rec=fct-x2cjy5mn@1
name: Normal observability mode, not inverted
rule: Viola is not a telemetry product: observability runs in normal mode, and events.ndjson is a product audit log, not a telemetry-collector function.
@row_none n=53 rec=fct-23yn4mby@1 type=9
@row_none n=54 rec=fct-q5t2vgpo@1 type=S
@row_none n=55 rec=fct-6zohmrdn@1 type=S
@row_none n=56 rec=fct-nieygaaw@1 type=S
@row_add n=91 row=decision/standard-obs-tier area=2-telemetry-strategy rec=fct-mxljj435@1
name: Standard telemetry tier with Minimal exporter carve-outs
rule: Telemetry follows the Standard signal pyramid, with Minimal-tier carve-outs for exporters.
@row_add n=92 row=overview/2-telemetry-strategy area=2-telemetry-strategy rec=fct-i24bcxka@1
text: Structured logs are the primary signal, one JSON-per-line file per process role; spans are declared in code but not serialized in v1, and cross-process joins use corr and conn; errors have no external reporter; metrics have no in-process instruments and no exporter and are derived by jq over the ndjson; events.ndjson is the product audit stream, not a compliance trail, and obs never copies event payloads into process logs.
@row_add n=93 row=decision/spans-not-serialized-v1 area=2-telemetry-strategy rec=fct-i24bcxka@1
name: Spans declared but not serialized
rule: Spans are declared in code but not serialized in v1.
@row_add n=94 row=decision/metrics-derived-by-jq area=2-telemetry-strategy rec=fct-i24bcxka@1
name: Metrics derived by jq, no instruments
rule: Metrics have no in-process instruments and no exporter; they are derived by jq over the ndjson logs.
@row_none n=57 rec=fct-eqimq7x6@1 type=S
@row_add n=95 row=decision/machine-parseable-signals area=2-telemetry-strategy rec=fct-irnyaxwz@1
name: Every signal machine-parseable
rule: Every signal produces machine-parseable output: JSON-per-line process logs, the ndjson product event stream, JSON harness documents, hyperfine JSON for perf gates and nextest JUnit XML in CI.
@row_add n=96 row=decision/no-human-only-dashboards area=2-telemetry-strategy rec=fct-dpql65fn@1
name: No human-only dashboards
rule: No Grafana, Kibana, Datadog or other human-only dashboard UI exists in the stack, and none may be added as a consumption surface.
@row_add n=97 row=decision/no-proprietary-apm-sdk area=2-telemetry-strategy rec=fct-7dfhzxv4@1
name: No proprietary APM SDK
rule: No proprietary APM SDK is used, as primary or at all; the v1 exporter role is filled by the local file sink.
@row_add n=98 row=decision/no-human-gated-log-analysis area=2-telemetry-strategy rec=fct-i3sjkwgk@1
name: No human-review-gated log analysis
rule: Every log verdict is a jq or harness assertion that sets an exit code, never a human review.
@row_add n=99 row=decision/one-line-panic-record area=2-telemetry-strategy rec=fct-z3yuahd4@1
name: One structured line per panic
rule: Every panic after init step 4 produces exactly one single-line structured record in its home-level role file with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file.
except: the bounded panic exemptions listed in the plan's Section 10
@row_add n=100 row=decision/corr-conn-correlation area=2-telemetry-strategy rec=fct-pcsvls55@1
name: Correlation through corr and conn
rule: Every cross-surface call carries correlation through the corr field plus the additive conn discriminator.
@row_none n=58 rec=fct-pi7x2mau@1 type=S
@row_add n=101 row=decision/span-names-not-wire-contract area=2-telemetry-strategy rec=fct-gyyrakol@1
name: Span names are code-internal
rule: Span names are code-internal and not part of any wire contract.
@row_add n=102 row=contract/process-log-line/field-names area=2-telemetry-strategy rec=fct-rcqby6f3@1
boundary: the process log line
item: field names
promise: Field names are snake_case.
@row_add n=103 row=contract/process-log-line/enum-values area=2-telemetry-strategy rec=fct-rcqby6f3@1
boundary: the process log line
item: enum values
promise: Enum values are kebab-case.
@row_add n=104 row=contract/process-log-line/at-instants area=2-telemetry-strategy rec=fct-rcqby6f3@1
boundary: the process log line
item: *_at
promise: Instants are named *_at and written in RFC 3339 UTC with milliseconds and a Z suffix.
@row_add n=105 row=contract/process-log-line/duration-ms area=2-telemetry-strategy rec=fct-rcqby6f3@1
boundary: the process log line
item: duration_ms
promise: Durations are duration_ms as an integer.
@row_add n=106 row=contract/process-start-event/service-name area=2-telemetry-strategy rec=fct-xd7arejm@1
boundary: the process-start event
item: service_name
promise: Emitted once per process as the fixed value "viola", the binary name.
@row_add n=107 row=contract/process-start-event/version area=2-telemetry-strategy rec=fct-xd7arejm@1
boundary: the process-start event
item: version
promise: Emitted once per process as the crate package version.
@row_none n=59 rec=fct-cloqkzeh@1 type=S
@row_none n=60 rec=fct-fova76zr@1 type=S
@row_none n=61 rec=fct-6wlikjns@1 type=S
@row_none n=62 rec=fct-ikyfhy6h@1 type=S
@row_add n=108 row=contract/hook-output/stdout area=3-observability-harness-contract rec=fct-bemloa4f@1
boundary: hook output streams
item: stdout
promise: A hook writes only its decision body to stdout, and no telemetry reaches stdout.
@row_add n=109 row=contract/hook-output/stderr area=3-observability-harness-contract rec=fct-bemloa4f@1
boundary: hook output streams
item: stderr
promise: A hook writes nothing to stderr.
@row_add n=110 row=contract/hook-output/statusline-stdout area=3-observability-harness-contract rec=fct-bemloa4f@1
boundary: hook output streams
item: viola hook statusline stdout
promise: Carries the user's status-line command output byte for byte; nothing when no command is recorded, on a refused instance, on oversize or malformed stdin, or under the hidden --capture arm.
@row_add n=111 row=contract/run-terminal-output/child-output area=3-observability-harness-contract rec=fct-26ufyipy@2
boundary: run terminal output
item: child output
promise: run writes nothing to the terminal except the child's own output; while the child runs, neither run nor a revived start writes anything of its own.
@row_add n=112 row=contract/run-terminal-output/start-refusal area=3-observability-harness-contract rec=fct-26ufyipy@2
boundary: run terminal output
item: start refusal
promise: A start refusal before any child is spawned writes exactly two fixed stderr lines with no path or pid: the home pair for a home-level refusal, the instance pair for an instance-level one.
@row_add n=113 row=contract/revive-list-stderr/refusal-pair area=3-observability-harness-contract rec=fct-26ufyipy@2
boundary: viola revive --list stderr
item: refusal pair
promise: viola revive --list writes the refusal pair for its two refusals and nothing else on stderr.
@row_add n=114 row=contract/list-stderr/unable-hint-pair area=3-observability-harness-contract rec=fct-26ufyipy@2
boundary: viola list stderr
item: unable / hint pair
promise: The unable / hint: pair carries no SGR colour codes.
@row_add n=115 row=decision/codes-only-process-logs area=3-observability-harness-contract rec=fct-k3pbv5ix@1
name: Codes-only process logs, content in detail files
rule: Every role's codes-only process log goes to its home-level role file under diagnostics, and content-bearing detail goes only to the instance's per-role detail file.
@row_add n=116 row=contract/mcp-output/stdout area=3-observability-harness-contract rec=fct-jujyptfd@1
boundary: mcp output streams
item: stdout
promise: mcp writes only MCP frames to stdout.
@row_add n=117 row=decision/short-lived-verbs-stderr area=3-observability-harness-contract rec=fct-3hlujd4h@1
name: Short-lived verbs use stderr for humans
rule: Short-lived CLI verbs may use stderr for their human output.
@row_add n=118 row=decision/obs-owns-logger-and-format area=3-observability-harness-contract rec=fct-adr52q64@1
name: Obs owns the logger and format
rule: The logger and the log format are owned by observability.
@row_none n=63 rec=fct-qcaucl2u@1 type=S
@row_none n=64 rec=fct-u5bzeuwo@2 type=S
@row_none n=65 rec=fct-rigkqwft@1 type=S
@row_add n=119 row=decision/static-span-names area=4-span-trace-coverage rec=fct-osius4ib@1
name: Static span names
rule: Span names are static strings; instance names, dialog ids and paths never appear in a span name.
@row_add n=120 row=decision/span-field-allow-list area=4-span-trace-coverage rec=fct-osius4ib@1
name: Spans declared with explicit field allow-list
rule: Every span is declared skipping all arguments with an explicit field allow-list.
@row_add n=121 row=decision/no-err-ret-span-options area=4-span-trace-coverage rec=fct-osius4ib@1
name: err and ret span options banned
rule: The err and ret instrumentation options are banned on spans.
@row_add n=122 row=decision/span-kinds-not-emitted-v1 area=4-span-trace-coverage rec=fct-7n4mjpbn@1
name: Span kinds recorded but not emitted
rule: Span kinds are recorded in the plan per span but not emitted in v1; if the deferred tracing-opentelemetry bridge is adopted, they map to the otel.kind span field.
@row_none n=66 rec=fct-ilrtsek3@1 type=S
@row_add n=123 row=decision/manual-spans-only area=4-span-trace-coverage rec=fct-4sjmn35v@1
name: Manual spans only
rule: Every span is written by hand; no surface uses an auto-instrumentation library.
@row_add n=124 row=decision/dependency-targets-off-in-sink area=4-span-trace-coverage rec=fct-4sjmn35v@1
name: rmcp and portable-pty targets off
rule: The rmcp and portable-pty log targets are switched OFF in the sink.
@row_add n=125 row=decision/channel-lines-no-params-or-results area=4-span-trace-coverage rec=fct-4sjmn35v@1
name: Channel lines carry no params or results
rule: Channel log lines never record request params or result bodies.
@row_add n=126 row=decision/uninstrumented-components area=4-span-trace-coverage rec=fct-vjo2uyhn@1
name: Components left uninstrumented
rule: viola-core, config.json and plugin files, and build/CI infrastructure are not instrumented.
why: viola-core only supplies ObsEvent and RefusalReason values; the rest are not instrumentable.
@row_add n=127 row=decision/no-trace-and-span-ids area=4-span-trace-coverage rec=fct-qh2p5n6e@2,fct-wlvn44lx@1,fct-xbpjnox3@1
name: No trace_id or span_id
rule: Log lines never emit trace_id or span_id; correlation is by corr plus conn.
why: No W3C trace context exists, and spans are not attached to lines.
@row_add n=128 row=absence/web-or-graphical-frontend area=4-span-trace-coverage rec=fct-qh2p5n6e@2
capability: A web or graphical frontend
@row_none n=67 rec=fct-krxskcsw@1 type=5
@row_add n=129 row=decision/no-endpoint-path-or-env-in-spans area=4-span-trace-coverage rec=fct-itztmsse@2
name: No endpoint, binary path or env values in spans
rule: A span never carries the endpoint name, the pinned_bin path or environment values.
@row_add n=130 row=decision/child-settings-never-logged area=4-span-trace-coverage rec=fct-itztmsse@2
name: Child settings argument never logged
rule: The claude child's --settings argument appears in no log line or span.
@row_add n=131 row=decision/statusline-command-never-logged area=4-span-trace-coverage rec=fct-itztmsse@2,fct-rwzldnx4@1
name: Statusline command never logged
rule: The statusline command and its source path appear in no log line or span.
@row_add n=132 row=decision/detail-codes-only area=4-span-trace-coverage rec=fct-itztmsse@2,fct-7pburmsm@1
name: detail is a closed code
rule: The detail field is a closed kebab-case code catalog and never carries a path or pid.
@row_none n=68 rec=fct-ophpx6bz@1 type=5
@row_add n=133 row=decision/sent-text-never-logged area=4-span-trace-coverage rec=fct-o5zzhzdu@1
name: Sent text never logged
rule: A sent message's text is never logged; only its size in text_bytes is.
@row_none n=69 rec=fct-gtvcxthg@1 type=5
@row_add n=134 row=decision/no-assistant-message-in-logs area=4-span-trace-coverage rec=fct-q67chfay@2
name: No assistant message or result body in logs
rule: last_assistant_message never appears in a log, and no result body reaches a log line.
@row_add n=135 row=decision/mid-call-disconnect-logged-once area=4-span-trace-coverage rec=fct-q67chfay@2
name: Mid-call disconnect logged once
rule: A disconnect in the middle of a call is logged once, by the client.
@row_add n=136 row=contract/cli-wait-last-connect/server-verify-failed area=4-span-trace-coverage rec=fct-q67chfay@2
boundary: CLI wait / last connect refusal
item: server-verify-failed
promise: A serving process that is not the recorded wrapper or runs under another euid ends the call before any frame with exit 21 and one WARN process-exit line (subject self, during connect) inside the dispatch span, and no channel-request line.
@row_add n=137 row=contract/cli-wait-last-connect/socket-dir-refused area=4-span-trace-coverage rec=fct-q67chfay@2
boundary: CLI wait / last connect refusal
item: socket-dir-refused
promise: A refused socket directory, checked before the server comparison, ends the call before any frame with exit 21 and one WARN process-exit line (during connect), and no channel-request line.
@row_add n=138 row=contract/cli-wait-last-connect/strict-modes-failed area=4-span-trace-coverage rec=fct-q67chfay@2
boundary: CLI wait / last connect refusal
item: strict-modes-failed
promise: A home or instance tree refused by the instance check, checked first, ends the call before any role file opens with exit 21 and no line; under --json the output is exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
@row_add n=139 row=contract/mcp-wait-last-connect/server-verify-failed area=4-span-trace-coverage rec=fct-q67chfay@2
boundary: MCP wait / last connect refusal
item: server-verify-failed
promise: A serving process that is not the recorded wrapper or runs under another euid returns instance-unreachable with detail server-verify-failed and writes no line.
@row_add n=140 row=contract/mcp-wait-last-connect/socket-dir-refused area=4-span-trace-coverage rec=fct-q67chfay@2
boundary: MCP wait / last connect refusal
item: socket-dir-refused
promise: A refused socket directory returns instance-unreachable with detail socket-dir-refused and writes no line.
@row_add n=141 row=contract/mcp-wait-last-connect/strict-modes-failed area=4-span-trace-coverage rec=fct-q67chfay@2
boundary: MCP wait / last connect refusal
item: strict-modes-failed
promise: A home or instance tree refused by the instance check returns instance-unreachable with detail strict-modes-failed and writes no line.
@row_none n=70 rec=fct-mwabsk26@1 type=5
@row_none n=71 rec=fct-qkp2jcsg@1 type=5
@row_none n=72 rec=fct-cbixfjiv@1 type=5
@row_add n=142 row=decision/unwrapped-hook-no-op area=4-span-trace-coverage rec=fct-hxgn7hhb@1,fct-wa67dogf@1
name: Unwrapped hook is a no-op
rule: A hook run without VIOLA_NAME writes no file and no line, exits 0 and leaves stdout and stderr empty.
guard: edge test E1, asserted by absence
@row_add n=143 row=contract/claude-child-process-start/env-stripped-count area=4-span-trace-coverage rec=fct-xnfonqro@1,fct-wa67dogf@1
boundary: process-start line of the claude child
item: env_stripped_count
promise: Reports the R8 environment strip on process-start{subject:"claude-child"}; never an environment value.
@row_add n=144 row=contract/claude-child-process-start/env-stripped-known area=4-span-trace-coverage rec=fct-xnfonqro@1,fct-wa67dogf@1
boundary: process-start line of the claude child
item: env_stripped_known
promise: Reports the R8 environment strip on process-start{subject:"claude-child"} by variable names only, never values.
@row_add n=145 row=contract/claude-child-process-start/env-kept area=4-span-trace-coverage rec=fct-xnfonqro@1,fct-wa67dogf@1
boundary: process-start line of the claude child
item: env_kept
promise: Reports the variables kept after the R8 environment strip on process-start{subject:"claude-child"} by names only, never values.
@row_add n=146 row=decision/link-has-no-own-trace area=4-span-trace-coverage rec=fct-3o7hzpx4@1,fct-wa67dogf@1
name: Link and unlink have no trace of their own
rule: Implicit and explicit link / unlink have no process-log line, event value or span of their own; their trace is the call's channel-request / channel-response pair, the product link / unlink line in events.ndjson and the snapshot's links.
guard: edge test E3
@row_add n=147 row=decision/link-verbs-log-to-driven-instance area=4-span-trace-coverage rec=fct-3o7hzpx4@1,fct-wa67dogf@1
name: Link verbs log to the driven instance
rule: The viola link and viola unlink verbs log into the driven instance's cli-<driven>.ndjson, their request inside the cli.link / cli.unlink span.
@row_add n=148 row=contract/state-recovered-detail/snapshot-replayed area=4-span-trace-coverage rec=fct-iyk74prg@1,fct-wa67dogf@1
boundary: state-recovered detail
item: snapshot-replayed
promise: A state-recovered line at WARN carries this detail, one line per snapshot read call.
@row_add n=149 row=contract/state-recovered-detail/snapshot-unsupported-v area=4-span-trace-coverage rec=fct-iyk74prg@1,fct-wa67dogf@1
boundary: state-recovered detail
item: snapshot-unsupported-v
promise: A state-recovered line at WARN carries this detail, one line per snapshot read call.
@row_add n=150 row=decision/no-endpoint-over-replayed-snapshot area=4-span-trace-coverage rec=fct-iyk74prg@1,fct-wa67dogf@1
name: No endpoint over a replayed snapshot
rule: Over a replayed or absent snapshot read, endpoint lookup and the hook connect give no endpoint, so no frame is sent, and the statusline runs no command.
guard: edge test E5
@row_add n=151 row=contract/state-recovered-detail/torn-line-healed area=4-span-trace-coverage rec=fct-ghtvc6rx@1
boundary: state-recovered detail
item: torn-line-healed
promise: The appender that heals a torn line writes one state-recovered line with this detail per heal.
@row_add n=152 row=decision/reader-never-heals-torn-lines area=4-span-trace-coverage rec=fct-ghtvc6rx@1
name: Reader never heals torn lines
rule: A reader of an append log neither heals a torn line nor logs one; only the appender heals.
@row_add n=153 row=decision/unreadable-ledger-is-unverified area=4-span-trace-coverage rec=fct-g4daqt4h@1
name: Unreadable ledger counts as unverified
rule: A missing stamp, or an unreadable, malformed or strict-modes-refused capability ledger, counts as unverified (cli_verified false).
@row_add n=154 row=decision/verify-logs-only-with-instance area=4-span-trace-coverage rec=fct-bharnesn@1
name: verify and plugin install log only inside an instance
rule: With VIOLA_NAME set, verify and plugin install log process-start / process-exit to cli-<name>.ndjson; from a plain terminal with no instance they write no process-log file.
@row_add n=155 row=decision/capture-arms-uninstrumented area=4-span-trace-coverage rec=fct-bharnesn@1
name: Hidden capture arms uninstrumented
rule: The two hidden capture arms of verify / plugin install are uninstrumented by design.
@row_add n=156 row=decision/verify-lines-redacted area=4-span-trace-coverage rec=fct-bharnesn@1
name: verify lines carry no content
rule: No verify or plugin install line carries row text, a prompt, a session id or a path.
@row_add n=157 row=decision/ci-verify-fake-agent-only area=4-span-trace-coverage rec=fct-bharnesn@1
name: CI verify against the fake agent only
rule: In CI, viola verify runs only against the fake agent.
@row_add n=158 row=contract/viola-verify-refusal/home-tree-refused area=4-span-trace-coverage rec=fct-bharnesn@1
boundary: viola verify strict-modes refusal
item: home tree refused
promise: Before logging, verify holds the home, ledger/, bin/ and plugin/ to the strict-modes check; a refused tree exits 1 with the home pair and writes no line.
@row_add n=159 row=contract/viola-verify-refusal/stamp-write-refused area=4-span-trace-coverage rec=fct-bharnesn@1
boundary: viola verify strict-modes refusal
item: stamp write refused
promise: A ledger/stamps.json the stamp write refuses prints the home pair and exits 1 through verify's own process-exit line.
@row_none n=73 rec=fct-rtdpsbcq@1 type=S
@row_none n=74 rec=fct-cbntkles@1 type=S
@row_add n=160 row=decision/metrics-derived-from-logs area=5-metric-coverage rec=fct-kvmjumn6@1,fct-x6ueqkr2@1
name: Metrics derived from logs
rule: Every metric is logical, computed by jq or the harness over the JSON-per-line logs or from hyperfine output, never by OTel or metrics-crate instruments.
why: There is no reader or exporter to flush instruments.
@row_add n=161 row=decision/metric-grouping-closed-enums area=5-metric-coverage rec=fct-5mkprlwu@1
name: Metric grouping by closed enums only
rule: Aggregate reports group only by closed enums (event, detail, refusal, method, hook_event, subject, parser, process); corr, conn, pid, child_pid and instance are join keys, never grouping keys.
@row_none n=75 rec=fct-zhhme5jd@1 type=S
@row_none n=76 rec=fct-qsbtfoen@1 type=S
@row_none n=77 rec=fct-546mputj@1 type=S
@row_add n=162 row=decision/log-schema-additive-only area=6-log-coverage rec=fct-wk2jyutc@1
name: Log schema additive to the tests' format
rule: The observability log schema only adds fields and enum values to the tests' log format; it renames and removes nothing.
why: The tests' log format is the binding text.
@row_add n=163 row=contract/log-line/timestamp area=6-log-coverage rec=fct-qwey6fxj@1,fct-3i3k7olh@1
boundary: every log line
item: timestamp
promise: RFC 3339 UTC with milliseconds and Z.
@row_add n=164 row=contract/log-line/level area=6-log-coverage rec=fct-vagpwmim@1,fct-3i3k7olh@1
boundary: every log line
item: level
promise: One of DEBUG, INFO, WARN, ERROR; TRACE is never emitted.
@row_add n=165 row=contract/log-line/target area=6-log-coverage rec=fct-324giizr@1,fct-3i3k7olh@1
boundary: every log line
item: target
promise: The Rust module path, for example viola_channel::server.
@row_add n=166 row=contract/log-line/message area=6-log-coverage rec=fct-gmpqxlxj@1,fct-3i3k7olh@1
boundary: every log line
item: message
promise: Flattened, a static literal equal to the event name.
@row_add n=167 row=contract/log-line/event area=6-log-coverage rec=fct-6pnuxsb3@1,fct-3i3k7olh@1
boundary: every log line
item: event
promise: A value of the closed kebab-case event enum.
@row_add n=168 row=contract/log-line/instance area=6-log-coverage rec=fct-ev4mirl5@1,fct-3i3k7olh@1
boundary: every log line
item: instance
promise: The ViolaName, or null.
@row_add n=169 row=contract/process-start-line/service-name area=6-log-coverage rec=fct-iee6b2de@1,fct-xbpjnox3@1
boundary: process-start line
item: service_name
promise: A constant service name, emitted once on process-start rather than on every line.
@row_add n=170 row=decision/no-event-type-field area=6-log-coverage rec=fct-w3dtohlr@1,fct-xbpjnox3@1
name: No event_type field
rule: Log lines carry no event_type field.
why: event already does that job, and a duplicate would amount to a rename.
@row_add n=171 row=decision/additive-field-naming area=6-log-coverage rec=fct-o3exhcyn@1
name: Additive log field naming
rule: Additive log fields are snake_case with kebab-case values, and carry codes only.
@row_add n=172 row=decision/skipped-symlink-never-logged area=6-log-coverage rec=fct-gr4iywnh@1
name: Skipped symlink never named in logs
rule: The entry name and link target of a skipped symlink are never logged.
@row_none n=78 rec=fct-illbksdl@1 type=S
@row_none n=79 rec=fct-42q5kq2o@1 type=S
@row_add n=173 row=contract/log-levels/error area=6-log-coverage rec=fct-boem2fct@1,fct-vfra73hn@1
boundary: log levels
item: error
promise: Used for a panic, a process-exit with exit_code 1 or 20, and a channel-response with error_code -32603.
@row_add n=174 row=contract/log-levels/warn area=6-log-coverage rec=fct-ufkep7p5@1
boundary: log levels
item: warn
promise: WARN carries state-recovered; parse-rejected; hook-decision with a fail-open detail; process-exit with exit 21; channel-response with -32600 or -32602.
@row_add n=175 row=contract/log-levels/info area=6-log-coverage rec=fct-remsrxge@1
boundary: log levels
item: info
promise: INFO carries all other boundary, lifecycle and state-transition events, including refusals, which are normal product outcomes.
@row_add n=176 row=contract/log-levels/debug area=6-log-coverage rec=fct-vji36ghd@1
boundary: log levels
item: debug
promise: DEBUG is reserved for per-frame channel framing detail and uses the same field allow-list.
@row_add n=177 row=decision/debug-level-by-config-only area=6-log-coverage rec=fct-vji36ghd@1
name: Debug level enabled by config only
rule: The debug log level is enabled only by diagnostics_level "debug" in config.json, never by RUST_LOG.
@row_add n=178 row=contract/log-levels/trace area=6-log-coverage rec=fct-c3cqwtsl@1
boundary: log levels
item: trace
promise: TRACE is never emitted; the level exists only in the tests' level enum.
@row_none n=80 rec=fct-5nu2dy7a@1 type=S
@row_none n=81 rec=fct-ee4kl3w4@1 type=S
@row_add n=179 row=decision/viola-crates-log-at-info area=6-log-coverage rec=fct-h3nedfqn@1,fct-ffszh776@1
name: Viola crates log at INFO
rule: The per-module log filter sets the viola, viola_core, viola_pty, viola_channel, viola_state, viola_agent_claude and viola_mcp targets to INFO, or DEBUG via diagnostics_level.
@row_add n=180 row=decision/third-party-crate-logs-off area=6-log-coverage rec=fct-7e4jvssp@1,fct-ffszh776@1
name: Third-party crate logs off
rule: The per-module log filter turns logging OFF for the rmcp, portable_pty, sysinfo and interprocess targets.
why: Their failures are captured at the viola seam as viola events.
@row_add n=181 row=decision/unlisted-targets-log-off area=6-log-coverage rec=fct-qberstet@1,fct-ffszh776@1
name: Unlisted log targets off
rule: The per-module log filter turns logging OFF for every target it does not list.
@row_add n=182 row=decision/file-only-per-role-log-sinks area=6-log-coverage rec=fct-fdakcnvs@1,fct-xeoybvbw@1
name: File-only log sinks per role
rule: Diagnostics are written to files only, one sink per role, under the home's diagnostics directory.
@row_add n=183 row=decision/content-detail-in-instance-file area=6-log-coverage rec=fct-fdakcnvs@1,fct-xeoybvbw@1
name: Content-bearing detail in the instance detail file
rule: Content-bearing detail goes only to the per-process detail file inside the instance's diagnostics directory.
@row_add n=184 row=decision/json-to-file-only-log-mode area=6-log-coverage rec=fct-p2v4kjoq@1
name: JSON-to-file as the only log mode
rule: JSON to file is the only log mode: there is no agent-mode flag and no stdout pretty sink on any role.
@row_none n=82 rec=fct-rv3gmelk@1 type=S
@row_add n=185 row=contract/ipc-invocation-log/fields area=6-log-coverage rec=fct-qmzbmwv2@1
boundary: IPC invocation log lines
item: fields
promise: Each channel-request and channel-response line carries method, corr, conn, result class, error code and sender.
@row_add n=186 row=decision/ipc-calls-logged-without-arguments area=6-log-coverage rec=fct-qmzbmwv2@1
name: IPC calls logged without arguments
rule: IPC params and result bodies are never recorded, not even as a hash or digest.
@row_add n=187 row=decision/euid-refused-peer-writes-no-line area=6-log-coverage rec=fct-qmzbmwv2@1
name: Euid-refused peer writes no line
rule: A channel peer refused by euid is dropped before its worker thread and before any frame, writes no log line, and the client reads end-of-file.
why: There is no frame to key a line on, and channel-response's detail is closed.
@row_add n=188 row=contract/pty-seam-log/spawn area=6-log-coverage rec=fct-3epowryi@1
boundary: PTY seam log events
item: spawn
promise: A spawn gives process-start{subject:"claude-child"}.
@row_add n=189 row=contract/pty-seam-log/kill-fallback area=6-log-coverage rec=fct-3epowryi@1
boundary: PTY seam log events
item: kill-fallback
promise: An exit through the kill fallback is logged with exit_source:"kill-fallback".
@row_add n=190 row=contract/pty-seam-log/paste area=6-log-coverage rec=fct-3epowryi@1
boundary: PTY seam log events
item: paste
promise: A paste is logged with text_bytes only.
@row_add n=191 row=contract/child-spawn-log/subject area=6-log-coverage rec=fct-7eai74no@1
boundary: child and shell spawn log events
item: subject
promise: process-start and process-exit lines for child and shell spawns carry a subject from a closed set.
@row_add n=192 row=contract/child-spawn-log/claude-agents-json-parse-failure area=6-log-coverage rec=fct-7eai74no@1
boundary: child and shell spawn log events
item: claude agents --json parse failure
promise: A claude agents --json parse failure gives one parse-rejected{parser:"claude-agents-json"} line and the value "unknown".
@row_add n=193 row=absence/database area=6-log-coverage rec=fct-oqqvdowc@1
capability: A database
@row_none n=83 rec=fct-tpoc756b@1 type=S
@row_none n=84 rec=fct-rqrxq7s4@1 type=S
@row_none n=85 rec=fct-tf3qxuqd@1 type=S
@row_add n=194 row=decision/local-error-capture-only area=7-error-capture-reporting rec=fct-nwrbbhk2@1,fct-bx64zmwr@1
name: Local error capture only
rule: Errors are captured locally only; there is no external error-reporting platform and no reporter to opt into.
@row_add n=195 row=decision/typed-errors-fixed-display area=7-error-capture-reporting rec=fct-nwrbbhk2@1,fct-i5zqforo@1
name: Typed errors with fixed Display messages
rule: Every typed error has a fixed-message Display, written with thiserror, and holds no fields with paths or payloads.
except: PtyError's Display is hand-written, as viola-pty takes no thiserror.
@row_add n=196 row=decision/anyhow-at-dispatch-edge-only area=7-error-capture-reporting rec=fct-nwrbbhk2@1
name: anyhow at the dispatch edge only
rule: anyhow is used only at the root binary's dispatch edge and its catch-site reporter.
@row_add n=197 row=decision/hand-written-panic-capture area=7-error-capture-reporting rec=fct-nwrbbhk2@1
name: Hand-written panic capture
rule: Panic capture is a hand-written std panic hook plus std catch_unwind, with no panic-reporting crate.
@row_add n=198 row=decision/sentry-not-admissible area=7-error-capture-reporting rec=fct-s7d7nvvm@1
name: Sentry not admissible
rule: sentry and sentry-tracing are not admissible.
why: They break the egress ban, and their default reqwest, native-tls and tokio transport breaks the cargo-deny C-crate and Tokio-in-sync-crates bans.
@row_add n=199 row=decision/tracing-panic-rejected area=7-error-capture-reporting rec=fct-hbeo3r33@1
name: tracing-panic rejected
rule: tracing-panic is not used.
why: The crate is stale.
@row_add n=200 row=decision/human-panic-rejected area=7-error-capture-reporting rec=fct-hbeo3r33@1
name: human-panic rejected
rule: human-panic is not used.
why: It writes a human report to stderr.
@row_add n=201 row=decision/future-reporter-ureq-rustls area=7-error-capture-reporting rec=fct-s5b2ujf4@1,fct-bx64zmwr@1
name: Future reporter uses ureq and rustls
rule: Any future error reporter must use the ureq and rustls transport.
@row_add n=202 row=decision/future-reporter-never-log-floor area=7-error-capture-reporting rec=fct-l46fiqal@1,fct-bx64zmwr@1
name: Future reporter applies the NEVER-log floor
rule: Any future error reporter must apply the NEVER-log floor in before_send.
@row_add n=203 row=decision/future-reporter-off-by-default area=7-error-capture-reporting rec=fct-5tiqkcpp@1,fct-bx64zmwr@1
name: Future reporter off by default
rule: Any future error reporter must be off by default.
@row_add n=204 row=decision/future-reporter-not-env-enabled area=7-error-capture-reporting rec=fct-i665y2l6@1,fct-bx64zmwr@1
name: Future reporter never enabled by env var
rule: Any future error reporter must never be enabled by an environment variable.
@row_add n=205 row=decision/future-reporter-needs-egress-entry area=7-error-capture-reporting rec=fct-uzwm6rhy@1,fct-bx64zmwr@1
name: Future reporter needs a security egress entry
rule: Any future error reporter also needs a security Decisions Log entry for its egress.
@row_add n=206 row=decision/scrubbing-at-source area=7-error-capture-reporting rec=fct-ugpuuxhx@1
name: Scrubbing at the source
rule: Error scrubbing sits at the source, not in a reporter's before_send hook.
why: No reporter exists.
@row_add n=207 row=decision/serde-errors-become-internal-error area=7-error-capture-reporting rec=fct-nmxgzm4p@1,fct-ugpuuxhx@1
name: Serde error chains replaced by internal error
rule: At dispatch, an error chain whose source is a serde_json or serde_path_to_error error is replaced by the fixed message "internal error" before it reaches stderr, --json, MCP isError or channel error.data.
why: A scrubbing layer at the source.
@row_add n=208 row=decision/full-error-chain-in-detail-file-only area=7-error-capture-reporting rec=fct-hesjwzzx@1,fct-ugpuuxhx@1
name: Full error chain in the detail file only
rule: The full error chain is written as a one-line JSON array chain:[...] only to the instance's per-process detail file, and only when an instance resolves.
@row_none n=86 rec=fct-q3jgjpq3@1 type=S
@row_add n=209 row=decision/panic-hook-silent-on-stderr area=7-error-capture-reporting rec=fct-iis47bt2@1
name: Panic hook installed first, silent on stderr
rule: Viola's panic hook is installed by main's first statement, never calls the default hook and never writes to stderr.
why: The hook contract and the run terminal both forbid it.
@row_add n=210 row=decision/panic-line-bypasses-tracing area=7-error-capture-reporting rec=fct-7mvwq4nm@1
name: Panic line bypasses tracing
rule: The panic hook writes its one JSON line with a single write to a pre-opened file, not through the tracing subscriber.
why: No re-entrancy if the panic happened inside formatting.
@row_add n=211 row=contract/panic-line/keys area=7-error-capture-reporting rec=fct-7mvwq4nm@1
boundary: the panic line
item: keys
promise: The panic line holds timestamp, level "ERROR", target "viola::panic", message "panic", event "panic", process, instance, panic_location and thread.
@row_add n=212 row=contract/panic-line/corr area=7-error-capture-reporting rec=fct-7mvwq4nm@1
boundary: the panic line
item: corr
promise: corr is never written on the panic line.
@row_add n=213 row=contract/panic-line/instance area=7-error-capture-reporting rec=fct-7mvwq4nm@1
boundary: the panic line
item: instance
promise: instance is omitted when none resolved, the same key-absence null encoding obs_event! uses.
@row_add n=214 row=decision/panic-backtrace-in-detail-file area=7-error-capture-reporting rec=fct-yn6razq4@1
name: Panic backtrace in the detail file
rule: When an instance resolves, the panic hook also writes the payload and a raw, never-symbolised backtrace as one line to the process's detail file, not gated by RUST_BACKTRACE.
why: Env vars are not a configuration channel.
@row_add n=215 row=decision/panic-exit-decided-at-catch-site area=7-error-capture-reporting rec=fct-sue3fxzz@1
name: Panic exit decided at a catch site
rule: The panic hook only writes; the exit is decided by one catch_unwind in the root binary's main around everything after the hook is set, so no role ends through an uncaught unwind.
@row_add n=216 row=decision/run-threads-inside-catch-unwind area=7-error-capture-reporting rec=fct-sue3fxzz@1
name: Run threads inside catch_unwind
rule: The run PTY pump and handle-wait threads each run inside catch_unwind; a worker panic that leaves main running with no process-exit is forbidden.
@row_add n=217 row=decision/panic-unwind-in-all-profiles area=7-error-capture-reporting rec=fct-sue3fxzz@1
name: panic = unwind in every profile
rule: Every Cargo profile keeps panic = "unwind".
guard: G3
@row_add n=218 row=contract/panic-outcome-by-role/hook area=7-error-capture-reporting rec=fct-x3mpnm6q@1
boundary: panic outcome by role
item: hook
promise: After a panic, hook exits 0 with empty stdout.
@row_add n=219 row=contract/panic-outcome-by-role/cli area=7-error-capture-reporting rec=fct-x3mpnm6q@1
boundary: panic outcome by role
item: cli
promise: After a panic, cli logs process-exit{subject:"self", exit_code:1, detail:"internal-error"} when an instance resolves, puts the chain in the detail file only, writes exactly "error: internal error\n" on stderr in one write with no hint, and exits 1.
@row_add n=220 row=contract/panic-outcome-by-role/cli-json area=7-error-capture-reporting rec=fct-x3mpnm6q@1
boundary: panic outcome by role
item: cli --json
promise: After a panic under --json, cli writes {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty, and exits 1.
@row_add n=221 row=contract/panic-outcome-by-role/run area=7-error-capture-reporting rec=fct-x3mpnm6q@1
boundary: panic outcome by role
item: run
promise: After a panic, run logs process-exit{subject:"self", exit_code:1, detail:"internal-error"} and exits 1, writing nothing on the terminal while the child runs.
@row_add n=222 row=contract/panic-outcome-by-role/mcp-task-panic area=7-error-capture-reporting rec=fct-x3mpnm6q@1
boundary: panic outcome by role
item: mcp spawned-task panic
promise: A panic inside an mcp spawned Tokio task is logged by the hook, contained at the task boundary, and the process continues.
@row_add n=223 row=contract/panic-outcome-by-role/mcp-main-thread-panic area=7-error-capture-reporting rec=fct-x3mpnm6q@1
boundary: panic outcome by role
item: mcp main-thread panic
promise: An mcp main-thread panic writes no process-exit line and nothing to stdout, and the process exits 1.
@row_none n=87 rec=fct-xpt2kzwy@1 type=4
relied: A panic before any role file is opened (clap parsing, home and instance checks) leaves no log record.
@row_add n=224 row=decision/unhandled-panics-always-captured area=7-error-capture-reporting rec=fct-236mfl4p@1,fct-u2647lgq@1
name: Unhandled panics always captured
rule: Every unhandled panic is captured as the one-line event "panic".
@row_add n=225 row=decision/boundary-errors-end-in-logged-line area=7-error-capture-reporting rec=fct-yc4xljp5@1,fct-u2647lgq@1
name: Crate-seam errors end in a logged line
rule: Every typed error that crosses a crate seam ends in a logged process-exit detail, channel-response error, hook-decision detail or send-refused line, carrying codes only at WARN or ERROR.
@row_add n=226 row=decision/vt100-panics-contained area=7-error-capture-reporting rec=fct-gxdjrwob@1,fct-u2647lgq@1
name: vt100 panics contained
rule: vt100 panics are caught around each feed and resize on run's feed thread; each poisoning episode gives exactly one parse-rejected{parser:"vt100-feed"} at WARN, and run continues.
@row_add n=227 row=decision/mcp-rejects-codes-only area=7-error-capture-reporting rec=fct-pqz5mgd5@2
name: MCP rejects are codes only
rule: MCP rejects are isError codes only, and a logged reject's line holds codes only.
@row_add n=228 row=decision/instance-unreachable-not-logged area=7-error-capture-reporting rec=fct-pqz5mgd5@2
name: instance-unreachable logged by no line
rule: An MCP instance-unreachable reject, from any instance, liveness, socket-directory, euid or server-verification check, is logged by no line; the isError code is its only record.
@row_add n=229 row=decision/error-correlation-without-trace-ids area=7-error-capture-reporting rec=fct-kd3a4grv@1
name: Error correlation without trace ids
rule: Errors correlate by corr, conn, process and instance, plus timestamp, not by trace_id or span_id.
@row_add n=230 row=contract/detail-file-line/correlation-keys area=7-error-capture-reporting rec=fct-kd3a4grv@1
boundary: the detail-file error line
item: correlation keys
promise: The detail-file line for an error carries the same event, corr and conn as its home-level counterpart, so jq joins them.
@row_none n=88 rec=fct-6be5cxpd@1 type=S
@row_none n=89 rec=fct-ytzmkt6r@1 type=S
@row_none n=90 rec=fct-rbwf5ypa@1 type=S
@row_none n=91 rec=fct-s2a63rdo@1 type=S
@row_add n=231 row=decision/secrets-never-logged area=8-pii-scrubbing-compliance rec=fct-btgnwmtl@1
name: Secrets never logged
rule: The Cookie header, signing credentials and R8-stripped CLAUDE* values are never logged.
@row_add n=232 row=decision/ci-secrets-not-in-artifacts area=8-pii-scrubbing-compliance rec=fct-btgnwmtl@1
name: CI secrets never in artifacts
rule: CI secrets are never echoed into artifacts.
@row_add n=233 row=decision/conversation-text-only-in-detail-file area=8-pii-scrubbing-compliance rec=fct-btgnwmtl@1
name: Conversation text only in the instance detail file
rule: Prompt, assistant, tool-input, plan and dialog text never reach a home-level line, only an instance's per-process detail file inside a chain or drift report.
@row_add n=234 row=decision/statusline-command-out-of-home-logs area=8-pii-scrubbing-compliance rec=fct-btgnwmtl@1
name: statusline_command out of home-level logs
rule: statusline_command stays out of home-level logs.
@row_add n=235 row=decision/endpoint-and-pinned-bin-never-logged area=8-pii-scrubbing-compliance rec=fct-qk7hj63u@1
name: endpoint and pinned_bin never logged
rule: The endpoint and pinned_bin values are never logged.
@row_add n=236 row=decision/claude-agents-rows-never-logged area=8-pii-scrubbing-compliance rec=fct-qk7hj63u@1
name: claude agents rows never logged verbatim
rule: claude agents --json rows are never logged verbatim, only parse-rejected counts.
@row_add n=237 row=decision/low-class-metadata-as-typed-fields area=8-pii-scrubbing-compliance rec=fct-qk7hj63u@1
name: Low-class metadata as typed fields
rule: Low-class metadata is logged as typed fields, never inside detail codes.
@row_none n=92 rec=fct-2jywf42w@1 type=S
@row_add n=238 row=decision/veil-redact-second-layer area=8-pii-scrubbing-compliance rec=fct-nfaxfmcg@1
name: veil Redact as the second redaction layer
rule: veil's derive Redact is the second redaction layer, so a stray value capture prints [REDACTED] instead of content.
@row_add n=239 row=decision/veil-toggle-feature-banned area=8-pii-scrubbing-compliance rec=fct-nfaxfmcg@1
name: veil toggle feature banned
rule: veil's non-default toggle feature, which lets redaction be disabled at runtime, is never enabled.
guard: cargo-deny bans.features entry denying toggle on veil
@row_add n=240 row=decision/no-regex-scrubbers area=8-pii-scrubbing-compliance rec=fct-z6ygfhpz@1
name: No regex scrubbers
rule: Sensitive data is never scrubbed by regular expressions.
why: viola's sensitive data is known by type
@row_add n=241 row=decision/redact-crate-not-adopted area=8-pii-scrubbing-compliance rec=fct-34ylsxhv@1
name: redact crate not adopted
rule: The redact crate, the researched lighter alternative to veil, is not adopted.
@row_add n=242 row=absence/sentry-error-reporting area=8-pii-scrubbing-compliance rec=fct-xpgyosmo@1
capability: Sentry error reporting
@row_add n=243 row=decision/source-allow-list-logging area=8-pii-scrubbing-compliance rec=fct-n2ewvgzr@1
name: Source allow-list for logged fields
rule: Spans skip all arguments and declare fields explicitly, and events go through obs_event! with explicit typed fields only; this source allow-list is the primary PII control.
guard: clippy disallowed-macros on tracing level macros; fail-closed raw event! grep; grep gate on bare instrument
@row_add n=244 row=decision/payload-types-derive-redact area=8-pii-scrubbing-compliance rec=fct-ijoesqsa@1
name: Type-level redaction of payload types
rule: Every channel, hook and MCP payload type derives veil Redact, so Debug never reveals content.
@row_add n=245 row=decision/fixed-error-messages area=8-pii-scrubbing-compliance rec=fct-6wyihaqe@1
name: Fixed error messages
rule: Error Display texts are fixed, and serde-sourced error chains are mapped to fixed messages at dispatch.
@row_add n=246 row=decision/content-only-in-detail-file area=8-pii-scrubbing-compliance rec=fct-42flcsyl@1
name: Content-bearing records only in the detail file
rule: Content-bearing records go only to the instance-scoped detail-<process>.ndjson file.
@row_add n=247 row=decision/content-canary-never-home-level area=8-pii-scrubbing-compliance rec=fct-tcjwyf2p@1
name: Content canary never in home-level files
rule: The content canary never appears in a home-level diagnostics ndjson file.
@row_add n=248 row=decision/detail-upload-synthetic-inputs-only area=8-pii-scrubbing-compliance rec=fct-tcjwyf2p@1
name: Detail-file upload only with synthetic inputs
rule: Detail files and the harness capture are uploaded only while every input is synthetic; a non-synthetic source first drops them from the upload paths.
why: every input is synthetic, so detail content carries no real data
@row_add n=249 row=decision/scan-failure-uploads-hit-report-only area=8-pii-scrubbing-compliance rec=fct-tcjwyf2p@1,fct-hmk2eyfw@1
name: Only the hit report uploaded on scan failure
rule: When the secret scan fails, only its hit report is uploaded, as the secret-scan-<os> artifact.
@row_add n=250 row=contract/secret-scan-hit-report/file area=8-pii-scrubbing-compliance rec=fct-fxeyaiva@1
boundary: the secret-scan hit report
item: file
promise: Each hit names the file it was found in.
@row_add n=251 row=contract/secret-scan-hit-report/line area=8-pii-scrubbing-compliance rec=fct-fxeyaiva@1
boundary: the secret-scan hit report
item: line
promise: Each hit names the line it was found on.
@row_add n=252 row=contract/secret-scan-hit-report/byte-offset area=8-pii-scrubbing-compliance rec=fct-fxeyaiva@1
boundary: the secret-scan hit report
item: byte offset
promise: Each hit names the byte offset of the match.
@row_add n=253 row=contract/secret-scan-hit-report/pattern-class area=8-pii-scrubbing-compliance rec=fct-fxeyaiva@1
boundary: the secret-scan hit report
item: pattern class
promise: Each hit names the class of pattern that matched.
@row_add n=254 row=decision/hit-report-never-matched-bytes area=8-pii-scrubbing-compliance rec=fct-fxeyaiva@1
name: Hit report never carries matched bytes
rule: The secret-scan hit report never contains the matched bytes.
@row_add n=255 row=decision/upload-members-scanned-or-dropped area=8-pii-scrubbing-compliance rec=fct-onsbpcol@1
name: Upload members scanned or dropped
rule: An uploaded member that could carry a host path or log content is scanned before upload or dropped; unscanned uploads are admissible by content only.
@row_add n=256 row=decision/local-run-outputs-never-uploaded area=8-pii-scrubbing-compliance rec=fct-onsbpcol@1
name: Local run outputs never uploaded
rule: mutants.out, the run archive and the local viola-harness pre-push document are never uploaded.
@row_add n=257 row=decision/fuzz-artifacts-synthetic-seeds-only area=8-pii-scrubbing-compliance rec=fct-onsbpcol@1
name: Fuzz artifacts uploaded only with synthetic seeds
rule: A non-synthetic fuzz corpus seed first drops the fuzz artifacts upload.
@row_add n=258 row=decision/default-deny-log-fields area=8-pii-scrubbing-compliance rec=fct-llmksgvm@1
name: Default-deny for logged fields
rule: A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure.
guard: gate G4 against schemas/diag-line.v1.json
@row_add n=259 row=decision/schema-top-level-unevaluated-properties area=8-pii-scrubbing-compliance rec=fct-llmksgvm@1
name: Default-deny schema via top-level unevaluatedProperties
rule: The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists.
why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then
guard: contract_diag_schema
@row_add n=260 row=decision/no-allow-list-widening area=8-pii-scrubbing-compliance rec=fct-llmksgvm@1
name: Allow-list cannot be widened
rule: No config key, flag or env var can widen the logged-field allow-list or disable redaction.
@row_add n=261 row=decision/detail-schema-self-contained area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
name: Separate self-contained detail-line schema
rule: Detail-file lines are validated by their own schema with a top-level unevaluatedProperties false, which inlines the event enum and uses no cross-file reference.
@row_add n=262 row=contract/detail-line/timestamp area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: timestamp
promise: Required on every detail-file line, as on home-level lines.
@row_add n=263 row=contract/detail-line/level area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: level
promise: Required on every detail-file line, as on home-level lines.
@row_add n=264 row=contract/detail-line/target area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: target
promise: Required on every detail-file line, as on home-level lines.
@row_add n=265 row=contract/detail-line/message area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: message
promise: Required on every detail-file line, as on home-level lines.
@row_add n=266 row=contract/detail-line/event area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: event
promise: Required on every detail-file line, as on home-level lines.
@row_add n=267 row=contract/detail-line/process area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: process
promise: Required on every detail-file line, as on home-level lines.
@row_add n=268 row=contract/detail-line/instance area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: instance
promise: Required on every detail-file line, as on home-level lines.
@row_add n=269 row=contract/detail-line/corr area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: corr
promise: Carried on a detail-file line when its home-level counterpart carries it.
@row_add n=270 row=contract/detail-line/conn area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: conn
promise: Carried on a detail-file line when its home-level counterpart carries it.
@row_add n=271 row=contract/detail-line/panic-location area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: panic_location
promise: Allowed on panic detail lines only, copied from the home-level panic line so both lines join without the payload.
@row_add n=272 row=contract/detail-line/thread area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: thread
promise: Allowed on panic detail lines only, copied from the home-level panic line so both lines join without the payload.
@row_add n=273 row=contract/detail-line/panic-payload area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: panic_payload
promise: One of exactly four content fields a detail-file line may carry: the panic's payload.
@row_add n=274 row=contract/detail-line/backtrace area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: backtrace
promise: One of exactly four content fields a detail-file line may carry: the panic's backtrace.
@row_add n=275 row=contract/detail-line/chain area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: chain
promise: One of exactly four content fields a detail-file line may carry: the full error chain.
@row_add n=276 row=contract/detail-line/drift-report area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: drift_report
promise: One of exactly four content fields a detail-file line may carry: the drift report.
@row_none n=93 rec=fct-fhqex5we@1 type=S
@row_none n=94 rec=fct-cdgkifg4@1 type=S
@row_none n=95 rec=fct-tzfcwoax@1 type=S
@row_add n=277 row=decision/ci-on-github-actions area=9-ci-integration rec=fct-xyexk37w@2
name: CI on GitHub Actions
rule: CI runs on GitHub Actions in one push and pull-request workflow on an ubuntu-latest matrix, beside a scheduled nightly workflow for advisory checks and fuzzing; there is no OTLP collector job.
@row_none n=96 rec=fct-7bv4tsaz@1 type=S
@row_none n=97 rec=fct-y73cmz67@1 type=S
@row_add n=278 row=decision/test-homes-under-e2e-home area=9-ci-integration rec=fct-xtfnzpyu@1
name: Integration-test homes under one root
rule: Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it.
why: a hook panic exits 0, so a home outside that root would hide it
except: the one chaos_feed_panic test home
@row_none n=98 rec=fct-rzzg2wyr@2 type=6
@row_add n=279 row=decision/g2-zero-panics-gate area=9-ci-integration rec=fct-eodqccd6@1
name: Zero-panics gate fails closed
rule: The zero-panics gate probes itself first, fails closed, runs in the test and perf jobs, and counts panics over home-level role files only.
guard: G2
except: the fake-agent seam's forced panic at exactly its seam location, compared as a whole string
@row_add n=280 row=decision/g4-schema-conformance-gate area=9-ci-integration rec=fct-htkftgtl@1
name: Schema conformance gate on every line
rule: Every home-level line is validated against the diagnostics line schema and every detail line against the detail schema, in a step that always runs.
guard: G4
@row_add n=281 row=decision/gate-output-never-line-content area=9-ci-integration rec=fct-htkftgtl@1
name: Schema gate output never shows line content
rule: The schema conformance gate prints file, line and keyword of a failure, never the line's content.
guard: G4
@row_add n=282 row=decision/no-abort-panic-strategy area=9-ci-integration rec=fct-2rxslar4@1
name: No abort panic strategy
rule: No panic strategy is set to abort in Cargo.toml, cargo config or workflows; the gate passes only on a clean no-match.
guard: G3
@row_add n=283 row=decision/no-ci-attributes-in-log-lines area=9-ci-integration rec=fct-z7z7o2va@1
name: No CI resource attributes in log lines
rule: deployment.environment, ci.run.id and git.commit.sha are never injected into viola log lines; CI identity lives in artifact names and GitHub run metadata.
why: env vars are not a configuration channel
@row_add n=284 row=decision/artifact-retention-seven-days area=9-ci-integration rec=fct-gc3zcqcq@1
name: Seven-day artifact retention
rule: CI artifacts are retained for 7 days, below the 90-day default.
@row_none n=99 rec=fct-7cctnpvb@1 type=S
@row_add n=285 row=decision/panic-and-schema-gates-always-run area=9-ci-integration rec=fct-4573rvoy@2
name: Panic and schema gates always run
rule: The zero-panics and schema gates run after the integration and E2E tests even when a test step failed.
why: a failed test step must not skip the zero-panics gate
@row_add n=286 row=decision/perf-gates-own-job area=9-ci-integration rec=fct-4573rvoy@2
name: Perf gates in their own job
rule: The hyperfine performance gates run in their own perf job.
@row_add n=287 row=decision/homes-kept-until-gates-run area=9-ci-integration rec=fct-4573rvoy@2
name: Test homes kept until the gates run
rule: Integration-test homes are kept until the gate steps have run.
@row_none n=100 rec=fct-5ybtic5h@1 type=6
@row_add n=288 row=decision/secret-scan-always-runs area=9-ci-integration rec=fct-xwehypb2@1
name: Secret scan always runs
rule: The secret scan step runs even when an earlier step failed.
why: otherwise a failed E2E step skips the scan and the gated uploads, which loses the failing job's diagnostics
@row_add n=289 row=decision/secret-scan-scope area=9-ci-integration rec=fct-xwehypb2@1
name: Secret scan scope
rule: The secret scan covers every test-home diagnostics file, the harness capture and the nextest JUnit file.
why: nextest stores failing tests' captured output in the JUnit file
except: the mutants run's chunk diff, skipped by exact path and excluded from the harness upload
@row_none n=101 rec=fct-d2xottqv@1 type=S
@row_none n=102 rec=fct-oij4u53u@1 type=S
@row_none n=103 rec=fct-7ivmlbcp@1 type=S
@row_none n=104 rec=fct-axh4ampq@1 type=S
@row_add n=290 row=decision/no-retry-policies area=10-slo-invariants-telemetry-budgets rec=fct-3qkbv2ca@1,fct-uhtssi46@1
name: Retry policies forbidden
rule: Retry-once policies are forbidden, for tests and for panics alike.
why: retry policies mask real signals
@row_add n=291 row=decision/exit-code-panic-evidence area=10-slo-invariants-telemetry-budgets rec=fct-73zqjpsk@1
name: Exit-code panic evidence
rule: A panic evidenced by an exit code without its log line fails the build only for a process whose process-start is in a home-level role file; for the exemptions the exit status is the only evidence.
@row_add n=292 row=decision/hook-panic-without-name-unobservable area=10-slo-invariants-telemetry-budgets rec=fct-73zqjpsk@1
name: Hook panic without VIOLA_NAME unobservable
rule: A hook panic without VIOLA_NAME set is unobservable by design.
@row_none n=105 rec=fct-cgglh3ad@1 type=S
@row_add n=293 row=decision/heartbeat-liveness area=10-slo-invariants-telemetry-budgets rec=fct-gqc4mh6b@1,fct-onnnq5uj@1
name: Heartbeat file as the liveness signal
rule: run touches the heartbeat every 1 s and no process logs a transition; liveness flips to stale at more than 5 s.
why: replaces periodic tick lines
guard: chaos class stale heartbeat vs live pid, through agent-run status
@row_add n=294 row=decision/module-boundary-error-logging area=10-slo-invariants-telemetry-budgets rec=fct-db3jadur@1
name: Errors logged once at module boundaries
rule: Every typed error that crosses a crate seam is logged once at WARN or ERROR with a closed detail or error_code; the full chain goes only to the instance detail file.
@row_add n=295 row=decision/correlation-propagation area=10-slo-invariants-telemetry-budgets rec=fct-dhcjb4zv@1
name: Correlation propagation
rule: Every channel call logs corr and conn on both sides, every send line carries the send cursor, and every dialog line carries dialog_id.
@row_add n=296 row=decision/deterministic-emission area=10-slo-invariants-telemetry-budgets rec=fct-ta6vgo2y@1
name: Deterministic event emission
rule: Events key on hook events and events.ndjson offsets; nothing is derived from screen content.
@row_add n=297 row=decision/hook-perf-budgets-timed area=10-slo-invariants-telemetry-budgets rec=fct-fruln4ot@2
name: Hook performance budgets timed with hyperfine
rule: The session-start, user-prompt-submit, stop, session-end and pre-tool-use hooks are timed with hyperfine on an unstamped session in the perf job, and the perf gate requires all five; the async tier is untimed.
guard: gate --require perf
@row_add n=298 row=decision/perf-gate-max-sample area=10-slo-invariants-telemetry-budgets rec=fct-onnnq5uj@1,fct-xa6ldddm@1
name: Performance budgets gate the max sample
rule: Every performance budget gates hyperfine's max sample; p99 is not computed.
why: max is an upper bound on every quantile
guard: gate --require perf
@row_add n=299 row=decision/session-end-hook-budget area=10-slo-invariants-telemetry-budgets rec=fct-onnnq5uj@1
name: session-end hook budget
rule: viola hook session-end completes in under 1.0 s.
guard: gate --require perf
@row_add n=300 row=decision/spine-hook-budget area=10-slo-invariants-telemetry-budgets rec=fct-onnnq5uj@1
name: Spine hook budget
rule: The spine hooks and pre-tool-use on an unverified CLI complete under the tests-owned spine deadline, provisionally 1.0 s.
guard: gate --require perf
state: provisional
@row_add n=301 row=decision/no-request-ratio-slo area=10-slo-invariants-telemetry-budgets rec=fct-d4bn7qgd@1
name: No request-ratio SLO
rule: There is no request-ratio SLO; the error budget is a set of zero-tolerance counts.
why: viola is a local tool with no deploys
@row_add n=302 row=decision/error-budget-zero-tolerance area=10-slo-invariants-telemetry-budgets rec=fct-zze2ezbr@1
name: Zero-tolerance error budget
rule: The error budget tolerates zero panic lines in any CI artifact, zero secret-scan hits and zero schema-conformance failures.
except: the fake-agent seam's deliberate panic, as the zero-panics gate counts it
@row_add n=303 row=decision/error-budget-measured-by-gate-exits area=10-slo-invariants-telemetry-budgets rec=fct-2v5pxi7x@1
name: Error budget measured by in-job gate exit codes
rule: The error budget is measured by in-job gate exit codes, not by the uploaded artifact: panics by G2, schema failures by G4, secret hits by the secret-scan step.
why: the diagnostics artifact is not uploaded when the scan fails, so it is the diagnosis input, not the measurement surface
@row_add n=304 row=decision/artifact-panic-count-excludes-detail area=10-slo-invariants-telemetry-budgets rec=fct-2v5pxi7x@1
name: Artifact panic count excludes detail files
rule: A panic count over a downloaded diagnostics artifact excludes detail files.
why: detail files repeat the panic line
@row_add n=305 row=decision/nonzero-count-blocks-chunk area=10-slo-invariants-telemetry-budgets rec=fct-e2fzkaej@1
name: Nonzero error count blocks the chunk
rule: Any nonzero error-budget count makes the chunk red until the root cause is fixed within that chunk.
why: the tests quarantine rule
@row_add n=306 row=absence/on-call-paging area=10-slo-invariants-telemetry-budgets rec=fct-e2fzkaej@1
capability: On-call or paging
@row_add n=307 row=decision/build-fails-on-any-panic area=10-slo-invariants-telemetry-budgets rec=fct-yb5pe275@1,fct-saexeipm@1
name: Build fails on any panic
rule: The build fails on any panic event line, or on any panic evidenced by an exit code without its line.
@row_add n=308 row=decision/hyperfine-max-regression-fails area=10-slo-invariants-telemetry-budgets rec=fct-y4gvhk7p@1
name: Hyperfine max regression is a failure
rule: A hyperfine max regression beyond the telemetry latency budgets counts as an SLO invariant failure.
@row_add n=309 row=decision/secret-scan-hit-fails area=10-slo-invariants-telemetry-budgets rec=fct-ydzhack5@2
name: Secret-scan hit is a failure
rule: A secret-scan hit counts as an SLO invariant failure.
@row_add n=310 row=decision/diagnostics-files-mode-0600 area=10-slo-invariants-telemetry-budgets rec=fct-ydzhack5@2
name: Diagnostics files are 0600
rule: A diagnostics file whose mode is not 0600 counts as an SLO invariant failure.
@row_add n=311 row=decision/schema-conformance-failure-fails area=10-slo-invariants-telemetry-budgets rec=fct-j3ksxxc2@1
name: Schema-conformance failure is a failure
rule: An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.
@row_add n=312 row=decision/print-and-macro-lint-failure-fails area=10-slo-invariants-telemetry-budgets rec=fct-5kn2jpbt@1
name: Print and macro lint failure is a failure
rule: A clippy disallowed-macros, print_stdout, print_stderr or dbg_macro failure counts as an SLO invariant failure.
@row_add n=313 row=decision/lint-probes-failure-fails area=10-slo-invariants-telemetry-budgets rec=fct-5kn2jpbt@1
name: Lint-probes failure is a failure
rule: A lint-probes failure (a ban that did not fire, a control that did not pass, or a raw event! outside the core obs module) counts as an SLO invariant failure.
@row_add n=314 row=decision/g3-panic-abort-gate area=10-slo-invariants-telemetry-budgets rec=fct-5kn2jpbt@1
name: G3 panic-abort gate
rule: A G3 hit on panic = "abort" counts as an SLO invariant failure.
guard: G3
@row_add n=315 row=decision/grep-gates-fail-closed area=10-slo-invariants-telemetry-budgets rec=fct-5kn2jpbt@1
name: Grep gates fail closed
rule: The G1 and G3 gates fail on any rg exit other than 1, including exit 2 on a regex or I/O error and 127 on a missing rg.
@row_add n=316 row=decision/cargo-deny-failure-fails area=10-slo-invariants-telemetry-budgets rec=fct-5kn2jpbt@1
name: cargo deny failure is a failure
rule: A cargo deny failure (Tokio, a C crate, veil toggle, banned obs crates) counts as an SLO invariant failure.
guard: cargo deny
@row_add n=317 row=decision/surviving-obs-mutant-fails area=10-slo-invariants-telemetry-budgets rec=fct-o5c3uxju@1
name: Surviving obs mutant is a failure
rule: A surviving cargo-mutants mutant in obs code at a run --mutants (the epoch-boundary audit or on demand) counts as an SLO invariant failure.
except: A missed mutant the host never compiles is not counted as surviving.
@row_none n=106 rec=fct-pa7xy3qa@1 type=S
@row_none n=107 rec=fct-ah2viroy@1 type=S
@row_none n=108 rec=fct-eid2uzhv@1 type=S
@row_add n=318 row=decision/no-hot-path-instrumentation area=11-obs-anti-patterns-never-do-these rec=fct-75yvf2rf@1
name: No hot-path instrumentation
rule: The PTY pump read loop and the vt100 feed get no per-byte or per-chunk span or event; only gate outcomes are logged.
@row_add n=319 row=decision/must-trace-paths-always-traced area=11-obs-anti-patterns-never-do-these rec=fct-3si6piva@1
name: Must-trace paths always traced
rule: Spans and events are never skipped on the 6 must-trace paths.
why: The agent loses its ability to debug without them.
@row_add n=320 row=decision/closed-event-and-corr-on-every-line area=11-obs-anti-patterns-never-do-these rec=fct-yv7xjoql@1
name: Closed event enum and corr on every line
rule: Every log line carries the closed event enum and corr, so the signal pyramid never collapses into free text.
@row_add n=321 row=decision/no-otel-without-decisions-log area=11-obs-anti-patterns-never-do-these rec=fct-n47oemak@1
name: No OTel without a Decisions Log entry
rule: No OTel SDK, OTLP exporter or opentelemetry-stdout is added without a Decisions Log entry.
why: opentelemetry-otlp's default reqwest-blocking-client spins up a Tokio runtime.
@row_add n=322 row=decision/no-otel-stdout-exporter area=11-obs-anti-patterns-never-do-these rec=fct-n47oemak@1
name: OTel stdout exporter inadmissible
rule: The OTel stdout exporter is inadmissible on every role.
why: stdout is reserved.
@row_add n=323 row=decision/no-telemetry-from-screen-content area=11-obs-anti-patterns-never-do-these rec=fct-gr6xdpkj@1
name: No telemetry from screen content
rule: Telemetry is never derived from screen content; vt100 is used for readiness and modal gate outcomes only.
@row_add n=324 row=decision/transcripts-not-a-contract area=11-obs-anti-patterns-never-do-these rec=fct-gr6xdpkj@1
name: Claude Code transcripts are not a contract
rule: Claude Code transcripts are never read as a contract.
@row_add n=325 row=decision/no-pii-in-span-fields area=11-obs-anti-patterns-never-do-these rec=fct-om4ml4hz@1
name: No PII or content in span fields
rule: Span fields never include PII or content.
why: Scrubbing at an exporter stage is too late, and v1 has no exporter stage at all.
@row_add n=326 row=absence/telemetry-exporter-stage area=11-obs-anti-patterns-never-do-these rec=fct-om4ml4hz@1
capability: A telemetry exporter stage
@row_add n=327 row=decision/no-bare-instrument area=11-obs-anti-patterns-never-do-these rec=fct-3n7snwrw@1
name: No bare #[instrument]
rule: Instrumentation always uses #[instrument(skip_all, fields(...))], never a bare #[instrument].
why: A bare #[instrument] records every argument with Debug and leaks send text, hook.dialog payloads and tool input.
@row_add n=328 row=decision/no-instrument-err-ret-on-sensitive area=11-obs-anti-patterns-never-do-these rec=fct-odnuwe7o@1
name: No instrument err or ret on sensitive results
rule: #[instrument(err)] and #[instrument(ret)] are never used where the error or return can carry a serde or serde_path_to_error source or user content.
@row_add n=329 row=decision/low-cardinality-span-names area=11-obs-anti-patterns-never-do-these rec=fct-zgyunnhf@1
name: Low-cardinality span names
rule: Span names never contain instance names, dialog ids or paths.
@row_add n=330 row=decision/explicit-corr-conn-instance area=11-obs-anti-patterns-never-do-these rec=fct-stl2zgws@1
name: corr, conn and instance passed explicitly
rule: corr, conn and instance are passed as explicit values across threads or tasks, never carried by span context.
why: Spans are not serialized.
@row_add n=331 row=decision/no-in-process-metrics area=11-obs-anti-patterns-never-do-these rec=fct-npbjgxck@1
name: No in-process metrics instruments
rule: No in-process metrics instruments (opentelemetry meters, metrics crate) are created.
why: There is no reader, exporter or listener to flush them.
@row_add n=332 row=decision/no-telemetry-endpoint area=11-obs-anti-patterns-never-do-these rec=fct-khkx5tob@1
name: No telemetry endpoint without a security decision
rule: No /metrics or other telemetry endpoint is added without a Security Decisions Log entry.
why: A telemetry endpoint is a new listener.
@row_add n=333 row=decision/derived-metrics-by-closed-enums area=11-obs-anti-patterns-never-do-these rec=fct-gy4gdls6@1
name: Derived metrics grouped by closed enums
rule: Derived metrics are grouped by the closed enums only, never by unbounded keys such as path, corr, conn, pid or instance.
@row_add n=334 row=decision/latency-with-gate-statistic area=11-obs-anti-patterns-never-do-these rec=fct-733e2isu@1
name: Latency reported with the gate statistic
rule: Latency is never reported without the gate statistic; p50 and p95 are report-only.
@row_add n=335 row=decision/hyperfine-max-is-enforced area=11-obs-anti-patterns-never-do-these rec=fct-733e2isu@1
name: Hyperfine max is the enforced number
rule: The max from hyperfine is the enforced latency number.
@row_add n=336 row=decision/no-telemetry-on-stdout area=11-obs-anti-patterns-never-do-these rec=fct-va5nveqe@1
name: No telemetry on stdout
rule: Telemetry is never written to stdout on any role.
why: stdout carries each role's own output and run's terminal carries the child's screen.
@row_add n=337 row=contract/stdout-by-role/cli area=11-obs-anti-patterns-never-do-these rec=fct-va5nveqe@1
boundary: stdout by role
item: cli
promise: stdout carries the --json results.
@row_add n=338 row=contract/stdout-by-role/hook area=11-obs-anti-patterns-never-do-these rec=fct-va5nveqe@1
boundary: stdout by role
item: hook
promise: stdout carries the decision body or, for hook statusline, the user's status-line output.
@row_add n=339 row=contract/stdout-by-role/mcp area=11-obs-anti-patterns-never-do-these rec=fct-va5nveqe@1
boundary: stdout by role
item: mcp
promise: stdout carries MCP frames.
@row_add n=340 row=contract/stdout-by-role/run area=11-obs-anti-patterns-never-do-these rec=fct-va5nveqe@1
boundary: stdout by role
item: run
promise: The terminal of run carries the child's screen.
@row_add n=341 row=decision/explicit-subscriber-writer area=11-obs-anti-patterns-never-do-these rec=fct-va5nveqe@1
name: Explicit tracing-subscriber writer
rule: with_writer is always set explicitly on the tracing-subscriber.
why: The default tracing-subscriber writer is stdout.
@row_add n=342 row=decision/log-internal-errors-off area=11-obs-anti-patterns-never-do-these rec=fct-rd5bgb55@1
name: log_internal_errors turned off
rule: log_internal_errors is never left at its default true.
why: Writer failures would eprintln to stderr and break the Hook contract and the run terminal.
@row_add n=343 row=decision/obs-event-macro-only area=11-obs-anti-patterns-never-do-these rec=fct-aemh5hkn@1
name: obs_event! is the only event macro
rule: tracing's event and level macros are never called directly outside viola_core::obs; events go through obs_event!.
why: Only obs_event! guarantees event, process and instance.
guard: clippy disallowed-macros; the raw-event! grep in lint-probes
@row_add n=344 row=decision/corr-set-explicitly-by-caller area=11-obs-anti-patterns-never-do-these rec=fct-aemh5hkn@1
name: corr set explicitly by the caller
rule: The caller sets corr explicitly as a typed field on the events that carry one.
@row_add n=345 row=decision/no-stringified-corr area=11-obs-anti-patterns-never-do-these rec=fct-dhc5cdwv@1
name: corr never recorded with ? or %
rule: corr is never recorded with ?corr or %corr.
why: Those stringify numbers.
@row_add n=346 row=decision/no-env-log-level area=11-obs-anti-patterns-never-do-these rec=fct-mk3zdfg7@1
name: Log level not read from the environment
rule: The level is never read from RUST_LOG, the env-filter feature is never enabled and EnvFilter::from_default_env is never called.
why: Env vars are not a configuration channel.
@row_add n=347 row=decision/one-line-panics area=11-obs-anti-patterns-never-do-these rec=fct-fgn7d524@1
name: Panics are one line
rule: Multi-line stack traces are never written; panics are logged as one line.
@row_add n=348 row=decision/backtraces-in-instance-detail-only area=11-obs-anti-patterns-never-do-these rec=fct-fgn7d524@1
name: Backtraces in the instance detail file only
rule: Backtraces are written as one-line JSON arrays in the instance detail file only.
@row_add n=349 row=decision/no-info-in-hot-loops area=11-obs-anti-patterns-never-do-these rec=fct-thq5vza6@1
name: No info logging in hot loops
rule: Nothing is logged at info inside hot loops such as the PTY pump; counts go on the boundary event instead.
@row_add n=350 row=decision/no-non-blocking-appender area=11-obs-anti-patterns-never-do-these rec=fct-v6m4e2mq@1
name: No non_blocking appender
rule: tracing-appender non_blocking is never used.
why: It drops lines when full, needs a WorkerGuard flush, and breaks the hook's exit-0 / max < 1.0 s gate and the zero-unlogged-panics invariant.
@row_add n=351 row=decision/log-field-names-frozen area=11-obs-anti-patterns-never-do-these rec=fct-v7dbdmio@1
name: Log field names frozen
rule: Log fields such as corr and event are never renamed without a Decisions Log entry.
@row_add n=352 row=decision/event-values-closed area=11-obs-anti-patterns-never-do-these rec=fct-v7dbdmio@1
name: event values closed
rule: No event value is added without a Decisions Log entry.
@row_add n=353 row=decision/no-log-bridge area=11-obs-anti-patterns-never-do-these rec=fct-vdlxpzho@1
name: No log bridge
rule: tracing_log::LogTracer or any log bridge is never installed by hand, and tracing-log stays off.
why: Bridged records lack event, corr, process and instance.
@row_add n=354 row=decision/no-error-reporter-in-v1 area=11-obs-anti-patterns-never-do-these rec=fct-fvlnws45@1
name: No error reporter in v1
rule: sentry or any other reporter is never added in v1.
why: Egress is banned, and sentry's default native-tls / tokio transport breaks cargo-deny.
@row_add n=355 row=decision/no-serde-source-in-error-output area=11-obs-anti-patterns-never-do-these rec=fct-qeuz4fkn@1
name: No serde source in printed errors
rule: An anyhow chain that still contains a serde_json or serde_path_to_error source is never printed to stderr, --json, MCP isError or channel error.data.
@row_add n=356 row=decision/default-panic-hook-never-runs area=11-obs-anti-patterns-never-do-these rec=fct-murrb5aj@1
name: Default panic hook never runs
rule: The default panic hook is never called or allowed to run.
why: Its stderr output breaks hook and the run terminal.
@row_add n=357 row=decision/reporter-opt-in-only area=11-obs-anti-patterns-never-do-these rec=fct-fb7hrxpx@1
name: Any future reporter is opt-in
rule: Any future reporter is opt-in, never enabled by env var or by default.
@row_add n=358 row=decision/reporter-ureq-rustls area=11-obs-anti-patterns-never-do-these rec=fct-fb7hrxpx@1
name: Future reporter transport is ureq and rustls
rule: Any future reporter uses ureq with rustls.
@row_add n=359 row=decision/reporter-never-log-floor area=11-obs-anti-patterns-never-do-these rec=fct-fb7hrxpx@1
name: Future reporter applies the NEVER-log floor
rule: Any future reporter applies the NEVER-log floor in before_send.
@row_add n=360 row=decision/file-sink-only-error-sink area=11-obs-anti-patterns-never-do-these rec=fct-5ak67rzk@1
name: File sink is the only error sink
rule: Local logs are never optional: the file sink is the only error sink and works with every other feature off.
@row_add n=361 row=decision/field-allow-list-default-deny area=11-obs-anti-patterns-never-do-these rec=fct-vq5djl4b@1
name: Field allow-list is default-deny
rule: No field is logged unless named in the field catalog.
@row_add n=362 row=decision/veil-toggle-banned area=11-obs-anti-patterns-never-do-these rec=fct-u3w4pedj@1
name: veil toggle feature banned
rule: veil's toggle feature (VEIL_DISABLE_REDACTION / veil::disable()) is never enabled.
guard: cargo-deny
@row_add n=363 row=decision/allow-list-not-widenable area=11-obs-anti-patterns-never-do-these rec=fct-fruyyip2@1
name: Allow-list and redaction not configurable
rule: config.json, a VIOLA_* env var or a CLI flag never widens the field allow-list or disables redaction.
@row_add n=364 row=decision/verbosity-changes-volume-only area=11-obs-anti-patterns-never-do-these rec=fct-fruyyip2@1
name: Verbosity changes volume only
rule: diagnostics_level changes the volume of logging only.
@row_add n=365 row=decision/claude-env-and-cookie-never-logged area=11-obs-anti-patterns-never-do-these rec=fct-vnsyc5om@1
name: CLAUDE* values and Cookie never logged
rule: CLAUDE* values and a Cookie header are never logged, at any level, in any file, detail files included.
@row_add n=366 row=decision/sensitive-content-detail-file-only area=11-obs-anti-patterns-never-do-these rec=fct-vnsyc5om@1
name: Sensitive content only in the instance detail file
rule: statusline_command, send text, tool input and last_assistant_message never reach a home-level file; they may reach only the instance-scoped detail-<process>.ndjson, inside a chain or drift report.
@row_add n=367 row=decision/no-paths-or-pids-in-detail-codes area=11-obs-anti-patterns-never-do-these rec=fct-hl4ehpjw@1
name: No paths or pids in detail codes
rule: detail codes never contain paths or pids.
@row_add n=368 row=decision/paths-only-in-instance-detail-files area=11-obs-anti-patterns-never-do-these rec=fct-hl4ehpjw@1
name: Paths only in instance detail files
rule: Paths appear only in instance detail files.
@row_add n=369 row=decision/upload-only-after-secret-scan area=11-obs-anti-patterns-never-do-these rec=fct-qteviymq@1,fct-ntdd4mdw@1
name: Diagnostics uploaded only after the secret scan
rule: diagnostics artifacts are never uploaded before the secret-scan step passes; uploads run if: always() / failure() combined with the secret-scan step's success, never bare.
@row_add n=370 row=decision/failing-job-diagnostics-kept area=11-obs-anti-patterns-never-do-these rec=fct-ntdd4mdw@1
name: Failing job's diagnostics kept
rule: A failing job's diagnostics are never lost: G2, G4 and the secret scan run if: always().
@row_add n=371 row=decision/failed-scan-uploads-hit-report area=11-obs-anti-patterns-never-do-these rec=fct-ntdd4mdw@1
name: Failed scan uploads the hit report
rule: On a failed secret scan the hit report is uploaded instead of the diagnostics.
@row_add n=372 row=decision/rerun-no-substitute area=11-obs-anti-patterns-never-do-these rec=fct-ntdd4mdw@1
name: Re-run is no substitute for diagnostics
rule: A re-run is not a substitute for the failing job's diagnostics.
why: CI runs under retries = 0.
@row_add n=373 row=decision/machine-asserted-telemetry-verdicts area=11-obs-anti-patterns-never-do-these rec=fct-on5vltm5@1,fct-feqirlyj@1
name: Machine-asserted telemetry verdicts
rule: Log analysis is never human-review-gated; every CI telemetry verdict is a jq or hyperfine assertion that sets an exit code.
@row_add n=374 row=decision/no-ci-env-metadata-in-logs area=11-obs-anti-patterns-never-do-these rec=fct-zbnejvt2@1
name: No CI env metadata in product logs
rule: CI env metadata is never injected into product log lines.
why: The product never reads env as configuration.
@row_add n=375 row=decision/no-retry-once area=11-obs-anti-patterns-never-do-these rec=fct-pardphhn@1
name: No retry-once policies
rule: No retry-once policy is added for panics or flaky telemetry assertions.
@row_add n=376 row=decision/budgets-asserted-by-exit-code area=11-obs-anti-patterns-never-do-these rec=fct-soiwyjd4@1
name: Budgets asserted by exit code
rule: Every budget has an exit-code-setting assertion (hyperfine max via jq).
@row_add n=377 row=decision/no-per-heartbeat-logging area=11-obs-anti-patterns-never-do-these rec=fct-ie2ur45l@1
name: No log line per heartbeat tick
rule: Heartbeats log transitions and lifecycle only, never a line per tick.
why: The log files are never rotated.
@row_add n=378 row=decision/no-proprietary-apm-only area=11-obs-anti-patterns-never-do-these rec=fct-df7l3zmj@1
name: No proprietary APM as the only exporter
rule: Proprietary APM (Datadog APM SDK, NewRelic agent, AppDynamics) is never the only exporter.
why: Vendor schemas break paste-to-AI workflows, and viola has no egress at all.
@row_add n=379 row=absence/network-egress area=11-obs-anti-patterns-never-do-these rec=fct-df7l3zmj@1
capability: Network egress of any kind
@row_add n=380 row=decision/no-dashboards-only-obs area=11-obs-anti-patterns-never-do-these rec=fct-h4xijqlt@1
name: No dashboards-only observability
rule: Every signal is consumed through agent-run logs / status and jq, never through dashboards as the sole surface.
@row_add n=381 row=decision/product-is-not-a-telemetry-collector area=11-obs-anti-patterns-never-do-these rec=fct-gd3tf5wl@1
name: Product is not a telemetry collector
rule: viola never self-instruments as if the product were telemetry: events.ndjson is not turned into a collector and no OTLP receiver is added.
why: viola is Normal mode (Founder Direction 3)
@row_add n=382 row=decision/service-identity-from-constants area=11-obs-anti-patterns-never-do-these rec=fct-jzb7kb3q@1
name: Service identity from compiled constants
rule: Service name and version come from viola-core constants, never from env vars such as OTEL_SERVICE_NAME or OTEL_RESOURCE_ATTRIBUTES; any future OTel Resource starts from an empty builder.
@row_add n=383 row=decision/scrub-before-bytes-leave-process area=11-obs-anti-patterns-never-do-these rec=fct-cegd5nw3@1
name: Scrubbing before any byte leaves the process
rule: Every byte is scrubbed of secrets before it leaves the process; in v1 the only destination is local files.
guard: the secret-scan test
@row_none n=109 rec=fct-q2i64n5w@1 type=S
@row_none n=110 rec=fct-gmkzqfqx@1 type=S
@row_add n=384 row=decision/ci-artifacts-gated-on-secret-scan area=ci-integration rec=fct-auyn3syc@1,fct-u6lpuily@1,fct-t3izd7ip@1,fct-gsrzhsh7@2
name: CI artifacts uploaded only after a passing secret scan
rule: Diagnostics, harness logs, JUnit XML and perf samples are uploaded on failing runs too, but only when the job's secret scan succeeded; a failed scan withholds every artifact.
why: a bare always() or failure() would also upload after a failed scan
@row_add n=385 row=decision/harness-logs-captured-before-scan area=ci-integration rec=fct-u6lpuily@1
name: Harness logs captured before the secret scan
rule: On test failure the agent-run logs and status are captured before the secret scan, so the scan covers them, and they go in an artifact of their own, not the diagnostics artifact.
why: upload artifacts are immutable, so a second upload under the same name conflicts, and overwriting would replace the raw diagnostics
@row_add n=386 row=decision/no-raw-tracing-macros area=ci-integration rec=fct-xpak76xj@2
name: No raw tracing macros outside the obs module
rule: Raw tracing info, warn, error, debug and trace macros and raw event! are refused outside viola_core::obs.
guard: clippy -D warnings disallowed-macros; raw-event! grep and ban probes in scripts/lint-probes.sh
@row_add n=387 row=decision/sync-crates-without-tokio area=ci-integration rec=fct-xpak76xj@2
name: Sync crates build without Tokio
rule: The synchronous crates must compile without Tokio.
guard: cargo check of the sync crates without Tokio
@row_add n=388 row=decision/banned-telemetry-dependencies area=ci-integration rec=fct-xpak76xj@2
name: Banned telemetry and logging dependencies
rule: opentelemetry-otlp, opentelemetry-stdout, sentry, tracing-appender, the tracing-subscriber env-filter feature and the veil toggle feature are not taken as dependencies.
guard: cargo deny check
@row_add n=389 row=decision/instrument-requires-skip-all area=ci-integration rec=fct-xpak76xj@2,fct-do3y4vos@1
name: Instrument attribute must skip all arguments
rule: Every #[instrument] in the workspace takes skip_all as its first argument; a bare #[instrument] fails the build.
guard: gate G1
@row_add n=390 row=decision/ci-gates-fail-closed area=ci-integration rec=fct-do3y4vos@1,fct-v4xtuswt@1
name: CI gates fail closed
rule: A gate whose tool is missing, errors or meets an empty scope fails the step rather than passing.
why: an erroring search with a must-be-empty check would pass on every commit
guard: G1, G3, scripts/g2-zero-panics.sh --probe
@row_add n=391 row=decision/diag-lines-schema-checked-in-unit-tests area=ci-integration rec=fct-a7stszdk@1
name: Emitted lines schema-checked in unit tests
rule: Unit tests write to an in-memory writer and parse every emitted line against schemas/diag-line.v1.json.
@row_add n=392 row=decision/test-homes-under-scan-root area=ci-integration rec=fct-uqr3kd37@1,fct-eilqg5wz@2,fct-pto2t3ps@1
name: Test homes live under the scanned root
rule: Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it.
why: a hook panic exits 0, so a home outside that root would hide it
except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
@row_add n=393 row=decision/perf-budget-asserted-on-max area=ci-integration rec=fct-uqr3kd37@1,fct-gsrzhsh7@2
name: Perf budgets asserted on the maximum sample
rule: Performance budgets are asserted on the max of hyperfine samples, and a failing perf gate still leaves its samples.
@row_add n=394 row=decision/zero-panics-gate area=ci-integration rec=fct-k3fxcuz7@1,fct-v4xtuswt@1
name: Zero panics gate
rule: No panic line may appear in the home-level role files of test runs; detail files repeat the panic line and are excluded.
why: ratified
guard: gate G2 (scripts/g2-zero-panics.sh, --probe first)
except: a panic whose panic_location is exactly src/cmd/hook/seam.rs:<digits>, the fake-agent seam's forced panic, compared as a whole string
@row_add n=395 row=decision/os-gated-test-census area=ci-integration rec=fct-t2ak7i7a@2
name: OS-gated test census
rule: Every OS-gated test function or file must be named in the census with a non-empty reason, and a gate predicate naming windows or macos is a fault even when named.
guard: contract_os_gated_tests
@row_add n=396 row=decision/obs-code-mutation-tested area=ci-integration rec=fct-r4tkrl6z@2
name: Obs code mutation-tested like product code
rule: Obs code is mutation-tested like product code at the epoch boundary and on demand, not on push or pull request; a timed-out or missed obs-code mutant, or unviable outnumbering caught, is red.
guard: test-plan Mutation gate
@row_add n=397 row=decision/schema-conformance-gate area=ci-integration rec=fct-k3fxcuz7@1,fct-ocs2orze@1
name: Schema conformance gate on real process output
rule: Every role-file line of test homes must validate against schemas/diag-line.v1.json and every detail line against schemas/diag-detail.v1.json; a failing line fails the build, a torn non-JSON line is skipped and counted.
why: schema failures count as a build failure and unit tests check only in-memory lines
guard: gate G4 (schema-conformance step)
@row_add n=398 row=decision/gate-output-omits-line-content area=ci-integration rec=fct-ocs2orze@1
name: Gate output never prints a line's content
rule: A failing schema check prints file, line number and failing keyword, never the line's content.
why: detail lines carry the canary
@row_add n=399 row=decision/panic-unwind-required area=ci-integration rec=fct-ocs2orze@1,fct-vfnyc5f5@2
name: Panic strategy is unwind
rule: Every Cargo profile keeps panic = "unwind"; no abort strategy in profiles, rustflags, env or workflows.
why: abort turns every catch_unwind into an abort, so hook would die by SIGABRT instead of exiting 0
guard: gate G3
@row_add n=400 row=decision/gates-run-after-failed-tests area=ci-integration rec=fct-eilqg5wz@2
name: Gates run even after failed tests
rule: G2 and G4 run if: always(), so a failed test step never skips them.
why: failing runs are where panic lines are most likely, and panics are measured only through G2's exit code
@row_add n=401 row=decision/test-homes-kept-until-gates area=ci-integration rec=fct-eilqg5wz@2,fct-ulqlzuau@1
name: Test homes kept until gate steps run
rule: CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it.
why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
@row_add n=402 row=decision/raw-unsymbolised-backtrace area=error-capture-reporting rec=fct-c5mu6ru4@2
name: Panic backtraces captured raw, never symbolised
rule: A panic's detail line carries raw frames (ip, module path, base, offset), up to 62, for offline resolution against the pinned copy; nothing is symbolised in the panic path.
why: symbolising cost most of a hook run and pushed the forced-panic hook past the 1.0 s spine bound
@row_add n=403 row=decision/env-vars-not-config-channel area=error-capture-reporting rec=fct-c5mu6ru4@2
name: Env vars are not a configuration channel
rule: Backtrace capture is not gated by RUST_BACKTRACE or any env var.
why: env vars are not a configuration channel
@row_none n=111 rec=fct-bprhan45@2 type=4
relied: A refused diagnostics-path check is exactly a failed open: the role writer ends at a sink and the detail line is dropped.
@row_add n=404 row=decision/exit-decided-at-catch-site area=error-capture-reporting rec=fct-vfnyc5f5@2
name: Exit decided at a catch site, not the panic hook
rule: The panic hook only writes; everything after the hook is set runs inside one catch_unwind in main, so no role ends through an uncaught unwind.
@row_add n=405 row=decision/role-classified-before-clap area=error-capture-reporting rec=fct-vfnyc5f5@2
name: Role taken from argv before clap
rule: The role is the first argument that is neither a global flag nor a flag's value and names a role subcommand, so viola --home <dir> hook stop is still a hook whose pre-init panic exits 0.
why: reading argv[1] alone would misclassify that form as cli and exit 1
@row_add n=406 row=contract/cli-internal-error/human-line area=error-capture-reporting rec=fct-vfnyc5f5@2
boundary: cli internal error
item: error: internal error
promise: Without --json the cli catch site prints exactly error: internal error; run, revive's start, mcp and hook print neither.
@row_add n=407 row=decision/run-worker-panic-ends-run area=error-capture-reporting rec=fct-vfnyc5f5@2
name: Run worker panic ends the session
rule: A panic in the PTY pump or handle-wait thread is sent to main, which takes run's exit path; a worker panic leaving main running with no process-exit is forbidden.
why: the session cannot continue without either thread
@row_add n=408 row=decision/channel-connection-panic-contained area=error-capture-reporting rec=fct-vfnyc5f5@2
name: Channel connection panic contained
rule: A panic in one channel connection worker logs channel-response error -32603, closes that connection, and run continues.
@row_add n=409 row=decision/vt100-feed-panic-contained area=error-capture-reporting rec=fct-vfnyc5f5@2,fct-pto2t3ps@1
name: vt100 feed panic contained
rule: A panic in a vt100 feed or resize is caught on the feed thread, poisons the screen model until the host size changes, writes one parse-rejected per episode, and run continues.
@row_none n=112 rec=fct-dujo6hv4@1 type=4
relied: Refusals of the instance and home-tree checks that run before a role file opens write no line in any log.
@row_none n=113 rec=fct-l5yuzjd3@2 type=5
@row_add n=410 row=contract/mcp-frame-tools/strict-modes-failed area=error-capture-reporting rec=fct-tikck2t5@1
boundary: viola mcp frame tools
item: strict-modes-failed
promise: A refused instance check returns isError instance-unreachable with detail strict-modes-failed, and writes no frame and no line.
@row_add n=411 row=contract/mcp-frame-tools/socket-dir-refused area=error-capture-reporting rec=fct-mgjsgjd2@1
boundary: viola mcp frame tools
item: socket-dir-refused
promise: A refused socket directory returns isError instance-unreachable with detail socket-dir-refused, and writes no frame and no line.
@row_none n=114 rec=fct-25dvifnk@1 type=4
relied: Each MCP refusal maps to exactly one UnreachableCause word.
@row_add n=412 row=decision/single-sanitising-fault-reader area=error-capture-reporting rec=fct-j2kwa4rj@1
name: One sanitising reader of peer faults
rule: A peer's fault is read only through one sanitiser: five known codes read fixed messages with data admitted only in validated shapes, a code outside them reads wrapper fault, and a fault without an integer code reads nothing.
@row_add n=413 row=decision/peer-replies-screened-as-code-words area=error-capture-reporting rec=fct-6qbwm4yh@1
name: Peer replies screened as code words
rule: A peer reply's refusal and detail are kept only as code words (1-64 bytes of [a-z0-9-], letter first); an unknown code word stays visible and exits 14.
why: by design
@row_none n=115 rec=fct-j5jj3gux@3 type=5
@row_none n=116 rec=fct-wmupioom@1 type=4
relied: The NotADirectory refusal message is fixed and names no path.
@row_add n=414 row=contract/answer-send-exit-2/stderr-line area=error-capture-reporting rec=fct-cq3l3475@1
boundary: viola answer and viola send exit 2
item: stderr line
promise: Exit 2 writes the fixed unusable-response or unusable-text line to stderr whatever --json says, with no document.
@row_add n=415 row=contract/exit-21-document/shape area=error-capture-reporting rec=fct-3qk5e3jo@1
boundary: exit 21 document
item: shape
promise: Exit 21 prints one document per cause: {"v":1,"error":"instance-unreachable","detail":"<cause>"}.
@row_add n=416 row=contract/exit-21-document/detail area=error-capture-reporting rec=fct-3qk5e3jo@1
boundary: exit 21 document
item: detail
promise: One of six closed words: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused.
@row_add n=417 row=contract/diag-line-fields/process-start area=log-coverage rec=fct-yb5jpaot@2
boundary: diagnostic line additive fields
item: process-start
promise: subject (self, claude-child, version-probe, verify-probe, verify-pty-probe, agents-probe, statusline-shell), service_name, version, os, pid, child_pid, endpoint_kind, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept (names only).
@row_add n=418 row=contract/diag-line-fields/process-exit area=log-coverage rec=fct-umidjbc4@1
boundary: diagnostic line additive fields
item: process-exit
promise: subject, exit_code (self), child_exit_status, shell_exit_status, exit_source (handle-wait or kill-fallback), detail, during (connect or call), duration_ms.
@row_add n=419 row=contract/diag-line-fields/channel-request area=log-coverage rec=fct-tnco7mng@1
boundary: diagnostic line additive fields
item: channel-request
promise: method, conn / srv_conn, from, from_trust, sender, v, after, timeout_ms.
@row_add n=420 row=contract/diag-line-fields/channel-response area=log-coverage rec=fct-ikvvw35l@1
boundary: diagnostic line additive fields
item: channel-response
promise: method (only a closed-list name), conn / srv_conn, result_class (ok, refusal, error), refusal, detail (only the three -32602 faults), error_code (-32700, -32600, -32601, -32602, -32603), outcome, duration_ms.
@row_add n=421 row=contract/diag-line-fields/send-events area=log-coverage rec=fct-hb5clvjs@1
boundary: diagnostic line additive fields
item: send-issued / send-confirmed / send-refused
promise: from, from_trust, text_bytes (normalised typed length), confirmed, refusal, detail, side (client or wrapper), wheel, window, used_percentage, resets_at, paused, duration_ms, and wrapper side only conn / srv_conn and rpc_id.
@row_add n=422 row=contract/diag-line-fields/dialog-events area=log-coverage rec=fct-qb47tiuw@1
boundary: diagnostic line additive fields
item: dialog-raised / dialog-answered
promise: dialog_kind, hook_event, from, from_trust, deadline_hit, duration_ms.
@row_add n=423 row=contract/diag-line-fields/hook-events area=log-coverage rec=fct-vrjzzaok@1
boundary: diagnostic line additive fields
item: hook-invoked / hook-decision
promise: hook_event (one of ten closed hook names incl. statusline), stdin_bytes, invoked_at, decision_emitted, deadline_hit, budget_written, detail, duration_ms.
@row_add n=424 row=contract/diag-line-fields/panic area=log-coverage rec=fct-hq2iiwoa@1
boundary: diagnostic line additive fields
item: panic
promise: panic_location (workspace-relative, or <crate>-<ver>/src/... for dependencies) and thread; payload and backtrace go to the detail file only.
@row_add n=425 row=contract/diag-line-fields/release-from-driver area=log-coverage rec=fct-izpoipm6@1
boundary: diagnostic line additive fields
item: release-from-driver
promise: conn, from (only when a valid ViolaName), from_trust (self-reported).
@row_add n=426 row=contract/diag-line-fields/state-recovered area=log-coverage rec=fct-rn7dqtoc@1
boundary: diagnostic line additive fields
item: state-recovered
promise: detail (torn-line-healed, snapshot-replayed, snapshot-unsupported-v), file (basename only), offset (torn-line-healed only), v_seen (snapshot-unsupported-v only), scanned (snapshot details only: the instance name when valid, never a path or null).
@row_add n=427 row=contract/diag-line-fields/parse-rejected area=log-coverage rec=fct-yp3xr6e4@1
boundary: diagnostic line additive fields
item: parse-rejected
promise: parser (claude-agents-json, vt100-feed, config-json, ledger-stamps, hook-stdin, channel-frame, mcp-stdin, prompt-submitted, paste-text, state-entry), detail, count.
@row_add n=428 row=contract/process-exit-detail/exit-1 area=log-coverage rec=fct-dsupowm6@2
boundary: process-exit detail
item: exit 1
promise: already-live, stale-heartbeat, squatted-name, pinned-hash-mismatch, internal-error, socket-dir-refused, and from revive's preflight no-session, session-live, cwd-missing; each one process-exit line with no child started.
@row_add n=429 row=decision/revive-preflight-order area=log-coverage rec=fct-dsupowm6@2
name: Revive start preflight order
rule: Revive's start preflight reads the holder, then the session, then session-live, then the directory, and stops at the first refusal.
@row_add n=430 row=decision/session-live-check-fails-open area=log-coverage rec=fct-dsupowm6@2
name: Session-live check fails open
rule: A failed, oversize or malformed claude agents read passes the session-live check; the check is skipped under --fork.
@row_add n=431 row=contract/process-exit-detail/interrupted area=log-coverage rec=fct-dsupowm6@2
boundary: process-exit detail
item: interrupted
promise: A viola verify stopped by SIGINT or SIGTERM before its rows are checked ends with one process-exit exit 1 detail interrupted, after every spawn pair it opened is closed.
@row_add n=432 row=contract/process-exit-detail/verify-refusal area=log-coverage rec=fct-dsupowm6@2
boundary: process-exit detail
item: viola verify refusal
promise: Under a parsed VIOLA_NAME, verify writes its refusal code: program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture or interrupted; a failing row carries no code.
@row_add n=433 row=decision/unemittable-schema-words-removed area=log-coverage rec=fct-j4k5a564@1
name: Schema words no build can emit are removed
rule: A closed word of the diag-line schema that no build can emit leaves the schema with its emitter; os is closed at linux.
@row_add n=434 row=contract/process-exit-detail/exit-21 area=log-coverage rec=fct-qzbqiq2k@2
boundary: process-exit detail
item: exit 21
promise: instance-dead, unwrapped-name, endpoint-lost, strict-modes-failed, server-verify-failed, socket-dir-refused.
@row_add n=435 row=contract/process-exit-detail/server-verify-failed area=log-coverage rec=fct-qzbqiq2k@2
boundary: process-exit detail
item: server-verify-failed
promise: The eight CLI frame verbs write it at WARN with exit 21, during connect, on a server of another euid, not the recorded wrapper, or an unverifiable check, with no pid, start time, path or endpoint.
@row_add n=436 row=contract/process-exit-detail/exit-20 area=log-coverage rec=fct-ylj2z2t5@1
boundary: process-exit detail
item: exit 20
promise: wrapper-fault.
@row_add n=437 row=contract/hook-decision-detail/fail-open-codes area=log-coverage rec=fct-qiql34nd@2
boundary: hook-decision detail
item: fail-open codes
promise: oversize-stdin, malformed-json, channel-unreachable, server-verify-failed, socket-dir-refused, strict-modes-failed, unverified-cli, deadline; no hook arm writes strict-modes-failed.
@row_add n=438 row=contract/hook-decision-detail/statusline-codes area=log-coverage rec=fct-qiql34nd@2
boundary: hook-decision detail
item: statusline codes
promise: viola hook statusline writes only oversize-stdin, malformed-json and deadline; a refused instance check writes no line.
@row_add n=439 row=decision/statusline-command-deadline area=log-coverage rec=fct-qiql34nd@2
name: Statusline command deadline
rule: The user's statusline command is bounded at 5 s, reported as deadline.
state: provisional
@row_add n=440 row=contract/parse-rejected-detail/malformed area=log-coverage rec=fct-vehppv2d@1
boundary: parse-rejected detail
item: malformed
promise: Any parser.
@row_add n=441 row=contract/parse-rejected-detail/unreadable area=log-coverage rec=fct-vehppv2d@1
boundary: parse-rejected detail
item: unreadable
promise: config-json, ledger-stamps, state-entry.
@row_add n=442 row=contract/parse-rejected-detail/oversize area=log-coverage rec=fct-vehppv2d@1
boundary: parse-rejected detail
item: oversize
promise: channel-frame beyond MAX_FRAME, hook-stdin, mcp-stdin past MAX_FRAME, vt100-feed when the bounded queue drops a copy (one line per poisoning episode), claude-agents-json past MAX_FRAME.
@row_add n=443 row=contract/parse-rejected-detail/unknown-keys area=log-coverage rec=fct-vehppv2d@1
boundary: parse-rejected detail
item: unknown-keys
promise: config-json skipped keys, counted in count.
@row_add n=444 row=contract/parse-rejected-detail/panicked area=log-coverage rec=fct-vehppv2d@1
boundary: parse-rejected detail
item: panicked
promise: vt100-feed under catch_unwind.
@row_add n=445 row=contract/parse-rejected-detail/control-character area=log-coverage rec=fct-vehppv2d@1
boundary: parse-rejected detail
item: control-character
promise: paste-text.
@row_add n=446 row=contract/parse-rejected-detail/symlink-ignored area=log-coverage rec=fct-vehppv2d@1
boundary: parse-rejected detail
item: symlink-ignored
promise: state-entry.
@row_add n=447 row=contract/parse-rejected-detail/strict-modes-failed area=log-coverage rec=fct-vehppv2d@1
boundary: parse-rejected detail
item: strict-modes-failed
promise: ledger-stamps: run's version gate refused the stamps file on the strict-modes check, and the read counts as unverified.
@row_add n=448 row=contract/parse-rejected/mcp-stdin-oversize area=log-coverage rec=fct-iy4rx7dp@1
boundary: parse-rejected diagnostic line
item: parser mcp-stdin, detail oversize
promise: Written at WARN into mcp.ndjson once per session, after the session ends, only when the stdin reader cut a line past MAX_FRAME, with count 1; the line holds no byte of the cut line.
@row_add n=449 row=contract/send-refused-detail/refusal-reason-words area=log-coverage rec=fct-icf6u7jt@1
boundary: send-refused detail
item: RefusalReason detail words
promise: Refusal details are the arch RefusalReason details verbatim: manual-pause, five-hour, seven-day, input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text.
@row_add n=450 row=contract/send-refused-detail/empty-text area=log-coverage rec=fct-icf6u7jt@1
boundary: send-refused detail
item: empty-text
promise: Written for an empty typed text on send; the word adds no field, and send-refused detail stays an open string in the diag-line schema.
@row_add n=451 row=contract/diag-detail/strict-modes-failed area=log-coverage rec=fct-mivk5eld@1,fct-tinnk5cr@1
boundary: diagnostic detail words
item: strict-modes-failed
promise: Written on the process-exit exit 1 of viola verify under a parsed VIOLA_NAME and on parse-rejected ledger-stamps from the version gate of run, at warn, with no path; every other refused strict-modes check writes no log line, its exit code or JSON document its only record.
@row_add n=452 row=contract/diag-detail/pinned-hash-mismatch area=log-coverage rec=fct-mivk5eld@1
boundary: diagnostic detail words
item: pinned-hash-mismatch
promise: A filesystem refusal word written on process-exit with exit 1.
@row_add n=453 row=decision/symlink-skip-logged-as-count area=log-coverage rec=fct-mivk5eld@1
name: Skipped state entries logged as a parse-rejected count
rule: A scan of instance state that skips a symlinked or malformed entry writes one parse-rejected line with parser state-entry, detail symlink-ignored or malformed and a count, once per scan, at warn, into the scanning process's role file, naming no entry and no link target.
why: A skipped symlink has no exit or decision to carry it.
@row_add n=454 row=contract/diag-detail/socket-dir-refused area=log-coverage rec=fct-mivk5eld@1,fct-l3f7hrdu@2
boundary: diagnostic detail words
item: socket-dir-refused
promise: A closed detail on process-exit and hook-decision for a socket dir refused as a symlink, no directory, foreign owner or mode bit: run and revive's start write process-exit exit 1, CLI frame verbs process-exit exit 21 during connect at WARN, viola hook a hook-decision at WARN; MCP writes none.
@row_add n=455 row=contract/diag-subject/self area=log-coverage rec=fct-lhnmh6dy@1
boundary: diag-line subject enum
item: self
promise: One value of the closed subject enum of the diag-line schema.
@row_add n=456 row=contract/diag-subject/claude-child area=log-coverage rec=fct-lhnmh6dy@1
boundary: diag-line subject enum
item: claude-child
promise: One value of the closed subject enum of the diag-line schema.
@row_add n=457 row=contract/diag-subject/version-probe area=log-coverage rec=fct-lhnmh6dy@1
boundary: diag-line subject enum
item: version-probe
promise: Subject of the process-start and process-exit pair of a version probe spawn.
@row_add n=458 row=contract/diag-subject/verify-probe area=log-coverage rec=fct-lhnmh6dy@1
boundary: diag-line subject enum
item: verify-probe
promise: Subject of the process-start and process-exit pair of the print-mode probe of viola verify.
@row_add n=459 row=contract/diag-subject/verify-pty-probe area=log-coverage rec=fct-lhnmh6dy@1
boundary: diag-line subject enum
item: verify-pty-probe
promise: Subject of the process-start and process-exit pair of each of the five interactive PTY runs of viola verify, Run A to Run E.
@row_add n=460 row=contract/diag-subject/agents-probe area=log-coverage rec=fct-lhnmh6dy@1
boundary: diag-line subject enum
item: agents-probe
promise: Subject of the process-start and process-exit pair of the claude agents read, written by viola list, viola revive's session-live reading, verify's Run B and the list tool of viola mcp into their role files.
@row_add n=461 row=contract/diag-subject/statusline-shell area=log-coverage rec=fct-lhnmh6dy@1
boundary: diag-line subject enum
item: statusline-shell
promise: Subject of the process-start and process-exit pair of a statusline shell spawn.
@row_add n=462 row=contract/verify-process-exit/child-exit-status-and-duration area=log-coverage rec=fct-lhnmh6dy@1
boundary: viola verify spawn process-exit lines
item: child_exit_status and duration_ms
promise: Each of the eight verify spawn pairs' process-exit carries child_exit_status and duration_ms, including a child of an interactive PTY run killed at a Ctrl-C or TERM stop; a stopped run writes only the pairs it started.
@row_add n=463 row=contract/parse-rejected/claude-agents-json area=log-coverage rec=fct-lhnmh6dy@1
boundary: parse-rejected diagnostic line
item: parser claude-agents-json
promise: A claude agents --json parse failure, oversize or malformed, gives one parse-rejected line with parser claude-agents-json and the value unknown.
@row_add n=464 row=decision/counters-from-home-role-files area=metric-coverage rec=fct-iwloyhyc@1
name: Counters derived from home-level role files
rule: A counter of an event, detail or refusal value is derived with jq over the home-level diagnostics role files only, skipping a torn line.
why: Detail files repeat lines.
@row_add n=465 row=decision/histograms-as-raw-samples area=metric-coverage rec=fct-zrfczzyu@1
name: Histograms as raw samples
rule: A latency distribution is the raw duration_ms samples per event, with no bucketing; quantiles are taken by sort and index in jq.
@row_add n=466 row=decision/gauges-from-harness-snapshots area=metric-coverage rec=fct-wlstziny@1
name: Gauges from harness snapshots
rule: A current-value gauge is read from harness snapshots: budget.read_at, and liveness via agent-run status.
@row_add n=467 row=contract/derived-metrics/viola-hook-duration-ms area=metric-coverage rec=fct-dhdxvyfp@1
boundary: derived metrics
item: viola.hook.duration_ms
promise: Hook latency per hook_event, a histogram in ms of raw hook-decision duration_ms samples; its gate is the hyperfine max.
@row_add n=468 row=contract/derived-metrics/viola-channel-duration-ms area=metric-coverage rec=fct-osbt2dub@1
boundary: derived metrics
item: viola.channel.duration_ms
promise: Channel call latency per method, a histogram in ms of raw channel-response duration_ms samples.
@row_add n=469 row=contract/derived-metrics/viola-send-confirm-ms area=metric-coverage rec=fct-zjs7gija@1
boundary: derived metrics
item: viola.send.confirm_ms
promise: Send to readback latency, a histogram in ms of the duration_ms of send-confirmed lines whose confirmed is not false; a sample runs from the send's arrival at the wrapper, so it includes the readiness gate's wait ahead of the window.
@row_add n=470 row=contract/derived-metrics/viola-send-outcome area=metric-coverage rec=fct-5mdkcty6@1
boundary: derived metrics
item: viola.send.outcome
promise: Send terminal state, a counter of send-confirmed by confirmed and send-refused by refusal, detail and side.
@row_add n=471 row=contract/derived-metrics/viola-wait-outcome area=metric-coverage rec=fct-czhvmddm@1,fct-l3f7hrdu@2
boundary: derived metrics
item: viola.wait.outcome
promise: A counter of channel-response outcome for method wait, counting only the wait calls answered by a channel-response; a wait refused before any frame writes none.
@row_add n=472 row=contract/derived-metrics/viola-state-recovered area=metric-coverage rec=fct-bzg5tciy@1
boundary: derived metrics
item: viola.state.recovered
promise: Heal and recovery under chaos, a counter of state-recovered by detail.
@row_add n=473 row=contract/derived-metrics/viola-parse-skipped area=metric-coverage rec=fct-ui2du4ba@1
boundary: derived metrics
item: viola.parse.skipped
promise: Skipped records, a gauge read from the skipped field of list --json.
@row_add n=474 row=contract/derived-metrics/viola-parse-rejected area=metric-coverage rec=fct-pabhuwqy@1
boundary: derived metrics
item: viola.parse.rejected
promise: Parse rejections per parser, a counter of parse-rejected by parser, detail and count.
@row_add n=475 row=contract/derived-metrics/viola-compat-unknown area=metric-coverage rec=fct-iynwva54@1
boundary: derived metrics
item: viola.compat.unknown
promise: A counter of the unknown fallback, from send-refused with refusal unknown.
@row_add n=476 row=contract/derived-metrics/viola-child-exit area=metric-coverage rec=fct-zkvtfud4@1
boundary: derived metrics
item: viola.child.exit
promise: Child and probe exits, a counter of process-exit by subject, child_exit_status and exit_source.
@row_add n=477 row=contract/derived-metrics/viola-panic area=metric-coverage rec=fct-wl54ld47@1
boundary: derived metrics
item: viola.panic
promise: A counter of panic events, an SLO that must stay 0.
@row_add n=478 row=decision/quantiles-report-only area=metric-coverage rec=fct-sqywzwf6@1
name: Quantiles are report-only
rule: Latency quantiles are for reports only; the gates use the max, and the quantile read skips a torn line rather than aborting.
@row_none n=117 rec=fct-tgmj25nt@1 type=5
@row_none n=118 rec=fct-qat4n66a@1 type=5
@row_add n=479 row=decision/linux-only-os-target area=obs-scope-summary rec=fct-ztmbo3l4@1
name: Linux-only OS target
rule: The OS target is Linux only: the core on any current distribution, CI on ubuntu-latest.
@row_none n=119 rec=fct-737rcqr6@1 type=5
@row_none n=120 rec=fct-drmiyyaa@2 type=5
@row_none n=121 rec=fct-4jptauao@1 type=9
@row_none n=122 rec=fct-lt3l45qm@1 type=9
@row_none n=123 rec=fct-4gkcm5vc@1 type=9
@row_none n=124 rec=fct-dgq2ctzu@1 type=9
@row_none n=125 rec=fct-m5nv47uz@1 type=S
@row_none n=126 rec=fct-d4mbb5n4@1 type=0
what: an instrumentability tag with no entity named
@row_add n=480 row=decision/anyhow-error-capture-boundary area=obs-scope-summary rec=fct-iyfle2l2@1
name: The anyhow crate is the error-capture boundary
rule: The only crate that uses anyhow is the error-capture boundary, with context chains only at dispatch; the full chain goes to diagnostics only.
@row_add n=481 row=decision/serde-chain-fixed-message area=obs-scope-summary rec=fct-iyfle2l2@1
name: Serde error chains become a fixed message on stderr
rule: An error chain that contains a serde source becomes a fixed message before it reaches stderr.
@row_none n=127 rec=fct-qjje6f7i@1 type=S
@row_none n=128 rec=fct-5xfc4cig@1 type=0
what: an instrumentability tag with no entity named
@row_add n=482 row=contract/unreachable-cause/strict-modes-failed area=obs-scope-summary rec=fct-y6hfs4wy@1
boundary: exit 21 cause words
item: strict-modes-failed
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_add n=483 row=contract/unreachable-cause/instance-dead area=obs-scope-summary rec=fct-y6hfs4wy@1
boundary: exit 21 cause words
item: instance-dead
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_add n=484 row=contract/unreachable-cause/unwrapped-name area=obs-scope-summary rec=fct-y6hfs4wy@1
boundary: exit 21 cause words
item: unwrapped-name
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_add n=485 row=contract/unreachable-cause/endpoint-lost area=obs-scope-summary rec=fct-y6hfs4wy@1
boundary: exit 21 cause words
item: endpoint-lost
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_add n=486 row=contract/unreachable-cause/server-verify-failed area=obs-scope-summary rec=fct-y6hfs4wy@1
boundary: exit 21 cause words
item: server-verify-failed
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_add n=487 row=contract/unreachable-cause/socket-dir-refused area=obs-scope-summary rec=fct-y6hfs4wy@1
boundary: exit 21 cause words
item: socket-dir-refused
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_none n=129 rec=fct-kemgo46r@1 type=S
@row_none n=130 rec=fct-cfbdfwxc@1 type=0
what: an instrumentability tag with no entity named
@row_none n=131 rec=fct-mr2vh66r@1 type=0
what: an instrumentability tag with no entity named
@row_add n=488 row=decision/ipc-crate-sync-instrumentation area=obs-scope-summary rec=fct-aowjxl3a@1,fct-2vabautb@1
name: Synchronous instrumentation in the IPC crate
rule: Instrumentation in the interprocess IPC crate must be synchronous.
why: Stack forbids Tokio in the crate's graph unless its feature is enabled.
@row_none n=132 rec=fct-qrw2kvvm@1 type=0
what: an instrumentability tag with no entity named
@row_add n=489 row=decision/state-crate-sync-instrumentation area=obs-scope-summary rec=fct-pssgrgft@1,fct-g6misrd3@1
name: Synchronous instrumentation in the state crate
rule: Instrumentation in the state crate built on ndjson, atomic-write-file and sysinfo must be synchronous.
why: Stack bans Tokio in that crate.
@row_none n=133 rec=fct-gwe6beo3@1 type=S
@row_none n=134 rec=fct-vaolg4ug@1 type=0
what: an instrumentability tag with no entity named
@row_add n=490 row=decision/stripped-claude-values-never-logged area=obs-scope-summary rec=fct-add24jhw@1
name: Stripped CLAUDE values never logged
rule: R8-stripped CLAUDE* values are never logged anywhere.
why: Its outputs are the most sensitive in the product.
@row_none n=135 rec=fct-crky2gz3@1 type=S
@row_none n=136 rec=fct-yqxfnq5h@1 type=0
what: an instrumentability tag with no entity named
@row_add n=491 row=decision/mcp-no-telemetry-on-stdout area=obs-scope-summary rec=fct-drz74jk7@1
name: No telemetry on the MCP server's stdout
rule: The MCP server sends no telemetry to stdout.
why: stdout is reserved for MCP frames.
@row_none n=137 rec=fct-3afzfojv@1 type=S
@row_none n=138 rec=fct-2drawtvt@1 type=0
what: an instrumentability tag with no entity named
@row_add n=492 row=decision/claude-boundary-only-instrumentation area=obs-scope-summary rec=fct-t3jwc6wb@1
name: Boundary-only instrumentation of claude
rule: The hosted claude binary is instrumented only at its boundary: spawn and exit, paste writes, and the vt100 readiness-gate outcome.
why: claude is a third-party binary run unmodified.
@row_add n=493 row=decision/claude-exit-on-process-handle area=obs-scope-summary rec=fct-t3jwc6wb@1
name: Claude exit detected on the process handle
rule: The exit of the hosted claude process is detected on the process handle, never on EOF.
@row_none n=139 rec=fct-2h4irdjs@1 type=S
@row_none n=140 rec=fct-podhtuoc@1 type=0
what: an instrumentability label naming no entity and no reason
@row_add n=494 row=decision/hook-mcp-instrumented-at-boundary area=obs-scope-summary rec=fct-lpbmjfvf@1
name: Hook and MCP instrumented at the boundary
rule: Hook and MCP traffic is instrumented only at viola's own boundary: hook-invoked and hook-decision, MCP tool-call handling, and the viola mcp stdin cut.
why: The caller is Claude Code, so viola sees only the inbound stdin payload and its own response, and no trace context arrives from Claude Code.
@row_none n=141 rec=fct-xenjwzhp@1 type=0
what: an instrumentability label naming no entity and no reason
@row_add n=495 row=decision/agents-cli-instrumented-at-boundary area=obs-scope-summary rec=fct-k7csbv6h@1,fct-drucxpuy@1
name: claude agents CLI instrumented at spawn, exit and parse
rule: The external `claude agents --json` CLI is instrumented only at its spawn, its exit and its parse outcome.
why: An external CLI.
@row_add n=496 row=decision/agents-cli-parse-failure-unknown area=obs-scope-summary rec=fct-k7csbv6h@1,fct-drucxpuy@1
name: claude agents parse failure reads unknown
rule: A failed parse of `claude agents --json` output reads "unknown" plus a counter.
@row_add n=497 row=decision/agents-cli-output-untrusted area=obs-scope-summary rec=fct-k7csbv6h@1,fct-drucxpuy@1
name: claude agents output untrusted
rule: The output rows of `claude agents --json` are treated as untrusted.
@row_none n=142 rec=fct-ly2hzjle@1 type=0
what: an instrumentability label naming no entity and no reason
@row_add n=498 row=decision/statusline-shell-out-instrumentation area=obs-scope-summary rec=fct-nvhb6dqn@1,fct-vr7juzn5@1
name: Status-line shell-out instrumented by duration and exit
rule: The `viola hook statusline` shell-out of the user's command is instrumented by its duration and exit status only.
why: A user-written shell command.
@row_add n=499 row=decision/statusline-command-only-in-diagnostics area=obs-scope-summary rec=fct-nvhb6dqn@1
name: Status-line command string kept to diagnostics
rule: The statusline_command string is never logged outside diagnostics/.
@row_none n=143 rec=fct-tedqfbbw@1 type=S
@row_none n=144 rec=fct-2v5blby2@1 type=0
what: an instrumentability label naming no entity and no reason
@row_add n=500 row=decision/instrument-call-sites-not-os-internals area=obs-scope-summary rec=fct-f3waaflx@2
name: OS operations instrumented at viola's call sites
rule: OS-level operations (bind, connect, lock, rename, kill fallback, the server check's peer reads, the strict-modes stat reads, the socket directory check) are instrumented at viola's own call sites, not inside the OS.
@row_add n=501 row=contract/exit-cause-detail/server-verify-failed area=obs-scope-summary rec=fct-f3waaflx@2
boundary: Exit cause detail codes
item: server-verify-failed
promise: A failed server check of peer euid, peer pid or start time is logged by the CLI and the hook as server-verify-failed; MCP writes no line for it.
@row_add n=502 row=contract/exit-cause-detail/socket-dir-refused area=obs-scope-summary rec=fct-f3waaflx@2,fct-oiibon7o@2,fct-h2wm2goy@2
boundary: Exit cause detail codes
item: socket-dir-refused
promise: A refused socket directory has its own detail code socket-dir-refused and writes its line (MCP writes none); on run and on a start made by viola revive that line is one process-exit{subject:"self", exit_code:1} before any snapshot, child or bind.
@row_add n=503 row=decision/refused-peer-dropped-without-line area=obs-scope-summary rec=fct-f3waaflx@2,fct-6xpygltd@1
name: Refused channel peer dropped without a log line
rule: A channel stream whose peer euid is refused is dropped on the accept thread before its worker thread and before any frame: the peer reads end-of-file, and the wrapper writes no log line and nothing on the terminal.
why: There is no frame to key a line on, and channel-response's detail is closed.
@row_none n=145 rec=fct-kbibhsg7@1 type=S
@row_none n=146 rec=fct-or7ezsji@1 type=0
what: an instrumentability label for the persistent stores
@row_add n=504 row=contract/replayed-snapshot/live-fields area=obs-scope-summary rec=fct-a3522pml@1
boundary: A snapshot.json rebuilt by replay
item: endpoint, pid, started_at, statusline_command
promise: A snapshot.json rebuilt from events.ndjson carries none of these, so the endpoint reads unreachable, hook connect fails open with channel-unreachable, hook statusline prints nothing, and run's collision check reads the heartbeat alone.
@row_add n=505 row=decision/budget-metadata-loggable area=obs-scope-summary rec=fct-a3522pml@1
name: Budget metadata loggable as fields
rule: Operational metadata in budget.json (percentages, resets_at) may be logged as structured fields.
@row_none n=147 rec=fct-ks33cou5@1 type=S
@row_none n=148 rec=fct-tk3iawkf@1 type=0
what: an instrumentability label naming no entity and no reason
@row_add n=506 row=decision/config-observed-at-load area=obs-scope-summary rec=fct-p5bvvhtw@1
name: Configuration observed only at load
rule: Configuration is observed only at its parse at load time, logged as skipped-key counts.
why: Static configuration and data.
@row_none n=149 rec=fct-3bsvrrhe@1 type=S
@row_none n=150 rec=fct-fvn7dgcf@1 type=0
what: an instrumentability label naming no entity and no reason
@row_add n=507 row=decision/fixtures-not-telemetry area=obs-scope-summary rec=fct-6un2lrdt@1
name: Recorded fixtures are no telemetry source
rule: Recorded fixtures are scrub-required data, not a telemetry source, and are excluded from the observability plan.
why: Not product runtime.
@row_add n=508 row=decision/tracing-json-logging-pipeline area=obs-scope-summary rec=fct-noyuw3eo@1
name: tracing with a JSON subscriber for logging
rule: Logging and spans go through the Rust tracing facade with a tracing-subscriber JSON structured logger.
@row_add n=509 row=decision/panic-hook-replaced area=obs-scope-summary rec=fct-noyuw3eo@1,fct-3ftavspi@1,fct-foy4up7k@1
name: Panic hook replaced by a one-line writer
rule: Each process replaces the default panic hook, as the first statement in main and before any child spawns, with one that writes one JSON line and nothing to stderr.
why: The default hook prints to stderr, which breaks the hook contract and is the child's terminal under run.
@row_add n=510 row=absence/interactive-terminal-interface area=obs-scope-summary rec=fct-6ujsabka@1
capability: An interactive terminal interface: prompt, TUI or pager
@row_add n=511 row=decision/process-logs-file-sink-only area=obs-scope-summary rec=fct-oy7tgncm@1,fct-he3oql74@1
name: Process logs to a file sink only
rule: Process logs are written only as JSON lines to one file per process role under diagnostics/; there is no network exporter and no stdout sink.
why: viola makes no outbound network calls, and stdout carries --json results.
@row_add n=512 row=contract/cli-log-file/revive-list area=obs-scope-summary rec=fct-oy7tgncm@1
boundary: CLI process-log file
item: viola revive --list
promise: Logs as cli into the caller's cli-<VIOLA_NAME>.ndjson once its strict-modes check has passed; no file with no VIOLA_NAME.
@row_add n=513 row=contract/cli-log-file/link-unlink area=obs-scope-summary rec=fct-oy7tgncm@1
boundary: CLI process-log file
item: viola link / viola unlink
promise: Log as cli into the driven instance's cli-<driven>.ndjson, never the caller's VIOLA_NAME, once their instance check of it has passed.
@row_add n=514 row=contract/cli-log-file/revive-start area=obs-scope-summary rec=fct-oy7tgncm@1
boundary: CLI process-log file
item: viola revive start arm
promise: Logs as the wrapper, process="run", into diagnostics/run-<name>.ndjson.
@row_add n=515 row=contract/cli-json-error/strict-modes-failed area=obs-scope-summary rec=fct-oy7tgncm@1,fct-oiibon7o@2
boundary: CLI --json error document
item: instance-unreachable strict-modes-failed
promise: A refused instance check of send, wait, last, answer, pause, release, link or unlink exits 21 with the instance pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
@row_add n=516 row=contract/viola-verify/home-tree-refused area=obs-scope-summary rec=fct-oy7tgncm@1
boundary: viola verify
item: refused home-tree check
promise: A refused home-tree check, or a refused update_stamps, exits 1 with the home pair, opens no log and writes no line.
@row_add n=517 row=contract/service-identity/service-version area=obs-scope-summary rec=fct-uwlbnhdg@1
boundary: Service identity
item: service.version
promise: The compile-time CARGO_PKG_VERSION, the same value used as sender and writer.
@row_add n=518 row=absence/deployment-environments area=obs-scope-summary rec=fct-uwlbnhdg@1
capability: Deployment environments or hosting
@row_add n=519 row=decision/instance-from-name-not-cwd area=obs-scope-summary rec=fct-oiibon7o@2,fct-uwlbnhdg@1
name: Instance from its name, never the working directory
rule: The instance comes from VIOLA_NAME or an explicit argument, never from the working directory.
@row_add n=520 row=decision/fixed-stderr-messages area=obs-scope-summary rec=fct-oiibon7o@2
name: Fixed stderr refusal and fault lines
rule: Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
@row_add n=521 row=contract/cli-internal-error/stderr-line area=obs-scope-summary rec=fct-oiibon7o@2
boundary: CLI internal error
item: stderr line
promise: Without --json a catch-site fault prints the fixed line `error: internal error` to stderr.
@row_add n=522 row=decision/exit-code-log-field area=obs-scope-summary rec=fct-oiibon7o@2
name: Exit code as a log field
rule: The exit code (0, 1, 2, 10–14, 20, 21), the machine-readable outcome, is carried as a log field.
@row_add n=523 row=decision/sync-instrumentation-no-tokio area=obs-scope-summary rec=fct-cuxf35wq@1,fct-foy4up7k@1
name: Sync instrumentation without Tokio
rule: Instrumentation in the sync crates runs on std threads with no Tokio.
why: Hybrid concurrency.
guard: cargo deny ban
@row_add n=524 row=decision/run-silent-on-terminal area=obs-scope-summary rec=fct-i2uu7hiy@1,fct-giamwcrt@1
name: run silent on the terminal
rule: run writes nothing to the terminal while the child runs; its process log goes only to diagnostics/run-<name>.ndjson.
why: The terminal carries the child's screen.
@row_add n=525 row=contract/process-log/process area=obs-scope-summary rec=fct-ivwsjbo2@1,fct-7fgifl76@1,fct-uy5gif5o@1,fct-ogkwmfvl@1
boundary: Process log line
item: process
promise: The process role, one of run, hook, mcp, cli.
@row_add n=526 row=contract/heartbeat-file/tick area=obs-scope-summary rec=fct-3ftavspi@1
boundary: Heartbeat file
item: tick
promise: The heartbeat file is touched every 1 s as the liveness tick.
@row_add n=527 row=decision/child-exit-from-handle-wait area=obs-scope-summary rec=fct-3ftavspi@1
name: Child exit from the process handle
rule: The child's process-exit comes from the wait on its process handle, never from reader end-of-file.
why: Child exit is detected on the process handle.
@row_add n=528 row=decision/hook-panic-one-line area=obs-scope-summary rec=fct-t4pdf4hq@1,fct-ogkwmfvl@1
name: Hook panic logged as one line
rule: A panic in a hook is caught with catch_unwind and logged as one level:"ERROR", event:"panic" line.
@row_add n=529 row=decision/hook-lines-required area=obs-scope-summary rec=fct-t4pdf4hq@1
name: Hook invocation and decision lines required
rule: Every hook process writes its hook-invoked and hook-decision lines.
@row_none n=151 rec=fct-x5lzlwa7@1 type=0
what: a not-applicable frontend line
@row_add n=530 row=decision/hook-never-writes-stderr area=obs-scope-summary rec=fct-d5lpbhzx@1,fct-ogkwmfvl@1
name: Hook never writes stderr
rule: A hook process never writes to stderr.
@row_add n=531 row=contract/hook-stdout/hook-event area=obs-scope-summary rec=fct-d5lpbhzx@1
boundary: Hook stdout
item: viola hook <event>
promise: stdout carries only the decision body.
@row_add n=532 row=contract/hook-stdout/statusline area=obs-scope-summary rec=fct-d5lpbhzx@1
boundary: Hook stdout
item: viola hook statusline
promise: stdout carries only the user's status-line command's output.
@row_add n=533 row=contract/hook-stdout/capture-arms area=obs-scope-summary rec=fct-d5lpbhzx@1
boundary: Hook stdout
item: --capture arms
promise: The two hidden --capture arms write no log sink and print nothing, except --answers' probe decision body.
@row_add n=534 row=decision/hook-log-failure-swallowed area=obs-scope-summary rec=fct-d5lpbhzx@1,fct-ogkwmfvl@1
name: Hook log write failure swallowed
rule: A failed log write in a hook is swallowed and the hook still exits 0.
@row_add n=535 row=decision/hook-silent-without-trusted-instance area=obs-scope-summary rec=fct-7fgifl76@1,fct-kv6frzyp@2
name: Hook silent without a trusted instance
rule: A hook with no VIOLA_NAME, or whose instance is absent or refused by its strict-modes check, exits 0 with no config read, no stdin read, no log line, no stderr, no frame and no SessionEnd append.
@row_add n=536 row=decision/hook-instance-check-before-obs-init area=obs-scope-summary rec=fct-7fgifl76@1
name: Hook instance check before logging starts
rule: The hook's event and dialog paths run the instance strict-modes check before reading the diagnostics level and initialising logging.
except: The two --capture arms run ahead of the check.
@row_add n=537 row=decision/hook-latency-budget area=obs-scope-summary rec=fct-kv6frzyp@2
name: Hook latency budget
rule: Each hook runs within max < 1.0 s, and logger init adds no blocking I/O beyond one file open and append.
guard: run --perf hook rows under SPINE_DEADLINE
except: viola hook statusline is outside the budget.
@row_add n=538 row=decision/correlation-in-corr-field area=obs-scope-summary rec=fct-kkjide4x@1
name: Correlation in the corr log field
rule: Correlation across a channel call travels in the corr log field; channel params declare no trace-context field.
@row_none n=152 rec=fct-4iim5xgf@1 type=0
what: a not-applicable frontend line
@row_none n=153 rec=fct-rj2a6toh@1 type=0
what: which log file each side of a channel call writes to
@row_add n=539 row=contract/channel-params/sender area=obs-scope-summary rec=fct-qe2js5tq@1
boundary: Channel params
item: sender
promise: params.sender carries the peer's version, CARGO_PKG_VERSION.
@row_add n=540 row=contract/json-rpc-frame/id area=obs-scope-summary rec=fct-6xpygltd@1
boundary: JSON-RPC frame
item: id
promise: Ids are integers, monotonic per connection, and so not unique across connections or processes.
@row_add n=541 row=contract/json-rpc-error/32603 area=obs-scope-summary rec=fct-6xpygltd@1
boundary: JSON-RPC error
item: -32603
promise: The -32603 error carries the message "internal error" with data null.
@row_add n=542 row=decision/from-not-authenticated area=obs-scope-summary rec=fct-6xpygltd@1
name: from never logged as authenticated
rule: The self-reported from field is never logged as an authenticated identity.
@row_none n=154 rec=fct-ff4quohy@1 type=4
@row_none n=155 rec=fct-g6v6qpwu@1 type=0
what: a not-applicable frontend line
@row_add n=543 row=decision/mcp-log-sink area=obs-scope-summary rec=fct-g4wb4nmr@1,fct-k56v7yhe@1,fct-he3oql74@1,fct-khwnpdov@2
name: MCP log sink
rule: viola mcp always logs to <home>/diagnostics/mcp.ndjson once the home and its diagnostics/ pass strict-modes, with stderr JSON only when that file cannot be opened.
@row_add n=544 row=decision/subscriber-explicit-writer area=obs-scope-summary rec=fct-g4wb4nmr@1,fct-foy4up7k@1
name: Subscriber installed with an explicit writer
rule: The tracing subscriber is always installed with an explicit writer, never stdout.
why: tracing-subscriber fmt() writes to stdout unless a writer is configured, and stdout carries MCP frames.
@row_add n=545 row=contract/mcp-stdin/line-within-max-frame area=obs-scope-summary rec=fct-g4wb4nmr@1
boundary: viola mcp stdin
item: line within MAX_FRAME
promise: A line of at most MAX_FRAME bytes, its newline counted, passes whole.
@row_add n=546 row=contract/mcp-stdin/oversize-before-initialize area=obs-scope-summary rec=fct-g4wb4nmr@1
boundary: viola mcp stdin
item: oversize line before initialize
promise: An oversize line before initialize exits the process 1 with both streams empty and the one parse-rejected record.
@row_add n=547 row=contract/mcp-server/instance area=obs-scope-summary rec=fct-uy5gif5o@1
boundary: MCP server identity
item: instance
promise: The server's own VIOLA_NAME when set, which is also the from it adds.
@row_add n=548 row=decision/mcp-results-codes-only area=obs-scope-summary rec=fct-k56v7yhe@1
name: MCP rejections carry codes only
rule: MCP isError results and the log lines of rejected calls carry codes only.
@row_add n=549 row=contract/mcp-iserror/instance-unreachable area=obs-scope-summary rec=fct-k56v7yhe@1
boundary: MCP isError result
item: instance-unreachable
promise: The instance-unreachable detail is one closed cause: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed or socket-dir-refused; the CLI's document less v, its text `refused: instance-unreachable`.
@row_add n=550 row=contract/mcp-iserror/invalid-params area=obs-scope-summary rec=fct-k56v7yhe@1
boundary: MCP isError result
item: invalid-params
promise: The invalid-params detail is a closed field word.
@row_add n=551 row=contract/mcp-iserror/refusal area=obs-scope-summary rec=fct-k56v7yhe@1
boundary: MCP isError result
item: refusal
promise: A peer refusal that is no code word reads unknown, and a string detail that is no code word reads null.
@row_add n=552 row=contract/mcp-iserror/wrapper-fault area=obs-scope-summary rec=fct-k56v7yhe@1
boundary: MCP isError result
item: wrapper-fault
promise: The wrapper-fault detail is a closed sanitised object: the fixed message of the five codes with data null or closed values; a code outside the five reads message "wrapper fault", data null.
@row_add n=553 row=decision/no-otel-exporter area=obs-scope-summary rec=fct-foy4up7k@1
name: No OTel SDK exporter
rule: v1 initialises no OTel SDK exporter; the tracing facade plus the tracing-subscriber JSON formatter is the binding pipeline.
why: No outbound network calls, a new listener needs a security decision, stdout is reserved on hook, mcp and run, and cargo deny bans Tokio and C-building crates.
except: An optional OTel bridge layer may feed only an in-process or file sink and stays Tokio-free on the sync crates.
@row_add n=554 row=decision/process-init-order area=obs-scope-summary rec=fct-foy4up7k@1
name: Process logging init order
rule: Each process installs its panic hook, parses arguments, resolves home and instance and runs its strict-modes check, opens its diagnostics file append-mode 0600 in a 0700 directory, installs the subscriber, then emits process-start.
why: For run this completes before the start sequence so the collision check and version gate are logged; for mcp, before the stdio transport starts.
@row_add n=555 row=decision/tracing-subscriber-version-floor area=obs-scope-summary rec=fct-he3oql74@1
name: tracing-subscriber version floor
rule: tracing-subscriber is held at version 0.3.20 or later.
why: The security floor for RUSTSEC-2025-0055.
@row_add n=556 row=decision/log-level-not-from-env area=obs-scope-summary rec=fct-he3oql74@1
name: Log level never from the environment
rule: The log level filter is never read from RUST_LOG; any verbosity control comes from config.json or a CLI flag and never bypasses redaction.
why: Environment variables are not a configuration channel, and no env, flag or config value may disable a control.
@row_add n=557 row=decision/json-to-file-only-mode area=obs-scope-summary rec=fct-he3oql74@1
name: JSON-to-file the only logging mode
rule: JSON-to-file is the only logging mode; there is no separate agent-mode flag.
@row_add n=558 row=contract/process-log/timestamp area=obs-scope-summary rec=fct-he3oql74@1,fct-ogkwmfvl@1
boundary: Process log line
item: timestamp
promise: RFC 3339 UTC with milliseconds and Z.
@row_add n=559 row=contract/event-line/shape area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: events.ndjson event line
item: line shape
promise: Exactly {"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}.
@row_add n=560 row=contract/process-log/level area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log line
item: level
promise: One of DEBUG, INFO, WARN, ERROR.
@row_add n=561 row=contract/process-log/target area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log line
item: target
promise: A required field.
@row_add n=562 row=contract/process-log/message area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log line
item: message
promise: A required field, as fields.message or the flattened message.
@row_add n=563 row=contract/process-log/event area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log line
item: event
promise: A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
@row_add n=564 row=contract/process-log/instance area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log line
item: instance
promise: A ViolaName or null.
@row_add n=565 row=contract/process-log/corr area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log line
item: corr
promise: Copied unchanged as a JSON number or string, never renamed.
@row_add n=566 row=contract/process-log-corr/channel-request area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: channel-request
promise: corr is the JSON-RPC request id.
@row_add n=567 row=contract/process-log-corr/channel-response area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: channel-response
promise: corr is the JSON-RPC request id.
@row_add n=568 row=contract/process-log-corr/dialog-raised area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: dialog-raised
promise: corr is the dialog_id.
@row_add n=569 row=contract/process-log-corr/dialog-answered area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: dialog-answered
promise: corr is the dialog_id.
@row_add n=570 row=contract/process-log-corr/hook-invoked area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: hook-invoked
promise: corr is the dialog_id for dialog hooks, otherwise null.
@row_add n=571 row=contract/process-log-corr/hook-decision area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: hook-decision
promise: corr is the dialog_id for dialog hooks, otherwise null.
@row_add n=572 row=contract/process-log-corr/send-issued area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: send-issued
promise: corr is the send cursor.
@row_add n=573 row=contract/process-log-corr/send-confirmed area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: send-confirmed
promise: corr is the send cursor.
@row_add n=574 row=contract/process-log-corr/send-refused area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: send-refused
promise: corr is the send cursor.
@row_add n=575 row=contract/process-log-corr/process-start area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: process-start
promise: corr is null.
@row_add n=576 row=contract/process-log-corr/process-exit area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: process-exit
promise: corr is null.
@row_add n=577 row=contract/process-log-corr/panic area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: panic
promise: corr is null.
@row_add n=578 row=decision/log-fields-additive-only area=obs-scope-summary rec=fct-ogkwmfvl@1
name: Required log fields only added to
rule: Fields may be added to process-log lines, but the required ones are never renamed or removed.
why: The harness greps on them.
@row_add n=579 row=decision/single-line-log-records area=obs-scope-summary rec=fct-ogkwmfvl@1
name: Single-line log records
rule: Every log record is one JSON line parseable by jq; never a multi-line stack trace.
@row_add n=580 row=decision/diagnostics-file-modes area=obs-scope-summary rec=fct-ogkwmfvl@1,fct-khwnpdov@2
name: Diagnostics file and directory modes
rule: Diagnostics files are 0600 inside 0700 directories.
guard: secret-scan test
@row_add n=581 row=decision/log-file-basenames-follow-tests area=obs-scope-summary rec=fct-khwnpdov@2
name: Log file names follow tests
rule: Process-log files are <home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson, as tests require.
why: Tests is upstream; arch and security place diagnostics under the instance directory instead.
@row_add n=582 row=contract/logs-wrapper/record-envelope area=obs-scope-summary rec=fct-khwnpdov@2
boundary: logs wrapper
item: record envelope
promise: Each process-log line is wrapped as {"src":"diag","file":<basename>,"record":<line>}.
@row_add n=583 row=decision/home-level-mcp-log-no-user-content area=obs-scope-summary rec=fct-khwnpdov@2
name: Home-level MCP log holds no user content
rule: An mcp.ndjson at home level never carries user content.
@row_add n=584 row=decision/diagnostics-checked-at-open area=obs-scope-summary rec=fct-khwnpdov@2
name: Diagnostics checked before open
rule: Before a role or detail file is created or narrowed, the existing diagnostics/ folder and the file are checked, a symlink refused and no group or other bit allowed; a refusal is a failed open.
@row_none n=156 rec=fct-6yzr723e@1 type=0
what: the list of surfaces a critical path crosses
@row_add n=585 row=contract/run-start-events/order area=obs-scope-summary rec=fct-jqtgp6hh@1
boundary: run start events in events.ndjson
item: order
promise: A run start writes wheel{cause:"start"}, then budget-gate, then session-start.
@row_add n=586 row=contract/run-log/cli-version area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run log line
item: cli_version
promise: A required field on run's log lines.
@row_add n=587 row=contract/run-log/cli-verified area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run log line
item: cli_verified
promise: A required field on run's log lines.
@row_add n=588 row=contract/run-log/exit-code area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run log line
item: exit_code
promise: A required field on run's log lines.
@row_add n=589 row=contract/run-exit-1-detail/already-live area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run exit-1 detail codes
item: already-live
promise: A live holder or a fresh replayed beat.
@row_add n=590 row=contract/run-exit-1-detail/stale-heartbeat area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run exit-1 detail codes
item: stale-heartbeat
promise: A stale holder.
@row_add n=591 row=contract/run-exit-1-detail/squatted-name area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run exit-1 detail codes
item: squatted-name
promise: A distinct detail code for a squatted name.
@row_add n=592 row=contract/run-exit-1-detail/sha-256-mismatch area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run exit-1 detail codes
item: sha-256 mismatch
promise: A distinct detail code for a sha-256 mismatch.
@row_add n=593 row=contract/run-exit-1-detail/no-session area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run exit-1 detail codes
item: no-session
promise: On a start made by viola revive, a distinct detail code.
@row_add n=594 row=contract/run-exit-1-detail/session-live area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run exit-1 detail codes
item: session-live
promise: On a start made by viola revive, a distinct detail code.
@row_add n=595 row=contract/run-exit-1-detail/cwd-missing area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run exit-1 detail codes
item: cwd-missing
promise: On a start made by viola revive, a distinct detail code.
@row_add n=596 row=decision/detail-codes-no-paths area=obs-scope-summary rec=fct-h2wm2goy@2
name: Detail codes carry no path or pid
rule: No path or pid appears in a detail code.
@row_none n=157 rec=fct-6dojwnci@1 type=S
@row_none n=158 rec=fct-7bjg2xvs@1 type=0
what: the list of surfaces a critical path crosses
@row_add n=597 row=contract/send-path-log-events/event-order area=obs-scope-summary rec=fct-zrxuhqku@1
boundary: Send path log events
item: event order
promise: A send logs channel-request, then send-issued, then hook-invoked (UserPromptSubmit, or SessionStart for a confirmed /clear or /compact; none awaited for an unconfirmable send), then send-confirmed or send-refused, then channel-response.
@row_add n=598 row=contract/send-log-line/required-fields area=obs-scope-summary rec=fct-gpdjinuy@1
boundary: Send log line
item: required fields
promise: A send line carries event, process, instance, corr, from, confirmed, refusal, detail and duration_ms.
@row_add n=599 row=contract/send-log-line/corr area=obs-scope-summary rec=fct-gpdjinuy@1,fct-exdrb7s2@1
boundary: Send log line
item: corr
promise: On send lines corr is the send cursor, which joins send-issued, send-confirmed and send-refused across CLI, MCP and wrapper; on channel lines corr is the JSON-RPC id.
@row_add n=600 row=contract/send-log-line/from area=obs-scope-summary rec=fct-gpdjinuy@1,fct-jlmrrvwc@1
boundary: Send log line
item: from
promise: The from field is self-reported and logged as unauthenticated, never as an identity.
@row_add n=601 row=contract/send-log-line/detail area=obs-scope-summary rec=fct-gpdjinuy@1
boundary: Send log line
item: detail
promise: The send detail is one of input-not-ready, no-prompt-submitted, control-character, empty-text or turn-running.
@row_none n=159 rec=fct-jztxixk3@1 type=S
@row_none n=160 rec=fct-r2x644q6@1 type=5
@row_add n=602 row=contract/wait-path-log-events/channel-pair area=obs-scope-summary rec=fct-leqqhyxz@1
boundary: Wait path log events
item: channel pair
promise: A wait logs channel-request and channel-response, the response carrying the outcome.
@row_add n=603 row=contract/wait-log-line/required-fields area=obs-scope-summary rec=fct-cetbw4yv@1
boundary: Wait log line
item: required fields
promise: A wait line carries event, process, instance, corr (the JSON-RPC id), after (a byte offset), timeout_ms, outcome, exit_code and duration_ms.
@row_add n=604 row=contract/wait-log-line/outcome area=obs-scope-summary rec=fct-cetbw4yv@1
boundary: Wait log line
item: outcome
promise: The wait outcome is the woken kind, timed_out or instance-unreachable.
@row_add n=605 row=contract/wait-log-line/exit-code-21 area=obs-scope-summary rec=fct-cetbw4yv@1
boundary: Wait log line
item: exit_code 21
promise: A wait exits 21 when the wrapper is killed mid-wait (detail endpoint-lost, during call), or when a CLI wait meets a refused socket directory before any frame (detail socket-dir-refused, during connect, no channel-request).
@row_none n=161 rec=fct-vtwqamxe@1 type=S
@row_none n=162 rec=fct-ewxn4524@1 type=5
@row_add n=606 row=contract/dialog-path-log-events/event-order area=obs-scope-summary rec=fct-3dvlg5hs@1
boundary: Dialog path log events
item: event order
promise: A dialog logs hook-invoked, dialog-raised, dialog-answered, then hook-decision, whose decision is null on deadline expiry.
@row_add n=607 row=contract/dialog-log-line/required-fields area=obs-scope-summary rec=fct-ycxx4ij4@2
boundary: Dialog log line
item: required fields
promise: A dialog line carries event, process, instance, corr, dialog_kind, refusal or detail, deadline_hit and duration_ms.
@row_add n=608 row=contract/dialog-log-line/corr area=obs-scope-summary rec=fct-ycxx4ij4@2,fct-exdrb7s2@1
boundary: Dialog log line
item: corr
promise: On dialog lines corr is the dialog_id, which joins hook, wrapper and driver lines.
@row_add n=609 row=contract/dialog-log-line/unknown-dialog area=obs-scope-summary rec=fct-ycxx4ij4@2
boundary: Dialog log line
item: unknown-dialog
promise: The detail unknown-dialog goes with exit 13.
@row_add n=610 row=contract/dialog-log-line/unverified-cli area=obs-scope-summary rec=fct-ycxx4ij4@2
boundary: Dialog log line
item: unverified-cli
promise: The detail unverified-cli goes with exit 12.
@row_add n=611 row=contract/dialog-log-line/server-verify-failed area=obs-scope-summary rec=fct-ycxx4ij4@2
boundary: Dialog log line
item: server-verify-failed
promise: A server-verify failure exits 21 on CLI answer with during connect, and on the dialog hook fails open with corr null and decision_emitted false; the line names no pid, start time, path or endpoint.
@row_add n=612 row=contract/dialog-log-line/socket-dir-refused area=obs-scope-summary rec=fct-ycxx4ij4@2
boundary: Dialog log line
item: socket-dir-refused
promise: A refused socket directory exits 21 on CLI answer with during connect, and on the dialog hook fails open with corr null and decision_emitted false; the line names no pid, start time, path or endpoint.
@row_add n=613 row=contract/cli-frame-verbs/strict-modes-failed area=obs-scope-summary rec=fct-ycxx4ij4@2,fct-ydc7ueix@3
boundary: CLI frame verbs
item: strict-modes-failed
promise: A strict-modes refusal of send, wait, last, answer, pause or release writes no line and prints only the fixed instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, exit 21, with no pid, path or endpoint.
@row_add n=614 row=contract/dialog-hook/refused-instance-check area=obs-scope-summary rec=fct-ycxx4ij4@2
boundary: Dialog hook
item: refused instance check
promise: A refused instance check on the dialog hook exits 0 at once with empty streams, no frame and no line.
@row_none n=163 rec=fct-qpelf55l@1 type=S
@row_none n=164 rec=fct-7ka2fpsx@1 type=5
@row_add n=615 row=contract/wheel-path-log-events/pause-and-release area=obs-scope-summary rec=fct-m4ttxbfi@1
boundary: Wheel path log events
item: pause and release
promise: A wheel take-over logs send-refused for the refused send and a channel-request and channel-response pair for pause and for release.
@row_add n=616 row=contract/diag-line-event/release-from-driver area=obs-scope-summary rec=fct-m4ttxbfi@1,fct-n7xftqkr@2
boundary: Diag line event
item: release-from-driver
promise: release-from-driver is its own logged event, admitted by the diag-line schema, not a generic -32602 wrapper fault.
@row_add n=617 row=contract/wheel-log-line/human-typing area=obs-scope-summary rec=fct-ylcomffg@1
boundary: Wheel log line
item: human-typing
promise: A send refused while a human holds the wheel logs refusal human-typing, detail null or manual-pause, the wheel holder and exit_code 10.
@row_none n=165 rec=fct-aajrejox@1 type=S
@row_none n=166 rec=fct-rrskex2m@1 type=5
@row_add n=618 row=contract/budget-path-log-events/event-set area=obs-scope-summary rec=fct-te5knmlh@1
boundary: Budget path log events
item: event set
promise: The budget path logs hook-invoked and hook-decision for statusline, and send-refused with budget-paused.
@row_add n=619 row=contract/budget-log-line/required-fields area=obs-scope-summary rec=fct-kbwooksf@1
boundary: Budget log line
item: required fields
promise: A budget line carries event, process, instance, corr, window, used_percentage, resets_at, paused, refusal, exit_code, shell_exit_status and duration_ms.
@row_add n=620 row=contract/budget-log-line/window area=obs-scope-summary rec=fct-kbwooksf@1
boundary: Budget log line
item: window
promise: The budget window is five-hour or seven-day.
@row_add n=621 row=contract/budget-log-line/budget-paused area=obs-scope-summary rec=fct-kbwooksf@1
boundary: Budget log line
item: budget-paused
promise: A send refused by the budget gate logs refusal budget-paused with exit_code 11.
@row_add n=622 row=decision/statusline-command-never-in-home-logs area=obs-scope-summary rec=fct-kbwooksf@1,fct-3znzbm34@1
name: Statusline command never in home-level logs
rule: The statusline_command string never appears outside diagnostics/, reaching a detail file only as part of an error chain; its shell-out records only shell_exit_status and duration_ms.
why: The statusline command is classed High for integrity.
@row_none n=167 rec=fct-mydtv3ut@1 type=S
@row_add n=623 row=decision/channel-log-scrub-rule area=obs-scope-summary rec=fct-jlmrrvwc@1
name: Channel log scrub rule
rule: Channel request and response lines log only the method, id or corr, the result class (ok, refusal or error code), the sender version and the closed detail word, never params or result bodies.
@row_add n=624 row=contract/channel-response/internal-error area=obs-scope-summary rec=fct-jlmrrvwc@1
boundary: Channel response line
item: -32603
promise: A -32603 is logged as the fixed text internal error.
@row_add n=625 row=decision/send-refused-by-each-deciding-side area=obs-scope-summary rec=fct-gtr2jsdi@1
name: Send refusal logged by each deciding side
rule: A send-refused with detail control-character or empty-text is emitted by each side that decides it, client and wrapper, without the text.
@row_add n=626 row=decision/hook-fail-open-logged-to-hook-file area=obs-scope-summary rec=fct-mxwnyc6t@2
name: Hook fail-open logged to the hook file
rule: Hook fail-open paths (oversize stdin, malformed JSON, channel failure, server-verify mismatch, refused socket directory, missing VIOLA_NAME) are logged to hook-<name>.ndjson only.
@row_add n=627 row=contract/hook-decision/server-verify-failed area=obs-scope-summary rec=fct-mxwnyc6t@2
boundary: Hook decision line
item: server-verify-failed
promise: A server-verify mismatch is one hook-decision at WARN with detail server-verify-failed and no frame sent; on a dialog hook, hook-invoked with corr null then hook-decision with corr null and decision_emitted false; no pid, start time, path or endpoint.
@row_add n=628 row=contract/hook-decision/socket-dir-refused area=obs-scope-summary rec=fct-mxwnyc6t@2,fct-5ckzuzgq@2
boundary: Hook decision line
item: socket-dir-refused
promise: A refused socket directory on viola hook writes one hook-decision in the server-verify form with detail socket-dir-refused, and the hook exits 0.
@row_add n=629 row=contract/hook-fail-open/exit area=obs-scope-summary rec=fct-mxwnyc6t@2
boundary: Hook fail-open
item: exit
promise: A hook that fails open exits 0 and leaves stderr empty.
@row_add n=630 row=decision/drift-reports-detail-only area=obs-scope-summary rec=fct-mxwnyc6t@2,fct-glcip7ij@1
name: Drift reports in the detail file only
rule: A full serde drift report goes only to the instance-scoped detail file, as its drift_report field.
@row_add n=631 row=contract/mcp-invalid-params/detail area=obs-scope-summary rec=fct-4bhscgzh@1
boundary: MCP invalid-params
item: detail
promise: The detail of an MCP invalid-params is a closed field word: target, text, after, timeout_ms, dialog_id or response.
@row_add n=632 row=contract/mcp-frame-tool-refusal/strict-modes-failed area=obs-scope-summary rec=fct-4bhscgzh@1,fct-ydc7ueix@3
boundary: MCP frame tool refusal
item: strict-modes-failed
promise: A frame tool (send, wait, last, answer) whose instance check refuses returns instance-unreachable with detail strict-modes-failed, writes no frame and no line.
@row_add n=633 row=contract/mcp-frame-tool-refusal/no-live-endpoint area=obs-scope-summary rec=fct-4bhscgzh@1
boundary: MCP frame tool refusal
item: no live endpoint
promise: A frame tool with no live endpoint returns instance-dead or unwrapped-name and writes no frame.
@row_add n=634 row=contract/mcp-frame-tool-refusal/socket-dir-refused area=obs-scope-summary rec=fct-4bhscgzh@1,fct-5ckzuzgq@2
boundary: MCP frame tool refusal
item: socket-dir-refused
promise: A frame tool whose socket directory is refused returns socket-dir-refused, writes no frame and no line.
@row_add n=635 row=contract/mcp-frame-tool-refusal/server-verify-failed area=obs-scope-summary rec=fct-4bhscgzh@1
boundary: MCP frame tool refusal
item: server-verify-failed
promise: A frame tool whose server check fails or whose server euid is refused returns server-verify-failed and writes no frame.
@row_add n=636 row=decision/messaging-secrets-never-logged area=obs-scope-summary rec=fct-rkannnby@1,fct-ja2bi6wz@1
name: Messaging secrets never logged
rule: CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every R8-stripped CLAUDE* value are never logged in any sink, detail files and fixtures included.
guard: secret-scan test
@row_add n=637 row=decision/r8-strip-logged-as-names-only area=obs-scope-summary rec=fct-rkannnby@1,fct-ja2bi6wz@1
name: R8 strip logged as names only
rule: The R8 environment strip is logged as a count and lists of stripped and kept variable names, values never read, and environment maps never pass to a span or event.
@row_add n=638 row=decision/serde-errors-become-fixed-message area=obs-scope-summary rec=fct-rkannnby@1,fct-glcip7ij@1
name: Serde errors become a fixed message
rule: An error chain holding a serde source is replaced by a fixed message before it reaches stderr.
@row_add n=639 row=decision/redaction-cannot-be-disabled area=obs-scope-summary rec=fct-rkannnby@1
name: Redaction cannot be disabled
rule: No environment variable, flag or config setting may disable log redaction.
@row_add n=640 row=decision/diagnostics-owner-only-permissions area=obs-scope-summary rec=fct-5ckzuzgq@2
name: Diagnostics owner-only permissions
rule: Files under diagnostics/ are 0600 inside 0700 directories.
guard: secret-scan test
@row_add n=641 row=contract/process-exit/socket-dir-refused-on-start area=obs-scope-summary rec=fct-5ckzuzgq@2
boundary: Process exit line
item: socket-dir-refused on a start
promise: A refused socket directory on a start writes one process-exit with detail socket-dir-refused, exit 1.
@row_add n=642 row=contract/process-exit/socket-dir-refused-on-frame-verb area=obs-scope-summary rec=fct-5ckzuzgq@2
boundary: Process exit line
item: socket-dir-refused on a CLI frame verb
promise: A refused socket directory on a CLI frame verb writes one process-exit with detail socket-dir-refused, exit 21, during connect.
@row_add n=643 row=decision/pinned-copy-race-adds-no-line area=obs-scope-summary rec=fct-5ckzuzgq@2
name: Pinned-copy race adds no telemetry
rule: A pinned-exe hash mismatch reached when a concurrent first start wins the no-clobber placement leaves the file as found and adds no line, field or detail code beyond pinned-hash-mismatch.
@row_add n=644 row=decision/skipped-counters area=obs-scope-summary rec=fct-5ckzuzgq@2,fct-plg35lpj@1
name: Skipped counters
rule: Unknown event kinds, unknown fields and torn lines are each counted in skipped counters.
@row_add n=645 row=contract/process-log/child-and-probe-lifecycle area=obs-scope-summary rec=fct-u2gugwuk@1
boundary: Process log
item: child and probe lifecycle
promise: process-start and process-exit are logged for the claude child and for the claude agents --json and --version probes.
@row_add n=646 row=decision/parse-failures-surface-as-unknown area=obs-scope-summary rec=fct-u2gugwuk@1
name: Parse failures surface as unknown
rule: A parse failure surfaces as the value unknown plus a counter.
@row_add n=647 row=decision/controls-escaped-in-terminal-output area=obs-scope-summary rec=fct-u2gugwuk@1,fct-7eu65g47@1
name: Controls escaped in terminal output
rule: C0 and C1 control characters are escaped in any human terminal rendering of logged rows.
@row_add n=648 row=decision/tracing-subscriber-logger area=obs-scope-summary rec=fct-3og7ujbn@2
name: tracing-subscriber is the logger
rule: The logger dependency is tracing-subscriber at version 0.3.20 or later.
@row_add n=649 row=decision/obs-deps-no-c-building-crates area=obs-scope-summary rec=fct-3og7ujbn@2
name: Obs dependencies build no C
rule: Observability dependencies add no C-building crates.
@row_add n=650 row=decision/obs-deps-pass-cargo-deny area=obs-scope-summary rec=fct-3og7ujbn@2
name: Obs dependencies pass cargo deny
rule: Observability dependencies pass cargo deny on its one target, x86_64-unknown-linux-gnu.
guard: cargo deny
@row_add n=651 row=contract/hook-log-line/duration-ms area=obs-scope-summary rec=fct-dijwakce@1
boundary: Hook log line
item: duration_ms
promise: duration_ms is on hook-decision, or on process-exit for process hook, for each hook event.
@row_add n=652 row=decision/heartbeat-staleness-threshold area=obs-scope-summary rec=fct-dijwakce@1
name: Heartbeat staleness threshold
rule: A heartbeat flips to stale after more than 5 s: 4.9 s reads live, 5.1 s reads stale or gone.
@row_add n=653 row=decision/latency-aggregation-outside-process area=obs-scope-summary rec=fct-dijwakce@1
name: Latency aggregation outside the process
rule: Percentile aggregation is done by the harness or jq over the ndjson, never by an in-process metrics pipeline.
@row_add n=654 row=decision/fault-telemetry-per-class area=obs-scope-summary rec=fct-yfoyoh37@2
name: Fault telemetry per class
rule: Each fault class emits its own telemetry: a healed torn ndjson line with file and offset, a corrupt or unsupported snapshot recovered by replay, a child exit seen without EOF, an endpoint vanished during wait, and a stale heartbeat against a live pid.
@row_add n=655 row=decision/one-linux-log-sink area=obs-scope-summary rec=fct-7bdi4wod@2
name: One file sink on Linux
rule: One file sink and one line schema serve Linux, with no OS-specific exporter.
@row_add n=656 row=decision/log-append-atomicity area=obs-scope-summary rec=fct-7bdi4wod@2
name: Append atomicity for concurrent writers
rule: Concurrent hook writers append to a log atomically through POSIX O_APPEND.
@row_add n=657 row=decision/server-check-peer-credentials area=obs-scope-summary rec=fct-7bdi4wod@2
name: Server check by peer credentials
rule: The server check reads the peer pid by SO_PEERCRED and the peer euid; a mismatch of either reads server-verify-failed and an unreadable euid is refused.
@row_add n=658 row=decision/no-formal-slo area=obs-scope-summary rec=fct-rjubzxp3@1
name: No formal SLO
rule: No formal latency or availability SLO is held beyond the tests perf gates.
@row_add n=659 row=decision/no-log-rotation-v1 area=obs-scope-summary rec=fct-lmrmnkyn@1
name: No log rotation in v1
rule: events.ndjson and diagnostics/*.ndjson are not rotated in v1; any future rotation keeps 0600/0700 permissions and keeps wait after and send cursor offsets valid.
why: The trigger is operational, not compliance: the security tier is Minimal with no compliance triggers.
@row_add n=660 row=decision/structured-json-process-logs area=obs-scope-summary rec=fct-urji5is2@1
name: Structured JSON process logs
rule: Every process role (run, hook, mcp, cli) writes structured JSON process logs under diagnostics/, which harness logs merges with events.ndjson.
@row_add n=661 row=decision/tests-event-list-adopted area=obs-scope-summary rec=fct-urji5is2@1
name: Tests event list adopted
rule: The tests' field and event list is adopted verbatim, with send-issued and send-refused among the logged events.
@row_add n=662 row=contract/exit-21-detail/instance-dead area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 21 detail codes
item: instance-dead
promise: instance-dead is a distinct detail code for an exit-21 cause, naming no path or pid.
@row_add n=663 row=contract/exit-21-detail/unwrapped-name area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 21 detail codes
item: unwrapped-name
promise: unwrapped-name is a distinct detail code for an exit-21 cause, naming no path or pid.
@row_add n=664 row=contract/exit-21-detail/endpoint-lost area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 21 detail codes
item: endpoint-lost
promise: endpoint-lost is a distinct detail code for an exit-21 cause, naming no path or pid.
@row_add n=665 row=contract/exit-21-detail/strict-modes-failed area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 21 detail codes
item: strict-modes-failed
promise: strict-modes-failed is a distinct detail code for an exit-21 cause, naming no path or pid.
@row_add n=666 row=contract/exit-21-detail/server-verify-failed area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 21 detail codes
item: server-verify-failed
promise: server-verify-failed is a distinct detail code for an exit-21 cause, distinct from instance-dead, naming no path or pid.
@row_add n=667 row=contract/exit-21-detail/socket-dir-refused area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 21 detail codes
item: socket-dir-refused
promise: socket-dir-refused is a distinct detail code for an exit-21 cause, naming no path or pid.
@row_add n=668 row=contract/exit-1-detail/already-live area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: already-live
promise: already-live is a distinct detail code for an exit-1 cause, naming no path or pid.
@row_add n=669 row=contract/exit-1-detail/stale-heartbeat area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: stale-heartbeat
promise: stale-heartbeat is a distinct detail code for an exit-1 cause, naming no path or pid.
@row_add n=670 row=contract/exit-1-detail/squatted-name area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: squatted-name
promise: squatted-name is a distinct detail code for an exit-1 cause, naming no path or pid.
@row_add n=671 row=contract/exit-1-detail/pinned-hash-mismatch area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: pinned-hash-mismatch
promise: pinned-hash-mismatch is a distinct detail code for an exit-1 cause, naming no path or pid.
@row_add n=672 row=contract/exit-1-detail/socket-dir-refused area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: socket-dir-refused
promise: socket-dir-refused is a distinct detail code for an exit-1 cause, naming no path or pid.
@row_add n=673 row=contract/exit-1-detail/no-session area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: no-session
promise: no-session is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
@row_add n=674 row=contract/exit-1-detail/session-live area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: session-live
promise: session-live is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
@row_add n=675 row=contract/exit-1-detail/cwd-missing area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: cwd-missing
promise: cwd-missing is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
@row_add n=676 row=contract/viola-revive/strict-modes-failed area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: viola revive output
item: strict-modes-failed
promise: A strict-mode fail of viola revive writes no log line; it prints the stderr pair, or under --json {"v":1,"error":"strict-modes-failed","detail":null}, and exits 1.
@row_add n=677 row=contract/viola-link-unlink/strict-modes-failed area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: viola link and unlink output
item: strict-modes-failed
promise: A strict-mode fail of the driven instance's check writes no log line; it prints the fixed instance-state unable and hint: pair on stderr, or under --json {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, and exits 21.
@row_add n=678 row=decision/socket-dir-one-code area=obs-scope-summary rec=fct-n7xftqkr@2
name: One code for socket-directory causes
rule: A refused check, a failed create or a lost race on the socket directory all read the one code socket-dir-refused.
@row_add n=679 row=contract/refusal-reason/unknown-fallback area=obs-scope-summary rec=fct-plg35lpj@1
boundary: Refusal reason
item: unknown fallback
promise: An unrecognised RefusalReason falls back to unknown with exit 14.
@row_add n=680 row=contract/channel-response/invalid-params-causes area=obs-scope-summary rec=fct-plg35lpj@1
boundary: Channel response line
item: invalid-params causes
promise: detail invalid-params covers send's invalid params, and a link or unlink whose driver or driven is missing, not a string or no valid name, whose driven is not the serving wrapper's own name, or whose driver equals driven.
@row_add n=681 row=decision/version-rejections-counted-by-detail area=obs-scope-summary rec=fct-plg35lpj@1
name: Version rejections counted by detail
rule: Higher-v rejections are counted by the detail unsupported-version, not by the -32602 code.
why: The -32602 code also carries invalid-params and release-from-driver.
@row_add n=682 row=decision/per-parser-parse-failure-counts area=obs-scope-summary rec=fct-plg35lpj@1
name: Per-parser parse-failure counts
rule: Each parser (paste validation, channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, vt100 under catch_unwind) keeps its own parse-failure count.
@row_add n=683 row=decision/deterministic-event-emission area=obs-scope-summary rec=fct-jz5zhprx@1
name: Deterministic event emission
rule: Events are emitted deterministically, keyed on hook events and events.ndjson offsets.
@row_add n=684 row=decision/no-telemetry-from-screen area=obs-scope-summary rec=fct-jz5zhprx@1
name: No telemetry from screen content
rule: No telemetry is derived from screen content; vt100 serves gate outcomes only.
@row_add n=685 row=decision/cookie-header-never-logged area=pii-scrubbing-compliance rec=fct-cthr64ip@1
name: Cookie header never logged
rule: A Cookie header, in its case-insensitive form, is classed Critical and never logged in any diagnostics/ file.
guard: secret-scan test
@row_add n=686 row=decision/signing-credentials-never-logged area=pii-scrubbing-compliance rec=fct-j3d6gqtu@1
name: Signing credentials never logged
rule: v1.x signing credentials are classed Critical and never logged, and CI secrets are never echoed into artifacts.
@row_add n=687 row=decision/prompt-and-output-never-in-home-logs area=pii-scrubbing-compliance rec=fct-a6wgb4sc@1,fct-cetbw4yv@1
name: Prompt and assistant output never in home logs
rule: Prompt text and assistant output, last_assistant_message included, never appear in a home-level process-log line; they reach an instance detail file only inside a chain or drift report, and only text_bytes is logged.
@row_add n=688 row=decision/veil-redaction area=pii-scrubbing-compliance rec=fct-a6wgb4sc@1,fct-jv26h5pj@1
name: veil derives redaction
rule: veil 0.3.0's derived Redact is what redacts the payload fields of contract types, and instrumented spans skip all arguments.
@row_add n=689 row=decision/endpoint-paths-never-logged area=pii-scrubbing-compliance rec=fct-5jqyxpdr@1
name: Endpoint and pinned binary never logged
rule: The snapshot endpoint and pinned_bin are never logged, only endpoint_kind.
@row_add n=690 row=decision/typed-ids-never-in-detail area=pii-scrubbing-compliance rec=fct-24jsq3a5@1
name: Ids logged as typed fields
rule: pid, child_pid, agent_session_id, started_at, budget percentages and resets_at are logged as typed fields, never inside detail codes.
@row_add n=691 row=decision/agent-rows-never-verbatim area=pii-scrubbing-compliance rec=fct-7eu65g47@1
name: Agent rows never logged verbatim
rule: claude agents --json rows are untrusted and never logged verbatim, only parse-rejected counts.
@row_add n=692 row=decision/link-names-never-in-process-log area=pii-scrubbing-compliance rec=fct-wjbkfws5@1
name: Link pair names never in a process log
rule: link and unlink driver and driven names appear only in the wrapper's link and unlink event lines in events.ndjson, never in a process-log line; the channel pair logs the method, never the params.
guard: link_verbs_log_the_channel_pair_and_no_names
@row_add n=693 row=decision/closed-protocol-codes area=pii-scrubbing-compliance rec=fct-exgbnztg@1
name: Closed protocol codes
rule: Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value.
guard: answer_invalid_params_names_its_cause_and_no_param
@row_add n=694 row=decision/fixtures-not-a-telemetry-source area=pii-scrubbing-compliance rec=fct-rp7rj7wv@1
name: Fixtures not a telemetry source
rule: Recorded fixtures are not a telemetry source, and observability adds no fixture content.
@row_add n=695 row=decision/canary-only-in-detail-files area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Canary only in detail files
rule: Every E2E and integration input is synthetic and embeds a fixed canary, which may appear in detail files but never in a home-level diagnostics file.
guard: mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line
@row_add n=696 row=decision/detail-upload-needs-synthetic-input area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Detail upload needs synthetic input
rule: Detail files and harness captures are uploaded only while every input is synthetic; a non-synthetic input source must first drop detail files and the harness capture from the upload paths.
@row_add n=697 row=decision/mutation-diff-never-uploaded area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Mutation diff never uploaded
rule: The mutation run's chunk.diff at the harness capture root is excluded from the harness upload, so it is never scanned and never uploaded.
why: The diff is repository source text by construction and hits the scan's Critical-class patterns.
@row_add n=698 row=decision/pre-push-document-path-free area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Pre-push document path-free
rule: The local pre-push gate document is never uploaded and carries no absolute path, only codes and counts.
guard: pre_push_document_carries_no_absolute_path
@row_add n=699 row=decision/server-verify-refusal-names-nothing area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Server-verify refusal names nothing
rule: A server-verification refusal carries no pid, start time, path or endpoint in any output or log line.
guard: security_negatives_server_identity
@row_add n=700 row=decision/socket-dir-refusal-names-nothing area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Socket-directory refusal names nothing
rule: A socket-directory refusal carries no path, pid or uid in its lines and documents, and its human pair names only the parsed instance name.
guard: security_negatives_socket_dir
@row_add n=701 row=decision/error-bodies-clean area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Clean error bodies
rule: Error bodies of CLI --json, MCP results and the channel faults hold no absolute path, upstream caused-by text or serde path, and every product error writer is named.
guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
@row_add n=702 row=decision/error-messages-interpolate-nothing area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Error messages interpolate nothing
rule: thiserror messages interpolate no value.
guard: thiserror_messages_interpolate_nothing
@row_add n=703 row=decision/content-sink-behind-strict-check area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Content sink behind a strict-modes check
rule: The diagnostics content sink opens only after a strict-modes check of the folder and file; a symlink or a folder or file another user may read refuses the open, the writer becomes a sink and the line is dropped.
why: High-class content is never written through a symlink or into a folder another user may read.
@row_add n=704 row=decision/unscanned-uploads-admissible-by-content area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Unscanned uploads admissible by content
rule: An artifact uploaded without a scan must carry no host path or log content; a member that could carry one is scanned before upload or dropped.
why: Ratified.
@row_add n=705 row=decision/mutants-out-never-uploaded area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Mutation outputs never uploaded
rule: mutants.out/ and the per-run archive of JUnit and outcomes files are never uploaded.
why: The outcomes file held absolute argv paths and the log folder held test output.
@row_add n=706 row=decision/fuzz-artifacts-need-synthetic-corpus area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Fuzz artifacts need a synthetic corpus
rule: Fuzz crash artifacts upload only while the committed fuzz corpus is synthetic; a non-synthetic corpus seed must first drop that upload.
@row_add n=707 row=decision/scan-failure-blocks-upload area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Scan failure blocks upload
rule: When the secret scan fails no diagnostics, harness or perf artifact uploads; a hit report with file, line, byte offset and pattern class, never the matched bytes, uploads instead.
why: The failing job still leaves agent-readable evidence.
@row_add n=708 row=decision/panic-count-home-level-only area=slo-invariants-telemetry-budgets rec=fct-6543go25@1
name: Panic count reads home-level files only
rule: The exactly-one panic rule, the viola.panic counter and gate G2 count home-level role files only; the companion panic line in a detail file is excluded.
guard: gate G2
except: G2 alone leaves out the fake-agent seam's deliberate panic, whose line is still written exactly once.
@row_add n=709 row=decision/no-retry-on-panic area=slo-invariants-telemetry-budgets rec=fct-6543go25@1
name: No retry on panic
rule: Retry-once policies for panic handling are forbidden.
why: Parallel to the tests' retries = 0.
@row_add n=710 row=decision/session-end-perf-gate area=slo-invariants-telemetry-budgets rec=fct-h65wem67@1,fct-dijwakce@1
name: Session-end hook perf gate
rule: viola hook session-end must keep its hyperfine max under 1.0 s; p50 and p95 are report-only.
guard: gate --require perf
@row_add n=711 row=decision/spine-hook-perf-gate area=slo-invariants-telemetry-budgets rec=fct-4od5l5b2@2,fct-dijwakce@1
name: Spine hook perf gate
rule: Spine hooks (session-start, user-prompt-submit, stop) and pre-tool-use on an unverified CLI keep their hyperfine max under the spine deadline, timed over every check a frame-writing hook makes before its first frame.
guard: gate --require perf
@row_add n=712 row=contract/hook-decision/channel-unreachable area=slo-invariants-telemetry-budgets rec=fct-4od5l5b2@2
boundary: Hook decision line
item: channel-unreachable
promise: A hook whose snapshot read is replayed or absent fails open as channel-unreachable.
@row_add n=713 row=decision/heartbeat-staleness-flip area=slo-invariants-telemetry-budgets rec=fct-bpjqjqbh@1
name: Heartbeat staleness flip
rule: An instance whose heartbeat is 4.9 s old reads live, and one whose heartbeat is 5.1 s old reads stale or gone, as reported in the liveness field of list --json.
@row_add n=714 row=decision/hook-benchmark-on-passing-instance area=slo-invariants-telemetry-budgets rec=fct-bxbg45yr@1
name: Hook benchmark runs against a passing instance
rule: Every hyperfine hook command runs with VIOLA_NAME set, a home under target/e2e-home/ and an instance directory that exists and passes the strict-modes check.
why: Without a name, or with an absent or refused instance, the hook times its no-op path and the gate passes vacuously.
@row_add n=715 row=decision/perf-job-runs-g2 area=slo-invariants-telemetry-budgets rec=fct-bxbg45yr@1
name: Perf job runs G2 over the benchmark home
rule: The perf job runs the G2 check over the hook benchmark home.
why: A hook panic exits 0 and hyperfine cannot see it.
@row_add n=716 row=contract/span-kinds/client area=span-trace-coverage rec=fct-swp2fwp2@1
boundary: Span kinds
item: CLIENT
promise: The CLIENT spans are send.client, wait.block, last.client, answer.client, pause.client, release.client, cli.link, cli.unlink, channel.request (client side), statusline.shell_out and pty.spawn.
@row_add n=717 row=contract/span-kinds/server area=span-trace-coverage rec=fct-walqnfde@1
boundary: Span kinds
item: SERVER
promise: The SERVER spans are channel.dispatch (the wrapper side of every channel.request), mcp.tool_call, hook.handle and hook.statusline.
@row_add n=718 row=contract/span-kinds/internal area=span-trace-coverage rec=fct-4t76aguz@1
boundary: Span kinds
item: INTERNAL
promise: The INTERNAL spans are run.*, state.*, hook.decision_emit and pty.paste_write.
@row_add n=719 row=decision/cli-manual-dispatch-span area=span-trace-coverage rec=fct-wac6zl42@1
name: Manual dispatch span per cli verb
rule: Each short-lived cli verb opens one manual dispatch span of its own.
why: clap 4.6.7 has no tracing integration.
@row_add n=720 row=decision/cli-log-only-under-viola-name area=span-trace-coverage rec=fct-wac6zl42@1
name: cli process log only under a parsed name
rule: A short-lived cli verb opens its process log and writes process-start and process-exit only when an instance resolves under a parsed VIOLA_NAME; with no name it writes no log.
@row_add n=721 row=decision/refused-check-writes-no-line area=span-trace-coverage rec=fct-wac6zl42@1,fct-espo2tl3@1
name: A refused check opens no process log
rule: A cli verb whose strict-modes, tree or instance check refuses (exit 21) opens no process log and writes no line.
@row_add n=722 row=contract/revive-list-process-exit/no-session area=span-trace-coverage rec=fct-wac6zl42@1
boundary: viola revive --list process log
item: no-session
promise: An empty session chain ends process-exit with exit_code 1 and detail no-session; any other listing ends exit_code 0 with no detail.
@row_add n=723 row=decision/revive-start-logs-as-run area=span-trace-coverage rec=fct-wac6zl42@1
name: viola revive start logs as run
rule: The start arm of viola revive is not a cli verb and logs as process run once its instance check has passed.
@row_add n=724 row=decision/error-chain-in-detail-log-only area=span-trace-coverage rec=fct-wac6zl42@1
name: Full error chain only in the cli detail log
rule: The full anyhow error chain of a cli verb goes to the cli detail log only.
@row_add n=725 row=decision/portable-pty-log-target-off area=span-trace-coverage rec=fct-xrkfnxjv@2
name: portable-pty log target OFF
rule: The log target of portable-pty is OFF in the sink, and PTY failures are captured at the pty seam instead.
@row_add n=726 row=decision/manual-readiness-gate-span area=span-trace-coverage rec=fct-xrkfnxjv@2
name: Manual span on the readiness gate
rule: The readiness gate of viola run carries a manual span, run.readiness_gate.
why: vt100 has no tracing.
@row_add n=727 row=contract/claude-child-process-log/handle-wait-thread area=span-trace-coverage rec=fct-xrkfnxjv@2,fct-mw7zqeom@2
boundary: run process log, claude-child lines
item: process-start and process-exit of claude-child
promise: Both are emitted from the handle-wait thread; process-exit{subject:"claude-child"} carries child_exit_status and exit_source handle-wait or kill-fallback, and is never emitted on reader EOF.
@row_add n=728 row=contract/hook-spans/span-chain area=span-trace-coverage rec=fct-kreejnxq@1
boundary: viola hook spans
item: span chain
promise: viola hook traces hook.handle › channel.request › hook.decision_emit.
@row_add n=729 row=decision/hook-always-exits-zero area=span-trace-coverage rec=fct-kreejnxq@1
name: viola hook keeps exit 0
rule: viola hook exits 0 even when its dispatch panics: a panic around dispatch is caught.
@row_add n=730 row=decision/hook-refused-instance-silent area=span-trace-coverage rec=fct-kreejnxq@1
name: Refused instance makes the hook silent
rule: On the event and dialog paths the instance check runs before the diagnostics level and log init; a refused or unresolved instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append.
@row_add n=731 row=decision/hook-capture-arms-unlogged area=span-trace-coverage rec=fct-kreejnxq@1
name: Hidden capture arms write no lines
rule: The hidden capture arms of viola hook (hook <event> --capture and hook statusline --capture) initialise no observability and write no lines.
@row_add n=732 row=contract/channel-lines/fields area=span-trace-coverage rec=fct-pt4swoyy@1
boundary: channel-request and channel-response lines
item: fields
promise: Both lines carry method, corr, conn, result_class, error_code and sender, plus detail on a -32602.
@row_add n=733 row=decision/no-params-in-channel-lines area=span-trace-coverage rec=fct-pt4swoyy@1
name: No params or result bodies in channel lines
rule: channel-request and channel-response lines never carry params or result bodies.
@row_add n=734 row=decision/accept-drops-foreign-euid area=span-trace-coverage rec=fct-6fdjkv6t@1
name: Unadmitted peers dropped at accept
rule: The wrapper drops an accepted stream whose peer euid is not admitted on the accept thread, before any worker thread or frame, with no span and no line; the peer reads end-of-file and the wheel does not move.
@row_add n=735 row=decision/rmcp-tracing-target-off area=span-trace-coverage rec=fct-felgh2fq@1
name: rmcp tracing target OFF
rule: The native tracing target of rmcp is OFF in the sink.
why: Its records lack required fields and can carry frame content.
@row_add n=736 row=contract/mcp-spans/mcp-tool-call area=span-trace-coverage rec=fct-felgh2fq@1
boundary: viola mcp spans
item: mcp.tool_call
promise: viola mcp opens one mcp.tool_call span per tool (send, wait, last, answer, list); rejects log codes-only send-refused or channel-response lines.
@row_add n=737 row=decision/mcp-explicit-log-writer area=span-trace-coverage rec=fct-felgh2fq@1
name: Explicit log writer in viola mcp
rule: The viola mcp log writer is always explicit: a file, with stderr only as a fallback.
@row_add n=738 row=decision/verify-server-before-frame area=span-trace-coverage rec=fct-2tnvi4o3@2
name: Server verified before any frame
rule: Each frame verb, after its own client-side refusals and before any frame, checks the socket directory, connects, decides on the peer euid, then compares the peer's pid and OS start time against the one snapshot read.
@row_add n=739 row=contract/cli-exit-21/server-verify-failed area=span-trace-coverage rec=fct-2tnvi4o3@2,fct-u2zbdxyd@2
boundary: cli exit 21 process-exit
item: server-verify-failed
promise: A refused euid, a server that is not the recorded wrapper, or a check that cannot be made opens no channel.request span, writes no channel-request line and ends process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}.
@row_add n=740 row=decision/peer-euid-must-match area=span-trace-coverage rec=fct-jvpve25b@2
name: Server euid must match the client's
rule: Every client seam refuses a server of another euid, or one whose euid cannot be read, after the connect and before the pid comparison; reading the euid writes and logs nothing.
@row_add n=741 row=decision/server-refusal-inside-root-span area=span-trace-coverage rec=fct-7w75bmha@1
name: Server-verification refusal inside the root span
rule: A frame verb's or the hook's server-verification refusal happens inside its scenario's root span.
guard: request_refuses_an_unverified_server_inside_its_dispatch_span; hook_refuses_an_unverified_server_inside_hook_handle
@row_add n=742 row=decision/socket-dir-refusal-inside-root-span area=span-trace-coverage rec=fct-vivmhmma@2
name: Socket-directory refusal inside the root span
rule: A frame verb's or the hook's socket-directory refusal happens inside its dispatch span or hook span.
guard: connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span
@row_add n=743 row=decision/instance-check-before-role-file area=span-trace-coverage rec=fct-espo2tl3@1
name: Instance check before any role file
rule: The frame verbs run the instance check on their named instance (the driven instance for link and unlink) before any role file opens.
@row_add n=744 row=contract/instance-unreachable-json/strict-modes-failed area=span-trace-coverage rec=fct-espo2tl3@1,fct-hluu43pb@2
boundary: cli --json refusal document
item: strict-modes-failed
promise: A refused instance check exits 21 with the instance pair on stderr, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, and sends no frame.
@row_add n=745 row=contract/cli-exit-21/socket-dir-refused area=span-trace-coverage rec=fct-zklcva6d@1
boundary: cli exit 21 process-exit
item: socket-dir-refused
promise: A frame verb meeting a refused socket directory writes one process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"} at WARN inside its dispatch span, with no channel-request line and no frame.
@row_add n=746 row=decision/socket-dir-check-changes-no-mode area=span-trace-coverage rec=fct-zklcva6d@1
name: Socket-directory check repairs nothing
rule: The socket-directory check on the recorded endpoint's parent changes no mode: a refused directory stays as found.
@row_add n=747 row=contract/refusal-stderr-pair/socket-dir-refused area=span-trace-coverage rec=fct-ucfyu5c3@1
boundary: cli refusal stderr pair
item: socket-directory refusal
promise: Printed on start and frame-verb refusals in plain ASCII with hint: last and no SGR, naming only the parsed ViolaName.
@row_add n=748 row=decision/replayed-snapshot-unreachable area=span-trace-coverage rec=fct-kohas2np@2
name: Replayed snapshot is unreachable
rule: Only a snapshot read whole leads to a connect; a replayed or absent snapshot is treated as unreachable and no frame is sent.
why: A replayed snapshot carries no endpoint, pid or started_at.
@row_add n=749 row=contract/mcp-instance-unreachable/strict-modes-failed area=span-trace-coverage rec=fct-ts6ky5ry@2
boundary: viola mcp frame tool refusal
item: strict-modes-failed
promise: A refused instance check, run before the live endpoint is read, returns instance-unreachable with detail strict-modes-failed, no frame and no line.
@row_add n=750 row=contract/mcp-instance-unreachable/socket-dir-refused area=span-trace-coverage rec=fct-ts6ky5ry@2
boundary: viola mcp frame tool refusal
item: socket-dir-refused
promise: A refused socket directory returns instance-unreachable with detail socket-dir-refused, no frame and no line.
@row_add n=751 row=contract/mcp-instance-unreachable/server-verify-failed area=span-trace-coverage rec=fct-ts6ky5ry@2
boundary: viola mcp frame tool refusal
item: server-verify-failed
promise: A refused euid, a mismatch or a check that cannot be made returns the codes-only isError instance-unreachable with detail server-verify-failed, no frame and no log line.
@row_none n=168 rec=fct-4gke2rkl@1 type=4
relied: The CLI and MCP clients screen a peer reply to code words and a sanitised fault before using it, adding no span and no line.
@row_add n=752 row=contract/run-start-spans/span-chain area=span-trace-coverage rec=fct-e6vtcesf@2
boundary: viola run start spans
item: span chain
promise: run.start (INTERNAL, root) › state.snapshot_recover › run.collision_check › run.pin_copy › run.version_gate › channel.bind › run.budget_eval (trigger start) › state.snapshot_write › state.heartbeat_start › pty.spawn (CLIENT).
@row_add n=753 row=decision/statusline-settings-steps-unlogged area=span-trace-coverage rec=fct-e6vtcesf@2
name: Statusline read and settings write unlogged
rule: The start's statusline source read and settings override write carry no span and no log line.
why: Ratified as by design.
@row_add n=754 row=decision/no-statusline-settings-in-telemetry area=span-trace-coverage rec=fct-e6vtcesf@2,fct-6peizmat@1
name: No settings or statusline command in telemetry
rule: The child's --settings argument, the statusline source path and the statusline command appear in no log line or span.
@row_add n=755 row=contract/run-start-failure-detail/socket-dir-refused area=span-trace-coverage rec=fct-e6vtcesf@2,fct-4g2at3oh@2
boundary: run start process-exit
item: socket-dir-refused
promise: A socket directory the start refuses is one process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"} before any snapshot, child or bind, on viola revive's start too; a bind failure keeps its own outcome.
@row_add n=756 row=contract/run-span-attributes/run-start area=span-trace-coverage rec=fct-gdmzm6bw@1
boundary: viola run start span attributes
item: run.start
promise: run.start carries instance.
@row_add n=757 row=contract/run-span-attributes/run-collision-check area=span-trace-coverage rec=fct-gdmzm6bw@1
boundary: viola run start span attributes
item: run.collision_check
promise: outcome is free, already-live or squatted-name; a live or stale holder is already-live, a gone one free; over a replayed snapshot a beat no older than STALE_AFTER is already-live, an older one free; an absent snapshot or failed read is free.
@row_add n=758 row=contract/run-span-attributes/run-pin-copy area=span-trace-coverage rec=fct-gdmzm6bw@1,fct-i4d4tneg@1
boundary: viola run start span attributes
item: run.pin_copy
promise: outcome is ok or pinned-hash-mismatch, and the set stays those two.
@row_add n=759 row=contract/run-span-attributes/run-version-gate area=span-trace-coverage rec=fct-gdmzm6bw@1
boundary: viola run start span attributes
item: run.version_gate
promise: run.version_gate carries cli_version and cli_verified.
@row_add n=760 row=contract/run-span-attributes/channel-bind area=span-trace-coverage rec=fct-gdmzm6bw@1
boundary: viola run start span attributes
item: channel.bind
promise: channel.bind carries endpoint_kind, unix-socket.
@row_add n=761 row=contract/run-span-attributes/state-snapshot-write area=span-trace-coverage rec=fct-gdmzm6bw@1
boundary: viola run start span attributes
item: state.snapshot_write
promise: state.snapshot_write carries v.
@row_add n=762 row=contract/run-span-attributes/pty-spawn area=span-trace-coverage rec=fct-gdmzm6bw@1
boundary: viola run start span attributes
item: pty.spawn
promise: pty.spawn carries pty_backend (openpty) and env_stripped_count.
@row_add n=763 row=decision/no-paths-in-run-spans area=span-trace-coverage rec=fct-gdmzm6bw@1
name: No endpoint, path or env values on start spans
rule: Start spans never carry the endpoint name, the pinned binary path or environment values.
@row_add n=764 row=contract/run-process-log/common-fields area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run process log
item: common fields
promise: Every run line carries the common fields with process="run" and corr null.
@row_add n=765 row=contract/run-process-log/process-start-self area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run process log
item: process-start self
promise: process-start{subject:"self", service_name, version, os, pid, endpoint_kind}.
@row_add n=766 row=contract/run-process-log/version-probe area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run process log
item: version-probe
promise: process-start and process-exit{subject:"version-probe", child_exit_status, duration_ms}.
@row_add n=767 row=contract/run-start-failure-detail/already-live area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run start process-exit
item: already-live
promise: process-exit{subject:"self", exit_code:1, detail:"already-live"} for a live holder or a fresh replayed beat, with no path or pid.
@row_add n=768 row=contract/run-start-failure-detail/stale-heartbeat area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run start process-exit
item: stale-heartbeat
promise: process-exit{subject:"self", exit_code:1, detail:"stale-heartbeat"} for a stale holder, with no path or pid.
@row_add n=769 row=contract/run-start-failure-detail/squatted-name area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run start process-exit
item: squatted-name
promise: process-exit{subject:"self", exit_code:1, detail:"squatted-name"}, with no path or pid.
@row_add n=770 row=contract/run-start-failure-detail/pinned-hash-mismatch area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run start process-exit
item: pinned-hash-mismatch
promise: process-exit{subject:"self", exit_code:1, detail:"pinned-hash-mismatch"}, with no path or pid.
@row_add n=771 row=decision/home-tree-check-before-start-log area=span-trace-coverage rec=fct-4g2at3oh@2
name: Home-tree check before the start's log
rule: The start of run and of viola revive checks the home tree (the home, instance set, ledger, bin, plugin, pinned exe and plugin hooks) before its log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
@row_add n=772 row=contract/revive-preflight-detail/no-session area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: viola revive preflight process-exit
item: no-session
promise: A preflight refusal adds detail no-session before any child is started.
@row_add n=773 row=contract/revive-preflight-detail/session-live area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: viola revive preflight process-exit
item: session-live
promise: A preflight refusal adds detail session-live before any child is started; the reading is skipped under --fork and passes on a failed read.
@row_add n=774 row=contract/revive-preflight-detail/cwd-missing area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: viola revive preflight process-exit
item: cwd-missing
promise: A preflight refusal adds detail cwd-missing before any child is started.
@row_add n=775 row=contract/revive-process-log/agents-probe area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: viola revive run process log
item: agents-probe
promise: The session-live reading writes one process-start and process-exit{subject:"agents-probe"} pair, plus one parse-rejected{parser:"claude-agents-json"} on an oversize or malformed answer.
@row_add n=776 row=contract/run-process-log/process-start-claude-child area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run process log
item: process-start claude-child
promise: On success, process-start{subject:"claude-child", child_pid, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept}.
@row_add n=777 row=contract/diag-line-schema/pty-backend area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: diag-line v1 schema
item: pty_backend
promise: pty_backend is any string.
@row_add n=778 row=contract/diag-line-schema/os area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: diag-line v1 schema
item: os
promise: os is closed to linux.
@row_add n=779 row=decision/product-order-from-events-log area=span-trace-coverage rec=fct-4g2at3oh@2
name: Start product order read from the events log
rule: The product order wheel{cause:"start"} then budget-gate then session-start{source:"hook"} is verified from the instance's events log, not duplicated into process logs.
@row_add n=780 row=contract/run-start-spans/run-start-close area=span-trace-coverage rec=fct-mw7zqeom@2
boundary: viola run start spans
item: run.start close
promise: run.start closes when pty.spawn returns.
@row_add n=781 row=contract/run-process-log/process-exit-self area=span-trace-coverage rec=fct-mw7zqeom@2
boundary: run process log
item: process-exit self
promise: After the child's process-exit, run emits process-exit{subject:"self", exit_code, duration_ms}.
@row_add n=782 row=decision/pinned-copy-never-replaced area=span-trace-coverage rec=fct-i4d4tneg@1
name: Pinned copy placed, never replaced
rule: An absent pinned copy is placed, never replaced; a start that loses the placement re-hashes the copy that stands, reusing an equal one and refusing a different one as pinned-hash-mismatch with the file left as found.
@row_add n=783 row=contract/confirmed-send-spans/span-chain area=span-trace-coverage rec=fct-lohdixpq@2
boundary: confirmed send spans
item: span chain
promise: send.client (CLIENT) › channel.request (CLIENT) → channel.dispatch (SERVER) › run.readiness_gate › pty.paste_write › run.confirm_window; an unconfirmable send ends at pty.paste_write. In the hook: hook.handle › channel.request(hook.event).
@row_add n=784 row=decision/implicit-link-on-send-and-answer area=span-trace-coverage rec=fct-lohdixpq@2,fct-46kjozmt@1
name: Implicit link recorded by send and answer
rule: An issued send or a taken answer whose from names an instance other than the wrapper records the link, one link product line for a new pair; a refused call, one with no from, or one under the wrapper's own name records nothing.
@row_add n=785 row=contract/events-log/link-before-send-issued area=span-trace-coverage rec=fct-lohdixpq@2
boundary: events log
item: implicit link and send-issued
promise: A new pair's link product line stands directly before that send's send-issued, whose cursor is the offset its own line starts at; a link record that fails answers -32603 with nothing issued, nothing typed and no send-issued.
@row_add n=786 row=decision/link-names-not-in-process-log area=span-trace-coverage rec=fct-lohdixpq@2,fct-lnbq3lnm@1
name: Link names stay out of process logs
rule: The driver and driven names reach no process-log line.
@row_add n=787 row=contract/no-endpoint-instance/classification area=span-trace-coverage rec=fct-lohdixpq@2
boundary: no-endpoint instance reading
item: instance-dead or unwrapped-name
promise: An instance with no endpoint whose path is a directory reads instance-dead; anything else (absent, a regular file, a symlink) reads unwrapped-name; the reading writes no line and the instance check stays first.
@row_add n=788 row=contract/send-span-attributes/run-readiness-gate area=span-trace-coverage rec=fct-xfymn57j@1
boundary: confirmed send span attributes
item: run.readiness_gate
promise: outcome (ready or input-not-ready) and vt100_panicked (bool).
@row_add n=789 row=contract/send-span-attributes/text-bytes area=span-trace-coverage rec=fct-xfymn57j@1,fct-gghzntp4@2
boundary: confirmed send span attributes
item: text_bytes
promise: text_bytes, on pty.paste_write and send-issued alike, is the typed text's length: the sent text with each CR LF pair and each other CR as one LF, trailing CR and LF removed; pty.paste_write also carries paste_mode="bracketed".
@row_add n=790 row=contract/send-span-attributes/window-ms area=span-trace-coverage rec=fct-xfymn57j@1
boundary: confirmed send span attributes
item: run.confirm_window window_ms
promise: window_ms is the bound the send's own deadline reads: 40 000 for a /compact send waiting for its compacted-session post-condition, else 10 000.
@row_add n=791 row=decision/no-text-in-telemetry area=span-trace-coverage rec=fct-xfymn57j@1,fct-gghzntp4@2
name: Sent text never in telemetry
rule: The sent text never appears in a span or log line; only its length does.
@row_add n=792 row=contract/send-log-fields/channel-request area=span-trace-coverage rec=fct-gghzntp4@2
boundary: confirmed send log fields
item: channel-request
promise: channel-request{corr:<id>, conn, method, from, from_trust:"self-reported", sender}.
@row_add n=793 row=contract/send-log-fields/send-issued area=span-trace-coverage rec=fct-gghzntp4@2
boundary: confirmed send log fields
item: send-issued
promise: send-issued{corr:<cursor>, from, text_bytes}.
@row_add n=794 row=contract/send-log-fields/hook-invoked area=span-trace-coverage rec=fct-gghzntp4@2
boundary: confirmed send log fields
item: hook-invoked
promise: hook-invoked{hook_event:"user-prompt-submit", corr:null} for an ordinary send; hook_event session-start for a /clear or /compact confirmed by its session; an unconfirmable send waits for no hook line.
@row_add n=795 row=contract/send-log-fields/send-confirmed area=span-trace-coverage rec=fct-gghzntp4@2
boundary: confirmed send log fields
item: send-confirmed
promise: send-confirmed{corr:<cursor>, confirmed, duration_ms}, confirmed false for unconfirmable, duration_ms from the send's arrival including the readiness gate's wait.
@row_add n=796 row=contract/send-refused-detail/input-not-ready area=span-trace-coverage rec=fct-gghzntp4@2
boundary: wrapper send-refused detail
item: input-not-ready
promise: send-refused{corr:<cursor>, refusal, detail:"input-not-ready", side:"wrapper", wheel}.
@row_add n=797 row=contract/send-refused-detail/no-prompt-submitted area=span-trace-coverage rec=fct-gghzntp4@2,fct-u2zbdxyd@2
boundary: wrapper send-refused detail
item: no-prompt-submitted
promise: Confirm-window expiry is send-refused{detail:"no-prompt-submitted"} for ordinary, /clear and /compact sends alike, with no new detail.
@row_add n=798 row=contract/send-refused-detail/turn-running area=span-trace-coverage rec=fct-gghzntp4@2
boundary: wrapper send-refused detail
item: turn-running
promise: Covers a running turn or another send in flight, with no new field; the refusal has no cursor (corr the end offset at refusal), no send-issued, and the client exits 13.
@row_add n=799 row=contract/send-refused-client/control-character area=span-trace-coverage rec=fct-gghzntp4@2
boundary: client-side send validation refusal
item: control-character
promise: Made before any frame: send-refused{corr:null, refusal:"not-delivered", detail:"control-character", side:"client"}, one line per deciding side, never the text.
@row_add n=800 row=contract/send-refused-client/empty-text area=span-trace-coverage rec=fct-gghzntp4@2
boundary: client-side send validation refusal
item: empty-text
promise: An empty typed text, read after control-character and ahead of both wheel reads; a wrapper deciding it writes its own send-refused{side:"wrapper"} with the same pair, corr, conn and rpc_id, and no send-issued.
@row_add n=801 row=decision/send-line-correlation area=span-trace-coverage rec=fct-gghzntp4@2
name: Send lines correlate with channel lines
rule: Every wrapper send-* line carries conn (or srv_conn) and rpc_id, so that pair on the send line equals (conn, corr) on the call's channel lines; the client-side send-refused carries neither.
@row_add n=802 row=contract/confirm-window-close/ordinary-send area=span-trace-coverage rec=fct-u2zbdxyd@2
boundary: run.confirm_window close
item: ordinary send
promise: Closes on the matching prompt-submitted{origin:"driver"}, or on window expiry.
@row_add n=803 row=contract/confirm-window-close/clear area=span-trace-coverage rec=fct-u2zbdxyd@2
boundary: run.confirm_window close
item: /clear
promise: On a verified CLI, closes on the session-start whose cause is clear and whose agent_session_id differs from the id the send slot remembers; no prompt-submitted claims the send.
@row_add n=804 row=contract/confirm-window-close/compact area=span-trace-coverage rec=fct-u2zbdxyd@2
boundary: run.confirm_window close
item: /compact
promise: On a verified CLI, closes on the session-start whose cause is compact and whose agent_session_id equals the id the send slot remembered; nothing remembered claims nothing, and no prompt-submitted claims the send.
@row_add n=805 row=contract/cli-exit/1-internal-error area=span-trace-coverage rec=fct-u2zbdxyd@2
boundary: send.client process-exit
item: 1 internal-error
promise: exit_code 1 internal-error on a reply with neither a parseable result nor an error.code.
@row_add n=806 row=contract/cli-exit/20-wrapper-fault area=span-trace-coverage rec=fct-u2zbdxyd@2
boundary: send.client process-exit
item: 20 wrapper-fault
promise: Exit 20 carries detail wrapper-fault.
@row_add n=807 row=contract/cli-exit-21/instance-dead area=span-trace-coverage rec=fct-u2zbdxyd@2,fct-hluu43pb@2
boundary: cli exit 21 process-exit
item: instance-dead
promise: detail instance-dead with during connect: no live snapshot, a replayed one included, or the connect failed.
@row_add n=808 row=contract/cli-exit-21/unwrapped-name area=span-trace-coverage rec=fct-u2zbdxyd@2,fct-hluu43pb@2
boundary: cli exit 21 process-exit
item: unwrapped-name
promise: detail unwrapped-name with during connect: no instance directory.
@row_add n=809 row=contract/cli-exit-21/endpoint-lost area=span-trace-coverage rec=fct-u2zbdxyd@2,fct-hluu43pb@2
boundary: cli exit 21 process-exit
item: endpoint-lost
promise: detail endpoint-lost with during call: the connection was lost after the connect; it is the only exit-21 detail whose during is call.
@row_add n=810 row=contract/wait-last-spans/span-chain area=span-trace-coverage rec=fct-l4ierws2@2
boundary: wait and last spans
item: span chain
promise: wait.block (CLIENT) › channel.request → channel.dispatch › run.wait_dispatch (INTERNAL); for last, last.client › channel.request.
@row_add n=811 row=contract/wait-span-attributes/wait-block area=span-trace-coverage rec=fct-ykdl4lpn@1
boundary: wait span attributes
item: wait.block
promise: wait.block carries after and timeout_ms.
@row_add n=812 row=contract/wait-span-attributes/run-wait-dispatch area=span-trace-coverage rec=fct-ykdl4lpn@1
boundary: wait span attributes
item: run.wait_dispatch
promise: woken_kind, one of turn-ended, question, permission, plan, session-end, dialog-expired, or timed_out.
@row_add n=813 row=contract/wait-log-fields/channel-request area=span-trace-coverage rec=fct-hluu43pb@2
boundary: wait and last log fields
item: channel-request
promise: channel-request{corr:<id>, conn, method:"wait"|"last"} on both sides, plus after and timeout_ms for wait only and only as u64 values.
@row_add n=814 row=contract/wait-log-fields/channel-response-outcome area=span-trace-coverage rec=fct-hluu43pb@2
boundary: wait and last log fields
item: channel-response outcome
promise: For wait only, outcome is the waking kind or timed-out, derived once over the shared wake set; the client side alone writes instance-unreachable when the reply never arrived; last carries no outcome and no result body reaches either line.
@row_add n=815 row=contract/mcp-refusal-text/instance-unreachable area=span-trace-coverage rec=fct-hluu43pb@2
boundary: viola mcp wait and last refusal
item: strict-modes-failed body
promise: MCP wait and last refused by the instance check return the cli document less v (error instance-unreachable, detail strict-modes-failed), their text refused: instance-unreachable, with no line.
@row_add n=816 row=decision/no-last-assistant-message area=span-trace-coverage rec=fct-hluu43pb@2
name: last_assistant_message never logged
rule: last_assistant_message never appears in a log line.
@row_add n=817 row=decision/disconnect-logged-by-client area=span-trace-coverage rec=fct-3omdhkir@1
name: Mid-call disconnect logged once by the client
rule: A mid-call disconnect is logged once, by the client.
@row_add n=818 row=contract/dialog-spans/span-chain area=span-trace-coverage rec=fct-u5crvqku@2
boundary: dialog spans
item: span chain
promise: hook.handle (SERVER) › channel.request(hook.dialog) → channel.dispatch › run.dialog_register › run.dialog_await › hook.decision_emit; driver side answer.client › channel.request(answer) → channel.dispatch.
@row_add n=819 row=contract/answer-link-record/failure area=span-trace-coverage rec=fct-u5crvqku@2
boundary: wrapper answer handling
item: failed link record
promise: A link record that fails answers -32603: the answer is not taken, the dialog stays held and no dialog-answered line is written.
@row_add n=820 row=decision/dialog-hook-fails-open area=span-trace-coverage rec=fct-u5crvqku@2
name: Dialog hook fails open on a failed check
rule: A dialog hook whose instance, socket-directory, euid or server check fails emits no decision and sends no frame, so the dialog falls back to the human.
@row_add n=821 row=decision/second-dialog-gets-null area=span-trace-coverage rec=fct-uicv6khr@1
name: A concurrent second dialog gets null
rule: A second concurrent dialog gets a null response.
@row_add n=822 row=contract/dialog-log-fields/dialog-raised area=span-trace-coverage rec=fct-ffumo3bh@1
boundary: dialog log fields
item: dialog-raised
promise: dialog-raised{corr:<dialog_id>, dialog_kind, hook_event}.
@row_add n=823 row=contract/dialog-log-fields/dialog-answered area=span-trace-coverage rec=fct-ffumo3bh@1
boundary: dialog log fields
item: dialog-answered
promise: dialog-answered{corr:<dialog_id>, dialog_kind, from, from_trust, deadline_hit, duration_ms}; from and from_trust only on an answer that carried a from.
@row_add n=824 row=contract/dialog-log-fields/hook-lines area=span-trace-coverage rec=fct-ffumo3bh@1
boundary: dialog log fields
item: hook-invoked and hook-decision
promise: hook-invoked{corr:<dialog_id>, hook_event, stdin_bytes, invoked_at}, then hook-decision{corr:<dialog_id>, hook_event, decision_emitted, deadline_hit, detail, duration_ms}.
@row_add n=825 row=decision/dialog-hook-invoked-after-response area=span-trace-coverage rec=fct-ffumo3bh@1
name: Dialog hook-invoked written after the response
rule: A dialog hook writes hook-invoked only once dialog_id is known, after the hook.dialog response; non-dialog hooks and fail-open paths before dialog_id write hook-invoked{corr:null} at once.
why: invoked_at still records the hook's true start, and the wrapper's dialog-raised gives live visibility during the wait.
@row_add n=826 row=contract/answer-refusal/not-delivered area=span-trace-coverage rec=fct-ffumo3bh@1
boundary: driver-side answer refusal
item: not-delivered
promise: channel-response{result_class:"refusal", refusal:"not-delivered"} with no detail on the line, exit 13.
@row_add n=827 row=contract/answer-refusal/unverified-cli area=span-trace-coverage rec=fct-ffumo3bh@1
boundary: driver-side answer refusal
item: unverified-cli
promise: refusal unverified-cli, exit 12, the answer withheld.
@row_add n=828 row=decision/no-dialog-content-in-logs area=span-trace-coverage rec=fct-ffumo3bh@1
name: Dialog content never logged
rule: The question, the answer and the tool input never appear in a log line.
@row_add n=829 row=contract/events-log/dialog-expired area=span-trace-coverage rec=fct-k233iq5j@1
boundary: events log
item: dialog-expired
promise: At the hold's end the wrapper appends one dialog-expired{dialog_id, dialog_kind} line (source wrapper, no question, plan or tool text) that wakes a parked wait, and writes dialog-answered with deadline_hit:true and no from.
@row_add n=830 row=decision/expired-dialog-renders-for-human area=span-trace-coverage rec=fct-k233iq5j@1
name: Expired dialog returns to the human
rule: At the hold's end the reply stays null, the hook writes hook-decision with decision_emitted false, exits 0 with empty stdout, and the dialog renders for the human.
@row_add n=831 row=contract/hook-decision/deadline-hit area=span-trace-coverage rec=fct-k233iq5j@1
boundary: dialog hook-decision line
item: deadline_hit
promise: The hook writes deadline_hit:true only when its own read times out at its backstop bound, DIALOG_HOLD_MAX + 5 s (3605 s).
@row_add n=832 row=contract/wheel-spans/span-chain area=span-trace-coverage rec=fct-a7vqqg3h@2
boundary: wheel spans
item: span chain
promise: run.wheel_transition (INTERNAL) › channel.dispatch(send) refused; pause.client or release.client (CLIENT) › channel.request(pause|release) → channel.dispatch(pause|release).
@row_add n=833 row=contract/pause-release/answer-after-wheel-record area=span-trace-coverage rec=fct-a7vqqg3h@2
boundary: pause and release methods
item: reply timing
promise: pause and release answer only after the move's wheel record lands, -32603 if it did not.
@row_add n=834 row=contract/wheel-span-attributes/run-wheel-transition area=span-trace-coverage rec=fct-yj7tr26f@1
boundary: wheel span attributes
item: run.wheel_transition
promise: wheel_from, wheel_to (human|driver), cause (human-input|manual-pause|release); a point-in-time span with a static name, opened once per wheel change on the wheel's worker thread.
@row_add n=835 row=decision/no-typed-keys-in-telemetry area=span-trace-coverage rec=fct-yj7tr26f@1
name: Typed keys never in telemetry
rule: The keys the human types never appear in a span.
@row_add n=836 row=contract/send-refused/human-typing area=span-trace-coverage rec=fct-wovktszp@1
boundary: wrapper send-refused
item: human-typing
promise: send-refused{corr:<cursor>, refusal:"human-typing", detail:null|"manual-pause", wheel:"human", side:"wrapper"}, with the client process-exit{exit_code:10}.
@row_add n=837 row=decision/release-from-driver-refused area=span-trace-coverage rec=fct-wovktszp@1
name: A release from a driver is refused
rule: A release that carries a string from is refused with -32602, and viola release forwards VIOLA_NAME, so a driver session's own viola release is refused too.
@row_add n=838 row=contract/release-refusal/release-from-driver area=span-trace-coverage rec=fct-wovktszp@1
boundary: release refusal lines
item: release-from-driver
promise: One release-from-driver{corr:<rpc id>, conn, from (only a valid name), from_trust:"self-reported"} at INFO beside the -32602 reply with data {"reason":"release-from-driver"}; its channel-response carries detail release-from-driver on both sides at WARN.
@row_add n=839 row=contract/release-refusal/invalid-params area=span-trace-coverage rec=fct-wovktszp@1
boundary: release refusal lines
item: invalid-params
promise: A from of another type, or a non-bool budget, is a plain -32602 (data null) with no release-from-driver line, its channel-response carrying detail invalid-params on both sides at WARN.
@row_add n=840 row=contract/release-budget/budget-gate area=span-trace-coverage rec=fct-wovktszp@1
boundary: release method
item: budget true
promise: A release with budget true does not touch the wheel and appends one budget-gate record when a pause stood, none otherwise; the string-from refusal and the budget type check come first.
@row_add n=841 row=contract/budget-spans/span-chain area=span-trace-coverage rec=fct-koyhaawm@1
boundary: budget governor spans
item: span chain
promise: hook.statusline (SERVER) › state.budget_write › statusline.shell_out (CLIENT); in run, run.budget_eval (INTERNAL).
@row_add n=842 row=contract/budget-span-attributes/state-budget-write area=span-trace-coverage rec=fct-6peizmat@1
boundary: budget span attributes
item: state.budget_write
promise: state.budget_write carries five_hour_pct and seven_day_pct.
@row_add n=843 row=contract/budget-span-attributes/statusline-shell-out area=span-trace-coverage rec=fct-6peizmat@1
boundary: budget span attributes
item: statusline.shell_out
promise: statusline.shell_out carries shell_exit_status and duration_ms.
@row_add n=844 row=contract/budget-span-attributes/run-budget-eval area=span-trace-coverage rec=fct-6peizmat@1
boundary: budget span attributes
item: run.budget_eval
promise: trigger (start|send|turn-ended), paused, and window only when paused; opened once per evaluation: at start, at a send's arrival, and after a turn-ended hook line is appended.
@row_add n=845 row=decision/unreadable-budget-reads-as-none area=span-trace-coverage rec=fct-6peizmat@1
name: Bad budget file reads as no reading
rule: A refused, unreadable, over-cap or other-version budget file reads as no reading.
@row_add n=846 row=contract/events-log/budget-gate area=span-trace-coverage rec=fct-6peizmat@1
boundary: events log
item: budget-gate
promise: A budget-gate record is appended only when the triple (paused, window, override end) changes.
@row_add n=847 row=contract/statusline-log-fields/hook-invoked area=span-trace-coverage rec=fct-gqw4imf3@2
boundary: statusline hook log fields
item: hook-invoked
promise: hook-invoked{hook_event:"statusline", corr:null}.
@row_add n=848 row=contract/statusline-log-fields/parse-rejected area=span-trace-coverage rec=fct-gqw4imf3@2
boundary: statusline hook log fields
item: parse-rejected
promise: parse-rejected{parser:"hook-stdin", detail:"oversize"|"malformed", count:1} for stdin the arm rejects.
@row_add n=849 row=contract/statusline-log-fields/statusline-shell area=span-trace-coverage rec=fct-gqw4imf3@2
boundary: statusline hook log fields
item: statusline-shell
promise: process-start and process-exit{subject:"statusline-shell", shell_exit_status, duration_ms}, only when the snapshot records a statusline_command.
@row_add n=850 row=contract/statusline-log-fields/hook-decision area=span-trace-coverage rec=fct-gqw4imf3@2
boundary: statusline hook log fields
item: hook-decision
promise: hook-decision{hook_event:"statusline", budget_written, duration_ms}, deadline_hit only when true; budget_written is true only when the payload held a rate_limits object and the write succeeded.
@row_add n=851 row=contract/statusline-decision-detail/oversize-stdin area=span-trace-coverage rec=fct-gqw4imf3@2
boundary: statusline hook-decision detail
item: oversize-stdin
promise: The statusline hook-decision carries detail oversize-stdin on that fail-open path.
@row_add n=852 row=contract/statusline-decision-detail/malformed-json area=span-trace-coverage rec=fct-gqw4imf3@2
boundary: statusline hook-decision detail
item: malformed-json
promise: The statusline hook-decision carries detail malformed-json on that fail-open path.
@row_add n=853 row=contract/statusline-decision-detail/deadline area=span-trace-coverage rec=fct-gqw4imf3@2,fct-v76wl5kz@1
boundary: statusline hook-decision detail
item: deadline
promise: The shell-out reaching its bound ends in hook-decision{detail:"deadline", deadline_hit:true}.
@row_add n=854 row=decision/statusline-silent-without-instance area=span-trace-coverage rec=fct-gqw4imf3@2
name: Statusline arm silent without a valid instance
rule: The statusline arm writes no line without a valid VIOLA_NAME or when the instance check refuses: it returns exit 0 at once with no role file opened.
@row_add n=855 row=contract/send-refused/budget-paused area=span-trace-coverage rec=fct-gqw4imf3@2
boundary: wrapper send-refused
item: budget-paused
promise: send-refused{corr:<cursor>, refusal:"budget-paused", detail:"five-hour"|"seven-day", window, used_percentage, resets_at, paused:true}, with client exit_code 11.
@row_add n=856 row=decision/rejected-statusline-stdin-runs-nothing area=span-trace-coverage rec=fct-v76wl5kz@1
name: Rejected statusline stdin runs nothing
rule: Oversize or malformed statusline stdin prints nothing and runs no shell.
@row_add n=857 row=decision/claude-env-strip area=span-trace-coverage rec=fct-6xa7lf3f@2
name: CLAUDE environment strip
rule: The child loses every inherited CLAUDE* name, matched exactly, except the persistent set kept in the claude_env_keep config key.
@row_add n=858 row=decision/identity-floor-always-stripped area=span-trace-coverage rec=fct-6xa7lf3f@2
name: Identity floor always stripped
rule: The 11 names of the identity floor are always removed whatever the persistent set says; beyond the floor there is no compile-time list of known names.
@row_add n=859 row=contract/claude-child-process-start/env-fields area=span-trace-coverage rec=fct-6xa7lf3f@2
boundary: process-start claude-child
item: env fields
promise: env_stripped_count, env_stripped_known (every stripped name, comma-joined) and env_kept (every kept persistent-set name, comma-joined).
@row_add n=860 row=decision/implicit-link-untraced area=span-trace-coverage rec=fct-46kjozmt@1
name: Implicit link has no line or span
rule: The implicit link record has no process-log line, no event value and no span; its trace is the call's channel pair, the link product line and the snapshot's links.
@row_add n=861 row=contract/link-methods/invalid-params area=span-trace-coverage rec=fct-lnbq3lnm@1
boundary: link and unlink methods
item: -32602
promise: A missing, non-string or invalid name, a driven that is not the serving wrapper's own name, or a driver equal to driven is -32602 with data null and no line; a record or remove error is -32603; neither method reads from.
@row_add n=862 row=decision/link-methods-untraced area=span-trace-coverage rec=fct-lnbq3lnm@1
name: Explicit link methods open no span
rule: The explicit link and unlink methods open no span and write no process-log line of their own; their channel lines hold no driver, driven or other param.
@row_add n=863 row=contract/events-log/link area=span-trace-coverage rec=fct-lnbq3lnm@1
boundary: events log
item: link
promise: link appends one link product line {driver, driven} (source wrapper) for a new pair and nothing for a pair the set holds, and answers after its line and the snapshot write.
@row_add n=864 row=contract/events-log/unlink area=span-trace-coverage rec=fct-lnbq3lnm@1
boundary: events log
item: unlink
promise: unlink appends one unlink product line {driver, driven} (source wrapper) only when it removes a pair, nothing for an absent one, and answers after its line and the snapshot write.
@row_add n=865 row=contract/state-recovered/snapshot-replayed area=span-trace-coverage rec=fct-wev2ceaa@1
boundary: state-recovered line
item: snapshot-replayed
promise: state-recovered{detail:"snapshot-replayed", file:"snapshot.json"} at WARN with no corr for an unreadable snapshot, one line per call; none for a present or absent snapshot, and none when the replay itself fails.
@row_add n=866 row=contract/state-recovered/snapshot-unsupported-v area=span-trace-coverage rec=fct-wev2ceaa@1
boundary: state-recovered line
item: snapshot-unsupported-v
promise: state-recovered{detail:"snapshot-unsupported-v", file:"snapshot.json", v_seen} at WARN with no corr for a newer snapshot, one line per call.
@row_add n=867 row=contract/state-recovered/scanned area=span-trace-coverage rec=fct-wev2ceaa@1
boundary: state-recovered line
item: scanned
promise: From the board read, the line carries instance (the caller) and scanned (the scanned instance's name when it parses, absent otherwise); with no VIOLA_NAME no log is open and the line goes nowhere.
@row_add n=868 row=decision/no-raw-snapshot-reads area=span-trace-coverage rec=fct-wev2ceaa@1
name: Product readers read the snapshot through replay
rule: Every product reader of the snapshot uses the replaying read; no product reader reads the snapshot raw.
@row_add n=869 row=contract/state-recovered/torn-line-healed area=span-trace-coverage rec=fct-wgyuisdz@1
boundary: state-recovered line
item: torn-line-healed
promise: state-recovered{detail:"torn-line-healed", file:"events.ndjson", offset} at WARN with no corr, by the process whose append healed, offset the log's length before the append; one line per heal.
@row_add n=870 row=decision/appender-heals-torn-line area=span-trace-coverage rec=fct-wgyuisdz@1
name: The appender heals a torn line
rule: An append that finds the events log's last byte is not LF writes one LF and its line in a single write; the reader neither heals nor logs.
@row_add n=871 row=contract/run-log-parse-rejected/ledger-stamps area=span-trace-coverage rec=fct-6z2owz4q@1
boundary: the run process log parse-rejected line
item: parser ledger-stamps
promise: An unreadable, malformed or strict-modes-refused capability ledger gives one parse-rejected line with parser "ledger-stamps", detail "unreadable", "malformed" or "strict-modes-failed", and a count, in the run log.
@row_add n=872 row=decision/unreadable-ledger-means-unverified area=span-trace-coverage rec=fct-6z2owz4q@1
name: Unreadable capability ledger means an unverified CLI
rule: A capability ledger that is missing a stamp, unreadable, malformed or refused by the strict-modes check is treated as an unverified CLI (cli_verified false).
@row_add n=873 row=contract/verify-spawn-subject/version-probe area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify child spawn subjects
item: version-probe
promise: The --version read of the agent CLI is logged as one process-start / process-exit pair with subject version-probe.
@row_add n=874 row=contract/verify-spawn-subject/verify-probe area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify child spawn subjects
item: verify-probe
promise: The print-mode probe is logged as one process-start / process-exit pair with subject verify-probe.
@row_add n=875 row=contract/verify-spawn-subject/agents-probe area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify child spawn subjects
item: agents-probe
promise: Run B's single agents --json read is logged as one process-start / process-exit pair with subject agents-probe.
@row_add n=876 row=contract/verify-spawn-subject/verify-pty-probe area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify child spawn subjects
item: verify-pty-probe
promise: Each of verify's five interactive PTY runs is logged as one process-start / process-exit pair with subject verify-pty-probe.
@row_add n=877 row=contract/verify-process-exit/interrupted area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify process-exit line
item: detail interrupted
promise: A verify stopped by SIGINT or SIGTERM starts no further spawn, closes every pair it opened and ends with process-exit subject self, exit_code 1, detail "interrupted".
@row_add n=878 row=contract/verify-exit-codes/0 area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify exit codes
item: 0
promise: Every relied ledger row passed.
@row_add n=879 row=contract/verify-exit-codes/1 area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify exit codes
item: 1
promise: A row failed, verify refused, or verify was stopped.
@row_add n=880 row=contract/verify-stop-stderr/unable-hint-pair area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify stderr on a stop
item: the stop pair
promise: A stopped verify prints exactly "unable: verify was stopped before it finished" and "hint: run viola verify again to stamp this CLI" on stderr, with empty stdout and no stamp written.
@row_add n=881 row=contract/verify-stdout/step-line area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify stdout
item: step line
promise: One "[NN/MM] <row id> <row words>  pass|fail" line is printed for each relied ledger row, MM being the relied row count.
@row_add n=882 row=contract/verify-stdout/summary-line area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify stdout
item: summary line
promise: The last stdout line is "stamped <version>  N pass  N fail", the two counts summing to MM.
@row_add n=883 row=contract/cli-stderr/refusal-pair area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: CLI stderr
item: unable and hint pair
promise: A refusal is printed to stderr as a fixed "unable:" line followed by a "hint:" line.
@row_add n=884 row=contract/cli-stderr/internal-error area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: CLI stderr
item: internal fault message
promise: An internal fault prints exactly "error: internal error".
@row_add n=885 row=contract/plugin-install-exit-codes/21 area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: plugin install exit codes
item: 21
promise: The strict-modes tree check refused; no log is opened and no line is written, with the stderr pair or, under --json, the instance-unreachable document.
@row_add n=886 row=contract/plugin-install-json/ok area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: plugin install --json document
item: ok
promise: Success prints {"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}, with no path in it.
@row_add n=887 row=contract/verify-process-exit/strict-modes-failed area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify process-exit line
item: detail strict-modes-failed
promise: A stamps store refused by its strict-modes check, under a parsed VIOLA_NAME, ends with process-exit carrying detail "strict-modes-failed" and the home pair on stderr.
@row_add n=888 row=decision/refused-home-tree-writes-no-line area=span-trace-coverage rec=fct-f3gi2vhc@2
name: Refused home tree writes no line
rule: verify and plugin install check the home tree before their log opens, and a refused tree opens no log and writes no line.
@row_add n=889 row=decision/verify-lines-carry-no-content area=span-trace-coverage rec=fct-f3gi2vhc@2
name: verify log lines carry no content
rule: No verify log line carries row text, a prompt, a session id, the probe command, a status line payload or a path.
@row_add n=890 row=decision/hidden-capture-arms-uninstrumented area=span-trace-coverage rec=fct-f3gi2vhc@2
name: Hidden hook capture arms uninstrumented
rule: The hidden hook capture arms used by verify's probes read no VIOLA_* variable, do no obs init, emit no hook lines, keep stderr empty and always exit 0.
why: uninstrumented by design, as ratified by the founder
@row_add n=891 row=decision/real-cli-verify-never-in-ci area=span-trace-coverage rec=fct-f3gi2vhc@2
name: Real-CLI verify never runs in CI
rule: In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
@row_add n=892 row=decision/verify-sole-stamps-writer area=span-trace-coverage rec=fct-f3gi2vhc@2
name: verify is the only stamps writer
rule: verify is the only writer of the capability ledger's stamps file.
@row_add n=893 row=decision/no-obs-gate-on-verify-lines area=span-trace-coverage rec=fct-f3gi2vhc@2
name: No obs gate depends on verify lines
rule: No observability gate depends on verify's own log lines; its home-level lines fall under the same panic, schema and secret-scan gates as any other.
@row_add n=894 row=decision/spans-not-serialized-in-v1 area=telemetry-strategy rec=fct-4joil3ca@1
name: Spans declared but not serialized in v1
rule: Spans are declared on the must-trace paths and every crate seam but are not written to output in v1.
why: they give the code its structure and are ready for the deferred OpenTelemetry bridge
@row_add n=895 row=decision/span-naming-skip-all area=telemetry-strategy rec=fct-4joil3ca@1
name: Span declaration convention
rule: Every span is declared with instrument, skip_all and a name of the form "<area>.<operation>".
@row_add n=896 row=decision/cross-process-join-corr-conn area=telemetry-strategy rec=fct-4joil3ca@1,fct-474ly2yg@1
name: Cross-process joins by corr and conn
rule: Lines across processes are joined by the corr and conn fields, not by W3C trace IDs.
@row_add n=897 row=decision/structured-logs-primary-signal area=telemetry-strategy rec=fct-4ns7wzxz@1
name: Structured logs are the primary signal
rule: tracing-subscriber JSON-per-line logs, one file per process role under the home's diagnostics directory, are the primary observability signal, merged with the event stream by agent-run logs.
@row_add n=898 row=decision/panic-caught-at-every-role area=telemetry-strategy rec=fct-6c7xwzms@1
name: Panics caught at every role's catch site
rule: A hand-written panic hook plus catch_unwind at every role's main-thread catch site, hook dispatch, the run PTY pump and handle-wait threads, and the vt100 feed turn a panic into one record.
@row_add n=899 row=decision/metrics-derived-from-ndjson area=telemetry-strategy rec=fct-6lxvbxmt@1
name: Metrics derived from logs, no instruments
rule: There are no in-process metric instruments and no exporter; metrics are derived by jq over the ndjson logs, and perf gates run on hyperfine JSON exports.
@row_add n=900 row=decision/events-stream-never-rotated area=telemetry-strategy rec=fct-mtgjpoqf@1,fct-kytmq3ju@1
name: Event stream never rotated
rule: The instance's events.ndjson is the product audit stream and is never rotated by a generic library.
why: byte offsets in it are the wait-after and send cursors
@row_add n=901 row=decision/panic-one-line-record area=telemetry-strategy rec=fct-6ukjckx6@1
name: One single-line record a panic
rule: Every panic after obs init produces exactly one single-line structured record in its home-level role file, with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file.
except: the pre-init window and the paths that refuse or lack an instance or home tree before their role file opens
@row_none n=169 rec=fct-pl6vsrqd@1 type=S
@row_none n=170 rec=fct-xumv4vtr@1 type=S
@row_add n=902 row=decision/hook-silent-without-instance area=telemetry-strategy rec=fct-n6l5o3fm@2
name: Hook silent without an instance
rule: A hook with no VIOLA_NAME, or with a refused or absent instance, exits 0 at once and writes no file, no line and no frame.
@row_none n=171 rec=fct-gqzo5zgv@1 type=S
@row_none n=172 rec=fct-kgjd4mty@1 type=S
@row_add n=903 row=decision/pre-open-strict-modes-refusal-silent area=telemetry-strategy rec=fct-ne5xp5cg@2,fct-4hckhgvs@1
name: Strict-modes refusal before a log opens writes no line
rule: A strict-modes refusal made before any process log opens writes no line and carries no logged detail code; its machine-readable form is the exit code and, under --json, its closed document.
except: a refused socket directory always logs detail socket-dir-refused, except at MCP's frame tool, which writes no line
@row_add n=904 row=contract/strict-modes-refusal-exit/21 area=telemetry-strategy rec=fct-4hckhgvs@1
boundary: strict-modes refusal exit codes
item: 21
promise: The six frame verbs and link / unlink exit 21 on a strict-modes refusal, under --json with the instance-unreachable document carrying detail strict-modes-failed.
@row_add n=905 row=contract/strict-modes-refusal-exit/1 area=telemetry-strategy rec=fct-4hckhgvs@1
boundary: strict-modes refusal exit codes
item: 1
promise: run, revive's start and verify exit 1 on a strict-modes refusal; revive under --json prints the strict-modes-failed document.
@row_add n=906 row=contract/socket-dir-refused-exit/codes area=telemetry-strategy rec=fct-4hckhgvs@1,fct-ne5xp5cg@2
boundary: socket-dir-refused exit codes
item: socket-dir-refused
promise: A refused socket directory logs detail socket-dir-refused with no path, pid or uid, and exits 21 at the eight CLI frame verbs and 1 at run and revive's start.
@row_none n=173 rec=fct-wkatnlg5@1 type=S
@row_none n=174 rec=fct-pso667is@1 type=S
@row_none n=175 rec=fct-7hbkkb6o@1 type=S
@row_none n=176 rec=fct-olppdytk@1 type=S
@row_none n=177 rec=fct-4vc4xc3n@1 type=S
@row_none n=178 rec=fct-6isebxud@2 type=S
@row_none n=179 rec=fct-shii35kl@1 type=S
@row_none n=180 rec=fct-oqunheh6@1 type=S
@row_none n=181 rec=fct-lrne4qjd@1 type=S
@row_none n=182 rec=fct-tdzuoari@1 type=S
@row_none n=183 rec=fct-go7yectq@1 type=S
@row_none n=184 rec=fct-fwi5gkgt@1 type=S
@row_add n=907 row=decision/no-opentelemetry-in-v1 area=bootstrap-phases-derive-for-route-setup-project rec=fct-7tykupyn@1
name: No OpenTelemetry in v1
rule: No opentelemetry crate is added in v1, and cargo deny bans opentelemetry-otlp, opentelemetry-stdout, sentry and tracing-appender on the workspace.
why: so they cannot arrive transitively without a Decisions Log entry
guard: cargo deny bans
@row_add n=908 row=decision/subscriber-only-in-root-bin area=bootstrap-phases-derive-for-route-setup-project rec=fct-y2h27pfd@2
name: tracing-subscriber only in the root binary
rule: tracing-subscriber is a product dependency of the root binary only, without its chrono feature; the product crates depend on tracing alone.
except: a library crate may take it as a dev-dependency for in-memory line capture in its own unit tests
@row_add n=909 row=decision/obs-event-single-macro area=bootstrap-phases-derive-for-route-setup-project rec=fct-y2h27pfd@2,fct-in3nxxob@1
name: One obs_event macro for every line
rule: Every log line is emitted through the one obs_event macro defined in viola-core, which expands to tracing's event at the caller, so viola-core stays I/O-free and tracing-free.
@row_add n=910 row=decision/direct-chrono-dependency area=bootstrap-phases-derive-for-route-setup-project rec=fct-y2h27pfd@2
name: Direct chrono dependency for timestamps
rule: The root binary takes chrono directly, without default features, to produce the millisecond UTC timestamp.
why: tracing-subscriber's chrono feature does not re-export chrono
@row_add n=911 row=decision/raw-tracing-ban area=bootstrap-phases-derive-for-route-setup-project rec=fct-y2h27pfd@2,fct-safgt2vp@1
name: Raw tracing macros banned
rule: The tracing level macros are banned by clippy disallowed-macros and raw event calls by a fail-closed grep outside the obs module, so only obs_event emits lines.
why: clippy reports a disallowed macro expanded inside an exported macro at the caller, so no allow inside obs_event can exempt it
guard: clippy disallowed-macros and scripts/lint-probes.sh
@row_add n=912 row=decision/single-service-identity-source area=bootstrap-phases-derive-for-route-setup-project rec=fct-lrh7vv4t@1
name: One source for service name and version
rule: The core crate's service name and version constants are the single source for process-start version, sender and writer, with the workspace version shared by every member.
@row_add n=913 row=decision/diag-schema-matches-event-enum area=bootstrap-phases-derive-for-route-setup-project rec=fct-vwautgwv@1
name: Diagnostic line schema matches the event enum
rule: A hand-maintained JSON Schema of the process-log line is committed, and its event enum must equal the ObsEvent variants, asserted by a tests-owned check.
@row_add n=914 row=decision/liveness-transitions-not-logged area=bootstrap-phases-derive-for-route-setup-project rec=fct-lnkipj6a@1,fct-t7swfgbl@1
name: Liveness transitions not logged
rule: No process-log line carries a liveness transition; liveness is exposed only by readers such as list --json.
why: no process watches another instance's heartbeat
@row_add n=915 row=decision/instrument-skip-all-everywhere area=bootstrap-phases-derive-for-route-setup-project rec=fct-vhrcelle@1,fct-gxspxfur@1
name: Spans skip all arguments
rule: Every instrumented function uses skip_all with explicit fields, so no argument is recorded implicitly.
guard: G1
@row_add n=916 row=decision/veil-redact-payload-types area=bootstrap-phases-derive-for-route-setup-project rec=fct-vhrcelle@1,fct-gxspxfur@1
name: veil redacts payload types
rule: Payload types derive veil's Redact, and cargo deny bans veil's toggle feature.
guard: cargo deny veil toggle ban
@row_add n=917 row=decision/fixed-error-display area=bootstrap-phases-derive-for-route-setup-project rec=fct-vhrcelle@1
name: Fixed error Display text
rule: Typed errors have fixed Display text that carries no interpolated content.
@row_add n=918 row=decision/content-only-in-instance-detail-file area=bootstrap-phases-derive-for-route-setup-project rec=fct-vhrcelle@1,fct-r43ltybn@2
name: Content only in instance-scoped detail files
rule: Content-bearing detail goes only to the instance's detail-<process>.ndjson file; home-level log files are codes-only and carry no user content, token, cookie or CLAUDE* value.
why: security allows user-derived content only in instance-scoped diagnostics
@row_add n=919 row=decision/sha-pinned-ci-actions area=bootstrap-phases-derive-for-route-setup-project rec=fct-safgt2vp@1
name: SHA-pinned CI actions
rule: The ripgrep install step and the upload-artifact action are pinned by SHA.
@row_none n=185 rec=fct-dpkhbaeo@1 type=0
what: the one-time ordering of bootstrap setup phases
@row_none n=186 rec=fct-hwfvcwas@1 type=0
what: the one-time placement of a merged bootstrap setup phase
@row_none n=187 rec=fct-sziec6n5@1 type=S
@row_none n=188 rec=fct-l76pk6pu@1 type=S
@row_add n=920 row=decision/heartbeat-file-not-logged area=heartbeat-ticks rec=fct-nmtbwule@1
name: Heartbeat file touched, not logged
rule: run touches its instance heartbeat file every second and logs nothing per tick; mcp, hook and short-lived verbs have no tick.
why: a one-second line into a never-rotated file would grow without bound
@row_add n=921 row=contract/list-json/liveness area=heartbeat-ticks rec=fct-t7swfgbl@1
boundary: list --json
item: liveness
promise: list --json exposes each instance's liveness.
@row_add n=922 row=contract/list-liveness/stale-threshold area=heartbeat-ticks rec=fct-s6vdltl4@1
boundary: list liveness
item: stale threshold
promise: An instance is live while its heartbeat is at most 5 s old and stale or gone beyond that.
@row_add n=923 row=contract/agent-run-status/state-degraded area=heartbeat-ticks rec=fct-s6vdltl4@1
boundary: agent-run status
item: state degraded
promise: The status is degraded when a booted instance's item is stale or absent.
@row_add n=924 row=contract/diagnostics-files/paths area=log-file-location rec=fct-r43ltybn@2
boundary: diagnostics files
item: process log paths
promise: Process logs are written to <home>/diagnostics/ as run-<name>, hook-<name>, mcp and cli-<name> .ndjson files, and detail to <home>/instances/<name>/diagnostics/detail-<process>.ndjson.
@row_add n=925 row=contract/diagnostics-files/home-resolution area=log-file-location rec=fct-r43ltybn@2
boundary: diagnostics files
item: home resolution
promise: The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
@row_add n=926 row=decision/log-files-owner-only area=log-file-location rec=fct-r43ltybn@2
name: Owner-only log files
rule: Log files are created 0600 inside 0700 directories.
guard: secret-scan test
@row_add n=927 row=decision/no-log-rotation-in-v1 area=log-file-location rec=fct-kytmq3ju@1
name: No log rotation in v1
rule: Diagnostics are not rotated and no rotation library is used; retention is the lifetime of the session home.
@row_add n=928 row=decision/tracing-appender-rejected area=log-file-location rec=fct-kytmq3ju@1
name: tracing-appender rejected
rule: tracing-appender is not used as a log writer.
why: its non_blocking writer drops lines, and it pulls in an advisory-affected time crate
@row_add n=929 row=decision/obs-aligns-to-tests-log-format area=log-format-json-schema rec=fct-5k3podjh@1,fct-evvm5jqh@1
name: Observability aligns to the tests log format
rule: The log format is bound by the test plan's harness contract; observability may add fields but must not rename or remove its fields.
why: the harness greps on them
@row_add n=930 row=contract/event-stream/line area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: the event stream
item: event line
promise: Each line of instances/<name>/events.ndjson is exactly {"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}.
@row_add n=931 row=contract/process-log-line/timestamp area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: the process log line
item: timestamp
promise: RFC 3339 UTC with milliseconds and Z.
@row_add n=932 row=contract/process-log-line/level area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: the process log line
item: level
promise: One of DEBUG, INFO, WARN, ERROR.
@row_add n=933 row=contract/process-log-line/target area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: the process log line
item: target
promise: The target is always present.
@row_add n=934 row=contract/process-log-line/message area=log-format-json-schema rec=fct-evvm5jqh@1,fct-in3nxxob@1
boundary: the process log line
item: message
promise: The message is a static literal equal to the event name and never carries interpolated content.
@row_add n=935 row=contract/process-log-line/process area=log-format-json-schema rec=fct-evvm5jqh@1,fct-m6rejt2n@1
boundary: the process log line
item: process
promise: One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing cli-<name>.ndjson.
@row_add n=936 row=contract/process-log-line/instance area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: the process log line
item: instance
promise: The instance's name, or absent when none resolved.
@row_add n=937 row=contract/process-log-line/corr area=log-format-json-schema rec=fct-evvm5jqh@1,fct-in3nxxob@1
boundary: the process log line
item: corr
promise: Copied unchanged as a JSON number or string, never renamed and never debug- or display-formatted; required only on the events whose rules always define it.
@row_add n=938 row=contract/corr-by-event/channel area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: corr by event
item: channel-request and channel-response
promise: corr is the JSON-RPC request id; null for an id-less hook.event notification and on a response to a frame that yielded no id.
@row_add n=939 row=contract/corr-by-event/dialog area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: corr by event
item: dialog-raised and dialog-answered
promise: corr is the dialog_id.
@row_add n=940 row=contract/corr-by-event/hook area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: corr by event
item: hook-invoked and hook-decision
promise: corr is the dialog_id for dialog hooks, otherwise null; a dialog hook-invoked before the id is known, or a hook-decision carrying detail, may be null.
@row_add n=941 row=contract/corr-by-event/send area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: corr by event
item: send-issued, send-confirmed, send-refused
promise: corr is the send cursor; a wrapper refusal before send-issued carries the target's event-stream end offset, never returned to the caller; wrapper send lines carry conn and rpc_id; the client-side send-refused has corr null.
@row_add n=942 row=contract/corr-by-event/release-from-driver area=log-format-json-schema rec=fct-evvm5jqh@1,fct-ufgywcj6@1
boundary: corr by event
item: release-from-driver
promise: corr is the JSON-RPC request id of the release refused with -32602 for carrying a string from.
@row_add n=943 row=contract/corr-by-event/process-and-panic area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: corr by event
item: process-start, process-exit, panic
promise: corr is null.
@row_add n=944 row=contract/corr-by-event/state-recovered area=log-format-json-schema rec=fct-evvm5jqh@1,fct-ufgywcj6@1
boundary: corr by event
item: state-recovered
promise: corr is null.
@row_add n=945 row=contract/corr-by-event/parse-rejected area=log-format-json-schema rec=fct-evvm5jqh@1,fct-ufgywcj6@1
boundary: corr by event
item: parse-rejected
promise: corr is null.
@row_add n=946 row=decision/null-as-key-absence area=log-format-json-schema rec=fct-evvm5jqh@1,fct-zf2424ri@1
name: Null written as key absence
rule: A null corr or instance is written as an absent key, readers treat absence and null as equal, and the schema rejects a literal null.
why: tracing has no null field value
@row_add n=947 row=decision/no-multiline-stack-traces area=log-format-json-schema rec=fct-evvm5jqh@1
name: No multi-line stack traces
rule: Every log line is one physical line parseable by jq; a hook panic is caught and logged as one ERROR line with event panic.
@row_none n=189 rec=fct-dd4kcev2@1 type=8
@row_none n=190 rec=fct-vbofen4i@1 type=S
@row_none n=191 rec=fct-imiry6ko@1 type=S
@row_none n=192 rec=fct-odc7kjss@1 type=0
what: an illustrative example log line
@row_add n=948 row=decision/tracing-logging-library area=logging-stack rec=fct-in3nxxob@1
name: tracing as the logging library
rule: tracing is the logging facade and tracing-subscriber's JSON formatter writes the lines.
why: Tokio-free and no C, and meets the security floor for tracing-subscriber
@row_add n=949 row=contract/obs-event-macro/attached-fields area=logging-stack rec=fct-in3nxxob@1
boundary: the obs_event macro
item: attached fields
promise: Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.
@row_add n=950 row=contract/diagnostics-line/format area=logging-stack rec=fct-i4z6jsd7@2
boundary: the diagnostics line
item: format
promise: Each diagnostics line is one structured JSON object on one line, written whole by one write of the complete line on an append-mode handle.
@row_add n=951 row=contract/diagnostics-line/role-file-line-size area=logging-stack rec=fct-i4z6jsd7@2
boundary: the diagnostics line
item: role-file line size
promise: A line in a role file (run, hook, mcp, cli) is codes-only and stays well under 4 KiB.
@row_add n=952 row=contract/diagnostics-line/detail-line-size area=logging-stack rec=fct-i4z6jsd7@2
boundary: the diagnostics line
item: detail-file line size
promise: A line in an instance detail file (chains, drift reports, one-line backtraces) is not codes-only and may exceed 4 KiB.
@row_add n=953 row=decision/shared-log-files-concurrent-append area=logging-stack rec=fct-i4z6jsd7@2
name: Concurrent appends to shared diagnostics files
rule: Every diagnostics file may have concurrent writers in separate processes; each line is one whole write on an append-mode handle, readers tolerate a torn line, and a concurrent-append check, including a detail line over 4 KiB, asserts every line parses.
why: Non-interleaving at this size is an assumption, not a POSIX guarantee, because the 4 KiB PIPE_BUF rule applies to pipes.
guard: concurrent-append check; hook_panics_append_whole_lines_over_4_kib_side_by_side
@row_add n=954 row=contract/agent-run-logs/torn-line area=logging-stack rec=fct-i4z6jsd7@2,fct-v725cwjk@2
boundary: agent-run logs output
item: torn line
promise: A torn line appears as {"torn":true,"offset":n}, and readers skip it rather than fail.
@row_add n=955 row=decision/no-internal-error-fallback area=logging-stack rec=fct-pbscnpyo@1
name: No stderr fallback for subscriber internal errors
rule: The diagnostics subscriber never reports its own internal errors by printing to stderr.
why: The default falls back to eprintln, which would break the Hook contract and the run terminal rule.
@row_add n=956 row=decision/no-span-list-in-lines area=logging-stack rec=fct-pbscnpyo@1
name: No span list in diagnostics lines
rule: Diagnostics lines never carry the list of entered spans.
why: Closes the spans leak path.
@row_add n=957 row=decision/explicit-log-writer area=logging-stack rec=fct-pbscnpyo@1
name: Explicit diagnostics writer
rule: The diagnostics subscriber always has its writer set explicitly.
why: The default writer is stdout.
@row_add n=958 row=decision/no-ansi-in-logs area=logging-stack rec=fct-pbscnpyo@1
name: No ANSI codes in diagnostics lines
rule: Diagnostics lines carry no ANSI colour codes.
@row_add n=959 row=decision/no-source-paths-in-logs area=logging-stack rec=fct-pbscnpyo@1
name: No source paths in diagnostics lines
rule: Diagnostics lines carry no source file names or line numbers.
@row_add n=960 row=decision/file-only-diagnostics-sinks area=logging-stack rec=fct-zlgd74pd@1
name: File-only diagnostics, one file per role
rule: Every role logs only to its own file under the home's diagnostics directory; no role has a stdout sink, and there is no pretty or dev sink.
@row_add n=961 row=contract/diagnostics-files/detail area=logging-stack rec=fct-zlgd74pd@1
boundary: the diagnostics files
item: detail
promise: Content-bearing records (full error chains, drift reports, panic payload and backtrace) go only to <home>/instances/<name>/diagnostics/detail-<process>.ndjson, file 0600 in a 0700 directory, and only when an instance resolves.
@row_add n=962 row=decision/content-records-bypass-subscriber area=logging-stack rec=fct-zlgd74pd@1
name: Content-bearing records bypass the subscriber
rule: Content-bearing records never pass through the tracing subscriber, whose one writer holds only the home-level role file; the detail writer formats each detail line itself and writes it with one whole write.
@row_add n=963 row=contract/detail-line/fields area=logging-stack rec=fct-zlgd74pd@1
boundary: the detail line
item: fields
promise: Each detail line carries timestamp (millisecond UTC format), level, target, message, event, process, instance, and the corr / conn keys.
@row_add n=964 row=decision/no-content-in-home-level-lines area=logging-stack rec=fct-zlgd74pd@1
name: No content fields in home-level lines
rule: No home-level diagnostics line carries chain, drift_report, panic_payload or backtrace.
guard: G4 diag-line.v1.json schema
@row_add n=965 row=decision/no-agent-mode-flag area=logging-stack rec=fct-brueynpb@3
name: No agent-mode flag
rule: There is no agent-mode flag; JSON to file is the only logging mode.
@row_add n=966 row=contract/config-json/diagnostics-level area=logging-stack rec=fct-brueynpb@3,fct-6bbewo2y@2
boundary: config.json
item: diagnostics_level
promise: Key diagnostics_level takes "info" (the default) or "debug"; a missing or unreadable file or an unknown value falls back to "info".
@row_add n=967 row=decision/log-level-never-from-env area=logging-stack rec=fct-brueynpb@3
name: Log level never from the environment
rule: The diagnostics level is never read from RUST_LOG and the env-filter feature is never enabled; the level comes only from config.json.
@row_add n=968 row=decision/level-changes-volume-only area=logging-stack rec=fct-brueynpb@3
name: Level changes volume only
rule: Every diagnostics level uses the same field allow-list, so the level changes only how many lines are written.
why: No setting can disable redaction.
@row_add n=969 row=decision/third-party-targets-off area=logging-stack rec=fct-brueynpb@3
name: Third-party log targets off
rule: Records of third-party targets (rmcp, portable_pty, sysinfo, interprocess) are off in every sink; their failures are captured at viola's seam from the returned results.
why: Their records lack the required fields.
@row_add n=970 row=decision/no-log-crate-bridge area=logging-stack rec=fct-brueynpb@3
name: No log-crate bridge
rule: The log-crate bridge (tracing-log, LogTracer) stays off; failures are captured from returned values or handles, and a gap is fixed at the seam or by the researched json-subscriber fallback, never by enabling the bridge.
why: The accepted loss is only third-party log context emitted without a returned error.
guard: chaos case "child exit without EOF"
@row_add n=971 row=decision/no-otel-sdk-v1 area=otel-sdk-init rec=fct-56r4goln@1,fct-rrjwwqcp@2
name: No OpenTelemetry SDK in v1
rule: No OpenTelemetry SDK, bridge or Resource is used in v1.
why: With no exporter they would add only dependency and cargo-mutants surface, and bridged span IDs would have no consumer.
@row_add n=972 row=decision/tracing-is-the-instrumentation-facade area=otel-sdk-init rec=fct-56r4goln@1
name: tracing as the instrumentation facade
rule: tracing is the instrumentation facade in every instrumentable crate, and tracing-subscriber is the formatter and sink, a product dependency of the root bin only.
@row_add n=973 row=decision/otel-resource-built-empty area=otel-sdk-init rec=fct-56r4goln@1,fct-rrjwwqcp@2
name: OTel Resource built empty if ever adopted
rule: If an OTel SDK is ever adopted, its Resource is built empty with viola's own identity values, never with the default builder.
why: The default builder's environment detector reads OTEL_* env vars.
@row_add n=974 row=decision/tracing-subscriber-minimal-features area=otel-sdk-init rec=fct-56r4goln@1
name: Minimal tracing-subscriber features
rule: tracing-subscriber is taken with default features off and only fmt, json, registry and std; ansi, tracing-log and chrono stay off.
why: The chrono feature only enables ChronoUtc / ChronoLocal, and the millisecond timer uses the root bin's direct chrono dependency.
@row_add n=975 row=decision/panic-hook-first area=otel-sdk-init rec=fct-6bbewo2y@2
name: Panic hook installed first
rule: The panic hook is installed as the first statement of main and writes only to the diagnostics file once that is open, never falling back to stderr.
@row_add n=976 row=decision/home-resolution-order area=otel-sdk-init rec=fct-6bbewo2y@2
name: Home resolution order
rule: The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
@row_add n=977 row=decision/instance-source-per-role area=otel-sdk-init rec=fct-6bbewo2y@2,fct-2geebg2r@2
name: Instance source fixed per role
rule: run takes its own name argument, never an inherited VIOLA_NAME; hook and mcp take VIOLA_NAME only; cli takes the caller's VIOLA_NAME, and the target argument only when VIOLA_NAME is unset.
why: A wrapper never writes into another instance's run log.
@row_add n=978 row=decision/strict-modes-before-diagnostics area=otel-sdk-init rec=fct-6bbewo2y@2
name: Home strict-modes check before diagnostics
rule: Nothing under the home, the diagnostics directory included, is created or opened before the role's strict-modes check on the home passes; a refused home gets no diagnostics line.
@row_add n=979 row=decision/private-diagnostics-files area=otel-sdk-init rec=fct-6bbewo2y@2
name: Owner-only diagnostics files
rule: The diagnostics directory is 0700 and each diagnostics file 0600, opened without following symlinks; before the first write each must be owned by the effective user, have no group or other bits and not be a symlink, a failed check counting as a failed open.
why: A pre-existing foreign, permissive or symlinked diagnostics file is never written.
@row_add n=980 row=decision/failed-log-open-discards area=otel-sdk-init rec=fct-6bbewo2y@2
name: Failed log open discards lines
rule: When the role file cannot be opened, the diagnostics writer discards lines; only mcp writes to stderr instead.
@row_add n=981 row=decision/lazy-detail-file area=otel-sdk-init rec=fct-6bbewo2y@2
name: Detail file opened lazily
rule: The instance detail file is opened only by the first content-bearing record or by the panic hook, and a failed detail open drops only the detail line, writing nothing to stderr.
why: The error-free hook path keeps exactly one log open.
@row_add n=982 row=contract/config-json/five-hour area=otel-sdk-init rec=fct-6bbewo2y@2
boundary: config.json
item: five_hour
promise: Key five_hour holds a whole percentage from 0 to 100; any other value is logged as one parse-rejected{parser:"config-json", detail:"malformed"} WARN line.
@row_add n=983 row=contract/config-json/seven-day area=otel-sdk-init rec=fct-6bbewo2y@2
boundary: config.json
item: seven_day
promise: Key seven_day holds a whole percentage from 0 to 100; any other value is logged as one parse-rejected{parser:"config-json", detail:"malformed"} WARN line.
@row_add n=984 row=contract/config-json/v area=otel-sdk-init rec=fct-6bbewo2y@2
boundary: config.json
item: v
promise: A document that is no object, or whose v is not 1, gives the threshold and hold defaults and no further line.
@row_add n=985 row=contract/config-json/known-keys area=otel-sdk-init rec=fct-6bbewo2y@2
boundary: config.json
item: known keys
promise: config.json has six known keys: v, diagnostics_level, claude_env_keep, five_hour, seven_day, dialog_hold_s; any other key is counted as skipped.
@row_add n=986 row=contract/diagnostics-events/config-parse-rejected area=otel-sdk-init rec=fct-6bbewo2y@2
boundary: diagnostics events
item: parse-rejected config-json
promise: Right after process-start, a config.json parse that failed or skipped keys logs parse-rejected{parser:"config-json", detail, count}.
@row_add n=987 row=decision/diagnostics-init-before-role-work area=otel-sdk-init rec=fct-6bbewo2y@2
name: Diagnostics init before the role's work
rule: Diagnostics init and the process-start line come before the role's work: in run before its collision check and the child spawn, in mcp before its stdio transport starts.
why: Exit-1 causes are logged.
@row_add n=988 row=decision/instance-check-before-log-init area=otel-sdk-init rec=fct-6bbewo2y@2
name: Instance check before log init
rule: Commands acting on an instance (revive's start arm, the hook arms, link, unlink, send, wait, last, answer, pause, release) run the instance strict-modes check before opening any log; a refusal opens and writes nothing and leaves modes as found.
why: Opening the role file narrows an existing home to 0700, so a check made after it passes a home at 0770.
except: The two hook --capture arms run ahead of the check.
@row_add n=989 row=decision/home-tree-check-before-log-init area=otel-sdk-init rec=fct-6bbewo2y@2
name: Home tree check before log init
rule: run, revive's start arm and viola verify check the home tree's modes before their log opens; a refusal writes no line, starts no child, leaves modes as found and exits 1.
@row_add n=990 row=decision/hook-single-log-open area=otel-sdk-init rec=fct-6bbewo2y@2
name: One config read and one log open per hook
rule: A hook invocation makes one config.json read and exactly one log open, then only appends.
guard: hook max < 1.0 s gate
@row_add n=991 row=decision/hook-refusal-exits-zero area=otel-sdk-init rec=fct-6bbewo2y@2
name: Refused hook fails open
rule: A hook whose instance is refused or absent exits 0 at once, reading nothing under the home and no stdin, writing no frame and no log line.
@row_add n=992 row=contract/cli-exit-codes/21 area=otel-sdk-init rec=fct-6bbewo2y@2
boundary: CLI exit codes
item: 21
promise: An instance strict-modes refusal exits 21 with the fixed instance-state pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
@row_none n=193 rec=fct-vguxziuc@1 type=S
@row_none n=194 rec=fct-rar73ens@1 type=4
relied: The subscriber flattens event fields to the top level of each line, and the Targets filter is the only configurable level gate.
@row_add n=993 row=decision/millisecond-utc-timestamps area=otel-sdk-init rec=fct-xxrgqpdc@1
name: Millisecond UTC timestamps
rule: Every diagnostics timestamp is RFC 3339 in UTC with millisecond precision and a Z suffix.
why: The default timer writes microseconds, and the chrono RFC 3339 timer writes +00:00 with automatic precision.
@row_add n=994 row=decision/normal-observability-mode area=product-mode-normal-vs-inverted-self-observing rec=fct-2los3fjb@1
name: Normal observability mode
rule: viola runs in Normal observability mode: events.ndjson is a product audit log, not a telemetry collector.
why: viola is not a telemetry product; the self-observing-inversion trigger did not fire.
@row_add n=995 row=decision/env-vars-not-configuration area=service-identity rec=fct-plkmesmz@1,fct-ah3dlz7g@1
name: Environment variables are not configuration
rule: Environment variables are not a configuration channel: service identity has no env-var source or fallback.
@row_add n=996 row=decision/single-version-constant area=service-identity rec=fct-ycmm64u7@1
name: One version constant for every crate
rule: One compile-time version constant taken from the package version serves every crate, and every crate takes the workspace version.
why: The log version, channel sender and snapshot writer cannot drift apart.
@row_add n=997 row=absence/hosted-deployment area=service-identity rec=fct-ah3dlz7g@1
capability: A hosted deployment or deployment environment; viola is local-only
@row_add n=998 row=contract/process-start/service-name area=service-identity rec=fct-rrjwwqcp@2
boundary: the process-start line
item: service_name
promise: process-start carries the service name as field service_name.
@row_add n=999 row=contract/process-start/version area=service-identity rec=fct-rrjwwqcp@2
boundary: the process-start line
item: version
promise: process-start carries the product version as field version.
@row_add n=1000 row=contract/process-start/os area=service-identity rec=fct-rrjwwqcp@2
boundary: the process-start line
item: os
promise: process-start carries field os, the one word linux.
@row_add n=1001 row=contract/process-start/pid area=service-identity rec=fct-rrjwwqcp@2
boundary: the process-start line
item: pid
promise: process-start carries the process id as field pid.
@row_add n=1002 row=contract/diagnostics-line/process area=service-identity rec=fct-rrjwwqcp@2
boundary: the diagnostics line
item: process
promise: Every diagnostics line carries the role as field process.
@row_add n=1003 row=contract/diagnostics-line/instance area=service-identity rec=fct-rrjwwqcp@2
boundary: the diagnostics line
item: instance
promise: Every diagnostics line carries the instance as field instance.
@row_add n=1004 row=absence/pulse-class-receiver area=snapshot-paste-to-ai-integration rec=fct-ehmrhyqq@1
capability: A pulse-class log receiver
@row_add n=1005 row=contract/agent-run-logs/events-record area=snapshot-paste-to-ai-integration rec=fct-v725cwjk@2
boundary: agent-run logs output
item: events record
promise: Each events.ndjson line is merged as {"src":"events","instance","offset","record"}.
@row_add n=1006 row=contract/agent-run-logs/diag-record area=snapshot-paste-to-ai-integration rec=fct-v725cwjk@2
boundary: agent-run logs output
item: diag record
promise: Each line of every diagnostics/*.ndjson is merged as {"src":"diag","file","record"}.
@row_add n=1007 row=contract/agent-run-logs/detail-record area=snapshot-paste-to-ai-integration rec=fct-v725cwjk@2,fct-45i3uku5@1
boundary: agent-run logs output
item: detail record
promise: Every instance detail line from instances/*/diagnostics/detail-*.ndjson is merged as {"src":"diag","file","instance","record"}, the instance telling apart the same detail file name across instances.
@row_add n=1008 row=contract/agent-run-logs/filters area=snapshot-paste-to-ai-integration rec=fct-v725cwjk@2
boundary: agent-run logs output
item: filters
promise: The merged output is filterable by --instance, --kind, --process and --after.
@row_add n=1009 row=contract/agent-run-status/state area=snapshot-paste-to-ai-integration rec=fct-mmhn4sme@1
boundary: agent-run status
item: state
promise: agent-run status, the aggregate health snapshot, reports a state.
@row_add n=1010 row=contract/agent-run-status/last-error area=snapshot-paste-to-ai-integration rec=fct-mmhn4sme@1
boundary: agent-run status
item: last_error
promise: agent-run status reports last_error from a closed enum.
@row_add n=1011 row=contract/send-line/wrapper-side area=snapshot-paste-to-ai-integration rec=fct-67w3nt6h@1
boundary: the send line
item: wrapper side
promise: send-* lines whose corr is a send cursor are emitted only by the target wrapper, with side:"wrapper".
@row_add n=1012 row=contract/send-line/client-side-refusal area=snapshot-paste-to-ai-integration rec=fct-67w3nt6h@1
boundary: the send line
item: client-side send-refused
promise: The caller-side send-refused{side:"client"} has corr null.
@row_add n=1013 row=contract/send-line/cursor-uniqueness area=snapshot-paste-to-ai-integration rec=fct-67w3nt6h@1,fct-2geebg2r@2
boundary: the send line
item: cursor corr uniqueness
promise: A send cursor is a byte offset into the target's own events.ndjson, unique per target only; refusals and later sends can share it, so one send is identified by target instance, corr, conn and rpc_id.
@row_add n=1014 row=contract/send-line/unconfirmable area=snapshot-paste-to-ai-integration rec=fct-67w3nt6h@1
boundary: the send line
item: unconfirmable send
promise: An unconfirmable send logs one send-issued and one send-confirmed{confirmed:false} under the same corr, with no send-refused and no hook side.
@row_add n=1015 row=contract/diagnostics-line/corr-on-channel-lines area=trace-context-propagation rec=fct-2geebg2r@2
boundary: the diagnostics line
item: corr on channel lines
promise: On channel lines, corr carries the JSON-RPC id; a notification such as hook.event has no id, so its channel-request has corr null.
@row_add n=1016 row=contract/diagnostics-line/corr-on-dialog-lines area=trace-context-propagation rec=fct-2geebg2r@2
boundary: the diagnostics line
item: corr on dialog and hook lines
promise: On dialog and hook lines, corr carries the dialog_id.
@row_add n=1017 row=contract/diagnostics-line/corr-on-send-lines area=trace-context-propagation rec=fct-2geebg2r@2
boundary: the diagnostics line
item: corr on send lines
promise: On send lines, corr carries the send cursor; a send refused before send-issued carries the target's events.ndjson end offset at refusal, a log join key only, never returned to the caller.
@row_add n=1018 row=contract/channel-params/conn area=trace-context-propagation rec=fct-2geebg2r@2
boundary: viola-channel request params
item: conn
promise: The client sends conn = "<process>-<pid>-<t0>-<n>" in params (t0 the process start in epoch milliseconds, n a per-process connection counter); both sides log it, and the server strips it before dispatch, never an input to a method.
@row_add n=1019 row=contract/channel-line/srv-conn area=trace-context-propagation rec=fct-2geebg2r@2
boundary: the channel line
item: srv_conn
promise: A server receiving no conn, or one outside ^[a-z]+-\d+-\d+-\d+$ or over 64 bytes, logs srv_conn = "srv-<accept counter>" instead.
@row_add n=1020 row=decision/channel-join-key-conn-corr area=trace-context-propagation rec=fct-2geebg2r@2
name: Channel join key is conn and corr
rule: The merged-log join key for channel lines is (conn, corr); instance is deliberately not part of it.
why: Each side logs its own instance, so the two sides of one call carry different instance values.
@row_add n=1021 row=decision/conn-includes-start-instant area=trace-context-propagation rec=fct-2geebg2r@2
name: Connection id includes the process start instant
rule: conn includes the process's start instant besides the pid and a counter, so it is unique across the session home.
why: PIDs are recycled, and every hook opens one connection with the same first id, so two hooks from a reused PID would share (conn, corr).
@row_add n=1022 row=contract/send-line/conn-rpc-id area=trace-context-propagation rec=fct-2geebg2r@2
boundary: the send line
item: conn and rpc_id
promise: Every wrapper send-* line carries conn (or srv_conn) and rpc_id, and (conn, rpc_id) equals (conn, corr) on that call's channel-request and channel-response on both sides.
@row_add n=1023 row=contract/channel-line/from-trust area=trace-context-propagation rec=fct-2geebg2r@2
boundary: the channel line
item: from_trust
promise: from is logged with the constant from_trust:"self-reported" and is never treated as identity.
@row_add n=1024 row=decision/hook-correlation-by-fields area=trace-context-propagation rec=fct-hg74abkb@1
name: Hook lines correlated by fields
rule: Hooks receive no inbound trace context from Claude Code; hook lines are correlated by instance plus dialog_id for dialog hooks, or by instance plus hook_event plus timestamp.
@row_add n=1025 row=decision/span-carried-into-tasks-and-threads area=trace-context-propagation rec=fct-pfzaam6b@1
name: Span carried into tasks and threads
rule: Spawned async tasks are instrumented with their span, and std thread closures take a cloned span and run inside it.
@row_add n=1026 row=decision/explicit-correlation-values area=trace-context-propagation rec=fct-pfzaam6b@1
name: Correlation values passed explicitly
rule: corr, conn and instance are passed as explicit values into each thread or task and set on each event, never recovered from span context.
why: Spans are not serialized.
@row_add n=1027 row=decision/pty-byte-stream-never-logged area=1-obs-scope-summary rec=fct-m24p7svx@2,fct-t2iihkln@3,fct-jzzdh6ip@2
name: PTY byte stream never logged
rule: The PTY byte stream is never logged; the viola-pty seam is instrumentable, and the openpty OS API behind it is boundary-only.
why: The PTY byte stream is user content.
@row_add n=1028 row=decision/drift-reports-diagnostics-only area=1-obs-scope-summary rec=fct-gira4fst@1,fct-73ei4s3r@2,fct-add24jhw@1
name: Drift reports go to diagnostics only
rule: Drift reports of viola-agent-claude go to diagnostics/ only and nowhere else.
why: The outputs of viola-agent-claude are the most sensitive in the product.
@row_add n=1029 row=decision/claude-internals-not-a-contract area=1-obs-scope-summary rec=fct-avrouaxx@1,fct-ngecoymf@1,fct-t3jwc6wb@1
name: Wrapped claude internals are not a contract
rule: The wrapped claude CLI is a third-party binary run unmodified and instrumented at its boundary only (spawn and exit, paste writes, the vt100 readiness-gate outcome); its internal state and transcripts are never read as a contract.
@row_add n=1030 row=decision/event-payload-not-in-process-logs area=1-obs-scope-summary rec=fct-5wgriooq@1,fct-a3522pml@1
name: Event payloads kept out of process logs
rule: The payload content of events.ndjson is never copied into process-log lines.
why: events.ndjson is itself the product audit stream.
@row_add n=1031 row=decision/absolute-paths-scrubbed-externally area=1-obs-scope-summary rec=fct-5wgriooq@1,fct-a3522pml@1
name: Absolute paths scrubbed from external output
rule: Absolute paths in snapshot.json (endpoint, pinned_bin) and in config values are scrubbed from external output and allowed in diagnostics/.
@row_add n=1032 row=contract/log-line/process area=1-obs-scope-summary rec=fct-df2jtbjl@1,fct-cmvi57xd@1,fct-3i3k7olh@1
boundary: every log line
item: process
promise: Every log line carries a process key naming its role: one of run, hook, mcp, cli.
@row_add n=1033 row=decision/hook-log-write-failure-swallowed area=1-obs-scope-summary rec=fct-qlfp3c3h@1,fct-z66kg5vk@1,fct-evvm5jqh@1
name: Hook log write failure swallowed
rule: A failed write to the hook log is swallowed, so the hook still exits 0 and never writes to stderr.
@row_add n=1034 row=decision/mcp-rejections-codes-only area=1-obs-scope-summary rec=fct-mawfkjfn@1,fct-276b34sw@1,fct-4bhscgzh@1
name: MCP rejections carry codes only
rule: MCP isError results and rejected tool calls carry codes only (refusal, detail, instance-unreachable, wrapper-fault, invalid-params), with no upstream text, absolute paths or type names.
@row_add n=1035 row=decision/one-write-per-log-line area=1-obs-scope-summary rec=fct-ccfaed7l@1,fct-kv6frzyp@2,fct-he3oql74@1
name: One write per log line
rule: Each formatted log line is emitted as a single write on an append-mode handle.
why: Concurrent hook processes share one file.
@row_add n=1036 row=contract/diagnostics-files/run area=1-obs-scope-summary rec=fct-igz2gfrn@1,fct-zlgd74pd@1
boundary: the diagnostics files
item: run
promise: Role run writes <home>/diagnostics/run-<name>.ndjson.
@row_add n=1037 row=contract/diagnostics-files/hook area=1-obs-scope-summary rec=fct-igz2gfrn@1,fct-zlgd74pd@1
boundary: the diagnostics files
item: hook
promise: Role hook writes <home>/diagnostics/hook-<name>.ndjson, shared by concurrent hook processes; a write failure is swallowed.
@row_add n=1038 row=contract/diagnostics-files/mcp area=1-obs-scope-summary rec=fct-igz2gfrn@1,fct-zlgd74pd@1
boundary: the diagnostics files
item: mcp
promise: Role mcp writes <home>/diagnostics/mcp.ndjson, which never carries user content, with stderr JSON only when the file cannot be opened.
@row_add n=1039 row=contract/diagnostics-files/cli area=1-obs-scope-summary rec=fct-igz2gfrn@1,fct-zlgd74pd@1
boundary: the diagnostics files
item: cli
promise: Role cli writes <home>/diagnostics/cli-<name>.ndjson, only when an instance resolves.
@row_add n=1040 row=decision/no-log-rotation area=1-obs-scope-summary rec=fct-igz2gfrn@1,fct-z22o2ytp@1,fct-575hof4h@1,fct-khwnpdov@2
name: No log rotation
rule: There is no rotation of events.ndjson or diagnostics/*.ndjson in v1, retention being the session home's lifetime, and any future diagnostics/ rotation must keep 0600/0700 and never touch events.ndjson offsets.
@row_add n=1041 row=absence/otlp-receiver area=1-obs-scope-summary rec=fct-q2zh72zo@2,fct-x2cjy5mn@1
capability: An OTLP or pulse-class telemetry receiver
@row_add n=1042 row=decision/send-text-never-logged area=1-obs-scope-summary rec=fct-hverrr7w@1,fct-xnd434rh@1,fct-gpdjinuy@1,fct-gtr2jsdi@1
name: Send text never logged
rule: Sent text never appears on stderr, in --json output, in MCP results or in any process-log field outside instances/<name>/diagnostics/.
@row_add n=1043 row=decision/typed-keys-never-logged area=1-obs-scope-summary rec=fct-iks5yrbc@1,fct-3y3q6wvi@1,fct-ybmmsj4h@1,fct-ylcomffg@1
name: Typed keys never logged
rule: Keys a human types never appear in a log line; the wheel product event in events.ndjson is the audit record.
@row_add n=1044 row=decision/obs-deps-tokio-free area=1-obs-scope-summary rec=fct-r2s52jjf@1,fct-3og7ujbn@2
name: Obs dependencies Tokio-free and C-free
rule: Obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass cargo deny.
guard: cargo deny
@row_add n=1045 row=decision/distinct-detail-code-per-exit-cause area=1-obs-scope-summary rec=fct-y6qmqcu2@1,fct-oiibon7o@2
name: Distinct detail code per exit cause
rule: Each cause behind exit 21 and exit 1 gets a distinct detail code with no path or pid.
except: A strict-modes check refused before any log opens writes no line; its cause is the fixed stderr pair or the --json document.
@row_add n=1046 row=decision/no-external-error-reporter area=2-telemetry-strategy rec=fct-i24bcxka@1,fct-6c7xwzms@1
name: No external error reporter
rule: Errors have no external reporter.
why: Security says no outbound network calls.
@row_add n=1047 row=decision/no-event-payloads-in-process-logs area=2-telemetry-strategy rec=fct-i24bcxka@1,fct-mtgjpoqf@1
name: No event payloads in process logs
rule: Obs never copies event payloads into process logs.
@row_add n=1048 row=decision/no-w3c-traceparent area=2-telemetry-strategy rec=fct-pcsvls55@1,fct-dhcjb4zv@1
name: No W3C traceparent in v1
rule: W3C traceparent is not used in v1; correlation goes through corr plus conn.
why: No consumer exists and channel params declare no trace-context field.
@row_add n=1049 row=decision/span-naming area=2-telemetry-strategy rec=fct-gyyrakol@1,fct-osius4ib@1
name: Span naming as <area>.<operation>
rule: Span names are <area>.<operation> in snake_case, <area> one of run, pty, channel, state, hook, statusline, send, wait, last, answer, pause, release, cli, mcp; cli.<verb> names the dispatch span of a short-lived verb.
@row_add n=1050 row=contract/process-log-line/event area=2-telemetry-strategy rec=fct-hliwgb3i@1,fct-evvm5jqh@1
boundary: the process log line
item: event
promise: The event field is a closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
@row_add n=1051 row=decision/derived-metric-naming area=2-telemetry-strategy rec=fct-oyi7zo4i@1,fct-27ooklk4@1
name: Derived metric naming
rule: Derived metrics, computed by jq and never instruments, are named viola.<area>.<measure>, for example viola.hook.duration_ms; no OTel semconv instruments exist.
@row_add n=1052 row=decision/channel-response-always-logged area=4-span-trace-coverage rec=fct-o5zzhzdu@1,fct-u2zbdxyd@2
name: channel-response always logged
rule: Every dispatched channel request logs its channel-response line, an early return or an error included.
why: The line is emitted from a Drop guard, so an early return or error still logs it.
@row_add n=1053 row=decision/compact-confirm-window area=4-span-trace-coverage rec=fct-o5zzhzdu@1,fct-xfymn57j@1,fct-u2zbdxyd@2
name: Compact confirmation window
rule: A /compact send waits 40 s for its compacted-session post-condition, while every other confirmed send waits 10 s.
state: provisional
@row_add n=1054 row=decision/dialog-content-never-logged area=4-span-trace-coverage rec=fct-trei2tnd@1,fct-ycxx4ij4@2,fct-jv26h5pj@1
name: Dialog content never logged
rule: Dialog questions, answers, plan text and tool input never appear in a home-level process-log line; dialog events carry codes only, and the text reaches an instance detail file only inside a chain or drift report.
@row_add n=1055 row=decision/wheel-event-is-audit-record area=4-span-trace-coverage rec=fct-ybmmsj4h@1,fct-k7yhktu7@1
name: Wheel product event is the audit record
rule: The wheel product event in events.ndjson is the audit record of a wheel change; the typed keys are never logged and the wheel transition span is only a point in time.
@row_add n=1056 row=decision/statusline-stdout-passthrough area=4-span-trace-coverage rec=fct-rwzldnx4@1,fct-v76wl5kz@1
name: Statusline stdout passes through unlogged
rule: The user's statusline stdout passes through unchanged and is never logged.
@row_add n=1057 row=decision/statusline-deadline area=4-span-trace-coverage rec=fct-rwzldnx4@1,fct-kv6frzyp@2,fct-v76wl5kz@1
name: Statusline shell-out deadline
rule: The statusline shell-out of the user's command is bounded by STATUSLINE_DEADLINE, 5 s.
state: provisional
@row_add n=1058 row=decision/env-names-only area=4-span-trace-coverage rec=fct-xnfonqro@1,fct-wa67dogf@1,fct-6xa7lf3f@2
name: Environment names only, never values
rule: The strip and every log record environment variable names only, never values, and no env map is passed to a span or event.
guard: E2
@row_add n=1059 row=decision/ledger-contents-never-logged area=4-span-trace-coverage rec=fct-g4daqt4h@1,fct-6z2owz4q@1
name: Capability ledger path and stamps never logged
rule: The capability ledger's path and its stamp contents never appear in a log line.
@row_add n=1060 row=absence/in-process-metrics-pipeline area=5-metric-coverage rec=fct-kvmjumn6@1,fct-dijwakce@1
capability: An in-process metrics pipeline, with no listener and no exporter
@row_add n=1061 row=contract/derived-metrics/viola-compat-v-rejected area=5-metric-coverage rec=fct-sam7sdeg@1,fct-edmo55x3@1,fct-iynwva54@1
boundary: derived metrics
item: viola.compat.v_rejected
promise: Counts version rejections alone, reading channel-response with error_code -32602 and detail unsupported-version; other -32602 responses for invalid send params, a bad link or unlink name or pair, or release carrying from carry another detail word.
@row_add n=1062 row=contract/log-line/corr area=6-log-coverage rec=fct-qaeihrqm@1,fct-3i3k7olh@1,fct-dhc5cdwv@1
boundary: every log line
item: corr
promise: Set per the tests' table and copied unchanged as a JSON number or string; null is represented by the key's absence.
@row_add n=1063 row=absence/grpc area=6-log-coverage rec=fct-oqqvdowc@1,fct-nvdezwhb@1
capability: gRPC of any kind
@row_add n=1064 row=decision/no-sentry-or-otel-bridge area=7-error-capture-reporting rec=fct-kd3a4grv@1
name: No Sentry or OTel bridge
rule: Errors have no Sentry or OTel bridge; they correlate by corr, conn, process and instance, not by trace_id or span_id.
why: Both were deferred or rejected.
@row_add n=1065 row=decision/home-level-lines-path-free area=8-pii-scrubbing-compliance rec=fct-qk7hj63u@1,fct-syepqt4n@1
name: Home-level lines path-free
rule: Home-level log lines are path-free: file fields are basenames, panic_location is workspace-relative and source file recording is off; absolute paths appear only in detail files.
why: The OS username inside absolute paths is the only PII in scope.
@row_add n=1066 row=absence/otel-span-processing area=8-pii-scrubbing-compliance rec=fct-xpgyosmo@1
capability: An OTel SpanProcessor
@row_add n=1067 row=decision/secret-scan-before-upload area=8-pii-scrubbing-compliance rec=fct-tcjwyf2p@1,fct-ydc7ueix@3
name: Secret scan before artifact upload
rule: CI runs the secret scan for the Cookie header, CLAUDE* values, the content canary and 0600 over every diagnostics/ file before any artifact upload.
guard: secret-scan test
@row_add n=1068 row=decision/g1-bare-instrument-gate area=9-ci-integration rec=fct-a4sgiwyc@1,fct-5kn2jpbt@1
name: G1 bare-instrument gate fails closed
rule: Every #[instrument] must have skip_all as its first argument; the G1 gate passes only on rg exit 1 (no match), so a violation, a regex or I/O error or a missing rg fails, and a G1 failure is an SLO invariant failure.
guard: G1
@row_add n=1069 row=decision/uploads-gated-on-secret-scan area=9-ci-integration rec=fct-cstqzpm2@1,fct-safgt2vp@1
name: CI uploads gated on the secret scan
rule: The diag, harness and junit CI artifact uploads run only when the secret scan, which always runs, succeeded.
guard: secret-scan
@row_add n=1070 row=decision/zero-unlogged-panics area=10-slo-invariants-telemetry-budgets rec=fct-3qkbv2ca@1,fct-wgmneptw@1,fct-rjubzxp3@1,fct-6543go25@1
name: Zero unlogged panics
rule: Every panic in run, hook, mcp and cli after init step 4 produces exactly one single-line ERROR panic record in its home-level role file, with no multi-line backtrace there, and hook still exits 0; retry-once policies for panic handling are forbidden.
guard: G2
except: the bounded exemptions where no file exists, so no line can be written; G2 alone leaves out the fake-agent seam's deliberate panic from its count.
@row_add n=1071 row=decision/no-sleep-based-log-waits area=10-slo-invariants-telemetry-budgets rec=fct-ta6vgo2y@1,fct-jz5zhprx@1
name: No sleep-based log waits
rule: No log-based wait relies on sleeps.
@row_add n=1072 row=decision/hook-obs-overhead-budget area=10-slo-invariants-telemetry-budgets rec=fct-onnnq5uj@1,fct-bxbg45yr@1
name: Hook observability overhead budget
rule: Obs overhead on hook, logger init being one open plus appends, is held inside the same max < 1.0 s perf gate, which holds only when every hook command runs with VIOLA_NAME set and a home under target/e2e-home.
guard: gate --require perf
@row_add n=1073 row=decision/no-print-macros area=11-obs-anti-patterns-never-do-these rec=fct-l2iw7ho4@1,fct-xpak76xj@2,fct-hfdtlj6j@2,fct-safgt2vp@1
name: No print macros in product code paths
rule: print!, println!, eprint!, eprintln! and dbg! are never used in the hook, run or mcp code paths of the product crates, and every product member inherits the workspace lints.
why: They bypass the file sink and break the Hook contract, the run terminal rule and MCP framing.
guard: clippy print_stdout, print_stderr and dbg_macro at deny
except: viola-harness, the fake agent and the output modules that own --json, human CLI stderr and the hook decision body.
@row_add n=1074 row=decision/pinned-actions area=11-obs-anti-patterns-never-do-these rec=fct-x6iew4rz@1
name: Actions pinned by commit SHA
rule: Actions are never used unpinned; actions/upload-artifact is pinned by commit SHA.
guard: zizmor
@row_add n=1075 row=absence/otel-trace-export area=ci-integration rec=fct-dajf6x5e@1
capability: OTel trace export: no exporter
@row_add n=1076 row=contract/cli-internal-error/json-document area=error-capture-reporting rec=fct-vfnyc5f5@2,fct-oiibon7o@2
boundary: cli internal error
item: --json document
promise: Under --json, read among the arguments before a --, the cli catch site prints {"v":1,"error":"internal-error","detail":null} on stdout and stderr stays empty.
@row_add n=1077 row=contract/mcp-stdin/oversize-line area=error-capture-reporting rec=fct-iy5crjhq@1,fct-44go5tc2@1,fct-lpbmjfvf@1,fct-g4wb4nmr@1
boundary: viola mcp stdin
item: line past MAX_FRAME
promise: A line past MAX_FRAME is refused whole, gives no isError result and ends the session as a closed stdin: exit 0 after initialize, exit 1 before it with both streams empty; its one record is parse-rejected mcp-stdin oversize, no byte of the line.
@row_add n=1078 row=contract/hook-decision-detail/server-verify-failed area=log-coverage rec=fct-qiql34nd@2,fct-kohas2np@2,fct-ffumo3bh@1,fct-n6l5o3fm@2
boundary: hook-decision detail
item: server-verify-failed
promise: viola hook writes it at WARN on a server of another euid, a server that is not the recorded wrapper, or a check that cannot be made; no frame is sent, the hook fails open, and on the dialog path it follows hook-invoked with decision_emitted false.
@row_add n=1079 row=contract/hook-decision-detail/channel-unreachable area=log-coverage rec=fct-qiql34nd@2,fct-kohas2np@2
boundary: hook-decision detail
item: channel-unreachable
promise: Written when the channel cannot be reached, a replayed or absent snapshot included; no frame is sent, the hook fails open and SessionEnd takes its direct append.
@row_add n=1080 row=contract/cli-process-exit/server-verify-failed area=metric-coverage rec=fct-l3f7hrdu@2,fct-n7xftqkr@2,fct-ydc7ueix@3
boundary: CLI process-exit diagnostic line
item: server-verify-failed
promise: The CLI's server-verification refusal writes one process-exit with subject self, exit_code 21, detail server-verify-failed and during connect at WARN in cli-<name>.ndjson, with no pid, start time, path or endpoint; MCP's wait tool writes no line for it.
@row_add n=1081 row=decision/no-process-log-without-instance area=obs-scope-summary rec=fct-oy7tgncm@1,fct-f3gi2vhc@2
name: No process log without an instance
rule: A short-lived CLI verb run with no resolved instance writes no process-log file; its human stderr carries only fixed-message lines, and its outcome is the exit code and stdout.
@row_add n=1082 row=decision/strict-modes-refusal-writes-no-line area=obs-scope-summary rec=fct-oy7tgncm@1,fct-oiibon7o@2,fct-foy4up7k@1,fct-h2wm2goy@2,fct-mxwnyc6t@2,fct-5ckzuzgq@2
name: Strict-modes refusal writes no line
rule: A strict-modes check refused before any log opens ends the process with nothing under the home opened and no log line; its cause is carried only by the fixed stderr pair or the --json document.
why: Ratified as the answer to whether a refused strict-modes check logs.
except: The ledger-stamps read in run's version gate and a refused update_stamps in viola verify each leave a line.
@row_add n=1083 row=contract/service-identity/service-name area=obs-scope-summary rec=fct-uwlbnhdg@1,fct-plkmesmz@1
boundary: Service identity
item: service.name
promise: Hardcoded "viola", the binary name, never read from OTEL_SERVICE_NAME or SERVICE_NAME.
@row_add n=1084 row=decision/no-secrets-in-log-lines area=obs-scope-summary rec=fct-ogkwmfvl@1,fct-evvm5jqh@1
name: No secrets in log lines
rule: No log line may contain a Cookie header or any stripped CLAUDE* value.
guard: secret-scan test
@row_add n=1085 row=contract/channel-response/detail area=obs-scope-summary rec=fct-m4ttxbfi@1,fct-jlmrrvwc@1,fct-plg35lpj@1,fct-exgbnztg@1,fct-fm6zc2nz@1,fct-gghzntp4@2
boundary: Channel response line
item: detail
promise: On a channel-response answering -32602, detail is one closed word: unsupported-version, invalid-params or release-from-driver, on both the server's and the client's line; it is absent on every other channel-response line and names no param or value.
@row_add n=1086 row=contract/hook-decision-detail/socket-dir-refused area=span-trace-coverage rec=fct-elpncyv4@1,fct-ffumo3bh@1,fct-n6l5o3fm@2,fct-ne5xp5cg@2
boundary: hook-decision line
item: socket-dir-refused
promise: A refused socket directory writes one hook-decision{detail:"socket-dir-refused"} at WARN, on the dialog path after hook-invoked{corr:null}, with decision_emitted:false; exit 0, empty streams, no frame.
@row_add n=1087 row=contract/config-json/dialog-hold-s area=span-trace-coverage rec=fct-uicv6khr@1,fct-6bbewo2y@2
boundary: config.json
item: dialog_hold_s
promise: The dialog hold, a whole number of seconds from 60 to 3600, default 60; any other value gives one parse-rejected malformed WARN line and the hold takes 60.
