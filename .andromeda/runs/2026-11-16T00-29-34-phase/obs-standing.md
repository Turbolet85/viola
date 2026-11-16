# obs · standing conventions

- S1 · each new integration test's home lives under the one scanned test-home root and is kept until the gate steps run · ci-integration · 9-ci-integration
  decision/homes-kept-until-gates-run · Test homes kept until the gates run — Integration-test homes are kept until the gate steps have run.
  decision/secret-scan-before-upload · Secret scan before artifact upload — CI runs the secret scan over every diagnostics/ file before any artifact upload, its classes being CLAUDE* values, the content canary, mode 0600 and an unreadable file; a Cookie header line is no class and no hit. · guard: secret-scan test
  decision/test-homes-kept-until-gates · Test homes kept until gate steps run — CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it. · why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
  decision/test-homes-under-e2e-home · Integration-test homes under one root — Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: the one chaos_feed_panic test home
  decision/test-homes-under-scan-root · Test homes live under the scanned root — Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
- S2 · lines emitted by a new integration test's processes are schema-checked against the diagnostics line schema · ci-integration · 9-ci-integration
  contract/diag-line-schema/os · diag-line v1 schema · os — os is closed to linux.
  contract/diag-line-schema/pty-backend · diag-line v1 schema · pty_backend — pty_backend is any string.
  contract/diag-line/instance · diagnostic log line · instance — The instance's `ViolaName`; the key is absent when the process has no instance.
  contract/diag-line/process · diagnostic log line · process — The role of the writing process, one of run, hook, mcp, cli; required on every line and matching `sender` / `writer`.
  contract/diag-line/service-name · diagnostic log line · service.name — Hardcoded to "viola".
  contract/diag-line/service-version · diagnostic log line · service.version — The crate version, fixed at compile time.
  contract/log-line/process · every log line · process — Every log line carries a process key naming its role: one of run, hook, mcp, cli.
  contract/logs-wrapper/record-envelope · logs wrapper · record envelope — Each process-log line is wrapped as {"src":"diag","file":<basename>,"record":<line>}.
  contract/process-log-corr/dialog-answered · Process log corr by event · dialog-answered — corr is the dialog_id.
  contract/process-log-corr/dialog-raised · Process log corr by event · dialog-raised — corr is the dialog_id.
  contract/process-log-corr/hook-decision · Process log corr by event · hook-decision — corr is the dialog_id for dialog hooks, otherwise null.
  contract/process-log-corr/hook-invoked · Process log corr by event · hook-invoked — corr is the dialog_id for dialog hooks, otherwise null.
  contract/process-log-corr/panic · Process log corr by event · panic — corr is null.
  contract/process-log-corr/send-confirmed · Process log corr by event · send-confirmed — corr is the send cursor.
  contract/process-log-corr/send-issued · Process log corr by event · send-issued — corr is the send cursor.
  contract/process-log-corr/send-refused · Process log corr by event · send-refused — corr is the send cursor.
  contract/process-log-line/at-instants · the process log line · *_at — Instants are named *_at and written in RFC 3339 UTC with milliseconds and a Z suffix.
  contract/process-log-line/duration-ms · the process log line · duration_ms — Durations are duration_ms as an integer.
  contract/process-log-line/enum-values · the process log line · enum values — Enum values are kebab-case.
  contract/process-log-line/field-names · the process log line · field names — Field names are snake_case.
  contract/process-log/corr · Process log line · corr — Copied unchanged as a JSON number or string, never renamed.
  contract/process-log/instance · Process log line · instance — A ViolaName or null.
  contract/process-log/level · Process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/process-log/message · Process log line · message — A required field, as fields.message or the flattened message.
  contract/process-log/process · Process log line · process — The process role, one of run, hook, mcp, cli.
  contract/process-log/target · Process log line · target — A required field.
  contract/process-log/timestamp · Process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  decision/diag-lines-schema-checked-in-unit-tests · Emitted lines schema-checked in integration and E2E tests — Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
  decision/machine-parseable-signals · Every signal machine-parseable — Every signal produces machine-parseable output: JSON-per-line process logs, the ndjson product event stream, JSON harness documents, hyperfine JSON for perf gates and nextest JUnit XML in CI.
  decision/schema-conformance-gate · Schema conformance gate on real process output — Every role-file line of test homes must validate against schemas/diag-line.v1.json and every detail line against schemas/diag-detail.v1.json; a failing line fails the build, a torn non-JSON line is skipped and counted. · why: schema failures count as a build failure and unit tests check only in-memory lines · guard: gate G4 (schema-conformance step)
  decision/schema-top-level-unevaluated-properties · Default-deny schema via top-level unevaluatedProperties — The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists. · why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then · guard: contract_diag_schema
