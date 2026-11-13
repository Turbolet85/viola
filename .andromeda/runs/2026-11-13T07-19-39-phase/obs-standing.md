# obs · standing conventions

- S1 · each new or moved integration test file takes its home from the one shared test-home root and keeps that home until the panic, schema and secret-scan gates have run · ci-integration · 9-ci-integration
- S2 · each new test that drives a viola process schema-checks the diagnostic lines it emits against the line schema · ci-integration · 9-ci-integration
  contract/diag-line-schema/pty-backend · diag-line v1 schema · pty_backend — pty_backend is any string.
  contract/diag-line/instance · diagnostic log line · instance — The instance's `ViolaName`; the key is absent when the process has no instance.
  contract/diag-line/process · diagnostic log line · process — The role of the writing process, one of run, hook, mcp, cli; required on every line and matching `sender` / `writer`.
  contract/diag-line/service-name · diagnostic log line · service.name — Hardcoded to "viola".
  contract/diag-line/service-version · diagnostic log line · service.version — The crate version, fixed at compile time.
  contract/log-line/corr · every log line · corr — Set per the tests' table and copied unchanged as a JSON number or string; null is represented by the key's absence.
  contract/log-line/event · every log line · event — A value of the closed kebab-case event enum.
  contract/log-line/instance · every log line · instance — The ViolaName; the key is absent when there is none.
  contract/log-line/level · every log line · level — One of DEBUG, INFO, WARN, ERROR; TRACE is never emitted.
  contract/log-line/message · every log line · message — Flattened, a static literal equal to the event name.
  contract/log-line/process · every log line · process — Every log line carries a process key naming its role: one of run, hook, mcp, cli.
  contract/log-line/target · every log line · target — The Rust module path, for example viola_channel::server.
  contract/log-line/timestamp · every log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  contract/process-log-line/duration-ms · the process log line · duration_ms — Durations are duration_ms as an integer.
  contract/process-log/corr · Process log line · corr — Copied unchanged as a JSON number or string, never renamed.
  contract/process-log/event · Process log line · event — A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
  contract/process-log/instance · Process log line · instance — A ViolaName or null.
  contract/process-log/level · Process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/process-log/message · Process log line · message — A required field, as fields.message or the flattened message.
  contract/process-log/process · Process log line · process — The process role, one of run, hook, mcp, cli.
  contract/process-log/target · Process log line · target — A required field.
  contract/process-log/timestamp · Process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  contract/process-start-event/service-name · the process-start event · service_name — Emitted once per process as the fixed value "viola", the binary name.
  contract/process-start-event/version · the process-start event · version — Emitted once per process as the crate package version.
  decision/additive-field-naming · Additive log field naming — Additive log fields are snake_case with kebab-case values, and carry codes only.
  decision/detail-schema-self-contained · Separate self-contained detail-line schema — Detail-file lines are validated by their own schema with a top-level unevaluatedProperties false, which inlines the event enum and uses no cross-file reference.
  decision/diag-lines-schema-checked-in-unit-tests · Emitted lines schema-checked in integration and E2E tests — Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
  decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove any field.
  decision/no-event-type-field · No event_type field — Log lines carry no event_type field. · why: event already does that job, and a duplicate would amount to a rename.
  decision/schema-top-level-unevaluated-properties · Default-deny schema via top-level unevaluatedProperties — The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists. · why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then · guard: contract_diag_schema
- S3 · each new test, fixture or harness output that may be uploaded uses synthetic inputs only and passes the secret scan before upload · pii-scrubbing-compliance · 8-pii-scrubbing-compliance
  decision/claude-fixtures-scrub-required · Claude fixtures are scrub-required — Build and CI infrastructure is not instrumented, and the claude fixtures are scrub-required data, never a telemetry source. · why: Not product runtime.
  decision/detail-upload-needs-synthetic-input · Detail upload needs synthetic input — Detail files and harness captures are uploaded only while every input is synthetic; a non-synthetic input source must first drop detail files and the harness capture from the upload paths.
  decision/detail-upload-synthetic-inputs-only · Detail-file upload only with synthetic inputs — Detail files and the harness capture are uploaded only while every input is synthetic; a non-synthetic source first drops them from the upload paths. · why: every input is synthetic, so detail content carries no real data
  decision/fixtures-not-telemetry · Recorded fixtures are no telemetry source — Recorded fixtures are scrub-required data, not a telemetry source, and are excluded from the observability plan. · why: Not product runtime.
  decision/fuzz-artifacts-synthetic-seeds-only · Fuzz artifacts uploaded only with synthetic seeds — A non-synthetic fuzz corpus seed first drops the fuzz artifacts upload.
  decision/stripped-env-never-logged · Stripped CLAUDE env values never logged — No R8-stripped `CLAUDE*` value appears in any sink or fixture; the strip is logged as a count or variable names only.
- S4 · each new test waits on a log line by reading for it, never by sleeping, and asserts budgets by exit code · slo-invariants-telemetry-budgets · 10-slo-invariants-telemetry-budgets
  decision/budgets-asserted-by-exit-code · Budgets asserted by exit code — Every budget has an exit-code-setting assertion (hyperfine max via jq).
  decision/no-sleep-based-log-waits · No sleep-based log waits — No log-based wait relies on sleeps.
