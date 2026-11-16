# obs · standing conventions

- S1 · every new log line goes through the single obs_event macro with a closed event value, the required fields and corr where correlated; no raw tracing or print macros · 11-obs-anti-patterns-never-do-these
  contract/detail-file-line/correlation-keys · the detail-file error line · correlation keys — The detail-file line for an error carries the same event, corr and conn as its home-level counterpart, so jq joins them.
  contract/detail-line/event · the detail-file line · event — Required on every detail-file line, as on home-level lines.
  contract/detail-line/instance · the detail-file line · instance — Required on every detail-file line, as on home-level lines.
  contract/detail-line/level · the detail-file line · level — Required on every detail-file line, as on home-level lines.
  contract/detail-line/message · the detail-file line · message — Required on every detail-file line, as on home-level lines.
  contract/detail-line/process · the detail-file line · process — Required on every detail-file line, as on home-level lines.
  contract/detail-line/target · the detail-file line · target — Required on every detail-file line, as on home-level lines.
  contract/detail-line/timestamp · the detail-file line · timestamp — Required on every detail-file line, as on home-level lines.
  contract/log-line/event · every log line · event — A value of the closed kebab-case event enum.
  contract/log-line/instance · every log line · instance — The ViolaName; the key is absent when there is none.
  contract/log-line/message · every log line · message — Flattened, a static literal equal to the event name.
  contract/log-line/target · every log line · target — The Rust module path, for example viola_channel::server.
  contract/log-line/timestamp · every log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  contract/obs-event-macro/attached-fields · the obs_event macro · attached fields — Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.
  contract/process-log-line/message · the process log line · message — The message is a static literal equal to the event name and never carries interpolated content.
  contract/process-log/corr · Process log line · corr — Copied unchanged as a JSON number or string, never renamed.
  contract/process-log/event · Process log line · event — A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
  contract/run-log/exit-code · run log line · exit_code — A required field on run's log lines.
  contract/run-process-log/common-fields · run process log · common fields — Every run line carries the common fields with process="run" and corr null.
  decision/closed-event-and-corr-on-every-line · Closed event enum on every line, corr where correlated — Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
  decision/event-values-closed · event values closed — No event value is added without a Decisions Log entry.
  decision/lint-probes-failure-fails · Lint-probes failure is a failure — A lint-probes failure (a ban that did not fire, a control that did not pass, or a raw event! outside the core obs module) counts as an SLO invariant failure.
  decision/mcp-no-telemetry-on-stdout · No telemetry on the MCP server's stdout — The MCP server sends no telemetry to stdout. · why: stdout is reserved for MCP frames.
  decision/no-event-type-field · No event_type field — Log lines carry no event_type field. · why: event already does that job, and a duplicate would amount to a rename.
  decision/no-print-macros · No print macros in product code paths — print!, println!, eprint!, eprintln! and dbg! are never used in the hook, run or mcp code paths of the product crates, and every product member inherits the workspace lints. · why: They bypass the file sink and break the Hook contract, the run terminal rule and MCP framing. · guard: clippy print_stdout, print_stderr and dbg_macro at deny · except: viola-harness, the fake agent and the output modules that own --json, human CLI stderr and the hook decision body.
  decision/no-raw-tracing-macros · No raw tracing macros outside the obs module — Raw tracing info, warn, error, debug and trace macros and raw event! are refused outside viola_core::obs. · guard: clippy -D warnings disallowed-macros; raw-event! grep and ban probes in scripts/lint-probes.sh
  decision/obs-event-macro-only · obs_event! is the only event macro — tracing's event and level macros are never called directly outside viola_core::obs; events go through obs_event!. · why: Only obs_event! guarantees event, process and instance. · guard: clippy disallowed-macros; the raw-event! grep in lint-probes
  decision/obs-event-single-macro · One obs_event macro for every line — Every log line is emitted through the one obs_event macro defined in viola-core, which expands to tracing's event at the caller, so viola-core stays I/O-free and tracing-free.
  decision/obs-owns-logger-and-format · Obs owns the logger and format — The logger and the log format are owned by observability.
  decision/print-and-macro-lint-failure-fails · Print and macro lint failure is a failure — A clippy disallowed-macros, print_stdout, print_stderr or dbg_macro failure counts as an SLO invariant failure.
  decision/raw-tracing-ban · Raw tracing macros banned — The tracing level macros are banned by clippy disallowed-macros and raw event calls by a fail-closed grep outside the obs module, so only obs_event emits lines. · why: clippy reports a disallowed macro expanded inside an exported macro at the caller, so no allow inside obs_event can exempt it · guard: clippy disallowed-macros and scripts/lint-probes.sh
  decision/source-allow-list-logging · Source allow-list for logged fields — Spans skip all arguments and declare fields explicitly, and events go through obs_event! with explicit typed fields only; this source allow-list is the primary PII control. · guard: disallowed-macros; raw_event_hits; G1 bare instrument
  decision/tracing-json-logging-pipeline · tracing with a JSON subscriber for logging — Logging and spans go through the Rust tracing facade with a tracing-subscriber JSON structured logger.
  decision/tracing-json-pipeline · tracing with the tracing-subscriber JSON formatter — The `tracing` facade plus the tracing-subscriber JSON formatter is the binding logging pipeline, and JSON-to-file is the only logging mode.
