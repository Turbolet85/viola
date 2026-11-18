# obs-plan · the rows that bind 2026-11-18-session-folder-trusted-by-viola — picked 177 of 1025 (82 by every draw) · rule any · pin 2658

## obs-plan-viola · (above the first heading)
decision/agent-readable-telemetry · Agent-readable telemetry only — Every telemetry signal must be readable by an agent from a snapshot or pasted log excerpt: no human-only dashboards, no proprietary APM lock-in, no human-review-gated log analysis. · except: A surface that cannot be agent-readable is flagged not-instrumentable and handled through boundary instrumentation only.

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/claude-internals-not-a-contract · Wrapped claude internals are not a contract — The wrapped claude CLI is a third-party binary run unmodified and instrumented at its boundary only (spawn and exit, paste writes, the vt100 readiness-gate outcome); its internal state and transcripts are never read as a contract. · except: A send's echo confirmation counts its local command's stdout tag in the session transcript, keeping no byte; viola verify writes and removes its own temp folder's trust record in the claude config.
decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits.
decision/error-chain-fixed-stderr-message · Error chains reach stderr as fixed messages — In the viola root binary, the only crate that uses anyhow, an error chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to the diagnostics logs only.
decision/os-apis-at-call-sites · OS APIs observed at call sites — OS APIs (openpty, Unix socket, filesystem locks and atomic rename, sysinfo) and the hyprctl spawn are instrumented at viola's call sites only, never in their internals.
decision/refusal-telemetry · Refusal telemetry for filesystem refusals — Pinned-exe hash mismatches and ignored symlinks produce refusal telemetry. · except: The send's transcript echo read and verify's trust-record write refuse a symlinked, non-regular or foreign file with no line of their own; the outcome shows only on the send's lines or the stamps.
decision/run-silent-terminal · viola run silent on the terminal — viola run writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns.
decision/run-sync-instrumentation · Sync instrumentation in viola run — Instrumentation in viola run is sync, with no Tokio.
decision/serde-errors-fixed-message · Serde error chains become a fixed message — A serde-sourced anyhow error chain is replaced by a fixed message before it reaches stderr.
decision/single-file-sink-no-os-exporter · One file sink and one line schema — Telemetry on Linux goes to one file sink with one line schema, with no OS-specific exporter.
decision/stderr-fixed-strings · Fixed strings on stderr — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/stripped-claude-env-never-logged · R8-stripped CLAUDE values never logged — R8-stripped CLAUDE* environment values are never logged anywhere. · why: The viola-agent-claude outputs are the most sensitive in the product.
decision/stripped-env-never-logged · Stripped CLAUDE env values never logged — No R8-stripped `CLAUDE*` value appears in any sink or fixture; the strip is logged as a count or variable names only.
contract/diagnostics-files/run · the diagnostics files · run — Role run writes <home>/diagnostics/run-<name>.ndjson.
contract/mcp-stdin/max-frame · viola mcp stdin · MAX_FRAME — Stdin is read in lines of at most `MAX_FRAME`; a longer line is refused whole and the session ends as at a closed stdin.
contract/run-log-file/path · the viola run log file · path — viola run logs only to a file sink at diagnostics/run-<name>.ndjson.
decision/agents-probe-parse-failure-unknown · Agents probe parse failure becomes unknown — A parse failure of the claude agents --json probe becomes status "unknown" plus a parse-rejected counter; an unparsed --version answer leaves cli_version absent and the version unverified.
decision/exit-1-detail-codes · Distinct detail codes for exit-1 causes — On the `viola run` start sequence to child spawn, each exit-1 cause logs a distinct `detail` code with no path or pid in it.
decision/obs-deps-tokio-free · Obs dependencies Tokio-free and C-free — Obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass cargo deny. · guard: cargo deny
decision/one-log-file-per-role · One log file per process role — Each process role writes its own file under `diagnostics/`, no role has a stdout sink, and no role falls back to stderr JSON when its file cannot be opened.
decision/pty-byte-stream-never-logged · PTY byte stream never logged — The PTY byte stream is never logged; the viola-pty seam is instrumentable, and the openpty OS API behind it is boundary-only. · why: The PTY byte stream is user content.
decision/strict-modes-failure-lines · Where a strict-modes failure is logged — A strict-modes failure has a log line only on the `ledger-stamps` read in the `run` version gate and on `verify`'s own `process-exit` for a refused stamp write.
contract/diag-line/process · diagnostic log line · process — The role of the writing process, one of run, hook, mcp, cli; required on every line and matching `sender` / `writer`.
contract/log-line/process · every log line · process — Every log line carries a process key naming its role: one of run, hook, mcp, cli.

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/detail-codes-no-paths · Detail codes carry no path or pid — No path or pid appears in a detail code.
decision/fault-telemetry-per-class · Fault telemetry per class — Each fault class emits its own telemetry: a healed torn ndjson line with file and offset, a corrupt or unsupported snapshot recovered by replay, a child exit seen without EOF, an endpoint vanished during wait, and a stale heartbeat against a live pid.
decision/fixed-stderr-messages · Fixed stderr refusal and fault lines — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but a required one the product still emits is never renamed or removed. · why: The harness greps on them.
decision/r8-strip-logged-as-names-only · R8 strip logged as names only — The R8 environment strip is logged as a count and lists of stripped and kept variable names, and environment maps never pass to a span or event; no value is read, bar the kept CLAUDE_CONFIG_DIR value viola verify reads, and no value is logged.
decision/serde-chain-fixed-message · Serde error chains become a fixed message on stderr — An error chain that contains a serde source becomes a fixed message before it reaches stderr.
decision/serde-errors-become-fixed-message · Serde errors become a fixed message — An error chain holding a serde source is replaced by a fixed message before it reaches stderr.
contract/cli-log-file/revive-start · CLI process-log file · viola revive start arm — Logs as the wrapper, process="run", into diagnostics/run-<name>.ndjson.
contract/run-log/cli-verified · run log line · cli_verified — A required field on run's log lines.
decision/instrument-call-sites-not-os-internals · OS operations instrumented at viola's call sites — OS-level operations (bind, connect, lock, rename, kill fallback, the server check's peer reads, the strict-modes stat reads, the socket directory check, the hyprctl spawn) are instrumented at viola's own call sites, not inside the OS.
decision/messaging-secrets-never-logged · Messaging secrets never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every R8-stripped CLAUDE* value are never logged in any sink, detail files and fixtures included. · guard: secret-scan test
decision/no-secrets-in-log-lines · No secrets in log lines — No log line may contain any stripped CLAUDE* value. · guard: secret-scan test
decision/no-telemetry-from-screen · No telemetry from screen content — No telemetry is derived from screen content; vt100 serves gate outcomes only.
decision/one-linux-log-sink · One file sink on Linux — One file sink and one line schema serve Linux, with no OS-specific exporter.
decision/parse-failures-surface-as-unknown · Parse failures surface as unknown — A parse failure surfaces as the value unknown plus a counter. · except: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown.
decision/run-silent-on-terminal · run silent on the terminal — run writes nothing to the terminal while the child runs; its process log goes only to diagnostics/run-<name>.ndjson. · why: The terminal carries the child's screen.
decision/stripped-claude-values-never-logged · Stripped CLAUDE values never logged — R8-stripped CLAUDE* values are never logged anywhere. · why: Its outputs are the most sensitive in the product.
contract/exit-cause-detail/socket-dir-refused · Exit cause detail codes · socket-dir-refused — A refused socket directory has its own detail code socket-dir-refused and writes its line (MCP writes none); on run and on a start made by viola revive that line is one process-exit{subject:"self", exit_code:1} before any snapshot, child or bind.
contract/mcp-stdin/line-within-max-frame · viola mcp stdin · line within MAX_FRAME — A line of at most MAX_FRAME bytes, its newline counted, passes whole.
contract/process-exit/socket-dir-refused-on-start · Process exit line · socket-dir-refused on a start — A refused socket directory on a start writes one process-exit with detail socket-dir-refused, exit 1.
contract/process-log/corr · Process log line · corr — Copied unchanged as a JSON number or string, never renamed.
contract/process-log/event · Process log line · event — A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
contract/process-log/instance · Process log line · instance — A ViolaName or null.
contract/process-log/level · Process log line · level — One of DEBUG, INFO, WARN, ERROR.
contract/process-log/message · Process log line · message — A required field, as fields.message or the flattened message.
contract/process-log/process · Process log line · process — The process role, one of run, hook, mcp, cli.
contract/process-log/target · Process log line · target — A required field.
contract/process-log/timestamp · Process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
contract/run-log/exit-code · run log line · exit_code — A required field on run's log lines.
contract/run-start-events/order · run start events in events.ndjson · order — A run start writes wheel{cause:"start"} before session-start; on a start whose folder-trust keys pass through the wrapper, wheel{human-input} also precedes session-start, and a later human key adds no wheel line, the wheel being already human.

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
contract/process-log-line/enum-values · the process log line · enum values — Enum values are kebab-case.
contract/process-log-line/field-names · the process log line · field names — Field names are snake_case.
decision/no-human-gated-log-analysis · No human-review-gated log analysis — Every log verdict is a jq or harness assertion that sets an exit code, never a human review.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/codes-only-process-logs · Codes-only process logs, content in detail files — Every role's codes-only process log goes to its home-level role file under diagnostics, and content-bearing detail goes only to the instance's per-role detail file.
contract/run-terminal-output/child-output · run terminal output · child output — run writes nothing to the terminal except the child's own output; while the child runs, neither run nor a revived start writes anything of its own.
contract/run-terminal-output/start-refusal · run terminal output · start refusal — A start refusal before any child is spawned writes exactly two fixed stderr lines with no path or pid: the home pair for a home-level refusal, the instance pair for an instance-level one.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.
decision/detail-codes-only · detail is a closed code — The detail field is a closed kebab-case code catalog and never carries a path or pid.
decision/env-names-only · Environment names only, never values — The strip and every log record environment variable names only, never values, and no env map is passed to a span or event. · guard: E2
decision/ledger-contents-never-logged · Capability ledger path and stamps never logged — The capability ledger's path and its stamp contents never appear in a log line.
decision/no-endpoint-path-or-env-in-spans · No endpoint, binary path or env values in spans — A span never carries the endpoint name, the pinned_bin path or environment values.
decision/trust-record-write-uninstrumented · Trust-record write and removal uninstrumented — viola verify's trust-record write and removal emit no log line and no span; the outcome rides only the stamps, as the trust-record row's word and the trust probe's write and remove words, and no path, key or value of the claude config reaches any line, stamp or summary.
decision/unreadable-ledger-is-unverified · Unreadable ledger counts as unverified — A missing stamp, or an unreadable, malformed or strict-modes-refused capability ledger, counts as unverified (cli_verified false).
decision/span-field-allow-list · Spans declared with explicit field allow-list — Every span is declared skipping all arguments with an explicit field allow-list.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/claude-env-strip · CLAUDE environment strip — The child loses every inherited CLAUDE* name, matched exactly, except the persistent set kept in the claude_env_keep config key.
decision/error-chain-in-detail-log-only · Full error chain only in the cli detail log — The full anyhow error chain of a cli verb goes to the cli detail log only.
decision/no-paths-in-run-spans · No endpoint, path or env values on start spans — Start spans never carry the endpoint name, the pinned binary path or environment values.
decision/real-cli-verify-never-in-ci · Real-CLI verify never runs in CI — In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
decision/revive-start-logs-as-run · viola revive start logs as run — The start arm of viola revive is not a cli verb and logs as process run once its instance check has passed.
decision/unreadable-ledger-means-unverified · Unreadable capability ledger means an unverified CLI — A capability ledger that is missing a stamp, unreadable, malformed or refused by the strict-modes check is treated as an unverified CLI (cli_verified false).
contract/run-process-log/common-fields · run process log · common fields — Every run line carries the common fields with process="run" and corr null.
contract/run-span-attributes/run-version-gate · viola run start span attributes · run.version_gate — run.version_gate carries cli_version and cli_verified.
contract/verify-stdout/step-line · verify stdout · step line — One "[NN/MM] <row id> <row words>  pass|fail" line is printed for each relied ledger row, MM being the relied row count.
decision/home-tree-check-before-start-log · Home-tree check before the start's log — The start of run and of viola revive checks the home tree (the home, instance set, ledger, bin, plugin, pinned exe and plugin hooks) before its log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
decision/identity-floor-always-stripped · Identity floor always stripped — The 11 names of the identity floor are always removed whatever the persistent set says; beyond the floor there is no compile-time list of known names.
decision/product-order-from-events-log · Start product order read from the events log — The start product order, wheel{cause:"start"} before session-start{source:"hook"} with wheel{human-input} also before it where folder-trust keys pass through the wrapper, is verified from that start's own lines of the instance's events log, not duplicated into process logs.
decision/verify-lines-carry-no-content · verify log lines carry no content — No verify log line carries row text, a prompt, a session id, the probe command or a path.
contract/run-log-parse-rejected/ledger-stamps · the run process log parse-rejected line · parser ledger-stamps — An unreadable, malformed or strict-modes-refused capability ledger gives one parse-rejected line with parser "ledger-stamps", detail "unreadable", "malformed" or "strict-modes-failed", and a count, in the run log.
contract/run-process-log/process-start-claude-child · run process log · process-start claude-child — On success, process-start{subject:"claude-child", child_pid, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept}.
contract/run-start-spans/span-chain · viola run start spans · span chain — run.start (INTERNAL, root) › state.snapshot_recover › run.collision_check › run.pin_copy › run.version_gate › channel.bind › state.snapshot_write › state.heartbeat_start › pty.spawn (CLIENT).
contract/verify-exit-codes/0 · verify exit codes · 0 — Every relied ledger row passed.
contract/verify-exit-codes/1 · verify exit codes · 1 — A row failed, verify refused, or verify was stopped.
contract/verify-spawn-subject/verify-pty-probe · verify child spawn subjects · verify-pty-probe — Each of verify's six interactive PTY runs is logged as one process-start / process-exit pair with subject verify-pty-probe.
contract/verify-stdout/summary-line · verify stdout · summary line — The last stdout line is "stamped <version>  N pass  N fail", the two counts summing to MM.

