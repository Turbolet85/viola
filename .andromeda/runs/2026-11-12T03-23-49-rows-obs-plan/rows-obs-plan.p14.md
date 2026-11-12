@row_add n=1 row=contract/diagnostics-line/format area=logging-stack rec=1
boundary: the diagnostics line
item: format
promise: Each diagnostics line is one structured JSON object on one line, written whole by one write of the complete line on an append-mode handle.
@row_add n=2 row=contract/diagnostics-line/role-file-line-size area=logging-stack rec=1
boundary: the diagnostics line
item: role-file line size
promise: A line in a role file (run, hook, mcp, cli) is codes-only and stays well under 4 KiB.
@row_add n=3 row=contract/diagnostics-line/detail-line-size area=logging-stack rec=1
boundary: the diagnostics line
item: detail-file line size
promise: A line in an instance detail file (chains, drift reports, one-line backtraces) is not codes-only and may exceed 4 KiB.
@row_add n=4 row=decision/shared-log-files-concurrent-append area=logging-stack rec=1
name: Concurrent appends to shared diagnostics files
rule: Every diagnostics file may have concurrent writers in separate processes; each line is one whole write on an append-mode handle, readers tolerate a torn line, and a concurrent-append check, including a detail line over 4 KiB, asserts every line parses.
why: Non-interleaving at this size is an assumption, not a POSIX guarantee, because the 4 KiB PIPE_BUF rule applies to pipes.
guard: concurrent-append check; hook_panics_append_whole_lines_over_4_kib_side_by_side
@row_add n=5 row=contract/agent-run-logs/torn-line area=logging-stack rec=1,16
boundary: agent-run logs output
item: torn line
promise: A torn line appears as {"torn":true,"offset":n}, and readers skip it rather than fail.
@row_add n=6 row=decision/no-internal-error-fallback area=logging-stack rec=2
name: No stderr fallback for subscriber internal errors
rule: The diagnostics subscriber never reports its own internal errors by printing to stderr.
why: The default falls back to eprintln, which would break the Hook contract and the run terminal rule.
@row_add n=7 row=decision/no-span-list-in-lines area=logging-stack rec=2
name: No span list in diagnostics lines
rule: Diagnostics lines never carry the list of entered spans.
why: Closes the spans leak path.
@row_add n=8 row=decision/explicit-log-writer area=logging-stack rec=2
name: Explicit diagnostics writer
rule: The diagnostics subscriber always has its writer set explicitly.
why: The default writer is stdout.
@row_add n=9 row=decision/no-ansi-in-logs area=logging-stack rec=2
name: No ANSI codes in diagnostics lines
rule: Diagnostics lines carry no ANSI colour codes.
@row_add n=10 row=decision/no-source-paths-in-logs area=logging-stack rec=2
name: No source paths in diagnostics lines
rule: Diagnostics lines carry no source file names or line numbers.
@row_add n=11 row=decision/file-only-diagnostics-sinks area=logging-stack rec=3
name: File-only diagnostics, one file per role
rule: Every role logs only to its own file under the home's diagnostics directory; no role has a stdout sink, and there is no pretty or dev sink.
@row_add n=12 row=contract/diagnostics-files/run area=logging-stack rec=3
boundary: the diagnostics files
item: run
promise: Role run writes <home>/diagnostics/run-<name>.ndjson.
@row_add n=13 row=contract/diagnostics-files/hook area=logging-stack rec=3
boundary: the diagnostics files
item: hook
promise: Role hook writes <home>/diagnostics/hook-<name>.ndjson, shared by concurrent hook processes; a write failure is swallowed.
@row_add n=14 row=contract/diagnostics-files/mcp area=logging-stack rec=3
boundary: the diagnostics files
item: mcp
promise: Role mcp writes <home>/diagnostics/mcp.ndjson, with stderr JSON only when the file cannot be opened.
@row_add n=15 row=contract/diagnostics-files/cli area=logging-stack rec=3
boundary: the diagnostics files
item: cli
promise: Role cli writes <home>/diagnostics/cli-<name>.ndjson, only when an instance resolves.
@row_add n=16 row=contract/diagnostics-files/detail area=logging-stack rec=3
boundary: the diagnostics files
item: detail
promise: Content-bearing records (full error chains, drift reports, panic payload and backtrace) go only to <home>/instances/<name>/diagnostics/detail-<process>.ndjson, file 0600 in a 0700 directory, and only when an instance resolves.
@row_add n=17 row=decision/content-records-bypass-subscriber area=logging-stack rec=3
name: Content-bearing records bypass the subscriber
rule: Content-bearing records never pass through the tracing subscriber, whose one writer holds only the home-level role file; the detail writer formats each detail line itself and writes it with one whole write.
@row_add n=18 row=contract/detail-line/fields area=logging-stack rec=3
boundary: the detail line
item: fields
promise: Each detail line carries timestamp (millisecond UTC format), level, target, message, event, process, instance, and the corr / conn keys.
@row_add n=19 row=decision/no-content-in-home-level-lines area=logging-stack rec=3
name: No content fields in home-level lines
rule: No home-level diagnostics line carries chain, drift_report, panic_payload or backtrace.
guard: G4 diag-line.v1.json schema
@row_add n=20 row=decision/no-agent-mode-flag area=logging-stack rec=4
name: No agent-mode flag
rule: There is no agent-mode flag; JSON to file is the only logging mode.
@row_add n=21 row=contract/config-json/diagnostics-level area=logging-stack rec=4,6
boundary: config.json
item: diagnostics_level
promise: Key diagnostics_level takes "info" (the default) or "debug"; a missing or unreadable file or an unknown value falls back to "info".
@row_add n=22 row=decision/log-level-never-from-env area=logging-stack rec=4
name: Log level never from the environment
rule: The diagnostics level is never read from RUST_LOG and the env-filter feature is never enabled; the level comes only from config.json.
@row_add n=23 row=decision/level-changes-volume-only area=logging-stack rec=4
name: Level changes volume only
rule: Every diagnostics level uses the same field allow-list, so the level changes only how many lines are written.
why: No setting can disable redaction.
@row_add n=24 row=decision/third-party-targets-off area=logging-stack rec=4
name: Third-party log targets off
rule: Records of third-party targets (rmcp, portable_pty, sysinfo, interprocess) are off in every sink; their failures are captured at viola's seam from the returned results.
why: Their records lack the required fields.
@row_add n=25 row=decision/no-log-crate-bridge area=logging-stack rec=4
name: No log-crate bridge
rule: The log-crate bridge (tracing-log, LogTracer) stays off; failures are captured from returned values or handles, and a gap is fixed at the seam or by the researched json-subscriber fallback, never by enabling the bridge.
why: The accepted loss is only third-party log context emitted without a returned error.
guard: chaos case "child exit without EOF"
@row_add n=26 row=decision/no-otel-sdk-v1 area=otel-sdk-init rec=5,14
name: No OpenTelemetry SDK in v1
rule: No OpenTelemetry SDK, bridge or Resource is used in v1.
why: With no exporter they would add only dependency and cargo-mutants surface, and bridged span IDs would have no consumer.
@row_add n=27 row=decision/tracing-is-the-instrumentation-facade area=otel-sdk-init rec=5
name: tracing as the instrumentation facade
rule: tracing is the instrumentation facade in every instrumentable crate, and tracing-subscriber is the formatter and sink, a product dependency of the root bin only.
@row_add n=28 row=decision/otel-resource-built-empty area=otel-sdk-init rec=5,14
name: OTel Resource built empty if ever adopted
rule: If an OTel SDK is ever adopted, its Resource is built empty with viola's own identity values, never with the default builder.
why: The default builder's environment detector reads OTEL_* env vars.
@row_add n=29 row=decision/tracing-subscriber-minimal-features area=otel-sdk-init rec=5
name: Minimal tracing-subscriber features
rule: tracing-subscriber is taken with default features off and only fmt, json, registry and std; ansi, tracing-log and chrono stay off.
why: The chrono feature only enables ChronoUtc / ChronoLocal, and the millisecond timer uses the root bin's direct chrono dependency.
@row_add n=30 row=decision/panic-hook-first area=otel-sdk-init rec=6
name: Panic hook installed first
rule: The panic hook is installed as the first statement of main and writes only to the diagnostics file once that is open, never falling back to stderr.
@row_add n=31 row=decision/home-resolution-order area=otel-sdk-init rec=6
name: Home resolution order
rule: The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
@row_add n=32 row=decision/instance-source-per-role area=otel-sdk-init rec=6,21
name: Instance source fixed per role
rule: run takes its own name argument, never an inherited VIOLA_NAME; hook and mcp take VIOLA_NAME only; cli takes the caller's VIOLA_NAME, and the target argument only when VIOLA_NAME is unset.
why: A wrapper never writes into another instance's run log.
@row_add n=33 row=decision/strict-modes-before-diagnostics area=otel-sdk-init rec=6
name: Home strict-modes check before diagnostics
rule: Nothing under the home, the diagnostics directory included, is created or opened before the role's strict-modes check on the home passes; a refused home gets no diagnostics line.
@row_add n=34 row=decision/private-diagnostics-files area=otel-sdk-init rec=6
name: Owner-only diagnostics files
rule: The diagnostics directory is 0700 and each diagnostics file 0600, opened without following symlinks; before the first write each must be owned by the effective user, have no group or other bits and not be a symlink, a failed check counting as a failed open.
why: A pre-existing foreign, permissive or symlinked diagnostics file is never written.
@row_add n=35 row=decision/failed-log-open-discards area=otel-sdk-init rec=6
name: Failed log open discards lines
rule: When the role file cannot be opened, the diagnostics writer discards lines; only mcp writes to stderr instead.
@row_add n=36 row=decision/lazy-detail-file area=otel-sdk-init rec=6
name: Detail file opened lazily
rule: The instance detail file is opened only by the first content-bearing record or by the panic hook, and a failed detail open drops only the detail line, writing nothing to stderr.
why: The error-free hook path keeps exactly one log open.
@row_add n=37 row=contract/config-json/five-hour area=otel-sdk-init rec=6
boundary: config.json
item: five_hour
promise: Key five_hour holds a whole percentage from 0 to 100; any other value is logged as one parse-rejected{parser:"config-json", detail:"malformed"} WARN line.
@row_add n=38 row=contract/config-json/seven-day area=otel-sdk-init rec=6
boundary: config.json
item: seven_day
promise: Key seven_day holds a whole percentage from 0 to 100; any other value is logged as one parse-rejected{parser:"config-json", detail:"malformed"} WARN line.
@row_add n=39 row=contract/config-json/dialog-hold-s area=otel-sdk-init rec=6
boundary: config.json
item: dialog_hold_s
promise: Key dialog_hold_s holds a whole number of seconds from 60 to 3600; any other value gives one parse-rejected malformed WARN line and the hold takes 60.
@row_add n=40 row=contract/config-json/v area=otel-sdk-init rec=6
boundary: config.json
item: v
promise: A document that is no object, or whose v is not 1, gives the threshold and hold defaults and no further line.
@row_add n=41 row=contract/config-json/known-keys area=otel-sdk-init rec=6
boundary: config.json
item: known keys
promise: config.json has six known keys: v, diagnostics_level, claude_env_keep, five_hour, seven_day, dialog_hold_s; any other key is counted as skipped.
@row_add n=42 row=contract/diagnostics-events/config-parse-rejected area=otel-sdk-init rec=6
boundary: diagnostics events
item: parse-rejected config-json
promise: Right after process-start, a config.json parse that failed or skipped keys logs parse-rejected{parser:"config-json", detail, count}.
@row_add n=43 row=decision/diagnostics-init-before-role-work area=otel-sdk-init rec=6
name: Diagnostics init before the role's work
rule: Diagnostics init and the process-start line come before the role's work: in run before its collision check and the child spawn, in mcp before its stdio transport starts.
why: Exit-1 causes are logged.
@row_add n=44 row=decision/instance-check-before-log-init area=otel-sdk-init rec=6
name: Instance check before log init
rule: Commands acting on an instance (revive's start arm, the hook arms, link, unlink, send, wait, last, answer, pause, release) run the instance strict-modes check before opening any log; a refusal opens and writes nothing and leaves modes as found.
why: Opening the role file narrows an existing home to 0700, so a check made after it passes a home at 0770.
except: The two hook --capture arms run ahead of the check.
@row_add n=45 row=decision/home-tree-check-before-log-init area=otel-sdk-init rec=6
name: Home tree check before log init
rule: run, revive's start arm and viola verify check the home tree's modes before their log opens; a refusal writes no line, starts no child, leaves modes as found and exits 1.
@row_add n=46 row=decision/hook-single-log-open area=otel-sdk-init rec=6
name: One config read and one log open per hook
rule: A hook invocation makes one config.json read and exactly one log open, then only appends.
guard: hook max < 1.0 s gate
@row_add n=47 row=decision/hook-refusal-exits-zero area=otel-sdk-init rec=6
name: Refused hook fails open
rule: A hook whose instance is refused or absent exits 0 at once, reading nothing under the home and no stdin, writing no frame and no log line.
@row_add n=48 row=contract/cli-exit-codes/21 area=otel-sdk-init rec=6
boundary: CLI exit codes
item: 21
promise: An instance strict-modes refusal exits 21 with the fixed instance-state pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
@row_none n=1 rec=7 type=S
@row_none n=2 rec=8 type=4
relied: The subscriber flattens event fields to the top level of each line, and the Targets filter is the only configurable level gate.
@row_add n=49 row=decision/millisecond-utc-timestamps area=otel-sdk-init rec=9
name: Millisecond UTC timestamps
rule: Every diagnostics timestamp is RFC 3339 in UTC with millisecond precision and a Z suffix.
why: The default timer writes microseconds, and the chrono RFC 3339 timer writes +00:00 with automatic precision.
@row_add n=50 row=decision/normal-observability-mode area=product-mode-normal-vs-inverted-self-observing rec=10
name: Normal observability mode
rule: viola runs in Normal observability mode: events.ndjson is a product audit log, not a telemetry collector.
why: viola is not a telemetry product; the self-observing-inversion trigger did not fire.
@row_add n=51 row=contract/service-identity/service-name area=service-identity rec=11
boundary: service identity
item: service.name
promise: service.name is the hardcoded binary name "viola", never read from OTEL_SERVICE_NAME or SERVICE_NAME.
@row_add n=52 row=decision/env-vars-not-configuration area=service-identity rec=11,13
name: Environment variables are not configuration
rule: Environment variables are not a configuration channel: service identity has no env-var source or fallback.
@row_add n=53 row=decision/single-version-constant area=service-identity rec=12
name: One version constant for every crate
rule: One compile-time version constant taken from the package version serves every crate, and every crate takes the workspace version.
why: The log version, channel sender and snapshot writer cannot drift apart.
@row_add n=54 row=absence/hosted-deployment area=service-identity rec=13
capability: A hosted deployment or deployment environment; viola is local-only
@row_add n=55 row=contract/process-start/service-name area=service-identity rec=14
boundary: the process-start line
item: service_name
promise: process-start carries the service name as field service_name.
@row_add n=56 row=contract/process-start/version area=service-identity rec=14
boundary: the process-start line
item: version
promise: process-start carries the product version as field version.
@row_add n=57 row=contract/process-start/os area=service-identity rec=14
boundary: the process-start line
item: os
promise: process-start carries field os, the one word linux.
@row_add n=58 row=contract/process-start/pid area=service-identity rec=14
boundary: the process-start line
item: pid
promise: process-start carries the process id as field pid.
@row_add n=59 row=contract/diagnostics-line/process area=service-identity rec=14
boundary: the diagnostics line
item: process
promise: Every diagnostics line carries the role as field process.
@row_add n=60 row=contract/diagnostics-line/instance area=service-identity rec=14
boundary: the diagnostics line
item: instance
promise: Every diagnostics line carries the instance as field instance.
@row_add n=61 row=absence/pulse-class-receiver area=snapshot-paste-to-ai-integration rec=15
capability: A pulse-class log receiver
@row_add n=62 row=contract/agent-run-logs/events-record area=snapshot-paste-to-ai-integration rec=16
boundary: agent-run logs output
item: events record
promise: Each events.ndjson line is merged as {"src":"events","instance","offset","record"}.
@row_add n=63 row=contract/agent-run-logs/diag-record area=snapshot-paste-to-ai-integration rec=16
boundary: agent-run logs output
item: diag record
promise: Each line of every diagnostics/*.ndjson is merged as {"src":"diag","file","record"}.
@row_add n=64 row=contract/agent-run-logs/detail-record area=snapshot-paste-to-ai-integration rec=16,19
boundary: agent-run logs output
item: detail record
promise: Every instance detail line from instances/*/diagnostics/detail-*.ndjson is merged as {"src":"diag","file","instance","record"}, the instance telling apart the same detail file name across instances.
@row_add n=65 row=contract/agent-run-logs/filters area=snapshot-paste-to-ai-integration rec=16
boundary: agent-run logs output
item: filters
promise: The merged output is filterable by --instance, --kind, --process and --after.
@row_add n=66 row=contract/agent-run-status/state area=snapshot-paste-to-ai-integration rec=17
boundary: agent-run status
item: state
promise: agent-run status, the aggregate health snapshot, reports a state.
@row_add n=67 row=contract/agent-run-status/last-error area=snapshot-paste-to-ai-integration rec=17
boundary: agent-run status
item: last_error
promise: agent-run status reports last_error from a closed enum.
@row_add n=68 row=contract/send-line/wrapper-side area=snapshot-paste-to-ai-integration rec=18
boundary: the send line
item: wrapper side
promise: send-* lines whose corr is a send cursor are emitted only by the target wrapper, with side:"wrapper".
@row_add n=69 row=contract/send-line/client-side-refusal area=snapshot-paste-to-ai-integration rec=18
boundary: the send line
item: client-side send-refused
promise: The caller-side send-refused{side:"client"} has corr null.
@row_add n=70 row=contract/send-line/cursor-uniqueness area=snapshot-paste-to-ai-integration rec=18,21
boundary: the send line
item: cursor corr uniqueness
promise: A send cursor is a byte offset into the target's own events.ndjson, unique per target only; refusals and later sends can share it, so one send is identified by target instance, corr, conn and rpc_id.
@row_add n=71 row=contract/send-line/unconfirmable area=snapshot-paste-to-ai-integration rec=18
boundary: the send line
item: unconfirmable send
promise: An unconfirmable send logs one send-issued and one send-confirmed{confirmed:false} under the same corr, with no send-refused and no hook side.
@row_add n=72 row=absence/grpc area=trace-context-propagation rec=20
capability: gRPC
@row_add n=73 row=contract/diagnostics-line/corr-on-channel-lines area=trace-context-propagation rec=21
boundary: the diagnostics line
item: corr on channel lines
promise: On channel lines, corr carries the JSON-RPC id; a notification such as hook.event has no id, so its channel-request has corr null.
@row_add n=74 row=contract/diagnostics-line/corr-on-dialog-lines area=trace-context-propagation rec=21
boundary: the diagnostics line
item: corr on dialog and hook lines
promise: On dialog and hook lines, corr carries the dialog_id.
@row_add n=75 row=contract/diagnostics-line/corr-on-send-lines area=trace-context-propagation rec=21
boundary: the diagnostics line
item: corr on send lines
promise: On send lines, corr carries the send cursor; a send refused before send-issued carries the target's events.ndjson end offset at refusal, a log join key only, never returned to the caller.
@row_add n=76 row=contract/channel-params/conn area=trace-context-propagation rec=21
boundary: viola-channel request params
item: conn
promise: The client sends conn = "<process>-<pid>-<t0>-<n>" in params (t0 the process start in epoch milliseconds, n a per-process connection counter); both sides log it, and the server strips it before dispatch, never an input to a method.
@row_add n=77 row=contract/channel-line/srv-conn area=trace-context-propagation rec=21
boundary: the channel line
item: srv_conn
promise: A server receiving no conn, or one outside ^[a-z]+-\d+-\d+-\d+$ or over 64 bytes, logs srv_conn = "srv-<accept counter>" instead.
@row_add n=78 row=decision/channel-join-key-conn-corr area=trace-context-propagation rec=21
name: Channel join key is conn and corr
rule: The merged-log join key for channel lines is (conn, corr); instance is deliberately not part of it.
why: Each side logs its own instance, so the two sides of one call carry different instance values.
@row_add n=79 row=decision/conn-includes-start-instant area=trace-context-propagation rec=21
name: Connection id includes the process start instant
rule: conn includes the process's start instant besides the pid and a counter, so it is unique across the session home.
why: PIDs are recycled, and every hook opens one connection with the same first id, so two hooks from a reused PID would share (conn, corr).
@row_add n=80 row=contract/send-line/conn-rpc-id area=trace-context-propagation rec=21
boundary: the send line
item: conn and rpc_id
promise: Every wrapper send-* line carries conn (or srv_conn) and rpc_id, and (conn, rpc_id) equals (conn, corr) on that call's channel-request and channel-response on both sides.
@row_add n=81 row=contract/channel-line/from-trust area=trace-context-propagation rec=21
boundary: the channel line
item: from_trust
promise: from is logged with the constant from_trust:"self-reported" and is never treated as identity.
@row_add n=82 row=decision/hook-correlation-by-fields area=trace-context-propagation rec=22
name: Hook lines correlated by fields
rule: Hooks receive no inbound trace context from Claude Code; hook lines are correlated by instance plus dialog_id for dialog hooks, or by instance plus hook_event plus timestamp.
@row_add n=83 row=decision/span-carried-into-tasks-and-threads area=trace-context-propagation rec=23
name: Span carried into tasks and threads
rule: Spawned async tasks are instrumented with their span, and std thread closures take a cloned span and run inside it.
@row_add n=84 row=decision/explicit-correlation-values area=trace-context-propagation rec=23
name: Correlation values passed explicitly
rule: corr, conn and instance are passed as explicit values into each thread or task and set on each event, never recovered from span context.
why: Spans are not serialized.
