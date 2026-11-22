# obs-plan · the rows that bind 2026-11-22-paste-write-bounded — picked 286 of 1034 (185 by every draw) · rule any · pin 3869

## obs-plan-viola · (above the first heading)
decision/agent-readable-telemetry · Agent-readable telemetry only — Every telemetry signal must be readable by an agent from a snapshot or pasted log excerpt: no human-only dashboards, no proprietary APM lock-in, no human-review-gated log analysis. · except: A surface that cannot be agent-readable is flagged not-instrumentable and handled through boundary instrumentation only.

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/claude-internals-not-a-contract · Wrapped claude internals are not a contract — The wrapped claude CLI is a third-party binary run unmodified and instrumented at its boundary only (spawn and exit, paste writes, the vt100 readiness-gate outcome); its internal state, transcripts and bundle strings, the MCP client's tool-call bound among them, are never read as a contract. · except: A send's echo confirmation counts its stdout tag in the transcript, keeping no byte; verify writes and removes its temp folder's trust record; a verified start sets its folder's trust flag only.
decision/cli-file-sink-only · Short-lived CLI logs to a file sink only — Short-lived CLI verbs log only to a JSON-per-line file sink in diagnostics when an instance resolves, never to stdout. · why: Stdout carries the --json results.
decision/corr-field-propagation · Propagation through the corr field — Correlation across the viola-channel is carried by the corr field in log lines of the process on each side. · why: Channel params declare no trace-context field.
decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits.
decision/distinct-detail-code-per-exit-cause · Distinct detail code per exit cause — Each cause behind exit 21 and exit 1 gets a distinct detail code with no path or pid. · except: A strict-modes check refused before any log opens writes no line; its cause is the fixed stderr pair or the --json document.
decision/mcp-log-file-sink · MCP logs to a file sink, never stdout — The `mcp` process always logs to the file sink `<home>/diagnostics/mcp.ndjson`, with no stderr fallback: a file that cannot be opened exits 1 printing nothing, and the log writer is set explicitly. · why: stdout is forbidden because it carries MCP frames
decision/mcp-rejections-codes-only · MCP rejections carry codes only — MCP isError results and rejected tool calls carry codes only (refusal, detail, instance-unreachable, wrapper-fault, invalid-params), with no upstream text, absolute paths or type names.
decision/mcp-stdout-frames-only · MCP stdout carries frames only — No telemetry from the viola-mcp server may go to stdout. · why: Stdout is reserved for MCP frames.
decision/no-traceparent-on-local-socket · No W3C traceparent on the local socket — Calls from CLI, MCP and hook to the wrapper carry no W3C `traceparent`; correlation uses `corr` (JSON-RPC `id`, `dialog_id`, send `cursor`) plus `process` and `instance`. · why: channel `params` declare no trace-context field
decision/os-apis-at-call-sites · OS APIs observed at call sites — OS APIs (openpty, Unix socket, filesystem locks and atomic rename, sysinfo) and the hyprctl spawn are instrumented at viola's call sites only, never in their internals.
decision/pty-byte-stream-never-logged · PTY byte stream never logged — The PTY byte stream is never logged; the viola-pty seam is instrumentable, and the openpty OS API behind it is boundary-only. · why: The PTY byte stream is user content.
decision/run-silent-terminal · viola run silent on the terminal — viola run writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns.
decision/run-sync-instrumentation · Sync instrumentation in viola run — Instrumentation in viola run is sync, with no Tokio.
decision/send-text-confined · Send text confined to instance diagnostics — `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
decision/send-text-never-logged · Send text never logged — Sent text never appears on stderr, in --json output, in MCP results or in any process-log field outside instances/<name>/diagnostics/.
decision/stderr-fixed-strings · Fixed strings on stderr — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/typed-keys-never-logged · Typed keys never logged — Keys a human types never appear in a log line; the wheel product event in events.ndjson is the audit record.
decision/viola-core-emits-no-telemetry · viola-core emits no telemetry — The viola-core crate supplies log-field values but emits nothing and is not instrumented. · why: A pure type crate with no I/O.
contract/diagnostics-files/mcp · the diagnostics files · mcp — Role mcp writes its role file mcp.ndjson in the home's diagnostics folder, which never carries user content; a role file that cannot be opened ends the server with exit 1 and nothing on stderr.
contract/process-log-event/release-from-driver · the process log event field · release-from-driver — Release from the driver is logged as its own distinct event.
contract/viola-mcp-tools/send · viola-mcp stdio server tools · send — The viola-mcp stdio server offers a send tool.
decision/channel-lines-carry-no-bodies · Channel log lines carry no bodies — IPC channel log lines never carry `params` or result bodies.
decision/child-exit-from-process-handle · Child exit from the process handle — The wrapped child's exit and its process-exit event come from the wait on the process handle, never from reader EOF.
decision/obs-deps-tokio-free · Obs dependencies Tokio-free and C-free — Obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass cargo deny. · guard: cargo deny
decision/spans-not-wire-contract · Tracing spans are not part of the wire contract — Span names are internal `tracing` spans in `<area>.<operation>` snake_case form; the agent-facing contract is the closed `event` enum plus `corr`. · why: the config sets `with_current_span(false)`, so spans do not reach the line
decision/tracing-json-line-file-pipeline · tracing JSON-per-line file pipeline — Telemetry is tracing plus tracing-subscriber writing JSON-per-line files under diagnostics/, correlated by corr and consumed through agent-run logs and status. · why: The pipeline is the one the tests plan binds.
contract/diagnostics-files/cli · the diagnostics files · cli — Role cli writes <home>/diagnostics/cli-<name>.ndjson, only when an instance resolves.
contract/diagnostics-files/run · the diagnostics files · run — Role run writes <home>/diagnostics/run-<name>.ndjson.
contract/run-log-file/path · the viola run log file · path — viola run logs only to a file sink at diagnostics/run-<name>.ndjson.

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/channel-log-scrub-rule · Channel log scrub rule — Channel request and response lines log only the method, id or corr, the result class (ok, refusal or error code), the sender version and the closed detail word, never params or result bodies.
decision/claude-boundary-only-instrumentation · Boundary-only instrumentation of claude — The hosted claude binary is instrumented only at its boundary: spawn and exit, paste writes, the vt100 readiness-gate outcome, and the start's one trust-record line for the trust record it writes in the claude config. · why: claude is a third-party binary run unmodified.
decision/correlation-in-corr-field · Correlation in the corr log field — Correlation across a channel call travels in the corr log field; channel params declare no trace-context field.
decision/detail-codes-no-paths · Detail codes carry no path or pid — No path or pid appears in a detail code.
decision/exit-code-log-field · Exit code as a log field — The exit code (0, 1, 2, 10, 12–14, 20, 21), the machine-readable outcome, is carried as a log field; 11 is retired and never reused.
decision/fault-telemetry-per-class · Fault telemetry per class — Each fault class emits its own telemetry: a healed torn ndjson line with file and offset, a corrupt or unsupported snapshot recovered by replay, a child exit seen without EOF, an endpoint vanished during wait, and a stale heartbeat against a live pid.
decision/fixed-stderr-messages · Fixed stderr refusal and fault lines — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/home-level-mcp-log-no-user-content · Home-level MCP log holds no user content — An mcp.ndjson at home level never carries user content.
decision/hook-mcp-instrumented-at-boundary · Hook and MCP instrumented at the boundary — Hook and MCP traffic is instrumented only at viola's own boundary: hook-invoked and hook-decision, MCP tool-call handling, and the viola mcp stdin cut. · why: The caller is Claude Code, so viola sees only the inbound stdin payload and its own response, and no trace context arrives from Claude Code.
decision/instrument-call-sites-not-os-internals · OS operations instrumented at viola's call sites — OS-level operations (bind, connect, lock, rename, kill fallback, the server check's peer reads, the strict-modes stat reads, the socket directory check, the hyprctl spawn) are instrumented at viola's own call sites, not inside the OS.
decision/mcp-results-codes-only · MCP rejections carry codes only — MCP isError results and the log lines of rejected calls carry codes only.
decision/no-telemetry-from-screen · No telemetry from screen content — No telemetry is derived from screen content; vt100 serves gate outcomes only.
decision/run-silent-on-terminal · run silent on the terminal — run writes nothing to the terminal while the child runs; its process log goes only to diagnostics/run-<name>.ndjson. · why: The terminal carries the child's screen.
decision/send-refused-by-each-deciding-side · Send refusal logged by each deciding side — A send-refused with detail control-character or empty-text is emitted by each side that decides it, client and wrapper, without the text.
decision/sync-instrumentation-no-tokio · Sync instrumentation without Tokio — Instrumentation in the sync crates runs on std threads with no Tokio. · why: Hybrid concurrency. · guard: cargo deny ban
decision/tests-event-list-adopted · Tests event list adopted — The tests' field and event list is adopted verbatim, with send-issued and send-refused among the logged events.
contract/channel-response/detail · Channel response line · detail — On a channel-response answering -32602, detail is one closed word: unsupported-version, invalid-params or release-from-driver, on both the server's and the client's line; it is absent on every other channel-response line and names no param or value.
contract/json-rpc-frame/id · JSON-RPC frame · id — Ids are integers, monotonic per connection, and so not unique across connections or processes.
contract/mcp-iserror/instance-unreachable · MCP isError result · instance-unreachable — The instance-unreachable detail is one closed cause: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused or call-deadline, the last written by the frame tools alone; the CLI's document less v, its text `refused: instance-unreachable`.
contract/mcp-iserror/refusal · MCP isError result · refusal — A peer refusal that is no code word reads unknown, and a string detail that is no code word reads null.
contract/mcp-iserror/wrapper-fault · MCP isError result · wrapper-fault — The wrapper-fault detail is a closed sanitised object: the fixed message of the five codes with data null or closed values; a code outside the five reads message "wrapper fault", data null.
contract/process-log-corr/channel-request · Process log corr by event · channel-request — corr is the JSON-RPC request id.
contract/process-log-corr/channel-response · Process log corr by event · channel-response — corr is the JSON-RPC request id.
contract/process-log-corr/panic · Process log corr by event · panic — corr is null.
contract/process-log-corr/process-exit · Process log corr by event · process-exit — corr is null.
contract/process-log-corr/send-confirmed · Process log corr by event · send-confirmed — corr is the send cursor.
contract/process-log-corr/send-issued · Process log corr by event · send-issued — corr is the send cursor.
contract/process-log-corr/send-refused · Process log corr by event · send-refused — corr is the send cursor.
contract/process-log/event · Process log line · event — A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
contract/refusal-reason/unknown-fallback · Refusal reason · unknown fallback — An unrecognised RefusalReason falls back to unknown with exit 14.
contract/send-log-line/corr · Send log line · corr — On the wrapper's send lines corr is the send cursor, which joins send-issued, send-confirmed and send-refused (a refusal before send-issued carries the log's end offset); the CLI's and MCP's client-side send-refused lines carry no corr; on channel lines corr is the JSON-RPC id.
contract/send-log-line/detail · Send log line · detail — The send detail is one of input-not-ready, no-prompt-submitted, control-character, empty-text or turn-running.
contract/send-log-line/required-fields · Send log line · required fields — A send line carries event, process, instance, corr, from, confirmed, refusal, detail and duration_ms.
contract/send-path-log-events/event-order · Send path log events · event order — A send logs channel-request, then send-issued, then hook-invoked (UserPromptSubmit, or SessionStart for a confirmed /clear or /compact; none awaited for an unconfirmable send or a local command confirmed by its transcript echo), then send-confirmed or send-refused, then channel-response.
contract/wheel-log-line/human-typing · Wheel log line · human-typing — A send refused while a human holds the wheel logs refusal human-typing, detail null or manual-pause, the wheel holder and exit_code 10.
contract/wheel-path-log-events/pause-and-release · Wheel path log events · pause and release — A wheel take-over logs send-refused for the refused send and a channel-request and channel-response pair for pause and for release.
decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but a required one the product still emits is never renamed or removed. · why: The harness greps on them.
decision/mcp-no-telemetry-on-stdout · No telemetry on the MCP server's stdout — The MCP server sends no telemetry to stdout. · why: stdout is reserved for MCP frames.
decision/no-process-log-without-instance · No process log without an instance — A short-lived CLI verb run with no resolved instance writes no process-log file; its human stderr carries only fixed-message lines, and its outcome is the exit code and stdout.
decision/per-parser-parse-failure-counts · Per-parser parse-failure counts — Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, the hyprctl replies, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead.
contract/cli-internal-error/stderr-line · CLI internal error · stderr line — Without --json a catch-site fault prints the fixed line `error: internal error` to stderr.
contract/exit-21-detail/endpoint-lost · Exit 21 detail codes · endpoint-lost — endpoint-lost is a distinct detail code for an exit-21 cause, naming no path or pid.
contract/exit-21-detail/instance-dead · Exit 21 detail codes · instance-dead — instance-dead is a distinct detail code for an exit-21 cause, naming no path or pid.
contract/exit-21-detail/server-verify-failed · Exit 21 detail codes · server-verify-failed — server-verify-failed is a distinct detail code for an exit-21 cause, distinct from instance-dead, naming no path or pid.
contract/exit-21-detail/socket-dir-refused · Exit 21 detail codes · socket-dir-refused — socket-dir-refused is a distinct detail code for an exit-21 cause, naming no path or pid.
contract/exit-21-detail/strict-modes-failed · Exit 21 detail codes · strict-modes-failed — strict-modes-failed is a distinct detail code for an exit-21 cause, naming no path or pid.
contract/exit-21-detail/unwrapped-name · Exit 21 detail codes · unwrapped-name — unwrapped-name is a distinct detail code for an exit-21 cause, naming no path or pid.
contract/process-log/corr · Process log line · corr — Copied unchanged as a JSON number or string, never renamed.
contract/run-start-events/order · run start events in events.ndjson · order — A run start writes wheel{cause:"start"} before session-start; on a start whose folder-trust keys pass through the wrapper, wheel{human-input} also precedes session-start, and a later human key adds no wheel line, the wheel being already human.
contract/send-log-line/from · Send log line · from — The from field is self-reported and logged as unauthenticated, never as an identity.
contract/unreachable-cause/endpoint-lost · exit 21 cause words · endpoint-lost — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/unreachable-cause/instance-dead · exit 21 cause words · instance-dead — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/unreachable-cause/server-verify-failed · exit 21 cause words · server-verify-failed — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/unreachable-cause/socket-dir-refused · exit 21 cause words · socket-dir-refused — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/unreachable-cause/strict-modes-failed · exit 21 cause words · strict-modes-failed — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/unreachable-cause/unwrapped-name · exit 21 cause words · unwrapped-name — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/corr-conn-correlation · Correlation through corr and conn — Every cross-surface call carries correlation through the corr field plus the additive conn discriminator.
decision/one-line-panic-record · One structured line per panic — Every panic after init step 4 produces exactly one single-line structured record in its home-level role file with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file. · except: the bounded panic exemptions listed in the plan's Section 10
decision/span-naming · Span naming as <area>.<operation> — Span names are <area>.<operation> in snake_case, <area> one of run, pty, channel, state, hook, send, wait, last, answer, pause, release, cli, mcp, hyprland; cli.<verb> names the dispatch span of a short-lived verb.
contract/process-log-line/enum-values · the process log line · enum values — Enum values are kebab-case.
contract/process-log-line/event · the process log line · event — The event field is a closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
decision/no-event-payloads-in-process-logs · No event payloads in process logs — Obs never copies event payloads into process logs.
decision/no-human-gated-log-analysis · No human-review-gated log analysis — Every log verdict is a jq or harness assertion that sets an exit code, never a human review.
decision/no-w3c-traceparent · No W3C traceparent in v1 — W3C traceparent is not used in v1; correlation goes through corr plus conn. · why: No consumer exists and channel params declare no trace-context field.
decision/spans-not-serialized-v1 · Spans declared but not serialized — Spans are declared in code but not serialized in v1.
contract/process-log-line/duration-ms · the process log line · duration_ms — Durations are duration_ms as an integer.
contract/process-log-line/field-names · the process log line · field names — Field names are snake_case.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/cross-process-join-corr-conn · Cross-process joins by corr and conn — Lines across processes are joined by the corr and conn fields, not by W3C trace IDs.
decision/panic-caught-at-every-role · Panics caught at every role's catch site — A hand-written panic hook plus catch_unwind at every role's main-thread catch site (covering hook dispatch and run's inline handle-wait), the two run PTY pump threads and the vt100 feed turn a panic into one record.
decision/panic-one-line-record · One single-line record a panic — Every panic after obs init produces exactly one single-line structured record in its home-level role file, with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file. · except: the pre-init window and the paths that refuse or lack an instance or home tree before their role file opens
decision/events-stream-never-rotated · Event stream never rotated — The instance's events.ndjson is the product audit stream, never rotated by a generic library nor rewritten: a reused instance name keeps the earlier starts' lines at its head, so a start's own lines are read from where that start begins. · why: byte offsets in it are the wait-after and send cursors
decision/spans-not-serialized-in-v1 · Spans declared but not serialized in v1 — Spans are declared on the must-trace paths and every crate seam but are not written to output in v1. · why: they give the code its structure and are ready for the deferred OpenTelemetry bridge

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/codes-only-process-logs · Codes-only process logs, content in detail files — Every role's codes-only process log goes to its home-level role file under diagnostics, and content-bearing detail goes only to the instance's per-role detail file.
decision/short-lived-verbs-stderr · Short-lived verbs use stderr for humans — Short-lived CLI verbs may use stderr for their human output.
contract/mcp-output/stdout · mcp output streams · stdout — mcp writes only MCP frames to stdout.
contract/run-terminal-output/child-output · run terminal output · child output — run writes nothing to the terminal except the child's own output; while the child runs, neither run nor a revived start writes anything of its own.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/channel-lines-no-params-or-results · Channel lines carry no params or results — Channel log lines never record request params or result bodies.
decision/channel-response-always-logged · channel-response always logged — Every dispatched channel request logs its channel-response line, an early return or an error included. · why: The line is emitted from a Drop guard, so an early return or error still logs it.
decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.
decision/compact-confirm-window · Compact confirmation window — A /compact send waits 40 s for its compacted-session post-condition, while every other confirmed send waits 10 s. · provisional
decision/dependency-targets-off-in-sink · rmcp and portable-pty targets off — The rmcp and portable-pty log targets are switched OFF in the sink.
decision/detail-codes-only · detail is a closed code — The detail field is a closed kebab-case code catalog and never carries a path or pid.
decision/mid-call-disconnect-logged-once · Mid-call disconnect logged once — A disconnect in the middle of a call is logged once, by the client.
decision/sent-text-never-logged · Sent text never logged — A sent message's text is never logged; only its size in text_bytes is.
decision/wheel-event-is-audit-record · Wheel product event is the audit record — The wheel product event in events.ndjson is the audit record of a wheel change; the typed keys are never logged and the wheel transition span is only a point in time.
decision/no-endpoint-path-or-env-in-spans · No endpoint, binary path or env values in spans — A span never carries the endpoint name, the pinned_bin path or environment values.
decision/no-trace-and-span-ids · No trace_id or span_id — Log lines never emit trace_id or span_id; correlation is by corr plus conn. · why: No W3C trace context exists, and spans are not attached to lines.
decision/uninstrumented-components · Components left uninstrumented — viola-core, config.json and plugin files, and build/CI infrastructure are not instrumented. · why: viola-core only supplies ObsEvent and RefusalReason values; the rest are not instrumentable.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/clients-screen-peer-replies-to-code-words · Clients screen peer replies to code words and a fixed fault — The CLI and MCP clients screen a peer reply before using it: a refusal and its detail must be code words, a wrapper fault keeps only its code, a fixed message and closed data, and the screening adds no span and no line. · why: No path or upstream text may cross from a wrapper into a document, a tool result or a log.
decision/disconnect-logged-by-client · Mid-call disconnect logged once by the client — A mid-call disconnect is logged once, by the client.
decision/manual-readiness-gate-span · Manual span on the readiness gate — The readiness gate of viola run carries a manual span, run.readiness_gate. · why: vt100 has no tracing.
decision/mcp-explicit-log-writer · Explicit log writer in viola mcp — The viola mcp log writer is always explicit: the mcp.ndjson file, with no stderr fallback.
decision/no-text-in-telemetry · Sent text never in telemetry — The sent text never appears in a span or log line; only its length does.
decision/no-typed-keys-in-telemetry · Typed keys never in telemetry — The keys the human types never appear in a span.
decision/portable-pty-log-target-off · portable-pty log target OFF — The log target of portable-pty is OFF in the sink, and PTY failures are captured at the pty seam instead.
decision/real-cli-verify-never-in-ci · Real-CLI verify never runs in CI — In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
decision/send-line-correlation · Send lines correlate with channel lines — Every wrapper send-* line carries conn (or srv_conn) and rpc_id, so that pair on the send line equals (conn, corr) on the call's channel lines; the client-side send-refused carries neither.
decision/transcript-echo-read-uninstrumented · Transcript echo read uninstrumented — The send's transcript echo read, the end offset before the paste and the tag count after it, opens no span and writes no line or span field of its own; its result shows only as the send-confirmed line inside run.confirm_window.
decision/wheel-typed-mark-uninstrumented · Wheel typed mark and wakes count uninstrumented — The stdin typed mark and the held scheduled-wakes count are in-memory wrapper state that open no span and write no line of their own, and carry no key and no prompt text.
contract/channel-lines/fields · channel-request and channel-response lines · fields — channel-request carries method, corr, conn and sender; channel-response carries method, corr, conn, result_class and error_code, plus detail on a -32602.
contract/cli-exit/20-wrapper-fault · send.client process-exit · 20 wrapper-fault — Exit 20 carries detail wrapper-fault.
contract/cli-stderr/refusal-pair · CLI stderr · unable and hint pair — A refusal is printed to stderr as a fixed "unable:" line followed by a "hint:" line.
contract/confirmed-send-spans/span-chain · confirmed send spans · span chain — send.client (CLIENT) › channel.request (CLIENT) → channel.dispatch (SERVER) › run.readiness_gate › pty.paste_write › run.confirm_window; an unconfirmable send ends at pty.paste_write. In the hook: hook.handle › channel.request(hook.event).
contract/mcp-spans/mcp-tool-call · viola mcp spans · mcp.tool_call — viola mcp opens one mcp.tool_call span per tool (send, wait, last, answer, list); rejects log codes-only send-refused or channel-response lines.
contract/send-log-fields/channel-request · confirmed send log fields · channel-request — channel-request{corr:<id>, conn, method, sender}.
contract/send-log-fields/hook-invoked · confirmed send log fields · hook-invoked — hook-invoked{hook_event:"user-prompt-submit", corr:null} for an ordinary send; hook_event session-start for a /clear or /compact confirmed by its session; an unconfirmable send, or a local command confirmed by its transcript echo, waits for no hook line.
contract/send-log-fields/send-confirmed · confirmed send log fields · send-confirmed — send-confirmed{corr:<cursor>, confirmed, duration_ms}, confirmed false for unconfirmable, duration_ms from the send's arrival including the readiness gate's wait.
contract/send-log-fields/send-issued · confirmed send log fields · send-issued — send-issued{corr:<cursor>, from, text_bytes}.
contract/send-refused-detail/no-prompt-submitted · wrapper send-refused detail · no-prompt-submitted — Confirm-window expiry is send-refused{detail:"no-prompt-submitted"} for ordinary, /clear, /compact and echo-confirmed local command sends alike, with no new detail.
contract/send-refused-detail/turn-running · wrapper send-refused detail · turn-running — Covers a running turn the wrapper knows of before the paste, or another send in flight, with no new field; the refusal has no cursor (corr the end offset at refusal), no send-issued, and the client exits 13.
contract/send-refused/human-typing · wrapper send-refused · human-typing — send-refused{corr:<cursor>, refusal:"human-typing", detail:null|"manual-pause", wheel:"human", side:"wrapper"}, with the client process-exit{exit_code:10}.
contract/send-span-attributes/run-readiness-gate · confirmed send span attributes · run.readiness_gate — outcome (ready or input-not-ready) and vt100_panicked (bool).
contract/send-span-attributes/text-bytes · confirmed send span attributes · text_bytes — text_bytes, on pty.paste_write and send-issued alike, is the typed text's length: the sent text with each CR LF pair and each other CR as one LF, trailing CR and LF removed; pty.paste_write also carries paste_mode="bracketed".
contract/send-span-attributes/window-ms · confirmed send span attributes · run.confirm_window window_ms — window_ms is the bound the send's own deadline reads: 40 000 for a /compact send waiting for its compacted-session post-condition, else 10 000.
contract/wheel-span-attributes/run-wheel-transition · wheel span attributes · run.wheel_transition — wheel_from, wheel_to (human|driver), cause (human-input|manual-pause|release); a point-in-time span with a static name, opened once per wheel change on the wheel's worker thread.
contract/wheel-spans/span-chain · wheel spans · span chain — run.wheel_transition (INTERNAL) › channel.dispatch(send) refused; pause.client or release.client (CLIENT) › channel.request(pause|release) → channel.dispatch(pause|release).
decision/cli-log-only-under-viola-name · cli process log only under a parsed name — A short-lived cli verb opens its process log and writes process-start and process-exit only when an instance resolves: for list, plugin install and revive --list under a parsed VIOLA_NAME (with no name no log), for the frame verbs under the named target (link and unlink the driven) instance.
decision/error-chain-in-detail-log-only · Full error chain only in the cli detail log — The full anyhow error chain of a cli verb goes to the cli detail log only.
decision/implicit-link-on-send-and-answer · Implicit link recorded by send and answer — An issued send or a taken answer whose from names an instance other than the wrapper records the link, one link product line for a new pair; a refused call, one with no from, or one under the wrapper's own name records nothing.
decision/no-params-in-channel-lines · No params or result bodies in channel lines — channel-request and channel-response lines never carry params or result bodies.
contract/cli-exit-21/endpoint-lost · cli exit 21 process-exit · endpoint-lost — detail endpoint-lost with during call: the connection was lost after the connect; it is the only exit-21 detail whose during is call.
contract/cli-exit/1-internal-error · send.client process-exit · 1 internal-error — exit_code 1 internal-error on a reply with neither a parseable result nor an error.code.
contract/confirm-window-close/ordinary-send · run.confirm_window close · ordinary send — Closes on the prompt-submitted{origin:"driver"} whose prompt equals the typed text, or is non-empty and equals the typed text with a trailing run of ASCII space, TAB, CR or LF removed, or on window expiry.
contract/refusal-stderr-pair/socket-dir-refused · cli refusal stderr pair · socket-directory refusal — Printed on start and frame-verb refusals in plain ASCII with hint: last and no SGR, naming only the parsed ViolaName.
contract/run-process-log/common-fields · run process log · common fields — Every run line carries the common fields with process="run" and corr null.
contract/send-refused-detail/input-not-ready · wrapper send-refused detail · input-not-ready — send-refused{corr:<cursor>, refusal, detail:"input-not-ready", side:"wrapper", wheel}.
contract/span-kinds/client · Span kinds · CLIENT — The CLIENT spans are send.client, wait.block, last.client, answer.client, pause.client, release.client, cli.link, cli.unlink, channel.request (client side) and pty.spawn.
contract/span-kinds/internal · Span kinds · INTERNAL — The INTERNAL spans are run.*, state.* and pty.paste_write; a hook's decision is the decision_emitted field, not a span.
contract/span-kinds/server · Span kinds · SERVER — The SERVER spans are channel.dispatch (the wrapper side of every channel.request), mcp.tool_call and hook.handle.

## metric-coverage · ## 5. Metric Coverage · twin 5-metric-coverage
contract/cli-process-exit/server-verify-failed · CLI process-exit diagnostic line · server-verify-failed — The CLI's server-verification refusal writes one process-exit with subject self, exit_code 21, detail server-verify-failed and during connect at WARN in cli-<name>.ndjson, with no pid, start time, path or endpoint; MCP's wait tool writes no line for it.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/additive-field-naming · Additive log field naming — Additive log fields are snake_case with kebab-case values, and carry codes only.
decision/ipc-calls-logged-without-arguments · IPC calls logged without arguments — IPC params and result bodies are never recorded, not even as a hash or digest.
decision/log-schema-additive-only · Log schema additive to the tests' format — The observability log schema only adds fields and enum values to the tests' log format; it renames nothing and removes only a field no build emits. · why: The tests' log format is the binding text.
decision/third-party-crate-logs-off · Third-party crate logs off — The per-module log filter turns logging OFF for the rmcp, portable_pty, sysinfo and interprocess targets. · why: Their failures are captured at the viola seam as viola events.
contract/ipc-invocation-log/fields · IPC invocation log lines · fields — Channel-request and channel-response lines carry method, conn (srv_conn when the client sent none) and the corr of a frame with an id; a request line adds sender, a response line result class and an error's error code but never sender.
contract/log-levels/error · log levels · error — Used for a panic, a process-exit that carries a detail, and a channel-response with error_code -32603.
contract/log-levels/info · log levels · info — INFO carries all other boundary, lifecycle and state-transition events, including refusals, which are normal product outcomes.
contract/log-line/corr · every log line · corr — Set per the tests' table and copied unchanged as a JSON number or string; null is represented by the key's absence.
contract/pty-seam-log/paste · PTY seam log events · paste — A paste is logged with text_bytes only.
decision/viola-crates-log-at-info · Viola crates log at INFO — The per-module log filter sets the viola, viola_core, viola_pty, viola_channel, viola_state, viola_agent_claude and viola_mcp targets to INFO, or DEBUG via diagnostics_level.
contract/log-levels/warn · log levels · warn — WARN carries state-recovered; parse-rejected; hook-decision with a fail-open detail; process-exit with exit 21; channel-response with -32600 or -32602.
contract/log-line/event · every log line · event — A value of the closed kebab-case event enum.
contract/log-line/message · every log line · message — Flattened, a static literal equal to the event name.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/unemittable-schema-words-removed · Schema words no build can emit are removed — A closed word of the diag-line schema that no build can emit leaves the schema with its emitter; os is closed at linux.
contract/diag-line-fields/channel-response · diagnostic line additive fields · channel-response — method (only a closed-list name), conn / srv_conn, result_class (ok, refusal, error), detail (only the three -32602 faults), error_code (-32700, -32600, -32601, -32602, -32603), outcome, duration_ms.
contract/diag-line-fields/process-exit · diagnostic line additive fields · process-exit — subject, exit_code (self), child_exit_status, exit_source (handle-wait or kill-fallback), detail, during (connect or call), duration_ms.
contract/diag-line-fields/send-events · diagnostic line additive fields · send-issued / send-confirmed / send-refused — from, from_trust, text_bytes (normalised typed length), confirmed, refusal, detail, side (client or wrapper), wheel, duration_ms, and wrapper side only conn / srv_conn and rpc_id.
contract/process-exit-detail/exit-20 · process-exit detail · exit 20 — wrapper-fault.
contract/send-refused-detail/refusal-reason-words · send-refused detail · RefusalReason detail words — Refusal details are the arch RefusalReason details verbatim: manual-pause, input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text.
contract/diag-line-fields/panic · diagnostic line additive fields · panic — panic_location (workspace-relative, or <crate>-<ver>/src/... for dependencies) and thread; payload and backtrace go to the detail file only.
contract/diag-subject/self · diag-line subject enum · self — One value of the closed subject enum of the diag-line schema.
contract/process-exit-detail/exit-21 · process-exit detail · exit 21 — instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused.
contract/send-refused-detail/empty-text · send-refused detail · empty-text — Written for an empty typed text on send; the word adds no field, and send-refused detail stays an open string in the diag-line schema.

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/boundary-errors-end-in-logged-line · Crate-seam errors end in a logged line — Every typed error that crosses a crate seam ends in a logged process-exit detail, channel-response error, hook-decision detail or send-refused line, carrying codes only at WARN or ERROR.
decision/error-correlation-without-trace-ids · Error correlation without trace ids — Errors correlate by corr, conn, process and instance, plus timestamp, not by trace_id or span_id.
decision/instance-unreachable-not-logged · instance-unreachable logged by no line — An MCP instance-unreachable reject, from any instance, liveness, socket-directory, euid or server-verification check or from the MCP call deadline, is logged by no line; the isError code is its only record.
decision/mcp-rejects-codes-only · MCP rejects are codes only — MCP rejects are isError codes only, and a logged reject's line holds codes only.
decision/run-threads-inside-catch-unwind · Run threads inside catch_unwind — The two run PTY pump threads each run inside catch_unwind, and the child's exit is polled inline on run's main thread under main's catch_unwind; a worker panic that leaves main running with no process-exit is forbidden.
decision/typed-errors-fixed-display · Typed errors with fixed Display messages — Every typed error has a fixed-message Display, written with thiserror, and holds no fields with paths or payloads. · except: PtyError's Display is hand-written, as viola-pty takes no thiserror.
contract/panic-outcome-by-role/run · panic outcome by role · run — After a panic, run logs process-exit{subject:"self", exit_code:1, detail:"internal-error"} and exits 1, writing nothing on the terminal while the child runs.
decision/no-sentry-or-otel-bridge · No Sentry or OTel bridge — Errors have no Sentry or OTel bridge; they correlate by corr, conn, process and instance, not by trace_id or span_id. · why: Both were deferred or rejected.
decision/panic-exit-decided-at-catch-site · Panic exit decided at a catch site — The panic hook only writes; the exit is decided by one catch_unwind in the root binary's main around everything after the hook is set, so no role ends through an uncaught unwind.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
decision/mcp-unreachable-carries-one-cause-word · Each MCP unreachable result carries one cause word — Each MCP unreachable outcome (a refused check, a dead or unwrapped instance, a lost endpoint, an unverified server, a refused socket directory or a frame call past the MCP call deadline) carries exactly one UnreachableCause word as its detail. · why: The MCP process writes no exit line for it, so the result's cause word is the cause's only record for the calling agent.
decision/peer-replies-screened-as-code-words · Peer replies screened as code words — A peer reply's refusal and detail are kept only as code words (1-64 bytes of [a-z0-9-], letter first); an unknown code word stays visible and exits 14. · why: by design
decision/run-worker-panic-ends-run · Run worker panic ends the session — A panic in either PTY pump thread is sent to main, which kills the child and takes run's exit path, exit 1 internal-error; the child's exit is polled on main, not on a handle-wait thread; a worker panic leaving main running with no process-exit is forbidden. · why: the session cannot continue without either thread
decision/exit-decided-at-catch-site · Exit decided at a catch site, not the panic hook — The panic hook only writes; everything after the hook is set runs inside one catch_unwind in main, so no role ends through an uncaught unwind.
decision/single-sanitising-fault-reader · One sanitising reader of peer faults — A peer's fault is read only through one sanitiser: five known codes read fixed messages with data admitted only in validated shapes, a code outside them reads wrapper fault, and a fault without an integer code reads nothing.
contract/answer-send-exit-2/stderr-line · viola answer and viola send exit 2 · stderr line — Exit 2 writes the fixed unusable-response or unusable-text line to stderr whatever --json says, with no document.
contract/exit-21-document/detail · exit 21 document · detail — One of six closed words: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused.
contract/exit-21-document/shape · exit 21 document · shape — Exit 21 prints one document per cause: {"v":1,"error":"instance-unreachable","detail":"<cause>"}.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/content-canary-never-home-level · Content canary never in home-level files — The content canary never appears in a home-level diagnostics ndjson file.
decision/conversation-text-only-in-detail-file · Conversation text only in the instance detail file — Prompt, assistant, tool-input, plan and dialog text never reach a home-level line, only an instance's per-process detail file inside a chain or drift report.
decision/default-deny-log-fields · Default-deny for logged fields — A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure. · guard: gate G4 against schemas/diag-line.v1.json
decision/fixed-error-messages · Fixed error messages — Error Display texts are fixed, and serde-sourced error chains are mapped to fixed messages at dispatch.
decision/home-level-lines-path-free · Home-level lines path-free — Home-level log lines are path-free: file fields are basenames, panic_location is workspace-relative and source file recording is off; absolute paths appear only in detail files. · why: The OS username inside absolute paths is the only PII in scope.
decision/low-class-metadata-as-typed-fields · Low-class metadata as typed fields — Low-class metadata is logged as typed fields, never inside detail codes.
decision/schema-top-level-unevaluated-properties · Default-deny schema via top-level unevaluatedProperties — The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists. · why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then · guard: contract_diag_schema
decision/content-only-in-detail-file · Content-bearing records only in the detail file — Content-bearing records go only to the instance-scoped detail-<process>.ndjson file.
decision/detail-schema-self-contained · Separate self-contained detail-line schema — Detail-file lines are validated by their own schema with a top-level unevaluatedProperties false, which inlines the event enum and uses no cross-file reference.
decision/source-allow-list-logging · Source allow-list for logged fields — Spans skip all arguments and declare fields explicitly, and events go through obs_event! with explicit typed fields only; this source allow-list is the primary PII control. · guard: disallowed-macros; raw_event_hits; G1 bare instrument

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/canary-only-in-detail-files · Canary only in detail files — Every E2E and integration input is synthetic and embeds a fixed canary, which may appear in detail files but never in a home-level diagnostics file. · guard: mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line
decision/closed-protocol-codes · Closed protocol codes — Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value. · guard: answer_invalid_params_names_its_cause_and_no_param
decision/error-bodies-clean · Clean error bodies — Error bodies of CLI --json, MCP results and the channel faults hold no absolute path, upstream caused-by text or serde path, and every product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — thiserror messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
decision/prompt-and-output-never-in-home-logs · Prompt and assistant output never in home logs — Prompt text and assistant output, last_assistant_message included, never appear in a home-level process-log line; they reach an instance detail file only inside a chain or drift report, and only text_bytes is logged.
decision/typed-ids-never-in-detail · Ids logged as typed fields — pid, child_pid, agent_session_id and started_at are logged as typed fields, never inside detail codes.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/g4-schema-conformance-gate · Schema conformance gate on every line — Every home-level line is validated against the diagnostics line schema and every detail line against the detail schema, in a step that always runs. · guard: G4

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/diag-lines-schema-checked-in-unit-tests · Emitted lines schema-checked in integration and E2E tests — Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
decision/panic-unwind-required · Panic strategy is unwind — Every Cargo profile keeps panic = "unwind"; no abort strategy in profiles, rustflags, env or workflows. · why: abort turns every catch_unwind into an abort, so hook would die by SIGABRT instead of exiting 0 · guard: gate G3
decision/schema-conformance-gate · Schema conformance gate on real process output — Every role-file line of test homes must validate against schemas/diag-line.v1.json and every detail line against schemas/diag-detail.v1.json; a failing line fails the build, a torn non-JSON line is skipped and counted. · why: schema failures count as a build failure and unit tests check only in-memory lines · guard: gate G4 (schema-conformance step)
decision/sync-crates-without-tokio · Sync crates build without Tokio — The synchronous crates must compile without Tokio. · guard: cargo check of the sync crates without Tokio

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/correlation-propagation · Correlation propagation — Every channel call logs corr and conn on both sides; a send line from send-issued on carries the send cursor, a wrapper refusal before it the events.ndjson end offset as corr, and a client-side refusal no corr; every dialog line carries dialog_id.
decision/deterministic-emission · Deterministic event emission — Events key on hook events and events.ndjson offsets; nothing is derived from screen content. · except: The readiness gate reads the vt100 screen before a paste and can refuse a send as input-not-ready; a local command's echo confirmation keys on its transcript's stdout tag count.
decision/module-boundary-error-logging · Errors logged once at module boundaries — Every typed error that crosses a crate seam is logged once at WARN or ERROR with a closed detail or error_code; the full chain goes only to the instance detail file.
decision/no-retry-policies · Retry policies forbidden — Retry-once policies are forbidden, for tests and for panics alike. · why: retry policies mask real signals
decision/schema-conformance-failure-fails · Schema-conformance failure is a failure — An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.
decision/zero-unlogged-panics · Zero unlogged panics — Every panic in run, hook, mcp and cli after init step 4 produces exactly one single-line ERROR panic record in its home-level role file, with no multi-line backtrace there, and hook still exits 0; retry-once policies for panic handling are forbidden. · guard: G2 · except: the bounded exemptions where no file exists, so no line can be written; G2 alone leaves out the fake-agent seam's deliberate panic from its count.

## slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin 10-slo-invariants-telemetry-budgets
decision/no-retry-on-panic · No retry on panic — Retry-once policies for panic handling are forbidden. · why: Parallel to the tests' retries = 0.

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/closed-event-and-corr-on-every-line · Closed event enum on every line, corr where correlated — Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
decision/corr-set-explicitly-by-caller · corr set explicitly by the caller — The caller sets corr explicitly as a typed field on the events that carry one.
decision/event-values-closed · event values closed — No event value is added without a Decisions Log entry.
decision/explicit-corr-conn-instance · corr, conn and instance passed explicitly — corr, conn and instance are passed as explicit values across threads or tasks, never carried by span context. · why: Spans are not serialized.
decision/field-allow-list-default-deny · Field allow-list is default-deny — No field is logged unless named in the field catalog.
decision/must-trace-paths-always-traced · Must-trace paths always traced — Spans and events are never skipped on the 6 must-trace paths. · why: The agent loses its ability to debug without them.
decision/no-hot-path-instrumentation · No hot-path instrumentation — The PTY pump read loop and the vt100 feed get no per-byte or per-chunk span or event; only gate outcomes are logged.
decision/no-info-in-hot-loops · No info logging in hot loops — Nothing is logged at info inside hot loops such as the PTY pump; counts go on the boundary event instead.
decision/no-paths-or-pids-in-detail-codes · No paths or pids in detail codes — detail codes never contain paths or pids.
decision/no-pii-in-span-fields · No PII or content in span fields — Span fields never include PII or content. · why: Scrubbing at an exporter stage is too late, and v1 has no exporter stage at all.
decision/no-retry-once · No retry-once policies — No retry-once policy is added for panics or flaky telemetry assertions.
decision/rerun-no-substitute · Re-run is no substitute for diagnostics — A re-run is not a substitute for the failing job's diagnostics. · why: CI runs under retries = 0.
decision/sensitive-content-detail-file-only · Sensitive content only in the instance detail file — Send text, tool input and last_assistant_message never reach a home-level file; they may reach only the instance-scoped detail-<process>.ndjson, inside a chain or drift report.
decision/budgets-asserted-by-exit-code · Budgets asserted by exit code — Every budget has an exit-code-setting assertion (hyperfine max via jq).
decision/default-panic-hook-never-runs · Default panic hook never runs — The default panic hook is never called or allowed to run. · why: Its stderr output breaks hook and the run terminal.
decision/hyperfine-max-is-enforced · Hyperfine max is the enforced number — The max from hyperfine is the enforced latency number.
decision/latency-with-gate-statistic · Latency reported with the gate statistic — Latency is never reported without the gate statistic; p50 and p95 are report-only.
decision/machine-asserted-telemetry-verdicts · Machine-asserted telemetry verdicts — Log analysis is never human-review-gated; every CI telemetry verdict is a jq or hyperfine assertion that sets an exit code.
decision/no-log-bridge · No log bridge — tracing_log::LogTracer or any log bridge is never installed by hand, and tracing-log stays off. · why: Bridged records lack event, corr, process and instance.
decision/no-serde-source-in-error-output · No serde source in printed errors — An anyhow chain that still contains a serde_json or serde_path_to_error source is never printed to stderr, --json, MCP isError or channel error.data.
decision/no-stringified-corr · corr never recorded with ? or % — corr is never recorded with ?corr or %corr. · why: Those stringify numbers.
decision/no-telemetry-from-screen-content · No telemetry from screen content — Telemetry is never derived from screen content; vt100 is used for readiness and modal gate outcomes only.
decision/paths-only-in-instance-detail-files · Paths only in instance detail files — Paths appear only in instance detail files.
decision/transcripts-not-a-contract · Claude Code transcripts are not a contract — Claude Code transcripts are never read as a contract. · except: A local command's stdout tag in the session transcript, counted past the end recorded before the paste, is read as that send's echo confirmation; no transcript byte is kept.
contract/stdout-by-role/cli · stdout by role · cli — stdout carries the --json results.
contract/stdout-by-role/mcp · stdout by role · mcp — stdout carries MCP frames.
contract/stdout-by-role/run · stdout by role · run — The terminal of run carries the child's screen.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/conn-in-params-joins-cross-process-logs · Conn in request params joins logs across processes — Every channel client request carries its conn label in params, and the server logs that conn on its lines (or its own srv label when the conn is missing or malformed) without handing it to the wrapper's answer. · why: Joining a client's request line to the wrapper's handling lines across processes relies on the shared label.
decision/content-only-in-instance-detail-file · Content only in instance-scoped detail files — Content-bearing detail goes only to the instance's detail-<process>.ndjson file; home-level log files are codes-only and carry no user content, token or CLAUDE* value. · why: security allows user-derived content only in instance-scoped diagnostics
decision/diag-schema-matches-event-enum · Diagnostic line schema matches the event enum — A hand-maintained JSON Schema of the process-log line is committed, and its event enum must equal the ObsEvent variants, asserted by a tests-owned check.
decision/fixed-error-display · Fixed error Display text — Typed errors have fixed Display text that carries no interpolated content.

## log-format-json-schema · key of ## 3. Observability Harness Contract
decision/null-as-key-absence · Null written as key absence — A null corr or instance is written as an absent key, readers treat absence and null as equal, and the schema rejects a literal null. · why: tracing has no null field value
contract/corr-by-event/channel · corr by event · channel-request and channel-response — corr is the JSON-RPC request id; null for an id-less hook.event notification and on a response to a frame that yielded no id.
contract/corr-by-event/process-and-panic · corr by event · process-start, process-exit, panic — corr is null.
contract/corr-by-event/send · corr by event · send-issued, send-confirmed, send-refused — corr is the send cursor; a wrapper refusal before send-issued carries the target's event-stream end offset, never returned to the caller; wrapper send lines carry conn and rpc_id; the client-side send-refused has corr null.
contract/event-stream/line · the event stream · event line — Each line of instances/<name>/events.ndjson is exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
contract/process-log-line/corr · the process log line · corr — Copied unchanged as a JSON number or string, never renamed and never debug- or display-formatted; required only on the events whose rules always define it.
contract/process-log-line/message · the process log line · message — The message is a static literal equal to the event name and never carries interpolated content.
decision/obs-aligns-to-tests-log-format · Observability aligns to the tests log format — The log format is bound by the test plan's harness contract; observability may add fields but must not rename them or remove one the product still emits. · why: the harness greps on them

## logging-stack · key of ## 3. Observability Harness Contract
decision/no-log-crate-bridge · No log-crate bridge — The log-crate bridge (tracing-log, LogTracer) stays off; failures are captured from returned values or handles, and a gap is fixed at the seam or by the researched json-subscriber fallback, never by enabling the bridge. · why: The accepted loss is only third-party log context emitted without a returned error. · guard: pty_exit_is_read_on_the_handle_while_the_output_is_held
decision/third-party-targets-off · Third-party log targets off — Records of third-party targets (rmcp, portable_pty, sysinfo, interprocess) are off in every sink; their failures are captured at viola's seam from the returned results. · why: Their records lack the required fields.
contract/diagnostics-line/role-file-line-size · the diagnostics line · role-file line size — A line in a role file (run, hook, mcp, cli) is codes-only and stays well under 4 KiB.
contract/obs-event-macro/attached-fields · the obs_event macro · attached fields — Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.
decision/no-content-in-home-level-lines · No content fields in home-level lines — No home-level diagnostics line carries chain, drift_report, panic_payload or backtrace. · guard: G4 diag-line.v1.json schema

## otel-sdk-init · key of ## 3. Observability Harness Contract
decision/instance-check-before-log-init · Instance check before log init — Commands acting on an instance (revive's start arm, the hook arms, link, unlink, send, wait, last, answer, pause, release) run the instance strict-modes check before opening any log; a refusal opens and writes nothing and leaves modes as found. · why: Opening the role file narrows an existing home to 0700, so a check made after it passes a home at 0770. · except: The hook --capture arm runs ahead of the check.

## service-identity · key of ## 3. Observability Harness Contract
decision/env-vars-not-configuration · Environment variables are not configuration — Environment variables are not a configuration channel: service identity has no env-var source or fallback, and the one CLAUDE_CONFIG_DIR value read by verify and the run and revive start, when the strip keeps that name, locates the claude config file, is never logged and configures nothing of viola.
contract/diagnostics-line/instance · the diagnostics line · instance — Every diagnostics line of a process with an instance carries it as field instance; a process without one, such as an mcp server with no instance name, writes no instance key.

## snapshot-paste-to-ai-integration · key of ## 3. Observability Harness Contract
contract/send-line/client-side-refusal · the send line · client-side send-refused — The caller-side send-refused{side:"client"} has corr null.
contract/send-line/cursor-uniqueness · the send line · cursor corr uniqueness — A send cursor is a byte offset into the target's own events.ndjson, unique per target only; refusals and later sends can share it, so one send is identified by target instance, corr, conn and rpc_id.
contract/send-line/unconfirmable · the send line · unconfirmable send — An unconfirmable send logs one send-issued and one send-confirmed{confirmed:false} under the same corr, with no send-refused and no hook side.
contract/send-line/wrapper-side · the send line · wrapper side — send-* lines whose corr is a send cursor are emitted only by the target wrapper; only its send-refused line carries side:"wrapper", its send-issued and send-confirmed lines carry no side, and a client-side send-refused carries side:"client" and no corr.

## trace-context-propagation · key of ## 3. Observability Harness Contract
decision/channel-join-key-conn-corr · Channel join key is conn and corr — The merged-log join key for channel lines is (conn, corr); instance is deliberately not part of it. · why: Each side logs its own instance, so the two sides of one call carry different instance values.
decision/explicit-correlation-values · Correlation values passed explicitly — corr, conn and instance are passed as explicit values into each thread or task and set on each event, never recovered from span context. · why: Spans are not serialized.
decision/span-carried-into-tasks-and-threads · Span not carried into tasks and threads — Spawned async tasks and std thread closures take no span from their spawner: spans never reach a log line, and correlation travels as explicit typed fields.
contract/diagnostics-line/corr-on-send-lines · the diagnostics line · corr on send lines — On send lines, corr carries the send cursor; a send refused before send-issued carries the target's events.ndjson end offset at refusal, a log join key only, never returned to the caller.
contract/send-line/conn-rpc-id · the send line · conn and rpc_id — Every wrapper send-* line carries conn (or srv_conn) and rpc_id, and (conn, rpc_id) equals (conn, corr) on that call's channel-request and channel-response on both sides.
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