## 5-metric-coverage · ## 5. Metric Coverage · twin metric-coverage
decision/metric-grouping-closed-enums · Metric grouping by closed enums only — Aggregate reports group only by closed enums (event, detail, refusal, method, hook_event, subject, parser, process); corr, conn, pid, child_pid and instance are join keys, never grouping keys.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/additive-field-naming · Additive log field naming — Additive log fields are snake_case with kebab-case values, and carry codes only.
decision/log-schema-additive-only · Log schema additive to the tests' format — The observability log schema only adds fields and enum values to the tests' log format; it renames nothing and removes only a field no build emits. · why: The tests' log format is the binding text.
decision/skipped-symlink-never-logged · Skipped symlink never named in logs — The entry name and link target of a skipped symlink are never logged.
contract/log-levels/info · log levels · info — INFO carries all other boundary, lifecycle and state-transition events, including refusals, which are normal product outcomes.
decision/content-detail-in-instance-file · Content-bearing detail in the instance detail file — Content-bearing detail goes only to the per-process detail file inside the instance's diagnostics directory.
decision/no-event-type-field · No event_type field — Log lines carry no event_type field. · why: event already does that job, and a duplicate would amount to a rename.
decision/unlisted-targets-log-off · Unlisted log targets off — The per-module log filter turns logging OFF for every target it does not list.
decision/viola-crates-log-at-info · Viola crates log at INFO — The per-module log filter sets the viola, viola_core, viola_pty, viola_channel, viola_state, viola_agent_claude and viola_mcp targets to INFO, or DEBUG via diagnostics_level.
contract/log-line/event · every log line · event — A value of the closed kebab-case event enum.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
contract/parse-rejected-detail/strict-modes-failed · parse-rejected detail · strict-modes-failed — ledger-stamps: run's version gate refused the stamps file on the strict-modes check, and the read counts as unverified.
decision/revive-preflight-order · Revive start preflight order — Revive's start preflight reads the holder, then the session, then session-live, then the directory, and stops at the first refusal.
contract/diag-line-fields/process-start · diagnostic line additive fields · process-start — subject (self, claude-child, version-probe, verify-probe, verify-pty-probe, agents-probe, hyprctl-probe), service_name, version, os, pid, child_pid, endpoint_kind, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept (names only).

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/boundary-errors-end-in-logged-line · Crate-seam errors end in a logged line — Every typed error that crosses a crate seam ends in a logged process-exit detail, channel-response error, hook-decision detail or send-refused line, carrying codes only at WARN or ERROR.
decision/full-error-chain-in-detail-file-only · Full error chain in the detail file only — The full error chain is written as a one-line JSON array chain:[...] only to the instance's per-process detail file, and only when an instance resolves.
decision/serde-errors-become-internal-error · Serde error chains replaced by internal error — At dispatch, an error chain whose source is a serde_json or serde_path_to_error error is replaced by the fixed message "internal error" before it reaches stderr, --json, MCP isError or channel error.data. · why: A scrubbing layer at the source.
decision/typed-errors-fixed-display · Typed errors with fixed Display messages — Every typed error has a fixed-message Display, written with thiserror, and holds no fields with paths or payloads. · except: PtyError's Display is hand-written, as viola-pty takes no thiserror.
decision/scrubbing-at-source · Scrubbing at the source — Error scrubbing sits at the source, not in a reporter's before_send hook. · why: No reporter exists.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
decision/env-vars-not-config-channel · Env vars are not a configuration channel — Backtrace capture is not gated by RUST_BACKTRACE or any env var. · why: env vars are not a configuration channel
contract/mcp-stdin/oversize-line · viola mcp stdin · line past MAX_FRAME — A line past MAX_FRAME is refused whole, gives no isError result and ends the session as a closed stdin: exit 0 after initialize, exit 1 before it with both streams empty; its one record is parse-rejected mcp-stdin oversize, no byte of the line.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/default-deny-log-fields · Default-deny for logged fields — A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure. · guard: gate G4 against schemas/diag-line.v1.json
decision/fixed-error-messages · Fixed error messages — Error Display texts are fixed, and serde-sourced error chains are mapped to fixed messages at dispatch.
decision/home-level-lines-path-free · Home-level lines path-free — Home-level log lines are path-free: file fields are basenames, panic_location is workspace-relative and source file recording is off; absolute paths appear only in detail files. · why: The OS username inside absolute paths is the only PII in scope.
decision/schema-top-level-unevaluated-properties · Default-deny schema via top-level unevaluatedProperties — The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists. · why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then · guard: contract_diag_schema
decision/secrets-never-logged · Secrets never logged — Signing credentials and R8-stripped CLAUDE* values are never logged.
decision/content-canary-never-home-level · Content canary never in home-level files — The content canary never appears in a home-level diagnostics ndjson file.
decision/content-only-in-detail-file · Content-bearing records only in the detail file — Content-bearing records go only to the instance-scoped detail-<process>.ndjson file.
decision/low-class-metadata-as-typed-fields · Low-class metadata as typed fields — Low-class metadata is logged as typed fields, never inside detail codes.
decision/no-allow-list-widening · Allow-list cannot be widened — No config key, flag or env var can widen the logged-field allow-list or disable redaction.
decision/secret-scan-before-upload · Secret scan before artifact upload — CI runs the secret scan over every diagnostics/ file before any artifact upload, its classes being CLAUDE* values, the content canary, mode 0600 and an unreadable file; a Cookie header line is no class and no hit. · guard: secret-scan test
decision/veil-redact-second-layer · veil Redact as the second redaction layer — veil's derive Redact is the second redaction layer, so a stray value capture prints [REDACTED] instead of content.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/closed-protocol-codes · Closed protocol codes — Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value. · guard: answer_invalid_params_names_its_cause_and_no_param
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — thiserror messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
decision/typed-ids-never-in-detail · Ids logged as typed fields — pid, child_pid, agent_session_id and started_at are logged as typed fields, never inside detail codes.
decision/canary-only-in-detail-files · Canary only in detail files — Every E2E and integration input is synthetic and embeds a fixed canary, which may appear in detail files but never in a home-level diagnostics file. · guard: mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/g4-schema-conformance-gate · Schema conformance gate on every line — Every home-level line is validated against the diagnostics line schema and every detail line against the detail schema, in a step that always runs. · guard: G4

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/diag-lines-schema-checked-in-unit-tests · Emitted lines schema-checked in integration and E2E tests — Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
decision/schema-conformance-gate · Schema conformance gate on real process output — Every role-file line of test homes must validate against schemas/diag-line.v1.json and every detail line against schemas/diag-detail.v1.json; a failing line fails the build, a torn non-JSON line is skipped and counted. · why: schema failures count as a build failure and unit tests check only in-memory lines · guard: gate G4 (schema-conformance step)

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/module-boundary-error-logging · Errors logged once at module boundaries — Every typed error that crosses a crate seam is logged once at WARN or ERROR with a closed detail or error_code; the full chain goes only to the instance detail file.
decision/no-retry-policies · Retry policies forbidden — Retry-once policies are forbidden, for tests and for panics alike. · why: retry policies mask real signals
decision/no-sleep-based-log-waits · No sleep-based log waits — No log-based wait relies on sleeps.
decision/schema-conformance-failure-fails · Schema-conformance failure is a failure — An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.
decision/print-and-macro-lint-failure-fails · Print and macro lint failure is a failure — A clippy disallowed-macros, print_stdout, print_stderr or dbg_macro failure counts as an SLO invariant failure.
decision/zero-unlogged-panics · Zero unlogged panics — Every panic in run, hook, mcp and cli after init step 4 produces exactly one single-line ERROR panic record in its home-level role file, with no multi-line backtrace there, and hook still exits 0; retry-once policies for panic handling are forbidden. · guard: G2 · except: the bounded exemptions where no file exists, so no line can be written; G2 alone leaves out the fake-agent seam's deliberate panic from its count.

## slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin 10-slo-invariants-telemetry-budgets
decision/no-retry-on-panic · No retry on panic — Retry-once policies for panic handling are forbidden. · why: Parallel to the tests' retries = 0.

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/claude-env-and-cookie-never-logged · CLAUDE* values never logged — CLAUDE* values are never logged, at any level, in any file, detail files included.
decision/field-allow-list-default-deny · Field allow-list is default-deny — No field is logged unless named in the field catalog.
decision/no-paths-or-pids-in-detail-codes · No paths or pids in detail codes — detail codes never contain paths or pids.
decision/no-pii-in-span-fields · No PII or content in span fields — Span fields never include PII or content. · why: Scrubbing at an exporter stage is too late, and v1 has no exporter stage at all.
decision/no-retry-once · No retry-once policies — No retry-once policy is added for panics or flaky telemetry assertions.
decision/no-serde-source-in-error-output · No serde source in printed errors — An anyhow chain that still contains a serde_json or serde_path_to_error source is never printed to stderr, --json, MCP isError or channel error.data.
decision/paths-only-in-instance-detail-files · Paths only in instance detail files — Paths appear only in instance detail files.
decision/closed-event-and-corr-on-every-line · Closed event enum on every line, corr where correlated — Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
decision/event-values-closed · event values closed — No event value is added without a Decisions Log entry.
decision/file-sink-only-error-sink · File sink is the only error sink — Local logs are never optional: the file sink is the only error sink and works with every other feature off.
decision/machine-asserted-telemetry-verdicts · Machine-asserted telemetry verdicts — Log analysis is never human-review-gated; every CI telemetry verdict is a jq or hyperfine assertion that sets an exit code.
decision/must-trace-paths-always-traced · Must-trace paths always traced — Spans and events are never skipped on the 6 must-trace paths. · why: The agent loses its ability to debug without them.
decision/no-ci-env-metadata-in-logs · No CI env metadata in product logs — CI env metadata is never injected into product log lines. · why: The product never reads env as configuration.
decision/no-instrument-err-ret-on-sensitive · No instrument err or ret on sensitive results — #[instrument(err)] and #[instrument(ret)] are never used where the error or return can carry a serde or serde_path_to_error source or user content.
decision/no-print-macros · No print macros in product code paths — print!, println!, eprint!, eprintln! and dbg! are never used in the hook, run or mcp code paths of the product crates, and every product member inherits the workspace lints. · why: They bypass the file sink and break the Hook contract, the run terminal rule and MCP framing. · guard: clippy print_stdout, print_stderr and dbg_macro at deny · except: viola-harness, the fake agent and the output modules that own --json, human CLI stderr and the hook decision body.
decision/no-telemetry-from-screen-content · No telemetry from screen content — Telemetry is never derived from screen content; vt100 is used for readiness and modal gate outcomes only.
decision/no-telemetry-on-stdout · No telemetry on stdout — Telemetry is never written to stdout on any role. · why: stdout carries each role's own output and run's terminal carries the child's screen.
contract/stdout-by-role/run · stdout by role · run — The terminal of run carries the child's screen.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/content-only-in-instance-detail-file · Content only in instance-scoped detail files — Content-bearing detail goes only to the instance's detail-<process>.ndjson file; home-level log files are codes-only and carry no user content, token or CLAUDE* value. · why: security allows user-derived content only in instance-scoped diagnostics
decision/diag-schema-matches-event-enum · Diagnostic line schema matches the event enum — A hand-maintained JSON Schema of the process-log line is committed, and its event enum must equal the ObsEvent variants, asserted by a tests-owned check.
decision/fixed-error-display · Fixed error Display text — Typed errors have fixed Display text that carries no interpolated content.

