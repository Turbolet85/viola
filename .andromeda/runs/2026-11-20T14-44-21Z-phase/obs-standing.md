# obs · standing conventions

- S1 · every new log line is emitted through the single obs_event macro with a closed event value, never a raw tracing macro or print macro · ci-integration
  contract/detail-line/event · the detail-file line · event — Required on every detail-file line, as on home-level lines.
  contract/detail-line/instance · the detail-file line · instance — Required on every detail-file line, as on home-level lines.
  contract/detail-line/level · the detail-file line · level — Required on every detail-file line, as on home-level lines.
  contract/detail-line/message · the detail-file line · message — Required on every detail-file line, as on home-level lines.
  contract/detail-line/process · the detail-file line · process — Required on every detail-file line, as on home-level lines.
  contract/detail-line/target · the detail-file line · target — Required on every detail-file line, as on home-level lines.
  contract/detail-line/timestamp · the detail-file line · timestamp — Required on every detail-file line, as on home-level lines.
  contract/log-line/message · every log line · message — Flattened, a static literal equal to the event name.
  contract/process-log-line/message · the process log line · message — The message is a static literal equal to the event name and never carries interpolated content.
  decision/lint-probes-failure-fails · Lint-probes failure is a failure — A lint-probes failure (a ban that did not fire, a control that did not pass, or a raw event! outside the core obs module) counts as an SLO invariant failure.
  decision/no-raw-tracing-macros · No raw tracing macros outside the obs module — Raw tracing info, warn, error, debug and trace macros and raw event! are refused outside viola_core::obs. · guard: clippy -D warnings disallowed-macros; raw-event! grep and ban probes in scripts/lint-probes.sh
  decision/obs-event-macro-only · obs_event! is the only event macro — tracing's event and level macros are never called directly outside viola_core::obs; events go through obs_event!. · why: Only obs_event! guarantees event, process and instance. · guard: clippy disallowed-macros; the raw-event! grep in lint-probes
  decision/obs-event-single-macro · One obs_event macro for every line — Every log line is emitted through the one obs_event macro defined in viola-core, which expands to tracing's event at the caller, so viola-core stays I/O-free and tracing-free.
  decision/obs-owns-logger-and-format · Obs owns the logger and format — The logger and the log format are owned by observability.
  decision/raw-tracing-ban · Raw tracing macros banned — The tracing level macros are banned by clippy disallowed-macros and raw event calls by a fail-closed grep outside the obs module, so only obs_event emits lines. · why: clippy reports a disallowed macro expanded inside an exported macro at the caller, so no allow inside obs_event can exempt it · guard: clippy disallowed-macros and scripts/lint-probes.sh
- S2 · every new diagnostic field or event word is additive to the diag-line schema, and every emitted line is schema-checked in the integration and E2E tests · log-coverage
  contract/detail-line/fields · the detail line · fields — Each detail line carries timestamp (millisecond UTC format), level, target, message, event, process and instance, then its content fields, and no corr or conn key.
  contract/diag-line-schema/os · diag-line v1 schema · os — os is closed to linux.
  contract/diag-line-schema/pty-backend · diag-line v1 schema · pty_backend — pty_backend is any string.
  contract/diagnostics-line/format · the diagnostics line · format — Each diagnostics line is one structured JSON object on one line, written whole by one write of the complete line on an append-mode handle.
  contract/diagnostics-line/process · the diagnostics line · process — Every diagnostics line carries the role as field process.
  contract/process-log-line/corr · the process log line · corr — Copied unchanged as a JSON number or string, never renamed and never debug- or display-formatted; required only on the events whose rules always define it.
  contract/process-log-line/instance · the process log line · instance — The instance's name, or absent when none resolved.
  contract/process-log-line/level · the process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/process-log-line/target · the process log line · target — The target is always present.
  contract/process-log-line/timestamp · the process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  contract/process-log/corr · Process log line · corr — Copied unchanged as a JSON number or string, never renamed.
  contract/process-log/level · Process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/process-log/message · Process log line · message — A required field, as fields.message or the flattened message.
  contract/process-log/process · Process log line · process — The process role, one of run, hook, mcp, cli.
  contract/process-log/target · Process log line · target — A required field.
  contract/process-log/timestamp · Process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  contract/process-start/os · the process-start line · os — process-start carries field os, the one word linux.
  contract/process-start/pid · the process-start line · pid — process-start carries the process id as field pid.
  contract/process-start/service-name · the process-start line · service_name — process-start carries the service name as field service_name.
  contract/process-start/version · the process-start line · version — process-start carries the product version as field version.
  contract/run-process-log/common-fields · run process log · common fields — Every run line carries the common fields with process="run" and corr null.
  decision/machine-parseable-signals · Every signal machine-parseable — Every signal produces machine-parseable output: JSON-per-line process logs, the ndjson product event stream, JSON harness documents, hyperfine JSON for perf gates and nextest JUnit XML in CI.
  decision/no-multiline-stack-traces · No multi-line stack traces — Every log line is one physical line parseable by jq; a hook panic is caught and logged as one ERROR line with event panic.
  decision/null-as-key-absence · Null written as key absence — A null corr or instance is written as an absent key, readers treat absence and null as equal, and the schema rejects a literal null. · why: tracing has no null field value
- S3 · every new integration test home lives under the one scanned test-home root and is kept until the gate steps run · ci-integration
  decision/homes-kept-until-gates-run · Test homes kept until the gates run — Integration-test homes are kept until the gate steps have run.
  decision/test-homes-kept-until-gates · Test homes kept until gate steps run — CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it. · why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
  decision/test-homes-under-e2e-home · Integration-test homes under one root — Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: the one chaos_feed_panic test home
  decision/test-homes-under-scan-root · Test homes live under the scanned root — Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
- S4 · every new test waits on a log or event line by condition, never by sleep, and adds no retry rule · slo-invariants-telemetry-budgets
- S5 · every new span uses instrument with skip all and an explicit field allow-list · 11-obs-anti-patterns-never-do-these
  decision/g1-bare-instrument-gate · G1 bare-instrument gate fails closed — Every #[instrument] must have skip_all as its first argument; the G1 gate passes only on rg exit 1 (no match), so a violation, a regex or I/O error or a missing rg fails the CI step. · guard: G1
  decision/instrument-requires-skip-all · Instrument attribute must skip all arguments — Every #[instrument] in the workspace takes skip_all as its first argument; a bare #[instrument] fails the build. · guard: gate G1
  decision/no-err-ret-span-options · err and ret span options banned — The err and ret instrumentation options are banned on spans.
