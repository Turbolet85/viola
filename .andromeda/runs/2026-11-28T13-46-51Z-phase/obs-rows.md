# obs-plan · the rows that bind 2026-11-28-console-actions — picked 521 of 1063 (337 by every draw) · rule any · pin 5384

## obs-plan-viola · (above the first heading)
decision/agent-readable-telemetry · Agent-readable telemetry only — Every telemetry signal must be readable by an agent from a snapshot or pasted log excerpt: no human-only dashboards, no proprietary APM lock-in, no human-review-gated log analysis. · except: A surface that cannot be agent-readable is flagged not-instrumentable and handled through boundary instrumentation only.

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/agents-probe-output-untrusted · Agents probe output untrusted — The output rows of the claude agents --json probe are untrusted.
decision/agents-probe-parse-failure-unknown · Agents probe parse failure becomes unknown — A parse failure of the claude agents --json probe becomes status "unknown" plus a parse-rejected counter; an unparsed --version answer leaves cli_version absent and the version unverified.
decision/assistant-message-never-logged · Assistant message never logged on readback — On `wait` / `last` readback, the `last_assistant_message` content never appears in log fields.
decision/cli-file-sink-only · Short-lived CLI logs to a file sink only — Short-lived CLI verbs log only to a JSON-per-line file sink in diagnostics when an instance resolves, never to stdout. · why: Stdout carries the --json results.
decision/corr-field-propagation · Propagation through the corr field — Correlation across the viola-channel is carried by the corr field in log lines of the process on each side. · why: Channel params declare no trace-context field.
decision/dialog-text-never-logged · Dialog text never logged on answer — On the dialog to `answer` path, the question, answer and tool `input` text never appear in log fields.
decision/distinct-detail-code-per-exit-cause · Distinct detail code per exit cause — Each cause behind exit 21 and exit 1 gets a distinct detail code with no path or pid. · except: A strict-modes check refused before any log opens, or viola console's terminal check, writes no line; its cause is the fixed stderr pair or the --json document.
decision/escape-controls-in-terminal · Control characters escaped in terminal rendering — C0 and C1 control characters are escaped in any human terminal rendering of logged rows.
decision/exit-1-detail-codes · Distinct detail codes for exit-1 causes — On the `viola run` start sequence to child spawn, each exit-1 cause logs a distinct `detail` code with no path or pid in it.
decision/instance-never-from-working-directory · Instance never from the working directory — The instance comes from VIOLA_NAME or an argument, never from the working directory.
decision/one-log-file-per-role · One log file per process role — Each process role writes its own file under `diagnostics/`, no role has a stdout sink, and no role falls back to stderr JSON when its file cannot be opened.
decision/parse-failures-unknown-counter · Parse failures surface as unknown plus a counter — A child-output parse failure surfaces as `unknown` plus a counter. · except: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown.
decision/pty-byte-stream-never-logged · PTY byte stream never logged — The PTY byte stream is never logged; the viola-pty seam is instrumentable, and the openpty OS API behind it is boundary-only. · why: The PTY byte stream is user content.
decision/run-silent-terminal · viola run silent on the terminal — viola run writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns.
decision/send-text-confined · Send text confined to instance diagnostics — `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
decision/send-text-never-logged · Send text never logged — Sent text never appears on stderr, in --json output, in MCP results or in any process-log field outside instances/<name>/diagnostics/.
decision/stderr-fixed-strings · Fixed strings on stderr — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/tracing-json-line-file-pipeline · tracing JSON-per-line file pipeline — Telemetry is tracing plus tracing-subscriber writing JSON-per-line files under diagnostics/, correlated by corr and consumed through agent-run logs and status. · why: The pipeline is the one the tests plan binds.
decision/tracing-json-pipeline · tracing with the tracing-subscriber JSON formatter — The `tracing` facade plus the tracing-subscriber JSON formatter is the binding logging pipeline, and JSON-to-file is the only logging mode.
decision/typed-keys-never-logged · Typed keys never logged — Keys a human types never appear in a log line; the wheel product event in events.ndjson is the audit record.
contract/diag-line/instance · diagnostic log line · instance — The instance's `ViolaName`; the key is absent when the process has no instance.
contract/diag-line/process · diagnostic log line · process — The role of the writing process, one of run, hook, mcp, cli, console; required on every line and matching `sender` / `writer`.
contract/diagnostics-files/cli · the diagnostics files · cli — Role cli writes <home>/diagnostics/cli-<name>.ndjson, only when an instance resolves.
contract/diagnostics-files/console · the diagnostics files · console — Role console writes <home>/diagnostics/console.ndjson, home-level, under process console with no instance key on its lines.
contract/log-line/process · every log line · process — Every log line carries a process key naming its role: one of run, hook, mcp, cli, console.
contract/process-log-event/release-from-driver · the process log event field · release-from-driver — Release from the driver is logged as its own distinct event.
decision/channel-lines-carry-no-bodies · Channel log lines carry no bodies — IPC channel log lines never carry `params` or result bodies.
decision/child-exit-from-process-handle · Child exit from the process handle — The wrapped child's exit and its process-exit event come from the wait on the process handle, never from reader EOF.
decision/claude-internals-not-a-contract · Wrapped claude internals are not a contract — The wrapped claude CLI is a third-party binary run unmodified and instrumented at its boundary only (spawn and exit, paste writes, the vt100 readiness-gate outcome); its internal state, transcripts and bundle strings, the MCP client's tool-call bound among them, are never read as a contract. · except: A send's echo confirmation counts its stdout tag in the transcript, keeping no byte; verify writes and removes its temp folder's trust record; a verified start sets its folder's trust flag only.
decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits.
decision/diagnostics-owner-only-modes · Diagnostics files are owner-only — Every diagnostics file is mode 0600 inside a 0700 directory.
decision/error-chain-fixed-stderr-message · Error chains reach stderr as fixed messages — In the viola root binary, the only crate that uses anyhow, an error chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to the diagnostics logs only.
decision/event-payload-not-in-process-logs · Event payloads kept out of process logs — The payload content of events.ndjson is never copied into process-log lines. · why: events.ndjson is itself the product audit stream.
decision/no-traceparent-on-local-socket · No W3C traceparent on the local socket — Calls from CLI, MCP and hook to the wrapper carry no W3C `traceparent`; correlation uses `corr` (JSON-RPC `id`, `dialog_id`, send `cursor`) plus `process` and `instance`. · why: channel `params` declare no trace-context field
decision/redaction-not-disableable · Redaction cannot be disabled — No verbosity control, env, flag or config may disable or bypass redaction.
decision/refused-diagnostics-drops-line · Refused diagnostics file drops its line — A refused `diagnostics/` folder or file is treated as a failed open and its log line is dropped.
decision/serde-errors-fixed-message · Serde error chains become a fixed message — A serde-sourced anyhow error chain is replaced by a fixed message before it reaches stderr.
decision/spans-not-wire-contract · Tracing spans are not part of the wire contract — Span names are internal `tracing` spans in `<area>.<operation>` snake_case form; the agent-facing contract is the closed `event` enum plus `corr`. · why: the config sets `with_current_span(false)`, so spans do not reach the line
contract/diagnostics-files/run · the diagnostics files · run — Role run writes <home>/diagnostics/run-<name>.ndjson.
contract/json-rpc-errors/32603 · viola-channel JSON-RPC errors · -32603 — Error -32603 is "internal error" with data:null.
contract/process-exit-detail/socket-dir-refused · process-exit detail codes · socket-dir-refused — Carried by the line of a refused socket directory, where it has one; a refused socket directory is no strict-modes failure.
contract/run-log-file/path · the viola run log file · path — viola run logs only to a file sink at diagnostics/run-<name>.ndjson.

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/agents-cli-instrumented-at-boundary · claude agents CLI instrumented at spawn, exit and parse — The external `claude agents --json` CLI is instrumented only at its spawn, its exit and its parse outcome. · why: An external CLI.
decision/agents-cli-output-untrusted · claude agents output untrusted — The output rows of `claude agents --json` are treated as untrusted.
decision/agents-cli-parse-failure-unknown · claude agents parse failure reads unknown — A failed parse of `claude agents --json` output reads "unknown" plus a counter.
decision/child-exit-from-handle-wait · Child exit from the process handle — The child's process-exit comes from the wait on its process handle, never from reader end-of-file. · why: Child exit is detected on the process handle.
decision/controls-escaped-in-terminal-output · Controls escaped in terminal output — C0 and C1 control characters are escaped in any human terminal rendering of logged rows.
decision/correlation-in-corr-field · Correlation in the corr log field — Correlation across a channel call travels in the corr log field; channel params declare no trace-context field.
decision/detail-codes-no-paths · Detail codes carry no path or pid — No path or pid appears in a detail code.
decision/diagnostics-checked-at-open · Diagnostics checked before open — Before a role or detail file is created or narrowed, the existing diagnostics/ folder and the file are checked, a symlink refused and no group or other bit allowed; a refusal is a failed open.
decision/diagnostics-file-modes · Diagnostics file and directory modes — Diagnostics files are 0600 inside 0700 directories. · guard: secret-scan test
decision/diagnostics-owner-only-permissions · Diagnostics owner-only permissions — Files under diagnostics/ are 0600 inside 0700 directories. · guard: secret-scan test
decision/exit-code-log-field · Exit code as a log field — The exit code (0, 1, 2, 10, 12–14, 20, 21), the machine-readable outcome, is carried as a log field; 11 is retired and never reused.
decision/fixed-stderr-messages · Fixed stderr refusal and fault lines — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/instance-from-name-not-cwd · Instance from its name, never the working directory — The instance comes from VIOLA_NAME or an explicit argument, never from the working directory.
decision/log-file-basenames-follow-tests · Log file names follow tests — Process-log files are <home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>,console}.ndjson, as tests require. · why: Tests is upstream; arch and security place diagnostics under the instance directory instead.
decision/no-process-log-without-instance · No process log without an instance — A short-lived CLI verb run with no resolved instance writes no process-log file; its human stderr carries only fixed-message lines, and its outcome is the exit code and stdout. · except: viola console, which logs under its own console role to the home-level console.ndjson with no instance.
decision/no-telemetry-from-screen · No telemetry from screen content — No telemetry is derived from screen content; vt100 serves gate outcomes only.
decision/panic-hook-replaced · Panic hook replaced by a one-line writer — Each process replaces the default panic hook, as the first statement in main and before any child spawns, with one that writes one JSON line and nothing to stderr. · why: The default hook prints to stderr, which breaks the hook contract and is the child's terminal under run.
decision/parse-failures-surface-as-unknown · Parse failures surface as unknown — A parse failure surfaces as the value unknown plus a counter. · except: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown.
decision/per-parser-parse-failure-counts · Per-parser parse-failure counts — Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, the hyprctl replies, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead.
decision/process-init-order · Process logging init order — Each process installs its panic hook, parses arguments, resolves home and instance and runs its strict-modes check, opens its diagnostics file append-mode 0600 in a 0700 directory, installs the subscriber, then emits process-start. · why: For run this completes before the start sequence so the collision check and version gate are logged; for mcp, before the stdio transport starts.
decision/process-logs-file-sink-only · Process logs to a file sink only — Process logs are written only as JSON lines to one file per process role under diagnostics/; there is no network exporter and no stdout sink. · why: viola makes no outbound network calls, and stdout carries --json results.
decision/send-refused-by-each-deciding-side · Send refusal logged by each deciding side — A send-refused with detail control-character or empty-text is emitted by each side that decides it, client and wrapper, without the text.
decision/structured-json-process-logs · Structured JSON process logs — Every process role (run, hook, mcp, cli, console) writes structured JSON process logs under diagnostics/, which harness logs merges with events.ndjson.
decision/subscriber-explicit-writer · Subscriber installed with an explicit writer — The tracing subscriber is always installed with an explicit writer, never stdout. · why: tracing-subscriber fmt() writes to stdout unless a writer is configured, and stdout carries MCP frames.
decision/tests-event-list-adopted · Tests event list adopted — The tests' field and event list is adopted verbatim, with send-issued and send-refused among the logged events.
decision/tracing-json-logging-pipeline · tracing with a JSON subscriber for logging — Logging and spans go through the Rust tracing facade with a tracing-subscriber JSON structured logger.
contract/channel-response/detail · Channel response line · detail — On a channel-response answering -32602, detail is one closed word: unsupported-version, invalid-params or release-from-driver, on both the server's and the client's line; it is absent on every other channel-response line and names no param or value.
contract/cli-frame-verbs/strict-modes-failed · CLI frame verbs · strict-modes-failed — A strict-modes refusal of send, wait, last, answer, pause or release writes no line and prints only the fixed instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, exit 21, with no pid, path or endpoint.
contract/cli-internal-error/stderr-line · CLI internal error · stderr line — Without --json a catch-site fault prints the fixed line `error: internal error` to stderr.
contract/cli-json-error/strict-modes-failed · CLI --json error document · instance-unreachable strict-modes-failed — A refused instance check of send, wait, last, answer, pause, release, link, unlink or switch exits 21 with the instance pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
contract/cli-log-file/revive-start · CLI process-log file · viola revive start arm — Logs as the wrapper, process="run", into diagnostics/run-<name>.ndjson.
contract/diag-line-event/release-from-driver · Diag line event · release-from-driver — release-from-driver is its own logged event, admitted by the diag-line schema, not a generic -32602 wrapper fault.
contract/dialog-log-line/server-verify-failed · Dialog log line · server-verify-failed — A server-verify failure exits 21 on CLI answer with during connect, and on the dialog hook fails open with corr null and decision_emitted false; the line names no pid, start time, path or endpoint.
contract/dialog-log-line/socket-dir-refused · Dialog log line · socket-dir-refused — A refused socket directory exits 21 on CLI answer with during connect, and on the dialog hook fails open with corr null and decision_emitted false; the line names no pid, start time, path or endpoint.
contract/dialog-log-line/unverified-cli · Dialog log line · unverified-cli — The detail unverified-cli goes with exit 12.
contract/exit-1-detail/already-live · Exit 1 detail codes · already-live — already-live is a distinct detail code for an exit-1 cause, naming no path or pid.
contract/exit-1-detail/cwd-missing · Exit 1 detail codes · cwd-missing — cwd-missing is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
contract/exit-1-detail/no-session · Exit 1 detail codes · no-session — no-session is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
contract/exit-1-detail/pinned-hash-mismatch · Exit 1 detail codes · pinned-hash-mismatch — pinned-hash-mismatch is a distinct detail code for an exit-1 cause, naming no path or pid.
contract/exit-1-detail/session-live · Exit 1 detail codes · session-live — session-live is a distinct detail code for the exit-1 cause of viola revive and of viola adopt, naming no path or pid.
contract/exit-1-detail/socket-dir-refused · Exit 1 detail codes · socket-dir-refused — socket-dir-refused is a distinct detail code for an exit-1 cause, naming no path or pid.
contract/exit-1-detail/squatted-name · Exit 1 detail codes · squatted-name — squatted-name is a distinct detail code for an exit-1 cause, naming no path or pid.
contract/exit-1-detail/stale-heartbeat · Exit 1 detail codes · stale-heartbeat — stale-heartbeat is a distinct detail code for an exit-1 cause, naming no path or pid.
contract/exit-21-detail/endpoint-lost · Exit 21 detail codes · endpoint-lost — endpoint-lost is a distinct detail code for an exit-21 cause, naming no path or pid.
contract/exit-21-detail/instance-dead · Exit 21 detail codes · instance-dead — instance-dead is a distinct detail code for an exit-21 cause, naming no path or pid.
contract/exit-21-detail/server-verify-failed · Exit 21 detail codes · server-verify-failed — server-verify-failed is a distinct detail code for an exit-21 cause, distinct from instance-dead, naming no path or pid.
contract/exit-21-detail/socket-dir-refused · Exit 21 detail codes · socket-dir-refused — socket-dir-refused is a distinct detail code for an exit-21 cause, naming no path or pid.
contract/exit-21-detail/strict-modes-failed · Exit 21 detail codes · strict-modes-failed — strict-modes-failed is a distinct detail code for an exit-21 cause, naming no path or pid.
contract/exit-21-detail/unwrapped-name · Exit 21 detail codes · unwrapped-name — unwrapped-name is a distinct detail code for an exit-21 cause, naming no path or pid.
contract/exit-cause-detail/server-verify-failed · Exit cause detail codes · server-verify-failed — A failed server check of peer euid, peer pid or start time is logged by the CLI and the hook as server-verify-failed; MCP writes no line for it.
contract/exit-cause-detail/socket-dir-refused · Exit cause detail codes · socket-dir-refused — A refused socket directory has its own detail code socket-dir-refused and writes its line (MCP writes none); on run and on a start made by viola revive that line is one process-exit{subject:"self", exit_code:1} before any snapshot, child or bind.
contract/json-rpc-error/32603 · JSON-RPC error · -32603 — The -32603 error carries the message "internal error" with data null.
contract/process-exit/socket-dir-refused-on-frame-verb · Process exit line · socket-dir-refused on a CLI frame verb — A refused socket directory on a CLI frame verb writes one process-exit with detail socket-dir-refused, exit 21, during connect.
contract/process-exit/socket-dir-refused-on-start · Process exit line · socket-dir-refused on a start — A refused socket directory on a start writes one process-exit with detail socket-dir-refused, exit 1.
contract/process-log-corr/panic · Process log corr by event · panic — corr is null.
contract/process-log-corr/process-exit · Process log corr by event · process-exit — corr is null.
contract/process-log-corr/process-start · Process log corr by event · process-start — corr is null.
contract/process-log-corr/send-confirmed · Process log corr by event · send-confirmed — corr is the send cursor.
contract/process-log-corr/send-issued · Process log corr by event · send-issued — corr is the send cursor.
contract/process-log-corr/send-refused · Process log corr by event · send-refused — corr is the send cursor.
contract/process-log/child-and-probe-lifecycle · Process log · child and probe lifecycle — process-start and process-exit are logged for the claude child, for the claude agents --json and --version probes and for verify's print-mode sender; the terminal viola open or viola up spawns gets a process-start alone.
contract/process-log/corr · Process log line · corr — Copied unchanged as a JSON number or string, never renamed.
contract/process-log/event · Process log line · event — A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
contract/process-log/process · Process log line · process — The process role, one of run, hook, mcp, cli, console.
contract/refusal-reason/unknown-fallback · Refusal reason · unknown fallback — An unrecognised RefusalReason falls back to unknown with exit 14.
contract/run-exit-1-detail/cwd-missing · run exit-1 detail codes · cwd-missing — On a start made by viola revive, a distinct detail code.
contract/run-exit-1-detail/no-session · run exit-1 detail codes · no-session — On a start made by viola revive, a distinct detail code.
contract/run-exit-1-detail/session-live · run exit-1 detail codes · session-live — On a start made by viola revive or viola adopt, a distinct detail code.
contract/send-log-line/corr · Send log line · corr — On the wrapper's send lines corr is the send cursor, which joins send-issued, send-confirmed and send-refused (a refusal before send-issued carries the log's end offset); the CLI's and MCP's client-side send-refused lines carry no corr; on channel lines corr is the JSON-RPC id.
contract/send-log-line/detail · Send log line · detail — The send detail is one of input-not-ready, no-prompt-submitted, control-character, empty-text, turn-running or paste-cut.
contract/send-log-line/from · Send log line · from — The from field is self-reported and logged as unauthenticated, never as an identity.
contract/send-log-line/required-fields · Send log line · required fields — A send line carries event, process, instance, corr, from, confirmed, refusal, detail and duration_ms.
contract/unreachable-cause/endpoint-lost · exit 21 cause words · endpoint-lost — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/unreachable-cause/instance-dead · exit 21 cause words · instance-dead — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/unreachable-cause/server-verify-failed · exit 21 cause words · server-verify-failed — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/unreachable-cause/socket-dir-refused · exit 21 cause words · socket-dir-refused — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/unreachable-cause/strict-modes-failed · exit 21 cause words · strict-modes-failed — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/unreachable-cause/unwrapped-name · exit 21 cause words · unwrapped-name — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/viola-revive/strict-modes-failed · viola revive output · strict-modes-failed — A strict-mode fail of viola revive writes no log line; it prints the stderr pair, or under --json {"v":1,"error":"strict-modes-failed","detail":null}, and exits 1.
contract/wheel-log-line/human-typing · Wheel log line · human-typing — A send refused while a human holds the wheel logs refusal human-typing, detail null or manual-pause, the wheel holder and exit_code 10.
contract/wheel-path-log-events/pause-and-release · Wheel path log events · pause and release — A wheel take-over logs send-refused for the refused send and a channel-request and channel-response pair for pause and for release.
decision/anyhow-error-capture-boundary · The anyhow crate is the error-capture boundary — The only crate that uses anyhow is the error-capture boundary, with context chains only at dispatch; the full chain goes to diagnostics only.
decision/channel-log-scrub-rule · Channel log scrub rule — Channel request and response lines log only the method, id or corr, the result class (ok, refusal or error code), the sender version and the closed detail word, never params or result bodies.
decision/deterministic-event-emission · Deterministic event emission — Events are emitted deterministically, keyed on hook events and events.ndjson offsets. · except: A local command's echo confirmation keys on the stdout tag count in its session transcript.
decision/fault-telemetry-per-class · Fault telemetry per class — Each fault class emits its own telemetry: a healed torn ndjson line with file and offset, a corrupt or unsupported snapshot recovered by replay, a child exit seen without EOF, an endpoint vanished during wait, and a stale heartbeat against a live pid.
decision/from-not-authenticated · from never logged as authenticated — The self-reported from field is never logged as an authenticated identity.
decision/instrument-call-sites-not-os-internals · OS operations instrumented at viola's call sites — OS-level operations (bind, connect, lock, rename, kill fallback, the server check's peer reads, the strict-modes stat reads, the socket directory check, the hyprctl spawn) are instrumented at viola's own call sites, not inside the OS.
decision/json-to-file-only-mode · JSON-to-file the only logging mode — JSON-to-file is the only logging mode; there is no separate agent-mode flag.
decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but a required one the product still emits is never renamed or removed. · why: The harness greps on them.
decision/no-interactive-terminal-interface · No interactive terminal interface — viola offers no interactive terminal interface: no prompt, TUI or pager. · except: the terminal fleet console (`viola console`), its own surface, which reads keys in raw mode and redraws its frame
decision/r8-strip-logged-as-names-only · R8 strip logged as names only — The R8 environment strip is logged as a count and lists of stripped and kept variable names, and environment maps never pass to a span or event; no value is read, bar the kept CLAUDE_CONFIG_DIR value that viola verify and the run, revive and adopt start read, and no value is logged.
decision/run-silent-on-terminal · run silent on the terminal — run writes nothing to the terminal while the child runs; its process log goes only to diagnostics/run-<name>.ndjson. · why: The terminal carries the child's screen.
decision/serde-chain-fixed-message · Serde error chains become a fixed message on stderr — An error chain that contains a serde source becomes a fixed message before it reaches stderr.
decision/serde-errors-become-fixed-message · Serde errors become a fixed message — An error chain holding a serde source is replaced by a fixed message before it reaches stderr.
decision/single-line-log-records · Single-line log records — Every log record is one JSON line parseable by jq; never a multi-line stack trace.
decision/strict-modes-refusal-writes-no-line · Strict-modes refusal writes no line — A strict-modes check refused before any log opens ends the process with nothing under the home opened and no log line; its cause is carried only by the fixed stderr pair or the --json document. · why: Ratified as the answer to whether a refused strict-modes check logs. · except: The ledger-stamps read in run's version gate and a refused update_stamps in viola verify each leave a line.
contract/channel-response/internal-error · Channel response line · -32603 — A -32603 is logged at ERROR with result_class error, error_code -32603 and no detail; the fixed text internal error is only the reply's message, never a log field.
contract/channel-response/invalid-params-causes · Channel response line · invalid-params causes — detail invalid-params covers send's invalid params, a release whose from is not a string or that carries any budget key, and a link or unlink whose driver or driven is missing, not a string or no valid name, whose driven is not the serving wrapper's own name, or whose driver equals driven.
contract/cli-log-file/revive-list · CLI process-log file · viola revive --list — Logs as cli into the caller's cli-<VIOLA_NAME>.ndjson once its strict-modes check has passed; no file with no VIOLA_NAME.
contract/dialog-log-line/corr · Dialog log line · corr — On dialog lines corr is the dialog_id, which joins hook and wrapper lines; the driver's channel lines carry the JSON-RPC id.
contract/dialog-log-line/required-fields · Dialog log line · required fields — A dialog line carries event, process, instance, corr, dialog_kind, deadline_hit and duration_ms; refusal or detail does not appear on a dialog-raised or dialog-answered line.
contract/dialog-path-log-events/event-order · Dialog path log events · event order — A dialog logs hook-invoked, dialog-raised, dialog-answered, then hook-decision, whose decision is null on deadline expiry.
contract/event-line/shape · events.ndjson event line · line shape — Exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
contract/mcp-iserror/instance-unreachable · MCP isError result · instance-unreachable — The instance-unreachable detail is one closed cause: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused or call-deadline, the last written by the frame tools alone; the CLI's document less v, its text `refused: instance-unreachable`.
contract/mcp-iserror/refusal · MCP isError result · refusal — A peer refusal that is no code word reads unknown, and a string detail that is no code word reads null.
contract/process-log-corr/dialog-answered · Process log corr by event · dialog-answered — corr is the dialog_id.
contract/process-log/instance · Process log line · instance — A ViolaName or null.
contract/process-log/level · Process log line · level — One of DEBUG, INFO, WARN, ERROR.
contract/process-log/message · Process log line · message — A required field, as fields.message or the flattened message.
contract/process-log/target · Process log line · target — A required field.
contract/process-log/timestamp · Process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
contract/run-exit-1-detail/already-live · run exit-1 detail codes · already-live — A live holder or a fresh replayed beat.
contract/run-exit-1-detail/sha-256-mismatch · run exit-1 detail codes · sha-256 mismatch — A distinct detail code for a sha-256 mismatch.
contract/run-exit-1-detail/squatted-name · run exit-1 detail codes · squatted-name — A distinct detail code for a squatted name.
contract/run-exit-1-detail/stale-heartbeat · run exit-1 detail codes · stale-heartbeat — A stale holder.
contract/run-log/cli-verified · run log line · cli_verified — A required field on run's log lines.
contract/run-log/exit-code · run log line · exit_code — A required field on run's log lines.
contract/run-start-events/order · run start events in events.ndjson · order — A run start writes wheel{cause:"start"}, then one lineage line, then one launch line before session-start; on a start whose folder-trust keys pass through the wrapper, wheel{human-input} also precedes session-start, and a later human key adds no wheel line, the wheel being already human.
contract/send-path-log-events/event-order · Send path log events · event order — A send logs channel-request, then send-issued, then hook-invoked (UserPromptSubmit, or SessionStart for a confirmed /clear or /compact; none awaited for an unconfirmable send, a cut paste or a local command its transcript echo confirms), then send-confirmed or send-refused, then channel-response.
contract/wait-log-line/exit-code-21 · Wait log line · exit_code 21 — A wait exits 21 when the wrapper is killed mid-wait (detail endpoint-lost, during call), or when a CLI wait meets a refused socket directory before any frame (detail socket-dir-refused, during connect, no channel-request).

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/corr-conn-correlation · Correlation through corr and conn — Every cross-surface call carries correlation through the corr field plus the additive conn discriminator.
decision/one-line-panic-record · One structured line per panic — Every panic after init step 4 produces exactly one single-line structured record in its home-level role file with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file. · except: the bounded panic exemptions listed in the plan's Section 10
decision/span-naming · Span naming as <area>.<operation> — Span names are <area>.<operation> in snake_case, <area> one of run, pty, channel, state, hook, send, wait, last, answer, pause, release, cli, mcp, hyprland; cli.<verb> names the dispatch span of a short-lived verb.
contract/process-log-line/enum-values · the process log line · enum values — Enum values are kebab-case.
contract/process-log-line/event · the process log line · event — The event field is a closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
decision/spans-not-serialized-v1 · Spans declared but not serialized — Spans are declared in code but not serialized in v1.
contract/process-log-line/at-instants · the process log line · *_at — Instants are named *_at and written in RFC 3339 UTC with milliseconds and a Z suffix.
contract/process-log-line/duration-ms · the process log line · duration_ms — Durations are duration_ms as an integer.
contract/process-log-line/field-names · the process log line · field names — Field names are snake_case.
contract/process-start-event/service-name · the process-start event · service_name — Emitted once per process as the fixed value "viola", the binary name.
contract/process-start-event/version · the process-start event · version — Emitted once per process as the crate package version.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/cross-process-join-corr-conn · Cross-process joins by corr and conn — Lines across processes are joined by the corr and conn fields, not by W3C trace IDs.
decision/panic-caught-at-every-role · Panics caught at every role's catch site — A hand-written panic hook plus catch_unwind at every role's main-thread catch site (covering hook dispatch and run's inline handle-wait), the three run PTY threads (the two pump copy threads and the paste writer thread) and the vt100 feed turn a panic into one record.
decision/panic-one-line-record · One single-line record a panic — Every panic after obs init produces exactly one single-line structured record in its home-level role file, with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file. · except: the pre-init window and the paths that refuse or lack an instance or home tree before their role file opens
decision/pre-open-strict-modes-refusal-silent · Strict-modes refusal before a log opens writes no line — A strict-modes refusal made before any process log opens writes no line and carries no logged detail code; its machine-readable form is the exit code and, under --json, its closed document. · except: a refused socket directory always logs detail socket-dir-refused, except at MCP's frame tool, which writes no line
decision/span-naming-skip-all · Span declaration convention — Every span is declared with instrument, skip_all and a name of the form "<area>.<operation>".
contract/socket-dir-refused-exit/codes · socket-dir-refused exit codes · socket-dir-refused — A refused socket directory logs detail socket-dir-refused with no path, pid or uid, and exits 21 at the eight CLI frame verbs and 1 at the start of run, revive and adopt.
contract/strict-modes-refusal-exit/1 · strict-modes refusal exit codes · 1 — run, revive's start, adopt's start, verify, open, up's home-tree check and viola console's mid-run board scan exit 1 on a strict-modes refusal; revive, adopt, open and up under --json print the strict-modes-failed document.
contract/strict-modes-refusal-exit/21 · strict-modes refusal exit codes · 21 — The six frame verbs, link / unlink, focus and up's board scan exit 21 on a strict-modes refusal, under --json with the instance-unreachable document carrying detail strict-modes-failed.
decision/spans-not-serialized-in-v1 · Spans declared but not serialized in v1 — Spans are declared on the must-trace paths and every crate seam but are not written to output in v1. · why: they give the code its structure and are ready for the deferred OpenTelemetry bridge
decision/structured-logs-primary-signal · Structured logs are the primary signal — tracing-subscriber JSON-per-line logs, one file per process role under the home's diagnostics directory, are the primary observability signal, merged with the event stream by agent-run logs.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/obs-owns-logger-and-format · Obs owns the logger and format — The logger and the log format are owned by observability.
decision/short-lived-verbs-stderr · Short-lived verbs use stderr for humans — Short-lived CLI verbs may use stderr for their human output.
contract/console-output/stdout · console output streams · stdout — viola console writes only its screen to stdout, in the alternate screen, plain ASCII with no colour; its refusals and its internal error go to stderr.
contract/list-stderr/unable-hint-pair · viola list stderr · unable / hint pair — The unable / hint: pair carries no SGR colour codes.
decision/codes-only-process-logs · Codes-only process logs, content in detail files — Every role's codes-only process log goes to its home-level role file under diagnostics, and content-bearing detail goes only to the instance's per-role detail file.
contract/run-terminal-output/child-output · run terminal output · child output — run writes nothing to the terminal except the child's own output and, before the child, a keyed start's automatic-verify lines and the verify child's own output, none under revive --json; while the child runs, neither run nor a revived start writes anything of its own.
contract/run-terminal-output/start-refusal · run terminal output · start refusal — A start refusal before any child is spawned writes exactly two fixed stderr lines with no path or pid: the home pair for a home-level refusal, the instance pair for an instance-level one.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/detail-codes-only · detail is a closed code — The detail field is a closed kebab-case code catalog and never carries a path or pid.
decision/dialog-content-never-logged · Dialog content never logged — Dialog questions, answers, plan text and tool input never appear in a home-level process-log line; dialog events carry codes only, and the text reaches an instance detail file only inside a chain or drift report.
decision/no-endpoint-path-or-env-in-spans · No endpoint, binary path or env values in spans — A span never carries the endpoint name, the pinned_bin path or environment values.
decision/no-err-ret-span-options · err and ret span options banned — The err and ret instrumentation options are banned on spans.
decision/sent-text-never-logged · Sent text never logged — A sent message's text is never logged; only its size in text_bytes is.
decision/span-field-allow-list · Spans declared with explicit field allow-list — Every span is declared skipping all arguments with an explicit field allow-list.
decision/standing-modal-reading-uninstrumented · Standing-modal reading uninstrumented — The readiness gate's standing-modal reading adds no log line, span field, event or detail code of its own: a modal label quoted above a ready input box lets the send proceed with nothing written, and a standing modal refuses with the existing detail input-not-ready.
decision/static-span-names · Static span names — Span names are static strings; instance names, dialog ids and paths never appear in a span name.
decision/wheel-event-is-audit-record · Wheel product event is the audit record — The wheel product event in events.ndjson is the audit record of a wheel change; the typed keys are never logged and the wheel transition span is only a point in time.
contract/cli-log-file/focus · CLI process-log file · viola focus — Logs as cli into the named instance's cli-<name>.ndjson, a process-start and a closed-detail process-exit inside the cli.focus dispatch span, which carries no fields.
contract/cli-wait-last-connect/server-verify-failed · CLI wait / last connect refusal · server-verify-failed — A serving process that is not the recorded wrapper or runs under another euid ends the call before any frame with exit 21 and one WARN process-exit line (subject self, during connect) inside the dispatch span, and no channel-request line.
contract/cli-wait-last-connect/socket-dir-refused · CLI wait / last connect refusal · socket-dir-refused — A refused socket directory, checked before the server comparison, ends the call before any frame with exit 21 and one WARN process-exit line (during connect), and no channel-request line.
contract/cli-wait-last-connect/strict-modes-failed · CLI wait / last connect refusal · strict-modes-failed — A home or instance tree refused by the instance check, checked first, ends the call before any role file opens with exit 21 and no line; under --json the output is exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
decision/channel-lines-no-params-or-results · Channel lines carry no params or results — Channel log lines never record request params or result bodies.
decision/channel-response-always-logged · channel-response always logged — Every dispatched channel request logs its channel-response line, an early return or an error included. · why: The line is emitted from a Drop guard, so an early return or error still logs it.
decision/child-settings-never-logged · Child settings argument never logged — The claude child's --settings argument appears in no log line or span.
decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.
decision/compact-confirm-window · Compact confirmation window — A /compact send waits 40 s for its compacted-session post-condition, while every other confirmed send waits 10 s.
decision/env-names-only · Environment names only, never values — The strip, every log, the hook capture arm's env-names file and the verify stamp and record carry environment variable names only, never values, and no env map is passed to a span or event. · guard: E2
decision/manual-spans-only · Manual spans only — Every span is written by hand; no surface uses an auto-instrumentation library.
decision/no-assistant-message-in-logs · No assistant message or result body in logs — last_assistant_message never appears in a log, and no result body reaches a log line.
decision/unreadable-ledger-is-unverified · Unreadable ledger counts as unverified — A missing stamp, or an unreadable, malformed or strict-modes-refused capability ledger, counts as unverified (cli_verified false).
contract/claude-child-process-start/env-kept · process-start line of the claude child · env_kept — Reports the variables kept after the R8 environment strip on process-start{subject:"claude-child"} by names only, never values.
contract/claude-child-process-start/env-stripped-count · process-start line of the claude child · env_stripped_count — Reports the R8 environment strip on process-start{subject:"claude-child"}; never an environment value.
contract/claude-child-process-start/env-stripped-known · process-start line of the claude child · env_stripped_known — Reports the R8 environment strip on process-start{subject:"claude-child"} by variable names only, never values.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/cli-log-only-under-viola-name · cli process log only under a parsed name — A short-lived cli verb opens its process log and writes process-start and process-exit only when an instance resolves: for list, plugin install and revive --list under a parsed VIOLA_NAME (with no name no log), for the frame verbs under the named target (link and unlink the driven) instance.
decision/clients-screen-peer-replies-to-code-words · Clients screen peer replies to code words and a fixed fault — The CLI and MCP clients screen a peer reply before using it: a refusal and its detail must be code words, a wrapper fault keeps only its code, a fixed message and closed data, and the screening adds no span and no line. · why: No path or upstream text may cross from a wrapper into a document, a tool result or a log.
decision/home-tree-check-before-start-log · Home-tree check before the start's log — The start of run, of viola revive and of viola adopt checks the home tree (the home, instance set, ledger, bin, plugin, pinned exe and plugin hooks) before its log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
decision/no-dialog-content-in-logs · Dialog content never logged — The question, the answer and the tool input never appear in a log line.
decision/no-last-assistant-message · last_assistant_message never logged — last_assistant_message never appears in a log line.
decision/no-params-in-channel-lines · No params or result bodies in channel lines — channel-request and channel-response lines never carry params or result bodies.
decision/no-text-in-telemetry · Sent text never in telemetry — The sent text never appears in a span or log line; only its length does.
decision/no-typed-keys-in-telemetry · Typed keys never in telemetry — The keys the human types never appear in a span.
decision/refused-check-writes-no-line · A refused check opens no process log — A cli verb whose strict-modes, tree or instance check refuses (exit 21) opens no process log and writes no line.
decision/release-from-driver-refused · A release from a driver is refused — A release that carries a string from is refused with -32602, and viola release forwards VIOLA_NAME, so a driver session's own viola release is refused too.
decision/revive-start-logs-as-run · viola revive start logs as run — The start arm of viola revive is not a cli verb and logs as process run once its instance check has passed.
decision/send-cut-mark-uninstrumented · Send cut mark uninstrumented — The cut mark a cut paste leaves is in-memory wrapper state that opens no span and writes no line of its own; its set shows as the cut send's send-refused paste-cut record and its clears as their own prompt-submitted, session-start and session-end records.
decision/send-line-correlation · Send lines correlate with channel lines — Every wrapper send-* line carries conn (or srv_conn) and rpc_id, so that pair on the send line equals (conn, corr) on the call's channel lines; the client-side send-refused carries neither.
contract/answer-refusal/not-delivered · driver-side answer refusal · not-delivered — channel-response{result_class:"refusal"} with no refusal word and no detail on the line, exit 13.
contract/answer-refusal/unverified-cli · driver-side answer refusal · unverified-cli — refusal unverified-cli, exit 12, the answer withheld.
contract/cli-exit-21/endpoint-lost · cli exit 21 process-exit · endpoint-lost — detail endpoint-lost with during call: the connection was lost after the connect; it is the only exit-21 detail whose during is call.
contract/cli-exit-21/instance-dead · cli exit 21 process-exit · instance-dead — detail instance-dead with during connect: no live snapshot, a replayed one included, or the connect failed.
contract/cli-exit-21/server-verify-failed · cli exit 21 process-exit · server-verify-failed — A refused euid, a server that is not the recorded wrapper, or a check that cannot be made opens no channel.request span, writes no channel-request line and ends process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}.
contract/cli-exit-21/socket-dir-refused · cli exit 21 process-exit · socket-dir-refused — A frame verb meeting a refused socket directory writes one process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"} at WARN inside its dispatch span, with no channel-request line and no frame.
contract/cli-exit-21/unwrapped-name · cli exit 21 process-exit · unwrapped-name — detail unwrapped-name with during connect: no instance directory.
contract/cli-exit/1-internal-error · send.client process-exit · 1 internal-error — exit_code 1 internal-error on a reply with neither a parseable result nor an error.code.
contract/cli-log-file/open · CLI process-log file · viola open — Logs as cli into the opened instance's cli-<name>.ndjson, its work inside the cli.open dispatch span.
contract/cli-log-file/switch · CLI process-log file · viola switch — Logs as cli into the named instance's cli-<name>.ndjson, a process-start and a process-exit inside the cli.switch dispatch span; a refusal ends process-exit with no detail, as send's does.
contract/cli-stderr/internal-error · CLI stderr · internal fault message — An internal fault prints exactly "error: internal error".
contract/cli-stderr/refusal-pair · CLI stderr · unable and hint pair — A refusal is printed to stderr as a fixed "unable:" line followed by a "hint:" line.
contract/confirm-window-close/failed-snapshot-write · run.confirm_window close · claimed session-start whose snapshot write fails — A /clear or /compact session-start line the window claims, whose snapshot write then fails, leaves the send unclaimed and does not close the window; the window's own deadline ends the send unconfirmed as send-refused not-delivered with detail no-prompt-submitted.
contract/instance-unreachable-json/strict-modes-failed · cli --json refusal document · strict-modes-failed — A refused instance check exits 21 with the instance pair on stderr, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, and sends no frame.
contract/pause-release/answer-after-wheel-record · pause and release methods · reply timing — pause and release answer only after the move's wheel record lands, -32603 if it did not.
contract/refusal-stderr-pair/socket-dir-refused · cli refusal stderr pair · socket-directory refusal — Printed on start and frame-verb refusals in plain ASCII with hint: last and no SGR, naming only the parsed ViolaName.
contract/release-refusal/invalid-params · release refusal lines · invalid-params — A from of another type, or any budget key (true, false, a string or null), is a plain -32602 (data null) after the release-from-driver check, with no release-from-driver line, no wheel moved and no record; its channel-response carries detail invalid-params on both sides at WARN.
contract/release-refusal/release-from-driver · release refusal lines · release-from-driver — One release-from-driver{corr:<rpc id>, conn, from (only a valid name), from_trust:"self-reported"} at INFO beside the -32602 reply with data {"reason":"release-from-driver"}; its channel-response carries detail release-from-driver on both sides at WARN.
contract/revive-preflight-detail/cwd-missing · viola revive preflight process-exit · cwd-missing — A preflight refusal adds detail cwd-missing before any child is started.
contract/revive-preflight-detail/no-session · viola revive preflight process-exit · no-session — A preflight refusal adds detail no-session before any child is started.
contract/revive-preflight-detail/session-live · viola revive preflight process-exit · session-live — A preflight refusal adds detail session-live before any child is started; the reading is skipped under --fork and passes on a failed read.
contract/revive-process-log/agents-probe · viola revive run process log · agents-probe — The session-live reading writes one process-start and process-exit{subject:"agents-probe"} pair, plus one parse-rejected{parser:"claude-agents-json"} on an oversize or malformed answer.
contract/run-process-log/process-exit-self · run process log · process-exit self — After the process-exit of the child the pump ends on, run emits process-exit{subject:"self", exit_code, duration_ms}; the process-exit of a child a switch replaces is followed by none.
contract/run-process-log/process-start-self · run process log · process-start self — process-start{subject:"self", service_name, version, os, pid, endpoint_kind}.
contract/run-start-failure-detail/already-live · run start process-exit · already-live — process-exit{subject:"self", exit_code:1, detail:"already-live"} for a live holder or a fresh replayed beat, with no path or pid.
contract/run-start-failure-detail/pinned-hash-mismatch · run start process-exit · pinned-hash-mismatch — process-exit{subject:"self", exit_code:1, detail:"pinned-hash-mismatch"}, with no path or pid.
contract/run-start-failure-detail/socket-dir-refused · run start process-exit · socket-dir-refused — A socket directory the start refuses is one process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"} before any snapshot, child or bind, on viola revive's start too; a bind failure keeps its own outcome.
contract/run-start-failure-detail/squatted-name · run start process-exit · squatted-name — process-exit{subject:"self", exit_code:1, detail:"squatted-name"}, with no path or pid.
contract/run-start-failure-detail/stale-heartbeat · run start process-exit · stale-heartbeat — process-exit{subject:"self", exit_code:1, detail:"stale-heartbeat"} for a stale holder, with no path or pid.
contract/send-log-fields/send-confirmed · confirmed send log fields · send-confirmed — send-confirmed{corr:<cursor>, confirmed, duration_ms}, confirmed false for unconfirmable, duration_ms from the send's arrival including the readiness gate's wait.
contract/send-log-fields/send-issued · confirmed send log fields · send-issued — send-issued{corr:<cursor>, from, text_bytes}.
contract/send-refused-client/control-character · client-side send validation refusal · control-character — Made before any frame: send-refused{corr:null, refusal:"not-delivered", detail:"control-character", side:"client"}, one line per deciding side, never the text.
contract/send-refused-client/empty-text · client-side send validation refusal · empty-text — An empty typed text, read after control-character and ahead of both wheel reads; a wrapper deciding it writes its own send-refused{side:"wrapper"} with the same pair, corr, conn and rpc_id, and no send-issued.
contract/send-refused-detail/input-not-ready · wrapper send-refused detail · input-not-ready — send-refused{corr:<cursor>, refusal, detail:"input-not-ready", side:"wrapper", wheel}.
contract/send-refused-detail/no-prompt-submitted · wrapper send-refused detail · no-prompt-submitted — Confirm-window expiry is send-refused{detail:"no-prompt-submitted"} for ordinary, /clear, /compact and echo-confirmed local command sends alike, with no new detail.
contract/send-refused-detail/paste-cut · wrapper send-refused detail · paste-cut — A paste cut at its bound refuses not-delivered paste-cut after send-issued, with the send's cursor; a send refused on a standing cut mark refuses paste-cut with no cursor and nothing issued.
contract/send-refused-detail/turn-running · wrapper send-refused detail · turn-running — Covers a running turn the wrapper knows of before the paste, another send in flight, or a switch standing from its request until the new child's session-start, with no new field; the refusal has no cursor (corr the end offset at refusal), no send-issued, and the client exits 13.
contract/send-refused/human-typing · wrapper send-refused · human-typing — send-refused{corr:<cursor>, refusal:"human-typing", detail:null|"manual-pause", wheel:"human", side:"wrapper"}, with the client process-exit{exit_code:10}.
contract/wait-log-fields/channel-response-outcome · wait and last log fields · channel-response outcome — For wait only, outcome is the waking kind or timed-out, derived once over the shared wake set; the client side alone writes instance-unreachable when the reply never arrived; last carries no outcome and no result body reaches either line.
contract/wheel-span-attributes/run-wheel-transition · wheel span attributes · run.wheel_transition — wheel_from, wheel_to (human|driver), cause (human-input|manual-pause|release); a point-in-time span with a static name, opened once per wheel change on the wheel's worker thread.
contract/wheel-spans/span-chain · wheel spans · span chain — run.wheel_transition (INTERNAL) › channel.dispatch(send) refused; pause.client or release.client (CLIENT) › channel.request(pause|release) → channel.dispatch(pause|release).
decision/cli-manual-dispatch-span · Manual dispatch span per cli verb — Each short-lived cli verb opens one manual dispatch span of its own. · why: clap 4.6.7 has no tracing integration.
decision/error-chain-in-detail-log-only · Full error chain only in the cli detail log — The full anyhow error chain of a cli verb goes to the cli detail log only.
decision/launch-line-untraced · Launch line has no trace of its own — The launch line written at every run, revive and adopt start and at every switch is log-only product audit data: it wakes no wait and opens no span, writes no process-log line and no terminal byte of its own.
decision/lineage-line-untraced · Lineage line has no trace of its own — The lineage line written at every run, revive and adopt start is log-only product audit data: it wakes no wait and writes no process-log line, span or terminal byte of its own.
decision/no-raw-snapshot-reads · Product readers read the snapshot through replay — Every product reader of the snapshot uses the replaying read; no product reader reads the snapshot raw.
decision/server-refusal-inside-root-span · Server-verification refusal inside the root span — A frame verb's or the hook's server-verification refusal happens inside its scenario's root span. · guard: request_refuses_an_unverified_server_inside_its_dispatch_span; hook_refuses_an_unverified_server_inside_hook_handle
decision/socket-dir-refusal-inside-root-span · Socket-directory refusal inside the root span — A frame verb's or the hook's socket-directory refusal happens inside its dispatch span or hook span. · guard: connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span
decision/transcript-echo-read-uninstrumented · Transcript echo read uninstrumented — The send's transcript echo read, the end offset before the paste and the tag count after it, opens no span and writes no line or span field of its own; its result shows only as the send-confirmed line inside run.confirm_window.
decision/unreadable-ledger-means-unverified · Unreadable capability ledger means an unverified CLI — A capability ledger that is missing a stamp, unreadable, malformed or refused by the strict-modes check is treated as an unverified CLI (cli_verified false).
decision/window-line-untraced · Window line has no trace of its own — The window line a start under --origin writes after its launch line, and a switch writes again after its own launch line, is log-only product audit data: it wakes no wait, opens no span and is never copied into a process-log line.
contract/answer-link-record/failure · wrapper answer handling · failed link record — A link record that fails answers -32603: the answer is not taken, the dialog stays held and no dialog-answered line is written.
contract/cli-exit/20-wrapper-fault · send.client process-exit · 20 wrapper-fault — Exit 20 carries detail wrapper-fault.
contract/confirm-window-close/clear · run.confirm_window close · /clear — On a verified CLI, closes on the session-start whose cause is clear and whose agent_session_id differs from the id the send slot remembers; no prompt-submitted claims the send.
contract/confirm-window-close/compact · run.confirm_window close · /compact — On a verified CLI, closes on the session-start whose cause is compact and whose agent_session_id equals the id the send slot remembered; nothing remembered claims nothing, and no prompt-submitted claims the send.
contract/confirm-window-close/context · run.confirm_window close · /context — On a verified CLI, closes on a new local-command-stdout tag in the session transcript within the fallback window, with no prompt submitted; on an unverified CLI, or before the session transcript exists, the send reads unconfirmable.
contract/confirm-window-close/ordinary-send · run.confirm_window close · ordinary send — Closes on the prompt-submitted{origin:"driver"} whose prompt equals the typed text, or is non-empty and equals the typed text with a trailing run of ASCII space, TAB, CR or LF removed, or on window expiry.
contract/diag-line-schema/os · diag-line v1 schema · os — os is closed to linux.
contract/dialog-log-fields/dialog-answered · dialog log fields · dialog-answered — dialog-answered{corr:<dialog_id>, dialog_kind, from, from_trust, deadline_hit, duration_ms}; from and from_trust only on an answer that carried a from.
contract/events-log/link-before-send-issued · events log · implicit link and send-issued — A new pair's link product line stands directly before that send's send-issued, whose cursor is the offset its own line starts at; a link record that fails answers -32603 with nothing issued, nothing typed and no send-issued.
contract/run-process-log/common-fields · run process log · common fields — Every run line carries the common fields with process="run" and corr null.
contract/run-process-log/process-start-claude-child · run process log · process-start claude-child — On success, process-start{subject:"claude-child", child_pid, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept}.
contract/run-process-log/version-probe · run process log · version-probe — process-start and process-exit{subject:"version-probe", child_exit_status, duration_ms}.
contract/send-log-fields/channel-request · confirmed send log fields · channel-request — channel-request{corr:<id>, conn, method, sender}.
contract/send-log-fields/hook-invoked · confirmed send log fields · hook-invoked — hook-invoked{hook_event:"user-prompt-submit", corr:null} for an ordinary send; hook_event session-start for a /clear or /compact confirmed by its session; an unconfirmable send, or a local command confirmed by its transcript echo, waits for no hook line.
contract/send-span-attributes/text-bytes · confirmed send span attributes · text_bytes — text_bytes, on pty.paste_write and send-issued alike, is the typed text's length: the sent text with each CR LF pair and each other CR as one LF, trailing CR and LF removed; pty.paste_write also carries paste_mode="bracketed".
contract/state-recovered/scanned · state-recovered line · scanned — From the board read, the line carries instance (the caller) and scanned (the scanned instance's name when it parses, absent otherwise); with no VIOLA_NAME no log is open and the line goes nowhere, bar viola console, which writes it into console.ndjson with no instance.
contract/wait-log-fields/channel-request · wait and last log fields · channel-request — channel-request{corr:<id>, conn, method:"wait"|"last"} on both sides, plus after and timeout_ms for wait only and only as u64 values.

## 5-metric-coverage · ## 5. Metric Coverage · twin metric-coverage
decision/metrics-derived-from-logs · Metrics derived from logs — Every metric is logical, computed by jq or the harness over the JSON-per-line logs or from hyperfine output, never by OTel or metrics-crate instruments. · why: There is no reader or exporter to flush instruments.

## metric-coverage · ## 5. Metric Coverage · twin 5-metric-coverage
contract/cli-process-exit/server-verify-failed · CLI process-exit diagnostic line · server-verify-failed — The CLI's server-verification refusal writes one process-exit with subject self, exit_code 21, detail server-verify-failed and during connect at WARN in cli-<name>.ndjson, with no pid, start time, path or endpoint; MCP's wait tool writes no line for it.
decision/panic-lines-stay-zero · Panic lines stay at zero — Gate G2 counts the panic lines of the home-level role files and fails on any; no viola.panic metric is derived.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/content-detail-in-instance-file · Content-bearing detail in the instance detail file — Content-bearing detail goes only to the per-process detail file inside the instance's diagnostics directory.
decision/file-only-per-role-log-sinks · File-only log sinks per role — Diagnostics are written to files only, one sink per role, under the home's diagnostics directory.
decision/ipc-calls-logged-without-arguments · IPC calls logged without arguments — IPC params and result bodies are never recorded, not even as a hash or digest.
decision/json-to-file-only-log-mode · JSON-to-file as the only log mode — JSON to file is the only log mode: there is no agent-mode flag and no stdout pretty sink on any role.
contract/child-spawn-log/claude-agents-json-parse-failure · child and shell spawn log events · claude agents --json parse failure — A claude agents --json parse failure gives one parse-rejected{parser:"claude-agents-json"} line and the value "unknown".
contract/child-spawn-log/subject · child and shell spawn log events · subject — process-start and process-exit lines for child spawns carry a subject from a closed set.
contract/diag-line-fields/console-frame · diagnostic line additive fields · console-frame — Logged on the first draw and then only when one of its fields changes, with cols, rows, shown and hidden (counts) and fits (boolean), all required; an unread terminal size logs 0x0, not fitting.
contract/diag-line-fields/console-key · diagnostic line additive fields · console-key — Logged once per key read with key, one of quit, interrupt or unbound, required; the key's bytes never reach a line.
contract/log-levels/error · log levels · error — Used for a panic, a process-exit that carries a detail, and a channel-response with error_code -32603.
contract/log-levels/info · log levels · info — INFO carries all other boundary, lifecycle and state-transition events, including refusals, which are normal product outcomes.
contract/log-levels/warn · log levels · warn — WARN carries state-recovered; parse-rejected; hook-decision with a fail-open detail; process-exit with exit 21; channel-response with -32600 or -32602.
contract/log-line/corr · every log line · corr — Set per the tests' table and copied unchanged as a JSON number or string; null is represented by the key's absence.
contract/log-line/event · every log line · event — A value of the closed kebab-case event enum.
contract/process-exit-detail/exit-1-focus · process-exit detail · exit 1 from viola focus — viola focus ends exit 1 with one process-exit detail: window-not-found (no window of the session, nothing dispatched), unsupported (no hyprctl reachable, nothing spawned), unreachable (hyprctl reachable, no compositor answering), or the hyprctl seam's oversize, malformed or refused.
decision/viola-crates-log-at-info · Viola crates log at INFO — The per-module log filter sets the viola, viola_core, viola_pty, viola_channel, viola_state, viola_agent_claude and viola_mcp targets to INFO, or DEBUG via diagnostics_level.
contract/log-line/instance · every log line · instance — The ViolaName; the key is absent when there is none.
contract/pty-seam-log/paste · PTY seam log events · paste — A paste is logged with text_bytes only.
contract/pty-seam-log/spawn · PTY seam log events · spawn — A spawn gives process-start{subject:"claude-child"}.
contract/start-trust-log/trust-record · the start's trust-record log line · trust-record — Each wrapped start of viola run, viola revive and viola adopt on a verified CLI writes exactly one INFO trust-record line to the run role file, its detail one of a closed set of 16 start words, carrying no path, project key or config value; the word never refuses the start.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/console-agents-probe-volume · Console agents-probe log volume accepted — viola console reads claude agents on its own thread at most every 5 s and writes an agents-probe pair on each read, up to 720 pairs an hour into console.ndjson; its other lines are written only on a real change. · why: The console's log volume is narrowed to these pairs and the lines of real changes, and is accepted as built. · provisional
contract/diag-detail/pinned-hash-mismatch · diagnostic detail words · pinned-hash-mismatch — A filesystem refusal word written on process-exit with exit 1.
contract/diag-detail/socket-dir-refused · diagnostic detail words · socket-dir-refused — A closed detail on process-exit and hook-decision for a socket dir refused as a symlink, no directory, foreign owner or mode bit: the start of run, revive and adopt writes process-exit exit 1, frame verbs process-exit exit 21 during connect at WARN, viola hook a hook-decision at WARN; MCP none.
contract/diag-line-fields/dialog-events · diagnostic line additive fields · dialog-raised / dialog-answered — dialog_kind, hook_event, from, from_trust, deadline_hit, duration_ms.
contract/diag-line-fields/panic · diagnostic line additive fields · panic — panic_location (workspace-relative, or <crate>-<ver>/src/... for dependencies) and thread; payload and backtrace go to the detail file only.
contract/diag-line-fields/process-exit · diagnostic line additive fields · process-exit — subject, exit_code (self), child_exit_status, exit_source (handle-wait or kill-fallback), detail, during (connect or call), duration_ms.
contract/diag-line-fields/process-start · diagnostic line additive fields · process-start — subject (one word of the diag-line subject enum), service_name, version, os, pid, child_pid, endpoint_kind, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept (names only).
contract/diag-line-fields/release-from-driver · diagnostic line additive fields · release-from-driver — conn, from (only when a valid ViolaName), from_trust (self-reported).
contract/diag-line-fields/send-events · diagnostic line additive fields · send-issued / send-confirmed / send-refused — from, from_trust, text_bytes (normalised typed length), confirmed, refusal, detail, side (client or wrapper), wheel, duration_ms, and wrapper side only conn / srv_conn and rpc_id.
contract/diag-subject/agents-probe · diag-line subject enum · agents-probe — Subject of the process-start and process-exit pair of the claude agents read, written by viola list, the session-live reading of viola revive and viola adopt, verify's Run B, the list tool of viola mcp and viola console's agents thread into their role files.
contract/diag-subject/terminal · diag-line subject enum · terminal — Subject of the one process-start, with no process-exit, of each foot terminal window that viola open or viola up spawns, written into that verb's cli role file; the terminal outlives the verb.
contract/parse-rejected-detail/control-character · parse-rejected detail · control-character — Listed in the diagnostics schema for the paste-text parser, but never emitted: a control character in a paste is refused as send-refused not-delivered.
contract/parse-rejected-detail/malformed · parse-rejected detail · malformed — Any parser.
contract/parse-rejected-detail/oversize · parse-rejected detail · oversize — channel-frame beyond MAX_FRAME, hook-stdin, mcp-stdin past MAX_FRAME, vt100-feed when the bounded queue drops a copy (one line per poisoning episode), claude-agents-json past MAX_FRAME, hyprctl-json past MAX_FRAME.
contract/parse-rejected-detail/panicked · parse-rejected detail · panicked — vt100-feed under catch_unwind.
contract/parse-rejected/claude-agents-json · parse-rejected diagnostic line · parser claude-agents-json — A claude agents --json parse failure, oversize or malformed, gives one parse-rejected line with parser claude-agents-json and the value unknown.
contract/process-exit-detail/exit-1 · process-exit detail · exit 1 — already-live, stale-heartbeat, squatted-name, pinned-hash-mismatch, internal-error, socket-dir-refused, from revive's preflight no-session, session-live, cwd-missing, and from viola open terminal-not-found and not-live; each one process-exit line with no child started, bar not-live's terminal.
contract/process-exit-detail/exit-21 · process-exit detail · exit 21 — instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused.
contract/process-exit-detail/server-verify-failed · process-exit detail · server-verify-failed — The eight CLI frame verbs write it at WARN with exit 21, during connect, on a server of another euid, not the recorded wrapper, or an unverifiable check, with no pid, start time, path or endpoint.
contract/send-refused-detail/empty-text · send-refused detail · empty-text — Written for an empty typed text on send; the word adds no field, and send-refused detail stays an open string in the diag-line schema.
contract/send-refused-detail/refusal-reason-words · send-refused detail · RefusalReason detail words — Refusal details are the arch RefusalReason details verbatim: manual-pause, input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text, paste-cut.
decision/revive-preflight-order · Revive start preflight order — Revive's start preflight reads the holder, then the session, then session-live, then the directory, and stops at the first refusal.
decision/session-live-check-fails-open · Session-live check fails open — A failed, oversize or malformed claude agents read passes the session-live check of revive's and adopt's starts; the check is skipped under revive's --fork.
decision/symlink-skip-logged-as-count · Skipped state entries logged as a parse-rejected count — A scan of instance state that skips a symlinked or malformed entry writes one parse-rejected line with parser state-entry, detail symlink-ignored or malformed and a count, once per scan, at warn, into the scanning process's role file, naming no entry and no link target. · why: A skipped symlink has no exit or decision to carry it. · except: viola up, which never takes the strict board read, writes no parse-rejected state-entry line; viola console writes the line only when the scan's counts change.
contract/diag-line-fields/parse-rejected · diagnostic line additive fields · parse-rejected — parser (claude-agents-json, vt100-feed, config-json, ledger-stamps, hook-stdin, channel-frame, mcp-stdin, state-entry, hyprctl-json), detail, count.
contract/diag-line-fields/trust-record · diagnostic line additive fields · trust-record — A fixed message and a required detail drawn from the 16 start words; trust-record stands in the event enum of both the diag-line and the detail-line schema.
contract/diag-subject/claude-child · diag-line subject enum · claude-child — One value of the closed subject enum of the diag-line schema.
contract/diag-subject/hyprctl-probe · diag-line subject enum · hyprctl-probe — Subject of the process-start and process-exit pair that the Hyprland seam writes around each hyprctl spawn.
contract/diag-subject/self · diag-line subject enum · self — One value of the closed subject enum of the diag-line schema.
contract/parse-rejected/hyprctl-json · parse-rejected diagnostic line · parser hyprctl-json — A hyprctl reply that is oversize or malformed gives one parse-rejected line with parser hyprctl-json, written by the Hyprland seam and carrying codes only.

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/boundary-errors-end-in-logged-line · Crate-seam errors end in a logged line — Every typed error that crosses a crate seam ends in a logged process-exit detail, channel-response error, hook-decision detail or send-refused line, carrying codes only at WARN or ERROR.
decision/error-correlation-without-trace-ids · Error correlation without trace ids — Errors correlate by corr, conn, process and instance, plus timestamp, not by trace_id or span_id.
decision/hand-written-panic-capture · Hand-written panic capture — Panic capture is a hand-written std panic hook plus std catch_unwind, with no panic-reporting crate.
decision/panic-exit-decided-at-catch-site · Panic exit decided at a catch site — The panic hook only writes; the exit is decided by one catch_unwind in the root binary's main around everything after the hook is set, so no role ends through an uncaught unwind.
decision/panic-hook-silent-on-stderr · Panic hook installed first, silent on stderr — Viola's panic hook is installed by main's first statement, never calls the default hook and never writes to stderr. · why: The hook contract and the run terminal both forbid it.
decision/panic-line-bypasses-tracing · Panic line bypasses tracing — The panic hook writes its one JSON line with a single write to a pre-opened file, not through the tracing subscriber. · why: No re-entrancy if the panic happened inside formatting.
decision/serde-errors-become-internal-error · Serde error chains replaced by internal error — At dispatch, an error chain whose source is a serde_json or serde_path_to_error error is replaced by the fixed message "internal error" before it reaches stderr, --json, MCP isError or channel error.data. · why: A scrubbing layer at the source.
decision/typed-errors-fixed-display · Typed errors with fixed Display messages — Every typed error has a fixed-message Display, written with thiserror, and holds no fields with paths or payloads. · except: PtyError's Display is hand-written, as viola-pty takes no thiserror.
decision/unhandled-panics-always-captured · Unhandled panics always captured — Every unhandled panic is captured as the one-line event "panic".
decision/vt100-panics-contained · vt100 panics contained — vt100 panics are caught around each feed and resize on run's feed thread; each poisoning episode gives exactly one parse-rejected{parser:"vt100-feed"} at WARN, and run continues.
contract/panic-line/corr · the panic line · corr — corr is never written on the panic line.
contract/panic-line/instance · the panic line · instance — instance is omitted when none resolved, the same key-absence null encoding obs_event! uses.
contract/panic-line/keys · the panic line · keys — The panic line holds timestamp, level "ERROR", target "viola::panic", message "panic", event "panic", process, instance, panic_location and thread.
contract/panic-outcome-by-role/console · panic outcome by role · console — After a dispatch error or caught panic, console restores the terminal first, then logs process-exit{subject:"self", exit_code:1, detail:"internal-error"} in console.ndjson, writes error: internal error once on stderr and exits 1; the catch site classes it as cli.
decision/anyhow-at-dispatch-edge-only · anyhow at the dispatch edge only — anyhow is used only at the root binary's dispatch edge and its catch-site reporter.
decision/full-error-chain-in-detail-file-only · Full error chain in the detail file only — The full error chain is written as a one-line JSON array chain:[...] only to the instance's per-process detail file, and only when an instance resolves.
decision/panic-backtrace-in-detail-file · Panic backtrace in the detail file — When an instance resolves, the panic hook also writes the payload and a raw, never-symbolised backtrace as one line to the process's detail file, not gated by RUST_BACKTRACE. · why: Env vars are not a configuration channel.
decision/panic-unwind-in-all-profiles · panic = unwind in every profile — Every Cargo profile keeps panic = "unwind". · guard: G3
contract/detail-file-line/correlation-keys · the detail-file error line · correlation keys — The detail-file line for an error carries the same event, corr and conn as its home-level counterpart, so jq joins them.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
decision/exit-decided-at-catch-site · Exit decided at a catch site, not the panic hook — The panic hook only writes; everything after the hook is set runs inside one catch_unwind in main, so no role ends through an uncaught unwind.
decision/peer-replies-screened-as-code-words · Peer replies screened as code words — A peer reply's refusal and detail are kept only as code words (1-64 bytes of [a-z0-9-], letter first); an unknown code word stays visible and exits 14. · why: by design
decision/single-sanitising-fault-reader · One sanitising reader of peer faults — A peer's fault is read only through one sanitiser: five known codes read fixed messages with data admitted only in validated shapes, a code outside them reads wrapper fault, and a fault without an integer code reads nothing.
decision/vt100-feed-panic-contained · vt100 feed panic contained — A panic in a vt100 feed or resize is caught on the feed thread, poisons the screen model until the host size changes, writes one parse-rejected per episode, and run continues.
contract/answer-send-exit-2/stderr-line · viola answer and viola send exit 2 · stderr line — Exit 2 writes the fixed unusable-response or unusable-text line to stderr whatever --json says, with no document.
contract/cli-internal-error/human-line · cli internal error · error: internal error — Without --json the cli catch site prints exactly error: internal error; run, revive's start, adopt's start, mcp and hook print neither.
contract/exit-21-document/detail · exit 21 document · detail — One of six closed words: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused.
decision/not-a-directory-refusal-message-fixed · NotADirectory refusal message is fixed and pathless — The NotADirectory strict-modes refusal reads the fixed message "a trusted state path is not a directory" and names no path. · why: The security plan's fixed-message rule keeps paths out of error text, and the fixed-messages unit test pins every refusal's text.
decision/refused-diagnostics-path-is-a-failed-open · A refused diagnostics path is a failed open — A refused diagnostics folder or file is exactly a failed open: the role file does not open, and a detail line is dropped with nothing written and nothing on stderr. · why: The hook carries on with no subscriber, the other roles end as an internal error, and no line is ever written through a widened folder or a symlink.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/content-canary-never-home-level · Content canary never in home-level files — The content canary never appears in a home-level diagnostics ndjson file.
decision/content-only-in-detail-file · Content-bearing records only in the detail file — Content-bearing records go only to the instance-scoped detail-<process>.ndjson file.
decision/conversation-text-only-in-detail-file · Conversation text only in the instance detail file — Prompt, assistant, tool-input, plan and dialog text never reach a home-level line, only an instance's per-process detail file inside a chain or drift report.
decision/default-deny-log-fields · Default-deny for logged fields — A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure. · guard: gate G4 against schemas/diag-line.v1.json
decision/fixed-error-messages · Fixed error messages — Error Display texts are fixed, and serde-sourced error chains are mapped to fixed messages at dispatch.
decision/home-level-lines-path-free · Home-level lines path-free — Home-level log lines are path-free: file fields are basenames, panic_location is workspace-relative and source file recording is off; absolute paths appear only in detail files. · why: The OS username inside absolute paths is the only PII in scope.
decision/low-class-metadata-as-typed-fields · Low-class metadata as typed fields — Low-class metadata is logged as typed fields, never inside detail codes.
decision/schema-top-level-unevaluated-properties · Default-deny schema via top-level unevaluatedProperties — The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists. · why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then · guard: contract_diag_schema
decision/source-allow-list-logging · Source allow-list for logged fields — Spans skip all arguments and declare fields explicitly, and events go through obs_event! with explicit typed fields only; this source allow-list is the primary PII control. · guard: disallowed-macros; raw_event_hits; G1 bare instrument
decision/veil-redact-second-layer · veil Redact as the second redaction layer — veil's derive Redact is the second redaction layer, so a stray value capture prints [REDACTED] instead of content.
decision/claude-agents-rows-never-logged · claude agents rows never logged verbatim — claude agents --json rows are never logged verbatim, only parse-rejected counts.
decision/endpoint-and-pinned-bin-never-logged · endpoint and pinned_bin never logged — The endpoint and pinned_bin values are never logged.
decision/no-regex-scrubbers · No regex scrubbers — Sensitive data is never scrubbed by regular expressions. · why: viola's sensitive data is known by type
decision/payload-types-derive-redact · Type-level redaction of payload types — Every channel, hook and MCP payload type that carries content derives veil Redact, so Debug never reveals content; a payload type with no content field derives none.
contract/detail-line/backtrace · the detail-file line · backtrace — One of exactly four content fields a detail-file line may carry: the panic's backtrace.
contract/detail-line/chain · the detail-file line · chain — One of exactly four content fields a detail-file line may carry: the full error chain.
contract/detail-line/event · the detail-file line · event — Required on every detail-file line, as on home-level lines.
contract/detail-line/instance · the detail-file line · instance — Required on every detail-file line, as on home-level lines.
contract/detail-line/level · the detail-file line · level — Required on every detail-file line, as on home-level lines.
contract/detail-line/message · the detail-file line · message — Required on every detail-file line, as on home-level lines.
contract/detail-line/panic-location · the detail-file line · panic_location — Allowed on panic detail lines only, copied from the home-level panic line so both lines join without the payload.
contract/detail-line/panic-payload · the detail-file line · panic_payload — One of exactly four content fields a detail-file line may carry: the panic's payload.
contract/detail-line/process · the detail-file line · process — Required on every detail-file line, as on home-level lines.
contract/detail-line/target · the detail-file line · target — Required on every detail-file line, as on home-level lines.
contract/detail-line/thread · the detail-file line · thread — Allowed on panic detail lines only, copied from the home-level panic line so both lines join without the payload.
contract/detail-line/timestamp · the detail-file line · timestamp — Required on every detail-file line, as on home-level lines.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/agent-rows-never-verbatim · Agent rows never logged verbatim — claude agents --json rows are untrusted and never logged verbatim, only parse-rejected counts.
decision/canary-only-in-detail-files · Canary only in detail files — Every E2E and integration input is synthetic and embeds a fixed canary, which may appear in detail files but never in a home-level diagnostics file. · guard: mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line
decision/closed-protocol-codes · Closed protocol codes — Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value. · guard: answer_invalid_params_names_its_cause_and_no_param
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — thiserror messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
decision/lineage-names-confined · Parent and role confined to product surfaces — A start's parent and role names appear only in events.ndjson, the snapshot, the viola list board, list --json and the MCP list result, each parent re-parsed as a viola name and each role as a code word and dropped otherwise, never in a process-log or detail line.
decision/prompt-and-output-never-in-home-logs · Prompt and assistant output never in home logs — Prompt text and assistant output, last_assistant_message included, never appear in a home-level process-log line; they reach an instance detail file only inside a chain or drift report, and only text_bytes is logged.
decision/typed-ids-never-in-detail · Ids logged as typed fields — pid, child_pid, agent_session_id and started_at are logged as typed fields, never inside detail codes.
decision/veil-redaction · veil derives redaction — veil 0.3.0's derived Redact is what redacts the payload fields of contract types, and instrumented spans skip all arguments.
decision/endpoint-paths-never-logged · Endpoint and pinned binary never logged — The snapshot endpoint and pinned_bin are never logged, only endpoint_kind.
decision/server-verify-refusal-names-nothing · Server-verify refusal names nothing — A server-verification refusal carries no pid, start time, path or endpoint in any output or log line. · guard: security_negatives_server_identity
decision/socket-dir-refusal-names-nothing · Socket-directory refusal names nothing — A socket-directory refusal carries no path, pid or uid in its lines and documents, and its human pair names only the parsed instance name. · guard: security_negatives_socket_dir

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/g1-bare-instrument-gate · G1 bare-instrument gate fails closed — Every #[instrument] must have skip_all as its first argument; the G1 gate passes only on rg exit 1 (no match), so a violation, a regex or I/O error or a missing rg fails the CI step. · guard: G1

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/instrument-requires-skip-all · Instrument attribute must skip all arguments — Every #[instrument] in the workspace takes skip_all as its first argument; a bare #[instrument] fails the build. · guard: gate G1
decision/no-raw-tracing-macros · No raw tracing macros outside the obs module — Raw tracing info, warn, error, debug and trace macros and raw event! are refused outside viola_core::obs. · guard: clippy -D warnings disallowed-macros; raw-event! grep and ban probes in scripts/lint-probes.sh
decision/panic-unwind-required · Panic strategy is unwind — Every Cargo profile keeps panic = "unwind"; no abort strategy in profiles, rustflags, env or workflows. · why: abort turns every catch_unwind into an abort, so hook would die by SIGABRT instead of exiting 0 · guard: gate G3

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/correlation-propagation · Correlation propagation — Every channel call logs corr and conn on both sides; a send line from send-issued on carries the send cursor, a wrapper refusal before it the events.ndjson end offset as corr, and a client-side refusal no corr; every dialog line carries dialog_id.
decision/deterministic-emission · Deterministic event emission — Events key on hook events and events.ndjson offsets; nothing is derived from screen content. · except: The readiness gate reads the vt100 screen before a paste and can refuse a send as input-not-ready; a local command's echo confirmation keys on its transcript's stdout tag count.
decision/diagnostics-files-mode-0600 · Diagnostics files are 0600 — A diagnostics file whose mode is not 0600 counts as an SLO invariant failure.
decision/lint-probes-failure-fails · Lint-probes failure is a failure — A lint-probes failure (a ban that did not fire, a control that did not pass, or a raw event! outside the core obs module) counts as an SLO invariant failure.
decision/module-boundary-error-logging · Errors logged once at module boundaries — Every typed error that crosses a crate seam is logged once at WARN or ERROR with a closed detail or error_code; the full chain goes only to the instance detail file.
decision/print-and-macro-lint-failure-fails · Print and macro lint failure is a failure — A clippy disallowed-macros, print_stdout, print_stderr or dbg_macro failure counts as an SLO invariant failure.
decision/schema-conformance-failure-fails · Schema-conformance failure is a failure — An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.
decision/zero-unlogged-panics · Zero unlogged panics — Every panic in run, hook, mcp, cli and console after init step 4 produces exactly one single-line ERROR panic record in its home-level role file, with no multi-line backtrace there, and hook still exits 0; retry-once policies for panic handling are forbidden. · guard: G2 · except: the bounded exemptions where no file exists, so no line can be written; G2 alone leaves out the fake-agent seam's deliberate panic from its count.

## slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin 10-slo-invariants-telemetry-budgets
decision/no-retry-on-panic · No retry on panic — Retry-once policies for panic handling are forbidden. · why: Parallel to the tests' retries = 0.
decision/panic-count-home-level-only · Panic count reads home-level files only — The exactly-one panic rule and gate G2 count home-level role files only; the companion panic line in a detail file is excluded. · guard: gate G2 · except: G2 alone leaves out the fake-agent seam's deliberate panic, whose line is still written exactly once.

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/backtraces-in-instance-detail-only · Backtraces in the instance detail file only — Backtraces are written as one-line JSON arrays in the instance detail file only.
decision/closed-event-and-corr-on-every-line · Closed event enum on every line, corr where correlated — Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
decision/corr-set-explicitly-by-caller · corr set explicitly by the caller — The caller sets corr explicitly as a typed field on the events that carry one.
decision/default-panic-hook-never-runs · Default panic hook never runs — The default panic hook is never called or allowed to run. · why: Its stderr output breaks hook and the run terminal.
decision/event-values-closed · event values closed — No event value is added without a Decisions Log entry.
decision/explicit-corr-conn-instance · corr, conn and instance passed explicitly — corr, conn and instance are passed as explicit values across threads or tasks, never carried by span context. · why: Spans are not serialized.
decision/field-allow-list-default-deny · Field allow-list is default-deny — No field is logged unless named in the field catalog.
decision/must-trace-paths-always-traced · Must-trace paths always traced — Spans and events are never skipped on the 6 must-trace paths. · why: The agent loses its ability to debug without them.
decision/no-bare-instrument · No bare #[instrument] — Instrumentation always uses #[instrument(skip_all, fields(...))], never a bare #[instrument]. · why: A bare #[instrument] records every argument with Debug and leaks send text, hook.dialog payloads and tool input.
decision/no-hot-path-instrumentation · No hot-path instrumentation — The PTY pump read loop and the vt100 feed get no per-byte or per-chunk span or event; only gate outcomes are logged.
decision/no-info-in-hot-loops · No info logging in hot loops — Nothing is logged at info inside hot loops such as the PTY pump; counts go on the boundary event instead.
decision/no-instrument-err-ret-on-sensitive · No instrument err or ret on sensitive results — #[instrument(err)] and #[instrument(ret)] are never used where the error or return can carry a serde or serde_path_to_error source or user content.
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
decision/explicit-subscriber-writer · Explicit tracing-subscriber writer — with_writer is always set explicitly on the tracing-subscriber. · why: The default tracing-subscriber writer is stdout.
decision/low-cardinality-span-names · Low-cardinality span names — Span names never contain instance names, dialog ids or paths.
decision/no-retry-once · No retry-once policies — No retry-once policy is added for panics or flaky telemetry assertions.
decision/no-serde-source-in-error-output · No serde source in printed errors — An anyhow chain that still contains a serde_json or serde_path_to_error source is never printed to stderr, --json, MCP isError or channel error.data.
decision/service-identity-from-constants · Service identity from compiled constants — Service name and version come from viola-core constants, never from env vars such as OTEL_SERVICE_NAME or OTEL_RESOURCE_ATTRIBUTES; any future OTel Resource starts from an empty builder.
decision/transcripts-not-a-contract · Claude Code transcripts are not a contract — Claude Code transcripts are never read as a contract. · except: A local command's stdout tag in the session transcript, counted past the end recorded before the paste, is read as that send's echo confirmation; no transcript byte is kept.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/content-only-in-instance-detail-file · Content only in instance-scoped detail files — Content-bearing detail goes only to the instance's detail-<process>.ndjson file; home-level log files are codes-only and carry no user content, token or CLAUDE* value. · why: security allows user-derived content only in instance-scoped diagnostics
decision/diag-schema-matches-event-enum · Diagnostic line schema matches the event enum — A hand-maintained JSON Schema of the process-log line is committed, and its event enum must equal the ObsEvent variants, asserted by a tests-owned check.
decision/fixed-error-display · Fixed error Display text — Typed errors have fixed Display text that carries no interpolated content.
decision/instrument-skip-all-everywhere · Spans skip all arguments — Every instrumented function uses skip_all with explicit fields, so no argument is recorded implicitly. · guard: G1
decision/obs-event-single-macro · One obs_event macro for every line — Every log line is emitted through the one obs_event macro defined in viola-core, which expands to tracing's event at the caller, so viola-core stays I/O-free and tracing-free.
decision/raw-tracing-ban · Raw tracing macros banned — The tracing level macros are banned by clippy disallowed-macros and raw event calls by a fail-closed grep outside the obs module, so only obs_event emits lines. · why: clippy reports a disallowed macro expanded inside an exported macro at the caller, so no allow inside obs_event can exempt it · guard: clippy disallowed-macros and scripts/lint-probes.sh
decision/veil-redact-payload-types · veil redacts payload types — Payload types derive veil's Redact, and cargo deny bans veil's toggle feature. · guard: cargo deny veil toggle ban · except: A payload type with no content field derives neither veil Redact nor Debug, because veil 0.3.0 refuses a Redact derive with no redact field.
decision/liveness-transitions-not-logged · Liveness transitions not logged — No process-log line carries a liveness transition; liveness is exposed only by readers such as list --json. · why: no process watches another instance's heartbeat
decision/single-service-identity-source · One source for service name and version — The core crate's service name and version constants are the single source for process-start version, sender and writer, with the workspace version shared by every member.

## log-file-location · key of ## 3. Observability Harness Contract
contract/diagnostics-files/home-resolution · diagnostics files · home resolution — The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
contract/diagnostics-files/paths · diagnostics files · process log paths — Process logs are written to <home>/diagnostics/ as run-<name>, hook-<name>, mcp, cli-<name> and console .ndjson files, and detail to <home>/instances/<name>/diagnostics/detail-<process>.ndjson.

## log-format-json-schema · key of ## 3. Observability Harness Contract
decision/no-multiline-stack-traces · No multi-line stack traces — Every log line is one physical line parseable by jq; a hook panic is caught and logged as one ERROR line with event panic.
contract/corr-by-event/dialog · corr by event · dialog-raised and dialog-answered — corr is the dialog_id.
contract/corr-by-event/process-and-panic · corr by event · process-start, process-exit, panic — corr is null.
contract/corr-by-event/release-from-driver · corr by event · release-from-driver — corr is the JSON-RPC request id of the release refused with -32602 for carrying a string from.
contract/corr-by-event/send · corr by event · send-issued, send-confirmed, send-refused — corr is the send cursor; a wrapper refusal before send-issued carries the target's event-stream end offset, never returned to the caller; wrapper send lines carry conn and rpc_id; the client-side send-refused has corr null.
contract/process-log-line/corr · the process log line · corr — Copied unchanged as a JSON number or string, never renamed and never debug- or display-formatted; required only on the events whose rules always define it.
contract/process-log-line/message · the process log line · message — The message is a static literal equal to the event name and never carries interpolated content.
contract/process-log-line/process · the process log line · process — One of run, hook, mcp, cli, console; cli is a short-lived verb with a resolved instance, writing cli-<name>.ndjson, and console is the fleet console, writing console.ndjson with no instance.
decision/null-as-key-absence · Null written as key absence — A null corr or instance is written as an absent key, readers treat absence and null as equal, and the schema rejects a literal null. · why: tracing has no null field value
decision/obs-aligns-to-tests-log-format · Observability aligns to the tests log format — The log format is bound by the test plan's harness contract; observability may add fields but must not rename them or remove one the product still emits. · why: the harness greps on them
contract/event-stream/line · the event stream · event line — Each line of instances/<name>/events.ndjson is exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
contract/process-log-line/instance · the process log line · instance — The instance's name, or absent when none resolved.
contract/process-log-line/level · the process log line · level — One of DEBUG, INFO, WARN, ERROR.

## logging-stack · key of ## 3. Observability Harness Contract
decision/file-only-diagnostics-sinks · File-only diagnostics, one file per role — Every role logs only to its own file under the home's diagnostics directory; no role has a stdout sink, and there is no pretty or dev sink.
contract/diagnostics-files/detail · the diagnostics files · detail — Content-bearing records (full error chains, drift reports, panic payload and backtrace) go only to <home>/instances/<name>/diagnostics/detail-<process>.ndjson, file 0600 in a 0700 directory, and only when an instance resolves.
contract/diagnostics-line/role-file-line-size · the diagnostics line · role-file line size — A line in a role file (run, hook, mcp, cli, console) is codes-only and stays well under 4 KiB.
contract/obs-event-macro/attached-fields · the obs_event macro · attached fields — Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.
decision/content-records-bypass-subscriber · Content-bearing records bypass the subscriber — Content-bearing records never pass through the tracing subscriber, whose one writer holds only the home-level role file; the detail writer formats each detail line itself and writes it with one whole write.
decision/explicit-log-writer · Explicit diagnostics writer — The diagnostics subscriber always has its writer set explicitly. · why: The default writer is stdout.
decision/level-changes-volume-only · Level changes volume only — Every diagnostics level uses the same field allow-list, so the level changes only how many lines are written. · why: No setting can disable redaction.
decision/no-content-in-home-level-lines · No content fields in home-level lines — No home-level diagnostics line carries chain, drift_report, panic_payload or backtrace. · guard: G4 diag-line.v1.json schema
decision/no-internal-error-fallback · No stderr fallback for subscriber internal errors — The diagnostics subscriber never reports its own internal errors by printing to stderr. · why: The default falls back to eprintln, which would break the Hook contract and the run terminal rule.
decision/no-source-paths-in-logs · No source paths in diagnostics lines — Diagnostics lines carry no source file names or line numbers.
decision/no-span-list-in-lines · No span list in diagnostics lines — Diagnostics lines never carry the list of entered spans. · why: Closes the spans leak path.
decision/shared-log-files-concurrent-append · Concurrent appends to shared diagnostics files — Every diagnostics file may have concurrent writers in separate processes; each line is one whole write on an append-mode handle, readers tolerate a torn line, and a concurrent-append check, including a detail line over 4 KiB, asserts every line parses. · why: Non-interleaving at this size is an assumption, not a POSIX guarantee, because the 4 KiB PIPE_BUF rule applies to pipes. · guard: concurrent-append check; hook_panics_append_whole_lines_over_4_kib_side_by_side
contract/diagnostics-line/format · the diagnostics line · format — Each diagnostics line is one structured JSON object on one line, written whole by one write of the complete line on an append-mode handle.

## otel-sdk-init · key of ## 3. Observability Harness Contract
decision/home-resolution-order · Home resolution order — The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
decision/home-tree-check-before-log-init · Home tree check before log init — run, revive's start arm, adopt's start and viola verify check the home tree's modes before their log opens; a refusal writes no line, starts no child, leaves modes as found and exits 1.
decision/instance-check-before-log-init · Instance check before log init — Commands acting on an instance (revive's start arm, adopt's start, the hook arms, link, unlink, send, wait, last, answer, pause, release, focus, switch) run the instance strict-modes check before opening any log; a refusal opens and writes nothing and leaves modes as found. · why: Opening the role file narrows an existing home to 0700, so a check made after it passes a home at 0770. · except: The hook --capture arm runs ahead of the check.
decision/instance-source-per-role · Instance source fixed per role — run takes its own name argument, never an inherited VIOLA_NAME; hook and mcp take VIOLA_NAME only; cli verbs that act on an instance (send, wait, last, answer, pause, release, link, unlink) take the target argument; list, verify, plugin install and revive --list take the caller's VIOLA_NAME only. · why: A wrapper never writes into another instance's run log.
decision/panic-hook-first · Panic hook installed first — The panic hook is installed as the first statement of main and writes only to the diagnostics file once that is open, never falling back to stderr.
contract/cli-exit-codes/21 · CLI exit codes · 21 — An instance strict-modes refusal exits 21 with the fixed instance-state pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
decision/diagnostics-init-before-role-work · Diagnostics init before the role's work — Diagnostics init and the process-start line come before the role's work: in run before its collision check and the child spawn, in mcp before its stdio transport starts. · why: Exit-1 causes are logged.
decision/failed-log-open-discards · Failed log open discards lines — When the role file cannot be opened, viola_obs_init returns the error and installs no subscriber, so no line is written; hook carries on and the other roles, mcp included, return the error.
decision/flat-log-lines-and-one-level-gate · Log lines are flat and Targets is the only level gate — The subscriber flattens event fields to the top level of each line, and the Targets filter is the only configurable level gate; its level comes from the home's diagnostics level and never from the environment. · why: The diagnostics schema and the log readers read event fields as top-level keys, and only the viola crates' targets pass the gate.
decision/lazy-detail-file · Detail file opened lazily — The instance detail file is opened only by the first content-bearing record or by the panic hook, and a failed detail open drops only the detail line, writing nothing to stderr. · why: The error-free hook path keeps exactly one log open.
decision/private-diagnostics-files · Owner-only diagnostics files — The diagnostics directory is 0700 and each diagnostics file 0600, opened without following symlinks; before the first write each must be owned by the effective user, have no group or other bits and not be a symlink, a failed check counting as a failed open. · why: A pre-existing foreign, permissive or symlinked diagnostics file is never written.
decision/strict-modes-before-diagnostics · Home strict-modes check before diagnostics — Nothing under the home, the diagnostics directory included, is created or opened before the role's strict-modes check on the home passes; a refused home gets no diagnostics line. · except: viola up's home-tree check before its first spawn runs after its board scan has opened its log, and a refusal there ends that log with process-exit detail strict-modes-failed.

## service-identity · key of ## 3. Observability Harness Contract
decision/env-vars-not-configuration · Environment variables are not configuration — Environment variables are not a configuration channel: service identity has no env-var source or fallback, and the one CLAUDE_CONFIG_DIR value read by verify and the run, revive and adopt start, when kept, locates the claude config file, is never logged and configures nothing of viola.
contract/diagnostics-line/instance · the diagnostics line · instance — Every diagnostics line of a process with an instance carries it as field instance; a process without one, such as an mcp server with no instance name, writes no instance key.
contract/diagnostics-line/process · the diagnostics line · process — Every diagnostics line carries the role as field process.
contract/process-start/os · the process-start line · os — process-start carries field os, the one word linux.
contract/process-start/pid · the process-start line · pid — process-start carries the process id as field pid.
contract/process-start/service-name · the process-start line · service_name — process-start carries the service name as field service_name.
contract/process-start/version · the process-start line · version — process-start carries the product version as field version.

## snapshot-paste-to-ai-integration · key of ## 3. Observability Harness Contract
contract/send-line/client-side-refusal · the send line · client-side send-refused — The caller-side send-refused{side:"client"} has corr null.
contract/send-line/cursor-uniqueness · the send line · cursor corr uniqueness — A send cursor is a byte offset into the target's own events.ndjson, unique per target only; refusals and later sends can share it, so one send is identified by target instance, corr, conn and rpc_id.
contract/send-line/unconfirmable · the send line · unconfirmable send — An unconfirmable send logs one send-issued and one send-confirmed{confirmed:false} under the same corr, with no send-refused and no hook side.
contract/send-line/wrapper-side · the send line · wrapper side — send-* lines whose corr is a send cursor are emitted only by the target wrapper; only its send-refused line carries side:"wrapper", its send-issued and send-confirmed lines carry no side, and a client-side send-refused carries side:"client" and no corr.

## trace-context-propagation · key of ## 3. Observability Harness Contract
decision/explicit-correlation-values · Correlation values passed explicitly — corr, conn and instance are passed as explicit values into each thread or task and set on each event, never recovered from span context. · why: Spans are not serialized.
decision/span-carried-into-tasks-and-threads · Span not carried into tasks and threads — Spawned async tasks and std thread closures take no span from their spawner: spans never reach a log line, and correlation travels as explicit typed fields.
contract/diagnostics-line/corr-on-dialog-lines · the diagnostics line · corr on dialog and hook lines — On dialog and hook lines, corr carries the dialog_id.
contract/diagnostics-line/corr-on-send-lines · the diagnostics line · corr on send lines — On send lines, corr carries the send cursor; a send refused before send-issued carries the target's events.ndjson end offset at refusal, a log join key only, never returned to the caller.
contract/send-line/conn-rpc-id · the send line · conn and rpc_id — Every wrapper send-* line carries conn (or srv_conn) and rpc_id, and (conn, rpc_id) equals (conn, corr) on that call's channel-request and channel-response on both sides.

# obs-plan · its absences, names and overviews, whole

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
absence/audit-log-compliance-trail — An audit-log compliance trail
absence/otlp-receiver — An OTLP or pulse-class telemetry receiver
absence/pii-scrubbing-regime — A PII-scrubbing regime for diagnostics lines beyond the NEVER-log floor

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
absence/deployment-environments — Deployment environments or hosting

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