## log-file-location · key of ## 3. Observability Harness Contract
contract/diagnostics-files/paths · diagnostics files · process log paths — Process logs are written to <home>/diagnostics/ as run-<name>, hook-<name>, mcp and cli-<name> .ndjson files, and detail to <home>/instances/<name>/diagnostics/detail-<process>.ndjson.

## log-format-json-schema · key of ## 3. Observability Harness Contract
contract/process-log-line/process · the process log line · process — One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing cli-<name>.ndjson.
decision/obs-aligns-to-tests-log-format · Observability aligns to the tests log format — The log format is bound by the test plan's harness contract; observability may add fields but must not rename them or remove one the product still emits. · why: the harness greps on them
contract/process-log-line/corr · the process log line · corr — Copied unchanged as a JSON number or string, never renamed and never debug- or display-formatted; required only on the events whose rules always define it.
contract/process-log-line/instance · the process log line · instance — The instance's name, or absent when none resolved.
contract/process-log-line/level · the process log line · level — One of DEBUG, INFO, WARN, ERROR.
contract/process-log-line/message · the process log line · message — The message is a static literal equal to the event name and never carries interpolated content.
contract/process-log-line/target · the process log line · target — The target is always present.
contract/process-log-line/timestamp · the process log line · timestamp — RFC 3339 UTC with milliseconds and Z.