- S2 · every new diagnostics line conforms to the diagnostic line schema, additive only, checked by the schema conformance gate on real process output in integration and E2E tests · 9-ci-integration · ci-integration
  contract/detail-line/fields · the detail line · fields — Each detail line carries timestamp (millisecond UTC format), level, target, message, event, process and instance, then its content fields, and no corr or conn key.
  contract/diag-line-schema/os · diag-line v1 schema · os — os is closed to linux.
  contract/diag-line-schema/pty-backend · diag-line v1 schema · pty_backend — pty_backend is any string.
  contract/diag-line/instance · diagnostic log line · instance — The instance's `ViolaName`; the key is absent when the process has no instance.
  contract/diag-line/process · diagnostic log line · process — The role of the writing process, one of run, hook, mcp, cli; required on every line and matching `sender` / `writer`.
  contract/diag-line/service-name · diagnostic log line · service.name — Hardcoded to "viola".
  contract/diag-line/service-version · diagnostic log line · service.version — The crate version, fixed at compile time.
  contract/diagnostics-line/format · the diagnostics line · format — Each diagnostics line is one structured JSON object on one line, written whole by one write of the complete line on an append-mode handle.
  contract/diagnostics-line/instance · the diagnostics line · instance — Every diagnostics line of a process with an instance carries it as field instance; a process without one, such as an mcp server with no instance name, writes no instance key.
  contract/diagnostics-line/process · the diagnostics line · process — Every diagnostics line carries the role as field process.
  contract/log-line/process · every log line · process — Every log line carries a process key naming its role: one of run, hook, mcp, cli.
  contract/logs-wrapper/record-envelope · logs wrapper · record envelope — Each process-log line is wrapped as {"src":"diag","file":<basename>,"record":<line>}.
  contract/process-log-line/at-instants · the process log line · *_at — Instants are named *_at and written in RFC 3339 UTC with milliseconds and a Z suffix.
  contract/process-log-line/field-names · the process log line · field names — Field names are snake_case.
  contract/process-log-line/instance · the process log line · instance — The instance's name, or absent when none resolved.
  contract/process-log-line/level · the process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/process-log-line/process · the process log line · process — One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing cli-<name>.ndjson.
  contract/process-log-line/target · the process log line · target — The target is always present.
  contract/process-log-line/timestamp · the process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  contract/process-log/instance · Process log line · instance — A ViolaName or null.
  contract/process-log/level · Process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/process-log/message · Process log line · message — A required field, as fields.message or the flattened message.
  contract/process-log/process · Process log line · process — The process role, one of run, hook, mcp, cli.
  contract/process-log/target · Process log line · target — A required field.
  contract/process-log/timestamp · Process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  decision/diag-lines-schema-checked-in-unit-tests · Emitted lines schema-checked in integration and E2E tests — Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
  decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits.
  decision/diag-schema-matches-event-enum · Diagnostic line schema matches the event enum — A hand-maintained JSON Schema of the process-log line is committed, and its event enum must equal the ObsEvent variants, asserted by a tests-owned check.
  decision/flat-log-lines-and-one-level-gate · Log lines are flat and Targets is the only level gate — The subscriber flattens event fields to the top level of each line, and the Targets filter is the only configurable level gate; its level comes from the home's diagnostics level and never from the environment. · why: The diagnostics schema and the log readers read event fields as top-level keys, and only the viola crates' targets pass the gate.
  decision/g4-schema-conformance-gate · Schema conformance gate on every line — Every home-level line is validated against the diagnostics line schema and every detail line against the detail schema, in a step that always runs. · guard: G4
  decision/level-changes-volume-only · Level changes volume only — Every diagnostics level uses the same field allow-list, so the level changes only how many lines are written. · why: No setting can disable redaction.
  decision/log-field-names-frozen · Log field names frozen — Log fields such as corr and event are never renamed without a Decisions Log entry.
  decision/machine-parseable-signals · Every signal machine-parseable — Every signal produces machine-parseable output: JSON-per-line process logs, the ndjson product event stream, JSON harness documents, hyperfine JSON for perf gates and nextest JUnit XML in CI.
  decision/millisecond-utc-timestamps · Millisecond UTC timestamps — Every diagnostics timestamp is RFC 3339 in UTC with millisecond precision and a Z suffix. · why: The default timer writes microseconds, and the chrono RFC 3339 timer writes +00:00 with automatic precision.
  decision/no-ansi-in-logs · No ANSI codes in diagnostics lines — Diagnostics lines carry no ANSI colour codes.
  decision/no-multiline-stack-traces · No multi-line stack traces — Every log line is one physical line parseable by jq; a hook panic is caught and logged as one ERROR line with event panic.
  decision/no-source-paths-in-logs · No source paths in diagnostics lines — Diagnostics lines carry no source file names or line numbers.
  decision/no-span-list-in-lines · No span list in diagnostics lines — Diagnostics lines never carry the list of entered spans. · why: Closes the spans leak path.
  decision/null-as-key-absence · Null written as key absence — A null corr or instance is written as an absent key, readers treat absence and null as equal, and the schema rejects a literal null. · why: tracing has no null field value
  decision/schema-conformance-gate · Schema conformance gate on real process output — Every role-file line of test homes must validate against schemas/diag-line.v1.json and every detail line against schemas/diag-detail.v1.json; a failing line fails the build, a torn non-JSON line is skipped and counted. · why: schema failures count as a build failure and unit tests check only in-memory lines · guard: gate G4 (schema-conformance step)
  decision/shared-log-files-concurrent-append · Concurrent appends to shared diagnostics files — Every diagnostics file may have concurrent writers in separate processes; each line is one whole write on an append-mode handle, readers tolerate a torn line, and a concurrent-append check, including a detail line over 4 KiB, asserts every line parses. · why: Non-interleaving at this size is an assumption, not a POSIX guarantee, because the 4 KiB PIPE_BUF rule applies to pipes. · guard: concurrent-append check; hook_panics_append_whole_lines_over_4_kib_side_by_side
  decision/structured-logs-primary-signal · Structured logs are the primary signal — tracing-subscriber JSON-per-line logs, one file per process role under the home's diagnostics directory, are the primary observability signal, merged with the event stream by agent-run logs.
