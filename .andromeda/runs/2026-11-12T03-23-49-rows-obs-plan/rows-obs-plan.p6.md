@row_add n=1 row=decision/veil-redact-second-layer area=8-pii-scrubbing-compliance rec=1
name: veil Redact as the second redaction layer
rule: veil's derive Redact is the second redaction layer, so a stray value capture prints [REDACTED] instead of content.
@row_add n=2 row=decision/veil-toggle-feature-banned area=8-pii-scrubbing-compliance rec=1
name: veil toggle feature banned
rule: veil's non-default toggle feature, which lets redaction be disabled at runtime, is never enabled.
guard: cargo-deny bans.features entry denying toggle on veil
@row_add n=3 row=decision/no-regex-scrubbers area=8-pii-scrubbing-compliance rec=2
name: No regex scrubbers
rule: Sensitive data is never scrubbed by regular expressions.
why: viola's sensitive data is known by type
@row_add n=4 row=decision/redact-crate-not-adopted area=8-pii-scrubbing-compliance rec=3
name: redact crate not adopted
rule: The redact crate, the researched lighter alternative to veil, is not adopted.
@row_add n=5 row=absence/otel-span-processing area=8-pii-scrubbing-compliance rec=4
capability: An OpenTelemetry span processor or any OpenTelemetry pipeline
@row_add n=6 row=absence/sentry-error-reporting area=8-pii-scrubbing-compliance rec=4
capability: Sentry error reporting
@row_add n=7 row=decision/source-allow-list-logging area=8-pii-scrubbing-compliance rec=5
name: Source allow-list for logged fields
rule: Spans skip all arguments and declare fields explicitly, and events go through obs_event! with explicit typed fields only; this source allow-list is the primary PII control.
guard: clippy disallowed-macros on tracing level macros; fail-closed raw event! grep; grep gate on bare instrument
@row_add n=8 row=decision/payload-types-derive-redact area=8-pii-scrubbing-compliance rec=6
name: Type-level redaction of payload types
rule: Every channel, hook and MCP payload type derives veil Redact, so Debug never reveals content.
@row_add n=9 row=decision/fixed-error-messages area=8-pii-scrubbing-compliance rec=7
name: Fixed error messages
rule: Error Display texts are fixed, and serde-sourced error chains are mapped to fixed messages at dispatch.
@row_add n=10 row=decision/content-only-in-detail-file area=8-pii-scrubbing-compliance rec=8
name: Content-bearing records only in the detail file
rule: Content-bearing records go only to the instance-scoped detail-<process>.ndjson file.
@row_add n=11 row=decision/secret-scan-before-upload area=8-pii-scrubbing-compliance rec=9
name: Secret scan before artifact upload
rule: CI runs the secret scan for the Cookie header, CLAUDE* values, the content canary and 0600 permissions over every diagnostics file before any artifact upload.
@row_add n=12 row=decision/content-canary-never-home-level area=8-pii-scrubbing-compliance rec=9
name: Content canary never in home-level files
rule: The content canary never appears in a home-level diagnostics ndjson file.
@row_add n=13 row=decision/detail-upload-synthetic-inputs-only area=8-pii-scrubbing-compliance rec=9
name: Detail-file upload only with synthetic inputs
rule: Detail files and the harness capture are uploaded only while every input is synthetic; a non-synthetic source first drops them from the upload paths.
why: every input is synthetic, so detail content carries no real data
@row_add n=14 row=decision/scan-failure-uploads-hit-report-only area=8-pii-scrubbing-compliance rec=9,33
name: Only the hit report uploaded on scan failure
rule: When the secret scan fails, only its hit report is uploaded, as the secret-scan-<os> artifact.
@row_add n=15 row=contract/secret-scan-hit-report/file area=8-pii-scrubbing-compliance rec=10
boundary: the secret-scan hit report
item: file
promise: Each hit names the file it was found in.
@row_add n=16 row=contract/secret-scan-hit-report/line area=8-pii-scrubbing-compliance rec=10
boundary: the secret-scan hit report
item: line
promise: Each hit names the line it was found on.
@row_add n=17 row=contract/secret-scan-hit-report/byte-offset area=8-pii-scrubbing-compliance rec=10
boundary: the secret-scan hit report
item: byte offset
promise: Each hit names the byte offset of the match.
@row_add n=18 row=contract/secret-scan-hit-report/pattern-class area=8-pii-scrubbing-compliance rec=10
boundary: the secret-scan hit report
item: pattern class
promise: Each hit names the class of pattern that matched.
@row_add n=19 row=decision/hit-report-never-matched-bytes area=8-pii-scrubbing-compliance rec=10
name: Hit report never carries matched bytes
rule: The secret-scan hit report never contains the matched bytes.
@row_add n=20 row=decision/upload-members-scanned-or-dropped area=8-pii-scrubbing-compliance rec=11
name: Upload members scanned or dropped
rule: An uploaded member that could carry a host path or log content is scanned before upload or dropped; unscanned uploads are admissible by content only.
@row_add n=21 row=decision/local-run-outputs-never-uploaded area=8-pii-scrubbing-compliance rec=11
name: Local run outputs never uploaded
rule: mutants.out, the run archive and the local viola-harness pre-push document are never uploaded.
@row_add n=22 row=decision/fuzz-artifacts-synthetic-seeds-only area=8-pii-scrubbing-compliance rec=11
name: Fuzz artifacts uploaded only with synthetic seeds
rule: A non-synthetic fuzz corpus seed first drops the fuzz artifacts upload.
@row_add n=23 row=decision/default-deny-log-fields area=8-pii-scrubbing-compliance rec=12
name: Default-deny for logged fields
rule: A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure.
guard: gate G4 against schemas/diag-line.v1.json
@row_add n=24 row=decision/schema-top-level-unevaluated-properties area=8-pii-scrubbing-compliance rec=12
name: Default-deny schema via top-level unevaluatedProperties
rule: The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists.
why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then
guard: contract_diag_schema
@row_add n=25 row=decision/no-allow-list-widening area=8-pii-scrubbing-compliance rec=12
name: Allow-list cannot be widened
rule: No config key, flag or env var can widen the logged-field allow-list or disable redaction.
@row_add n=26 row=decision/detail-schema-self-contained area=8-pii-scrubbing-compliance rec=13
name: Separate self-contained detail-line schema
rule: Detail-file lines are validated by their own schema with a top-level unevaluatedProperties false, which inlines the event enum and uses no cross-file reference.
@row_add n=27 row=contract/detail-line/timestamp area=8-pii-scrubbing-compliance rec=13
boundary: the detail-file line
item: timestamp
promise: Required on every detail-file line, as on home-level lines.
@row_add n=28 row=contract/detail-line/level area=8-pii-scrubbing-compliance rec=13
boundary: the detail-file line
item: level
promise: Required on every detail-file line, as on home-level lines.
@row_add n=29 row=contract/detail-line/target area=8-pii-scrubbing-compliance rec=13
boundary: the detail-file line
item: target
promise: Required on every detail-file line, as on home-level lines.
@row_add n=30 row=contract/detail-line/message area=8-pii-scrubbing-compliance rec=13
boundary: the detail-file line
item: message
promise: Required on every detail-file line, as on home-level lines.
@row_add n=31 row=contract/detail-line/event area=8-pii-scrubbing-compliance rec=13
boundary: the detail-file line
item: event
promise: Required on every detail-file line, as on home-level lines.
@row_add n=32 row=contract/detail-line/process area=8-pii-scrubbing-compliance rec=13
boundary: the detail-file line
item: process
promise: Required on every detail-file line, as on home-level lines.
@row_add n=33 row=contract/detail-line/instance area=8-pii-scrubbing-compliance rec=13
boundary: the detail-file line
item: instance
promise: Required on every detail-file line, as on home-level lines.
@row_add n=34 row=contract/detail-line/corr area=8-pii-scrubbing-compliance rec=13
boundary: the detail-file line
item: corr
promise: Carried on a detail-file line when its home-level counterpart carries it.
@row_add n=35 row=contract/detail-line/conn area=8-pii-scrubbing-compliance rec=13
boundary: the detail-file line
item: conn
promise: Carried on a detail-file line when its home-level counterpart carries it.
@row_add n=36 row=contract/detail-line/panic-location area=8-pii-scrubbing-compliance rec=13
boundary: the detail-file line
item: panic_location
promise: Allowed on panic detail lines only, copied from the home-level panic line so both lines join without the payload.
@row_add n=37 row=contract/detail-line/thread area=8-pii-scrubbing-compliance rec=13
boundary: the detail-file line
item: thread
promise: Allowed on panic detail lines only, copied from the home-level panic line so both lines join without the payload.
@row_add n=38 row=contract/detail-line/panic-payload area=8-pii-scrubbing-compliance rec=13
boundary: the detail-file line
item: panic_payload
promise: One of exactly four content fields a detail-file line may carry: the panic's payload.
@row_add n=39 row=contract/detail-line/backtrace area=8-pii-scrubbing-compliance rec=13
boundary: the detail-file line
item: backtrace
promise: One of exactly four content fields a detail-file line may carry: the panic's backtrace.
@row_add n=40 row=contract/detail-line/chain area=8-pii-scrubbing-compliance rec=13
boundary: the detail-file line
item: chain
promise: One of exactly four content fields a detail-file line may carry: the full error chain.
@row_add n=41 row=contract/detail-line/drift-report area=8-pii-scrubbing-compliance rec=13
boundary: the detail-file line
item: drift_report
promise: One of exactly four content fields a detail-file line may carry: the drift report.
@row_none n=1 rec=14 type=S
@row_none n=2 rec=15 type=S
@row_none n=3 rec=16 type=S
@row_add n=42 row=decision/ci-on-github-actions area=9-ci-integration rec=17
name: CI on GitHub Actions
rule: CI runs on GitHub Actions in one push and pull-request workflow on an ubuntu-latest matrix, beside a scheduled nightly workflow for advisory checks and fuzzing; there is no OTLP collector job.
@row_none n=4 rec=18 type=S
@row_none n=5 rec=19 type=S
@row_add n=43 row=decision/test-homes-under-e2e-home area=9-ci-integration rec=20
name: Integration-test homes under one root
rule: Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it.
why: a hook panic exits 0, so a home outside that root would hide it
except: the one chaos_feed_panic test home
@row_none n=6 rec=21 type=6
@row_add n=44 row=decision/g1-bare-instrument-gate area=9-ci-integration rec=22
name: Bare instrument gate fails closed
rule: Every instrument attribute must have skip_all as its first argument, and the gate passes only on a clean no-match, so a violation, a search error or a missing search tool all fail.
guard: G1
@row_add n=45 row=decision/g2-zero-panics-gate area=9-ci-integration rec=23
name: Zero-panics gate fails closed
rule: The zero-panics gate probes itself first, fails closed, runs in the test and perf jobs, and counts panics over home-level role files only.
guard: G2
except: the fake-agent seam's forced panic at exactly its seam location, compared as a whole string
@row_add n=46 row=decision/g4-schema-conformance-gate area=9-ci-integration rec=24
name: Schema conformance gate on every line
rule: Every home-level line is validated against the diagnostics line schema and every detail line against the detail schema, in a step that always runs.
guard: G4
@row_add n=47 row=decision/gate-output-never-line-content area=9-ci-integration rec=24
name: Schema gate output never shows line content
rule: The schema conformance gate prints file, line and keyword of a failure, never the line's content.
guard: G4
@row_add n=48 row=decision/no-abort-panic-strategy area=9-ci-integration rec=25
name: No abort panic strategy
rule: No panic strategy is set to abort in Cargo.toml, cargo config or workflows; the gate passes only on a clean no-match.
guard: G3
@row_add n=49 row=decision/no-ci-attributes-in-log-lines area=9-ci-integration rec=26
name: No CI resource attributes in log lines
rule: deployment.environment, ci.run.id and git.commit.sha are never injected into viola log lines; CI identity lives in artifact names and GitHub run metadata.
why: env vars are not a configuration channel
@row_add n=50 row=decision/artifact-retention-seven-days area=9-ci-integration rec=27
name: Seven-day artifact retention
rule: CI artifacts are retained for 7 days, below the 90-day default.
@row_none n=7 rec=28 type=S
@row_add n=51 row=decision/panic-and-schema-gates-always-run area=9-ci-integration rec=29
name: Panic and schema gates always run
rule: The zero-panics and schema gates run after the integration and E2E tests even when a test step failed.
why: a failed test step must not skip the zero-panics gate
@row_add n=52 row=decision/perf-gates-own-job area=9-ci-integration rec=29
name: Perf gates in their own job
rule: The hyperfine performance gates run in their own perf job.
@row_add n=53 row=decision/homes-kept-until-gates-run area=9-ci-integration rec=29
name: Test homes kept until the gates run
rule: Integration-test homes are kept until the gate steps have run.
@row_none n=8 rec=30 type=6
@row_add n=54 row=decision/secret-scan-always-runs area=9-ci-integration rec=31
name: Secret scan always runs
rule: The secret scan step runs even when an earlier step failed.
why: otherwise a failed E2E step skips the scan and the gated uploads, which loses the failing job's diagnostics
@row_add n=55 row=decision/secret-scan-scope area=9-ci-integration rec=31
name: Secret scan scope
rule: The secret scan covers every test-home diagnostics file, the harness capture and the nextest JUnit file.
why: nextest stores failing tests' captured output in the JUnit file
except: the mutants run's chunk diff, skipped by exact path and excluded from the harness upload
@row_add n=56 row=decision/uploads-gated-on-secret-scan area=9-ci-integration rec=32
name: Uploads gated on a passing secret scan
rule: The diagnostics, harness and JUnit uploads run only when the secret scan succeeded.
@row_none n=9 rec=34 type=S
@row_none n=10 rec=35 type=S
@row_none n=11 rec=36 type=S
@row_none n=12 rec=37 type=S
@row_add n=57 row=decision/zero-unlogged-panics area=10-slo-invariants-telemetry-budgets rec=38
name: Zero unlogged panics
rule: Every panic in run, hook, mcp and cli after init step 4 produces exactly one single-line ERROR panic record in its home-level role file, with no multi-line backtrace there, and hook still exits 0.
guard: G2
except: the bounded exemptions where no file exists, and the fake-agent seam's deliberate panic
@row_add n=58 row=decision/no-retry-policies area=10-slo-invariants-telemetry-budgets rec=38,53
name: Retry policies forbidden
rule: Retry-once policies are forbidden, for tests and for panics alike.
why: retry policies mask real signals
@row_add n=59 row=decision/exit-code-panic-evidence area=10-slo-invariants-telemetry-budgets rec=39
name: Exit-code panic evidence
rule: A panic evidenced by an exit code without its log line fails the build only for a process whose process-start is in a home-level role file; for the exemptions the exit status is the only evidence.
@row_add n=60 row=decision/hook-panic-without-name-unobservable area=10-slo-invariants-telemetry-budgets rec=39
name: Hook panic without VIOLA_NAME unobservable
rule: A hook panic without VIOLA_NAME set is unobservable by design.
@row_none n=13 rec=40 type=S
@row_add n=61 row=decision/heartbeat-liveness area=10-slo-invariants-telemetry-budgets rec=41,46
name: Heartbeat file as the liveness signal
rule: run touches the heartbeat every 1 s and no process logs a transition; liveness flips to stale at more than 5 s.
why: replaces periodic tick lines
guard: chaos class stale heartbeat vs live pid, through agent-run status
@row_add n=62 row=decision/module-boundary-error-logging area=10-slo-invariants-telemetry-budgets rec=42
name: Errors logged once at module boundaries
rule: Every typed error that crosses a crate seam is logged once at WARN or ERROR with a closed detail or error_code; the full chain goes only to the instance detail file.
@row_add n=63 row=decision/correlation-propagation area=10-slo-invariants-telemetry-budgets rec=43
name: Correlation propagation
rule: Every channel call logs corr and conn on both sides, every send line carries the send cursor, and every dialog line carries dialog_id.
@row_add n=64 row=decision/no-w3c-traceparent area=10-slo-invariants-telemetry-budgets rec=43
name: W3C traceparent not used
rule: W3C traceparent is not used for correlation.
@row_add n=65 row=decision/deterministic-emission area=10-slo-invariants-telemetry-budgets rec=44
name: Deterministic event emission
rule: Events key on hook events and events.ndjson offsets; nothing is derived from screen content.
@row_add n=66 row=decision/no-sleep-based-log-waits area=10-slo-invariants-telemetry-budgets rec=44
name: No sleep-based log waits
rule: No log-based wait relies on sleeps.
@row_add n=67 row=decision/hook-perf-budgets-timed area=10-slo-invariants-telemetry-budgets rec=45
name: Hook performance budgets timed with hyperfine
rule: The session-start, user-prompt-submit, stop, session-end and pre-tool-use hooks are timed with hyperfine on an unstamped session in the perf job, and the perf gate requires all five; the async tier is untimed.
guard: gate --require perf
@row_add n=68 row=decision/perf-gate-max-sample area=10-slo-invariants-telemetry-budgets rec=46,47
name: Performance budgets gate the max sample
rule: Every performance budget gates hyperfine's max sample; p99 is not computed.
why: max is an upper bound on every quantile
guard: gate --require perf
@row_add n=69 row=decision/session-end-hook-budget area=10-slo-invariants-telemetry-budgets rec=46
name: session-end hook budget
rule: viola hook session-end completes in under 1.0 s.
guard: gate --require perf
@row_add n=70 row=decision/spine-hook-budget area=10-slo-invariants-telemetry-budgets rec=46
name: Spine hook budget
rule: The spine hooks and pre-tool-use on an unverified CLI complete under the tests-owned spine deadline, provisionally 1.0 s.
guard: gate --require perf
state: provisional
@row_add n=71 row=decision/hook-obs-overhead-budget area=10-slo-invariants-telemetry-budgets rec=46
name: Hook observability overhead budget
rule: Observability overhead on hook is held inside the same perf gate, which holds only when every hook command runs with VIOLA_NAME set and a home under the test-home root.
guard: gate --require perf
@row_add n=72 row=decision/no-request-ratio-slo area=10-slo-invariants-telemetry-budgets rec=48
name: No request-ratio SLO
rule: There is no request-ratio SLO; the error budget is a set of zero-tolerance counts.
why: viola is a local tool with no deploys
@row_add n=73 row=decision/error-budget-zero-tolerance area=10-slo-invariants-telemetry-budgets rec=49
name: Zero-tolerance error budget
rule: The error budget tolerates zero panic lines in any CI artifact, zero secret-scan hits and zero schema-conformance failures.
except: the fake-agent seam's deliberate panic, as the zero-panics gate counts it
@row_add n=74 row=decision/error-budget-measured-by-gate-exits area=10-slo-invariants-telemetry-budgets rec=50
name: Error budget measured by in-job gate exit codes
rule: The error budget is measured by in-job gate exit codes, not by the uploaded artifact: panics by G2, schema failures by G4, secret hits by the secret-scan step.
why: the diagnostics artifact is not uploaded when the scan fails, so it is the diagnosis input, not the measurement surface
@row_add n=75 row=decision/artifact-panic-count-excludes-detail area=10-slo-invariants-telemetry-budgets rec=50
name: Artifact panic count excludes detail files
rule: A panic count over a downloaded diagnostics artifact excludes detail files.
why: detail files repeat the panic line
@row_add n=76 row=decision/nonzero-count-blocks-chunk area=10-slo-invariants-telemetry-budgets rec=51
name: Nonzero error count blocks the chunk
rule: Any nonzero error-budget count makes the chunk red until the root cause is fixed within that chunk.
why: the tests quarantine rule
@row_add n=77 row=absence/on-call-paging area=10-slo-invariants-telemetry-budgets rec=51
capability: On-call or paging
@row_add n=78 row=decision/no-log-rotation area=10-slo-invariants-telemetry-budgets rec=52
name: No log rotation
rule: events.ndjson and diagnostics files are not rotated; any future diagnostics rotation must keep 0600/0700 permissions and never touch events.ndjson offsets.
@row_none n=14 rec=54 type=S
@row_add n=79 row=decision/build-fails-on-any-panic area=10-slo-invariants-telemetry-budgets rec=55,54
name: Build fails on any panic
rule: The build fails on any panic event line, or on any panic evidenced by an exit code without its line.