## logging-stack · key of ## 3. Observability Harness Contract
decision/no-content-in-home-level-lines · No content fields in home-level lines — No home-level diagnostics line carries chain, drift_report, panic_payload or backtrace. · guard: G4 diag-line.v1.json schema
contract/diagnostics-line/role-file-line-size · the diagnostics line · role-file line size — A line in a role file (run, hook, mcp, cli) is codes-only and stays well under 4 KiB.
decision/file-only-diagnostics-sinks · File-only diagnostics, one file per role — Every role logs only to its own file under the home's diagnostics directory; no role has a stdout sink, and there is no pretty or dev sink.
contract/diagnostics-files/detail · the diagnostics files · detail — Content-bearing records (full error chains, drift reports, panic payload and backtrace) go only to <home>/instances/<name>/diagnostics/detail-<process>.ndjson, file 0600 in a 0700 directory, and only when an instance resolves.
contract/diagnostics-line/format · the diagnostics line · format — Each diagnostics line is one structured JSON object on one line, written whole by one write of the complete line on an append-mode handle.
contract/obs-event-macro/attached-fields · the obs_event macro · attached fields — Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.

## otel-sdk-init · key of ## 3. Observability Harness Contract
decision/diagnostics-init-before-role-work · Diagnostics init before the role's work — Diagnostics init and the process-start line come before the role's work: in run before its collision check and the child spawn, in mcp before its stdio transport starts. · why: Exit-1 causes are logged.
decision/flat-log-lines-and-one-level-gate · Log lines are flat and Targets is the only level gate — The subscriber flattens event fields to the top level of each line, and the Targets filter is the only configurable level gate; its level comes from the home's diagnostics level and never from the environment. · why: The diagnostics schema and the log readers read event fields as top-level keys, and only the viola crates' targets pass the gate.
contract/config-json/known-keys · config.json · known keys — config.json has four known keys: v, diagnostics_level, claude_env_keep, dialog_hold_s; any other key, a retired five_hour or seven_day included, is counted as skipped in one parse-rejected config-json unknown-keys line at its process's start, and nothing refuses.
contract/config-json/v · config.json · v — A document that is no object, or whose v is not 1, gives the hold default and no further line.

## service-identity · key of ## 3. Observability Harness Contract
decision/env-vars-not-configuration · Environment variables are not configuration — Environment variables are not a configuration channel: service identity has no env-var source or fallback, and the one CLAUDE_CONFIG_DIR value viola verify reads, only when the strip keeps that name, locates the claude config file its child reads, is never logged and configures nothing of viola.
contract/diagnostics-line/instance · the diagnostics line · instance — Every diagnostics line of a process with an instance carries it as field instance; a process without one, such as an mcp server with no instance name, writes no instance key.
contract/diagnostics-line/process · the diagnostics line · process — Every diagnostics line carries the role as field process.

## trace-context-propagation · key of ## 3. Observability Harness Contract
decision/explicit-correlation-values · Correlation values passed explicitly — corr, conn and instance are passed as explicit values into each thread or task and set on each event, never recovered from span context. · why: Spans are not serialized.

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