- S3 · every new integration test home lives under the scanned test-home root and is kept until the gates run · ci-integration
  decision/homes-kept-until-gates-run · Test homes kept until the gates run — Integration-test homes are kept until the gate steps have run.
  decision/test-homes-kept-until-gates · Test homes kept until gate steps run — CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it. · why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
  decision/test-homes-under-e2e-home · Integration-test homes under one root — Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: the one chaos_feed_panic test home
  decision/test-homes-under-scan-root · Test homes live under the scanned root — Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
- S4 · every new detail code is a closed code carrying no path, pid or content · 4-span-trace-coverage
  contract/exit-1-detail/already-live · Exit 1 detail codes · already-live — already-live is a distinct detail code for an exit-1 cause, naming no path or pid.
  contract/exit-1-detail/cwd-missing · Exit 1 detail codes · cwd-missing — cwd-missing is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
  contract/exit-1-detail/no-session · Exit 1 detail codes · no-session — no-session is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
  contract/exit-1-detail/pinned-hash-mismatch · Exit 1 detail codes · pinned-hash-mismatch — pinned-hash-mismatch is a distinct detail code for an exit-1 cause, naming no path or pid.
  contract/exit-1-detail/session-live · Exit 1 detail codes · session-live — session-live is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
  contract/exit-1-detail/socket-dir-refused · Exit 1 detail codes · socket-dir-refused — socket-dir-refused is a distinct detail code for an exit-1 cause, naming no path or pid.
  contract/exit-1-detail/squatted-name · Exit 1 detail codes · squatted-name — squatted-name is a distinct detail code for an exit-1 cause, naming no path or pid.
  contract/exit-1-detail/stale-heartbeat · Exit 1 detail codes · stale-heartbeat — stale-heartbeat is a distinct detail code for an exit-1 cause, naming no path or pid.
  contract/exit-21-detail/endpoint-lost · Exit 21 detail codes · endpoint-lost — endpoint-lost is a distinct detail code for an exit-21 cause, naming no path or pid.
  contract/exit-21-detail/instance-dead · Exit 21 detail codes · instance-dead — instance-dead is a distinct detail code for an exit-21 cause, naming no path or pid.
  contract/exit-21-detail/server-verify-failed · Exit 21 detail codes · server-verify-failed — server-verify-failed is a distinct detail code for an exit-21 cause, distinct from instance-dead, naming no path or pid.
  contract/exit-21-detail/socket-dir-refused · Exit 21 detail codes · socket-dir-refused — socket-dir-refused is a distinct detail code for an exit-21 cause, naming no path or pid.
  contract/exit-21-detail/strict-modes-failed · Exit 21 detail codes · strict-modes-failed — strict-modes-failed is a distinct detail code for an exit-21 cause, naming no path or pid.
  contract/exit-21-detail/unwrapped-name · Exit 21 detail codes · unwrapped-name — unwrapped-name is a distinct detail code for an exit-21 cause, naming no path or pid.
  contract/mcp-invalid-params/detail · MCP invalid-params · detail — The detail of an MCP invalid-params is a closed field word: target, text, after, timeout_ms, dialog_id or response.
  contract/mcp-iserror/instance-unreachable · MCP isError result · instance-unreachable — The instance-unreachable detail is one closed cause: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed or socket-dir-refused; the CLI's document less v, its text `refused: instance-unreachable`.
  contract/mcp-iserror/invalid-params · MCP isError result · invalid-params — The invalid-params detail is a closed field word.
  contract/mcp-iserror/wrapper-fault · MCP isError result · wrapper-fault — The wrapper-fault detail is a closed sanitised object: the fixed message of the five codes with data null or closed values; a code outside the five reads message "wrapper fault", data null.
  contract/unreachable-cause/endpoint-lost · exit 21 cause words · endpoint-lost — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
  contract/unreachable-cause/instance-dead · exit 21 cause words · instance-dead — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
  contract/unreachable-cause/server-verify-failed · exit 21 cause words · server-verify-failed — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
  contract/unreachable-cause/socket-dir-refused · exit 21 cause words · socket-dir-refused — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
  contract/unreachable-cause/strict-modes-failed · exit 21 cause words · strict-modes-failed — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
  contract/unreachable-cause/unwrapped-name · exit 21 cause words · unwrapped-name — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
  decision/detail-codes-no-paths · Detail codes carry no path or pid — No path or pid appears in a detail code.
  decision/distinct-detail-code-per-exit-cause · Distinct detail code per exit cause — Each cause behind exit 21 and exit 1 gets a distinct detail code with no path or pid. · except: A strict-modes check refused before any log opens writes no line; its cause is the fixed stderr pair or the --json document.
  decision/exit-1-detail-codes · Distinct detail codes for exit-1 causes — On the `viola run` start sequence to child spawn, each exit-1 cause logs a distinct `detail` code with no path or pid in it.
  decision/no-paths-or-pids-in-detail-codes · No paths or pids in detail codes — detail codes never contain paths or pids.
