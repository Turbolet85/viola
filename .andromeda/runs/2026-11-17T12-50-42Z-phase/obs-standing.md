# obs · standing conventions

- S1 · every new integration test keeps its test home under the one scanned e2e home root, kept until the gate steps run · ci-integration · 9-ci-integration
- S2 · every new test that waits on a process, channel or log does so with a bound, never a sleep-based wait and never a retry · slo-invariants-telemetry-budgets · 10-slo-invariants-telemetry-budgets
  decision/no-retry-once · No retry-once policies — No retry-once policy is added for panics or flaky telemetry assertions.
- S3 · every diagnostics line a new test's processes emit stays schema-checked and panic-free under the CI gates · ci-integration · 9-ci-integration
  contract/child-spawn-log/subject · child and shell spawn log events · subject — process-start and process-exit lines for child spawns carry a subject from a closed set.
  contract/detail-line/backtrace · the detail-file line · backtrace — One of exactly four content fields a detail-file line may carry: the panic's backtrace.
  contract/detail-line/chain · the detail-file line · chain — One of exactly four content fields a detail-file line may carry: the full error chain.
  contract/detail-line/drift-report · the detail-file line · drift_report — One of exactly four content fields a detail-file line may carry: the drift report.
  contract/detail-line/panic-payload · the detail-file line · panic_payload — One of exactly four content fields a detail-file line may carry: the panic's payload.
  contract/diag-line-fields/channel-response · diagnostic line additive fields · channel-response — method (only a closed-list name), conn / srv_conn, result_class (ok, refusal, error), detail (only the three -32602 faults), error_code (-32700, -32600, -32601, -32602, -32603), outcome, duration_ms.
  contract/diag-line-fields/process-exit · diagnostic line additive fields · process-exit — subject, exit_code (self), child_exit_status, exit_source (handle-wait or kill-fallback), detail, during (connect or call), duration_ms.
  contract/diag-line-fields/process-start · diagnostic line additive fields · process-start — subject (self, claude-child, version-probe, verify-probe, verify-pty-probe, agents-probe, hyprctl-probe), service_name, version, os, pid, child_pid, endpoint_kind, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept (names only).
  contract/diag-line-schema/os · diag-line v1 schema · os — os is closed to linux.
  contract/diag-line-schema/pty-backend · diag-line v1 schema · pty_backend — pty_backend is any string.
  contract/diag-subject/claude-child · diag-line subject enum · claude-child — One value of the closed subject enum of the diag-line schema.
  contract/diag-subject/self · diag-line subject enum · self — One value of the closed subject enum of the diag-line schema.
  contract/diagnostics-line/role-file-line-size · the diagnostics line · role-file line size — A line in a role file (run, hook, mcp, cli) is codes-only and stays well under 4 KiB.
  contract/log-levels/debug · log levels · debug — DEBUG is reserved for per-frame channel framing detail and uses the same field allow-list.
  contract/log-levels/info · log levels · info — INFO carries all other boundary, lifecycle and state-transition events, including refusals, which are normal product outcomes.
  contract/log-levels/trace · log levels · trace — TRACE is never emitted; the level exists only in the tests' level enum.
  contract/log-levels/warn · log levels · warn — WARN carries state-recovered; parse-rejected; hook-decision with a fail-open detail; process-exit with exit 21; channel-response with -32600 or -32602.
  contract/log-line/corr · every log line · corr — Set per the tests' table and copied unchanged as a JSON number or string; null is represented by the key's absence.
  contract/log-line/event · every log line · event — A value of the closed kebab-case event enum.
  contract/log-line/instance · every log line · instance — The ViolaName; the key is absent when there is none.
  contract/log-line/level · every log line · level — One of DEBUG, INFO, WARN, ERROR; TRACE is never emitted.
  contract/log-line/message · every log line · message — Flattened, a static literal equal to the event name.
  contract/log-line/target · every log line · target — The Rust module path, for example viola_channel::server.
  contract/log-line/timestamp · every log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  contract/logs-wrapper/record-envelope · logs wrapper · record envelope — Each process-log line is wrapped as {"src":"diag","file":<basename>,"record":<line>}.
  contract/parse-rejected-detail/malformed · parse-rejected detail · malformed — Any parser.
  contract/process-log-corr/channel-request · Process log corr by event · channel-request — corr is the JSON-RPC request id.
  contract/process-log-corr/channel-response · Process log corr by event · channel-response — corr is the JSON-RPC request id.
  contract/process-log-corr/dialog-answered · Process log corr by event · dialog-answered — corr is the dialog_id.
  contract/process-log-corr/dialog-raised · Process log corr by event · dialog-raised — corr is the dialog_id.
  contract/process-log-corr/hook-decision · Process log corr by event · hook-decision — corr is the dialog_id for dialog hooks, otherwise null.
  contract/process-log-corr/hook-invoked · Process log corr by event · hook-invoked — corr is the dialog_id for dialog hooks, otherwise null.
  contract/process-log-corr/panic · Process log corr by event · panic — corr is null.
  contract/process-log-corr/process-exit · Process log corr by event · process-exit — corr is null.
  contract/process-log-corr/process-start · Process log corr by event · process-start — corr is null.
  contract/process-log-corr/send-confirmed · Process log corr by event · send-confirmed — corr is the send cursor.
  contract/process-log-corr/send-issued · Process log corr by event · send-issued — corr is the send cursor.
  contract/process-log-corr/send-refused · Process log corr by event · send-refused — corr is the send cursor.
  contract/process-log-line/at-instants · the process log line · *_at — Instants are named *_at and written in RFC 3339 UTC with milliseconds and a Z suffix.
  contract/process-log-line/duration-ms · the process log line · duration_ms — Durations are duration_ms as an integer.
  contract/process-log-line/enum-values · the process log line · enum values — Enum values are kebab-case.
  contract/process-log-line/event · the process log line · event — The event field is a closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
  contract/process-log-line/field-names · the process log line · field names — Field names are snake_case.
  contract/process-log-line/timestamp · the process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
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
  contract/process-start-line/service-name · process-start line · service_name — A constant service name, emitted once on process-start rather than on every line.
  contract/run-log/cli-verified · run log line · cli_verified — A required field on run's log lines.
  contract/run-log/cli-version · run log line · cli_version — An optional field on run's claude-child process-start line; absent when the CLI version is unreadable.
  contract/run-log/exit-code · run log line · exit_code — A required field on run's log lines.
  decision/additive-field-naming · Additive log field naming — Additive log fields are snake_case with kebab-case values, and carry codes only.
  decision/closed-event-and-corr-on-every-line · Closed event enum on every line, corr where correlated — Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
  decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits.
  decision/diag-schema-matches-event-enum · Diagnostic line schema matches the event enum — A hand-maintained JSON Schema of the process-log line is committed, and its event enum must equal the ObsEvent variants, asserted by a tests-owned check.
  decision/file-only-per-role-log-sinks · File-only log sinks per role — Diagnostics are written to files only, one sink per role, under the home's diagnostics directory.
  decision/json-to-file-only-log-mode · JSON-to-file as the only log mode — JSON to file is the only log mode: there is no agent-mode flag and no stdout pretty sink on any role.
  decision/machine-parseable-signals · Every signal machine-parseable — Every signal produces machine-parseable output: JSON-per-line process logs, the ndjson product event stream, JSON harness documents, hyperfine JSON for perf gates and nextest JUnit XML in CI.
  decision/no-ansi-in-logs · No ANSI codes in diagnostics lines — Diagnostics lines carry no ANSI colour codes.
  decision/no-event-type-field · No event_type field — Log lines carry no event_type field. · why: event already does that job, and a duplicate would amount to a rename.
  decision/no-source-paths-in-logs · No source paths in diagnostics lines — Diagnostics lines carry no source file names or line numbers.
  decision/no-span-list-in-lines · No span list in diagnostics lines — Diagnostics lines never carry the list of entered spans. · why: Closes the spans leak path.
  decision/single-line-log-records · Single-line log records — Every log record is one JSON line parseable by jq; never a multi-line stack trace.
  decision/structured-json-process-logs · Structured JSON process logs — Every process role (run, hook, mcp, cli) writes structured JSON process logs under diagnostics/, which harness logs merges with events.ndjson.
  decision/tests-event-list-adopted · Tests event list adopted — The tests' field and event list is adopted verbatim, with send-issued and send-refused among the logged events.
  decision/unemittable-schema-words-removed · Schema words no build can emit are removed — A closed word of the diag-line schema that no build can emit leaves the schema with its emitter; os is closed at linux.
