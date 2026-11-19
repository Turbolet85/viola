# obs-plan · the rows 2026-11-19-the-2-1-328-recorded-fixture-set touches — picked 3 of 1028 · rule any · pin 3058

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/absolute-paths-scrubbed-externally · Absolute paths scrubbed from external output — Absolute paths in snapshot.json (endpoint, pinned_bin) and in config values are scrubbed from external output and allowed in diagnostics/. · r1
decision/agents-probe-output-untrusted · Agents probe output untrusted — The output rows of the claude agents --json probe are untrusted. · r1
decision/agents-probe-parse-failure-unknown · Agents probe parse failure becomes unknown — A parse failure of the claude agents --json probe becomes status "unknown" plus a parse-rejected counter; an unparsed --version answer leaves cli_version absent and the version unverified. · r1
decision/assistant-message-never-logged · Assistant message never logged on readback — On `wait` / `last` readback, the `last_assistant_message` content never appears in log fields. · r1
decision/channel-from-unauthenticated · Channel sender logged as unauthenticated — The `from` of an IPC channel call is logged as unauthenticated. · r1
decision/channel-json-rpc-over-ndjson · Channel is JSON-RPC 2.0 over ndjson — The viola-channel crate speaks JSON-RPC 2.0 over ndjson on local Unix sockets, one endpoint per viola run, with a sync client and server and a Tokio client behind the tokio feature. · r1
decision/channel-lines-carry-no-bodies · Channel log lines carry no bodies — IPC channel log lines never carry `params` or result bodies. · r1
decision/channel-sync-instrumentation · Sync instrumentation in viola-channel — Instrumentation in viola-channel must be sync, because Tokio is forbidden in that crate's graph. · except: The tokio feature, when enabled. · r1
decision/child-exit-from-process-handle · Child exit from the process handle — The wrapped child's exit and its process-exit event come from the wait on the process handle, never from reader EOF. · r1
decision/claude-fixtures-scrub-required · Claude fixtures are scrub-required — Build and CI infrastructure is not instrumented, and the claude fixtures are scrub-required data, never a telemetry source. · why: Not product runtime. · r1
decision/claude-internals-not-a-contract · Wrapped claude internals are not a contract — The wrapped claude CLI is a third-party binary run unmodified and instrumented at its boundary only (spawn and exit, paste writes, the vt100 readiness-gate outcome); its internal state and transcripts are never read as a contract. · except: A send's echo confirmation counts its stdout tag in the transcript, keeping no byte; verify writes and removes its temp folder's trust record; a verified start sets its folder's trust flag only. · r4 · stale
decision/cli-file-sink-only · Short-lived CLI logs to a file sink only — Short-lived CLI verbs log only to a JSON-per-line file sink in diagnostics when an instance resolves, never to stdout. · why: Stdout carries the --json results. · r1
decision/corr-field-propagation · Propagation through the corr field — Correlation across the viola-channel is carried by the corr field in log lines of the process on each side. · why: Channel params declare no trace-context field. · r1
decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits. · r2
decision/diagnostics-owner-only-modes · Diagnostics files are owner-only — Every diagnostics file is mode 0600 inside a 0700 directory. · r1
decision/dialog-text-never-logged · Dialog text never logged on answer — On the dialog to `answer` path, the question, answer and tool `input` text never appear in log fields. · r1
decision/distinct-detail-code-per-exit-cause · Distinct detail code per exit cause — Each cause behind exit 21 and exit 1 gets a distinct detail code with no path or pid. · except: A strict-modes check refused before any log opens writes no line; its cause is the fixed stderr pair or the --json document. · r1
decision/drift-reports-diagnostics-only · Drift reports go to diagnostics only — Drift reports of viola-agent-claude go to diagnostics/ only and nowhere else. · why: The outputs of viola-agent-claude are the most sensitive in the product. · r1
decision/drift-reports-to-diagnostics · Drift reports go to diagnostics only — Hook drift reports are written to `diagnostics/` only. · r1
decision/error-chain-fixed-stderr-message · Error chains reach stderr as fixed messages — In the viola root binary, the only crate that uses anyhow, an error chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to the diagnostics logs only. · r1
decision/escape-controls-in-terminal · Control characters escaped in terminal rendering — C0 and C1 control characters are escaped in any human terminal rendering of logged rows. · r1
decision/event-log-never-rotated · Event log never rotated — The events.ndjson product event log is never rotated; its retention is the whole lifetime of the session home. · why: byte offsets into the log are cursors · r1
decision/event-payload-not-in-process-logs · Event payloads kept out of process logs — The payload content of events.ndjson is never copied into process-log lines. · why: events.ndjson is itself the product audit stream. · r1
decision/exit-1-detail-codes · Distinct detail codes for exit-1 causes — On the `viola run` start sequence to child spawn, each exit-1 cause logs a distinct `detail` code with no path or pid in it. · r1
decision/hook-fail-open-silent · Hook fails open silently — Hook fail-open paths log only to `hook-<name>.ndjson` and exit 0 with stderr empty. · r1
decision/hook-log-line-single-append-write · One write per hook log line — Each viola hook log line is one write on an append-mode handle. · r1
decision/hook-log-write-failure-swallowed · Hook log write failure swallowed — A failed write to the hook log is swallowed, so the hook still exits 0 and never writes to stderr. · r1
decision/hook-logger-init-no-blocking-io · Hook logger init adds no blocking I/O — Logger initialisation in viola hook reads the diagnostics level, runs the strict-modes checks, creates or narrows the home and diagnostics/ to 0700 and opens the role file once for append at 0600, adding no other I/O. · r1
decision/hook-silent-no-op-without-instance · Hook silent no-op without an instance — Without VIOLA_NAME, on an instance the instance check refuses, or on a VIOLA_DIR naming an instance directory that does not exist, viola hook is a silent no-op: exit 0, no file opened, no line, no frame. · r1
decision/hook-stderr-forbidden · No stderr from hooks — viola hook writes nothing to stderr. · r1
decision/hook-stdin-correlation · Hook stdin correlated by instance and dialog — Hook stdin from Claude Code carries no inbound trace context, so it is correlated by `instance` and `dialog_id`. · r1
decision/hook-stdout-decision-body-only · Hook stdout carries the decision body only — viola hook stdout carries only the decision body. · r2
decision/instance-never-from-working-directory · Instance never from the working directory — The instance comes from VIOLA_NAME or an argument, never from the working directory. · r1
decision/level-filter-not-from-env · Level filter never read from RUST_LOG — The log level filter is never read from `RUST_LOG`. · r1
decision/mcp-log-file-sink · MCP logs to a file sink, never stdout — The `mcp` process always logs to the file sink `<home>/diagnostics/mcp.ndjson`, with no stderr fallback: a file that cannot be opened exits 1 printing nothing, and the log writer is set explicitly. · why: stdout is forbidden because it carries MCP frames · r1
decision/mcp-rejections-codes-only · MCP rejections carry codes only — MCP isError results and rejected tool calls carry codes only (refusal, detail, instance-unreachable, wrapper-fault, invalid-params), with no upstream text, absolute paths or type names. · r1
decision/mcp-stdio-server-tokio-allowed · MCP server on stdio with Tokio allowed — The `viola mcp` surface is an internal stdio MCP server, and Tokio is allowed in it. · r1
decision/mcp-stdout-frames-only · MCP stdout carries frames only — No telemetry from the viola-mcp server may go to stdout. · why: Stdout is reserved for MCP frames. · r1
decision/ndjson-logs-on-disk · Bridge logs as ndjson on disk — The bridge's logs are ndjson on disk, and the event log serves as the audit trail. · why: logs on disk survive restarts on either side · r1
decision/no-compliance-audit-trail · No compliance audit trail — Observability keeps no compliance audit trail; events.ndjson is the product audit stream, not a compliance trail. · why: the security tier is Minimal and no compliance triggers apply · r1
decision/no-env-log-levels · No per-module log levels from the environment — Per-module log levels are never read from environment variables. · why: Environment variables are not a configuration channel. · r1
decision/no-error-reporter · No opt-in error reporter — The product has no opt-in error reporter; the Standard default is replaced by a Minimal-tier shape. · why: Upstream bans outbound network calls, reserves stdout on 3 of 4 roles, and bans Tokio and C crates in the sync graph. · r1
decision/no-log-rotation · No log rotation — There is no rotation of events.ndjson or diagnostics/*.ndjson in v1, retention being the session home's lifetime, and any future diagnostics/ rotation must keep 0600/0700 and never touch events.ndjson offsets. · r1
decision/no-network-otlp-exporter · No network OTLP exporter — Telemetry has no network OTLP exporter, loopback included; the Standard default is replaced by a Minimal-tier shape. · why: Upstream bans outbound network calls and new listeners without a Security Decisions Log entry; viola makes no outbound network calls. · r1
decision/no-otel-sdk-exporter · No OTel SDK exporter — No OTel SDK exporter is initialised; any OTel bridge may only feed an in-process or file sink and must be Tokio-free. · why: outbound network calls are banned, a new listener needs a Security Decisions Log entry, stdout is reserved on hook, mcp and run, and cargo deny bans Tokio in sync crates · except: an optional Tokio-free OTel bridge feeding an in-process or file sink · r1
decision/no-traceparent-on-local-socket · No W3C traceparent on the local socket — Calls from CLI, MCP and hook to the wrapper carry no W3C `traceparent`; correlation uses `corr` (JSON-RPC `id`, `dialog_id`, send `cursor`) plus `process` and `instance`. · why: channel `params` declare no trace-context field · r1
decision/normal-mode-not-inverted · Normal observability mode, not inverted — Viola is not a telemetry product: observability runs in normal mode, and events.ndjson is a product audit log, not a telemetry-collector function. · r1
decision/obs-deps-tokio-free · Obs dependencies Tokio-free and C-free — Obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass cargo deny. · guard: cargo deny · r1
decision/obs-tier-standard · Observability tier Standard — The observability tier is Standard (1), with Minimal-tier exporter carve-outs. · why: The tests tier is Comprehensive (2) and obs sits one step below it within the plus-or-minus-one band; the Minimal security tier gives no Comprehensive driver. · r1
decision/one-log-file-per-role · One log file per process role — Each process role writes its own file under `diagnostics/`, no role has a stdout sink, and no role falls back to stderr JSON when its file cannot be opened. · r1
decision/one-write-per-log-line · One write per log line — Each formatted log line is emitted as a single write on an append-mode handle. · why: Concurrent hook processes share one file. · r1
decision/os-apis-at-call-sites · OS APIs observed at call sites — OS APIs (openpty, Unix socket, filesystem locks and atomic rename, sysinfo) and the hyprctl spawn are instrumented at viola's call sites only, never in their internals. · r2
decision/parse-failures-unknown-counter · Parse failures surface as unknown plus a counter — A child-output parse failure surfaces as `unknown` plus a counter. · except: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown. · r3
decision/pty-byte-stream-never-logged · PTY byte stream never logged — The PTY byte stream is never logged; the viola-pty seam is instrumentable, and the openpty OS API behind it is boundary-only. · why: The PTY byte stream is user content. · r1
decision/redaction-not-disableable · Redaction cannot be disabled — No verbosity control, env, flag or config may disable or bypass redaction. · r1
decision/refusal-telemetry · Refusal telemetry for filesystem refusals — Pinned-exe hash mismatches and ignored symlinks produce refusal telemetry. · except: The transcript echo read and the verify and start trust-record writes refuse a symlinked, non-regular or foreign file with no refusal line; the send's lines, stamps or trust-record word show it. · r4
decision/refused-diagnostics-drops-line · Refused diagnostics file drops its line — A refused `diagnostics/` folder or file is treated as a failed open and its log line is dropped. · r1
decision/rmcp-stdio-mcp-server · rmcp stdio MCP server — The viola-mcp crate is an rmcp stdio MCP server. · r1
decision/run-silent-terminal · viola run silent on the terminal — viola run writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns. · r1
decision/run-sync-instrumentation · Sync instrumentation in viola run — Instrumentation in viola run is sync, with no Tokio. · r1
decision/self-reported-from-not-identity · Self-reported from is not an identity — The self-reported from on the viola-channel is never logged as an authenticated identity. · r1
decision/send-text-confined · Send text confined to instance diagnostics — `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text. · r1
decision/send-text-never-logged · Send text never logged — Sent text never appears on stderr, in --json output, in MCP results or in any process-log field outside instances/<name>/diagnostics/. · r1
decision/serde-errors-fixed-message · Serde error chains become a fixed message — A serde-sourced anyhow error chain is replaced by a fixed message before it reaches stderr. · r1
decision/single-file-sink-no-os-exporter · One file sink and one line schema — Telemetry on Linux goes to one file sink with one line schema, with no OS-specific exporter. · r1
decision/spans-not-wire-contract · Tracing spans are not part of the wire contract — Span names are internal `tracing` spans in `<area>.<operation>` snake_case form; the agent-facing contract is the closed `event` enum plus `corr`. · why: the config sets `with_current_span(false)`, so spans do not reach the line · r1
decision/state-sync-instrumentation · Sync instrumentation in viola-state — Instrumentation in viola-state must be sync. · why: The Stack bans Tokio in viola-state. · r1
decision/stderr-fixed-strings · Fixed strings on stderr — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text. · r1
decision/strict-modes-failure-lines · Where a strict-modes failure is logged — A strict-modes failure has a log line only on the `ledger-stamps` read in the `run` version gate and on `verify`'s own `process-exit` for a refused stamp write. · r1
decision/stripped-claude-env-never-logged · R8-stripped CLAUDE values never logged — R8-stripped CLAUDE* environment values are never logged anywhere. · why: The viola-agent-claude outputs are the most sensitive in the product. · r1
decision/stripped-env-never-logged · Stripped CLAUDE env values never logged — No R8-stripped `CLAUDE*` value appears in any sink or fixture; the strip is logged as a count or variable names only. · r1
decision/tracing-json-line-file-pipeline · tracing JSON-per-line file pipeline — Telemetry is tracing plus tracing-subscriber writing JSON-per-line files under diagnostics/, correlated by corr and consumed through agent-run logs and status. · why: The pipeline is the one the tests plan binds. · r1
decision/tracing-json-pipeline · tracing with the tracing-subscriber JSON formatter — The `tracing` facade plus the tracing-subscriber JSON formatter is the binding logging pipeline, and JSON-to-file is the only logging mode. · r1
decision/typed-keys-never-logged · Typed keys never logged — Keys a human types never appear in a log line; the wheel product event in events.ndjson is the audit record. · r1
decision/viola-core-emits-no-telemetry · viola-core emits no telemetry — The viola-core crate supplies log-field values but emits nothing and is not instrumented. · why: A pure type crate with no I/O. · r1
contract/agent-run-status/state-last-error · agent-run status · state and last_error — The aggregate health snapshot gives `state` (`ready`, `degraded` or `down`) and `last_error` (null, or `list-exit-<code>` built from the list exit code). · r1
contract/diag-line/instance · diagnostic log line · instance — The instance's `ViolaName`; the key is absent when the process has no instance. · r1
contract/diag-line/process · diagnostic log line · process — The role of the writing process, one of run, hook, mcp, cli; required on every line and matching `sender` / `writer`. · r1
contract/diag-line/service-name · diagnostic log line · service.name — Hardcoded to "viola". · r1
contract/diag-line/service-version · diagnostic log line · service.version — The crate version, fixed at compile time. · r1
contract/diagnostics-files/cli · the diagnostics files · cli — Role cli writes <home>/diagnostics/cli-<name>.ndjson, only when an instance resolves. · r1
contract/diagnostics-files/hook · the diagnostics files · hook — Role hook writes <home>/diagnostics/hook-<name>.ndjson, shared by concurrent hook processes; a write failure is swallowed. · r1
contract/diagnostics-files/mcp · the diagnostics files · mcp — Role mcp writes its role file mcp.ndjson in the home's diagnostics folder, which never carries user content; a role file that cannot be opened ends the server with exit 1 and nothing on stderr. · r1
contract/diagnostics-files/run · the diagnostics files · run — Role run writes <home>/diagnostics/run-<name>.ndjson. · r1
contract/harness-logs/filters · harness logs command · filters — Output can be filtered by `--instance`, `--kind` and `--process`. · r1
contract/harness-logs/merge · harness logs command · merged sources — Merges `events.ndjson` lines (`src:"events"`, with byte `offset`) with `diagnostics/*.ndjson` lines (`src:"diag"`). · r1
contract/harness-logs/output · harness logs command · output — Output is ndjson on stdout, ready for `jq` or pasting to an agent. · r1
contract/harness-logs/torn-line · harness logs command · torn lines — A torn line is emitted with its `src` and `offset` and `"torn":true`, never dropped. · r1
contract/hook-log-file/path · the viola hook log file · path — viola hook logs only to a file sink at diagnostics/hook-<name>.ndjson. · r1
contract/json-rpc-errors/32603 · viola-channel JSON-RPC errors · -32603 — Error -32603 is "internal error" with data:null. · r1
contract/log-line/process · every log line · process — Every log line carries a process key naming its role: one of run, hook, mcp, cli. · r1
contract/mcp-stdin/max-frame · viola mcp stdin · MAX_FRAME — Stdin is read in lines of at most `MAX_FRAME`; a longer line is refused whole and the session ends as at a closed stdin. · r1
contract/process-exit-detail/socket-dir-refused · process-exit detail codes · socket-dir-refused — Carried by the line of a refused socket directory, where it has one; a refused socket directory is no strict-modes failure. · r1
contract/process-exit-detail/strict-modes-failed · process-exit detail codes · strict-modes-failed — Carried by `verify`'s `process-exit` for a refused stamp write, with exit 1. · r1
contract/process-log-event/release-from-driver · the process log event field · release-from-driver — Release from the driver is logged as its own distinct event. · r1
contract/run-log-file/path · the viola run log file · path — viola run logs only to a file sink at diagnostics/run-<name>.ndjson. · r1
contract/viola-mcp-tools/answer · viola-mcp stdio server tools · answer — The viola-mcp stdio server offers an answer tool. · r1
contract/viola-mcp-tools/last · viola-mcp stdio server tools · last — The viola-mcp stdio server offers a last tool. · r1
contract/viola-mcp-tools/list · viola-mcp stdio server tools · list — The viola-mcp stdio server offers a list tool. · r1
contract/viola-mcp-tools/send · viola-mcp stdio server tools · send — The viola-mcp stdio server offers a send tool. · r1
contract/viola-mcp-tools/wait · viola-mcp stdio server tools · wait — The viola-mcp stdio server offers a wait tool. · r1
absence/audit-log-compliance-trail — An audit-log compliance trail · r1
absence/otlp-receiver — An OTLP or pulse-class telemetry receiver · r1
absence/pii-scrubbing-regime — A PII-scrubbing regime for diagnostics lines beyond the NEVER-log floor · r1

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/agents-cli-instrumented-at-boundary · claude agents CLI instrumented at spawn, exit and parse — The external `claude agents --json` CLI is instrumented only at its spawn, its exit and its parse outcome. · why: An external CLI. · r1
decision/agents-cli-output-untrusted · claude agents output untrusted — The output rows of `claude agents --json` are treated as untrusted. · r1
decision/agents-cli-parse-failure-unknown · claude agents parse failure reads unknown — A failed parse of `claude agents --json` output reads "unknown" plus a counter. · r1
decision/anyhow-error-capture-boundary · The anyhow crate is the error-capture boundary — The only crate that uses anyhow is the error-capture boundary, with context chains only at dispatch; the full chain goes to diagnostics only. · r1
decision/channel-log-scrub-rule · Channel log scrub rule — Channel request and response lines log only the method, id or corr, the result class (ok, refusal or error code), the sender version and the closed detail word, never params or result bodies. · r1
decision/child-exit-from-handle-wait · Child exit from the process handle — The child's process-exit comes from the wait on its process handle, never from reader end-of-file. · why: Child exit is detected on the process handle. · r1
decision/claude-boundary-only-instrumentation · Boundary-only instrumentation of claude — The hosted claude binary is instrumented only at its boundary: spawn and exit, paste writes, the vt100 readiness-gate outcome, and the start's one trust-record line for the trust record it writes in the claude config. · why: claude is a third-party binary run unmodified. · r2
decision/claude-exit-on-process-handle · Claude exit detected on the process handle — The exit of the hosted claude process is detected on the process handle, never on EOF. · r1
decision/config-observed-at-load · Configuration observed only at load — Configuration is observed only at its parse at load time, logged as skipped-key counts. · why: Static configuration and data. · r1
decision/controls-escaped-in-terminal-output · Controls escaped in terminal output — C0 and C1 control characters are escaped in any human terminal rendering of logged rows. · r1
decision/correlation-in-corr-field · Correlation in the corr log field — Correlation across a channel call travels in the corr log field; channel params declare no trace-context field. · r1
decision/detail-codes-no-paths · Detail codes carry no path or pid — No path or pid appears in a detail code. · r1
decision/deterministic-event-emission · Deterministic event emission — Events are emitted deterministically, keyed on hook events and events.ndjson offsets. · except: A local command's echo confirmation keys on the stdout tag count in its session transcript. · r2
decision/diagnostics-checked-at-open · Diagnostics checked before open — Before a role or detail file is created or narrowed, the existing diagnostics/ folder and the file are checked, a symlink refused and no group or other bit allowed; a refusal is a failed open. · r1
decision/diagnostics-file-modes · Diagnostics file and directory modes — Diagnostics files are 0600 inside 0700 directories. · guard: secret-scan test · r1
decision/diagnostics-owner-only-permissions · Diagnostics owner-only permissions — Files under diagnostics/ are 0600 inside 0700 directories. · guard: secret-scan test · r1
decision/drift-reports-detail-only · Drift reports in the detail file only — A full serde drift report goes only to the instance-scoped detail file, as its drift_report field. · r1
decision/exit-code-log-field · Exit code as a log field — The exit code (0, 1, 2, 10, 12–14, 20, 21), the machine-readable outcome, is carried as a log field; 11 is retired and never reused. · r2
decision/fault-telemetry-per-class · Fault telemetry per class — Each fault class emits its own telemetry: a healed torn ndjson line with file and offset, a corrupt or unsupported snapshot recovered by replay, a child exit seen without EOF, an endpoint vanished during wait, and a stale heartbeat against a live pid. · r1
decision/fixed-stderr-messages · Fixed stderr refusal and fault lines — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text. · r1
decision/fixtures-not-telemetry · Recorded fixtures are no telemetry source — Recorded fixtures are scrub-required data, not a telemetry source, and are excluded from the observability plan. · why: Not product runtime. · r1
decision/from-not-authenticated · from never logged as authenticated — The self-reported from field is never logged as an authenticated identity. · r1
decision/heartbeat-staleness-threshold · Heartbeat staleness threshold — A heartbeat flips to stale after more than 5 s: 4.9 s reads live, 5.1 s reads stale or gone. · r1
decision/home-level-mcp-log-no-user-content · Home-level MCP log holds no user content — An mcp.ndjson at home level never carries user content. · r1
decision/hook-fail-open-logged-to-hook-file · Hook fail-open logged to the hook file — Hook fail-open paths (oversize stdin, malformed JSON, channel failure, server-verify mismatch, refused socket directory) are logged to hook-<name>.ndjson only; a missing VIOLA_NAME writes no line. · r1
decision/hook-instance-check-before-obs-init · Hook instance check before logging starts — The hook's event and dialog paths run the instance strict-modes check before reading the diagnostics level and initialising logging. · except: The --capture arm runs ahead of the check. · r2
decision/hook-latency-budget · Hook latency budget — Each hook runs within max < 1.0 s, and logger init adds no blocking I/O beyond one file open and append. · guard: run --perf hook rows under SPINE_DEADLINE · r2
decision/hook-lines-required · Hook invocation and decision lines required — Every hook process in a trusted wrapped session writes its hook-invoked and hook-decision lines, bar a PreToolUse for a tool outside the dialog matcher; a hook outside such a session or on a --capture probe writes neither. · r1
decision/hook-log-failure-swallowed · Hook log write failure swallowed — A failed log write in a hook is swallowed and the hook still exits 0. · r1
decision/hook-mcp-instrumented-at-boundary · Hook and MCP instrumented at the boundary — Hook and MCP traffic is instrumented only at viola's own boundary: hook-invoked and hook-decision, MCP tool-call handling, and the viola mcp stdin cut. · why: The caller is Claude Code, so viola sees only the inbound stdin payload and its own response, and no trace context arrives from Claude Code. · r1
decision/hook-never-writes-stderr · Hook never writes stderr — A hook process never writes to stderr. · r1
decision/hook-panic-one-line · Hook panic logged as one line — A panic in a hook is caught with catch_unwind and logged as one level:"ERROR", event:"panic" line. · r1
decision/hook-silent-without-trusted-instance · Hook silent without a trusted instance — A hook with no VIOLA_NAME, or whose instance is absent or refused by its strict-modes check, exits 0 with no config read, no stdin read, no log line, no stderr, no frame and no SessionEnd append. · r1
decision/instance-from-name-not-cwd · Instance from its name, never the working directory — The instance comes from VIOLA_NAME or an explicit argument, never from the working directory. · r1
decision/instrument-call-sites-not-os-internals · OS operations instrumented at viola's call sites — OS-level operations (bind, connect, lock, rename, kill fallback, the server check's peer reads, the strict-modes stat reads, the socket directory check, the hyprctl spawn) are instrumented at viola's own call sites, not inside the OS. · r2
decision/ipc-crate-sync-instrumentation · Synchronous instrumentation in the IPC crate — Instrumentation in the interprocess IPC crate must be synchronous. · why: Stack forbids Tokio in the crate's graph unless its feature is enabled. · r1
decision/json-to-file-only-mode · JSON-to-file the only logging mode — JSON-to-file is the only logging mode; there is no separate agent-mode flag. · r1
decision/latency-aggregation-outside-process · Latency aggregation outside the process — Percentile aggregation is done by the harness or jq over the ndjson, never by an in-process metrics pipeline. · r1
decision/linux-only-os-target · Linux-only OS target — The OS target is Linux only: the core on any current distribution, CI on ubuntu-latest. · r1
decision/log-append-atomicity · Append atomicity for concurrent writers — Concurrent hook writers append to a log atomically through POSIX O_APPEND. · r1
decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but a required one the product still emits is never renamed or removed. · why: The harness greps on them. · r2
decision/log-file-basenames-follow-tests · Log file names follow tests — Process-log files are <home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson, as tests require. · why: Tests is upstream; arch and security place diagnostics under the instance directory instead. · r1
decision/log-level-not-from-env · Log level never from the environment — The log level filter is never read from RUST_LOG; any verbosity control comes from config.json or a CLI flag and never bypasses redaction. · why: Environment variables are not a configuration channel, and no env, flag or config value may disable a control. · r1
decision/mcp-log-sink · MCP log sink — viola mcp always logs to <home>/diagnostics/mcp.ndjson once the home and its diagnostics/ pass strict-modes; a file that cannot be opened exits 1 printing nothing, with no stderr fallback. · r1
decision/mcp-no-telemetry-on-stdout · No telemetry on the MCP server's stdout — The MCP server sends no telemetry to stdout. · why: stdout is reserved for MCP frames. · r1
decision/mcp-results-codes-only · MCP rejections carry codes only — MCP isError results and the log lines of rejected calls carry codes only. · r1
decision/messaging-secrets-never-logged · Messaging secrets never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every R8-stripped CLAUDE* value are never logged in any sink, detail files and fixtures included. · guard: secret-scan test · r1
decision/no-formal-slo · No formal SLO — No formal latency or availability SLO is held beyond the tests perf gates. · r1
decision/no-log-rotation-v1 · No log rotation in v1 — events.ndjson and diagnostics/*.ndjson are not rotated in v1; any future rotation keeps 0600/0700 permissions and keeps wait after and send cursor offsets valid. · why: The trigger is operational, not compliance: the security tier is Minimal with no compliance triggers. · r1
decision/no-otel-exporter · No OTel SDK exporter — v1 initialises no OTel SDK exporter; the tracing facade plus the tracing-subscriber JSON formatter is the binding pipeline. · why: No outbound network calls, a new listener needs a security decision, stdout is reserved on hook, mcp and run, and cargo deny bans Tokio and C-building crates. · except: An optional OTel bridge layer may feed only an in-process or file sink and stays Tokio-free on the sync crates. · r1
decision/no-process-log-without-instance · No process log without an instance — A short-lived CLI verb run with no resolved instance writes no process-log file; its human stderr carries only fixed-message lines, and its outcome is the exit code and stdout. · r1
decision/no-secrets-in-log-lines · No secrets in log lines — No log line may contain any stripped CLAUDE* value. · guard: secret-scan test · r2
decision/no-telemetry-from-screen · No telemetry from screen content — No telemetry is derived from screen content; vt100 serves gate outcomes only. · r1
decision/obs-deps-no-c-building-crates · Obs dependencies build no C — Observability dependencies add no C-building crates. · r1
decision/obs-deps-pass-cargo-deny · Obs dependencies pass cargo deny — Observability dependencies pass cargo deny on its one target, x86_64-unknown-linux-gnu. · guard: cargo deny · r1
decision/one-linux-log-sink · One file sink on Linux — One file sink and one line schema serve Linux, with no OS-specific exporter. · r1
decision/panic-hook-replaced · Panic hook replaced by a one-line writer — Each process replaces the default panic hook, as the first statement in main and before any child spawns, with one that writes one JSON line and nothing to stderr. · why: The default hook prints to stderr, which breaks the hook contract and is the child's terminal under run. · r1
decision/parse-failures-surface-as-unknown · Parse failures surface as unknown — A parse failure surfaces as the value unknown plus a counter. · except: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown. · r3
decision/per-parser-parse-failure-counts · Per-parser parse-failure counts — Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, the hyprctl replies, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead. · r2
decision/pinned-copy-race-adds-no-line · Pinned-copy race adds no telemetry — A pinned-exe hash mismatch reached when a concurrent first start wins the no-clobber placement leaves the file as found and adds no line, field or detail code beyond pinned-hash-mismatch. · r1
decision/process-init-order · Process logging init order — Each process installs its panic hook, parses arguments, resolves home and instance and runs its strict-modes check, opens its diagnostics file append-mode 0600 in a 0700 directory, installs the subscriber, then emits process-start. · why: For run this completes before the start sequence so the collision check and version gate are logged; for mcp, before the stdio transport starts. · r1
decision/process-logs-file-sink-only · Process logs to a file sink only — Process logs are written only as JSON lines to one file per process role under diagnostics/; there is no network exporter and no stdout sink. · why: viola makes no outbound network calls, and stdout carries --json results. · r1
decision/r8-strip-logged-as-names-only · R8 strip logged as names only — The R8 environment strip is logged as a count and lists of stripped and kept variable names, and environment maps never pass to a span or event; no value is read, bar the kept CLAUDE_CONFIG_DIR value that viola verify and the run and revive start read, and no value is logged. · r3
decision/redaction-cannot-be-disabled · Redaction cannot be disabled — No environment variable, flag or config setting may disable log redaction. · r1
decision/refused-peer-dropped-without-line · Refused channel peer dropped without a log line — A channel stream whose peer euid is refused is dropped on the accept thread before its worker thread and before any frame: the peer reads end-of-file, and the wrapper writes no log line and nothing on the terminal. · why: There is no frame to key a line on, and channel-response's detail is closed. · r1
decision/run-silent-on-terminal · run silent on the terminal — run writes nothing to the terminal while the child runs; its process log goes only to diagnostics/run-<name>.ndjson. · why: The terminal carries the child's screen. · r1
decision/send-refused-by-each-deciding-side · Send refusal logged by each deciding side — A send-refused with detail control-character or empty-text is emitted by each side that decides it, client and wrapper, without the text. · r1
decision/serde-chain-fixed-message · Serde error chains become a fixed message on stderr — An error chain that contains a serde source becomes a fixed message before it reaches stderr. · r1
decision/serde-errors-become-fixed-message · Serde errors become a fixed message — An error chain holding a serde source is replaced by a fixed message before it reaches stderr. · r1
decision/server-check-peer-credentials · Server check by peer credentials — The server check reads the peer pid by SO_PEERCRED and the peer euid; a mismatch of either reads server-verify-failed and an unreadable euid is refused. · r1
decision/single-line-log-records · Single-line log records — Every log record is one JSON line parseable by jq; never a multi-line stack trace. · r1
decision/skipped-counters · Skipped counters — Unknown event kinds, unknown fields and torn lines are each counted in skipped counters. · r1
decision/socket-dir-one-code · One code for socket-directory causes — A refused check, a failed create or a lost race on the socket directory all read the one code socket-dir-refused. · r1
decision/state-crate-sync-instrumentation · Synchronous instrumentation in the state crate — Instrumentation in the state crate built on ndjson, atomic-write-file and sysinfo must be synchronous. · why: Stack bans Tokio in that crate. · r1
decision/strict-modes-refusal-writes-no-line · Strict-modes refusal writes no line — A strict-modes check refused before any log opens ends the process with nothing under the home opened and no log line; its cause is carried only by the fixed stderr pair or the --json document. · why: Ratified as the answer to whether a refused strict-modes check logs. · except: The ledger-stamps read in run's version gate and a refused update_stamps in viola verify each leave a line. · r1
decision/stripped-claude-values-never-logged · Stripped CLAUDE values never logged — R8-stripped CLAUDE* values are never logged anywhere. · why: Its outputs are the most sensitive in the product. · r1
decision/structured-json-process-logs · Structured JSON process logs — Every process role (run, hook, mcp, cli) writes structured JSON process logs under diagnostics/, which harness logs merges with events.ndjson. · r1
decision/subscriber-explicit-writer · Subscriber installed with an explicit writer — The tracing subscriber is always installed with an explicit writer, never stdout. · why: tracing-subscriber fmt() writes to stdout unless a writer is configured, and stdout carries MCP frames. · r1
decision/sync-instrumentation-no-tokio · Sync instrumentation without Tokio — Instrumentation in the sync crates runs on std threads with no Tokio. · why: Hybrid concurrency. · guard: cargo deny ban · r1
decision/tests-event-list-adopted · Tests event list adopted — The tests' field and event list is adopted verbatim, with send-issued and send-refused among the logged events. · r1
decision/tracing-json-logging-pipeline · tracing with a JSON subscriber for logging — Logging and spans go through the Rust tracing facade with a tracing-subscriber JSON structured logger. · r1
decision/tracing-subscriber-logger · tracing-subscriber is the logger — The logger dependency is tracing-subscriber at version 0.3.20 or later. · r1
decision/tracing-subscriber-version-floor · tracing-subscriber version floor — tracing-subscriber is held at version 0.3.20 or later. · why: The security floor for RUSTSEC-2025-0055. · r1
decision/version-rejections-counted-by-detail · Version rejections counted by detail — Higher-v rejections are counted by the detail unsupported-version, not by the -32602 code. · why: The -32602 code also carries invalid-params and release-from-driver. · r1
contract/channel-params/sender · Channel params · sender — params.sender carries the peer's version, CARGO_PKG_VERSION. · r1
contract/channel-response/detail · Channel response line · detail — On a channel-response answering -32602, detail is one closed word: unsupported-version, invalid-params or release-from-driver, on both the server's and the client's line; it is absent on every other channel-response line and names no param or value. · r1
contract/channel-response/internal-error · Channel response line · -32603 — A -32603 is logged at ERROR with result_class error, error_code -32603 and no detail; the fixed text internal error is only the reply's message, never a log field. · r1
contract/channel-response/invalid-params-causes · Channel response line · invalid-params causes — detail invalid-params covers send's invalid params, a release whose from is not a string or that carries any budget key, and a link or unlink whose driver or driven is missing, not a string or no valid name, whose driven is not the serving wrapper's own name, or whose driver equals driven. · r2
contract/cli-frame-verbs/strict-modes-failed · CLI frame verbs · strict-modes-failed — A strict-modes refusal of send, wait, last, answer, pause or release writes no line and prints only the fixed instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, exit 21, with no pid, path or endpoint. · r1
contract/cli-internal-error/stderr-line · CLI internal error · stderr line — Without --json a catch-site fault prints the fixed line `error: internal error` to stderr. · r1
contract/cli-json-error/strict-modes-failed · CLI --json error document · instance-unreachable strict-modes-failed — A refused instance check of send, wait, last, answer, pause, release, link or unlink exits 21 with the instance pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}. · r1
contract/cli-log-file/link-unlink · CLI process-log file · viola link / viola unlink — Log as cli into the driven instance's cli-<driven>.ndjson, never the caller's VIOLA_NAME, once their instance check of it has passed. · r1
contract/cli-log-file/revive-list · CLI process-log file · viola revive --list — Logs as cli into the caller's cli-<VIOLA_NAME>.ndjson once its strict-modes check has passed; no file with no VIOLA_NAME. · r1
contract/cli-log-file/revive-start · CLI process-log file · viola revive start arm — Logs as the wrapper, process="run", into diagnostics/run-<name>.ndjson. · r1
contract/diag-line-event/release-from-driver · Diag line event · release-from-driver — release-from-driver is its own logged event, admitted by the diag-line schema, not a generic -32602 wrapper fault. · r1
contract/dialog-hook/refused-instance-check · Dialog hook · refused instance check — A refused instance check on the dialog hook exits 0 at once with empty streams, no frame and no line. · r1
contract/dialog-log-line/corr · Dialog log line · corr — On dialog lines corr is the dialog_id, which joins hook and wrapper lines; the driver's channel lines carry the JSON-RPC id. · r1
contract/dialog-log-line/required-fields · Dialog log line · required fields — A dialog line carries event, process, instance, corr, dialog_kind, deadline_hit and duration_ms; refusal or detail does not appear on a dialog-raised or dialog-answered line. · r1
contract/dialog-log-line/server-verify-failed · Dialog log line · server-verify-failed — A server-verify failure exits 21 on CLI answer with during connect, and on the dialog hook fails open with corr null and decision_emitted false; the line names no pid, start time, path or endpoint. · r1
contract/dialog-log-line/socket-dir-refused · Dialog log line · socket-dir-refused — A refused socket directory exits 21 on CLI answer with during connect, and on the dialog hook fails open with corr null and decision_emitted false; the line names no pid, start time, path or endpoint. · r1
contract/dialog-log-line/unknown-dialog · Dialog log line · unknown-dialog — The detail unknown-dialog goes with exit 13. · r1
contract/dialog-log-line/unverified-cli · Dialog log line · unverified-cli — The detail unverified-cli goes with exit 12. · r1
contract/dialog-path-log-events/event-order · Dialog path log events · event order — A dialog logs hook-invoked, dialog-raised, dialog-answered, then hook-decision, whose decision is null on deadline expiry. · r1
contract/event-line/shape · events.ndjson event line · line shape — Exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}. · r1
contract/exit-1-detail/already-live · Exit 1 detail codes · already-live — already-live is a distinct detail code for an exit-1 cause, naming no path or pid. · r1
contract/exit-1-detail/cwd-missing · Exit 1 detail codes · cwd-missing — cwd-missing is a distinct detail code for viola revive's exit-1 cause, naming no path or pid. · r1
contract/exit-1-detail/no-session · Exit 1 detail codes · no-session — no-session is a distinct detail code for viola revive's exit-1 cause, naming no path or pid. · r1
contract/exit-1-detail/pinned-hash-mismatch · Exit 1 detail codes · pinned-hash-mismatch — pinned-hash-mismatch is a distinct detail code for an exit-1 cause, naming no path or pid. · r1
contract/exit-1-detail/session-live · Exit 1 detail codes · session-live — session-live is a distinct detail code for viola revive's exit-1 cause, naming no path or pid. · r1
contract/exit-1-detail/socket-dir-refused · Exit 1 detail codes · socket-dir-refused — socket-dir-refused is a distinct detail code for an exit-1 cause, naming no path or pid. · r1
contract/exit-1-detail/squatted-name · Exit 1 detail codes · squatted-name — squatted-name is a distinct detail code for an exit-1 cause, naming no path or pid. · r1
contract/exit-1-detail/stale-heartbeat · Exit 1 detail codes · stale-heartbeat — stale-heartbeat is a distinct detail code for an exit-1 cause, naming no path or pid. · r1
contract/exit-21-detail/endpoint-lost · Exit 21 detail codes · endpoint-lost — endpoint-lost is a distinct detail code for an exit-21 cause, naming no path or pid. · r1
contract/exit-21-detail/instance-dead · Exit 21 detail codes · instance-dead — instance-dead is a distinct detail code for an exit-21 cause, naming no path or pid. · r1
contract/exit-21-detail/server-verify-failed · Exit 21 detail codes · server-verify-failed — server-verify-failed is a distinct detail code for an exit-21 cause, distinct from instance-dead, naming no path or pid. · r1
contract/exit-21-detail/socket-dir-refused · Exit 21 detail codes · socket-dir-refused — socket-dir-refused is a distinct detail code for an exit-21 cause, naming no path or pid. · r1
contract/exit-21-detail/strict-modes-failed · Exit 21 detail codes · strict-modes-failed — strict-modes-failed is a distinct detail code for an exit-21 cause, naming no path or pid. · r1
contract/exit-21-detail/unwrapped-name · Exit 21 detail codes · unwrapped-name — unwrapped-name is a distinct detail code for an exit-21 cause, naming no path or pid. · r1
contract/exit-cause-detail/server-verify-failed · Exit cause detail codes · server-verify-failed — A failed server check of peer euid, peer pid or start time is logged by the CLI and the hook as server-verify-failed; MCP writes no line for it. · r1
contract/exit-cause-detail/socket-dir-refused · Exit cause detail codes · socket-dir-refused — A refused socket directory has its own detail code socket-dir-refused and writes its line (MCP writes none); on run and on a start made by viola revive that line is one process-exit{subject:"self", exit_code:1} before any snapshot, child or bind. · r1
contract/heartbeat-file/tick · Heartbeat file · tick — The heartbeat file is touched every 1 s as the liveness tick. · r1
contract/hook-decision/server-verify-failed · Hook decision line · server-verify-failed — A server-verify mismatch is one hook-decision at WARN with detail server-verify-failed and no frame sent; on a dialog hook, hook-invoked with corr null then hook-decision with corr null and decision_emitted false; no pid, start time, path or endpoint. · r1
contract/hook-decision/socket-dir-refused · Hook decision line · socket-dir-refused — A refused socket directory on viola hook writes one hook-decision in the server-verify form with detail socket-dir-refused, and the hook exits 0. · r1
contract/hook-fail-open/exit · Hook fail-open · exit — A hook that fails open exits 0 and leaves stderr empty. · r1
contract/hook-log-line/duration-ms · Hook log line · duration_ms — duration_ms is on hook-decision, or on process-exit for process hook, for each hook event. · r1
contract/hook-stdout/capture-arms · Hook stdout · --capture arms — The hidden --capture arm writes no log sink and prints nothing, except --answers' probe decision body. · r2
contract/hook-stdout/hook-event · Hook stdout · viola hook <event> — stdout carries only the decision body. · r1
contract/json-rpc-error/32603 · JSON-RPC error · -32603 — The -32603 error carries the message "internal error" with data null. · r1
contract/json-rpc-frame/id · JSON-RPC frame · id — Ids are integers, monotonic per connection, and so not unique across connections or processes. · r1
contract/logs-wrapper/record-envelope · logs wrapper · record envelope — Each process-log line is wrapped as {"src":"diag","file":<basename>,"record":<line>}. · r1
contract/mcp-frame-tool-refusal/no-live-endpoint · MCP frame tool refusal · no live endpoint — A frame tool with no live endpoint returns instance-dead or unwrapped-name and writes no frame. · r1
contract/mcp-frame-tool-refusal/server-verify-failed · MCP frame tool refusal · server-verify-failed — A frame tool whose server check fails or whose server euid is refused returns server-verify-failed and writes no frame. · r1
contract/mcp-frame-tool-refusal/socket-dir-refused · MCP frame tool refusal · socket-dir-refused — A frame tool whose socket directory is refused returns socket-dir-refused, writes no frame and no line. · r1
contract/mcp-frame-tool-refusal/strict-modes-failed · MCP frame tool refusal · strict-modes-failed — A frame tool (send, wait, last, answer) whose instance check refuses returns instance-unreachable with detail strict-modes-failed, writes no frame and no line. · r1
contract/mcp-invalid-params/detail · MCP invalid-params · detail — The detail of an MCP invalid-params is a closed field word: target, text, after, timeout_ms, dialog_id or response. · r1
contract/mcp-iserror/instance-unreachable · MCP isError result · instance-unreachable — The instance-unreachable detail is one closed cause: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed or socket-dir-refused; the CLI's document less v, its text `refused: instance-unreachable`. · r1
contract/mcp-iserror/invalid-params · MCP isError result · invalid-params — The invalid-params detail is a closed field word. · r1
contract/mcp-iserror/refusal · MCP isError result · refusal — A peer refusal that is no code word reads unknown, and a string detail that is no code word reads null. · r1
contract/mcp-iserror/wrapper-fault · MCP isError result · wrapper-fault — The wrapper-fault detail is a closed sanitised object: the fixed message of the five codes with data null or closed values; a code outside the five reads message "wrapper fault", data null. · r1
contract/mcp-server/instance · MCP server identity · instance — The server's own VIOLA_NAME when set, which is also the from it adds. · r1
contract/mcp-stdin/line-within-max-frame · viola mcp stdin · line within MAX_FRAME — A line of at most MAX_FRAME bytes, its newline counted, passes whole. · r1
contract/mcp-stdin/oversize-before-initialize · viola mcp stdin · oversize line before initialize — An oversize line before initialize exits the process 1 with both streams empty and the one parse-rejected record. · r1
contract/process-exit/socket-dir-refused-on-frame-verb · Process exit line · socket-dir-refused on a CLI frame verb — A refused socket directory on a CLI frame verb writes one process-exit with detail socket-dir-refused, exit 21, during connect. · r1
contract/process-exit/socket-dir-refused-on-start · Process exit line · socket-dir-refused on a start — A refused socket directory on a start writes one process-exit with detail socket-dir-refused, exit 1. · r1
contract/process-log-corr/channel-request · Process log corr by event · channel-request — corr is the JSON-RPC request id. · r1
contract/process-log-corr/channel-response · Process log corr by event · channel-response — corr is the JSON-RPC request id. · r1
contract/process-log-corr/dialog-answered · Process log corr by event · dialog-answered — corr is the dialog_id. · r1
contract/process-log-corr/dialog-raised · Process log corr by event · dialog-raised — corr is the dialog_id. · r1
contract/process-log-corr/hook-decision · Process log corr by event · hook-decision — corr is the dialog_id for dialog hooks, otherwise null. · r1
contract/process-log-corr/hook-invoked · Process log corr by event · hook-invoked — corr is the dialog_id for dialog hooks, otherwise null. · r1
contract/process-log-corr/panic · Process log corr by event · panic — corr is null. · r1
contract/process-log-corr/process-exit · Process log corr by event · process-exit — corr is null. · r1
contract/process-log-corr/process-start · Process log corr by event · process-start — corr is null. · r1
contract/process-log-corr/send-confirmed · Process log corr by event · send-confirmed — corr is the send cursor. · r1
contract/process-log-corr/send-issued · Process log corr by event · send-issued — corr is the send cursor. · r1
contract/process-log-corr/send-refused · Process log corr by event · send-refused — corr is the send cursor. · r1
contract/process-log/child-and-probe-lifecycle · Process log · child and probe lifecycle — process-start and process-exit are logged for the claude child and for the claude agents --json and --version probes. · r1
contract/process-log/corr · Process log line · corr — Copied unchanged as a JSON number or string, never renamed. · r1
contract/process-log/event · Process log line · event — A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds. · r1
contract/process-log/instance · Process log line · instance — A ViolaName or null. · r1
contract/process-log/level · Process log line · level — One of DEBUG, INFO, WARN, ERROR. · r1
contract/process-log/message · Process log line · message — A required field, as fields.message or the flattened message. · r1
contract/process-log/process · Process log line · process — The process role, one of run, hook, mcp, cli. · r1
contract/process-log/target · Process log line · target — A required field. · r1
contract/process-log/timestamp · Process log line · timestamp — RFC 3339 UTC with milliseconds and Z. · r1
contract/refusal-reason/unknown-fallback · Refusal reason · unknown fallback — An unrecognised RefusalReason falls back to unknown with exit 14. · r1
contract/replayed-snapshot/live-fields · A snapshot.json rebuilt by replay · endpoint, pid, started_at, statusline_command — A snapshot.json rebuilt from events.ndjson carries none of these, so the endpoint reads unreachable, hook connect fails open with channel-unreachable, and run's collision check reads the heartbeat alone. · r2
contract/run-exit-1-detail/already-live · run exit-1 detail codes · already-live — A live holder or a fresh replayed beat. · r1
contract/run-exit-1-detail/cwd-missing · run exit-1 detail codes · cwd-missing — On a start made by viola revive, a distinct detail code. · r1
contract/run-exit-1-detail/no-session · run exit-1 detail codes · no-session — On a start made by viola revive, a distinct detail code. · r1
contract/run-exit-1-detail/session-live · run exit-1 detail codes · session-live — On a start made by viola revive, a distinct detail code. · r1
contract/run-exit-1-detail/sha-256-mismatch · run exit-1 detail codes · sha-256 mismatch — A distinct detail code for a sha-256 mismatch. · r1
contract/run-exit-1-detail/squatted-name · run exit-1 detail codes · squatted-name — A distinct detail code for a squatted name. · r1
contract/run-exit-1-detail/stale-heartbeat · run exit-1 detail codes · stale-heartbeat — A stale holder. · r1
contract/run-log/cli-verified · run log line · cli_verified — A required field on run's log lines. · r1
contract/run-log/cli-version · run log line · cli_version — An optional field on run's claude-child process-start line; absent when the CLI version is unreadable. · r1
contract/run-log/exit-code · run log line · exit_code — A required field on run's log lines. · r1
contract/run-start-events/order · run start events in events.ndjson · order — A run start writes wheel{cause:"start"} before session-start; on a start whose folder-trust keys pass through the wrapper, wheel{human-input} also precedes session-start, and a later human key adds no wheel line, the wheel being already human. · r3
contract/send-log-line/corr · Send log line · corr — On the wrapper's send lines corr is the send cursor, which joins send-issued, send-confirmed and send-refused (a refusal before send-issued carries the log's end offset); the CLI's and MCP's client-side send-refused lines carry no corr; on channel lines corr is the JSON-RPC id. · r1
contract/send-log-line/detail · Send log line · detail — The send detail is one of input-not-ready, no-prompt-submitted, control-character, empty-text or turn-running. · r1
contract/send-log-line/from · Send log line · from — The from field is self-reported and logged as unauthenticated, never as an identity. · r1
contract/send-log-line/required-fields · Send log line · required fields — A send line carries event, process, instance, corr, from, confirmed, refusal, detail and duration_ms. · r1
contract/send-path-log-events/event-order · Send path log events · event order — A send logs channel-request, then send-issued, then hook-invoked (UserPromptSubmit, or SessionStart for a confirmed /clear or /compact; none awaited for an unconfirmable send or a local command confirmed by its transcript echo), then send-confirmed or send-refused, then channel-response. · r2 · contradict
contract/service-identity/service-name · Service identity · service.name — Hardcoded "viola", the binary name, never read from OTEL_SERVICE_NAME or SERVICE_NAME. · r1
contract/service-identity/service-version · Service identity · service.version — The compile-time CARGO_PKG_VERSION, the same value used as sender and writer. · r1
contract/unreachable-cause/endpoint-lost · exit 21 cause words · endpoint-lost — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body. · r1
contract/unreachable-cause/instance-dead · exit 21 cause words · instance-dead — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body. · r1
contract/unreachable-cause/server-verify-failed · exit 21 cause words · server-verify-failed — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body. · r1
contract/unreachable-cause/socket-dir-refused · exit 21 cause words · socket-dir-refused — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body. · r1
contract/unreachable-cause/strict-modes-failed · exit 21 cause words · strict-modes-failed — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body. · r1
contract/unreachable-cause/unwrapped-name · exit 21 cause words · unwrapped-name — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body. · r1
contract/viola-link-unlink/strict-modes-failed · viola link and unlink output · strict-modes-failed — A strict-mode fail of the driven instance's check writes no log line; it prints the fixed instance-state unable and hint: pair on stderr, or under --json {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, and exits 21. · r1
contract/viola-revive/strict-modes-failed · viola revive output · strict-modes-failed — A strict-mode fail of viola revive writes no log line; it prints the stderr pair, or under --json {"v":1,"error":"strict-modes-failed","detail":null}, and exits 1. · r1
contract/viola-verify/home-tree-refused · viola verify · refused home-tree check — A refused home-tree check exits 1 with the home pair, opens no log and writes no line; a refused update_stamps exits 1 with the home pair and, when an instance is named, ends the opened log with a process-exit line carrying detail strict-modes-failed. · r1
contract/wait-log-line/exit-code-21 · Wait log line · exit_code 21 — A wait exits 21 when the wrapper is killed mid-wait (detail endpoint-lost, during call), or when a CLI wait meets a refused socket directory before any frame (detail socket-dir-refused, during connect, no channel-request). · r1
contract/wait-log-line/outcome · Wait log line · outcome — The wait outcome is the woken kind, timed-out or instance-unreachable. · r1
contract/wait-log-line/required-fields · Wait log line · required fields — A wait line carries event, process, instance, corr (the JSON-RPC id), after (a byte offset), timeout_ms, outcome, exit_code and duration_ms. · r1
contract/wait-path-log-events/channel-pair · Wait path log events · channel pair — A wait logs channel-request and channel-response, the response carrying the outcome. · r1
contract/wheel-log-line/human-typing · Wheel log line · human-typing — A send refused while a human holds the wheel logs refusal human-typing, detail null or manual-pause, the wheel holder and exit_code 10. · r1
contract/wheel-path-log-events/pause-and-release · Wheel path log events · pause and release — A wheel take-over logs send-refused for the refused send and a channel-request and channel-response pair for pause and for release. · r1
absence/deployment-environments — Deployment environments or hosting · r1
absence/interactive-terminal-interface — An interactive terminal interface: prompt, TUI or pager · r1

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/allow-list-not-widenable · Allow-list and redaction not configurable — config.json, a VIOLA_* env var or a CLI flag never widens the field allow-list or disables redaction. · r1
decision/backtraces-in-instance-detail-only · Backtraces in the instance detail file only — Backtraces are written as one-line JSON arrays in the instance detail file only. · r1
decision/budgets-asserted-by-exit-code · Budgets asserted by exit code — Every budget has an exit-code-setting assertion (hyperfine max via jq). · r1
decision/claude-env-and-cookie-never-logged · CLAUDE* values never logged — CLAUDE* values are never logged, at any level, in any file, detail files included. · r3
decision/closed-event-and-corr-on-every-line · Closed event enum on every line, corr where correlated — Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text. · r1
decision/corr-set-explicitly-by-caller · corr set explicitly by the caller — The caller sets corr explicitly as a typed field on the events that carry one. · r1
decision/default-panic-hook-never-runs · Default panic hook never runs — The default panic hook is never called or allowed to run. · why: Its stderr output breaks hook and the run terminal. · r1
decision/derived-metrics-by-closed-enums · Derived metrics grouped by closed enums — Derived metrics are grouped by the closed enums only, never by unbounded keys such as path, corr, conn, pid or instance. · r1
decision/event-values-closed · event values closed — No event value is added without a Decisions Log entry. · r1
decision/explicit-corr-conn-instance · corr, conn and instance passed explicitly — corr, conn and instance are passed as explicit values across threads or tasks, never carried by span context. · why: Spans are not serialized. · r1
decision/explicit-subscriber-writer · Explicit tracing-subscriber writer — with_writer is always set explicitly on the tracing-subscriber. · why: The default tracing-subscriber writer is stdout. · r1
decision/failed-scan-uploads-hit-report · Failed scan uploads the hit report — On a failed secret scan the hit report is uploaded instead of the diagnostics. · r1
decision/failing-job-diagnostics-kept · Failing job's diagnostics kept — A failing job's diagnostics are never lost: G2, G4 and the secret scan run if: always(). · r1
decision/field-allow-list-default-deny · Field allow-list is default-deny — No field is logged unless named in the field catalog. · r1
decision/file-sink-only-error-sink · File sink is the only error sink — Local logs are never optional: the file sink is the only error sink and works with every other feature off. · r1
decision/hyperfine-max-is-enforced · Hyperfine max is the enforced number — The max from hyperfine is the enforced latency number. · r1
decision/latency-with-gate-statistic · Latency reported with the gate statistic — Latency is never reported without the gate statistic; p50 and p95 are report-only. · r1
decision/log-field-names-frozen · Log field names frozen — Log fields such as corr and event are never renamed without a Decisions Log entry. · r1
decision/log-internal-errors-off · log_internal_errors turned off — log_internal_errors is never left at its default true. · why: Writer failures would eprintln to stderr and break the Hook contract and the run terminal. · r1
decision/low-cardinality-span-names · Low-cardinality span names — Span names never contain instance names, dialog ids or paths. · r1
decision/machine-asserted-telemetry-verdicts · Machine-asserted telemetry verdicts — Log analysis is never human-review-gated; every CI telemetry verdict is a jq or hyperfine assertion that sets an exit code. · r1
decision/must-trace-paths-always-traced · Must-trace paths always traced — Spans and events are never skipped on the 6 must-trace paths. · why: The agent loses its ability to debug without them. · r1
decision/no-bare-instrument · No bare #[instrument] — Instrumentation always uses #[instrument(skip_all, fields(...))], never a bare #[instrument]. · why: A bare #[instrument] records every argument with Debug and leaks send text, hook.dialog payloads and tool input. · r1
decision/no-ci-env-metadata-in-logs · No CI env metadata in product logs — CI env metadata is never injected into product log lines. · why: The product never reads env as configuration. · r1
decision/no-dashboards-only-obs · No dashboards-only observability — Every signal is consumed through agent-run logs / status and jq, never through dashboards as the sole surface. · r1
decision/no-env-log-level · Log level not read from the environment — The level is never read from RUST_LOG, the env-filter feature is never enabled and EnvFilter::from_default_env is never called. · why: Env vars are not a configuration channel. · r1
decision/no-error-reporter-in-v1 · No error reporter in v1 — sentry or any other reporter is never added in v1. · why: Egress is banned, and sentry's default native-tls / tokio transport breaks cargo-deny. · r1
decision/no-hot-path-instrumentation · No hot-path instrumentation — The PTY pump read loop and the vt100 feed get no per-byte or per-chunk span or event; only gate outcomes are logged. · r1
decision/no-in-process-metrics · No in-process metrics instruments — No in-process metrics instruments (opentelemetry meters, metrics crate) are created. · why: There is no reader, exporter or listener to flush them. · r1
decision/no-info-in-hot-loops · No info logging in hot loops — Nothing is logged at info inside hot loops such as the PTY pump; counts go on the boundary event instead. · r1
decision/no-instrument-err-ret-on-sensitive · No instrument err or ret on sensitive results — #[instrument(err)] and #[instrument(ret)] are never used where the error or return can carry a serde or serde_path_to_error source or user content. · r1
decision/no-log-bridge · No log bridge — tracing_log::LogTracer or any log bridge is never installed by hand, and tracing-log stays off. · why: Bridged records lack event, corr, process and instance. · r1
decision/no-non-blocking-appender · No non_blocking appender — tracing-appender non_blocking is never used. · why: It drops lines when full, needs a WorkerGuard flush, and breaks the hook's exit-0 / max < 1.0 s gate and the zero-unlogged-panics invariant. · r1
decision/no-otel-stdout-exporter · OTel stdout exporter inadmissible — The OTel stdout exporter is inadmissible on every role. · why: stdout is reserved. · r1
decision/no-otel-without-decisions-log · No OTel without a Decisions Log entry — No OTel SDK, OTLP exporter or opentelemetry-stdout is added without a Decisions Log entry. · why: opentelemetry-otlp's default reqwest-blocking-client spins up a Tokio runtime. · r1
decision/no-paths-or-pids-in-detail-codes · No paths or pids in detail codes — detail codes never contain paths or pids. · r1
decision/no-per-heartbeat-logging · No log line per heartbeat tick — Heartbeats log transitions and lifecycle only, never a line per tick. · why: The log files are never rotated. · r1
decision/no-pii-in-span-fields · No PII or content in span fields — Span fields never include PII or content. · why: Scrubbing at an exporter stage is too late, and v1 has no exporter stage at all. · r1
decision/no-print-macros · No print macros in product code paths — print!, println!, eprint!, eprintln! and dbg! are never used in the hook, run or mcp code paths of the product crates, and every product member inherits the workspace lints. · why: They bypass the file sink and break the Hook contract, the run terminal rule and MCP framing. · guard: clippy print_stdout, print_stderr and dbg_macro at deny · except: viola-harness, the fake agent and the output modules that own --json, human CLI stderr and the hook decision body. · r1
decision/no-proprietary-apm-only · No proprietary APM as the only exporter — Proprietary APM (Datadog APM SDK, NewRelic agent, AppDynamics) is never the only exporter. · why: Vendor schemas break paste-to-AI workflows, and viola has no egress at all. · r1
decision/no-retry-once · No retry-once policies — No retry-once policy is added for panics or flaky telemetry assertions. · r1
decision/no-serde-source-in-error-output · No serde source in printed errors — An anyhow chain that still contains a serde_json or serde_path_to_error source is never printed to stderr, --json, MCP isError or channel error.data. · r1
decision/no-stringified-corr · corr never recorded with ? or % — corr is never recorded with ?corr or %corr. · why: Those stringify numbers. · r1
decision/no-telemetry-endpoint · No telemetry endpoint without a security decision — No /metrics or other telemetry endpoint is added without a Security Decisions Log entry. · why: A telemetry endpoint is a new listener. · r1
decision/no-telemetry-from-screen-content · No telemetry from screen content — Telemetry is never derived from screen content; vt100 is used for readiness and modal gate outcomes only. · r1
decision/no-telemetry-on-stdout · No telemetry on stdout — Telemetry is never written to stdout on any role. · why: stdout carries each role's own output and run's terminal carries the child's screen. · r1
decision/obs-event-macro-only · obs_event! is the only event macro — tracing's event and level macros are never called directly outside viola_core::obs; events go through obs_event!. · why: Only obs_event! guarantees event, process and instance. · guard: clippy disallowed-macros; the raw-event! grep in lint-probes · r1
decision/one-line-panics · Panics are one line — Multi-line stack traces are never written; panics are logged as one line. · r1
decision/paths-only-in-instance-detail-files · Paths only in instance detail files — Paths appear only in instance detail files. · r1
decision/pinned-actions · Actions pinned by commit SHA — Actions are never used unpinned; actions/upload-artifact is pinned by commit SHA. · guard: zizmor · r1
decision/product-is-not-a-telemetry-collector · Product is not a telemetry collector — viola never self-instruments as if the product were telemetry: events.ndjson is not turned into a collector and no OTLP receiver is added. · why: viola is Normal mode (Founder Direction 3) · r1
decision/reporter-never-log-floor · Future reporter applies the NEVER-log floor — Any future reporter applies the NEVER-log floor in before_send. · r1
decision/reporter-opt-in-only · Any future reporter is opt-in — Any future reporter is opt-in, never enabled by env var or by default. · r1
decision/reporter-ureq-rustls · Future reporter transport is ureq and rustls — Any future reporter uses ureq with rustls. · r1
decision/rerun-no-substitute · Re-run is no substitute for diagnostics — A re-run is not a substitute for the failing job's diagnostics. · why: CI runs under retries = 0. · r1
decision/scrub-before-bytes-leave-process · Scrubbing before any byte leaves the process — Every byte is scrubbed of secrets before it leaves the process; in v1 the only destination is local files. · guard: the secret-scan test · r1
decision/sensitive-content-detail-file-only · Sensitive content only in the instance detail file — Send text, tool input and last_assistant_message never reach a home-level file; they may reach only the instance-scoped detail-<process>.ndjson, inside a chain or drift report. · r2
decision/service-identity-from-constants · Service identity from compiled constants — Service name and version come from viola-core constants, never from env vars such as OTEL_SERVICE_NAME or OTEL_RESOURCE_ATTRIBUTES; any future OTel Resource starts from an empty builder. · r1
decision/transcripts-not-a-contract · Claude Code transcripts are not a contract — Claude Code transcripts are never read as a contract. · except: A local command's stdout tag in the session transcript, counted past the end recorded before the paste, is read as that send's echo confirmation; no transcript byte is kept. · r2 · stale
decision/upload-only-after-secret-scan · Diagnostics uploaded only after the secret scan — diagnostics artifacts are never uploaded before the secret-scan step passes; uploads run if: always() / failure() combined with the secret-scan step's success, never bare. · r1
decision/veil-toggle-banned · veil toggle feature banned — veil's toggle feature (VEIL_DISABLE_REDACTION / veil::disable()) is never enabled. · guard: cargo-deny · r1
decision/verbosity-changes-volume-only · Verbosity changes volume only — diagnostics_level changes the volume of logging only. · r1
contract/stdout-by-role/cli · stdout by role · cli — stdout carries the --json results. · r1
contract/stdout-by-role/hook · stdout by role · hook — stdout carries the decision body. · r2
contract/stdout-by-role/mcp · stdout by role · mcp — stdout carries MCP frames. · r1
contract/stdout-by-role/run · stdout by role · run — The terminal of run carries the child's screen. · r1
absence/network-egress — Network egress of any kind · r1
absence/telemetry-exporter-stage — A telemetry exporter stage · r1

## obs-anti-patterns · ## 11. Obs Anti-Patterns (NEVER do these) · twin 11-obs-anti-patterns-never-do-these
