@row_add n=1 row=decision/manual-spans-only area=4-span-trace-coverage rec=1
name: Manual spans only
rule: Every span is written by hand; no surface uses an auto-instrumentation library.
@row_add n=2 row=decision/dependency-targets-off-in-sink area=4-span-trace-coverage rec=1
name: rmcp and portable-pty targets off
rule: The rmcp and portable-pty log targets are switched OFF in the sink.
@row_add n=3 row=decision/channel-lines-no-params-or-results area=4-span-trace-coverage rec=1
name: Channel lines carry no params or results
rule: Channel log lines never record request params or result bodies.
@row_add n=4 row=decision/uninstrumented-components area=4-span-trace-coverage rec=2
name: Components left uninstrumented
rule: viola-core, config.json and plugin files, and build/CI infrastructure are not instrumented.
why: viola-core only supplies ObsEvent and RefusalReason values; the rest are not instrumentable.
@row_add n=5 row=decision/no-trace-and-span-ids area=4-span-trace-coverage rec=3,46,45
name: No trace_id or span_id
rule: Log lines never emit trace_id or span_id; correlation is by corr plus conn.
why: No W3C trace context exists, and spans are not attached to lines.
@row_add n=6 row=absence/web-or-graphical-frontend area=4-span-trace-coverage rec=3
capability: A web or graphical frontend
@row_none n=1 rec=4 type=5
@row_add n=7 row=decision/no-endpoint-path-or-env-in-spans area=4-span-trace-coverage rec=5
name: No endpoint, binary path or env values in spans
rule: A span never carries the endpoint name, the pinned_bin path or environment values.
@row_add n=8 row=decision/child-settings-never-logged area=4-span-trace-coverage rec=5
name: Child settings argument never logged
rule: The claude child's --settings argument appears in no log line or span.
@row_add n=9 row=decision/statusline-command-never-logged area=4-span-trace-coverage rec=5,15
name: Statusline command never logged
rule: The statusline command and its source path appear in no log line or span.
@row_add n=10 row=decision/detail-codes-only area=4-span-trace-coverage rec=5,50
name: detail is a closed code
rule: The detail field is a closed kebab-case code catalog and never carries a path or pid.
@row_none n=2 rec=6 type=5
@row_add n=11 row=decision/sent-text-never-logged area=4-span-trace-coverage rec=7
name: Sent text never logged
rule: A sent message's text is never logged; only its size in text_bytes is.
@row_add n=12 row=decision/channel-response-always-logged area=4-span-trace-coverage rec=7
name: channel-response always logged
rule: Every dispatched channel request logs its channel-response line, even on an early return or an error.
why: The line is emitted from a Drop guard, so no exit path can skip it.
@row_add n=13 row=decision/compact-confirm-window area=4-span-trace-coverage rec=7
name: Compact confirmation window
rule: A /compact send is confirmed within a 40 s window.
state: provisional
@row_none n=3 rec=8 type=5
@row_add n=14 row=decision/no-assistant-message-in-logs area=4-span-trace-coverage rec=9
name: No assistant message or result body in logs
rule: last_assistant_message never appears in a log, and no result body reaches a log line.
@row_add n=15 row=decision/mid-call-disconnect-logged-once area=4-span-trace-coverage rec=9
name: Mid-call disconnect logged once
rule: A disconnect in the middle of a call is logged once, by the client.
@row_add n=16 row=contract/cli-wait-last-connect/server-verify-failed area=4-span-trace-coverage rec=9
boundary: CLI wait / last connect refusal
item: server-verify-failed
promise: A serving process that is not the recorded wrapper or runs under another euid ends the call before any frame with exit 21 and one WARN process-exit line (subject self, during connect) inside the dispatch span, and no channel-request line.
@row_add n=17 row=contract/cli-wait-last-connect/socket-dir-refused area=4-span-trace-coverage rec=9
boundary: CLI wait / last connect refusal
item: socket-dir-refused
promise: A refused socket directory, checked before the server comparison, ends the call before any frame with exit 21 and one WARN process-exit line (during connect), and no channel-request line.
@row_add n=18 row=contract/cli-wait-last-connect/strict-modes-failed area=4-span-trace-coverage rec=9
boundary: CLI wait / last connect refusal
item: strict-modes-failed
promise: A home or instance tree refused by the instance check, checked first, ends the call before any role file opens with exit 21 and no line; under --json the output is exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
@row_add n=19 row=contract/mcp-wait-last-connect/server-verify-failed area=4-span-trace-coverage rec=9
boundary: MCP wait / last connect refusal
item: server-verify-failed
promise: A serving process that is not the recorded wrapper or runs under another euid returns instance-unreachable with detail server-verify-failed and writes no line.
@row_add n=20 row=contract/mcp-wait-last-connect/socket-dir-refused area=4-span-trace-coverage rec=9
boundary: MCP wait / last connect refusal
item: socket-dir-refused
promise: A refused socket directory returns instance-unreachable with detail socket-dir-refused and writes no line.
@row_add n=21 row=contract/mcp-wait-last-connect/strict-modes-failed area=4-span-trace-coverage rec=9
boundary: MCP wait / last connect refusal
item: strict-modes-failed
promise: A home or instance tree refused by the instance check returns instance-unreachable with detail strict-modes-failed and writes no line.
@row_none n=4 rec=10 type=5
@row_add n=22 row=decision/dialog-content-never-logged area=4-span-trace-coverage rec=11
name: Dialog content never logged
rule: A dialog's question, its answer and the tool input never appear in a log line.
@row_none n=5 rec=12 type=5
@row_add n=23 row=decision/typed-keys-never-logged area=4-span-trace-coverage rec=13
name: Typed keys never logged
rule: Keys the user types are never logged.
@row_add n=24 row=decision/wheel-event-is-audit-record area=4-span-trace-coverage rec=13
name: wheel event as the audit record
rule: The wheel product event in events.ndjson is the audit record of key input.
@row_none n=6 rec=14 type=5
@row_add n=25 row=decision/statusline-stdout-passthrough area=4-span-trace-coverage rec=15
name: Statusline output passed through
rule: The user's statusline stdout passes through unchanged and is never logged.
@row_add n=26 row=decision/statusline-deadline area=4-span-trace-coverage rec=15
name: Statusline shell-out deadline
rule: The statusline shell-out is bounded by a 5 s deadline.
state: provisional
@row_add n=27 row=decision/unwrapped-hook-no-op area=4-span-trace-coverage rec=17,16
name: Unwrapped hook is a no-op
rule: A hook run without VIOLA_NAME writes no file and no line, exits 0 and leaves stdout and stderr empty.
guard: edge test E1, asserted by absence
@row_add n=28 row=contract/claude-child-process-start/env-stripped-count area=4-span-trace-coverage rec=18,16
boundary: process-start line of the claude child
item: env_stripped_count
promise: Reports the R8 environment strip on process-start{subject:"claude-child"}; never an environment value.
@row_add n=29 row=contract/claude-child-process-start/env-stripped-known area=4-span-trace-coverage rec=18,16
boundary: process-start line of the claude child
item: env_stripped_known
promise: Reports the R8 environment strip on process-start{subject:"claude-child"} by variable names only, never values.
@row_add n=30 row=contract/claude-child-process-start/env-kept area=4-span-trace-coverage rec=18,16
boundary: process-start line of the claude child
item: env_kept
promise: Reports the variables kept after the R8 environment strip on process-start{subject:"claude-child"} by names only, never values.
@row_add n=31 row=decision/env-names-only area=4-span-trace-coverage rec=18,16
name: Environment by names only
rule: Environment variables are logged by name only, never by value, and no environment map is passed to a span or event.
guard: edge test E2
@row_add n=32 row=decision/link-has-no-own-trace area=4-span-trace-coverage rec=19,16
name: Link and unlink have no trace of their own
rule: Implicit and explicit link / unlink have no process-log line, event value or span of their own; their trace is the call's channel-request / channel-response pair, the product link / unlink line in events.ndjson and the snapshot's links.
guard: edge test E3
@row_add n=33 row=decision/link-verbs-log-to-driven-instance area=4-span-trace-coverage rec=19,16
name: Link verbs log to the driven instance
rule: The viola link and viola unlink verbs log into the driven instance's cli-<driven>.ndjson, their request inside the cli.link / cli.unlink span.
@row_add n=34 row=contract/state-recovered-detail/snapshot-replayed area=4-span-trace-coverage rec=20,16
boundary: state-recovered detail
item: snapshot-replayed
promise: A state-recovered line at WARN carries this detail, one line per snapshot read call.
@row_add n=35 row=contract/state-recovered-detail/snapshot-unsupported-v area=4-span-trace-coverage rec=20,16
boundary: state-recovered detail
item: snapshot-unsupported-v
promise: A state-recovered line at WARN carries this detail, one line per snapshot read call.
@row_add n=36 row=decision/no-endpoint-over-replayed-snapshot area=4-span-trace-coverage rec=20,16
name: No endpoint over a replayed snapshot
rule: Over a replayed or absent snapshot read, endpoint lookup and the hook connect give no endpoint, so no frame is sent, and the statusline runs no command.
guard: edge test E5
@row_add n=37 row=contract/state-recovered-detail/torn-line-healed area=4-span-trace-coverage rec=21
boundary: state-recovered detail
item: torn-line-healed
promise: The appender that heals a torn line writes one state-recovered line with this detail per heal.
@row_add n=38 row=decision/reader-never-heals-torn-lines area=4-span-trace-coverage rec=21
name: Reader never heals torn lines
rule: A reader of an append log neither heals a torn line nor logs one; only the appender heals.
@row_add n=39 row=decision/unreadable-ledger-is-unverified area=4-span-trace-coverage rec=22
name: Unreadable ledger counts as unverified
rule: A missing stamp, or an unreadable, malformed or strict-modes-refused capability ledger, counts as unverified (cli_verified false).
@row_add n=40 row=decision/ledger-contents-never-logged area=4-span-trace-coverage rec=22
name: Ledger path and stamps never logged
rule: The capability ledger's path and stamp contents never appear in a log line.
@row_add n=41 row=decision/verify-logs-only-with-instance area=4-span-trace-coverage rec=23
name: verify and plugin install log only inside an instance
rule: With VIOLA_NAME set, verify and plugin install log process-start / process-exit to cli-<name>.ndjson; from a plain terminal with no instance they write no process-log file.
@row_add n=42 row=decision/capture-arms-uninstrumented area=4-span-trace-coverage rec=23
name: Hidden capture arms uninstrumented
rule: The two hidden capture arms of verify / plugin install are uninstrumented by design.
@row_add n=43 row=decision/verify-lines-redacted area=4-span-trace-coverage rec=23
name: verify lines carry no content
rule: No verify or plugin install line carries row text, a prompt, a session id or a path.
@row_add n=44 row=decision/ci-verify-fake-agent-only area=4-span-trace-coverage rec=23
name: CI verify against the fake agent only
rule: In CI, viola verify runs only against the fake agent.
@row_add n=45 row=contract/viola-verify-refusal/home-tree-refused area=4-span-trace-coverage rec=23
boundary: viola verify strict-modes refusal
item: home tree refused
promise: Before logging, verify holds the home, ledger/, bin/ and plugin/ to the strict-modes check; a refused tree exits 1 with the home pair and writes no line.
@row_add n=46 row=contract/viola-verify-refusal/stamp-write-refused area=4-span-trace-coverage rec=23
boundary: viola verify strict-modes refusal
item: stamp write refused
promise: A ledger/stamps.json the stamp write refuses prints the home pair and exits 1 through verify's own process-exit line.
@row_none n=7 rec=24 type=S
@row_none n=8 rec=25 type=S
@row_add n=47 row=absence/in-process-metrics-pipeline area=5-metric-coverage rec=26
capability: An in-process metrics pipeline (no listener, no exporter)
@row_add n=48 row=decision/metrics-derived-from-logs area=5-metric-coverage rec=26,27
name: Metrics derived from logs
rule: Every metric is logical, computed by jq or the harness over the JSON-per-line logs or from hyperfine output, never by OTel or metrics-crate instruments.
why: There is no reader or exporter to flush instruments.
@row_add n=49 row=decision/derived-metric-naming area=5-metric-coverage rec=28
name: Derived metric naming
rule: Derived metrics are named viola.<area>.<measure>; no OTel semconv instruments exist.
@row_add n=50 row=contract/derived-metrics/viola-compat-v-rejected area=5-metric-coverage rec=30,29
boundary: derived metrics
item: viola.compat.v_rejected
promise: Counts version rejections alone, reading channel-response with error_code -32602 and detail unsupported-version; other -32602 invalid-params responses carry another detail word.
@row_add n=51 row=decision/metric-grouping-closed-enums area=5-metric-coverage rec=31
name: Metric grouping by closed enums only
rule: Aggregate reports group only by closed enums (event, detail, refusal, method, hook_event, subject, parser, process); corr, conn, pid, child_pid and instance are join keys, never grouping keys.
@row_none n=9 rec=32 type=S
@row_none n=10 rec=33 type=S
@row_none n=11 rec=34 type=S
@row_add n=52 row=decision/log-schema-additive-only area=6-log-coverage rec=35
name: Log schema additive to the tests' format
rule: The observability log schema only adds fields and enum values to the tests' log format; it renames and removes nothing.
why: The tests' log format is the binding text.
@row_add n=53 row=contract/log-line/timestamp area=6-log-coverage rec=37,36
boundary: every log line
item: timestamp
promise: RFC 3339 UTC with milliseconds and Z.
@row_add n=54 row=contract/log-line/level area=6-log-coverage rec=38,36
boundary: every log line
item: level
promise: One of DEBUG, INFO, WARN, ERROR; TRACE is never emitted.
@row_add n=55 row=contract/log-line/target area=6-log-coverage rec=39,36
boundary: every log line
item: target
promise: The Rust module path, for example viola_channel::server.
@row_add n=56 row=contract/log-line/message area=6-log-coverage rec=40,36
boundary: every log line
item: message
promise: Flattened, a static literal equal to the event name.
@row_add n=57 row=contract/log-line/event area=6-log-coverage rec=41,36
boundary: every log line
item: event
promise: A value of the closed kebab-case event enum.
@row_add n=58 row=contract/log-line/corr area=6-log-coverage rec=42,36
boundary: every log line
item: corr
promise: Set per the tests' table; null is represented by the key's absence.
@row_add n=59 row=contract/log-line/process area=6-log-coverage rec=43,36
boundary: every log line
item: process
promise: One of run, hook, mcp, cli.
@row_add n=60 row=contract/log-line/instance area=6-log-coverage rec=44,36
boundary: every log line
item: instance
promise: The ViolaName, or null.
@row_add n=61 row=contract/process-start-line/service-name area=6-log-coverage rec=47,45
boundary: process-start line
item: service_name
promise: A constant service name, emitted once on process-start rather than on every line.
@row_add n=62 row=decision/no-event-type-field area=6-log-coverage rec=48,45
name: No event_type field
rule: Log lines carry no event_type field.
why: event already does that job, and a duplicate would amount to a rename.
@row_add n=63 row=decision/additive-field-naming area=6-log-coverage rec=49
name: Additive log field naming
rule: Additive log fields are snake_case with kebab-case values, and carry codes only.
@row_add n=64 row=decision/skipped-symlink-never-logged area=6-log-coverage rec=51
name: Skipped symlink never named in logs
rule: The entry name and link target of a skipped symlink are never logged.
@row_none n=12 rec=53 type=S
@row_none n=13 rec=54 type=S
@row_add n=65 row=contract/log-levels/error area=6-log-coverage rec=55,52
boundary: log levels
item: error
promise: Used for a panic, a process-exit with exit_code 1 or 20, and a channel-response with error_code -32603.
