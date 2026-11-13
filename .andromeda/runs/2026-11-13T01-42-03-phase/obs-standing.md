# obs · standing conventions

- S1 · every new integration or E2E test checks the lines it makes against the diagnostic line schema and keeps its test home under the scanned integration-test root until the gates have run · ci-integration · 9-ci-integration
  contract/diag-line-schema/os · diag-line v1 schema · os — os is closed to linux.
  contract/diagnostics-line/format · the diagnostics line · format — Each diagnostics line is one structured JSON object on one line, written whole by one write of the complete line on an append-mode handle.
  contract/diagnostics-line/instance · the diagnostics line · instance — Every diagnostics line of a process with an instance carries it as field instance; a process without one, such as an mcp server with no instance name, writes no instance key.
  contract/diagnostics-line/process · the diagnostics line · process — Every diagnostics line carries the role as field process.
  contract/obs-event-macro/attached-fields · the obs_event macro · attached fields — Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.
  contract/process-log-line/at-instants · the process log line · *_at — Instants are named *_at and written in RFC 3339 UTC with milliseconds and a Z suffix.
  contract/process-log-line/corr · the process log line · corr — Copied unchanged as a JSON number or string, never renamed and never debug- or display-formatted; required only on the events whose rules always define it.
  contract/process-log-line/duration-ms · the process log line · duration_ms — Durations are duration_ms as an integer.
  contract/process-log-line/enum-values · the process log line · enum values — Enum values are kebab-case.
  contract/process-log-line/event · the process log line · event — The event field is a closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
  contract/process-log-line/field-names · the process log line · field names — Field names are snake_case.
  contract/process-log-line/instance · the process log line · instance — The instance's name, or absent when none resolved.
  contract/process-log-line/level · the process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/process-log-line/message · the process log line · message — The message is a static literal equal to the event name and never carries interpolated content.
  contract/process-log-line/process · the process log line · process — One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing cli-<name>.ndjson.
  contract/process-log-line/target · the process log line · target — The target is always present.
  contract/process-log-line/timestamp · the process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  contract/process-start-event/service-name · the process-start event · service_name — Emitted once per process as the fixed value "viola", the binary name.
  contract/process-start-event/version · the process-start event · version — Emitted once per process as the crate package version.
  decision/diag-lines-schema-checked-in-unit-tests · Emitted lines schema-checked in integration and E2E tests — Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
  decision/homes-kept-until-gates-run · Test homes kept until the gates run — Integration-test homes are kept until the gate steps have run.
  decision/level-changes-volume-only · Level changes volume only — Every diagnostics level uses the same field allow-list, so the level changes only how many lines are written. · why: No setting can disable redaction.
  decision/millisecond-utc-timestamps · Millisecond UTC timestamps — Every diagnostics timestamp is RFC 3339 in UTC with millisecond precision and a Z suffix. · why: The default timer writes microseconds, and the chrono RFC 3339 timer writes +00:00 with automatic precision.
  decision/no-ansi-in-logs · No ANSI codes in diagnostics lines — Diagnostics lines carry no ANSI colour codes.
  decision/no-source-paths-in-logs · No source paths in diagnostics lines — Diagnostics lines carry no source file names or line numbers.
  decision/no-span-list-in-lines · No span list in diagnostics lines — Diagnostics lines never carry the list of entered spans. · why: Closes the spans leak path.
  decision/no-trace-and-span-ids · No trace_id or span_id — Log lines never emit trace_id or span_id; correlation is by corr plus conn. · why: No W3C trace context exists, and spans are not attached to lines.
  decision/null-as-key-absence · Null written as key absence — A null corr or instance is written as an absent key, readers treat absence and null as equal, and the schema rejects a literal null. · why: tracing has no null field value
  decision/obs-aligns-to-tests-log-format · Observability aligns to the tests log format — The log format is bound by the test plan's harness contract; observability may add fields but must not rename or remove its fields. · why: the harness greps on them
  decision/schema-conformance-gate · Schema conformance gate on real process output — Every role-file line of test homes must validate against schemas/diag-line.v1.json and every detail line against schemas/diag-detail.v1.json; a failing line fails the build, a torn non-JSON line is skipped and counted. · why: schema failures count as a build failure and unit tests check only in-memory lines · guard: gate G4 (schema-conformance step)
  decision/test-homes-kept-until-gates · Test homes kept until gate steps run — CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it. · why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
  decision/test-homes-under-e2e-home · Integration-test homes under one root — Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: the one chaos_feed_panic test home
  decision/test-homes-under-scan-root · Test homes live under the scanned root — Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
  decision/tests-event-list-adopted · Tests event list adopted — The tests' field and event list is adopted verbatim, with send-issued and send-refused among the logged events.
- S2 · every new recorded claude fixture or fake-agent screen is scrubbed before commit and is never a telemetry source · 1-obs-scope-summary · obs-scope-summary
- S3 · every new test waits on a condition, never on a sleep, and adds no retry; the CI gates still fail closed on its output · slo-invariants-telemetry-budgets · 10-slo-invariants-telemetry-budgets
  decision/ci-gates-fail-closed · CI gates fail closed — A gate whose tool is missing, errors or meets an empty scope fails the step rather than passing. · why: an erroring search with a must-be-empty check would pass on every commit · guard: G1, G3, scripts/g2-zero-panics.sh --probe
  decision/no-retry-once · No retry-once policies — No retry-once policy is added for panics or flaky telemetry assertions.