- S5 · every new span is declared with skip_all and an explicit field allow-list, a static name, and no err or ret option · span-trace-coverage
  decision/g1-bare-instrument-gate · G1 bare-instrument gate fails closed — Every #[instrument] must have skip_all as its first argument; the G1 gate passes only on rg exit 1 (no match), so a violation, a regex or I/O error or a missing rg fails the CI step. · guard: G1
  decision/instrument-requires-skip-all · Instrument attribute must skip all arguments — Every #[instrument] in the workspace takes skip_all as its first argument; a bare #[instrument] fails the build. · guard: gate G1
  decision/instrument-skip-all-everywhere · Spans skip all arguments — Every instrumented function uses skip_all with explicit fields, so no argument is recorded implicitly. · guard: G1
  decision/low-cardinality-span-names · Low-cardinality span names — Span names never contain instance names, dialog ids or paths.
  decision/no-bare-instrument · No bare #[instrument] — Instrumentation always uses #[instrument(skip_all, fields(...))], never a bare #[instrument]. · why: A bare #[instrument] records every argument with Debug and leaks send text, hook.dialog payloads and tool input.
  decision/no-err-ret-span-options · err and ret span options banned — The err and ret instrumentation options are banned on spans.
  decision/no-instrument-err-ret-on-sensitive · No instrument err or ret on sensitive results — #[instrument(err)] and #[instrument(ret)] are never used where the error or return can carry a serde or serde_path_to_error source or user content.
  decision/span-field-allow-list · Spans declared with explicit field allow-list — Every span is declared skipping all arguments with an explicit field allow-list.
  decision/span-kinds-not-emitted-v1 · Span kinds recorded but not emitted — Span kinds are recorded in the plan per span but not emitted in v1; if the deferred tracing-opentelemetry bridge is adopted, they map to the otel.kind span field.
  decision/span-naming · Span naming as <area>.<operation> — Span names are <area>.<operation> in snake_case, <area> one of run, pty, channel, state, hook, send, wait, last, answer, pause, release, cli, mcp, hyprland; cli.<verb> names the dispatch span of a short-lived verb.
  decision/span-naming-skip-all · Span declaration convention — Every span is declared with instrument, skip_all and a name of the form "<area>.<operation>".
  decision/static-span-names · Static span names — Span names are static strings; instance names, dialog ids and paths never appear in a span name.
- S6 · every new test leaves zero panic lines and does not wait by sleeping on a log · 10-slo-invariants-telemetry-budgets
  decision/build-fails-on-any-panic · Build fails on any panic line — The build fails on any panic event line in the kept homes' role files, bar the fake-agent seam's forced panic; no gate infers a panic from an exit code.
  decision/g2-zero-panics-gate · Zero-panics gate fails closed — The zero-panics gate probes itself first, fails closed, runs in the test and perf jobs, and counts panics over home-level role files only. · guard: G2 · except: the fake-agent seam's forced panic at exactly its seam location, compared as a whole string
  decision/panic-lines-stay-zero · Panic lines stay at zero — Gate G2 counts the panic lines of the home-level role files and fails on any; no viola.panic metric is derived.
  decision/zero-panics-gate · Zero panics gate — No panic line may appear in the home-level role files of test runs; detail files repeat the panic line and are excluded. · why: ratified · guard: gate G2 (scripts/g2-zero-panics.sh, --probe first) · except: a panic whose panic_location is exactly src/cmd/hook/seam.rs:<digits>, the fake-agent seam's forced panic, compared as a whole string
