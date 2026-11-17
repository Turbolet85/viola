# obs · standing conventions

- S1 · Integration and E2E test homes live under one root inside the secret-scan root and are kept until the panic, schema and scan gates run. · 9-ci-integration · ci-integration
- S2 · Every line a test or rig process emits is schema-checked in integration and E2E tests against the diagnostics line schema. · ci-integration
  contract/diag-line-schema/os · diag-line v1 schema · os — os is closed to linux.
  contract/diag-line/instance · diagnostic log line · instance — The instance's `ViolaName`; the key is absent when the process has no instance.
  contract/diag-line/process · diagnostic log line · process — The role of the writing process, one of run, hook, mcp, cli; required on every line and matching `sender` / `writer`.
  contract/diag-line/service-name · diagnostic log line · service.name — Hardcoded to "viola".
  contract/diag-line/service-version · diagnostic log line · service.version — The crate version, fixed at compile time.
  contract/log-line/instance · every log line · instance — The ViolaName; the key is absent when there is none.
  contract/log-line/level · every log line · level — One of DEBUG, INFO, WARN, ERROR; TRACE is never emitted.
  contract/log-line/message · every log line · message — Flattened, a static literal equal to the event name.
  contract/log-line/process · every log line · process — Every log line carries a process key naming its role: one of run, hook, mcp, cli.
  contract/log-line/target · every log line · target — The Rust module path, for example viola_channel::server.
  contract/process-log/level · Process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/process-log/message · Process log line · message — A required field, as fields.message or the flattened message.
  contract/process-log/target · Process log line · target — A required field.
  contract/run-log/cli-verified · run log line · cli_verified — A required field on run's log lines.
  contract/run-log/exit-code · run log line · exit_code — A required field on run's log lines.
  decision/diag-lines-schema-checked-in-unit-tests · Emitted lines schema-checked in integration and E2E tests — Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
  decision/no-event-type-field · No event_type field — Log lines carry no event_type field. · why: event already does that job, and a duplicate would amount to a rename.
  decision/single-line-log-records · Single-line log records — Every log record is one JSON line parseable by jq; never a multi-line stack trace.
- S3 · No sleep-based log waits; a test waits on a condition with a bound. · 10-slo-invariants-telemetry-budgets
- S4 · An artifact a test or CI job uploads comes only from synthetic inputs and only after a passing secret scan; local run outputs are never uploaded. · 8-pii-scrubbing-compliance · pii-scrubbing-compliance
  decision/failed-scan-uploads-hit-report · Failed scan uploads the hit report — On a failed secret scan the hit report is uploaded instead of the diagnostics.
  decision/fuzz-artifacts-synthetic-seeds-only · Fuzz artifacts uploaded only with synthetic seeds — A non-synthetic fuzz corpus seed first drops the fuzz artifacts upload.
  decision/scan-failure-uploads-hit-report-only · Only the hit report uploaded on scan failure — When the secret scan fails, only its hit report is uploaded, as the secret-scan-<os> artifact.
