@row_add n=1 row=decision/hyperfine-max-regression-fails area=10-slo-invariants-telemetry-budgets rec=1
name: Hyperfine max regression is a failure
rule: A hyperfine max regression beyond the telemetry latency budgets counts as an SLO invariant failure.
@row_add n=2 row=decision/secret-scan-hit-fails area=10-slo-invariants-telemetry-budgets rec=2
name: Secret-scan hit is a failure
rule: A secret-scan hit counts as an SLO invariant failure.
@row_add n=3 row=decision/diagnostics-files-mode-0600 area=10-slo-invariants-telemetry-budgets rec=2
name: Diagnostics files are 0600
rule: A diagnostics file whose mode is not 0600 counts as an SLO invariant failure.
@row_add n=4 row=decision/schema-conformance-failure-fails area=10-slo-invariants-telemetry-budgets rec=3
name: Schema-conformance failure is a failure
rule: An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.
@row_add n=5 row=decision/print-and-macro-lint-failure-fails area=10-slo-invariants-telemetry-budgets rec=4
name: Print and macro lint failure is a failure
rule: A clippy disallowed-macros, print_stdout, print_stderr or dbg_macro failure counts as an SLO invariant failure.
@row_add n=6 row=decision/lint-probes-failure-fails area=10-slo-invariants-telemetry-budgets rec=4
name: Lint-probes failure is a failure
rule: A lint-probes failure (a ban that did not fire, a control that did not pass, or a raw event! outside the core obs module) counts as an SLO invariant failure.
@row_add n=7 row=decision/g1-bare-instrument-gate area=10-slo-invariants-telemetry-budgets rec=4
name: G1 bare-instrument gate
rule: A G1 hit on a bare #[instrument] counts as an SLO invariant failure.
guard: G1
@row_add n=8 row=decision/g3-panic-abort-gate area=10-slo-invariants-telemetry-budgets rec=4
name: G3 panic-abort gate
rule: A G3 hit on panic = "abort" counts as an SLO invariant failure.
guard: G3
@row_add n=9 row=decision/grep-gates-fail-closed area=10-slo-invariants-telemetry-budgets rec=4
name: Grep gates fail closed
rule: The G1 and G3 gates fail on any rg exit other than 1, including exit 2 on a regex or I/O error and 127 on a missing rg.
@row_add n=10 row=decision/cargo-deny-failure-fails area=10-slo-invariants-telemetry-budgets rec=4
name: cargo deny failure is a failure
rule: A cargo deny failure (Tokio, a C crate, veil toggle, banned obs crates) counts as an SLO invariant failure.
guard: cargo deny
@row_add n=11 row=decision/surviving-obs-mutant-fails area=10-slo-invariants-telemetry-budgets rec=5
name: Surviving obs mutant is a failure
rule: A surviving cargo-mutants mutant in obs code at a run --mutants (the epoch-boundary audit or on demand) counts as an SLO invariant failure.
except: A missed mutant the host never compiles is not counted as surviving.
@row_none n=1 rec=6 type=S
@row_none n=2 rec=7 type=S
@row_none n=3 rec=8 type=S
@row_add n=12 row=decision/no-hot-path-instrumentation area=11-obs-anti-patterns-never-do-these rec=9
name: No hot-path instrumentation
rule: The PTY pump read loop and the vt100 feed get no per-byte or per-chunk span or event; only gate outcomes are logged.
@row_add n=13 row=decision/must-trace-paths-always-traced area=11-obs-anti-patterns-never-do-these rec=10
name: Must-trace paths always traced
rule: Spans and events are never skipped on the 6 must-trace paths.
why: The agent loses its ability to debug without them.
@row_add n=14 row=decision/closed-event-and-corr-on-every-line area=11-obs-anti-patterns-never-do-these rec=11
name: Closed event enum and corr on every line
rule: Every log line carries the closed event enum and corr, so the signal pyramid never collapses into free text.
@row_add n=15 row=decision/no-otel-without-decisions-log area=11-obs-anti-patterns-never-do-these rec=12
name: No OTel without a Decisions Log entry
rule: No OTel SDK, OTLP exporter or opentelemetry-stdout is added without a Decisions Log entry.
why: opentelemetry-otlp's default reqwest-blocking-client spins up a Tokio runtime.
@row_add n=16 row=decision/no-otel-stdout-exporter area=11-obs-anti-patterns-never-do-these rec=12
name: OTel stdout exporter inadmissible
rule: The OTel stdout exporter is inadmissible on every role.
why: stdout is reserved.
@row_add n=17 row=decision/no-telemetry-from-screen-content area=11-obs-anti-patterns-never-do-these rec=13
name: No telemetry from screen content
rule: Telemetry is never derived from screen content; vt100 is used for readiness and modal gate outcomes only.
@row_add n=18 row=decision/transcripts-not-a-contract area=11-obs-anti-patterns-never-do-these rec=13
name: Claude Code transcripts are not a contract
rule: Claude Code transcripts are never read as a contract.
@row_add n=19 row=decision/no-pii-in-span-fields area=11-obs-anti-patterns-never-do-these rec=14
name: No PII or content in span fields
rule: Span fields never include PII or content.
why: Scrubbing at an exporter stage is too late, and v1 has no exporter stage at all.
@row_add n=20 row=absence/telemetry-exporter-stage area=11-obs-anti-patterns-never-do-these rec=14
capability: A telemetry exporter stage
@row_add n=21 row=decision/no-bare-instrument area=11-obs-anti-patterns-never-do-these rec=15
name: No bare #[instrument]
rule: Instrumentation always uses #[instrument(skip_all, fields(...))], never a bare #[instrument].
why: A bare #[instrument] records every argument with Debug and leaks send text, hook.dialog payloads and tool input.
@row_add n=22 row=decision/no-instrument-err-ret-on-sensitive area=11-obs-anti-patterns-never-do-these rec=16
name: No instrument err or ret on sensitive results
rule: #[instrument(err)] and #[instrument(ret)] are never used where the error or return can carry a serde or serde_path_to_error source or user content.
@row_add n=23 row=decision/low-cardinality-span-names area=11-obs-anti-patterns-never-do-these rec=17
name: Low-cardinality span names
rule: Span names never contain instance names, dialog ids or paths.
@row_add n=24 row=decision/explicit-corr-conn-instance area=11-obs-anti-patterns-never-do-these rec=18
name: corr, conn and instance passed explicitly
rule: corr, conn and instance are passed as explicit values across threads or tasks, never carried by span context.
why: Spans are not serialized.
@row_add n=25 row=decision/no-in-process-metrics area=11-obs-anti-patterns-never-do-these rec=19
name: No in-process metrics instruments
rule: No in-process metrics instruments (opentelemetry meters, metrics crate) are created.
why: There is no reader, exporter or listener to flush them.
@row_add n=26 row=decision/no-telemetry-endpoint area=11-obs-anti-patterns-never-do-these rec=20
name: No telemetry endpoint without a security decision
rule: No /metrics or other telemetry endpoint is added without a Security Decisions Log entry.
why: A telemetry endpoint is a new listener.
@row_add n=27 row=decision/derived-metrics-by-closed-enums area=11-obs-anti-patterns-never-do-these rec=21
name: Derived metrics grouped by closed enums
rule: Derived metrics are grouped by the closed enums only, never by unbounded keys such as path, corr, conn, pid or instance.
@row_add n=28 row=decision/latency-with-gate-statistic area=11-obs-anti-patterns-never-do-these rec=22
name: Latency reported with the gate statistic
rule: Latency is never reported without the gate statistic; p50 and p95 are report-only.
@row_add n=29 row=decision/hyperfine-max-is-enforced area=11-obs-anti-patterns-never-do-these rec=22
name: Hyperfine max is the enforced number
rule: The max from hyperfine is the enforced latency number.
@row_add n=30 row=decision/no-telemetry-on-stdout area=11-obs-anti-patterns-never-do-these rec=23
name: No telemetry on stdout
rule: Telemetry is never written to stdout on any role.
why: stdout carries each role's own output and run's terminal carries the child's screen.
@row_add n=31 row=contract/stdout-by-role/cli area=11-obs-anti-patterns-never-do-these rec=23
boundary: stdout by role
item: cli
promise: stdout carries the --json results.
@row_add n=32 row=contract/stdout-by-role/hook area=11-obs-anti-patterns-never-do-these rec=23
boundary: stdout by role
item: hook
promise: stdout carries the decision body or, for hook statusline, the user's status-line output.
@row_add n=33 row=contract/stdout-by-role/mcp area=11-obs-anti-patterns-never-do-these rec=23
boundary: stdout by role
item: mcp
promise: stdout carries MCP frames.
@row_add n=34 row=contract/stdout-by-role/run area=11-obs-anti-patterns-never-do-these rec=23
boundary: stdout by role
item: run
promise: The terminal of run carries the child's screen.
@row_add n=35 row=decision/explicit-subscriber-writer area=11-obs-anti-patterns-never-do-these rec=23
name: Explicit tracing-subscriber writer
rule: with_writer is always set explicitly on the tracing-subscriber.
why: The default tracing-subscriber writer is stdout.
@row_add n=36 row=decision/log-internal-errors-off area=11-obs-anti-patterns-never-do-these rec=24
name: log_internal_errors turned off
rule: log_internal_errors is never left at its default true.
why: Writer failures would eprintln to stderr and break the Hook contract and the run terminal.
@row_add n=37 row=decision/obs-event-macro-only area=11-obs-anti-patterns-never-do-these rec=25
name: obs_event! is the only event macro
rule: tracing's event and level macros are never called directly outside viola_core::obs; events go through obs_event!.
why: Only obs_event! guarantees event, process and instance.
guard: clippy disallowed-macros; the raw-event! grep in lint-probes
@row_add n=38 row=decision/corr-set-explicitly-by-caller area=11-obs-anti-patterns-never-do-these rec=25
name: corr set explicitly by the caller
rule: The caller sets corr explicitly as a typed field on the events that carry one.
@row_add n=39 row=decision/no-stringified-corr area=11-obs-anti-patterns-never-do-these rec=26
name: corr never recorded with ? or %
rule: corr is never recorded with ?corr or %corr.
why: Those stringify numbers.
@row_add n=40 row=contract/log-line/corr area=11-obs-anti-patterns-never-do-these rec=26
boundary: the log line
item: corr
promise: corr is copied unchanged as a JSON number or string.
@row_add n=41 row=decision/no-env-log-level area=11-obs-anti-patterns-never-do-these rec=27
name: Log level not read from the environment
rule: The level is never read from RUST_LOG, the env-filter feature is never enabled and EnvFilter::from_default_env is never called.
why: Env vars are not a configuration channel.
@row_add n=42 row=decision/one-line-panics area=11-obs-anti-patterns-never-do-these rec=28
name: Panics are one line
rule: Multi-line stack traces are never written; panics are logged as one line.
@row_add n=43 row=decision/backtraces-in-instance-detail-only area=11-obs-anti-patterns-never-do-these rec=28
name: Backtraces in the instance detail file only
rule: Backtraces are written as one-line JSON arrays in the instance detail file only.
@row_add n=44 row=decision/no-info-in-hot-loops area=11-obs-anti-patterns-never-do-these rec=29
name: No info logging in hot loops
rule: Nothing is logged at info inside hot loops such as the PTY pump; counts go on the boundary event instead.
@row_add n=45 row=decision/no-non-blocking-appender area=11-obs-anti-patterns-never-do-these rec=30
name: No non_blocking appender
rule: tracing-appender non_blocking is never used.
why: It drops lines when full, needs a WorkerGuard flush, and breaks the hook's exit-0 / max < 1.0 s gate and the zero-unlogged-panics invariant.
@row_add n=46 row=decision/log-field-names-frozen area=11-obs-anti-patterns-never-do-these rec=31
name: Log field names frozen
rule: Log fields such as corr and event are never renamed without a Decisions Log entry.
@row_add n=47 row=decision/event-values-closed area=11-obs-anti-patterns-never-do-these rec=31
name: event values closed
rule: No event value is added without a Decisions Log entry.
@row_add n=48 row=decision/no-print-macros area=11-obs-anti-patterns-never-do-these rec=32
name: No print macros in product code paths
rule: print!, println!, eprint!, eprintln! and dbg! are never used in the hook, run or mcp code paths of the product crates.
why: They bypass the file sink and break the Hook contract, the run terminal rule and MCP framing.
guard: clippy print_stdout, print_stderr and dbg_macro at deny
except: viola-harness, the fake agent and the output modules that own --json, human CLI stderr and the hook decision body.
@row_add n=49 row=decision/no-log-bridge area=11-obs-anti-patterns-never-do-these rec=33
name: No log bridge
rule: tracing_log::LogTracer or any log bridge is never installed by hand, and tracing-log stays off.
why: Bridged records lack event, corr, process and instance.
@row_add n=50 row=decision/no-error-reporter-in-v1 area=11-obs-anti-patterns-never-do-these rec=34
name: No error reporter in v1
rule: sentry or any other reporter is never added in v1.
why: Egress is banned, and sentry's default native-tls / tokio transport breaks cargo-deny.
@row_add n=51 row=decision/no-serde-source-in-error-output area=11-obs-anti-patterns-never-do-these rec=35
name: No serde source in printed errors
rule: An anyhow chain that still contains a serde_json or serde_path_to_error source is never printed to stderr, --json, MCP isError or channel error.data.
@row_add n=52 row=decision/default-panic-hook-never-runs area=11-obs-anti-patterns-never-do-these rec=36
name: Default panic hook never runs
rule: The default panic hook is never called or allowed to run.
why: Its stderr output breaks hook and the run terminal.
@row_add n=53 row=decision/reporter-opt-in-only area=11-obs-anti-patterns-never-do-these rec=37
name: Any future reporter is opt-in
rule: Any future reporter is opt-in, never enabled by env var or by default.
@row_add n=54 row=decision/reporter-ureq-rustls area=11-obs-anti-patterns-never-do-these rec=37
name: Future reporter transport is ureq and rustls
rule: Any future reporter uses ureq with rustls.
@row_add n=55 row=decision/reporter-never-log-floor area=11-obs-anti-patterns-never-do-these rec=37
name: Future reporter applies the NEVER-log floor
rule: Any future reporter applies the NEVER-log floor in before_send.
@row_add n=56 row=decision/file-sink-only-error-sink area=11-obs-anti-patterns-never-do-these rec=38
name: File sink is the only error sink
rule: Local logs are never optional: the file sink is the only error sink and works with every other feature off.
@row_add n=57 row=decision/field-allow-list-default-deny area=11-obs-anti-patterns-never-do-these rec=39
name: Field allow-list is default-deny
rule: No field is logged unless named in the field catalog.
@row_add n=58 row=decision/veil-toggle-banned area=11-obs-anti-patterns-never-do-these rec=40
name: veil toggle feature banned
rule: veil's toggle feature (VEIL_DISABLE_REDACTION / veil::disable()) is never enabled.
guard: cargo-deny
@row_add n=59 row=decision/allow-list-not-widenable area=11-obs-anti-patterns-never-do-these rec=41
name: Allow-list and redaction not configurable
rule: config.json, a VIOLA_* env var or a CLI flag never widens the field allow-list or disables redaction.
@row_add n=60 row=decision/verbosity-changes-volume-only area=11-obs-anti-patterns-never-do-these rec=41
name: Verbosity changes volume only
rule: diagnostics_level changes the volume of logging only.
@row_add n=61 row=decision/claude-env-and-cookie-never-logged area=11-obs-anti-patterns-never-do-these rec=42
name: CLAUDE* values and Cookie never logged
rule: CLAUDE* values and a Cookie header are never logged, at any level, in any file, detail files included.
@row_add n=62 row=decision/sensitive-content-detail-file-only area=11-obs-anti-patterns-never-do-these rec=42
name: Sensitive content only in the instance detail file
rule: statusline_command, send text, tool input and last_assistant_message never reach a home-level file; they may reach only the instance-scoped detail-<process>.ndjson, inside a chain or drift report.
@row_add n=63 row=decision/no-paths-or-pids-in-detail-codes area=11-obs-anti-patterns-never-do-these rec=43
name: No paths or pids in detail codes
rule: detail codes never contain paths or pids.
@row_add n=64 row=decision/paths-only-in-instance-detail-files area=11-obs-anti-patterns-never-do-these rec=43
name: Paths only in instance detail files
rule: Paths appear only in instance detail files.
@row_add n=65 row=decision/upload-only-after-secret-scan area=11-obs-anti-patterns-never-do-these rec=44,46
name: Diagnostics uploaded only after the secret scan
rule: diagnostics artifacts are never uploaded before the secret-scan step passes; uploads run if: always() / failure() combined with the secret-scan step's success, never bare.
@row_add n=66 row=decision/pinned-actions area=11-obs-anti-patterns-never-do-these rec=45
name: Actions pinned by commit SHA
rule: GitHub Actions are never used unpinned; actions/upload-artifact is pinned by commit SHA.
guard: zizmor
@row_add n=67 row=decision/failing-job-diagnostics-kept area=11-obs-anti-patterns-never-do-these rec=46
name: Failing job's diagnostics kept
rule: A failing job's diagnostics are never lost: G2, G4 and the secret scan run if: always().
@row_add n=68 row=decision/failed-scan-uploads-hit-report area=11-obs-anti-patterns-never-do-these rec=46
name: Failed scan uploads the hit report
rule: On a failed secret scan the hit report is uploaded instead of the diagnostics.
@row_add n=69 row=decision/rerun-no-substitute area=11-obs-anti-patterns-never-do-these rec=46
name: Re-run is no substitute for diagnostics
rule: A re-run is not a substitute for the failing job's diagnostics.
why: CI runs under retries = 0.
@row_add n=70 row=decision/machine-asserted-telemetry-verdicts area=11-obs-anti-patterns-never-do-these rec=47,55
name: Machine-asserted telemetry verdicts
rule: Log analysis is never human-review-gated; every CI telemetry verdict is a jq or hyperfine assertion that sets an exit code.
@row_add n=71 row=decision/no-ci-env-metadata-in-logs area=11-obs-anti-patterns-never-do-these rec=48
name: No CI env metadata in product logs
rule: CI env metadata is never injected into product log lines.
why: The product never reads env as configuration.
@row_add n=72 row=decision/zero-unlogged-panics area=11-obs-anti-patterns-never-do-these rec=49
name: Zero unlogged panics
rule: The zero-unlogged-panics invariant is never skipped.
@row_add n=73 row=decision/no-retry-once area=11-obs-anti-patterns-never-do-these rec=50
name: No retry-once policies
rule: No retry-once policy is added for panics or flaky telemetry assertions.
@row_add n=74 row=decision/budgets-asserted-by-exit-code area=11-obs-anti-patterns-never-do-these rec=51
name: Budgets asserted by exit code
rule: Every budget has an exit-code-setting assertion (hyperfine max via jq).
@row_add n=75 row=decision/no-per-heartbeat-logging area=11-obs-anti-patterns-never-do-these rec=52
name: No log line per heartbeat tick
rule: Heartbeats log transitions and lifecycle only, never a line per tick.
why: The log files are never rotated.
@row_add n=76 row=decision/no-proprietary-apm-only area=11-obs-anti-patterns-never-do-these rec=53
name: No proprietary APM as the only exporter
rule: Proprietary APM (Datadog APM SDK, NewRelic agent, AppDynamics) is never the only exporter.
why: Vendor schemas break paste-to-AI workflows, and viola has no egress at all.
@row_add n=77 row=absence/network-egress area=11-obs-anti-patterns-never-do-these rec=53
capability: Network egress of any kind
@row_add n=78 row=decision/no-dashboards-only-obs area=11-obs-anti-patterns-never-do-these rec=54
name: No dashboards-only observability
rule: Every signal is consumed through agent-run logs / status and jq, never through dashboards as the sole surface.