- S3 · a new test's run must leave zero panic lines in its home-level files; the zero-panics gate reads them · ci-integration · 9-ci-integration
  decision/artifact-panic-count-excludes-detail · Artifact panic count excludes detail files — A panic count over a downloaded diagnostics artifact excludes detail files. · why: detail files repeat the panic line
  decision/build-fails-on-any-panic · Build fails on any panic line — The build fails on any panic event line in the kept homes' role files, bar the fake-agent seam's forced panic; no gate infers a panic from an exit code.
  decision/error-budget-zero-tolerance · Zero-tolerance error budget — The error budget tolerates zero panic lines in any CI artifact, zero secret-scan hits and zero schema-conformance failures. · except: the fake-agent seam's deliberate panic, as the zero-panics gate counts it
  decision/g2-zero-panics-gate · Zero-panics gate fails closed — The zero-panics gate probes itself first, fails closed, runs in the test and perf jobs, and counts panics over home-level role files only. · guard: G2 · except: the fake-agent seam's forced panic at exactly its seam location, compared as a whole string
  decision/no-obs-gate-on-verify-lines · No obs gate depends on verify lines — No observability gate depends on verify's own log lines; its home-level lines fall under the same panic, schema and secret-scan gates as any other.
  decision/panic-count-home-level-only · Panic count reads home-level files only — The exactly-one panic rule and gate G2 count home-level role files only; the companion panic line in a detail file is excluded. · guard: gate G2 · except: G2 alone leaves out the fake-agent seam's deliberate panic, whose line is still written exactly once.
  decision/panic-lines-stay-zero · Panic lines stay at zero — Gate G2 counts the panic lines of the home-level role files and fails on any; no viola.panic metric is derived.
  decision/zero-panics-gate · Zero panics gate — No panic line may appear in the home-level role files of test runs; detail files repeat the panic line and are excluded. · why: ratified · guard: gate G2 (scripts/g2-zero-panics.sh, --probe first) · except: a panic whose panic_location is exactly src/cmd/hook/seam.rs:<digits>, the fake-agent seam's forced panic, compared as a whole string
- S4 · a new or changed fake-agent script is synthetic input, never a recording of claude and never a telemetry source · pii-scrubbing-compliance · 8-pii-scrubbing-compliance
  decision/canary-only-in-detail-files · Canary only in detail files — Every E2E and integration input is synthetic and embeds a fixed canary, which may appear in detail files but never in a home-level diagnostics file. · guard: mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line
  decision/claude-fixtures-scrub-required · Claude fixtures are scrub-required — Build and CI infrastructure is not instrumented, and the claude fixtures are scrub-required data, never a telemetry source. · why: Not product runtime.
  decision/detail-upload-synthetic-inputs-only · Detail-file upload only with synthetic inputs — Detail files and the harness capture are uploaded only while every input is synthetic; a non-synthetic source first drops them from the upload paths. · why: every input is synthetic, so detail content carries no real data
  decision/fixtures-not-a-telemetry-source · Fixtures not a telemetry source — Recorded fixtures are not a telemetry source, and observability adds no fixture content.
  decision/fixtures-not-telemetry · Recorded fixtures are no telemetry source — Recorded fixtures are scrub-required data, not a telemetry source, and are excluded from the observability plan. · why: Not product runtime.
- S5 · a new test waits on a logged line by reading it, never by a sleep, and asserts its budget by exit code · 10-slo-invariants-telemetry-budgets · slo-invariants-telemetry-budgets
  decision/no-human-gated-log-analysis · No human-review-gated log analysis — Every log verdict is a jq or harness assertion that sets an exit code, never a human review.
