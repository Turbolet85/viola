# obs-plan · the rows that bind 2026-11-28-console-scaffold — picked 406 of 1056 (257 by every draw) · rule any · pin 5229

## obs-plan-viola · (above the first heading)
decision/agent-readable-telemetry · Agent-readable telemetry only — Every telemetry signal must be readable by an agent from a snapshot or pasted log excerpt: no human-only dashboards, no proprietary APM lock-in, no human-review-gated log analysis. · except: A surface that cannot be agent-readable is flagged not-instrumentable and handled through boundary instrumentation only.

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/cli-file-sink-only · Short-lived CLI logs to a file sink only — Short-lived CLI verbs log only to a JSON-per-line file sink in diagnostics when an instance resolves, never to stdout. · why: Stdout carries the --json results.
decision/diagnostics-owner-only-modes · Diagnostics files are owner-only — Every diagnostics file is mode 0600 inside a 0700 directory.
decision/distinct-detail-code-per-exit-cause · Distinct detail code per exit cause — Each cause behind exit 21 and exit 1 gets a distinct detail code with no path or pid. · except: A strict-modes check refused before any log opens writes no line; its cause is the fixed stderr pair or the --json document.
decision/error-chain-fixed-stderr-message · Error chains reach stderr as fixed messages — In the viola root binary, the only crate that uses anyhow, an error chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to the diagnostics logs only.
decision/instance-never-from-working-directory · Instance never from the working directory — The instance comes from VIOLA_NAME or an argument, never from the working directory.
decision/level-filter-not-from-env · Level filter never read from RUST_LOG — The log level filter is never read from `RUST_LOG`.
decision/no-env-log-levels · No per-module log levels from the environment — Per-module log levels are never read from environment variables. · why: Environment variables are not a configuration channel.
decision/obs-deps-tokio-free · Obs dependencies Tokio-free and C-free — Obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass cargo deny. · guard: cargo deny
decision/one-log-file-per-role · One log file per process role — Each process role writes its own file under `diagnostics/`, no role has a stdout sink, and no role falls back to stderr JSON when its file cannot be opened.
decision/pty-byte-stream-never-logged · PTY byte stream never logged — The PTY byte stream is never logged; the viola-pty seam is instrumentable, and the openpty OS API behind it is boundary-only. · why: The PTY byte stream is user content.
decision/redaction-not-disableable · Redaction cannot be disabled — No verbosity control, env, flag or config may disable or bypass redaction.
decision/refused-diagnostics-drops-line · Refused diagnostics file drops its line — A refused `diagnostics/` folder or file is treated as a failed open and its log line is dropped.
decision/serde-errors-fixed-message · Serde error chains become a fixed message — A serde-sourced anyhow error chain is replaced by a fixed message before it reaches stderr.
decision/single-file-sink-no-os-exporter · One file sink and one line schema — Telemetry on Linux goes to one file sink with one line schema, with no OS-specific exporter.
decision/spans-not-wire-contract · Tracing spans are not part of the wire contract — Span names are internal `tracing` spans in `<area>.<operation>` snake_case form; the agent-facing contract is the closed `event` enum plus `corr`. · why: the config sets `with_current_span(false)`, so spans do not reach the line
decision/stderr-fixed-strings · Fixed strings on stderr — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/strict-modes-failure-lines · Where a strict-modes failure is logged — A strict-modes failure has a log line only on the `ledger-stamps` read in the `run` version gate, on `verify`'s own `process-exit` for a refused stamp write, and on `viola up`'s own `process-exit` for a home tree refused after its board scan.
decision/tracing-json-line-file-pipeline · tracing JSON-per-line file pipeline — Telemetry is tracing plus tracing-subscriber writing JSON-per-line files under diagnostics/, correlated by corr and consumed through agent-run logs and status. · why: The pipeline is the one the tests plan binds.
decision/tracing-json-pipeline · tracing with the tracing-subscriber JSON formatter — The `tracing` facade plus the tracing-subscriber JSON formatter is the binding logging pipeline, and JSON-to-file is the only logging mode.
decision/typed-keys-never-logged · Typed keys never logged — Keys a human types never appear in a log line; the wheel product event in events.ndjson is the audit record.
contract/diag-line/instance · diagnostic log line · instance — The instance's `ViolaName`; the key is absent when the process has no instance.
contract/diag-line/process · diagnostic log line · process — The role of the writing process, one of run, hook, mcp, cli; required on every line and matching `sender` / `writer`.
contract/diag-line/service-name · diagnostic log line · service.name — Hardcoded to "viola".
contract/diag-line/service-version · diagnostic log line · service.version — The crate version, fixed at compile time.
contract/diagnostics-files/cli · the diagnostics files · cli — Role cli writes <home>/diagnostics/cli-<name>.ndjson, only when an instance resolves.
contract/harness-logs/filters · harness logs command · filters — Output can be filtered by `--instance`, `--kind` and `--process`.
contract/harness-logs/merge · harness logs command · merged sources — Merges `events.ndjson` lines (`src:"events"`, with byte `offset`) with `diagnostics/*.ndjson` lines (`src:"diag"`).
contract/harness-logs/output · harness logs command · output — Output is ndjson on stdout, ready for `jq` or pasting to an agent.
contract/harness-logs/torn-line · harness logs command · torn lines — A torn line is emitted with its `src` and `offset` and `"torn":true`, never dropped.
contract/log-line/process · every log line · process — Every log line carries a process key naming its role: one of run, hook, mcp, cli.
decision/claude-fixtures-scrub-required · Claude fixtures are scrub-required — Build and CI infrastructure is not instrumented, and the claude fixtures are scrub-required data, never a telemetry source. · why: Not product runtime.
decision/corr-field-propagation · Propagation through the corr field — Correlation across the viola-channel is carried by the corr field in log lines of the process on each side. · why: Channel params declare no trace-context field.
decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits.
decision/ndjson-logs-on-disk · Bridge logs as ndjson on disk — The bridge's logs are ndjson on disk, and the event log serves as the audit trail. · why: logs on disk survive restarts on either side
decision/os-apis-at-call-sites · OS APIs observed at call sites — OS APIs (openpty, Unix socket, filesystem locks and atomic rename, sysinfo) and the hyprctl spawn are instrumented at viola's call sites only, never in their internals.
decision/send-text-confined · Send text confined to instance diagnostics — `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
decision/send-text-never-logged · Send text never logged — Sent text never appears on stderr, in --json output, in MCP results or in any process-log field outside instances/<name>/diagnostics/.
decision/viola-core-emits-no-telemetry · viola-core emits no telemetry — The viola-core crate supplies log-field values but emits nothing and is not instrumented. · why: A pure type crate with no I/O.
contract/process-exit-detail/strict-modes-failed · process-exit detail codes · strict-modes-failed — Carried by `verify`'s `process-exit` for a refused stamp write, and by `viola up`'s `process-exit` for a home tree refused after its board scan, each with exit 1.

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/config-observed-at-load · Configuration observed only at load — Configuration is observed only at its parse at load time, logged as skipped-key counts and as one malformed line per key whose value is refused. · why: Static configuration and data.
decision/controls-escaped-in-terminal-output · Controls escaped in terminal output — C0 and C1 control characters are escaped in any human terminal rendering of logged rows.
decision/detail-codes-no-paths · Detail codes carry no path or pid — No path or pid appears in a detail code.
decision/diagnostics-checked-at-open · Diagnostics checked before open — Before a role or detail file is created or narrowed, the existing diagnostics/ folder and the file are checked, a symlink refused and no group or other bit allowed; a refusal is a failed open.
decision/diagnostics-file-modes · Diagnostics file and directory modes — Diagnostics files are 0600 inside 0700 directories. · guard: secret-scan test
decision/diagnostics-owner-only-permissions · Diagnostics owner-only permissions — Files under diagnostics/ are 0600 inside 0700 directories. · guard: secret-scan test
decision/exit-code-log-field · Exit code as a log field — The exit code (0, 1, 2, 10, 12–14, 20, 21), the machine-readable outcome, is carried as a log field; 11 is retired and never reused.
decision/fault-telemetry-per-class · Fault telemetry per class — Each fault class emits its own telemetry: a healed torn ndjson line with file and offset, a corrupt or unsupported snapshot recovered by replay, a child exit seen without EOF, an endpoint vanished during wait, and a stale heartbeat against a live pid.
decision/fixed-stderr-messages · Fixed stderr refusal and fault lines — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/instance-from-name-not-cwd · Instance from its name, never the working directory — The instance comes from VIOLA_NAME or an explicit argument, never from the working directory.
decision/json-to-file-only-mode · JSON-to-file the only logging mode — JSON-to-file is the only logging mode; there is no separate agent-mode flag.
decision/linux-only-os-target · Linux-only OS target — The OS target is Linux only: the core on any current distribution, CI on ubuntu-latest.
decision/log-file-basenames-follow-tests · Log file names follow tests — Process-log files are <home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson, as tests require. · why: Tests is upstream; arch and security place diagnostics under the instance directory instead.
decision/log-level-not-from-env · Log level never from the environment — The log level filter is never read from RUST_LOG; any verbosity control comes from config.json or a CLI flag and never bypasses redaction. · why: Environment variables are not a configuration channel, and no env, flag or config value may disable a control.
decision/no-process-log-without-instance · No process log without an instance — A short-lived CLI verb run with no resolved instance writes no process-log file; its human stderr carries only fixed-message lines, and its outcome is the exit code and stdout.
decision/no-telemetry-from-screen · No telemetry from screen content — No telemetry is derived from screen content; vt100 serves gate outcomes only.
decision/obs-deps-no-c-building-crates · Obs dependencies build no C — Observability dependencies add no C-building crates.
decision/obs-deps-pass-cargo-deny · Obs dependencies pass cargo deny — Observability dependencies pass cargo deny on its one target, x86_64-unknown-linux-gnu. · guard: cargo deny
decision/panic-hook-replaced · Panic hook replaced by a one-line writer — Each process replaces the default panic hook, as the first statement in main and before any child spawns, with one that writes one JSON line and nothing to stderr. · why: The default hook prints to stderr, which breaks the hook contract and is the child's terminal under run.
decision/process-init-order · Process logging init order — Each process installs its panic hook, parses arguments, resolves home and instance and runs its strict-modes check, opens its diagnostics file append-mode 0600 in a 0700 directory, installs the subscriber, then emits process-start. · why: For run this completes before the start sequence so the collision check and version gate are logged; for mcp, before the stdio transport starts.
decision/process-logs-file-sink-only · Process logs to a file sink only — Process logs are written only as JSON lines to one file per process role under diagnostics/; there is no network exporter and no stdout sink. · why: viola makes no outbound network calls, and stdout carries --json results.
decision/redaction-cannot-be-disabled · Redaction cannot be disabled — No environment variable, flag or config setting may disable log redaction.
decision/serde-chain-fixed-message · Serde error chains become a fixed message on stderr — An error chain that contains a serde source becomes a fixed message before it reaches stderr.
decision/serde-errors-become-fixed-message · Serde errors become a fixed message — An error chain holding a serde source is replaced by a fixed message before it reaches stderr.
decision/single-line-log-records · Single-line log records — Every log record is one JSON line parseable by jq; never a multi-line stack trace.
decision/skipped-counters · Skipped counters — Unknown event kinds, unknown fields and torn lines are each counted in skipped counters.
decision/strict-modes-refusal-writes-no-line · Strict-modes refusal writes no line — A strict-modes check refused before any log opens ends the process with nothing under the home opened and no log line; its cause is carried only by the fixed stderr pair or the --json document. · why: Ratified as the answer to whether a refused strict-modes check logs. · except: The ledger-stamps read in run's version gate and a refused update_stamps in viola verify each leave a line.
decision/structured-json-process-logs · Structured JSON process logs — Every process role (run, hook, mcp, cli) writes structured JSON process logs under diagnostics/, which harness logs merges with events.ndjson.
decision/subscriber-explicit-writer · Subscriber installed with an explicit writer — The tracing subscriber is always installed with an explicit writer, never stdout. · why: tracing-subscriber fmt() writes to stdout unless a writer is configured, and stdout carries MCP frames.
decision/sync-instrumentation-no-tokio · Sync instrumentation without Tokio — Instrumentation in the sync crates runs on std threads with no Tokio. · why: Hybrid concurrency. · guard: cargo deny ban
decision/tests-event-list-adopted · Tests event list adopted — The tests' field and event list is adopted verbatim, with send-issued and send-refused among the logged events.
decision/tracing-json-logging-pipeline · tracing with a JSON subscriber for logging — Logging and spans go through the Rust tracing facade with a tracing-subscriber JSON structured logger.
contract/cli-internal-error/stderr-line · CLI internal error · stderr line — Without --json a catch-site fault prints the fixed line `error: internal error` to stderr.
contract/logs-wrapper/record-envelope · logs wrapper · record envelope — Each process-log line is wrapped as {"src":"diag","file":<basename>,"record":<line>}.
contract/process-log-corr/panic · Process log corr by event · panic — corr is null.
contract/process-log-corr/process-exit · Process log corr by event · process-exit — corr is null.
contract/process-log-corr/process-start · Process log corr by event · process-start — corr is null.
contract/process-log/event · Process log line · event — A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
contract/process-log/instance · Process log line · instance — A ViolaName or null.
contract/process-log/process · Process log line · process — The process role, one of run, hook, mcp, cli.
contract/process-log/timestamp · Process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
contract/service-identity/service-name · Service identity · service.name — Hardcoded "viola", the binary name, never read from OTEL_SERVICE_NAME or SERVICE_NAME.
contract/service-identity/service-version · Service identity · service.version — The compile-time CARGO_PKG_VERSION, the same value used as sender and writer.
decision/anyhow-error-capture-boundary · The anyhow crate is the error-capture boundary — The only crate that uses anyhow is the error-capture boundary, with context chains only at dispatch; the full chain goes to diagnostics only.
decision/child-exit-from-handle-wait · Child exit from the process handle — The child's process-exit comes from the wait on its process handle, never from reader end-of-file. · why: Child exit is detected on the process handle.
decision/no-log-rotation-v1 · No log rotation in v1 — events.ndjson and diagnostics/*.ndjson are not rotated in v1; any future rotation keeps 0600/0700 permissions and keeps wait after and send cursor offsets valid. · why: The trigger is operational, not compliance: the security tier is Minimal with no compliance triggers.
decision/one-linux-log-sink · One file sink on Linux — One file sink and one line schema serve Linux, with no OS-specific exporter.
decision/per-parser-parse-failure-counts · Per-parser parse-failure counts — Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, the hyprctl replies, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead.
contract/cli-json-error/strict-modes-failed · CLI --json error document · instance-unreachable strict-modes-failed — A refused instance check of send, wait, last, answer, pause, release, link, unlink or switch exits 21 with the instance pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
contract/cli-log-file/link-unlink · CLI process-log file · viola link / viola unlink — Log as cli into the driven instance's cli-<driven>.ndjson, never the caller's VIOLA_NAME, once their instance check of it has passed.
contract/cli-log-file/revive-list · CLI process-log file · viola revive --list — Logs as cli into the caller's cli-<VIOLA_NAME>.ndjson once its strict-modes check has passed; no file with no VIOLA_NAME.
contract/cli-log-file/revive-start · CLI process-log file · viola revive start arm — Logs as the wrapper, process="run", into diagnostics/run-<name>.ndjson.
contract/exit-21-detail/strict-modes-failed · Exit 21 detail codes · strict-modes-failed — strict-modes-failed is a distinct detail code for an exit-21 cause, naming no path or pid.
contract/process-log/child-and-probe-lifecycle · Process log · child and probe lifecycle — process-start and process-exit are logged for the claude child, for the claude agents --json and --version probes and for verify's print-mode sender; the terminal viola open or viola up spawns gets a process-start alone.
contract/process-log/corr · Process log line · corr — Copied unchanged as a JSON number or string, never renamed.
contract/process-log/level · Process log line · level — One of DEBUG, INFO, WARN, ERROR.
contract/send-log-line/detail · Send log line · detail — The send detail is one of input-not-ready, no-prompt-submitted, control-character, empty-text, turn-running or paste-cut.
contract/send-path-log-events/event-order · Send path log events · event order — A send logs channel-request, then send-issued, then hook-invoked (UserPromptSubmit, or SessionStart for a confirmed /clear or /compact; none awaited for an unconfirmable send, a cut paste or a local command its transcript echo confirms), then send-confirmed or send-refused, then channel-response.
contract/viola-verify/home-tree-refused · viola verify · refused home-tree check — A refused home-tree check exits 1 with the home pair, opens no log and writes no line; a refused update_stamps exits 1 with the home pair and, when an instance is named, ends the opened log with a process-exit line carrying detail strict-modes-failed.

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/one-line-panic-record · One structured line per panic — Every panic after init step 4 produces exactly one single-line structured record in its home-level role file with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file. · except: the bounded panic exemptions listed in the plan's Section 10
decision/span-naming · Span naming as <area>.<operation> — Span names are <area>.<operation> in snake_case, <area> one of run, pty, channel, state, hook, send, wait, last, answer, pause, release, cli, mcp, hyprland; cli.<verb> names the dispatch span of a short-lived verb.
contract/process-log-line/event · the process log line · event — The event field is a closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
contract/process-start-event/service-name · the process-start event · service_name — Emitted once per process as the fixed value "viola", the binary name.
contract/process-start-event/version · the process-start event · version — Emitted once per process as the crate package version.
decision/machine-parseable-signals · Every signal machine-parseable — Every signal produces machine-parseable output: JSON-per-line process logs, the ndjson product event stream, JSON harness documents, hyperfine JSON for perf gates and nextest JUnit XML in CI.
decision/no-event-payloads-in-process-logs · No event payloads in process logs — Obs never copies event payloads into process logs.
decision/no-human-gated-log-analysis · No human-review-gated log analysis — Every log verdict is a jq or harness assertion that sets an exit code, never a human review.
decision/no-human-only-dashboards · No human-only dashboards — No Grafana, Kibana, Datadog or other human-only dashboard UI exists in the stack, and none may be added as a consumption surface.
decision/no-proprietary-apm-sdk · No proprietary APM SDK — No proprietary APM SDK is used, as primary or at all; the v1 exporter role is filled by the local file sink.
decision/spans-not-serialized-v1 · Spans declared but not serialized — Spans are declared in code but not serialized in v1.
contract/process-log-line/at-instants · the process log line · *_at — Instants are named *_at and written in RFC 3339 UTC with milliseconds and a Z suffix.
contract/process-log-line/enum-values · the process log line · enum values — Enum values are kebab-case.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/panic-caught-at-every-role · Panics caught at every role's catch site — A hand-written panic hook plus catch_unwind at every role's main-thread catch site (covering hook dispatch and run's inline handle-wait), the three run PTY threads (the two pump copy threads and the paste writer thread) and the vt100 feed turn a panic into one record.
decision/panic-one-line-record · One single-line record a panic — Every panic after obs init produces exactly one single-line structured record in its home-level role file, with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file. · except: the pre-init window and the paths that refuse or lack an instance or home tree before their role file opens
decision/pre-open-strict-modes-refusal-silent · Strict-modes refusal before a log opens writes no line — A strict-modes refusal made before any process log opens writes no line and carries no logged detail code; its machine-readable form is the exit code and, under --json, its closed document. · except: a refused socket directory always logs detail socket-dir-refused, except at MCP's frame tool, which writes no line
decision/span-naming-skip-all · Span declaration convention — Every span is declared with instrument, skip_all and a name of the form "<area>.<operation>".
decision/structured-logs-primary-signal · Structured logs are the primary signal — tracing-subscriber JSON-per-line logs, one file per process role under the home's diagnostics directory, are the primary observability signal, merged with the event stream by agent-run logs.
decision/cross-process-join-corr-conn · Cross-process joins by corr and conn — Lines across processes are joined by the corr and conn fields, not by W3C trace IDs.
decision/spans-not-serialized-in-v1 · Spans declared but not serialized in v1 — Spans are declared on the must-trace paths and every crate seam but are not written to output in v1. · why: they give the code its structure and are ready for the deferred OpenTelemetry bridge
contract/strict-modes-refusal-exit/1 · strict-modes refusal exit codes · 1 — run, revive's start, adopt's start, verify, open and up's home-tree check exit 1 on a strict-modes refusal; revive, adopt, open and up under --json print the strict-modes-failed document.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/codes-only-process-logs · Codes-only process logs, content in detail files — Every role's codes-only process log goes to its home-level role file under diagnostics, and content-bearing detail goes only to the instance's per-role detail file.
decision/obs-owns-logger-and-format · Obs owns the logger and format — The logger and the log format are owned by observability.
contract/run-terminal-output/start-refusal · run terminal output · start refusal — A start refusal before any child is spawned writes exactly two fixed stderr lines with no path or pid: the home pair for a home-level refusal, the instance pair for an instance-level one.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/dependency-targets-off-in-sink · rmcp and portable-pty targets off — The rmcp and portable-pty log targets are switched OFF in the sink.
decision/detail-codes-only · detail is a closed code — The detail field is a closed kebab-case code catalog and never carries a path or pid.
decision/reader-never-heals-torn-lines · Reader never heals torn lines — A reader of an append log neither heals a torn line nor logs one; only the appender heals.
decision/span-field-allow-list · Spans declared with explicit field allow-list — Every span is declared skipping all arguments with an explicit field allow-list.
decision/static-span-names · Static span names — Span names are static strings; instance names, dialog ids and paths never appear in a span name.
decision/no-assistant-message-in-logs · No assistant message or result body in logs — last_assistant_message never appears in a log, and no result body reaches a log line.
decision/no-err-ret-span-options · err and ret span options banned — The err and ret instrumentation options are banned on spans.
decision/uninstrumented-components · Components left uninstrumented — viola-core, config.json and plugin files, and build/CI infrastructure are not instrumented. · why: viola-core only supplies ObsEvent and RefusalReason values; the rest are not instrumentable.
decision/wheel-event-is-audit-record · Wheel product event is the audit record — The wheel product event in events.ndjson is the audit record of a wheel change; the typed keys are never logged and the wheel transition span is only a point in time.
contract/cli-log-file/focus · CLI process-log file · viola focus — Logs as cli into the named instance's cli-<name>.ndjson, a process-start and a closed-detail process-exit inside the cli.focus dispatch span, which carries no fields.
contract/viola-verify-refusal/home-tree-refused · viola verify strict-modes refusal · home tree refused — Before logging, verify holds the home, ledger/, bin/ and plugin/ to the strict-modes check; a refused tree exits 1 with the home pair and writes no line.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/cli-log-only-under-viola-name · cli process log only under a parsed name — A short-lived cli verb opens its process log and writes process-start and process-exit only when an instance resolves: for list, plugin install and revive --list under a parsed VIOLA_NAME (with no name no log), for the frame verbs under the named target (link and unlink the driven) instance.
decision/cli-manual-dispatch-span · Manual dispatch span per cli verb — Each short-lived cli verb opens one manual dispatch span of its own. · why: clap 4.6.7 has no tracing integration.
decision/error-chain-in-detail-log-only · Full error chain only in the cli detail log — The full anyhow error chain of a cli verb goes to the cli detail log only.
decision/home-tree-check-before-start-log · Home-tree check before the start's log — The start of run, of viola revive and of viola adopt checks the home tree (the home, instance set, ledger, bin, plugin, pinned exe and plugin hooks) before its log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
decision/no-typed-keys-in-telemetry · Typed keys never in telemetry — The keys the human types never appear in a span.
decision/portable-pty-log-target-off · portable-pty log target OFF — The log target of portable-pty is OFF in the sink, and PTY failures are captured at the pty seam instead.
decision/refused-check-writes-no-line · A refused check opens no process log — A cli verb whose strict-modes, tree or instance check refuses (exit 21) opens no process log and writes no line.
decision/refused-home-tree-writes-no-line · Refused home tree writes no line — verify and plugin install check the home tree before their log opens, and a refused tree opens no log and writes no line.
contract/cli-stderr/internal-error · CLI stderr · internal fault message — An internal fault prints exactly "error: internal error".
contract/cli-stderr/refusal-pair · CLI stderr · unable and hint pair — A refusal is printed to stderr as a fixed "unable:" line followed by a "hint:" line.
contract/diag-line-schema/os · diag-line v1 schema · os — os is closed to linux.
decision/appender-heals-torn-line · The appender heals a torn line — An append that finds the events log's last byte is not LF writes one LF and its line in a single write; the reader neither heals nor logs.
decision/mcp-explicit-log-writer · Explicit log writer in viola mcp — The viola mcp log writer is always explicit: the mcp.ndjson file, with no stderr fallback.
decision/rmcp-tracing-target-off · rmcp tracing target OFF — The native tracing target of rmcp is OFF in the sink. · why: Its records lack required fields and can carry frame content.
decision/transcript-echo-read-uninstrumented · Transcript echo read uninstrumented — The send's transcript echo read, the end offset before the paste and the tag count after it, opens no span and writes no line or span field of its own; its result shows only as the send-confirmed line inside run.confirm_window.
decision/unreadable-ledger-means-unverified · Unreadable capability ledger means an unverified CLI — A capability ledger that is missing a stamp, unreadable, malformed or refused by the strict-modes check is treated as an unverified CLI (cli_verified false).
contract/plugin-install-exit-codes/21 · plugin install exit codes · 21 — The strict-modes tree check refused; no log is opened and no line is written, with the stderr pair or, under --json, the instance-unreachable document.
contract/refusal-stderr-pair/socket-dir-refused · cli refusal stderr pair · socket-directory refusal — Printed on start and frame-verb refusals in plain ASCII with hint: last and no SGR, naming only the parsed ViolaName.
contract/run-log-parse-rejected/ledger-stamps · the run process log parse-rejected line · parser ledger-stamps — An unreadable, malformed or strict-modes-refused capability ledger gives one parse-rejected line with parser "ledger-stamps", detail "unreadable", "malformed" or "strict-modes-failed", and a count, in the run log.
contract/run-process-log/process-exit-self · run process log · process-exit self — After the process-exit of the child the pump ends on, run emits process-exit{subject:"self", exit_code, duration_ms}; the process-exit of a child a switch replaces is followed by none.
contract/run-process-log/process-start-self · run process log · process-start self — process-start{subject:"self", service_name, version, os, pid, endpoint_kind}.
contract/send-refused-detail/input-not-ready · wrapper send-refused detail · input-not-ready — send-refused{corr:<cursor>, refusal, detail:"input-not-ready", side:"wrapper", wheel}.
contract/send-refused-detail/paste-cut · wrapper send-refused detail · paste-cut — A paste cut at its bound refuses not-delivered paste-cut after send-issued, with the send's cursor; a send refused on a standing cut mark refuses paste-cut with no cursor and nothing issued.
contract/send-refused-detail/turn-running · wrapper send-refused detail · turn-running — Covers a running turn the wrapper knows of before the paste, another send in flight, or a switch standing from its request until the new child's session-start, with no new field; the refusal has no cursor (corr the end offset at refusal), no send-issued, and the client exits 13.
contract/send-refused/human-typing · wrapper send-refused · human-typing — send-refused{corr:<cursor>, refusal:"human-typing", detail:null|"manual-pause", wheel:"human", side:"wrapper"}, with the client process-exit{exit_code:10}.
contract/send-span-attributes/run-readiness-gate · confirmed send span attributes · run.readiness_gate — outcome (ready or input-not-ready) and vt100_panicked (bool).
contract/span-kinds/internal · Span kinds · INTERNAL — The INTERNAL spans are run.*, state.* and pty.paste_write; a hook's decision is the decision_emitted field, not a span.
contract/verify-exit-codes/0 · verify exit codes · 0 — Every relied ledger row counted: each passed, or the browser-tools-prompt row read absent.
contract/verify-exit-codes/1 · verify exit codes · 1 — A row failed, verify refused, or verify was stopped.
contract/verify-process-exit/strict-modes-failed · verify process-exit line · detail strict-modes-failed — A stamps store refused by its strict-modes check, under a parsed VIOLA_NAME, ends with process-exit carrying detail "strict-modes-failed" and the home pair on stderr.
contract/verify-stdout/step-line · verify stdout · step line — One "[NN/MM] <row id> <row words>  pass|absent|fail" line is printed for each relied ledger row, MM being the relied row count; absent is the browser-tools-prompt row's word when no run met the prompt.
contract/verify-stdout/summary-line · verify stdout · summary line — The last stdout line is "stamped <version>  N pass  N fail", the two counts summing to MM; the pass count holds every row that counts, an absent browser-tools-prompt row among them.

## 5-metric-coverage · ## 5. Metric Coverage · twin metric-coverage
decision/metric-grouping-closed-enums · Metric grouping by closed enums only — Aggregate reports group only by closed enums (event, detail, refusal, method, hook_event, subject, parser, process); corr, conn, pid, child_pid and instance are join keys, never grouping keys.

## metric-coverage · ## 5. Metric Coverage · twin 5-metric-coverage
decision/panic-lines-stay-zero · Panic lines stay at zero — Gate G2 counts the panic lines of the home-level role files and fails on any; no viola.panic metric is derived.
decision/counters-from-home-role-files · Counters derived from home-level role files — A counter of an event, detail or refusal value is derived with jq over the home-level diagnostics role files only, skipping a torn line. · why: Detail files repeat lines.
contract/cli-process-exit/server-verify-failed · CLI process-exit diagnostic line · server-verify-failed — The CLI's server-verification refusal writes one process-exit with subject self, exit_code 21, detail server-verify-failed and during connect at WARN in cli-<name>.ndjson, with no pid, start time, path or endpoint; MCP's wait tool writes no line for it.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/debug-level-by-config-only · Debug level enabled by config only — The debug log level is enabled only by diagnostics_level "debug" in config.json, never by RUST_LOG.
decision/file-only-per-role-log-sinks · File-only log sinks per role — Diagnostics are written to files only, one sink per role, under the home's diagnostics directory.
decision/json-to-file-only-log-mode · JSON-to-file as the only log mode — JSON to file is the only log mode: there is no agent-mode flag and no stdout pretty sink on any role.
decision/third-party-crate-logs-off · Third-party crate logs off — The per-module log filter turns logging OFF for the rmcp, portable_pty, sysinfo and interprocess targets. · why: Their failures are captured at the viola seam as viola events.
decision/unlisted-targets-log-off · Unlisted log targets off — The per-module log filter turns logging OFF for every target it does not list.
decision/viola-crates-log-at-info · Viola crates log at INFO — The per-module log filter sets the viola, viola_core, viola_pty, viola_channel, viola_state, viola_agent_claude and viola_mcp targets to INFO, or DEBUG via diagnostics_level.
contract/log-levels/error · log levels · error — Used for a panic, a process-exit that carries a detail, and a channel-response with error_code -32603.
contract/log-levels/info · log levels · info — INFO carries all other boundary, lifecycle and state-transition events, including refusals, which are normal product outcomes.
contract/log-line/corr · every log line · corr — Set per the tests' table and copied unchanged as a JSON number or string; null is represented by the key's absence.
contract/log-line/event · every log line · event — A value of the closed kebab-case event enum.
contract/log-line/instance · every log line · instance — The ViolaName; the key is absent when there is none.
contract/log-line/timestamp · every log line · timestamp — RFC 3339 UTC with milliseconds and Z.
contract/process-start-line/service-name · process-start line · service_name — A constant service name, emitted once on process-start rather than on every line.
decision/additive-field-naming · Additive log field naming — Additive log fields are snake_case with kebab-case values, and carry codes only.
decision/content-detail-in-instance-file · Content-bearing detail in the instance detail file — Content-bearing detail goes only to the per-process detail file inside the instance's diagnostics directory.
decision/log-schema-additive-only · Log schema additive to the tests' format — The observability log schema only adds fields and enum values to the tests' log format; it renames nothing and removes only a field no build emits. · why: The tests' log format is the binding text.
contract/log-levels/debug · log levels · debug — DEBUG is reserved for per-frame channel framing detail and uses the same field allow-list.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/unemittable-schema-words-removed · Schema words no build can emit are removed — A closed word of the diag-line schema that no build can emit leaves the schema with its emitter; os is closed at linux.
contract/diag-detail/strict-modes-failed · diagnostic detail words · strict-modes-failed — Written on the process-exit exit 1 of viola verify and of viola up under a parsed VIOLA_NAME, and on parse-rejected ledger-stamps from the version gate of run, at warn, with no path; every other refused strict-modes check writes no log line, its exit code or JSON document its only record.
contract/diag-line-fields/panic · diagnostic line additive fields · panic — panic_location (workspace-relative, or <crate>-<ver>/src/... for dependencies) and thread; payload and backtrace go to the detail file only.
contract/diag-line-fields/process-exit · diagnostic line additive fields · process-exit — subject, exit_code (self), child_exit_status, exit_source (handle-wait or kill-fallback), detail, during (connect or call), duration_ms.
contract/diag-line-fields/process-start · diagnostic line additive fields · process-start — subject (one word of the diag-line subject enum), service_name, version, os, pid, child_pid, endpoint_kind, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept (names only).
contract/diag-line-fields/state-recovered · diagnostic line additive fields · state-recovered — detail (torn-line-healed, snapshot-replayed, snapshot-unsupported-v), file (basename only), offset (torn-line-healed only), v_seen (snapshot-unsupported-v only), scanned (snapshot details only: the instance name when valid, never a path or null).
contract/diag-subject/self · diag-line subject enum · self — One value of the closed subject enum of the diag-line schema.

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/boundary-errors-end-in-logged-line · Crate-seam errors end in a logged line — Every typed error that crosses a crate seam ends in a logged process-exit detail, channel-response error, hook-decision detail or send-refused line, carrying codes only at WARN or ERROR.
decision/full-error-chain-in-detail-file-only · Full error chain in the detail file only — The full error chain is written as a one-line JSON array chain:[...] only to the instance's per-process detail file, and only when an instance resolves.
decision/hand-written-panic-capture · Hand-written panic capture — Panic capture is a hand-written std panic hook plus std catch_unwind, with no panic-reporting crate.
decision/panic-backtrace-in-detail-file · Panic backtrace in the detail file — When an instance resolves, the panic hook also writes the payload and a raw, never-symbolised backtrace as one line to the process's detail file, not gated by RUST_BACKTRACE. · why: Env vars are not a configuration channel.
decision/panic-before-role-file-leaves-no-record · A panic before the role file opens leaves no record — A panic before any role file is opened, in argument parsing or the home and instance checks, leaves no log record, because the panic hook has no sink yet and returns without writing. · why: Strict-modes refusals rely on nothing being created under the home before their check; the panic hook set first keeps the default panic text off stderr.
decision/panic-exit-decided-at-catch-site · Panic exit decided at a catch site — The panic hook only writes; the exit is decided by one catch_unwind in the root binary's main around everything after the hook is set, so no role ends through an uncaught unwind.
decision/panic-hook-silent-on-stderr · Panic hook installed first, silent on stderr — Viola's panic hook is installed by main's first statement, never calls the default hook and never writes to stderr. · why: The hook contract and the run terminal both forbid it.
decision/panic-line-bypasses-tracing · Panic line bypasses tracing — The panic hook writes its one JSON line with a single write to a pre-opened file, not through the tracing subscriber. · why: No re-entrancy if the panic happened inside formatting.
decision/panic-unwind-in-all-profiles · panic = unwind in every profile — Every Cargo profile keeps panic = "unwind". · guard: G3
decision/sentry-not-admissible · Sentry not admissible — sentry and sentry-tracing are not admissible. · why: They break the egress ban, and their default reqwest, native-tls and tokio transport breaks the cargo-deny C-crate and Tokio-in-sync-crates bans.
decision/serde-errors-become-internal-error · Serde error chains replaced by internal error — At dispatch, an error chain whose source is a serde_json or serde_path_to_error error is replaced by the fixed message "internal error" before it reaches stderr, --json, MCP isError or channel error.data. · why: A scrubbing layer at the source.
decision/typed-errors-fixed-display · Typed errors with fixed Display messages — Every typed error has a fixed-message Display, written with thiserror, and holds no fields with paths or payloads. · except: PtyError's Display is hand-written, as viola-pty takes no thiserror.
decision/unhandled-panics-always-captured · Unhandled panics always captured — Every unhandled panic is captured as the one-line event "panic".
contract/panic-line/corr · the panic line · corr — corr is never written on the panic line.
contract/panic-line/instance · the panic line · instance — instance is omitted when none resolved, the same key-absence null encoding obs_event! uses.
contract/panic-line/keys · the panic line · keys — The panic line holds timestamp, level "ERROR", target "viola::panic", message "panic", event "panic", process, instance, panic_location and thread.
decision/anyhow-at-dispatch-edge-only · anyhow at the dispatch edge only — anyhow is used only at the root binary's dispatch edge and its catch-site reporter.
decision/error-correlation-without-trace-ids · Error correlation without trace ids — Errors correlate by corr, conn, process and instance, plus timestamp, not by trace_id or span_id.
decision/human-panic-rejected · human-panic rejected — human-panic is not used. · why: It writes a human report to stderr.
decision/tracing-panic-rejected · tracing-panic rejected — tracing-panic is not used. · why: The crate is stale.
contract/detail-file-line/correlation-keys · the detail-file error line · correlation keys — The detail-file line for an error carries the same event, corr and conn as its home-level counterpart, so jq joins them.
contract/panic-outcome-by-role/cli · panic outcome by role · cli — After a panic, cli logs process-exit{subject:"self", exit_code:1, detail:"internal-error"} when an instance resolves, puts the chain in the detail file only, writes exactly "error: internal error\n" on stderr in one write with no hint, and exits 1.
contract/panic-outcome-by-role/cli-json · panic outcome by role · cli --json — After a panic under --json, cli writes {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty, and exits 1.
contract/panic-outcome-by-role/hook · panic outcome by role · hook — After a panic, hook exits 0 with empty stdout.
contract/panic-outcome-by-role/mcp-main-thread-panic · panic outcome by role · mcp main-thread panic — An mcp main-thread panic writes no process-exit line and nothing to stdout, and the process exits 1.
contract/panic-outcome-by-role/mcp-task-panic · panic outcome by role · mcp spawned-task panic — A panic inside an mcp spawned Tokio task is logged by the hook, contained at the task boundary, and the process continues.
contract/panic-outcome-by-role/run · panic outcome by role · run — After a panic, run logs process-exit{subject:"self", exit_code:1, detail:"internal-error"} and exits 1, writing nothing on the terminal while the child runs.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
decision/env-vars-not-config-channel · Env vars are not a configuration channel — Backtrace capture is not gated by RUST_BACKTRACE or any env var. · why: env vars are not a configuration channel
decision/exit-decided-at-catch-site · Exit decided at a catch site, not the panic hook — The panic hook only writes; everything after the hook is set runs inside one catch_unwind in main, so no role ends through an uncaught unwind.
decision/not-a-directory-refusal-message-fixed · NotADirectory refusal message is fixed and pathless — The NotADirectory strict-modes refusal reads the fixed message "a trusted state path is not a directory" and names no path. · why: The security plan's fixed-message rule keeps paths out of error text, and the fixed-messages unit test pins every refusal's text.
decision/pre-log-strict-refusals-write-no-line · Strict-modes refusals before the log opens write no line — Refusals of the instance and home-tree checks that run before a role file opens write no line in any log and open nothing under the home. · why: Opening a log narrows the home and would erase the refused state; the strict-modes negative tests assert that no diagnostics folder appears.
decision/raw-unsymbolised-backtrace · Panic backtraces captured raw, never symbolised — A panic's detail line carries raw frames (ip, module path, base, offset), up to 62, for offline resolution against the pinned copy; nothing is symbolised in the panic path. · why: symbolising cost most of a hook run and pushed the forced-panic hook past the 1.0 s spine bound
decision/refused-diagnostics-path-is-a-failed-open · A refused diagnostics path is a failed open — A refused diagnostics folder or file is exactly a failed open: the role file does not open, and a detail line is dropped with nothing written and nothing on stderr. · why: The hook carries on with no subscriber, the other roles end as an internal error, and no line is ever written through a widened folder or a symlink.
decision/role-classified-before-clap · Role taken from argv before clap — The role is the first argument that is neither a global flag nor a flag's value and names a role subcommand, so viola --home <dir> hook stop is still a hook whose pre-init panic exits 0. · why: reading argv[1] alone would misclassify that form as cli and exit 1
contract/cli-internal-error/human-line · cli internal error · error: internal error — Without --json the cli catch site prints exactly error: internal error; run, revive's start, adopt's start, mcp and hook print neither.
contract/cli-internal-error/json-document · cli internal error · --json document — Under --json, read among the arguments before a --, the cli catch site prints {"v":1,"error":"internal-error","detail":null} on stdout and stderr stays empty.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/content-canary-never-home-level · Content canary never in home-level files — The content canary never appears in a home-level diagnostics ndjson file.
decision/default-deny-log-fields · Default-deny for logged fields — A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure. · guard: gate G4 against schemas/diag-line.v1.json
decision/detail-schema-self-contained · Separate self-contained detail-line schema — Detail-file lines are validated by their own schema with a top-level unevaluatedProperties false, which inlines the event enum and uses no cross-file reference.
decision/fixed-error-messages · Fixed error messages — Error Display texts are fixed, and serde-sourced error chains are mapped to fixed messages at dispatch.
decision/home-level-lines-path-free · Home-level lines path-free — Home-level log lines are path-free: file fields are basenames, panic_location is workspace-relative and source file recording is off; absolute paths appear only in detail files. · why: The OS username inside absolute paths is the only PII in scope.
decision/low-class-metadata-as-typed-fields · Low-class metadata as typed fields — Low-class metadata is logged as typed fields, never inside detail codes.
decision/no-allow-list-widening · Allow-list cannot be widened — No config key, flag or env var can widen the logged-field allow-list or disable redaction.
decision/schema-top-level-unevaluated-properties · Default-deny schema via top-level unevaluatedProperties — The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists. · why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then · guard: contract_diag_schema
decision/source-allow-list-logging · Source allow-list for logged fields — Spans skip all arguments and declare fields explicitly, and events go through obs_event! with explicit typed fields only; this source allow-list is the primary PII control. · guard: disallowed-macros; raw_event_hits; G1 bare instrument
contract/detail-line/backtrace · the detail-file line · backtrace — One of exactly four content fields a detail-file line may carry: the panic's backtrace.
contract/detail-line/chain · the detail-file line · chain — One of exactly four content fields a detail-file line may carry: the full error chain.
contract/detail-line/instance · the detail-file line · instance — Required on every detail-file line, as on home-level lines.
contract/detail-line/panic-location · the detail-file line · panic_location — Allowed on panic detail lines only, copied from the home-level panic line so both lines join without the payload.
contract/detail-line/panic-payload · the detail-file line · panic_payload — One of exactly four content fields a detail-file line may carry: the panic's payload.
contract/detail-line/process · the detail-file line · process — Required on every detail-file line, as on home-level lines.
contract/detail-line/thread · the detail-file line · thread — Allowed on panic detail lines only, copied from the home-level panic line so both lines join without the payload.
decision/content-only-in-detail-file · Content-bearing records only in the detail file — Content-bearing records go only to the instance-scoped detail-<process>.ndjson file.
decision/secret-scan-before-upload · Secret scan before artifact upload — CI runs the secret scan over every diagnostics/ file before any artifact upload, its classes being CLAUDE* values, the content canary, mode 0600 and an unreadable file; a Cookie header line is no class and no hit. · guard: secret-scan test
contract/detail-line/event · the detail-file line · event — Required on every detail-file line, as on home-level lines.
contract/detail-line/level · the detail-file line · level — Required on every detail-file line, as on home-level lines.
contract/detail-line/message · the detail-file line · message — Required on every detail-file line, as on home-level lines.
contract/detail-line/target · the detail-file line · target — Required on every detail-file line, as on home-level lines.
contract/detail-line/timestamp · the detail-file line · timestamp — Required on every detail-file line, as on home-level lines.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/closed-protocol-codes · Closed protocol codes — Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value. · guard: answer_invalid_params_names_its_cause_and_no_param
decision/content-sink-behind-strict-check · Content sink behind a strict-modes check — The diagnostics content sink opens only after a strict-modes check of the folder and file; a symlink or a folder or file another user may read refuses the open as a failed open and the detail line is dropped. · why: High-class content is never written through a symlink or into a folder another user may read.
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — thiserror messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
decision/typed-ids-never-in-detail · Ids logged as typed fields — pid, child_pid, agent_session_id and started_at are logged as typed fields, never inside detail codes.
decision/veil-redaction · veil derives redaction — veil 0.3.0's derived Redact is what redacts the payload fields of contract types, and instrumented spans skip all arguments.
decision/error-bodies-clean · Clean error bodies — Error bodies of CLI --json, MCP results and the channel faults hold no absolute path, upstream caused-by text or serde path, and every product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
decision/fixtures-not-a-telemetry-source · Fixtures not a telemetry source — Recorded fixtures are not a telemetry source, and observability adds no fixture content.
decision/prompt-and-output-never-in-home-logs · Prompt and assistant output never in home logs — Prompt text and assistant output, last_assistant_message included, never appear in a home-level process-log line; they reach an instance detail file only inside a chain or drift report, and only text_bytes is logged.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/no-abort-panic-strategy · No abort panic strategy — No panic strategy is set to abort in Cargo.toml, cargo config or workflows; the gate passes only on a clean no-match. · guard: G3
decision/g1-bare-instrument-gate · G1 bare-instrument gate fails closed — Every #[instrument] must have skip_all as its first argument; the G1 gate passes only on rg exit 1 (no match), so a violation, a regex or I/O error or a missing rg fails the CI step. · guard: G1
decision/g2-zero-panics-gate · Zero-panics gate fails closed — The zero-panics gate probes itself first, fails closed, runs in the test and perf jobs, and counts panics over home-level role files only. · guard: G2 · except: the fake-agent seam's forced panic at exactly its seam location, compared as a whole string
decision/g4-schema-conformance-gate · Schema conformance gate on every line — Every home-level line is validated against the diagnostics line schema and every detail line against the detail schema, in a step that always runs. · guard: G4

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/banned-telemetry-dependencies · Banned telemetry and logging dependencies — opentelemetry-otlp, opentelemetry-stdout, sentry, tracing-appender, the tracing-subscriber env-filter feature and the veil toggle feature are not taken as dependencies. · guard: cargo deny check
decision/instrument-requires-skip-all · Instrument attribute must skip all arguments — Every #[instrument] in the workspace takes skip_all as its first argument; a bare #[instrument] fails the build. · guard: gate G1
decision/no-raw-tracing-macros · No raw tracing macros outside the obs module — Raw tracing info, warn, error, debug and trace macros and raw event! are refused outside viola_core::obs. · guard: clippy -D warnings disallowed-macros; raw-event! grep and ban probes in scripts/lint-probes.sh
decision/panic-unwind-required · Panic strategy is unwind — Every Cargo profile keeps panic = "unwind"; no abort strategy in profiles, rustflags, env or workflows. · why: abort turns every catch_unwind into an abort, so hook would die by SIGABRT instead of exiting 0 · guard: gate G3
decision/sync-crates-without-tokio · Sync crates build without Tokio — The synchronous crates must compile without Tokio. · guard: cargo check of the sync crates without Tokio
decision/os-gated-test-census · OS-gated test census — Every OS-gated test function or file must be named in the census with a non-empty reason, and a gate predicate naming windows or macos is a fault even when named. · guard: contract_os_gated_tests
decision/schema-conformance-gate · Schema conformance gate on real process output — Every role-file line of test homes must validate against schemas/diag-line.v1.json and every detail line against schemas/diag-detail.v1.json; a failing line fails the build, a torn non-JSON line is skipped and counted. · why: schema failures count as a build failure and unit tests check only in-memory lines · guard: gate G4 (schema-conformance step)
decision/zero-panics-gate · Zero panics gate — No panic line may appear in the home-level role files of test runs; detail files repeat the panic line and are excluded. · why: ratified · guard: gate G2 (scripts/g2-zero-panics.sh, --probe first) · except: a panic whose panic_location is exactly src/cmd/hook/seam.rs:<digits>, the fake-agent seam's forced panic, compared as a whole string

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/cargo-deny-failure-fails · cargo deny failure is a failure — A cargo deny failure (Tokio, a C crate, veil toggle, banned obs crates) counts as an SLO invariant failure. · guard: cargo deny
decision/deterministic-emission · Deterministic event emission — Events key on hook events and events.ndjson offsets; nothing is derived from screen content. · except: The readiness gate reads the vt100 screen before a paste and can refuse a send as input-not-ready; a local command's echo confirmation keys on its transcript's stdout tag count.
decision/diagnostics-files-mode-0600 · Diagnostics files are 0600 — A diagnostics file whose mode is not 0600 counts as an SLO invariant failure.
decision/g3-panic-abort-gate · G3 panic-abort gate — A G3 hit on panic = "abort" counts as an SLO invariant failure. · guard: G3
decision/print-and-macro-lint-failure-fails · Print and macro lint failure is a failure — A clippy disallowed-macros, print_stdout, print_stderr or dbg_macro failure counts as an SLO invariant failure.
decision/schema-conformance-failure-fails · Schema-conformance failure is a failure — An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.
decision/zero-unlogged-panics · Zero unlogged panics — Every panic in run, hook, mcp and cli after init step 4 produces exactly one single-line ERROR panic record in its home-level role file, with no multi-line backtrace there, and hook still exits 0; retry-once policies for panic handling are forbidden. · guard: G2 · except: the bounded exemptions where no file exists, so no line can be written; G2 alone leaves out the fake-agent seam's deliberate panic from its count.
decision/build-fails-on-any-panic · Build fails on any panic line — The build fails on any panic event line in the kept homes' role files, bar the fake-agent seam's forced panic; no gate infers a panic from an exit code.
decision/correlation-propagation · Correlation propagation — Every channel call logs corr and conn on both sides; a send line from send-issued on carries the send cursor, a wrapper refusal before it the events.ndjson end offset as corr, and a client-side refusal no corr; every dialog line carries dialog_id.
decision/error-budget-zero-tolerance · Zero-tolerance error budget — The error budget tolerates zero panic lines in any CI artifact, zero secret-scan hits and zero schema-conformance failures. · except: the fake-agent seam's deliberate panic, as the zero-panics gate counts it
decision/lint-probes-failure-fails · Lint-probes failure is a failure — A lint-probes failure (a ban that did not fire, a control that did not pass, or a raw event! outside the core obs module) counts as an SLO invariant failure.
decision/module-boundary-error-logging · Errors logged once at module boundaries — Every typed error that crosses a crate seam is logged once at WARN or ERROR with a closed detail or error_code; the full chain goes only to the instance detail file.
decision/no-retry-policies · Retry policies forbidden — Retry-once policies are forbidden, for tests and for panics alike. · why: retry policies mask real signals

## slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin 10-slo-invariants-telemetry-budgets
decision/panic-count-home-level-only · Panic count reads home-level files only — The exactly-one panic rule and gate G2 count home-level role files only; the companion panic line in a detail file is excluded. · guard: gate G2 · except: G2 alone leaves out the fake-agent seam's deliberate panic, whose line is still written exactly once.
decision/no-retry-on-panic · No retry on panic — Retry-once policies for panic handling are forbidden. · why: Parallel to the tests' retries = 0.

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/allow-list-not-widenable · Allow-list and redaction not configurable — config.json, a VIOLA_* env var or a CLI flag never widens the field allow-list or disables redaction.
decision/backtraces-in-instance-detail-only · Backtraces in the instance detail file only — Backtraces are written as one-line JSON arrays in the instance detail file only.
decision/closed-event-and-corr-on-every-line · Closed event enum on every line, corr where correlated — Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
decision/default-panic-hook-never-runs · Default panic hook never runs — The default panic hook is never called or allowed to run. · why: Its stderr output breaks hook and the run terminal.
decision/event-values-closed · event values closed — No event value is added without a Decisions Log entry.
decision/explicit-subscriber-writer · Explicit tracing-subscriber writer — with_writer is always set explicitly on the tracing-subscriber. · why: The default tracing-subscriber writer is stdout.
decision/field-allow-list-default-deny · Field allow-list is default-deny — No field is logged unless named in the field catalog.
decision/file-sink-only-error-sink · File sink is the only error sink — Local logs are never optional: the file sink is the only error sink and works with every other feature off.
decision/log-internal-errors-off · log_internal_errors turned off — log_internal_errors is never left at its default true. · why: Writer failures would eprintln to stderr and break the Hook contract and the run terminal.
decision/low-cardinality-span-names · Low-cardinality span names — Span names never contain instance names, dialog ids or paths.
decision/no-bare-instrument · No bare #[instrument] — Instrumentation always uses #[instrument(skip_all, fields(...))], never a bare #[instrument]. · why: A bare #[instrument] records every argument with Debug and leaks send text, hook.dialog payloads and tool input.
decision/no-env-log-level · Log level not read from the environment — The level is never read from RUST_LOG, the env-filter feature is never enabled and EnvFilter::from_default_env is never called. · why: Env vars are not a configuration channel.
decision/no-hot-path-instrumentation · No hot-path instrumentation — The PTY pump read loop and the vt100 feed get no per-byte or per-chunk span or event; only gate outcomes are logged.
decision/no-info-in-hot-loops · No info logging in hot loops — Nothing is logged at info inside hot loops such as the PTY pump; counts go on the boundary event instead.
decision/no-non-blocking-appender · No non_blocking appender — tracing-appender non_blocking is never used. · why: It drops lines when full, needs a WorkerGuard flush, and breaks the hook's exit-0 / max < 1.0 s gate and the zero-unlogged-panics invariant.
decision/no-paths-or-pids-in-detail-codes · No paths or pids in detail codes — detail codes never contain paths or pids.
decision/no-pii-in-span-fields · No PII or content in span fields — Span fields never include PII or content. · why: Scrubbing at an exporter stage is too late, and v1 has no exporter stage at all.
decision/no-print-macros · No print macros in product code paths — print!, println!, eprint!, eprintln! and dbg! are never used in the hook, run or mcp code paths of the product crates, and every product member inherits the workspace lints. · why: They bypass the file sink and break the Hook contract, the run terminal rule and MCP framing. · guard: clippy print_stdout, print_stderr and dbg_macro at deny · except: viola-harness, the fake agent and the output modules that own --json, human CLI stderr and the hook decision body.
decision/no-stringified-corr · corr never recorded with ? or % — corr is never recorded with ?corr or %corr. · why: Those stringify numbers.
decision/no-telemetry-from-screen-content · No telemetry from screen content — Telemetry is never derived from screen content; vt100 is used for readiness and modal gate outcomes only.
decision/no-telemetry-on-stdout · No telemetry on stdout — Telemetry is never written to stdout on any role. · why: stdout carries each role's own output and run's terminal carries the child's screen.
decision/obs-event-macro-only · obs_event! is the only event macro — tracing's event and level macros are never called directly outside viola_core::obs; events go through obs_event!. · why: Only obs_event! guarantees event, process and instance. · guard: clippy disallowed-macros; the raw-event! grep in lint-probes
decision/one-line-panics · Panics are one line — Multi-line stack traces are never written; panics are logged as one line.
decision/paths-only-in-instance-detail-files · Paths only in instance detail files — Paths appear only in instance detail files.
decision/sensitive-content-detail-file-only · Sensitive content only in the instance detail file — Send text, tool input and last_assistant_message never reach a home-level file; they may reach only the instance-scoped detail-<process>.ndjson, inside a chain or drift report.
decision/service-identity-from-constants · Service identity from compiled constants — Service name and version come from viola-core constants, never from env vars such as OTEL_SERVICE_NAME or OTEL_RESOURCE_ATTRIBUTES; any future OTel Resource starts from an empty builder.
decision/transcripts-not-a-contract · Claude Code transcripts are not a contract — Claude Code transcripts are never read as a contract. · except: A local command's stdout tag in the session transcript, counted past the end recorded before the paste, is read as that send's echo confirmation; no transcript byte is kept.
decision/corr-set-explicitly-by-caller · corr set explicitly by the caller — The caller sets corr explicitly as a typed field on the events that carry one.
decision/explicit-corr-conn-instance · corr, conn and instance passed explicitly — corr, conn and instance are passed as explicit values across threads or tasks, never carried by span context. · why: Spans are not serialized.
decision/machine-asserted-telemetry-verdicts · Machine-asserted telemetry verdicts — Log analysis is never human-review-gated; every CI telemetry verdict is a jq or hyperfine assertion that sets an exit code.
decision/must-trace-paths-always-traced · Must-trace paths always traced — Spans and events are never skipped on the 6 must-trace paths. · why: The agent loses its ability to debug without them.
decision/no-dashboards-only-obs · No dashboards-only observability — Every signal is consumed through agent-run logs / status and jq, never through dashboards as the sole surface.
decision/no-instrument-err-ret-on-sensitive · No instrument err or ret on sensitive results — #[instrument(err)] and #[instrument(ret)] are never used where the error or return can carry a serde or serde_path_to_error source or user content.
decision/no-log-bridge · No log bridge — tracing_log::LogTracer or any log bridge is never installed by hand, and tracing-log stays off. · why: Bridged records lack event, corr, process and instance.
decision/no-retry-once · No retry-once policies — No retry-once policy is added for panics or flaky telemetry assertions.
decision/no-serde-source-in-error-output · No serde source in printed errors — An anyhow chain that still contains a serde_json or serde_path_to_error source is never printed to stderr, --json, MCP isError or channel error.data.
decision/verbosity-changes-volume-only · Verbosity changes volume only — diagnostics_level changes the volume of logging only.
contract/stdout-by-role/cli · stdout by role · cli — stdout carries the --json results.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/content-only-in-instance-detail-file · Content only in instance-scoped detail files — Content-bearing detail goes only to the instance's detail-<process>.ndjson file; home-level log files are codes-only and carry no user content, token or CLAUDE* value. · why: security allows user-derived content only in instance-scoped diagnostics
decision/diag-schema-matches-event-enum · Diagnostic line schema matches the event enum — A hand-maintained JSON Schema of the process-log line is committed, and its event enum must equal the ObsEvent variants, asserted by a tests-owned check.
decision/direct-chrono-dependency · Direct chrono dependency for timestamps — The root binary takes chrono directly, without default features, for the millisecond UTC timestamp, and keeps it though unmaintained: RUSTSEC-2026-0543 stands in the cargo deny ignore list on the unmaintained-upstream ground, re-read at each epoch boundary; jiff and time are not adopted. · why: tracing-subscriber's chrono feature does not re-export chrono
decision/fixed-error-display · Fixed error Display text — Typed errors have fixed Display text that carries no interpolated content.
decision/instrument-skip-all-everywhere · Spans skip all arguments — Every instrumented function uses skip_all with explicit fields, so no argument is recorded implicitly. · guard: G1
decision/no-opentelemetry-in-v1 · No OpenTelemetry in v1 — No opentelemetry crate is added in v1, and cargo deny bans opentelemetry-otlp, opentelemetry-stdout, sentry and tracing-appender on the workspace. · why: so they cannot arrive transitively without a Decisions Log entry · guard: cargo deny bans
decision/obs-event-single-macro · One obs_event macro for every line — Every log line is emitted through the one obs_event macro defined in viola-core, which expands to tracing's event at the caller, so viola-core stays I/O-free and tracing-free.
decision/raw-tracing-ban · Raw tracing macros banned — The tracing level macros are banned by clippy disallowed-macros and raw event calls by a fail-closed grep outside the obs module, so only obs_event emits lines. · why: clippy reports a disallowed macro expanded inside an exported macro at the caller, so no allow inside obs_event can exempt it · guard: clippy disallowed-macros and scripts/lint-probes.sh
decision/single-service-identity-source · One source for service name and version — The core crate's service name and version constants are the single source for process-start version, sender and writer, with the workspace version shared by every member.
decision/subscriber-only-in-root-bin · tracing-subscriber only in the root binary — tracing-subscriber is a product dependency of the root binary only, without its chrono feature; the product crates depend on tracing alone. · except: a library crate may take it as a dev-dependency for in-memory line capture in its own unit tests
decision/veil-redact-payload-types · veil redacts payload types — Payload types derive veil's Redact, and cargo deny bans veil's toggle feature. · guard: cargo deny veil toggle ban · except: A payload type with no content field derives neither veil Redact nor Debug, because veil 0.3.0 refuses a Redact derive with no redact field.

## log-file-location · key of ## 3. Observability Harness Contract
decision/log-files-owner-only · Owner-only log files — Log files are created 0600 inside 0700 directories. · guard: secret-scan test
decision/tracing-appender-rejected · tracing-appender rejected — tracing-appender is not used as a log writer. · why: its non_blocking writer drops lines, and it pulls in an advisory-affected time crate
contract/diagnostics-files/home-resolution · diagnostics files · home resolution — The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
contract/diagnostics-files/paths · diagnostics files · process log paths — Process logs are written to <home>/diagnostics/ as run-<name>, hook-<name>, mcp and cli-<name> .ndjson files, and detail to <home>/instances/<name>/diagnostics/detail-<process>.ndjson.
decision/no-log-rotation-in-v1 · No log rotation in v1 — Diagnostics are not rotated and no rotation library is used; retention is the lifetime of the session home.

## log-format-json-schema · key of ## 3. Observability Harness Contract
decision/no-multiline-stack-traces · No multi-line stack traces — Every log line is one physical line parseable by jq; a hook panic is caught and logged as one ERROR line with event panic.
contract/corr-by-event/process-and-panic · corr by event · process-start, process-exit, panic — corr is null.
contract/process-log-line/process · the process log line · process — One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing cli-<name>.ndjson.
decision/null-as-key-absence · Null written as key absence — A null corr or instance is written as an absent key, readers treat absence and null as equal, and the schema rejects a literal null. · why: tracing has no null field value
contract/corr-by-event/parse-rejected · corr by event · parse-rejected — corr is null.
contract/corr-by-event/send · corr by event · send-issued, send-confirmed, send-refused — corr is the send cursor; a wrapper refusal before send-issued carries the target's event-stream end offset, never returned to the caller; wrapper send lines carry conn and rpc_id; the client-side send-refused has corr null.
contract/process-log-line/instance · the process log line · instance — The instance's name, or absent when none resolved.
contract/process-log-line/timestamp · the process log line · timestamp — RFC 3339 UTC with milliseconds and Z.

## logging-stack · key of ## 3. Observability Harness Contract
decision/content-records-bypass-subscriber · Content-bearing records bypass the subscriber — Content-bearing records never pass through the tracing subscriber, whose one writer holds only the home-level role file; the detail writer formats each detail line itself and writes it with one whole write.
decision/explicit-log-writer · Explicit diagnostics writer — The diagnostics subscriber always has its writer set explicitly. · why: The default writer is stdout.
decision/file-only-diagnostics-sinks · File-only diagnostics, one file per role — Every role logs only to its own file under the home's diagnostics directory; no role has a stdout sink, and there is no pretty or dev sink.
decision/log-level-never-from-env · Log level never from the environment — The diagnostics level is never read from RUST_LOG and the env-filter feature is never enabled; the level comes only from config.json.
decision/no-agent-mode-flag · No agent-mode flag — There is no agent-mode flag; JSON to file is the only logging mode.
decision/no-ansi-in-logs · No ANSI codes in diagnostics lines — Diagnostics lines carry no ANSI colour codes.
decision/no-internal-error-fallback · No stderr fallback for subscriber internal errors — The diagnostics subscriber never reports its own internal errors by printing to stderr. · why: The default falls back to eprintln, which would break the Hook contract and the run terminal rule.
decision/third-party-targets-off · Third-party log targets off — Records of third-party targets (rmcp, portable_pty, sysinfo, interprocess) are off in every sink; their failures are captured at viola's seam from the returned results. · why: Their records lack the required fields.
contract/agent-run-logs/torn-line · agent-run logs output · torn line — A torn line appears as {"torn":true,"offset":n}, and readers skip it rather than fail.
contract/config-json/diagnostics-level · config.json · diagnostics_level — Key diagnostics_level takes "info" (the default) or "debug"; a missing or unreadable file or an unknown value falls back to "info".
contract/diagnostics-line/role-file-line-size · the diagnostics line · role-file line size — A line in a role file (run, hook, mcp, cli) is codes-only and stays well under 4 KiB.
contract/obs-event-macro/attached-fields · the obs_event macro · attached fields — Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.
decision/level-changes-volume-only · Level changes volume only — Every diagnostics level uses the same field allow-list, so the level changes only how many lines are written. · why: No setting can disable redaction.
decision/no-content-in-home-level-lines · No content fields in home-level lines — No home-level diagnostics line carries chain, drift_report, panic_payload or backtrace. · guard: G4 diag-line.v1.json schema
decision/no-log-crate-bridge · No log-crate bridge — The log-crate bridge (tracing-log, LogTracer) stays off; failures are captured from returned values or handles, and a gap is fixed at the seam or by the researched json-subscriber fallback, never by enabling the bridge. · why: The accepted loss is only third-party log context emitted without a returned error. · guard: pty_exit_is_read_on_the_handle_while_the_output_is_held
decision/no-source-paths-in-logs · No source paths in diagnostics lines — Diagnostics lines carry no source file names or line numbers.
decision/no-span-list-in-lines · No span list in diagnostics lines — Diagnostics lines never carry the list of entered spans. · why: Closes the spans leak path.
decision/shared-log-files-concurrent-append · Concurrent appends to shared diagnostics files — Every diagnostics file may have concurrent writers in separate processes; each line is one whole write on an append-mode handle, readers tolerate a torn line, and a concurrent-append check, including a detail line over 4 KiB, asserts every line parses. · why: Non-interleaving at this size is an assumption, not a POSIX guarantee, because the 4 KiB PIPE_BUF rule applies to pipes. · guard: concurrent-append check; hook_panics_append_whole_lines_over_4_kib_side_by_side
decision/tracing-logging-library · tracing as the logging library — tracing is the logging facade and tracing-subscriber's JSON formatter writes the lines. · why: Tokio-free and no C, and meets the security floor for tracing-subscriber
contract/detail-line/fields · the detail line · fields — Each detail line carries timestamp (millisecond UTC format), level, target, message, event, process and instance, then its content fields, and no corr or conn key.
contract/diagnostics-files/detail · the diagnostics files · detail — Content-bearing records (full error chains, drift reports, panic payload and backtrace) go only to <home>/instances/<name>/diagnostics/detail-<process>.ndjson, file 0600 in a 0700 directory, and only when an instance resolves.
contract/diagnostics-line/format · the diagnostics line · format — Each diagnostics line is one structured JSON object on one line, written whole by one write of the complete line on an append-mode handle.

## otel-sdk-init · key of ## 3. Observability Harness Contract
decision/diagnostics-init-before-role-work · Diagnostics init before the role's work — Diagnostics init and the process-start line come before the role's work: in run before its collision check and the child spawn, in mcp before its stdio transport starts. · why: Exit-1 causes are logged.
decision/failed-log-open-discards · Failed log open discards lines — When the role file cannot be opened, viola_obs_init returns the error and installs no subscriber, so no line is written; hook carries on and the other roles, mcp included, return the error.
decision/flat-log-lines-and-one-level-gate · Log lines are flat and Targets is the only level gate — The subscriber flattens event fields to the top level of each line, and the Targets filter is the only configurable level gate; its level comes from the home's diagnostics level and never from the environment. · why: The diagnostics schema and the log readers read event fields as top-level keys, and only the viola crates' targets pass the gate.
decision/home-resolution-order · Home resolution order — The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
decision/home-tree-check-before-log-init · Home tree check before log init — run, revive's start arm, adopt's start and viola verify check the home tree's modes before their log opens; a refusal writes no line, starts no child, leaves modes as found and exits 1.
decision/instance-source-per-role · Instance source fixed per role — run takes its own name argument, never an inherited VIOLA_NAME; hook and mcp take VIOLA_NAME only; cli verbs that act on an instance (send, wait, last, answer, pause, release, link, unlink) take the target argument; list, verify, plugin install and revive --list take the caller's VIOLA_NAME only. · why: A wrapper never writes into another instance's run log.
decision/lazy-detail-file · Detail file opened lazily — The instance detail file is opened only by the first content-bearing record or by the panic hook, and a failed detail open drops only the detail line, writing nothing to stderr. · why: The error-free hook path keeps exactly one log open.
decision/millisecond-utc-timestamps · Millisecond UTC timestamps — Every diagnostics timestamp is RFC 3339 in UTC with millisecond precision and a Z suffix. · why: The default timer writes microseconds, and the chrono RFC 3339 timer writes +00:00 with automatic precision.
decision/panic-hook-first · Panic hook installed first — The panic hook is installed as the first statement of main and writes only to the diagnostics file once that is open, never falling back to stderr.
decision/private-diagnostics-files · Owner-only diagnostics files — The diagnostics directory is 0700 and each diagnostics file 0600, opened without following symlinks; before the first write each must be owned by the effective user, have no group or other bits and not be a symlink, a failed check counting as a failed open. · why: A pre-existing foreign, permissive or symlinked diagnostics file is never written.
decision/strict-modes-before-diagnostics · Home strict-modes check before diagnostics — Nothing under the home, the diagnostics directory included, is created or opened before the role's strict-modes check on the home passes; a refused home gets no diagnostics line. · except: viola up's home-tree check before its first spawn runs after its board scan has opened its log, and a refusal there ends that log with process-exit detail strict-modes-failed.
decision/tracing-is-the-instrumentation-facade · tracing as the instrumentation facade — tracing is the instrumentation facade in every instrumentable crate, and tracing-subscriber is the formatter and sink, a product dependency of the root bin only.
decision/tracing-subscriber-minimal-features · Minimal tracing-subscriber features — tracing-subscriber is taken with default features off and only fmt, json, registry and std; ansi, tracing-log and chrono stay off. · why: The chrono feature only enables ChronoUtc / ChronoLocal, and the millisecond timer uses the root bin's direct chrono dependency.
decision/instance-check-before-log-init · Instance check before log init — Commands acting on an instance (revive's start arm, adopt's start, the hook arms, link, unlink, send, wait, last, answer, pause, release, focus, switch) run the instance strict-modes check before opening any log; a refusal opens and writes nothing and leaves modes as found. · why: Opening the role file narrows an existing home to 0700, so a check made after it passes a home at 0770. · except: The hook --capture arm runs ahead of the check.
contract/cli-exit-codes/21 · CLI exit codes · 21 — An instance strict-modes refusal exits 21 with the fixed instance-state pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
contract/config-json/v · config.json · v — A document that is no object, or whose v is not 1, gives the hold default and no further line.
contract/diagnostics-events/config-parse-rejected · diagnostics events · parse-rejected config-json — Right after process-start, a config.json parse that failed or skipped keys logs parse-rejected{parser:"config-json", detail, count}.

## service-identity · key of ## 3. Observability Harness Contract
decision/env-vars-not-configuration · Environment variables are not configuration — Environment variables are not a configuration channel: service identity has no env-var source or fallback, and the one CLAUDE_CONFIG_DIR value read by verify and the run, revive and adopt start, when kept, locates the claude config file, is never logged and configures nothing of viola.
decision/single-version-constant · One version constant for every crate — One compile-time version constant taken from the package version serves every crate, and every crate takes the workspace version. · why: The log version, channel sender and snapshot writer cannot drift apart.
contract/diagnostics-line/process · the diagnostics line · process — Every diagnostics line carries the role as field process.
contract/process-start/os · the process-start line · os — process-start carries field os, the one word linux.
contract/process-start/pid · the process-start line · pid — process-start carries the process id as field pid.
contract/process-start/service-name · the process-start line · service_name — process-start carries the service name as field service_name.
contract/process-start/version · the process-start line · version — process-start carries the product version as field version.
contract/diagnostics-line/instance · the diagnostics line · instance — Every diagnostics line of a process with an instance carries it as field instance; a process without one, such as an mcp server with no instance name, writes no instance key.

## snapshot-paste-to-ai-integration · key of ## 3. Observability Harness Contract
contract/agent-run-logs/diag-record · agent-run logs output · diag record — Each line of every diagnostics/*.ndjson is merged as {"src":"diag","file","record"}.
contract/agent-run-logs/filters · agent-run logs output · filters — The merged output is filterable by --instance, --kind and --process.
contract/agent-run-logs/detail-record · agent-run logs output · detail record — Every instance detail line from instances/*/diagnostics/detail-*.ndjson is merged as {"src":"diag","file","instance","record"}, the instance telling apart the same detail file name across instances.
contract/send-line/client-side-refusal · the send line · client-side send-refused — The caller-side send-refused{side:"client"} has corr null.
contract/send-line/cursor-uniqueness · the send line · cursor corr uniqueness — A send cursor is a byte offset into the target's own events.ndjson, unique per target only; refusals and later sends can share it, so one send is identified by target instance, corr, conn and rpc_id.
contract/send-line/unconfirmable · the send line · unconfirmable send — An unconfirmable send logs one send-issued and one send-confirmed{confirmed:false} under the same corr, with no send-refused and no hook side.
contract/send-line/wrapper-side · the send line · wrapper side — send-* lines whose corr is a send cursor are emitted only by the target wrapper; only its send-refused line carries side:"wrapper", its send-issued and send-confirmed lines carry no side, and a client-side send-refused carries side:"client" and no corr.

## trace-context-propagation · key of ## 3. Observability Harness Contract
decision/span-carried-into-tasks-and-threads · Span not carried into tasks and threads — Spawned async tasks and std thread closures take no span from their spawner: spans never reach a log line, and correlation travels as explicit typed fields.
contract/diagnostics-line/corr-on-send-lines · the diagnostics line · corr on send lines — On send lines, corr carries the send cursor; a send refused before send-issued carries the target's events.ndjson end offset at refusal, a log join key only, never returned to the caller.

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
