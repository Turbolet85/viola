@row_add n=1 row=contract/process-log-line/event area=2-telemetry-strategy rec=1,2
boundary: the process log line
item: event
promise: The event field is a closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
@row_add n=2 row=decision/derived-metric-naming area=2-telemetry-strategy rec=3,4
name: Derived metric naming
rule: Derived metrics, computed by jq and never instruments, are named viola.<area>.<measure>, for example viola.hook.duration_ms; no OTel semconv instruments exist.
@row_add n=3 row=decision/channel-response-always-logged area=4-span-trace-coverage rec=5,6
name: channel-response always logged
rule: Every dispatched channel request logs its channel-response line, an early return or an error included.
why: The line is emitted from a Drop guard, so an early return or error still logs it.
@row_add n=4 row=decision/compact-confirm-window area=4-span-trace-coverage rec=5,7,6
name: Compact confirmation window
rule: A /compact send waits 40 s for its compacted-session post-condition, while every other confirmed send waits 10 s.
state: provisional
@row_add n=5 row=decision/dialog-content-never-logged area=4-span-trace-coverage rec=8,9,10
name: Dialog content never logged
rule: Dialog questions, answers, plan text and tool input never appear in a home-level process-log line; dialog events carry codes only, and the text reaches an instance detail file only inside a chain or drift report.
@row_add n=6 row=decision/wheel-event-is-audit-record area=4-span-trace-coverage rec=11,12
name: Wheel product event is the audit record
rule: The wheel product event in events.ndjson is the audit record of a wheel change; the typed keys are never logged and the wheel transition span is only a point in time.
@row_add n=7 row=decision/statusline-stdout-passthrough area=4-span-trace-coverage rec=13,14
name: Statusline stdout passes through unlogged
rule: The user's statusline stdout passes through unchanged and is never logged.
@row_add n=8 row=decision/statusline-deadline area=4-span-trace-coverage rec=13,15,14
name: Statusline shell-out deadline
rule: The statusline shell-out of the user's command is bounded by STATUSLINE_DEADLINE, 5 s.
state: provisional
@row_add n=9 row=decision/env-names-only area=4-span-trace-coverage rec=16,17,18
name: Environment names only, never values
rule: The strip and every log record environment variable names only, never values, and no env map is passed to a span or event.
guard: E2
@row_add n=10 row=decision/ledger-contents-never-logged area=4-span-trace-coverage rec=19,20
name: Capability ledger path and stamps never logged
rule: The capability ledger's path and its stamp contents never appear in a log line.
@row_add n=11 row=absence/in-process-metrics-pipeline area=5-metric-coverage rec=21,22
capability: An in-process metrics pipeline, with no listener and no exporter
@row_add n=12 row=contract/derived-metrics/viola-compat-v-rejected area=5-metric-coverage rec=23,24,25
boundary: derived metrics
item: viola.compat.v_rejected
promise: Counts version rejections alone, reading channel-response with error_code -32602 and detail unsupported-version; other -32602 responses for invalid send params, a bad link or unlink name or pair, or release carrying from carry another detail word.
@row_add n=13 row=contract/log-line/corr area=6-log-coverage rec=26,27,28
boundary: every log line
item: corr
promise: Set per the tests' table and copied unchanged as a JSON number or string; null is represented by the key's absence.
@row_add n=14 row=absence/grpc area=6-log-coverage rec=29,30
capability: gRPC of any kind
@row_add n=15 row=decision/no-sentry-or-otel-bridge area=7-error-capture-reporting rec=31
name: No Sentry or OTel bridge
rule: Errors have no Sentry or OTel bridge; they correlate by corr, conn, process and instance, not by trace_id or span_id.
why: Both were deferred or rejected.
@row_add n=16 row=decision/home-level-lines-path-free area=8-pii-scrubbing-compliance rec=32,33
name: Home-level lines path-free
rule: Home-level log lines are path-free: file fields are basenames, panic_location is workspace-relative and source file recording is off; absolute paths appear only in detail files.
why: The OS username inside absolute paths is the only PII in scope.
@row_add n=17 row=absence/otel-span-processing area=8-pii-scrubbing-compliance rec=34
capability: An OTel SpanProcessor
@row_add n=18 row=decision/secret-scan-before-upload area=8-pii-scrubbing-compliance rec=37,38
name: Secret scan before artifact upload
rule: CI runs the secret scan for the Cookie header, CLAUDE* values, the content canary and 0600 over every diagnostics/ file before any artifact upload.
guard: secret-scan test
@row_add n=19 row=decision/g1-bare-instrument-gate area=9-ci-integration rec=39,40
name: G1 bare-instrument gate fails closed
rule: Every #[instrument] must have skip_all as its first argument; the G1 gate passes only on rg exit 1 (no match), so a violation, a regex or I/O error or a missing rg fails, and a G1 failure is an SLO invariant failure.
guard: G1
@row_add n=20 row=decision/uploads-gated-on-secret-scan area=9-ci-integration rec=41,42
name: CI uploads gated on the secret scan
rule: The diag, harness and junit CI artifact uploads run only when the secret scan, which always runs, succeeded.
guard: secret-scan
