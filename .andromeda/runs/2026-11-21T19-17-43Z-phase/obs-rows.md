# obs-plan · the rows that bind 2026-11-21-mcp-calls-bounded — picked 379 of 1034 (213 by every draw) · rule any · pin 3707

## obs-plan-viola · (above the first heading)
decision/agent-readable-telemetry · Agent-readable telemetry only — Every telemetry signal must be readable by an agent from a snapshot or pasted log excerpt: no human-only dashboards, no proprietary APM lock-in, no human-review-gated log analysis. · except: A surface that cannot be agent-readable is flagged not-instrumentable and handled through boundary instrumentation only.

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/assistant-message-never-logged · Assistant message never logged on readback — On `wait` / `last` readback, the `last_assistant_message` content never appears in log fields.
decision/channel-lines-carry-no-bodies · Channel log lines carry no bodies — IPC channel log lines never carry `params` or result bodies.
decision/claude-internals-not-a-contract · Wrapped claude internals are not a contract — The wrapped claude CLI is a third-party binary run unmodified and instrumented at its boundary only (spawn and exit, paste writes, the vt100 readiness-gate outcome); its internal state and transcripts are never read as a contract. · except: A send's echo confirmation counts its stdout tag in the transcript, keeping no byte; verify writes and removes its temp folder's trust record; a verified start sets its folder's trust flag only.
decision/corr-field-propagation · Propagation through the corr field — Correlation across the viola-channel is carried by the corr field in log lines of the process on each side. · why: Channel params declare no trace-context field.
decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits.
decision/dialog-text-never-logged · Dialog text never logged on answer — On the dialog to `answer` path, the question, answer and tool `input` text never appear in log fields.
decision/drift-reports-diagnostics-only · Drift reports go to diagnostics only — Drift reports of viola-agent-claude go to diagnostics/ only and nowhere else. · why: The outputs of viola-agent-claude are the most sensitive in the product.
decision/mcp-log-file-sink · MCP logs to a file sink, never stdout — The `mcp` process always logs to the file sink `<home>/diagnostics/mcp.ndjson`, with no stderr fallback: a file that cannot be opened exits 1 printing nothing, and the log writer is set explicitly. · why: stdout is forbidden because it carries MCP frames
decision/mcp-rejections-codes-only · MCP rejections carry codes only — MCP isError results and rejected tool calls carry codes only (refusal, detail, instance-unreachable, wrapper-fault, invalid-params), with no upstream text, absolute paths or type names.
decision/mcp-stdout-frames-only · MCP stdout carries frames only — No telemetry from the viola-mcp server may go to stdout. · why: Stdout is reserved for MCP frames.
decision/no-traceparent-on-local-socket · No W3C traceparent on the local socket — Calls from CLI, MCP and hook to the wrapper carry no W3C `traceparent`; correlation uses `corr` (JSON-RPC `id`, `dialog_id`, send `cursor`) plus `process` and `instance`. · why: channel `params` declare no trace-context field
decision/one-log-file-per-role · One log file per process role — Each process role writes its own file under `diagnostics/`, no role has a stdout sink, and no role falls back to stderr JSON when its file cannot be opened.
decision/pty-byte-stream-never-logged · PTY byte stream never logged — The PTY byte stream is never logged; the viola-pty seam is instrumentable, and the openpty OS API behind it is boundary-only. · why: The PTY byte stream is user content.
decision/rmcp-stdio-mcp-server · rmcp stdio MCP server — The viola-mcp crate is an rmcp stdio MCP server.
decision/run-sync-instrumentation · Sync instrumentation in viola run — Instrumentation in viola run is sync, with no Tokio.
decision/send-text-confined · Send text confined to instance diagnostics — `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
decision/send-text-never-logged · Send text never logged — Sent text never appears on stderr, in --json output, in MCP results or in any process-log field outside instances/<name>/diagnostics/.
decision/typed-keys-never-logged · Typed keys never logged — Keys a human types never appear in a log line; the wheel product event in events.ndjson is the audit record.
contract/diagnostics-files/mcp · the diagnostics files · mcp — Role mcp writes its role file mcp.ndjson in the home's diagnostics folder, which never carries user content; a role file that cannot be opened ends the server with exit 1 and nothing on stderr.
contract/viola-mcp-tools/answer · viola-mcp stdio server tools · answer — The viola-mcp stdio server offers an answer tool.
contract/viola-mcp-tools/last · viola-mcp stdio server tools · last — The viola-mcp stdio server offers a last tool.
contract/viola-mcp-tools/list · viola-mcp stdio server tools · list — The viola-mcp stdio server offers a list tool.
contract/viola-mcp-tools/send · viola-mcp stdio server tools · send — The viola-mcp stdio server offers a send tool.
contract/viola-mcp-tools/wait · viola-mcp stdio server tools · wait — The viola-mcp stdio server offers a wait tool.
decision/absolute-paths-scrubbed-externally · Absolute paths scrubbed from external output — Absolute paths in snapshot.json (endpoint, pinned_bin) and in config values are scrubbed from external output and allowed in diagnostics/.
decision/channel-json-rpc-over-ndjson · Channel is JSON-RPC 2.0 over ndjson — The viola-channel crate speaks JSON-RPC 2.0 over ndjson on local Unix sockets, one endpoint per viola run, with a sync client and server and a Tokio client behind the tokio feature.
decision/channel-sync-instrumentation · Sync instrumentation in viola-channel — Instrumentation in viola-channel must be sync, because Tokio is forbidden in that crate's graph. · except: The tokio feature, when enabled.
decision/event-payload-not-in-process-logs · Event payloads kept out of process logs — The payload content of events.ndjson is never copied into process-log lines. · why: events.ndjson is itself the product audit stream.
decision/mcp-stdio-server-tokio-allowed · MCP server on stdio with Tokio allowed — The `viola mcp` surface is an internal stdio MCP server, and Tokio is allowed in it.
decision/run-silent-terminal · viola run silent on the terminal — viola run writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns.
decision/single-file-sink-no-os-exporter · One file sink and one line schema — Telemetry on Linux goes to one file sink with one line schema, with no OS-specific exporter.
decision/spans-not-wire-contract · Tracing spans are not part of the wire contract — Span names are internal `tracing` spans in `<area>.<operation>` snake_case form; the agent-facing contract is the closed `event` enum plus `corr`. · why: the config sets `with_current_span(false)`, so spans do not reach the line
decision/strict-modes-failure-lines · Where a strict-modes failure is logged — A strict-modes failure has a log line only on the `ledger-stamps` read in the `run` version gate and on `verify`'s own `process-exit` for a refused stamp write.
decision/stripped-claude-env-never-logged · R8-stripped CLAUDE values never logged — R8-stripped CLAUDE* environment values are never logged anywhere. · why: The viola-agent-claude outputs are the most sensitive in the product.
decision/stripped-env-never-logged · Stripped CLAUDE env values never logged — No R8-stripped `CLAUDE*` value appears in any sink or fixture; the strip is logged as a count or variable names only.
decision/tracing-json-line-file-pipeline · tracing JSON-per-line file pipeline — Telemetry is tracing plus tracing-subscriber writing JSON-per-line files under diagnostics/, correlated by corr and consumed through agent-run logs and status. · why: The pipeline is the one the tests plan binds.
decision/tracing-json-pipeline · tracing with the tracing-subscriber JSON formatter — The `tracing` facade plus the tracing-subscriber JSON formatter is the binding logging pipeline, and JSON-to-file is the only logging mode.
contract/diag-line/instance · diagnostic log line · instance — The instance's `ViolaName`; the key is absent when the process has no instance.
contract/diag-line/process · diagnostic log line · process — The role of the writing process, one of run, hook, mcp, cli; required on every line and matching `sender` / `writer`.
contract/diag-line/service-name · diagnostic log line · service.name — Hardcoded to "viola".
contract/diag-line/service-version · diagnostic log line · service.version — The crate version, fixed at compile time.
contract/diagnostics-files/run · the diagnostics files · run — Role run writes <home>/diagnostics/run-<name>.ndjson.
contract/log-line/process · every log line · process — Every log line carries a process key naming its role: one of run, hook, mcp, cli.
contract/process-exit-detail/strict-modes-failed · process-exit detail codes · strict-modes-failed — Carried by `verify`'s `process-exit` for a refused stamp write, with exit 1.
contract/run-log-file/path · the viola run log file · path — viola run logs only to a file sink at diagnostics/run-<name>.ndjson.

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/channel-log-scrub-rule · Channel log scrub rule — Channel request and response lines log only the method, id or corr, the result class (ok, refusal or error code), the sender version and the closed detail word, never params or result bodies.
decision/claude-boundary-only-instrumentation · Boundary-only instrumentation of claude — The hosted claude binary is instrumented only at its boundary: spawn and exit, paste writes, the vt100 readiness-gate outcome, and the start's one trust-record line for the trust record it writes in the claude config. · why: claude is a third-party binary run unmodified.
decision/correlation-in-corr-field · Correlation in the corr log field — Correlation across a channel call travels in the corr log field; channel params declare no trace-context field.
decision/detail-codes-no-paths · Detail codes carry no path or pid — No path or pid appears in a detail code.
decision/drift-reports-detail-only · Drift reports in the detail file only — A full serde drift report goes only to the instance-scoped detail file, as its drift_report field.
decision/fault-telemetry-per-class · Fault telemetry per class — Each fault class emits its own telemetry: a healed torn ndjson line with file and offset, a corrupt or unsupported snapshot recovered by replay, a child exit seen without EOF, an endpoint vanished during wait, and a stale heartbeat against a live pid.
decision/home-level-mcp-log-no-user-content · Home-level MCP log holds no user content — An mcp.ndjson at home level never carries user content.
decision/hook-mcp-instrumented-at-boundary · Hook and MCP instrumented at the boundary — Hook and MCP traffic is instrumented only at viola's own boundary: hook-invoked and hook-decision, MCP tool-call handling, and the viola mcp stdin cut. · why: The caller is Claude Code, so viola sees only the inbound stdin payload and its own response, and no trace context arrives from Claude Code.
decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but a required one the product still emits is never renamed or removed. · why: The harness greps on them.
decision/mcp-log-sink · MCP log sink — viola mcp always logs to <home>/diagnostics/mcp.ndjson once the home and its diagnostics/ pass strict-modes; a file that cannot be opened exits 1 printing nothing, with no stderr fallback.
decision/mcp-no-telemetry-on-stdout · No telemetry on the MCP server's stdout — The MCP server sends no telemetry to stdout. · why: stdout is reserved for MCP frames.
decision/mcp-results-codes-only · MCP rejections carry codes only — MCP isError results and the log lines of rejected calls carry codes only.
decision/no-telemetry-from-screen · No telemetry from screen content — No telemetry is derived from screen content; vt100 serves gate outcomes only.
decision/process-logs-file-sink-only · Process logs to a file sink only — Process logs are written only as JSON lines to one file per process role under diagnostics/; there is no network exporter and no stdout sink. · why: viola makes no outbound network calls, and stdout carries --json results.
decision/send-refused-by-each-deciding-side · Send refusal logged by each deciding side — A send-refused with detail control-character or empty-text is emitted by each side that decides it, client and wrapper, without the text.
decision/subscriber-explicit-writer · Subscriber installed with an explicit writer — The tracing subscriber is always installed with an explicit writer, never stdout. · why: tracing-subscriber fmt() writes to stdout unless a writer is configured, and stdout carries MCP frames.
decision/tests-event-list-adopted · Tests event list adopted — The tests' field and event list is adopted verbatim, with send-issued and send-refused among the logged events.
contract/mcp-invalid-params/detail · MCP invalid-params · detail — The detail of an MCP invalid-params is a closed field word: target, text, after, timeout_ms, dialog_id or response.
contract/mcp-iserror/instance-unreachable · MCP isError result · instance-unreachable — The instance-unreachable detail is one closed cause: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed or socket-dir-refused; the CLI's document less v, its text `refused: instance-unreachable`.
contract/mcp-iserror/invalid-params · MCP isError result · invalid-params — The invalid-params detail is a closed field word.
contract/mcp-iserror/refusal · MCP isError result · refusal — A peer refusal that is no code word reads unknown, and a string detail that is no code word reads null.
contract/mcp-iserror/wrapper-fault · MCP isError result · wrapper-fault — The wrapper-fault detail is a closed sanitised object: the fixed message of the five codes with data null or closed values; a code outside the five reads message "wrapper fault", data null.
contract/process-log-corr/send-confirmed · Process log corr by event · send-confirmed — corr is the send cursor.
contract/process-log-corr/send-refused · Process log corr by event · send-refused — corr is the send cursor.
contract/process-log/corr · Process log line · corr — Copied unchanged as a JSON number or string, never renamed.
contract/process-log/event · Process log line · event — A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
contract/process-log/process · Process log line · process — The process role, one of run, hook, mcp, cli.
contract/send-log-line/corr · Send log line · corr — On the wrapper's send lines corr is the send cursor, which joins send-issued, send-confirmed and send-refused (a refusal before send-issued carries the log's end offset); the CLI's and MCP's client-side send-refused lines carry no corr; on channel lines corr is the JSON-RPC id.
contract/send-log-line/detail · Send log line · detail — The send detail is one of input-not-ready, no-prompt-submitted, control-character, empty-text or turn-running.
contract/send-log-line/required-fields · Send log line · required fields — A send line carries event, process, instance, corr, from, confirmed, refusal, detail and duration_ms.
contract/send-path-log-events/event-order · Send path log events · event order — A send logs channel-request, then send-issued, then hook-invoked (UserPromptSubmit, or SessionStart for a confirmed /clear or /compact; none awaited for an unconfirmable send or a local command confirmed by its transcript echo), then send-confirmed or send-refused, then channel-response.
contract/wait-log-line/outcome · Wait log line · outcome — The wait outcome is the woken kind, timed-out or instance-unreachable.
contract/wait-path-log-events/channel-pair · Wait path log events · channel pair — A wait logs channel-request and channel-response, the response carrying the outcome.
contract/wheel-log-line/human-typing · Wheel log line · human-typing — A send refused while a human holds the wheel logs refusal human-typing, detail null or manual-pause, the wheel holder and exit_code 10.
decision/log-file-basenames-follow-tests · Log file names follow tests — Process-log files are <home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson, as tests require. · why: Tests is upstream; arch and security place diagnostics under the instance directory instead.
decision/messaging-secrets-never-logged · Messaging secrets never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every R8-stripped CLAUDE* value are never logged in any sink, detail files and fixtures included. · guard: secret-scan test
decision/one-linux-log-sink · One file sink on Linux — One file sink and one line schema serve Linux, with no OS-specific exporter.
decision/per-parser-parse-failure-counts · Per-parser parse-failure counts — Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, the hyprctl replies, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead.
decision/r8-strip-logged-as-names-only · R8 strip logged as names only — The R8 environment strip is logged as a count and lists of stripped and kept variable names, and environment maps never pass to a span or event; no value is read, bar the kept CLAUDE_CONFIG_DIR value that viola verify and the run and revive start read, and no value is logged.
decision/run-silent-on-terminal · run silent on the terminal — run writes nothing to the terminal while the child runs; its process log goes only to diagnostics/run-<name>.ndjson. · why: The terminal carries the child's screen.
decision/single-line-log-records · Single-line log records — Every log record is one JSON line parseable by jq; never a multi-line stack trace.
decision/stripped-claude-values-never-logged · Stripped CLAUDE values never logged — R8-stripped CLAUDE* values are never logged anywhere. · why: Its outputs are the most sensitive in the product.
decision/structured-json-process-logs · Structured JSON process logs — Every process role (run, hook, mcp, cli) writes structured JSON process logs under diagnostics/, which harness logs merges with events.ndjson.
decision/tracing-json-logging-pipeline · tracing with a JSON subscriber for logging — Logging and spans go through the Rust tracing facade with a tracing-subscriber JSON structured logger.
contract/channel-response/detail · Channel response line · detail — On a channel-response answering -32602, detail is one closed word: unsupported-version, invalid-params or release-from-driver, on both the server's and the client's line; it is absent on every other channel-response line and names no param or value.
contract/channel-response/invalid-params-causes · Channel response line · invalid-params causes — detail invalid-params covers send's invalid params, a release whose from is not a string or that carries any budget key, and a link or unlink whose driver or driven is missing, not a string or no valid name, whose driven is not the serving wrapper's own name, or whose driver equals driven.
contract/cli-frame-verbs/strict-modes-failed · CLI frame verbs · strict-modes-failed — A strict-modes refusal of send, wait, last, answer, pause or release writes no line and prints only the fixed instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, exit 21, with no pid, path or endpoint.
contract/cli-json-error/strict-modes-failed · CLI --json error document · instance-unreachable strict-modes-failed — A refused instance check of send, wait, last, answer, pause, release, link or unlink exits 21 with the instance pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
contract/diag-line-event/release-from-driver · Diag line event · release-from-driver — release-from-driver is its own logged event, admitted by the diag-line schema, not a generic -32602 wrapper fault.
contract/dialog-log-line/corr · Dialog log line · corr — On dialog lines corr is the dialog_id, which joins hook and wrapper lines; the driver's channel lines carry the JSON-RPC id.
contract/json-rpc-error/32603 · JSON-RPC error · -32603 — The -32603 error carries the message "internal error" with data null.
contract/mcp-frame-tool-refusal/no-live-endpoint · MCP frame tool refusal · no live endpoint — A frame tool with no live endpoint returns instance-dead or unwrapped-name and writes no frame.
contract/mcp-frame-tool-refusal/server-verify-failed · MCP frame tool refusal · server-verify-failed — A frame tool whose server check fails or whose server euid is refused returns server-verify-failed and writes no frame.
contract/mcp-frame-tool-refusal/socket-dir-refused · MCP frame tool refusal · socket-dir-refused — A frame tool whose socket directory is refused returns socket-dir-refused, writes no frame and no line.
contract/mcp-frame-tool-refusal/strict-modes-failed · MCP frame tool refusal · strict-modes-failed — A frame tool (send, wait, last, answer) whose instance check refuses returns instance-unreachable with detail strict-modes-failed, writes no frame and no line.
contract/mcp-server/instance · MCP server identity · instance — The server's own VIOLA_NAME when set, which is also the from it adds.
contract/mcp-stdin/line-within-max-frame · viola mcp stdin · line within MAX_FRAME — A line of at most MAX_FRAME bytes, its newline counted, passes whole.
contract/mcp-stdin/oversize-before-initialize · viola mcp stdin · oversize line before initialize — An oversize line before initialize exits the process 1 with both streams empty and the one parse-rejected record.
contract/process-log-corr/channel-request · Process log corr by event · channel-request — corr is the JSON-RPC request id.
contract/process-log-corr/channel-response · Process log corr by event · channel-response — corr is the JSON-RPC request id.
contract/process-log-corr/send-issued · Process log corr by event · send-issued — corr is the send cursor.
contract/process-log/child-and-probe-lifecycle · Process log · child and probe lifecycle — process-start and process-exit are logged for the claude child, for the claude agents --json and --version probes, and for verify's print-mode sender.
contract/process-log/level · Process log line · level — One of DEBUG, INFO, WARN, ERROR.
contract/unreachable-cause/endpoint-lost · exit 21 cause words · endpoint-lost — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/unreachable-cause/instance-dead · exit 21 cause words · instance-dead — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/unreachable-cause/server-verify-failed · exit 21 cause words · server-verify-failed — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/unreachable-cause/socket-dir-refused · exit 21 cause words · socket-dir-refused — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/unreachable-cause/strict-modes-failed · exit 21 cause words · strict-modes-failed — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/unreachable-cause/unwrapped-name · exit 21 cause words · unwrapped-name — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/viola-verify/home-tree-refused · viola verify · refused home-tree check — A refused home-tree check exits 1 with the home pair, opens no log and writes no line; a refused update_stamps exits 1 with the home pair and, when an instance is named, ends the opened log with a process-exit line carrying detail strict-modes-failed.
contract/wait-log-line/exit-code-21 · Wait log line · exit_code 21 — A wait exits 21 when the wrapper is killed mid-wait (detail endpoint-lost, during call), or when a CLI wait meets a refused socket directory before any frame (detail socket-dir-refused, during connect, no channel-request).
contract/wait-log-line/required-fields · Wait log line · required fields — A wait line carries event, process, instance, corr (the JSON-RPC id), after (a byte offset), timeout_ms, outcome, exit_code and duration_ms.
contract/wheel-path-log-events/pause-and-release · Wheel path log events · pause and release — A wheel take-over logs send-refused for the refused send and a channel-request and channel-response pair for pause and for release.

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/corr-conn-correlation · Correlation through corr and conn — Every cross-surface call carries correlation through the corr field plus the additive conn discriminator.
decision/no-event-payloads-in-process-logs · No event payloads in process logs — Obs never copies event payloads into process logs.
contract/process-log-line/enum-values · the process log line · enum values — Enum values are kebab-case.
contract/process-log-line/event · the process log line · event — The event field is a closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
contract/process-log-line/field-names · the process log line · field names — Field names are snake_case.
decision/machine-parseable-signals · Every signal machine-parseable — Every signal produces machine-parseable output: JSON-per-line process logs, the ndjson product event stream, JSON harness documents, hyperfine JSON for perf gates and nextest JUnit XML in CI.
decision/span-naming · Span naming as <area>.<operation> — Span names are <area>.<operation> in snake_case, <area> one of run, pty, channel, state, hook, send, wait, last, answer, pause, release, cli, mcp, hyprland; cli.<verb> names the dispatch span of a short-lived verb.
contract/process-log-line/at-instants · the process log line · *_at — Instants are named *_at and written in RFC 3339 UTC with milliseconds and a Z suffix.
contract/process-log-line/duration-ms · the process log line · duration_ms — Durations are duration_ms as an integer.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/cross-process-join-corr-conn · Cross-process joins by corr and conn — Lines across processes are joined by the corr and conn fields, not by W3C trace IDs.
decision/pre-open-strict-modes-refusal-silent · Strict-modes refusal before a log opens writes no line — A strict-modes refusal made before any process log opens writes no line and carries no logged detail code; its machine-readable form is the exit code and, under --json, its closed document. · except: a refused socket directory always logs detail socket-dir-refused, except at MCP's frame tool, which writes no line
decision/events-stream-never-rotated · Event stream never rotated — The instance's events.ndjson is the product audit stream, never rotated by a generic library nor rewritten: a reused instance name keeps the earlier starts' lines at its head, so a start's own lines are read from where that start begins. · why: byte offsets in it are the wait-after and send cursors
decision/span-naming-skip-all · Span declaration convention — Every span is declared with instrument, skip_all and a name of the form "<area>.<operation>".
decision/structured-logs-primary-signal · Structured logs are the primary signal — tracing-subscriber JSON-per-line logs, one file per process role under the home's diagnostics directory, are the primary observability signal, merged with the event stream by agent-run logs.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/codes-only-process-logs · Codes-only process logs, content in detail files — Every role's codes-only process log goes to its home-level role file under diagnostics, and content-bearing detail goes only to the instance's per-role detail file.
contract/mcp-output/stdout · mcp output streams · stdout — mcp writes only MCP frames to stdout.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/channel-lines-no-params-or-results · Channel lines carry no params or results — Channel log lines never record request params or result bodies.
decision/channel-response-always-logged · channel-response always logged — Every dispatched channel request logs its channel-response line, an early return or an error included. · why: The line is emitted from a Drop guard, so an early return or error still logs it.
decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.
decision/compact-confirm-window · Compact confirmation window — A /compact send waits 40 s for its compacted-session post-condition, while every other confirmed send waits 10 s. · provisional
decision/detail-codes-only · detail is a closed code — The detail field is a closed kebab-case code catalog and never carries a path or pid.
decision/env-names-only · Environment names only, never values — The strip, every log, the hook capture arm's env-names file and the verify stamp and record carry environment variable names only, never values, and no env map is passed to a span or event. · guard: E2
decision/ledger-contents-never-logged · Capability ledger path and stamps never logged — The capability ledger's path and its stamp contents never appear in a log line.
decision/mid-call-disconnect-logged-once · Mid-call disconnect logged once — A disconnect in the middle of a call is logged once, by the client.
decision/no-assistant-message-in-logs · No assistant message or result body in logs — last_assistant_message never appears in a log, and no result body reaches a log line.
decision/no-endpoint-path-or-env-in-spans · No endpoint, binary path or env values in spans — A span never carries the endpoint name, the pinned_bin path or environment values.
decision/sent-text-never-logged · Sent text never logged — A sent message's text is never logged; only its size in text_bytes is.
decision/unreadable-ledger-is-unverified · Unreadable ledger counts as unverified — A missing stamp, or an unreadable, malformed or strict-modes-refused capability ledger, counts as unverified (cli_verified false).
decision/verify-lines-redacted · verify lines carry no content — No verify or plugin install line carries row text, a prompt, a session id or a path.
contract/mcp-wait-last-connect/server-verify-failed · MCP wait / last connect refusal · server-verify-failed — A serving process that is not the recorded wrapper or runs under another euid returns instance-unreachable with detail server-verify-failed and writes no line.
contract/mcp-wait-last-connect/socket-dir-refused · MCP wait / last connect refusal · socket-dir-refused — A refused socket directory returns instance-unreachable with detail socket-dir-refused and writes no line.
contract/mcp-wait-last-connect/strict-modes-failed · MCP wait / last connect refusal · strict-modes-failed — A home or instance tree refused by the instance check returns instance-unreachable with detail strict-modes-failed and writes no line.
decision/dialog-content-never-logged · Dialog content never logged — Dialog questions, answers, plan text and tool input never appear in a home-level process-log line; dialog events carry codes only, and the text reaches an instance detail file only inside a chain or drift report.
decision/no-trace-and-span-ids · No trace_id or span_id — Log lines never emit trace_id or span_id; correlation is by corr plus conn. · why: No W3C trace context exists, and spans are not attached to lines.
decision/span-field-allow-list · Spans declared with explicit field allow-list — Every span is declared skipping all arguments with an explicit field allow-list.
decision/verify-logs-only-with-instance · verify and plugin install log only inside an instance — With VIOLA_NAME set, verify and plugin install log process-start / process-exit to cli-<name>.ndjson; from a plain terminal with no instance they write no process-log file.
decision/wheel-event-is-audit-record · Wheel product event is the audit record — The wheel product event in events.ndjson is the audit record of a wheel change; the typed keys are never logged and the wheel transition span is only a point in time.
contract/viola-verify-refusal/home-tree-refused · viola verify strict-modes refusal · home tree refused — Before logging, verify holds the home, ledger/, bin/ and plugin/ to the strict-modes check; a refused tree exits 1 with the home pair and writes no line.
contract/viola-verify-refusal/stamp-write-refused · viola verify strict-modes refusal · stamp write refused — A ledger/stamps.json the stamp write refuses prints the home pair and exits 1 through verify's own process-exit line.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/clients-screen-peer-replies-to-code-words · Clients screen peer replies to code words and a fixed fault — The CLI and MCP clients screen a peer reply before using it: a refusal and its detail must be code words, a wrapper fault keeps only its code, a fixed message and closed data, and the screening adds no span and no line. · why: No path or upstream text may cross from a wrapper into a document, a tool result or a log.
decision/disconnect-logged-by-client · Mid-call disconnect logged once by the client — A mid-call disconnect is logged once, by the client.
decision/mcp-explicit-log-writer · Explicit log writer in viola mcp — The viola mcp log writer is always explicit: the mcp.ndjson file, with no stderr fallback.
decision/no-dialog-content-in-logs · Dialog content never logged — The question, the answer and the tool input never appear in a log line.
decision/no-last-assistant-message · last_assistant_message never logged — last_assistant_message never appears in a log line.
decision/no-params-in-channel-lines · No params or result bodies in channel lines — channel-request and channel-response lines never carry params or result bodies.
decision/no-text-in-telemetry · Sent text never in telemetry — The sent text never appears in a span or log line; only its length does.
decision/no-typed-keys-in-telemetry · Typed keys never in telemetry — The keys the human types never appear in a span.
decision/real-cli-verify-never-in-ci · Real-CLI verify never runs in CI — In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
decision/rmcp-tracing-target-off · rmcp tracing target OFF — The native tracing target of rmcp is OFF in the sink. · why: Its records lack required fields and can carry frame content.
decision/send-line-correlation · Send lines correlate with channel lines — Every wrapper send-* line carries conn (or srv_conn) and rpc_id, so that pair on the send line equals (conn, corr) on the call's channel lines; the client-side send-refused carries neither.
decision/transcript-echo-read-uninstrumented · Transcript echo read uninstrumented — The send's transcript echo read, the end offset before the paste and the tag count after it, opens no span and writes no line or span field of its own; its result shows only as the send-confirmed line inside run.confirm_window.
decision/unreadable-ledger-means-unverified · Unreadable capability ledger means an unverified CLI — A capability ledger that is missing a stamp, unreadable, malformed or refused by the strict-modes check is treated as an unverified CLI (cli_verified false).
decision/verify-lines-carry-no-content · verify log lines carry no content — No verify log line carries row text, a prompt, a session id, the probe command or a path.
decision/verify-sole-stamps-writer · verify is the only stamps writer — verify is the only writer of the capability ledger's stamps file.
decision/wheel-typed-mark-uninstrumented · Wheel typed mark and wakes count uninstrumented — The stdin typed mark and the held scheduled-wakes count are in-memory wrapper state that open no span and write no line of their own, and carry no key and no prompt text.
contract/confirm-window-close/clear · run.confirm_window close · /clear — On a verified CLI, closes on the session-start whose cause is clear and whose agent_session_id differs from the id the send slot remembers; no prompt-submitted claims the send.
contract/confirm-window-close/compact · run.confirm_window close · /compact — On a verified CLI, closes on the session-start whose cause is compact and whose agent_session_id equals the id the send slot remembered; nothing remembered claims nothing, and no prompt-submitted claims the send.
contract/confirm-window-close/context · run.confirm_window close · /context — On a verified CLI, closes on a new local-command-stdout tag in the session transcript within the fallback window, with no prompt submitted; on an unverified CLI, or before the session transcript exists, the send reads unconfirmable.
contract/confirm-window-close/ordinary-send · run.confirm_window close · ordinary send — Closes on the prompt-submitted{origin:"driver"} whose prompt equals the typed text, or is non-empty and equals the typed text with a trailing run of ASCII space, TAB, CR or LF removed, or on window expiry.
contract/confirmed-send-spans/span-chain · confirmed send spans · span chain — send.client (CLIENT) › channel.request (CLIENT) → channel.dispatch (SERVER) › run.readiness_gate › pty.paste_write › run.confirm_window; an unconfirmable send ends at pty.paste_write. In the hook: hook.handle › channel.request(hook.event).
contract/mcp-instance-unreachable/server-verify-failed · viola mcp frame tool refusal · server-verify-failed — A refused euid, a mismatch or a check that cannot be made returns the codes-only isError instance-unreachable with detail server-verify-failed, no frame and no log line.
contract/mcp-instance-unreachable/socket-dir-refused · viola mcp frame tool refusal · socket-dir-refused — A refused socket directory returns instance-unreachable with detail socket-dir-refused, no frame and no line.
contract/mcp-instance-unreachable/strict-modes-failed · viola mcp frame tool refusal · strict-modes-failed — A refused instance check, run before the live endpoint is read, returns instance-unreachable with detail strict-modes-failed, no frame and no line.
contract/mcp-refusal-text/instance-unreachable · viola mcp wait and last refusal · strict-modes-failed body — MCP wait and last refused by the instance check return the cli document less v (error instance-unreachable, detail strict-modes-failed), their text refused: instance-unreachable, with no line.
contract/mcp-spans/mcp-tool-call · viola mcp spans · mcp.tool_call — viola mcp opens one mcp.tool_call span per tool (send, wait, last, answer, list); rejects log codes-only send-refused or channel-response lines.
contract/send-log-fields/channel-request · confirmed send log fields · channel-request — channel-request{corr:<id>, conn, method, sender}.
contract/send-log-fields/send-confirmed · confirmed send log fields · send-confirmed — send-confirmed{corr:<cursor>, confirmed, duration_ms}, confirmed false for unconfirmable, duration_ms from the send's arrival including the readiness gate's wait.
contract/send-log-fields/send-issued · confirmed send log fields · send-issued — send-issued{corr:<cursor>, from, text_bytes}.
contract/send-refused-client/control-character · client-side send validation refusal · control-character — Made before any frame: send-refused{corr:null, refusal:"not-delivered", detail:"control-character", side:"client"}, one line per deciding side, never the text.
contract/send-refused-client/empty-text · client-side send validation refusal · empty-text — An empty typed text, read after control-character and ahead of both wheel reads; a wrapper deciding it writes its own send-refused{side:"wrapper"} with the same pair, corr, conn and rpc_id, and no send-issued.
contract/send-refused-detail/input-not-ready · wrapper send-refused detail · input-not-ready — send-refused{corr:<cursor>, refusal, detail:"input-not-ready", side:"wrapper", wheel}.
contract/send-refused-detail/no-prompt-submitted · wrapper send-refused detail · no-prompt-submitted — Confirm-window expiry is send-refused{detail:"no-prompt-submitted"} for ordinary, /clear, /compact and echo-confirmed local command sends alike, with no new detail.
contract/send-refused-detail/turn-running · wrapper send-refused detail · turn-running — Covers a running turn the wrapper knows of before the paste, or another send in flight, with no new field; the refusal has no cursor (corr the end offset at refusal), no send-issued, and the client exits 13.
contract/send-refused/human-typing · wrapper send-refused · human-typing — send-refused{corr:<cursor>, refusal:"human-typing", detail:null|"manual-pause", wheel:"human", side:"wrapper"}, with the client process-exit{exit_code:10}.
contract/send-span-attributes/run-readiness-gate · confirmed send span attributes · run.readiness_gate — outcome (ready or input-not-ready) and vt100_panicked (bool).
contract/send-span-attributes/window-ms · confirmed send span attributes · run.confirm_window window_ms — window_ms is the bound the send's own deadline reads: 40 000 for a /compact send waiting for its compacted-session post-condition, else 10 000.
contract/span-kinds/server · Span kinds · SERVER — The SERVER spans are channel.dispatch (the wrapper side of every channel.request), mcp.tool_call and hook.handle.
contract/verify-exit-codes/0 · verify exit codes · 0 — Every relied ledger row counted: each passed, or the browser-tools-prompt row read absent.
contract/verify-process-exit/interrupted · verify process-exit line · detail interrupted — A verify stopped by SIGINT or SIGTERM starts no further spawn, closes every pair it opened and ends with process-exit subject self, exit_code 1, detail "interrupted".
contract/verify-stdout/step-line · verify stdout · step line — One "[NN/MM] <row id> <row words>  pass|absent|fail" line is printed for each relied ledger row, MM being the relied row count; absent is the browser-tools-prompt row's word when no run met the prompt.
contract/verify-stdout/summary-line · verify stdout · summary line — The last stdout line is "stamped <version>  N pass  N fail", the two counts summing to MM; the pass count holds every row that counts, an absent browser-tools-prompt row among them.
contract/wait-last-spans/span-chain · wait and last spans · span chain — wait.block (CLIENT) › channel.request → channel.dispatch › run.wait_dispatch (INTERNAL); for last, last.client › channel.request.
contract/wait-log-fields/channel-request · wait and last log fields · channel-request — channel-request{corr:<id>, conn, method:"wait"|"last"} on both sides, plus after and timeout_ms for wait only and only as u64 values.
contract/wait-log-fields/channel-response-outcome · wait and last log fields · channel-response outcome — For wait only, outcome is the waking kind or timed-out, derived once over the shared wake set; the client side alone writes instance-unreachable when the reply never arrived; last carries no outcome and no result body reaches either line.
contract/wait-span-attributes/run-wait-dispatch · wait span attributes · run.wait_dispatch — woken_kind, one of turn-ended, question, permission, plan, session-end, dialog-expired, or timed_out.
contract/wait-span-attributes/wait-block · wait span attributes · wait.block — wait.block carries after and timeout_ms.
decision/instance-check-before-role-file · Instance check before any role file — The frame verbs run the instance check on their named instance (the driven instance for link and unlink) before any role file opens.
decision/manual-readiness-gate-span · Manual span on the readiness gate — The readiness gate of viola run carries a manual span, run.readiness_gate. · why: vt100 has no tracing.
decision/no-obs-gate-on-verify-lines · No obs gate depends on verify lines — No observability gate depends on verify's own log lines; its home-level lines fall under the same panic, schema and secret-scan gates as any other.
contract/channel-lines/fields · channel-request and channel-response lines · fields — channel-request carries method, corr, conn and sender; channel-response carries method, corr, conn, result_class and error_code, plus detail on a -32602.
contract/cli-exit-21/endpoint-lost · cli exit 21 process-exit · endpoint-lost — detail endpoint-lost with during call: the connection was lost after the connect; it is the only exit-21 detail whose during is call.
contract/diag-line-schema/os · diag-line v1 schema · os — os is closed to linux.
contract/events-log/dialog-expired · events log · dialog-expired — At the hold's end the wrapper appends one dialog-expired{dialog_id, dialog_kind} line (source wrapper, no question, plan or tool text) that wakes a parked wait, and writes dialog-answered with deadline_hit:true and no from.
contract/run-log-parse-rejected/ledger-stamps · the run process log parse-rejected line · parser ledger-stamps — An unreadable, malformed or strict-modes-refused capability ledger gives one parse-rejected line with parser "ledger-stamps", detail "unreadable", "malformed" or "strict-modes-failed", and a count, in the run log.
contract/send-log-fields/hook-invoked · confirmed send log fields · hook-invoked — hook-invoked{hook_event:"user-prompt-submit", corr:null} for an ordinary send; hook_event session-start for a /clear or /compact confirmed by its session; an unconfirmable send, or a local command confirmed by its transcript echo, waits for no hook line.
contract/send-span-attributes/text-bytes · confirmed send span attributes · text_bytes — text_bytes, on pty.paste_write and send-issued alike, is the typed text's length: the sent text with each CR LF pair and each other CR as one LF, trailing CR and LF removed; pty.paste_write also carries paste_mode="bracketed".
contract/span-kinds/client · Span kinds · CLIENT — The CLIENT spans are send.client, wait.block, last.client, answer.client, pause.client, release.client, cli.link, cli.unlink, channel.request (client side) and pty.spawn.
contract/verify-exit-codes/1 · verify exit codes · 1 — A row failed, verify refused, or verify was stopped.
contract/verify-process-exit/strict-modes-failed · verify process-exit line · detail strict-modes-failed — A stamps store refused by its strict-modes check, under a parsed VIOLA_NAME, ends with process-exit carrying detail "strict-modes-failed" and the home pair on stderr.
contract/verify-spawn-subject/agents-probe · verify child spawn subjects · agents-probe — Run B's single agents --json read is logged as one process-start / process-exit pair with subject agents-probe.
contract/verify-spawn-subject/verify-probe · verify child spawn subjects · verify-probe — The print-mode probe is logged as one process-start / process-exit pair with subject verify-probe.
contract/verify-spawn-subject/verify-pty-probe · verify child spawn subjects · verify-pty-probe — Each of verify's six interactive PTY runs is logged as one process-start / process-exit pair with subject verify-pty-probe.
contract/verify-spawn-subject/verify-sender · verify child spawn subjects · verify-sender — The print-mode claude sender of Run B's cross-session turn is logged as one process-start / process-exit pair with subject verify-sender; its prompt and the composed socket address appear in no line.
contract/verify-spawn-subject/version-probe · verify child spawn subjects · version-probe — The --version read of the agent CLI is logged as one process-start / process-exit pair with subject version-probe.
contract/verify-stop-stderr/unable-hint-pair · verify stderr on a stop · the stop pair — A stopped verify prints exactly "unable: verify was stopped before it finished" and "hint: run viola verify again to stamp this CLI" on stderr, with empty stdout and no stamp written.

## 5-metric-coverage · ## 5. Metric Coverage · twin metric-coverage
decision/metric-grouping-closed-enums · Metric grouping by closed enums only — Aggregate reports group only by closed enums (event, detail, refusal, method, hook_event, subject, parser, process); corr, conn, pid, child_pid and instance are join keys, never grouping keys.

## metric-coverage · ## 5. Metric Coverage · twin 5-metric-coverage
contract/cli-process-exit/server-verify-failed · CLI process-exit diagnostic line · server-verify-failed — The CLI's server-verification refusal writes one process-exit with subject self, exit_code 21, detail server-verify-failed and during connect at WARN in cli-<name>.ndjson, with no pid, start time, path or endpoint; MCP's wait tool writes no line for it.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/additive-field-naming · Additive log field naming — Additive log fields are snake_case with kebab-case values, and carry codes only.
decision/file-only-per-role-log-sinks · File-only log sinks per role — Diagnostics are written to files only, one sink per role, under the home's diagnostics directory.
decision/ipc-calls-logged-without-arguments · IPC calls logged without arguments — IPC params and result bodies are never recorded, not even as a hash or digest.
decision/json-to-file-only-log-mode · JSON-to-file as the only log mode — JSON to file is the only log mode: there is no agent-mode flag and no stdout pretty sink on any role.
decision/log-schema-additive-only · Log schema additive to the tests' format — The observability log schema only adds fields and enum values to the tests' log format; it renames nothing and removes only a field no build emits. · why: The tests' log format is the binding text.
contract/ipc-invocation-log/fields · IPC invocation log lines · fields — Channel-request and channel-response lines carry method, conn (srv_conn when the client sent none) and the corr of a frame with an id; a request line adds sender, a response line result class and an error's error code but never sender.
contract/log-levels/info · log levels · info — INFO carries all other boundary, lifecycle and state-transition events, including refusals, which are normal product outcomes.
contract/log-line/corr · every log line · corr — Set per the tests' table and copied unchanged as a JSON number or string; null is represented by the key's absence.
contract/log-line/event · every log line · event — A value of the closed kebab-case event enum.
decision/content-detail-in-instance-file · Content-bearing detail in the instance detail file — Content-bearing detail goes only to the per-process detail file inside the instance's diagnostics directory.
decision/third-party-crate-logs-off · Third-party crate logs off — The per-module log filter turns logging OFF for the rmcp, portable_pty, sysinfo and interprocess targets. · why: Their failures are captured at the viola seam as viola events.
decision/viola-crates-log-at-info · Viola crates log at INFO — The per-module log filter sets the viola, viola_core, viola_pty, viola_channel, viola_state, viola_agent_claude and viola_mcp targets to INFO, or DEBUG via diagnostics_level.
contract/child-spawn-log/subject · child and shell spawn log events · subject — process-start and process-exit lines for child spawns carry a subject from a closed set.
contract/pty-seam-log/paste · PTY seam log events · paste — A paste is logged with text_bytes only.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
contract/diag-line-fields/channel-request · diagnostic line additive fields · channel-request — method, conn / srv_conn, sender, v, after, timeout_ms.
contract/diag-line-fields/channel-response · diagnostic line additive fields · channel-response — method (only a closed-list name), conn / srv_conn, result_class (ok, refusal, error), detail (only the three -32602 faults), error_code (-32700, -32600, -32601, -32602, -32603), outcome, duration_ms.
contract/diag-line-fields/process-exit · diagnostic line additive fields · process-exit — subject, exit_code (self), child_exit_status, exit_source (handle-wait or kill-fallback), detail, during (connect or call), duration_ms.
contract/diag-line-fields/send-events · diagnostic line additive fields · send-issued / send-confirmed / send-refused — from, from_trust, text_bytes (normalised typed length), confirmed, refusal, detail, side (client or wrapper), wheel, duration_ms, and wrapper side only conn / srv_conn and rpc_id.
contract/parse-rejected-detail/control-character · parse-rejected detail · control-character — Listed in the diagnostics schema for the paste-text parser, but never emitted: a control character in a paste is refused as send-refused not-delivered.
contract/parse-rejected-detail/strict-modes-failed · parse-rejected detail · strict-modes-failed — ledger-stamps: run's version gate refused the stamps file on the strict-modes check, and the read counts as unverified.
contract/parse-rejected-detail/unreadable · parse-rejected detail · unreadable — config-json, ledger-stamps.
contract/send-refused-detail/empty-text · send-refused detail · empty-text — Written for an empty typed text on send; the word adds no field, and send-refused detail stays an open string in the diag-line schema.
contract/send-refused-detail/refusal-reason-words · send-refused detail · RefusalReason detail words — Refusal details are the arch RefusalReason details verbatim: manual-pause, input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text.
contract/diag-detail/socket-dir-refused · diagnostic detail words · socket-dir-refused — A closed detail on process-exit and hook-decision for a socket dir refused as a symlink, no directory, foreign owner or mode bit: run and revive's start write process-exit exit 1, CLI frame verbs process-exit exit 21 during connect at WARN, viola hook a hook-decision at WARN; MCP writes none.
contract/diag-detail/strict-modes-failed · diagnostic detail words · strict-modes-failed — Written on the process-exit exit 1 of viola verify under a parsed VIOLA_NAME and on parse-rejected ledger-stamps from the version gate of run, at warn, with no path; every other refused strict-modes check writes no log line, its exit code or JSON document its only record.
contract/diag-line-fields/parse-rejected · diagnostic line additive fields · parse-rejected — parser (claude-agents-json, vt100-feed, config-json, ledger-stamps, hook-stdin, channel-frame, mcp-stdin, state-entry, hyprctl-json), detail, count.
contract/diag-subject/agents-probe · diag-line subject enum · agents-probe — Subject of the process-start and process-exit pair of the claude agents read, written by viola list, viola revive's session-live reading, verify's Run B and the list tool of viola mcp into their role files.
contract/diag-subject/auto-verify · diag-line subject enum · auto-verify — Subject of the process-start and process-exit pair of the viola verify child that a viola run start spawns, the exit carrying child_exit_status and duration_ms, written to the run log.
contract/diag-subject/claude-child · diag-line subject enum · claude-child — One value of the closed subject enum of the diag-line schema.
contract/diag-subject/hyprctl-probe · diag-line subject enum · hyprctl-probe — Subject of the process-start and process-exit pair that the Hyprland seam writes around each hyprctl spawn.
contract/diag-subject/self · diag-line subject enum · self — One value of the closed subject enum of the diag-line schema.
contract/diag-subject/verify-probe · diag-line subject enum · verify-probe — Subject of the process-start and process-exit pair of the print-mode probe of viola verify.
contract/diag-subject/verify-pty-probe · diag-line subject enum · verify-pty-probe — Subject of the process-start and process-exit pair of each of the six interactive PTY runs of viola verify, Run A to Run F.
contract/diag-subject/verify-sender · diag-line subject enum · verify-sender — Subject of the process-start and process-exit pair of the print-mode sender that viola verify's Run B cross-session turn spawns.
contract/diag-subject/version-probe · diag-line subject enum · version-probe — Subject of the process-start and process-exit pair of a version probe spawn.
contract/parse-rejected-detail/malformed · parse-rejected detail · malformed — Any parser.
contract/parse-rejected-detail/oversize · parse-rejected detail · oversize — channel-frame beyond MAX_FRAME, hook-stdin, mcp-stdin past MAX_FRAME, vt100-feed when the bounded queue drops a copy (one line per poisoning episode), claude-agents-json past MAX_FRAME, hyprctl-json past MAX_FRAME.
contract/parse-rejected/mcp-stdin-oversize · parse-rejected diagnostic line · parser mcp-stdin, detail oversize — Written at WARN into mcp.ndjson once per session, after the session ends, only when the stdin reader cut a line past MAX_FRAME, with count 1; the line holds no byte of the cut line.
contract/process-exit-detail/exit-20 · process-exit detail · exit 20 — wrapper-fault.
contract/process-exit-detail/exit-21 · process-exit detail · exit 21 — instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused.
contract/verify-process-exit/child-exit-status-and-duration · viola verify spawn process-exit lines · child_exit_status and duration_ms — Each verify spawn pair's process-exit carries child_exit_status and duration_ms, including a child of an interactive PTY run killed at a Ctrl-C or TERM stop; a stopped run writes only the pairs it started.

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/boundary-errors-end-in-logged-line · Crate-seam errors end in a logged line — Every typed error that crosses a crate seam ends in a logged process-exit detail, channel-response error, hook-decision detail or send-refused line, carrying codes only at WARN or ERROR.
decision/error-correlation-without-trace-ids · Error correlation without trace ids — Errors correlate by corr, conn, process and instance, plus timestamp, not by trace_id or span_id.
decision/instance-unreachable-not-logged · instance-unreachable logged by no line — An MCP instance-unreachable reject, from any instance, liveness, socket-directory, euid or server-verification check, is logged by no line; the isError code is its only record.
decision/mcp-rejects-codes-only · MCP rejects are codes only — MCP rejects are isError codes only, and a logged reject's line holds codes only.
decision/serde-errors-become-internal-error · Serde error chains replaced by internal error — At dispatch, an error chain whose source is a serde_json or serde_path_to_error error is replaced by the fixed message "internal error" before it reaches stderr, --json, MCP isError or channel error.data. · why: A scrubbing layer at the source.
contract/panic-outcome-by-role/mcp-task-panic · panic outcome by role · mcp spawned-task panic — A panic inside an mcp spawned Tokio task is logged by the hook, contained at the task boundary, and the process continues.
decision/typed-errors-fixed-display · Typed errors with fixed Display messages — Every typed error has a fixed-message Display, written with thiserror, and holds no fields with paths or payloads. · except: PtyError's Display is hand-written, as viola-pty takes no thiserror.
contract/panic-outcome-by-role/mcp-main-thread-panic · panic outcome by role · mcp main-thread panic — An mcp main-thread panic writes no process-exit line and nothing to stdout, and the process exits 1.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
decision/env-vars-not-config-channel · Env vars are not a configuration channel — Backtrace capture is not gated by RUST_BACKTRACE or any env var. · why: env vars are not a configuration channel
decision/mcp-unreachable-carries-one-cause-word · Each MCP unreachable result carries one cause word — Each MCP unreachable outcome (a refused check, a dead or unwrapped instance, a lost endpoint, an unverified server or a refused socket directory) carries exactly one UnreachableCause word as its detail. · why: The MCP process writes no exit line for it, so the result's cause word is the cause's only record for the calling agent.
decision/peer-replies-screened-as-code-words · Peer replies screened as code words — A peer reply's refusal and detail are kept only as code words (1-64 bytes of [a-z0-9-], letter first); an unknown code word stays visible and exits 14. · why: by design
decision/single-sanitising-fault-reader · One sanitising reader of peer faults — A peer's fault is read only through one sanitiser: five known codes read fixed messages with data admitted only in validated shapes, a code outside them reads wrapper fault, and a fault without an integer code reads nothing.
contract/answer-send-exit-2/stderr-line · viola answer and viola send exit 2 · stderr line — Exit 2 writes the fixed unusable-response or unusable-text line to stderr whatever --json says, with no document.
contract/mcp-frame-tools/socket-dir-refused · viola mcp frame tools · socket-dir-refused — A refused socket directory returns isError instance-unreachable with detail socket-dir-refused, and writes no frame and no line.
contract/mcp-frame-tools/strict-modes-failed · viola mcp frame tools · strict-modes-failed — A refused instance check returns isError instance-unreachable with detail strict-modes-failed, and writes no frame and no line.
contract/mcp-stdin/oversize-line · viola mcp stdin · line past MAX_FRAME — A line past MAX_FRAME is refused whole, gives no isError result and ends the session as a closed stdin: exit 0 after initialize, exit 1 before it with both streams empty; its one record is parse-rejected mcp-stdin oversize, no byte of the line.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/content-only-in-detail-file · Content-bearing records only in the detail file — Content-bearing records go only to the instance-scoped detail-<process>.ndjson file.
decision/conversation-text-only-in-detail-file · Conversation text only in the instance detail file — Prompt, assistant, tool-input, plan and dialog text never reach a home-level line, only an instance's per-process detail file inside a chain or drift report.
decision/default-deny-log-fields · Default-deny for logged fields — A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure. · guard: gate G4 against schemas/diag-line.v1.json
decision/endpoint-and-pinned-bin-never-logged · endpoint and pinned_bin never logged — The endpoint and pinned_bin values are never logged.
decision/low-class-metadata-as-typed-fields · Low-class metadata as typed fields — Low-class metadata is logged as typed fields, never inside detail codes.
decision/schema-top-level-unevaluated-properties · Default-deny schema via top-level unevaluatedProperties — The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists. · why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then · guard: contract_diag_schema
decision/source-allow-list-logging · Source allow-list for logged fields — Spans skip all arguments and declare fields explicitly, and events go through obs_event! with explicit typed fields only; this source allow-list is the primary PII control. · guard: disallowed-macros; raw_event_hits; G1 bare instrument
decision/content-canary-never-home-level · Content canary never in home-level files — The content canary never appears in a home-level diagnostics ndjson file.
decision/fixed-error-messages · Fixed error messages — Error Display texts are fixed, and serde-sourced error chains are mapped to fixed messages at dispatch.
decision/home-level-lines-path-free · Home-level lines path-free — Home-level log lines are path-free: file fields are basenames, panic_location is workspace-relative and source file recording is off; absolute paths appear only in detail files. · why: The OS username inside absolute paths is the only PII in scope.
decision/secrets-never-logged · Secrets never logged — Signing credentials and R8-stripped CLAUDE* values are never logged.
contract/detail-line/drift-report · the detail-file line · drift_report — One of exactly four content fields a detail-file line may carry: the drift report.
contract/detail-line/event · the detail-file line · event — Required on every detail-file line, as on home-level lines.
contract/detail-line/instance · the detail-file line · instance — Required on every detail-file line, as on home-level lines.
contract/detail-line/level · the detail-file line · level — Required on every detail-file line, as on home-level lines.
contract/detail-line/message · the detail-file line · message — Required on every detail-file line, as on home-level lines.
contract/detail-line/process · the detail-file line · process — Required on every detail-file line, as on home-level lines.
contract/detail-line/target · the detail-file line · target — Required on every detail-file line, as on home-level lines.
contract/detail-line/timestamp · the detail-file line · timestamp — Required on every detail-file line, as on home-level lines.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/closed-protocol-codes · Closed protocol codes — Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value. · guard: answer_invalid_params_names_its_cause_and_no_param
decision/endpoint-paths-never-logged · Endpoint and pinned binary never logged — The snapshot endpoint and pinned_bin are never logged, only endpoint_kind.
decision/error-bodies-clean · Clean error bodies — Error bodies of CLI --json, MCP results and the channel faults hold no absolute path, upstream caused-by text or serde path, and every product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
decision/link-names-never-in-process-log · Link pair names never in a process log — link and unlink driver and driven names appear only in the wrapper's link and unlink event lines in events.ndjson, never in a process-log line; the channel pair logs the method, never the params. · guard: link_verbs_log_the_channel_pair_and_no_names
decision/prompt-and-output-never-in-home-logs · Prompt and assistant output never in home logs — Prompt text and assistant output, last_assistant_message included, never appear in a home-level process-log line; they reach an instance detail file only inside a chain or drift report, and only text_bytes is logged.
decision/veil-redaction · veil derives redaction — veil 0.3.0's derived Redact is what redacts the payload fields of contract types, and instrumented spans skip all arguments.
decision/canary-only-in-detail-files · Canary only in detail files — Every E2E and integration input is synthetic and embeds a fixed canary, which may appear in detail files but never in a home-level diagnostics file. · guard: mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — thiserror messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
decision/server-verify-refusal-names-nothing · Server-verify refusal names nothing — A server-verification refusal carries no pid, start time, path or endpoint in any output or log line. · guard: security_negatives_server_identity

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/instrument-requires-skip-all · Instrument attribute must skip all arguments — Every #[instrument] in the workspace takes skip_all as its first argument; a bare #[instrument] fails the build. · guard: gate G1

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/correlation-propagation · Correlation propagation — Every channel call logs corr and conn on both sides; a send line from send-issued on carries the send cursor, a wrapper refusal before it the events.ndjson end offset as corr, and a client-side refusal no corr; every dialog line carries dialog_id.
decision/deterministic-emission · Deterministic event emission — Events key on hook events and events.ndjson offsets; nothing is derived from screen content. · except: The readiness gate reads the vt100 screen before a paste and can refuse a send as input-not-ready; a local command's echo confirmation keys on its transcript's stdout tag count.
decision/module-boundary-error-logging · Errors logged once at module boundaries — Every typed error that crosses a crate seam is logged once at WARN or ERROR with a closed detail or error_code; the full chain goes only to the instance detail file.
decision/no-retry-policies · Retry policies forbidden — Retry-once policies are forbidden, for tests and for panics alike. · why: retry policies mask real signals
decision/no-sleep-based-log-waits · No sleep-based log waits — No log-based wait relies on sleeps.
decision/nonzero-count-blocks-chunk · Nonzero error count blocks the chunk — Any nonzero error-budget count makes the chunk red until the root cause is fixed within that chunk. · why: the tests quarantine rule
decision/print-and-macro-lint-failure-fails · Print and macro lint failure is a failure — A clippy disallowed-macros, print_stdout, print_stderr or dbg_macro failure counts as an SLO invariant failure.
decision/schema-conformance-failure-fails · Schema-conformance failure is a failure — An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.
decision/zero-unlogged-panics · Zero unlogged panics — Every panic in run, hook, mcp and cli after init step 4 produces exactly one single-line ERROR panic record in its home-level role file, with no multi-line backtrace there, and hook still exits 0; retry-once policies for panic handling are forbidden. · guard: G2 · except: the bounded exemptions where no file exists, so no line can be written; G2 alone leaves out the fake-agent seam's deliberate panic from its count.

## slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin 10-slo-invariants-telemetry-budgets
decision/no-retry-on-panic · No retry on panic — Retry-once policies for panic handling are forbidden. · why: Parallel to the tests' retries = 0.

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/claude-env-and-cookie-never-logged · CLAUDE* values never logged — CLAUDE* values are never logged, at any level, in any file, detail files included.
decision/closed-event-and-corr-on-every-line · Closed event enum on every line, corr where correlated — Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
decision/corr-set-explicitly-by-caller · corr set explicitly by the caller — The caller sets corr explicitly as a typed field on the events that carry one.
decision/event-values-closed · event values closed — No event value is added without a Decisions Log entry.
decision/explicit-corr-conn-instance · corr, conn and instance passed explicitly — corr, conn and instance are passed as explicit values across threads or tasks, never carried by span context. · why: Spans are not serialized.
decision/field-allow-list-default-deny · Field allow-list is default-deny — No field is logged unless named in the field catalog.
decision/file-sink-only-error-sink · File sink is the only error sink — Local logs are never optional: the file sink is the only error sink and works with every other feature off.
decision/no-bare-instrument · No bare #[instrument] — Instrumentation always uses #[instrument(skip_all, fields(...))], never a bare #[instrument]. · why: A bare #[instrument] records every argument with Debug and leaks send text, hook.dialog payloads and tool input.
decision/no-paths-or-pids-in-detail-codes · No paths or pids in detail codes — detail codes never contain paths or pids.
decision/no-pii-in-span-fields · No PII or content in span fields — Span fields never include PII or content. · why: Scrubbing at an exporter stage is too late, and v1 has no exporter stage at all.
decision/no-print-macros · No print macros in product code paths — print!, println!, eprint!, eprintln! and dbg! are never used in the hook, run or mcp code paths of the product crates, and every product member inherits the workspace lints. · why: They bypass the file sink and break the Hook contract, the run terminal rule and MCP framing. · guard: clippy print_stdout, print_stderr and dbg_macro at deny · except: viola-harness, the fake agent and the output modules that own --json, human CLI stderr and the hook decision body.
decision/no-retry-once · No retry-once policies — No retry-once policy is added for panics or flaky telemetry assertions.
decision/no-serde-source-in-error-output · No serde source in printed errors — An anyhow chain that still contains a serde_json or serde_path_to_error source is never printed to stderr, --json, MCP isError or channel error.data.
decision/no-stringified-corr · corr never recorded with ? or % — corr is never recorded with ?corr or %corr. · why: Those stringify numbers.
decision/no-telemetry-on-stdout · No telemetry on stdout — Telemetry is never written to stdout on any role. · why: stdout carries each role's own output and run's terminal carries the child's screen.
decision/paths-only-in-instance-detail-files · Paths only in instance detail files — Paths appear only in instance detail files.
decision/sensitive-content-detail-file-only · Sensitive content only in the instance detail file — Send text, tool input and last_assistant_message never reach a home-level file; they may reach only the instance-scoped detail-<process>.ndjson, inside a chain or drift report.
contract/stdout-by-role/mcp · stdout by role · mcp — stdout carries MCP frames.
decision/failing-job-diagnostics-kept · Failing job's diagnostics kept — A failing job's diagnostics are never lost: G2, G4 and the secret scan run if: always().
decision/must-trace-paths-always-traced · Must-trace paths always traced — Spans and events are never skipped on the 6 must-trace paths. · why: The agent loses its ability to debug without them.
decision/no-info-in-hot-loops · No info logging in hot loops — Nothing is logged at info inside hot loops such as the PTY pump; counts go on the boundary event instead.
decision/no-telemetry-from-screen-content · No telemetry from screen content — Telemetry is never derived from screen content; vt100 is used for readiness and modal gate outcomes only.
decision/rerun-no-substitute · Re-run is no substitute for diagnostics — A re-run is not a substitute for the failing job's diagnostics. · why: CI runs under retries = 0.
decision/transcripts-not-a-contract · Claude Code transcripts are not a contract — Claude Code transcripts are never read as a contract. · except: A local command's stdout tag in the session transcript, counted past the end recorded before the paste, is read as that send's echo confirmation; no transcript byte is kept.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/content-only-in-instance-detail-file · Content only in instance-scoped detail files — Content-bearing detail goes only to the instance's detail-<process>.ndjson file; home-level log files are codes-only and carry no user content, token or CLAUDE* value. · why: security allows user-derived content only in instance-scoped diagnostics
decision/diag-schema-matches-event-enum · Diagnostic line schema matches the event enum — A hand-maintained JSON Schema of the process-log line is committed, and its event enum must equal the ObsEvent variants, asserted by a tests-owned check.
decision/instrument-skip-all-everywhere · Spans skip all arguments — Every instrumented function uses skip_all with explicit fields, so no argument is recorded implicitly. · guard: G1
decision/fixed-error-display · Fixed error Display text — Typed errors have fixed Display text that carries no interpolated content.

## log-file-location · key of ## 3. Observability Harness Contract
contract/diagnostics-files/paths · diagnostics files · process log paths — Process logs are written to <home>/diagnostics/ as run-<name>, hook-<name>, mcp and cli-<name> .ndjson files, and detail to <home>/instances/<name>/diagnostics/detail-<process>.ndjson.

## log-format-json-schema · key of ## 3. Observability Harness Contract
contract/corr-by-event/send · corr by event · send-issued, send-confirmed, send-refused — corr is the send cursor; a wrapper refusal before send-issued carries the target's event-stream end offset, never returned to the caller; wrapper send lines carry conn and rpc_id; the client-side send-refused has corr null.
contract/process-log-line/corr · the process log line · corr — Copied unchanged as a JSON number or string, never renamed and never debug- or display-formatted; required only on the events whose rules always define it.
decision/null-as-key-absence · Null written as key absence — A null corr or instance is written as an absent key, readers treat absence and null as equal, and the schema rejects a literal null. · why: tracing has no null field value
decision/obs-aligns-to-tests-log-format · Observability aligns to the tests log format — The log format is bound by the test plan's harness contract; observability may add fields but must not rename them or remove one the product still emits. · why: the harness greps on them
contract/corr-by-event/channel · corr by event · channel-request and channel-response — corr is the JSON-RPC request id; null for an id-less hook.event notification and on a response to a frame that yielded no id.
contract/event-stream/line · the event stream · event line — Each line of instances/<name>/events.ndjson is exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.

## logging-stack · key of ## 3. Observability Harness Contract
decision/file-only-diagnostics-sinks · File-only diagnostics, one file per role — Every role logs only to its own file under the home's diagnostics directory; no role has a stdout sink, and there is no pretty or dev sink.
decision/no-content-in-home-level-lines · No content fields in home-level lines — No home-level diagnostics line carries chain, drift_report, panic_payload or backtrace. · guard: G4 diag-line.v1.json schema
decision/explicit-log-writer · Explicit diagnostics writer — The diagnostics subscriber always has its writer set explicitly. · why: The default writer is stdout.
decision/no-log-crate-bridge · No log-crate bridge — The log-crate bridge (tracing-log, LogTracer) stays off; failures are captured from returned values or handles, and a gap is fixed at the seam or by the researched json-subscriber fallback, never by enabling the bridge. · why: The accepted loss is only third-party log context emitted without a returned error. · guard: pty_exit_is_read_on_the_handle_while_the_output_is_held
decision/third-party-targets-off · Third-party log targets off — Records of third-party targets (rmcp, portable_pty, sysinfo, interprocess) are off in every sink; their failures are captured at viola's seam from the returned results. · why: Their records lack the required fields.
contract/diagnostics-files/detail · the diagnostics files · detail — Content-bearing records (full error chains, drift reports, panic payload and backtrace) go only to <home>/instances/<name>/diagnostics/detail-<process>.ndjson, file 0600 in a 0700 directory, and only when an instance resolves.
contract/diagnostics-line/role-file-line-size · the diagnostics line · role-file line size — A line in a role file (run, hook, mcp, cli) is codes-only and stays well under 4 KiB.
contract/obs-event-macro/attached-fields · the obs_event macro · attached fields — Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.

## otel-sdk-init · key of ## 3. Observability Harness Contract
decision/diagnostics-init-before-role-work · Diagnostics init before the role's work — Diagnostics init and the process-start line come before the role's work: in run before its collision check and the child spawn, in mcp before its stdio transport starts. · why: Exit-1 causes are logged.

## product-mode-normal-vs-inverted-self-observing · key of ## 3. Observability Harness Contract
decision/normal-observability-mode · Normal observability mode — viola runs in Normal observability mode: events.ndjson is a product audit log, not a telemetry collector. · why: viola is not a telemetry product; the self-observing-inversion trigger did not fire.

## service-identity · key of ## 3. Observability Harness Contract
decision/env-vars-not-configuration · Environment variables are not configuration — Environment variables are not a configuration channel: service identity has no env-var source or fallback, and the one CLAUDE_CONFIG_DIR value read by verify and the run and revive start, when the strip keeps that name, locates the claude config file, is never logged and configures nothing of viola.

## snapshot-paste-to-ai-integration · key of ## 3. Observability Harness Contract
contract/send-line/client-side-refusal · the send line · client-side send-refused — The caller-side send-refused{side:"client"} has corr null.
contract/send-line/unconfirmable · the send line · unconfirmable send — An unconfirmable send logs one send-issued and one send-confirmed{confirmed:false} under the same corr, with no send-refused and no hook side.
contract/send-line/wrapper-side · the send line · wrapper side — send-* lines whose corr is a send cursor are emitted only by the target wrapper; only its send-refused line carries side:"wrapper", its send-issued and send-confirmed lines carry no side, and a client-side send-refused carries side:"client" and no corr.
contract/send-line/cursor-uniqueness · the send line · cursor corr uniqueness — A send cursor is a byte offset into the target's own events.ndjson, unique per target only; refusals and later sends can share it, so one send is identified by target instance, corr, conn and rpc_id.

## trace-context-propagation · key of ## 3. Observability Harness Contract
decision/explicit-correlation-values · Correlation values passed explicitly — corr, conn and instance are passed as explicit values into each thread or task and set on each event, never recovered from span context. · why: Spans are not serialized.
decision/span-carried-into-tasks-and-threads · Span not carried into tasks and threads — Spawned async tasks and std thread closures take no span from their spawner: spans never reach a log line, and correlation travels as explicit typed fields.
contract/diagnostics-line/corr-on-send-lines · the diagnostics line · corr on send lines — On send lines, corr carries the send cursor; a send refused before send-issued carries the target's events.ndjson end offset at refusal, a log join key only, never returned to the caller.
contract/send-line/conn-rpc-id · the send line · conn and rpc_id — Every wrapper send-* line carries conn (or srv_conn) and rpc_id, and (conn, rpc_id) equals (conn, corr) on that call's channel-request and channel-response on both sides.
decision/channel-join-key-conn-corr · Channel join key is conn and corr — The merged-log join key for channel lines is (conn, corr); instance is deliberately not part of it. · why: Each side logs its own instance, so the two sides of one call carry different instance values.
contract/channel-params/conn · viola-channel request params · conn — The client sends conn = "<process>-<pid>-<t0>-<n>" in params (t0 the process start in epoch milliseconds, n a per-process connection counter); both sides log it, and the server strips it before dispatch, never an input to a method.
contract/diagnostics-line/corr-on-channel-lines · the diagnostics line · corr on channel lines — On channel lines, corr carries the JSON-RPC id; a notification such as hook.event has no id, so its channel-request has corr null.

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
