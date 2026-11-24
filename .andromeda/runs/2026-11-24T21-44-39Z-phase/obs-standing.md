# obs · standing conventions

- S1 · each new log line is a single-line obs_event record with a closed event value, codes only, and is schema-checked in integration and E2E tests · log-coverage
  contract/detail-line/backtrace · the detail-file line · backtrace — One of exactly four content fields a detail-file line may carry: the panic's backtrace.
  contract/detail-line/chain · the detail-file line · chain — One of exactly four content fields a detail-file line may carry: the full error chain.
  contract/detail-line/drift-report · the detail-file line · drift_report — One of exactly four content fields a detail-file line may carry: the drift report.
  contract/detail-line/event · the detail-file line · event — Required on every detail-file line, as on home-level lines.
  contract/detail-line/instance · the detail-file line · instance — Required on every detail-file line, as on home-level lines.
  contract/detail-line/level · the detail-file line · level — Required on every detail-file line, as on home-level lines.
  contract/detail-line/message · the detail-file line · message — Required on every detail-file line, as on home-level lines.
  contract/detail-line/panic-payload · the detail-file line · panic_payload — One of exactly four content fields a detail-file line may carry: the panic's payload.
  contract/detail-line/process · the detail-file line · process — Required on every detail-file line, as on home-level lines.
  contract/detail-line/target · the detail-file line · target — Required on every detail-file line, as on home-level lines.
  contract/detail-line/timestamp · the detail-file line · timestamp — Required on every detail-file line, as on home-level lines.
  contract/diagnostics-line/format · the diagnostics line · format — Each diagnostics line is one structured JSON object on one line, written whole by one write of the complete line on an append-mode handle.
  contract/diagnostics-line/role-file-line-size · the diagnostics line · role-file line size — A line in a role file (run, hook, mcp, cli) is codes-only and stays well under 4 KiB.
  contract/log-levels/trace · log levels · trace — TRACE is never emitted; the level exists only in the tests' level enum.
  contract/log-line/level · every log line · level — One of DEBUG, INFO, WARN, ERROR; TRACE is never emitted.
  contract/log-line/message · every log line · message — Flattened, a static literal equal to the event name.
  contract/log-line/target · every log line · target — The Rust module path, for example viola_channel::server.
  contract/log-line/timestamp · every log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  contract/process-log-line/target · the process log line · target — The target is always present.
  contract/process-log-line/timestamp · the process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  contract/process-start-line/service-name · process-start line · service_name — A constant service name, emitted once on process-start rather than on every line.
  decision/diag-lines-schema-checked-in-unit-tests · Emitted lines schema-checked in integration and E2E tests — Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
  decision/json-to-file-only-log-mode · JSON-to-file as the only log mode — JSON to file is the only log mode: there is no agent-mode flag and no stdout pretty sink on any role.
  decision/millisecond-utc-timestamps · Millisecond UTC timestamps — Every diagnostics timestamp is RFC 3339 in UTC with millisecond precision and a Z suffix. · why: The default timer writes microseconds, and the chrono RFC 3339 timer writes +00:00 with automatic precision.
  decision/no-dialog-content-in-logs · Dialog content never logged — The question, the answer and the tool input never appear in a log line.
  decision/no-event-type-field · No event_type field — Log lines carry no event_type field. · why: event already does that job, and a duplicate would amount to a rename.
  decision/no-last-assistant-message · last_assistant_message never logged — last_assistant_message never appears in a log line.
  decision/no-multiline-stack-traces · No multi-line stack traces — Every log line is one physical line parseable by jq; a hook panic is caught and logged as one ERROR line with event panic.
  decision/no-text-in-telemetry · Sent text never in telemetry — The sent text never appears in a span or log line; only its length does.
  decision/single-line-log-records · Single-line log records — Every log record is one JSON line parseable by jq; never a multi-line stack trace.
- S2 · each new detail code is a closed word, distinct per exit cause, carrying no path or pid · 1-obs-scope-summary · obs-scope-summary
  contract/exit-21-detail/endpoint-lost · Exit 21 detail codes · endpoint-lost — endpoint-lost is a distinct detail code for an exit-21 cause, naming no path or pid.
  contract/exit-21-detail/instance-dead · Exit 21 detail codes · instance-dead — instance-dead is a distinct detail code for an exit-21 cause, naming no path or pid.
  contract/exit-21-detail/server-verify-failed · Exit 21 detail codes · server-verify-failed — server-verify-failed is a distinct detail code for an exit-21 cause, distinct from instance-dead, naming no path or pid.
  contract/exit-21-detail/socket-dir-refused · Exit 21 detail codes · socket-dir-refused — socket-dir-refused is a distinct detail code for an exit-21 cause, naming no path or pid.
  contract/exit-21-detail/strict-modes-failed · Exit 21 detail codes · strict-modes-failed — strict-modes-failed is a distinct detail code for an exit-21 cause, naming no path or pid.
  contract/exit-21-detail/unwrapped-name · Exit 21 detail codes · unwrapped-name — unwrapped-name is a distinct detail code for an exit-21 cause, naming no path or pid.
  contract/exit-cause-detail/socket-dir-refused · Exit cause detail codes · socket-dir-refused — A refused socket directory has its own detail code socket-dir-refused and writes its line (MCP writes none); on run and on a start made by viola revive that line is one process-exit{subject:"self", exit_code:1} before any snapshot, child or bind.
