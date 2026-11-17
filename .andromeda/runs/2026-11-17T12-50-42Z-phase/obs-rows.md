# obs-plan · the rows that bind 2026-11-17-supply-chain-green-and-booted-tests-end — picked 105 of 1024 (28 by every draw) · rule any · pin 2304

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/obs-deps-tokio-free · Obs dependencies Tokio-free and C-free — Obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass cargo deny. · guard: cargo deny
decision/rmcp-stdio-mcp-server · rmcp stdio MCP server — The viola-mcp crate is an rmcp stdio MCP server.
contract/diag-line/instance · diagnostic log line · instance — The instance's `ViolaName`; the key is absent when the process has no instance.
contract/diag-line/process · diagnostic log line · process — The role of the writing process, one of run, hook, mcp, cli; required on every line and matching `sender` / `writer`.
contract/diag-line/service-name · diagnostic log line · service.name — Hardcoded to "viola".
contract/diag-line/service-version · diagnostic log line · service.version — The crate version, fixed at compile time.
contract/log-line/process · every log line · process — Every log line carries a process key naming its role: one of run, hook, mcp, cli.

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/linux-only-os-target · Linux-only OS target — The OS target is Linux only: the core on any current distribution, CI on ubuntu-latest.
decision/obs-deps-pass-cargo-deny · Obs dependencies pass cargo deny — Observability dependencies pass cargo deny on its one target, x86_64-unknown-linux-gnu. · guard: cargo deny
decision/panic-hook-replaced · Panic hook replaced by a one-line writer — Each process replaces the default panic hook, as the first statement in main and before any child spawns, with one that writes one JSON line and nothing to stderr. · why: The default hook prints to stderr, which breaks the hook contract and is the child's terminal under run.
decision/child-exit-from-handle-wait · Child exit from the process handle — The child's process-exit comes from the wait on its process handle, never from reader end-of-file. · why: Child exit is detected on the process handle.
decision/claude-exit-on-process-handle · Claude exit detected on the process handle — The exit of the hosted claude process is detected on the process handle, never on EOF.
decision/instance-from-name-not-cwd · Instance from its name, never the working directory — The instance comes from VIOLA_NAME or an explicit argument, never from the working directory.
contract/viola-verify/home-tree-refused · viola verify · refused home-tree check — A refused home-tree check exits 1 with the home pair, opens no log and writes no line; a refused update_stamps exits 1 with the home pair and, when an instance is named, ends the opened log with a process-exit line carrying detail strict-modes-failed.

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/one-line-panic-record · One structured line per panic — Every panic after init step 4 produces exactly one single-line structured record in its home-level role file with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file. · except: the bounded panic exemptions listed in the plan's Section 10

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/panic-one-line-record · One single-line record a panic — Every panic after obs init produces exactly one single-line structured record in its home-level role file, with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file. · except: the pre-init window and the paths that refuse or lack an instance or home tree before their role file opens
decision/panic-caught-at-every-role · Panics caught at every role's catch site — A hand-written panic hook plus catch_unwind at every role's main-thread catch site (covering hook dispatch and run's inline handle-wait), the two run PTY pump threads and the vt100 feed turn a panic into one record.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.
decision/verify-logs-only-with-instance · verify and plugin install log only inside an instance — With VIOLA_NAME set, verify and plugin install log process-start / process-exit to cli-<name>.ndjson; from a plain terminal with no instance they write no process-log file.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/no-obs-gate-on-verify-lines · No obs gate depends on verify lines — No observability gate depends on verify's own log lines; its home-level lines fall under the same panic, schema and secret-scan gates as any other.
decision/real-cli-verify-never-in-ci · Real-CLI verify never runs in CI — In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
decision/verify-sole-stamps-writer · verify is the only stamps writer — verify is the only writer of the capability ledger's stamps file.
contract/claude-child-process-log/handle-wait-exit · run process log, claude-child lines · process-start and process-exit of claude-child — process-start is logged once the child is spawned and process-exit once the pump returns; process-exit{subject:"claude-child"} carries child_exit_status and exit_source handle-wait or kill-fallback, and is never emitted on reader EOF.
contract/run-start-failure-detail/socket-dir-refused · run start process-exit · socket-dir-refused — A socket directory the start refuses is one process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"} before any snapshot, child or bind, on viola revive's start too; a bind failure keeps its own outcome.
contract/verify-spawn-subject/agents-probe · verify child spawn subjects · agents-probe — Run B's single agents --json read is logged as one process-start / process-exit pair with subject agents-probe.
contract/verify-spawn-subject/verify-probe · verify child spawn subjects · verify-probe — The print-mode probe is logged as one process-start / process-exit pair with subject verify-probe.
contract/verify-spawn-subject/verify-pty-probe · verify child spawn subjects · verify-pty-probe — Each of verify's five interactive PTY runs is logged as one process-start / process-exit pair with subject verify-pty-probe.
contract/verify-spawn-subject/version-probe · verify child spawn subjects · version-probe — The --version read of the agent CLI is logged as one process-start / process-exit pair with subject version-probe.

## metric-coverage · ## 5. Metric Coverage · twin 5-metric-coverage
decision/panic-lines-stay-zero · Panic lines stay at zero — Gate G2 counts the panic lines of the home-level role files and fails on any; no viola.panic metric is derived.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
contract/log-levels/error · log levels · error — Used for a panic, a process-exit that carries a detail, and a channel-response with error_code -32603.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
contract/diag-line-fields/panic · diagnostic line additive fields · panic — panic_location (workspace-relative, or <crate>-<ver>/src/... for dependencies) and thread; payload and backtrace go to the detail file only.
contract/parse-rejected-detail/panicked · parse-rejected detail · panicked — vt100-feed under catch_unwind.

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/unhandled-panics-always-captured · Unhandled panics always captured — Every unhandled panic is captured as the one-line event "panic".
contract/panic-line/keys · the panic line · keys — The panic line holds timestamp, level "ERROR", target "viola::panic", message "panic", event "panic", process, instance, panic_location and thread.
contract/panic-line/corr · the panic line · corr — corr is never written on the panic line.
contract/panic-line/instance · the panic line · instance — instance is omitted when none resolved, the same key-absence null encoding obs_event! uses.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/default-deny-log-fields · Default-deny for logged fields — A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure. · guard: gate G4 against schemas/diag-line.v1.json
decision/detail-schema-self-contained · Separate self-contained detail-line schema — Detail-file lines are validated by their own schema with a top-level unevaluatedProperties false, which inlines the event enum and uses no cross-file reference.
decision/schema-top-level-unevaluated-properties · Default-deny schema via top-level unevaluatedProperties — The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists. · why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then · guard: contract_diag_schema
contract/detail-line/event · the detail-file line · event — Required on every detail-file line, as on home-level lines.
contract/detail-line/instance · the detail-file line · instance — Required on every detail-file line, as on home-level lines.
contract/detail-line/level · the detail-file line · level — Required on every detail-file line, as on home-level lines.
contract/detail-line/message · the detail-file line · message — Required on every detail-file line, as on home-level lines.
contract/detail-line/panic-location · the detail-file line · panic_location — Allowed on panic detail lines only, copied from the home-level panic line so both lines join without the payload.
contract/detail-line/process · the detail-file line · process — Required on every detail-file line, as on home-level lines.
contract/detail-line/target · the detail-file line · target — Required on every detail-file line, as on home-level lines.
contract/detail-line/thread · the detail-file line · thread — Allowed on panic detail lines only, copied from the home-level panic line so both lines join without the payload.
contract/detail-line/timestamp · the detail-file line · timestamp — Required on every detail-file line, as on home-level lines.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/g2-zero-panics-gate · Zero-panics gate fails closed — The zero-panics gate probes itself first, fails closed, runs in the test and perf jobs, and counts panics over home-level role files only. · guard: G2 · except: the fake-agent seam's forced panic at exactly its seam location, compared as a whole string
decision/g4-schema-conformance-gate · Schema conformance gate on every line — Every home-level line is validated against the diagnostics line schema and every detail line against the detail schema, in a step that always runs. · guard: G4
decision/panic-and-schema-gates-always-run · Panic and schema gates always run — The zero-panics and schema gates run after the integration and E2E tests even when a test step failed. · why: a failed test step must not skip the zero-panics gate
decision/homes-kept-until-gates-run · Test homes kept until the gates run — Integration-test homes are kept until the gate steps have run.
decision/test-homes-under-e2e-home · Integration-test homes under one root — Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: the one chaos_feed_panic test home

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-gates-fail-closed · CI gates fail closed — A gate whose tool is missing, errors or meets an empty scope fails the step rather than passing. · why: an erroring search with a must-be-empty check would pass on every commit · guard: G1, G3, scripts/g2-zero-panics.sh --probe
decision/gates-run-after-failed-tests · Gates run even after failed tests — G2 and G4 run if: always(), so a failed test step never skips them. · why: failing runs are where panic lines are most likely, and panics are measured only through G2's exit code
decision/os-gated-test-census · OS-gated test census — Every OS-gated test function or file must be named in the census with a non-empty reason, and a gate predicate naming windows or macos is a fault even when named. · guard: contract_os_gated_tests
decision/banned-telemetry-dependencies · Banned telemetry and logging dependencies — opentelemetry-otlp, opentelemetry-stdout, sentry, tracing-appender, the tracing-subscriber env-filter feature and the veil toggle feature are not taken as dependencies. · guard: cargo deny check
decision/diag-lines-schema-checked-in-unit-tests · Emitted lines schema-checked in integration and E2E tests — Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
decision/schema-conformance-gate · Schema conformance gate on real process output — Every role-file line of test homes must validate against schemas/diag-line.v1.json and every detail line against schemas/diag-detail.v1.json; a failing line fails the build, a torn non-JSON line is skipped and counted. · why: schema failures count as a build failure and unit tests check only in-memory lines · guard: gate G4 (schema-conformance step)
decision/test-homes-kept-until-gates · Test homes kept until gate steps run — CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it. · why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
decision/test-homes-under-scan-root · Test homes live under the scanned root — Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
decision/zero-panics-gate · Zero panics gate — No panic line may appear in the home-level role files of test runs; detail files repeat the panic line and are excluded. · why: ratified · guard: gate G2 (scripts/g2-zero-panics.sh, --probe first) · except: a panic whose panic_location is exactly src/cmd/hook/seam.rs:<digits>, the fake-agent seam's forced panic, compared as a whole string

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/cargo-deny-failure-fails · cargo deny failure is a failure — A cargo deny failure (Tokio, a C crate, veil toggle, banned obs crates) counts as an SLO invariant failure. · guard: cargo deny
decision/nonzero-count-blocks-chunk · Nonzero error count blocks the chunk — Any nonzero error-budget count makes the chunk red until the root cause is fixed within that chunk. · why: the tests quarantine rule
decision/zero-unlogged-panics · Zero unlogged panics — Every panic in run, hook, mcp and cli after init step 4 produces exactly one single-line ERROR panic record in its home-level role file, with no multi-line backtrace there, and hook still exits 0; retry-once policies for panic handling are forbidden. · guard: G2 · except: the bounded exemptions where no file exists, so no line can be written; G2 alone leaves out the fake-agent seam's deliberate panic from its count.
decision/build-fails-on-any-panic · Build fails on any panic line — The build fails on any panic event line in the kept homes' role files, bar the fake-agent seam's forced panic; no gate infers a panic from an exit code.
decision/error-budget-measured-by-gate-exits · Error budget measured by in-job gate exit codes — The error budget is measured by in-job gate exit codes, not by the uploaded artifact: panics by G2, schema failures by G4, secret hits by the secret-scan step. · why: the diagnostics artifact is not uploaded when the scan fails, so it is the diagnosis input, not the measurement surface
decision/error-budget-zero-tolerance · Zero-tolerance error budget — The error budget tolerates zero panic lines in any CI artifact, zero secret-scan hits and zero schema-conformance failures. · except: the fake-agent seam's deliberate panic, as the zero-panics gate counts it
decision/no-retry-policies · Retry policies forbidden — Retry-once policies are forbidden, for tests and for panics alike. · why: retry policies mask real signals
decision/no-sleep-based-log-waits · No sleep-based log waits — No log-based wait relies on sleeps.
decision/schema-conformance-failure-fails · Schema-conformance failure is a failure — An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.

## slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin 10-slo-invariants-telemetry-budgets
decision/no-retry-on-panic · No retry on panic — Retry-once policies for panic handling are forbidden. · why: Parallel to the tests' retries = 0.
decision/panic-count-home-level-only · Panic count reads home-level files only — The exactly-one panic rule and gate G2 count home-level role files only; the companion panic line in a detail file is excluded. · guard: gate G2 · except: G2 alone leaves out the fake-agent seam's deliberate panic, whose line is still written exactly once.

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/rerun-no-substitute · Re-run is no substitute for diagnostics — A re-run is not a substitute for the failing job's diagnostics. · why: CI runs under retries = 0.
decision/default-panic-hook-never-runs · Default panic hook never runs — The default panic hook is never called or allowed to run. · why: Its stderr output breaks hook and the run terminal.
decision/one-line-panics · Panics are one line — Multi-line stack traces are never written; panics are logged as one line.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/direct-chrono-dependency · Direct chrono dependency for timestamps — The root binary takes chrono directly, without default features, to produce the millisecond UTC timestamp. · why: tracing-subscriber's chrono feature does not re-export chrono

## log-file-location · key of ## 3. Observability Harness Contract
contract/diagnostics-files/home-resolution · diagnostics files · home resolution — The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.

## log-format-json-schema · key of ## 3. Observability Harness Contract
decision/no-multiline-stack-traces · No multi-line stack traces — Every log line is one physical line parseable by jq; a hook panic is caught and logged as one ERROR line with event panic.
decision/null-as-key-absence · Null written as key absence — A null corr or instance is written as an absent key, readers treat absence and null as equal, and the schema rejects a literal null. · why: tracing has no null field value
decision/obs-aligns-to-tests-log-format · Observability aligns to the tests log format — The log format is bound by the test plan's harness contract; observability may add fields but must not rename them or remove one the product still emits. · why: the harness greps on them
contract/corr-by-event/process-and-panic · corr by event · process-start, process-exit, panic — corr is null.
contract/event-stream/line · the event stream · event line — Each line of instances/<name>/events.ndjson is exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
contract/process-log-line/corr · the process log line · corr — Copied unchanged as a JSON number or string, never renamed and never debug- or display-formatted; required only on the events whose rules always define it.
contract/process-log-line/instance · the process log line · instance — The instance's name, or absent when none resolved.
contract/process-log-line/level · the process log line · level — One of DEBUG, INFO, WARN, ERROR.
contract/process-log-line/message · the process log line · message — The message is a static literal equal to the event name and never carries interpolated content.
contract/process-log-line/process · the process log line · process — One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing cli-<name>.ndjson.
contract/process-log-line/target · the process log line · target — The target is always present.

## logging-stack · key of ## 3. Observability Harness Contract
decision/no-content-in-home-level-lines · No content fields in home-level lines — No home-level diagnostics line carries chain, drift_report, panic_payload or backtrace. · guard: G4 diag-line.v1.json schema
contract/detail-line/fields · the detail line · fields — Each detail line carries timestamp (millisecond UTC format), level, target, message, event, process and instance, then its content fields, and no corr or conn key.
contract/diagnostics-files/detail · the diagnostics files · detail — Content-bearing records (full error chains, drift reports, panic payload and backtrace) go only to <home>/instances/<name>/diagnostics/detail-<process>.ndjson, file 0600 in a 0700 directory, and only when an instance resolves.
contract/diagnostics-line/format · the diagnostics line · format — Each diagnostics line is one structured JSON object on one line, written whole by one write of the complete line on an append-mode handle.
contract/obs-event-macro/attached-fields · the obs_event macro · attached fields — Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.

## otel-sdk-init · key of ## 3. Observability Harness Contract
decision/home-resolution-order · Home resolution order — The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
decision/instance-source-per-role · Instance source fixed per role — run takes its own name argument, never an inherited VIOLA_NAME; hook and mcp take VIOLA_NAME only; cli verbs that act on an instance (send, wait, last, answer, pause, release, link, unlink) take the target argument; list, verify, plugin install and revive --list take the caller's VIOLA_NAME only. · why: A wrapper never writes into another instance's run log.
decision/flat-log-lines-and-one-level-gate · Log lines are flat and Targets is the only level gate — The subscriber flattens event fields to the top level of each line, and the Targets filter is the only configurable level gate; its level comes from the home's diagnostics level and never from the environment. · why: The diagnostics schema and the log readers read event fields as top-level keys, and only the viola crates' targets pass the gate.
decision/home-tree-check-before-log-init · Home tree check before log init — run, revive's start arm and viola verify check the home tree's modes before their log opens; a refusal writes no line, starts no child, leaves modes as found and exits 1.
decision/panic-hook-first · Panic hook installed first — The panic hook is installed as the first statement of main and writes only to the diagnostics file once that is open, never falling back to stderr.

## service-identity · key of ## 3. Observability Harness Contract
contract/diagnostics-line/instance · the diagnostics line · instance — Every diagnostics line of a process with an instance carries it as field instance; a process without one, such as an mcp server with no instance name, writes no instance key.
contract/diagnostics-line/process · the diagnostics line · process — Every diagnostics line carries the role as field process.
contract/process-start/os · the process-start line · os — process-start carries field os, the one word linux.
contract/process-start/pid · the process-start line · pid — process-start carries the process id as field pid.
contract/process-start/service-name · the process-start line · service_name — process-start carries the service name as field service_name.
contract/process-start/version · the process-start line · version — process-start carries the product version as field version.

# obs-plan · its absences, names and overviews, whole

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
absence/audit-log-compliance-trail — An audit-log compliance trail
absence/otlp-receiver — An OTLP or pulse-class telemetry receiver
absence/pii-scrubbing-regime — A PII-scrubbing regime for diagnostics lines beyond the NEVER-log floor

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
absence/deployment-environments — Deployment environments or hosting
absence/interactive-terminal-interface — An interactive terminal interface: prompt, TUI or pager

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
overview/2-telemetry-strategy — Structured logs are the primary signal, one JSON-per-line file per process role; spans are declared in code but not serialized in v1, and cross-process joins use corr and conn; errors have no external reporter; metrics have no in-process instruments and no exporter and are derived by jq over the ndjson; events.ndjson is the product audit stream, not a compliance trail, and obs never copies event payloads into process logs.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
absence/web-or-graphical-frontend — A web or graphical frontend

## 5-metric-coverage · ## 5. Metric Coverage · twin metric-coverage
absence/in-process-metrics-pipeline — An in-process metrics pipeline, with no listener and no exporter

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
absence/database — A database
absence/grpc — gRPC of any kind

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
absence/otel-span-processing — An OTel SpanProcessor
absence/sentry-error-reporting — Sentry error reporting

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
absence/otel-trace-export — OTel trace export: no exporter

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
absence/on-call-paging — On-call or paging

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
absence/network-egress — Network egress of any kind
absence/telemetry-exporter-stage — A telemetry exporter stage

## service-identity · key of ## 3. Observability Harness Contract
absence/hosted-deployment — A hosted deployment or deployment environment; viola is local-only

## snapshot-paste-to-ai-integration · key of ## 3. Observability Harness Contract
absence/pulse-class-receiver — A pulse-class log receiver
