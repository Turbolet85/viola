# obs · standing conventions

- S1 · every new test's emitted diagnostics lines are schema-checked, and the schema stays additive only · ci-integration · 9-ci-integration
  contract/diag-line-schema/os · diag-line v1 schema · os — os is closed to linux.
  contract/diag-line-schema/pty-backend · diag-line v1 schema · pty_backend — pty_backend is any string.
  contract/logs-wrapper/record-envelope · logs wrapper · record envelope — Each process-log line is wrapped as {"src":"diag","file":<basename>,"record":<line>}.
  contract/process-log/corr · Process log line · corr — Copied unchanged as a JSON number or string, never renamed.
  contract/process-log/instance · Process log line · instance — A ViolaName or null.
  contract/process-log/level · Process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/process-log/message · Process log line · message — A required field, as fields.message or the flattened message.
  contract/process-log/process · Process log line · process — The process role, one of run, hook, mcp, cli.
  contract/process-log/target · Process log line · target — A required field.
  contract/process-log/timestamp · Process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  decision/diag-lines-schema-checked-in-unit-tests · Emitted lines schema-checked in integration and E2E tests — Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
  decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits.
  decision/diag-schema-matches-event-enum · Diagnostic line schema matches the event enum — A hand-maintained JSON Schema of the process-log line is committed, and its event enum must equal the ObsEvent variants, asserted by a tests-owned check.
  decision/g4-schema-conformance-gate · Schema conformance gate on every line — Every home-level line is validated against the diagnostics line schema and every detail line against the detail schema, in a step that always runs. · guard: G4
  decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but a required one the product still emits is never renamed or removed. · why: The harness greps on them.
  decision/log-schema-additive-only · Log schema additive to the tests' format — The observability log schema only adds fields and enum values to the tests' log format; it renames nothing and removes only a field no build emits. · why: The tests' log format is the binding text.
  decision/obs-aligns-to-tests-log-format · Observability aligns to the tests log format — The log format is bound by the test plan's harness contract; observability may add fields but must not rename them or remove one the product still emits. · why: the harness greps on them
  decision/schema-conformance-gate · Schema conformance gate on real process output — Every role-file line of test homes must validate against schemas/diag-line.v1.json and every detail line against schemas/diag-detail.v1.json; a failing line fails the build, a torn non-JSON line is skipped and counted. · why: schema failures count as a build failure and unit tests check only in-memory lines · guard: gate G4 (schema-conformance step)
  decision/schema-top-level-unevaluated-properties · Default-deny schema via top-level unevaluatedProperties — The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists. · why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then · guard: contract_diag_schema
- S2 · new integration-test homes live under the one scanned root and are kept until the gates run · 9-ci-integration · ci-integration
  decision/homes-kept-until-gates-run · Test homes kept until the gates run — Integration-test homes are kept until the gate steps have run.
  decision/test-homes-kept-until-gates · Test homes kept until gate steps run — CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it. · why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
  decision/test-homes-under-e2e-home · Integration-test homes under one root — Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: the one chaos_feed_panic test home
  decision/test-homes-under-scan-root · Test homes live under the scanned root — Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
- S3 · new fixtures carry synthetic input only and pass the secret scan before any upload · 8-pii-scrubbing-compliance · pii-scrubbing-compliance
  decision/secret-scan-hit-fails · Secret-scan hit is a failure — A secret-scan hit counts as an SLO invariant failure.
  decision/upload-members-scanned-or-dropped · Upload members scanned or dropped — An uploaded member that could carry a host path or log content is scanned before upload or dropped; unscanned uploads are admissible by content only.
  decision/upload-only-after-secret-scan · Diagnostics uploaded only after the secret scan — diagnostics artifacts are never uploaded before the secret-scan step passes; uploads run if: always() / failure() combined with the secret-scan step's success, never bare.