- S3 · each new stderr refusal or fault line is a fixed, pathless unable/hint pair, with serde and error chains reduced to a fixed message · error-capture-reporting
  contract/cli-exit-codes/21 · CLI exit codes · 21 — An instance strict-modes refusal exits 21 with the fixed instance-state pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
  contract/cli-stderr/internal-error · CLI stderr · internal fault message — An internal fault prints exactly "error: internal error".
  contract/list-stderr/unable-hint-pair · viola list stderr · unable / hint pair — The unable / hint: pair carries no SGR colour codes.
  contract/process-exit-detail/boot-time-unread · process-exit detail · boot-time-unread — Written by viola up on its process-exit with exit 1 when the host boot time cannot be read; the refusal has its own unable and hint pair.
  contract/viola-link-unlink/strict-modes-failed · viola link and unlink output · strict-modes-failed — A strict-mode fail of the driven instance's check writes no log line; it prints the fixed instance-state unable and hint: pair on stderr, or under --json {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, and exits 21.
  contract/viola-revive/strict-modes-failed · viola revive output · strict-modes-failed — A strict-mode fail of viola revive writes no log line; it prints the stderr pair, or under --json {"v":1,"error":"strict-modes-failed","detail":null}, and exits 1.
  contract/viola-verify/home-tree-refused · viola verify · refused home-tree check — A refused home-tree check exits 1 with the home pair, opens no log and writes no line; a refused update_stamps exits 1 with the home pair and, when an instance is named, ends the opened log with a process-exit line carrying detail strict-modes-failed.
  decision/error-chain-fixed-stderr-message · Error chains reach stderr as fixed messages — In the viola root binary, the only crate that uses anyhow, an error chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to the diagnostics logs only.
  decision/error-chain-in-detail-log-only · Full error chain only in the cli detail log — The full anyhow error chain of a cli verb goes to the cli detail log only.
  decision/serde-chain-fixed-message · Serde error chains become a fixed message on stderr — An error chain that contains a serde source becomes a fixed message before it reaches stderr.
  decision/serde-errors-become-fixed-message · Serde errors become a fixed message — An error chain holding a serde source is replaced by a fixed message before it reaches stderr.
  decision/serde-errors-become-internal-error · Serde error chains replaced by internal error — At dispatch, an error chain whose source is a serde_json or serde_path_to_error error is replaced by the fixed message "internal error" before it reaches stderr, --json, MCP isError or channel error.data. · why: A scrubbing layer at the source.
  decision/serde-errors-fixed-message · Serde error chains become a fixed message — A serde-sourced anyhow error chain is replaced by a fixed message before it reaches stderr.
- S4 · each new test that spawns a child uses the fake agent, never the real claude · 4-span-trace-coverage
  decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.
- S5 · each new test home lives under the one scanned integration-test root and is kept until the gates run · 9-ci-integration · ci-integration
  decision/homes-kept-until-gates-run · Test homes kept until the gates run — Integration-test homes are kept until the gate steps have run.
  decision/test-homes-kept-until-gates · Test homes kept until gate steps run — CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it. · why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
  decision/test-homes-under-e2e-home · Integration-test homes under one root — Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: the one chaos_feed_panic test home
  decision/test-homes-under-scan-root · Test homes live under the scanned root — Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
- S6 · each new fixture is synthetic or scrubbed, and no fixture is a telemetry source · pii-scrubbing-compliance
  decision/claude-fixtures-scrub-required · Claude fixtures are scrub-required — Build and CI infrastructure is not instrumented, and the claude fixtures are scrub-required data, never a telemetry source. · why: Not product runtime.
  decision/fixtures-not-a-telemetry-source · Fixtures not a telemetry source — Recorded fixtures are not a telemetry source, and observability adds no fixture content.
  decision/fixtures-not-telemetry · Recorded fixtures are no telemetry source — Recorded fixtures are scrub-required data, not a telemetry source, and are excluded from the observability plan. · why: Not product runtime.
