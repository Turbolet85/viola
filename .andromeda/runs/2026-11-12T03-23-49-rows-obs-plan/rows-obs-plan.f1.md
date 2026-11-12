@row_add n=1 row=decision/pty-byte-stream-never-logged area=1-obs-scope-summary rec=1,2,3
name: PTY byte stream never logged
rule: The PTY byte stream is never logged; the viola-pty seam is instrumentable, and the openpty OS API behind it is boundary-only.
why: The PTY byte stream is user content.
@row_add n=2 row=decision/drift-reports-diagnostics-only area=1-obs-scope-summary rec=4,5,6
name: Drift reports go to diagnostics only
rule: Drift reports of viola-agent-claude go to diagnostics/ only and nowhere else.
why: The outputs of viola-agent-claude are the most sensitive in the product.
@row_add n=3 row=decision/claude-internals-not-a-contract area=1-obs-scope-summary rec=7,8,9
name: Wrapped claude internals are not a contract
rule: The wrapped claude CLI is a third-party binary run unmodified and instrumented at its boundary only (spawn and exit, paste writes, the vt100 readiness-gate outcome); its internal state and transcripts are never read as a contract.
@row_add n=4 row=decision/event-payload-not-in-process-logs area=1-obs-scope-summary rec=10,11
name: Event payloads kept out of process logs
rule: The payload content of events.ndjson is never copied into process-log lines.
why: events.ndjson is itself the product audit stream.
@row_add n=5 row=decision/absolute-paths-scrubbed-externally area=1-obs-scope-summary rec=10,11
name: Absolute paths scrubbed from external output
rule: Absolute paths in snapshot.json (endpoint, pinned_bin) and in config values are scrubbed from external output and allowed in diagnostics/.
@row_add n=6 row=contract/log-line/process area=1-obs-scope-summary rec=12,13,14
boundary: every log line
item: process
promise: Every log line carries a process key naming its role: one of run, hook, mcp, cli.
@row_add n=7 row=decision/hook-log-write-failure-swallowed area=1-obs-scope-summary rec=15,16,17
name: Hook log write failure swallowed
rule: A failed write to the hook log is swallowed, so the hook still exits 0 and never writes to stderr.
@row_add n=8 row=decision/mcp-rejections-codes-only area=1-obs-scope-summary rec=18,19,20
name: MCP rejections carry codes only
rule: MCP isError results and rejected tool calls carry codes only (refusal, detail, instance-unreachable, wrapper-fault, invalid-params), with no upstream text, absolute paths or type names.
@row_add n=9 row=decision/one-write-per-log-line area=1-obs-scope-summary rec=21,22,23
name: One write per log line
rule: Each formatted log line is emitted as a single write on an append-mode handle.
why: Concurrent hook processes share one file.
@row_add n=10 row=contract/diagnostics-files/run area=1-obs-scope-summary rec=25,26
boundary: the diagnostics files
item: run
promise: Role run writes <home>/diagnostics/run-<name>.ndjson.
@row_add n=11 row=contract/diagnostics-files/hook area=1-obs-scope-summary rec=25,26
boundary: the diagnostics files
item: hook
promise: Role hook writes <home>/diagnostics/hook-<name>.ndjson, shared by concurrent hook processes; a write failure is swallowed.
@row_add n=12 row=contract/diagnostics-files/mcp area=1-obs-scope-summary rec=25,26
boundary: the diagnostics files
item: mcp
promise: Role mcp writes <home>/diagnostics/mcp.ndjson, which never carries user content, with stderr JSON only when the file cannot be opened.
@row_add n=13 row=contract/diagnostics-files/cli area=1-obs-scope-summary rec=25,26
boundary: the diagnostics files
item: cli
promise: Role cli writes <home>/diagnostics/cli-<name>.ndjson, only when an instance resolves.
@row_add n=14 row=decision/no-log-rotation area=1-obs-scope-summary rec=25,27,28,29
name: No log rotation
rule: There is no rotation of events.ndjson or diagnostics/*.ndjson in v1, retention being the session home's lifetime, and any future diagnostics/ rotation must keep 0600/0700 and never touch events.ndjson offsets.
@row_add n=15 row=absence/otlp-receiver area=1-obs-scope-summary rec=30,31
capability: An OTLP or pulse-class telemetry receiver
@row_add n=16 row=decision/send-text-never-logged area=1-obs-scope-summary rec=32,33,34,35
name: Send text never logged
rule: Sent text never appears on stderr, in --json output, in MCP results or in any process-log field outside instances/<name>/diagnostics/.
@row_add n=17 row=decision/typed-keys-never-logged area=1-obs-scope-summary rec=36,37,38,39
name: Typed keys never logged
rule: Keys a human types never appear in a log line; the wheel product event in events.ndjson is the audit record.
@row_add n=18 row=decision/obs-deps-tokio-free area=1-obs-scope-summary rec=40,41
name: Obs dependencies Tokio-free and C-free
rule: Obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass cargo deny.
guard: cargo deny
@row_add n=19 row=decision/distinct-detail-code-per-exit-cause area=1-obs-scope-summary rec=42,43
name: Distinct detail code per exit cause
rule: Each cause behind exit 21 and exit 1 gets a distinct detail code with no path or pid.
except: A strict-modes check refused before any log opens writes no line; its cause is the fixed stderr pair or the --json document.
@row_add n=20 row=decision/no-external-error-reporter area=2-telemetry-strategy rec=44,45
name: No external error reporter
rule: Errors have no external reporter.
why: Security says no outbound network calls.
@row_add n=21 row=decision/no-event-payloads-in-process-logs area=2-telemetry-strategy rec=44,46
name: No event payloads in process logs
rule: Obs never copies event payloads into process logs.
@row_add n=22 row=decision/no-w3c-traceparent area=2-telemetry-strategy rec=47,48
name: No W3C traceparent in v1
rule: W3C traceparent is not used in v1; correlation goes through corr plus conn.
why: No consumer exists and channel params declare no trace-context field.
@row_add n=23 row=decision/span-naming area=2-telemetry-strategy rec=49,50
name: Span naming as <area>.<operation>
rule: Span names are <area>.<operation> in snake_case, <area> one of run, pty, channel, state, hook, statusline, send, wait, last, answer, pause, release, cli, mcp; cli.<verb> names the dispatch span of a short-lived verb.
