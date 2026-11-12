@row_add n=1 row=decision/mcp-stdio-server-tokio-allowed area=1-obs-scope-summary rec=1
name: MCP server on stdio with Tokio allowed
rule: The `viola mcp` surface is an internal stdio MCP server, and Tokio is allowed in it.
@row_add n=2 row=contract/mcp-stdin/max-frame area=1-obs-scope-summary rec=1
boundary: viola mcp stdin
item: MAX_FRAME
promise: Stdin is read in lines of at most `MAX_FRAME`; a longer line is refused whole and the session ends as at a closed stdin.
@row_add n=3 row=decision/mcp-log-file-sink area=1-obs-scope-summary rec=2
name: MCP logs to a file sink, never stdout
rule: The `mcp` process always logs to the file sink `<home>/diagnostics/mcp.ndjson`, with stderr JSON only when that file cannot be opened, and the log writer is set explicitly.
why: stdout is forbidden because it carries MCP frames
@row_add n=4 row=decision/mcp-rejections-codes-only area=1-obs-scope-summary rec=2,39
name: MCP rejections carry codes only
rule: MCP `isError` results and the log lines for rejected tool calls carry codes only, with no upstream text, absolute paths or type names.
@row_none n=1 rec=3 type=S
@row_add n=5 row=decision/no-otel-sdk-exporter area=1-obs-scope-summary rec=4
name: No OTel SDK exporter
rule: No OTel SDK exporter is initialised; any OTel bridge may only feed an in-process or file sink and must be Tokio-free.
why: outbound network calls are banned, a new listener needs a Security Decisions Log entry, stdout is reserved on hook, mcp and run, and cargo deny bans Tokio in sync crates
except: an optional Tokio-free OTel bridge feeding an in-process or file sink
@row_add n=6 row=decision/tracing-json-pipeline area=1-obs-scope-summary rec=4,6
name: tracing with the tracing-subscriber JSON formatter
rule: The `tracing` facade plus the tracing-subscriber JSON formatter is the binding logging pipeline, and JSON-to-file is the only logging mode.
@row_add n=7 row=contract/diag-line/service-name area=1-obs-scope-summary rec=5
boundary: diagnostic log line
item: service.name
promise: Hardcoded to "viola".
@row_add n=8 row=contract/diag-line/service-version area=1-obs-scope-summary rec=5
boundary: diagnostic log line
item: service.version
promise: The crate version, fixed at compile time.
@row_add n=9 row=contract/diag-line/process area=1-obs-scope-summary rec=5
boundary: diagnostic log line
item: process
promise: The role of the writing process, one of run, hook, mcp, cli; required on every line and matching `sender` / `writer`.
@row_add n=10 row=contract/diag-line/instance area=1-obs-scope-summary rec=5
boundary: diagnostic log line
item: instance
promise: The instance's `ViolaName`, or null; required on every line.
@row_add n=11 row=decision/one-log-file-per-role area=1-obs-scope-summary rec=6
name: One log file per process role
rule: Each process role writes its own file under `diagnostics/`, no role has a stdout sink, and stderr JSON is used only by `mcp` when its file cannot be opened.
@row_add n=12 row=decision/one-write-per-log-line area=1-obs-scope-summary rec=6
name: One write per log line
rule: Each log line is written with one `write` on an append-mode handle.
@row_add n=13 row=decision/level-filter-not-from-env area=1-obs-scope-summary rec=6
name: Level filter never read from RUST_LOG
rule: The log level filter is never read from `RUST_LOG`.
@row_add n=14 row=decision/redaction-not-disableable area=1-obs-scope-summary rec=6,42
name: Redaction cannot be disabled
rule: No verbosity control, env, flag or config may disable or bypass redaction.
@row_add n=15 row=decision/diag-schema-additive-only area=1-obs-scope-summary rec=7
name: Diagnostic log schema is additive only
rule: The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove any field.
@row_add n=16 row=contract/diagnostics-files/run area=1-obs-scope-summary rec=8
boundary: diagnostics log files
item: run-<name>.ndjson
promise: The log file of `run` for one instance, under `<home>/diagnostics/`.
@row_add n=17 row=contract/diagnostics-files/hook area=1-obs-scope-summary rec=8
boundary: diagnostics log files
item: hook-<name>.ndjson
promise: The log file of `hook` for one instance, under `<home>/diagnostics/`.
@row_add n=18 row=contract/diagnostics-files/mcp area=1-obs-scope-summary rec=8
boundary: diagnostics log files
item: mcp.ndjson
promise: The log file of `mcp`, at home level under `<home>/diagnostics/`; it never carries user content.
@row_add n=19 row=contract/diagnostics-files/cli area=1-obs-scope-summary rec=8
boundary: diagnostics log files
item: cli-<name>.ndjson
promise: The log file of CLI verbs for one instance, under `<home>/diagnostics/`.
@row_add n=20 row=decision/diagnostics-owner-only-modes area=1-obs-scope-summary rec=8,45
name: Diagnostics files are owner-only
rule: Every diagnostics file is mode 0600 inside a 0700 directory.
@row_add n=21 row=decision/no-log-rotation area=1-obs-scope-summary rec=8
name: No log rotation
rule: Diagnostics log files are not rotated.
@row_add n=22 row=absence/otlp-receiver area=1-obs-scope-summary rec=9
capability: An OTLP or pulse-class telemetry receiver
@row_add n=23 row=contract/harness-logs/merge area=1-obs-scope-summary rec=9
boundary: harness logs command
item: merged sources
promise: Merges `events.ndjson` lines (`src:"events"`, with byte `offset`) with `diagnostics/*.ndjson` lines (`src:"diag"`).
@row_add n=24 row=contract/harness-logs/torn-line area=1-obs-scope-summary rec=9
boundary: harness logs command
item: torn lines
promise: A torn line is emitted as `{"torn":true}`.
@row_add n=25 row=contract/harness-logs/filters area=1-obs-scope-summary rec=9
boundary: harness logs command
item: filters
promise: Output can be filtered by `--instance`, `--kind`, `--process` and `--after`.
@row_add n=26 row=contract/harness-logs/output area=1-obs-scope-summary rec=9
boundary: harness logs command
item: output
promise: Output is ndjson on stdout, ready for `jq` or pasting to an agent.
@row_add n=27 row=contract/agent-run-status/state-last-error area=1-obs-scope-summary rec=9
boundary: agent-run status
item: state and last_error
promise: The aggregate health snapshot gives `state` and `last_error` as a closed enum.
@row_add n=28 row=decision/no-traceparent-on-local-socket area=1-obs-scope-summary rec=10
name: No W3C traceparent on the local socket
rule: Calls from CLI, MCP and hook to the wrapper carry no W3C `traceparent`; correlation uses `corr` (JSON-RPC `id`, `dialog_id`, send `cursor`) plus `process` and `instance`.
why: channel `params` declare no trace-context field
@row_add n=29 row=decision/hook-stdin-correlation area=1-obs-scope-summary rec=10
name: Hook stdin correlated by instance and dialog
rule: Hook stdin from Claude Code carries no inbound trace context, so it is correlated by `instance` and `dialog_id`.
@row_add n=30 row=decision/liveness-logged-on-change area=1-obs-scope-summary rec=11
name: Liveness transitions logged on change only
rule: The `run` heartbeat file touched every second is the tick; liveness transitions (live, stale, gone) are logged on change only, never as a per-second line.
why: a 1 s log line would grow a never-rotated file without bound
@row_none n=2 rec=12 type=S
@row_add n=31 row=decision/spans-not-wire-contract area=1-obs-scope-summary rec=13
name: Tracing spans are not part of the wire contract
rule: Span names are internal `tracing` spans in `<area>.<operation>` snake_case form; the agent-facing contract is the closed `event` enum plus `corr`.
why: the config sets `with_current_span(false)`, so spans do not reach the line
@row_add n=32 row=decision/exit-1-detail-codes area=1-obs-scope-summary rec=15,14
name: Distinct detail codes for exit-1 causes
rule: On the `viola run` start sequence to child spawn, each exit-1 cause logs a distinct `detail` code with no path or pid in it.
@row_add n=33 row=decision/send-text-never-logged area=1-obs-scope-summary rec=17,16
name: Sent text never logged on send
rule: On a confirmed `send` from driver to readback, the sent text never appears in log fields.
@row_add n=34 row=decision/assistant-message-never-logged area=1-obs-scope-summary rec=19,18
name: Assistant message never logged on readback
rule: On `wait` / `last` readback, the `last_assistant_message` content never appears in log fields.
@row_add n=35 row=decision/dialog-text-never-logged area=1-obs-scope-summary rec=21,20
name: Dialog text never logged on answer
rule: On the dialog to `answer` path, the question, answer and tool `input` text never appear in log fields.
@row_add n=36 row=decision/typed-keys-never-logged area=1-obs-scope-summary rec=23,22
name: Typed keys never logged
rule: When the human takes the wheel and automation is refused until `release`, typed keys never appear in log fields.
@row_add n=37 row=decision/statusline-command-confined area=1-obs-scope-summary rec=25,24
name: Statusline command confined to diagnostics
rule: On the budget governor path, the `statusline_command` string never appears outside `diagnostics/`.
@row_none n=3 rec=26 type=9
@row_none n=4 rec=27 type=S
@row_none n=5 rec=28 type=S
@row_none n=6 rec=29 type=S
@row_add n=38 row=decision/channel-lines-carry-no-bodies area=1-obs-scope-summary rec=30
name: Channel log lines carry no bodies
rule: IPC channel log lines never carry `params` or result bodies.
@row_add n=39 row=decision/channel-from-unauthenticated area=1-obs-scope-summary rec=30
name: Channel sender logged as unauthenticated
rule: The `from` of an IPC channel call is logged as unauthenticated.
@row_none n=7 rec=31 type=S
@row_none n=8 rec=32 type=S
@row_add n=40 row=decision/send-text-confined area=1-obs-scope-summary rec=33
name: Send text confined to instance diagnostics
rule: `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
@row_none n=9 rec=34 type=S
@row_none n=10 rec=35 type=S
@row_add n=41 row=decision/hook-fail-open-silent area=1-obs-scope-summary rec=36
name: Hook fails open silently
rule: Hook fail-open paths log only to `hook-<name>.ndjson` and exit 0 with stderr empty.
@row_add n=42 row=decision/drift-reports-to-diagnostics area=1-obs-scope-summary rec=36
name: Drift reports go to diagnostics only
rule: Hook drift reports are written to `diagnostics/` only.
@row_none n=11 rec=37 type=S
@row_none n=12 rec=38 type=S
@row_none n=13 rec=40 type=S
@row_none n=14 rec=41 type=S
@row_add n=43 row=decision/stripped-env-never-logged area=1-obs-scope-summary rec=42
name: Stripped CLAUDE env values never logged
rule: No R8-stripped `CLAUDE*` value appears in any sink or fixture; the strip is logged as a count or variable names only.
@row_add n=44 row=decision/serde-errors-fixed-message area=1-obs-scope-summary rec=42
name: Serde error chains become a fixed message
rule: A serde-sourced anyhow error chain is replaced by a fixed message before it reaches stderr.
@row_none n=15 rec=43 type=S
@row_none n=16 rec=44 type=S
@row_add n=45 row=decision/refusal-telemetry area=1-obs-scope-summary rec=45
name: Refusal telemetry for filesystem refusals
rule: Pinned-exe hash mismatches and ignored symlinks produce refusal telemetry.
@row_add n=46 row=decision/strict-modes-failure-lines area=1-obs-scope-summary rec=45
name: Where a strict-modes failure is logged
rule: A strict-modes failure has a log line only on the `ledger-stamps` read in the `run` version gate and on `verify`'s own `process-exit` for a refused stamp write.
@row_add n=47 row=decision/refused-diagnostics-drops-line area=1-obs-scope-summary rec=45
name: Refused diagnostics file drops its line
rule: A refused `diagnostics/` folder or file is treated as a failed open and its log line is dropped.
@row_add n=48 row=contract/process-exit-detail/strict-modes-failed area=1-obs-scope-summary rec=45
boundary: process-exit detail codes
item: strict-modes-failed
promise: Carried by `verify`'s `process-exit` for a refused stamp write, with exit 1.
@row_add n=49 row=contract/process-exit-detail/socket-dir-refused area=1-obs-scope-summary rec=45
boundary: process-exit detail codes
item: socket-dir-refused
promise: Carried by the line of a refused socket directory, where it has one; a refused socket directory is no strict-modes failure.
@row_none n=17 rec=46 type=S
@row_none n=18 rec=47 type=S
@row_add n=50 row=decision/parse-failures-unknown-counter area=1-obs-scope-summary rec=48
name: Parse failures surface as unknown plus a counter
rule: A child-output parse failure surfaces as `unknown` plus a counter.
@row_add n=51 row=decision/escape-controls-in-terminal area=1-obs-scope-summary rec=48
name: Control characters escaped in terminal rendering
rule: C0 and C1 control characters are escaped in any human terminal rendering of logged rows.
@row_none n=19 rec=49 type=S
@row_none n=20 rec=50 type=S
@row_add n=52 row=decision/obs-deps-tokio-free area=1-obs-scope-summary rec=51
name: Observability dependencies Tokio-free and C-free
rule: Observability dependencies stay Tokio-free in the sync crates, add no C-building crates and pass `cargo deny`.
guard: cargo deny
@row_none n=21 rec=52 type=S
@row_none n=22 rec=53 type=S
@row_none n=23 rec=54 type=S
@row_none n=24 rec=55 type=S
