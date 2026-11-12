@row_none n=1 rec=1 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=2 rec=2 type=0
what: a source citation for a trigger
@row_none n=3 rec=3 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=4 rec=4 type=0
what: a source citation for a trigger
@row_add n=1 row=decision/single-file-sink-no-os-exporter area=1-obs-scope-summary rec=5
name: One file sink and one line schema
rule: Telemetry on Linux goes to one file sink with one line schema, with no OS-specific exporter.
@row_none n=5 rec=6 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=6 rec=7 type=0
what: a source citation for a trigger
@row_none n=7 rec=8 type=0
what: a trigger-type label naming why the obs scope applies
@row_add n=2 row=decision/event-log-never-rotated area=1-obs-scope-summary rec=9
name: Event log never rotated
rule: The events.ndjson product event log is never rotated; its retention is the whole lifetime of the session home.
why: byte offsets into the log are cursors
@row_none n=8 rec=10 type=0
what: a trigger-type label naming why the obs scope applies
@row_add n=3 row=decision/ndjson-logs-on-disk area=1-obs-scope-summary rec=11
name: Bridge logs as ndjson on disk
rule: The bridge's logs are ndjson on disk, and the event log serves as the audit trail.
why: logs on disk survive restarts on either side
@row_none n=9 rec=12 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=10 rec=13 type=0
what: a source citation for a trigger
@row_add n=4 row=contract/process-log-event/release-from-driver area=1-obs-scope-summary rec=14
boundary: the process log event field
item: release-from-driver
promise: Release from the driver is logged as its own distinct event.
@row_add n=5 row=decision/distinct-detail-code-per-exit-cause area=1-obs-scope-summary rec=14
name: Distinct detail code per exit cause
rule: Each cause behind exit 21 and exit 1 is logged with its own distinct detail code that carries no path or pid.
except: A strict-modes check refused before any log opens writes no line; its cause is the fixed stderr pair or the --json document.
@row_none n=11 rec=15 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=12 rec=16 type=0
what: a source citation for a trigger
@row_none n=13 rec=17 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=14 rec=18 type=0
what: a source citation for a trigger
@row_add n=6 row=decision/no-compliance-audit-trail area=1-obs-scope-summary rec=19,26
name: No compliance audit trail
rule: Observability keeps no compliance audit trail; events.ndjson is the product audit stream, not a compliance trail.
why: the security tier is Minimal and no compliance triggers apply
@row_add n=7 row=decision/normal-mode-not-inverted area=1-obs-scope-summary rec=20
name: Normal observability mode, not inverted
rule: Viola is not a telemetry product: observability runs in normal mode, and events.ndjson is a product audit log, not a telemetry-collector function.
@row_add n=8 row=absence/otlp-receiver area=1-obs-scope-summary rec=20
capability: An OTLP receiver or OTLP proto handling
@row_none n=15 rec=21 type=9
@row_none n=16 rec=22 type=S
@row_none n=17 rec=23 type=S
@row_none n=18 rec=24 type=S
@row_add n=9 row=decision/standard-obs-tier area=2-telemetry-strategy rec=25
name: Standard telemetry tier with Minimal exporter carve-outs
rule: Telemetry follows the Standard signal pyramid, with Minimal-tier carve-outs for exporters.
@row_add n=10 row=overview/2-telemetry-strategy area=2-telemetry-strategy rec=26
text: Structured logs are the primary signal, one JSON-per-line file per process role; spans are declared in code but not serialized in v1, and cross-process joins use corr and conn; errors have no external reporter; metrics have no in-process instruments and no exporter and are derived by jq over the ndjson; events.ndjson is the product audit stream, not a compliance trail, and obs never copies event payloads into process logs.
@row_add n=11 row=decision/spans-not-serialized-v1 area=2-telemetry-strategy rec=26
name: Spans declared but not serialized
rule: Spans are declared in code but not serialized in v1.
@row_add n=12 row=decision/no-external-error-reporter area=2-telemetry-strategy rec=26
name: No external error reporter
rule: Errors are not sent to any external error reporter.
@row_add n=13 row=decision/metrics-derived-by-jq area=2-telemetry-strategy rec=26
name: Metrics derived by jq, no instruments
rule: Metrics have no in-process instruments and no exporter; they are derived by jq over the ndjson logs.
@row_add n=14 row=decision/no-event-payloads-in-process-logs area=2-telemetry-strategy rec=26
name: No event payloads in process logs
rule: Observability never copies product event payloads into process logs.
@row_none n=19 rec=27 type=S
@row_add n=15 row=decision/machine-parseable-signals area=2-telemetry-strategy rec=28
name: Every signal machine-parseable
rule: Every signal produces machine-parseable output: JSON-per-line process logs, the ndjson product event stream, JSON harness documents, hyperfine JSON for perf gates and nextest JUnit XML in CI.
@row_add n=16 row=decision/no-human-only-dashboards area=2-telemetry-strategy rec=29
name: No human-only dashboards
rule: No Grafana, Kibana, Datadog or other human-only dashboard UI exists in the stack, and none may be added as a consumption surface.
@row_add n=17 row=decision/no-proprietary-apm-sdk area=2-telemetry-strategy rec=30
name: No proprietary APM SDK
rule: No proprietary APM SDK is used, as primary or at all; the v1 exporter role is filled by the local file sink.
@row_add n=18 row=decision/no-human-gated-log-analysis area=2-telemetry-strategy rec=31
name: No human-review-gated log analysis
rule: Every log verdict is a jq or harness assertion that sets an exit code, never a human review.
@row_add n=19 row=decision/one-line-panic-record area=2-telemetry-strategy rec=32
name: One structured line per panic
rule: Every panic after init step 4 produces exactly one single-line structured record in its home-level role file with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file.
except: the bounded panic exemptions listed in the plan's Section 10
@row_add n=20 row=decision/corr-conn-correlation area=2-telemetry-strategy rec=33
name: Correlation through corr and conn
rule: Every cross-surface call carries correlation through the corr field plus the additive conn discriminator.
@row_add n=21 row=decision/no-w3c-traceparent area=2-telemetry-strategy rec=33
name: No W3C traceparent in v1
rule: W3C traceparent is not used in v1.
why: no consumer exists and channel params declare no trace-context field
@row_none n=20 rec=34 type=S
@row_add n=22 row=decision/span-naming area=2-telemetry-strategy rec=35,53
name: Span naming as area.operation
rule: Span names are <area>.<operation> in snake_case, area one of run, pty, channel, state, hook, statusline, send, wait, last, answer, pause, release, cli, mcp; cli.<verb> names a short-lived verb's dispatch span.
@row_add n=23 row=decision/span-names-not-wire-contract area=2-telemetry-strategy rec=35
name: Span names are code-internal
rule: Span names are code-internal and not part of any wire contract.
@row_add n=24 row=contract/process-log-line/event area=2-telemetry-strategy rec=36
boundary: the process log line
item: event
promise: The event field is a closed kebab-case enum.
@row_add n=25 row=decision/derived-metric-naming area=2-telemetry-strategy rec=37
name: Derived metric naming
rule: Derived metrics, computed by jq and never instruments, are named viola.<area>.<measure>, for example viola.hook.duration_ms.
@row_add n=26 row=contract/process-log-line/field-names area=2-telemetry-strategy rec=38
boundary: the process log line
item: field names
promise: Field names are snake_case.
@row_add n=27 row=contract/process-log-line/enum-values area=2-telemetry-strategy rec=38
boundary: the process log line
item: enum values
promise: Enum values are kebab-case.
@row_add n=28 row=contract/process-log-line/at-instants area=2-telemetry-strategy rec=38
boundary: the process log line
item: *_at
promise: Instants are named *_at and written in RFC 3339 UTC with milliseconds and a Z suffix.
@row_add n=29 row=contract/process-log-line/duration-ms area=2-telemetry-strategy rec=38
boundary: the process log line
item: duration_ms
promise: Durations are duration_ms as an integer.
@row_add n=30 row=contract/process-start-event/service-name area=2-telemetry-strategy rec=39
boundary: the process-start event
item: service_name
promise: Emitted once per process as the fixed value "viola", the binary name.
@row_add n=31 row=contract/process-start-event/version area=2-telemetry-strategy rec=39
boundary: the process-start event
item: version
promise: Emitted once per process as the crate package version.
@row_none n=21 rec=40 type=S
@row_none n=22 rec=41 type=S
@row_none n=23 rec=42 type=S
@row_none n=24 rec=43 type=S
@row_add n=32 row=contract/hook-output/stdout area=3-observability-harness-contract rec=44
boundary: hook output streams
item: stdout
promise: A hook writes only its decision body to stdout, and no telemetry reaches stdout.
@row_add n=33 row=contract/hook-output/stderr area=3-observability-harness-contract rec=44
boundary: hook output streams
item: stderr
promise: A hook writes nothing to stderr.
@row_add n=34 row=contract/hook-output/statusline-stdout area=3-observability-harness-contract rec=44
boundary: hook output streams
item: viola hook statusline stdout
promise: Carries the user's status-line command output byte for byte; nothing when no command is recorded, on a refused instance, on oversize or malformed stdin, or under the hidden --capture arm.
@row_add n=35 row=contract/run-terminal-output/child-output area=3-observability-harness-contract rec=45
boundary: run terminal output
item: child output
promise: run writes nothing to the terminal except the child's own output; while the child runs, neither run nor a revived start writes anything of its own.
@row_add n=36 row=contract/run-terminal-output/start-refusal area=3-observability-harness-contract rec=45
boundary: run terminal output
item: start refusal
promise: A start refusal before any child is spawned writes exactly two fixed stderr lines with no path or pid: the home pair for a home-level refusal, the instance pair for an instance-level one.
@row_add n=37 row=contract/revive-list-stderr/refusal-pair area=3-observability-harness-contract rec=45
boundary: viola revive --list stderr
item: refusal pair
promise: viola revive --list writes the refusal pair for its two refusals and nothing else on stderr.
@row_add n=38 row=contract/list-stderr/unable-hint-pair area=3-observability-harness-contract rec=45
boundary: viola list stderr
item: unable / hint pair
promise: The unable / hint: pair carries no SGR colour codes.
@row_add n=39 row=decision/codes-only-process-logs area=3-observability-harness-contract rec=46
name: Codes-only process logs, content in detail files
rule: Every role's codes-only process log goes to its home-level role file under diagnostics, and content-bearing detail goes only to the instance's per-role detail file.
@row_add n=40 row=contract/mcp-output/stdout area=3-observability-harness-contract rec=47
boundary: mcp output streams
item: stdout
promise: mcp writes only MCP frames to stdout.
@row_add n=41 row=decision/short-lived-verbs-stderr area=3-observability-harness-contract rec=48
name: Short-lived verbs use stderr for humans
rule: Short-lived CLI verbs may use stderr for their human output.
@row_add n=42 row=decision/obs-owns-logger-and-format area=3-observability-harness-contract rec=49
name: Obs owns the logger and format
rule: The logger and the log format are owned by observability.
@row_none n=25 rec=50 type=S
@row_none n=26 rec=51 type=S
@row_none n=27 rec=52 type=S
@row_add n=43 row=decision/static-span-names area=4-span-trace-coverage rec=53
name: Static span names
rule: Span names are static strings; instance names, dialog ids and paths never appear in a span name.
@row_add n=44 row=decision/span-field-allow-list area=4-span-trace-coverage rec=53
name: Spans declared with explicit field allow-list
rule: Every span is declared skipping all arguments with an explicit field allow-list.
@row_add n=45 row=decision/no-err-ret-span-options area=4-span-trace-coverage rec=53
name: err and ret span options banned
rule: The err and ret instrumentation options are banned on spans.
@row_add n=46 row=decision/span-kinds-not-emitted-v1 area=4-span-trace-coverage rec=54
name: Span kinds recorded but not emitted
rule: Span kinds are recorded in the plan per span but not emitted in v1; if the deferred tracing-opentelemetry bridge is adopted, they map to the otel.kind span field.
@row_none n=28 rec=55 type=S
