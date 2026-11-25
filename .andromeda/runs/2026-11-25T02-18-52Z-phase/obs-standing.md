# obs · standing conventions

- S1 · each new integration or E2E test's emitted diagnostics lines are schema-checked, and its test home lives under the one scanned test-home root, kept until the gate steps run · ci-integration · 9-ci-integration
  decision/diag-lines-schema-checked-in-unit-tests · Emitted lines schema-checked in integration and E2E tests — Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
  decision/homes-kept-until-gates-run · Test homes kept until the gates run — Integration-test homes are kept until the gate steps have run.
  decision/test-homes-kept-until-gates · Test homes kept until gate steps run — CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it. · why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
  decision/test-homes-under-e2e-home · Integration-test homes under one root — Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: the one chaos_feed_panic test home
  decision/test-homes-under-scan-root · Test homes live under the scanned root — Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
- S2 · each new log line goes through the obs_event macro with a closed event value and corr where correlated, no raw tracing or print macros, no bare instrument attribute · 11-obs-anti-patterns-never-do-these
  contract/process-log-line/corr · the process log line · corr — Copied unchanged as a JSON number or string, never renamed and never debug- or display-formatted; required only on the events whose rules always define it.
  contract/process-log/corr · Process log line · corr — Copied unchanged as a JSON number or string, never renamed.
  contract/process-log/instance · Process log line · instance — A ViolaName or null.
  contract/process-log/level · Process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/process-log/message · Process log line · message — A required field, as fields.message or the flattened message.
  contract/process-log/process · Process log line · process — The process role, one of run, hook, mcp, cli.
  contract/process-log/target · Process log line · target — A required field.
  contract/process-log/timestamp · Process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  decision/corr-conn-correlation · Correlation through corr and conn — Every cross-surface call carries correlation through the corr field plus the additive conn discriminator.
  decision/cross-process-join-corr-conn · Cross-process joins by corr and conn — Lines across processes are joined by the corr and conn fields, not by W3C trace IDs.
  decision/event-values-closed · event values closed — No event value is added without a Decisions Log entry.
  decision/tracing-json-logging-pipeline · tracing with a JSON subscriber for logging — Logging and spans go through the Rust tracing facade with a tracing-subscriber JSON structured logger.
- S3 · each new event or detail value a diagnostics line can carry is added to the line schema additively, never renaming a field · log-coverage · 6-log-coverage
  decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits.
  decision/log-field-names-frozen · Log field names frozen — Log fields such as corr and event are never renamed without a Decisions Log entry.
  decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but a required one the product still emits is never renamed or removed. · why: The harness greps on them.