- S4 · every new log line goes through the one obs_event macro with a closed event value and default-deny fields · 11-obs-anti-patterns-never-do-these
  contract/log-line/corr · every log line · corr — Set per the tests' table and copied unchanged as a JSON number or string; null is represented by the key's absence.
  contract/log-line/event · every log line · event — A value of the closed kebab-case event enum.
  contract/log-line/level · every log line · level — One of DEBUG, INFO, WARN, ERROR; TRACE is never emitted.
  contract/log-line/message · every log line · message — Flattened, a static literal equal to the event name.
  contract/log-line/target · every log line · target — The Rust module path, for example viola_channel::server.
  contract/log-line/timestamp · every log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  contract/obs-event-macro/attached-fields · the obs_event macro · attached fields — Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.
  contract/process-log-line/event · the process log line · event — The event field is a closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
  contract/process-log/event · Process log line · event — A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
  decision/closed-event-and-corr-on-every-line · Closed event enum on every line, corr where correlated — Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
  decision/default-deny-log-fields · Default-deny for logged fields — A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure. · guard: gate G4 against schemas/diag-line.v1.json
  decision/event-values-closed · event values closed — No event value is added without a Decisions Log entry.
  decision/field-allow-list-default-deny · Field allow-list is default-deny — No field is logged unless named in the field catalog.
  decision/level-changes-volume-only · Level changes volume only — Every diagnostics level uses the same field allow-list, so the level changes only how many lines are written. · why: No setting can disable redaction.
  decision/lint-probes-failure-fails · Lint-probes failure is a failure — A lint-probes failure (a ban that did not fire, a control that did not pass, or a raw event! outside the core obs module) counts as an SLO invariant failure.
  decision/low-class-metadata-as-typed-fields · Low-class metadata as typed fields — Low-class metadata is logged as typed fields, never inside detail codes.
  decision/no-event-type-field · No event_type field — Log lines carry no event_type field. · why: event already does that job, and a duplicate would amount to a rename.
  decision/no-raw-tracing-macros · No raw tracing macros outside the obs module — Raw tracing info, warn, error, debug and trace macros and raw event! are refused outside viola_core::obs. · guard: clippy -D warnings disallowed-macros; raw-event! grep and ban probes in scripts/lint-probes.sh
  decision/no-trace-and-span-ids · No trace_id or span_id — Log lines never emit trace_id or span_id; correlation is by corr plus conn. · why: No W3C trace context exists, and spans are not attached to lines.
  decision/obs-event-macro-only · obs_event! is the only event macro — tracing's event and level macros are never called directly outside viola_core::obs; events go through obs_event!. · why: Only obs_event! guarantees event, process and instance. · guard: clippy disallowed-macros; the raw-event! grep in lint-probes
  decision/obs-event-single-macro · One obs_event macro for every line — Every log line is emitted through the one obs_event macro defined in viola-core, which expands to tracing's event at the caller, so viola-core stays I/O-free and tracing-free.
  decision/obs-owns-logger-and-format · Obs owns the logger and format — The logger and the log format are owned by observability.
  decision/raw-tracing-ban · Raw tracing macros banned — The tracing level macros are banned by clippy disallowed-macros and raw event calls by a fail-closed grep outside the obs module, so only obs_event emits lines. · why: clippy reports a disallowed macro expanded inside an exported macro at the caller, so no allow inside obs_event can exempt it · guard: clippy disallowed-macros and scripts/lint-probes.sh
  decision/source-allow-list-logging · Source allow-list for logged fields — Spans skip all arguments and declare fields explicitly, and events go through obs_event! with explicit typed fields only; this source allow-list is the primary PII control. · guard: disallowed-macros; raw_event_hits; G1 bare instrument
- S5 · new obs-touching code is mutation-tested like product code and fails closed on a surviving mutant · ci-integration · 10-slo-invariants-telemetry-budgets
  decision/obs-code-mutation-tested · Obs code mutation-tested like product code — Obs code is mutation-tested like product code at the epoch boundary and on demand, not on push or pull request; a timed-out or missed obs-code mutant, or unviable outnumbering caught, is red. · guard: test-plan Mutation gate
  decision/surviving-obs-mutant-fails · Surviving obs mutant is a failure — A surviving cargo-mutants mutant in obs code at a run --mutants (the epoch-boundary audit or on demand) counts as an SLO invariant failure. · except: A missed mutant the host never compiles is not counted as surviving.
