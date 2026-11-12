@row_add n=1 row=contract/run-log-parse-rejected/ledger-stamps area=span-trace-coverage rec=1
boundary: the run process log parse-rejected line
item: parser ledger-stamps
promise: An unreadable, malformed or strict-modes-refused capability ledger gives one parse-rejected line with parser "ledger-stamps", detail "unreadable", "malformed" or "strict-modes-failed", and a count, in the run log.
@row_add n=2 row=decision/unreadable-ledger-means-unverified area=span-trace-coverage rec=1
name: Unreadable capability ledger means an unverified CLI
rule: A capability ledger that is missing a stamp, unreadable, malformed or refused by the strict-modes check is treated as an unverified CLI (cli_verified false).
@row_add n=3 row=decision/ledger-contents-never-logged area=span-trace-coverage rec=1
name: Capability ledger path and stamps never logged
rule: The capability ledger's path and its stamp contents never appear in a log line.
@row_add n=4 row=contract/verify-spawn-subject/version-probe area=span-trace-coverage rec=2
boundary: verify child spawn subjects
item: version-probe
promise: The --version read of the agent CLI is logged as one process-start / process-exit pair with subject version-probe.
@row_add n=5 row=contract/verify-spawn-subject/verify-probe area=span-trace-coverage rec=2
boundary: verify child spawn subjects
item: verify-probe
promise: The print-mode probe is logged as one process-start / process-exit pair with subject verify-probe.
@row_add n=6 row=contract/verify-spawn-subject/agents-probe area=span-trace-coverage rec=2
boundary: verify child spawn subjects
item: agents-probe
promise: Run B's single agents --json read is logged as one process-start / process-exit pair with subject agents-probe.
@row_add n=7 row=contract/verify-spawn-subject/verify-pty-probe area=span-trace-coverage rec=2
boundary: verify child spawn subjects
item: verify-pty-probe
promise: Each of verify's five interactive PTY runs is logged as one process-start / process-exit pair with subject verify-pty-probe.
@row_add n=8 row=contract/verify-process-exit/interrupted area=span-trace-coverage rec=2
boundary: verify process-exit line
item: detail interrupted
promise: A verify stopped by SIGINT or SIGTERM starts no further spawn, closes every pair it opened and ends with process-exit subject self, exit_code 1, detail "interrupted".
@row_add n=9 row=contract/verify-exit-codes/0 area=span-trace-coverage rec=2
boundary: verify exit codes
item: 0
promise: Every relied ledger row passed.
@row_add n=10 row=contract/verify-exit-codes/1 area=span-trace-coverage rec=2
boundary: verify exit codes
item: 1
promise: A row failed, verify refused, or verify was stopped.
@row_add n=11 row=contract/verify-stop-stderr/unable-hint-pair area=span-trace-coverage rec=2
boundary: verify stderr on a stop
item: the stop pair
promise: A stopped verify prints exactly "unable: verify was stopped before it finished" and "hint: run viola verify again to stamp this CLI" on stderr, with empty stdout and no stamp written.
@row_add n=12 row=contract/verify-stdout/step-line area=span-trace-coverage rec=2
boundary: verify stdout
item: step line
promise: One "[NN/MM] <row id> <row words>  pass|fail" line is printed for each relied ledger row, MM being the relied row count.
@row_add n=13 row=contract/verify-stdout/summary-line area=span-trace-coverage rec=2
boundary: verify stdout
item: summary line
promise: The last stdout line is "stamped <version>  N pass  N fail", the two counts summing to MM.
@row_add n=14 row=contract/cli-stderr/refusal-pair area=span-trace-coverage rec=2
boundary: CLI stderr
item: unable and hint pair
promise: A refusal is printed to stderr as a fixed "unable:" line followed by a "hint:" line.
@row_add n=15 row=contract/cli-stderr/internal-error area=span-trace-coverage rec=2
boundary: CLI stderr
item: internal fault message
promise: An internal fault prints exactly "error: internal error".
@row_add n=16 row=contract/plugin-install-exit-codes/21 area=span-trace-coverage rec=2
boundary: plugin install exit codes
item: 21
promise: The strict-modes tree check refused; no log is opened and no line is written, with the stderr pair or, under --json, the instance-unreachable document.
@row_add n=17 row=contract/plugin-install-json/ok area=span-trace-coverage rec=2
boundary: plugin install --json document
item: ok
promise: Success prints {"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}, with no path in it.
@row_add n=18 row=contract/verify-process-exit/strict-modes-failed area=span-trace-coverage rec=2
boundary: verify process-exit line
item: detail strict-modes-failed
promise: A stamps store refused by its strict-modes check, under a parsed VIOLA_NAME, ends with process-exit carrying detail "strict-modes-failed" and the home pair on stderr.
@row_add n=19 row=decision/no-process-log-without-instance area=span-trace-coverage rec=2
name: No process log without an instance
rule: verify and plugin install run from a plain terminal with no instance write no process-log file; their outcome is the exit code and stdout.
@row_add n=20 row=decision/refused-home-tree-writes-no-line area=span-trace-coverage rec=2
name: Refused home tree writes no line
rule: verify and plugin install check the home tree before their log opens, and a refused tree opens no log and writes no line.
@row_add n=21 row=decision/verify-lines-carry-no-content area=span-trace-coverage rec=2
name: verify log lines carry no content
rule: No verify log line carries row text, a prompt, a session id, the probe command, a status line payload or a path.
@row_add n=22 row=decision/hidden-capture-arms-uninstrumented area=span-trace-coverage rec=2
name: Hidden hook capture arms uninstrumented
rule: The hidden hook capture arms used by verify's probes read no VIOLA_* variable, do no obs init, emit no hook lines, keep stderr empty and always exit 0.
why: uninstrumented by design, as ratified by the founder
@row_add n=23 row=decision/real-cli-verify-never-in-ci area=span-trace-coverage rec=2
name: Real-CLI verify never runs in CI
rule: In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
@row_add n=24 row=decision/verify-sole-stamps-writer area=span-trace-coverage rec=2
name: verify is the only stamps writer
rule: verify is the only writer of the capability ledger's stamps file.
@row_add n=25 row=decision/no-obs-gate-on-verify-lines area=span-trace-coverage rec=2
name: No obs gate depends on verify lines
rule: No observability gate depends on verify's own log lines; its home-level lines fall under the same panic, schema and secret-scan gates as any other.
@row_add n=26 row=decision/spans-not-serialized-in-v1 area=telemetry-strategy rec=3
name: Spans declared but not serialized in v1
rule: Spans are declared on the must-trace paths and every crate seam but are not written to output in v1.
why: they give the code its structure and are ready for the deferred OpenTelemetry bridge
@row_add n=27 row=decision/span-naming-skip-all area=telemetry-strategy rec=3
name: Span declaration convention
rule: Every span is declared with instrument, skip_all and a name of the form "<area>.<operation>".
@row_add n=28 row=decision/cross-process-join-corr-conn area=telemetry-strategy rec=3,32
name: Cross-process joins by corr and conn
rule: Lines across processes are joined by the corr and conn fields, not by W3C trace IDs.
@row_add n=29 row=decision/structured-logs-primary-signal area=telemetry-strategy rec=4
name: Structured logs are the primary signal
rule: tracing-subscriber JSON-per-line logs, one file per process role under the home's diagnostics directory, are the primary observability signal, merged with the event stream by agent-run logs.
@row_add n=30 row=decision/no-external-error-reporter area=telemetry-strategy rec=5
name: No external error reporter
rule: Errors and panics are reported to no external service; they stay in local log lines and exit codes.
why: security requires no outbound network calls
@row_add n=31 row=decision/panic-caught-at-every-role area=telemetry-strategy rec=5
name: Panics caught at every role's catch site
rule: A hand-written panic hook plus catch_unwind at every role's main-thread catch site, hook dispatch, the run PTY pump and handle-wait threads, and the vt100 feed turn a panic into one record.
@row_add n=32 row=decision/metrics-derived-from-ndjson area=telemetry-strategy rec=6
name: Metrics derived from logs, no instruments
rule: There are no in-process metric instruments and no exporter; metrics are derived by jq over the ndjson logs, and perf gates run on hyperfine JSON exports.
@row_add n=33 row=decision/events-stream-never-rotated area=telemetry-strategy rec=7,45
name: Event stream never rotated
rule: The instance's events.ndjson is the product audit stream and is never rotated by a generic library.
why: byte offsets in it are the wait-after and send cursors
@row_add n=34 row=decision/no-event-payloads-in-process-logs area=telemetry-strategy rec=7
name: No event payloads in process logs
rule: Observability never copies event payloads into process logs.
@row_add n=35 row=decision/panic-one-line-record area=telemetry-strategy rec=8
name: One single-line record a panic
rule: Every panic after obs init produces exactly one single-line structured record in its home-level role file, with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file.
except: the pre-init window and the paths that refuse or lack an instance or home tree before their role file opens
@row_none n=1 rec=9 type=S
@row_none n=2 rec=10 type=S
@row_add n=36 row=decision/hook-silent-without-instance area=telemetry-strategy rec=11
name: Hook silent without an instance
rule: A hook with no VIOLA_NAME, or with a refused or absent instance, exits 0 at once and writes no file, no line and no frame.
@row_add n=37 row=contract/hook-decision-detail/socket-dir-refused area=telemetry-strategy rec=11,14
boundary: hook-decision detail codes
item: socket-dir-refused
promise: A refused socket directory at the hook's connect writes one hook-decision line with detail "socket-dir-refused" at WARN, exit 0, empty streams and no frame.
@row_add n=38 row=contract/hook-decision-detail/server-verify-failed area=telemetry-strategy rec=11
boundary: hook-decision detail codes
item: server-verify-failed
promise: A server-verify mismatch writes one hook-decision line with detail "server-verify-failed" at WARN.
@row_none n=3 rec=12 type=S
@row_none n=4 rec=13 type=S
@row_add n=39 row=decision/pre-open-strict-modes-refusal-silent area=telemetry-strategy rec=14,24
name: Strict-modes refusal before a log opens writes no line
rule: A strict-modes refusal made before any process log opens writes no line and carries no logged detail code; its machine-readable form is the exit code and, under --json, its closed document.
except: a refused socket directory always logs detail socket-dir-refused, except at MCP's frame tool, which writes no line
@row_add n=40 row=contract/strict-modes-refusal-exit/21 area=telemetry-strategy rec=24
boundary: strict-modes refusal exit codes
item: 21
promise: The six frame verbs and link / unlink exit 21 on a strict-modes refusal, under --json with the instance-unreachable document carrying detail strict-modes-failed.
@row_add n=41 row=contract/strict-modes-refusal-exit/1 area=telemetry-strategy rec=24
boundary: strict-modes refusal exit codes
item: 1
promise: run, revive's start and verify exit 1 on a strict-modes refusal; revive under --json prints the strict-modes-failed document.
@row_add n=42 row=contract/socket-dir-refused-exit/codes area=telemetry-strategy rec=24,14
boundary: socket-dir-refused exit codes
item: socket-dir-refused
promise: A refused socket directory logs detail socket-dir-refused with no path, pid or uid, and exits 21 at the eight CLI frame verbs and 1 at run and revive's start.
@row_none n=5 rec=15 type=S
@row_none n=6 rec=16 type=S
@row_none n=7 rec=17 type=S
@row_none n=8 rec=18 type=S
@row_none n=9 rec=19 type=S
@row_none n=10 rec=20 type=S
@row_none n=11 rec=21 type=S
@row_none n=12 rec=22 type=S
@row_none n=13 rec=23 type=S
@row_none n=14 rec=25 type=S
@row_none n=15 rec=26 type=S
@row_none n=16 rec=27 type=S
@row_add n=43 row=decision/no-opentelemetry-in-v1 area=bootstrap-phases-derive-for-route-setup-project rec=28
name: No OpenTelemetry in v1
rule: No opentelemetry crate is added in v1, and cargo deny bans opentelemetry-otlp, opentelemetry-stdout, sentry and tracing-appender on the workspace.
why: so they cannot arrive transitively without a Decisions Log entry
guard: cargo deny bans
@row_add n=44 row=decision/subscriber-only-in-root-bin area=bootstrap-phases-derive-for-route-setup-project rec=29
name: tracing-subscriber only in the root binary
rule: tracing-subscriber is a product dependency of the root binary only, without its chrono feature; the product crates depend on tracing alone.
except: a library crate may take it as a dev-dependency for in-memory line capture in its own unit tests
@row_add n=45 row=decision/obs-event-single-macro area=bootstrap-phases-derive-for-route-setup-project rec=29,55
name: One obs_event macro for every line
rule: Every log line is emitted through the one obs_event macro defined in viola-core, which expands to tracing's event at the caller, so viola-core stays I/O-free and tracing-free.
@row_add n=46 row=decision/direct-chrono-dependency area=bootstrap-phases-derive-for-route-setup-project rec=29
name: Direct chrono dependency for timestamps
rule: The root binary takes chrono directly, without default features, to produce the millisecond UTC timestamp.
why: tracing-subscriber's chrono feature does not re-export chrono
@row_add n=47 row=decision/raw-tracing-ban area=bootstrap-phases-derive-for-route-setup-project rec=29,35
name: Raw tracing macros banned
rule: The tracing level macros are banned by clippy disallowed-macros and raw event calls by a fail-closed grep outside the obs module, so only obs_event emits lines.
why: clippy reports a disallowed macro expanded inside an exported macro at the caller, so no allow inside obs_event can exempt it
guard: clippy disallowed-macros and scripts/lint-probes.sh
@row_add n=48 row=decision/single-service-identity-source area=bootstrap-phases-derive-for-route-setup-project rec=30
name: One source for service name and version
rule: The core crate's service name and version constants are the single source for process-start version, sender and writer, with the workspace version shared by every member.
@row_add n=49 row=decision/diag-schema-matches-event-enum area=bootstrap-phases-derive-for-route-setup-project rec=31
name: Diagnostic line schema matches the event enum
rule: A hand-maintained JSON Schema of the process-log line is committed, and its event enum must equal the ObsEvent variants, asserted by a tests-owned check.
@row_none n=17 rec=32 type=4
relied: Cross-process joins rely on conn being added to the channel client's params and server logging.
@row_add n=50 row=decision/liveness-transitions-not-logged area=bootstrap-phases-derive-for-route-setup-project rec=33,42
name: Liveness transitions not logged
rule: No process-log line carries a liveness transition; liveness is exposed only by readers such as list --json.
why: no process watches another instance's heartbeat
@row_add n=51 row=decision/instrument-skip-all-everywhere area=bootstrap-phases-derive-for-route-setup-project rec=34,38
name: Spans skip all arguments
rule: Every instrumented function uses skip_all with explicit fields, so no argument is recorded implicitly.
guard: G1
@row_add n=52 row=decision/veil-redact-payload-types area=bootstrap-phases-derive-for-route-setup-project rec=34,38
name: veil redacts payload types
rule: Payload types derive veil's Redact, and cargo deny bans veil's toggle feature.
guard: cargo deny veil toggle ban
@row_add n=53 row=decision/fixed-error-display area=bootstrap-phases-derive-for-route-setup-project rec=34
name: Fixed error Display text
rule: Typed errors have fixed Display text that carries no interpolated content.
@row_add n=54 row=decision/content-only-in-instance-detail-file area=bootstrap-phases-derive-for-route-setup-project rec=34,44
name: Content only in instance-scoped detail files
rule: Content-bearing detail goes only to the instance's detail-<process>.ndjson file; home-level log files are codes-only and carry no user content, token, cookie or CLAUDE* value.
why: security allows user-derived content only in instance-scoped diagnostics
@row_add n=55 row=decision/no-print-macros area=bootstrap-phases-derive-for-route-setup-project rec=35
name: No print macros in product crates
rule: print_stdout, print_stderr and dbg_macro are denied in every product crate, each of which must inherit the workspace lints.
why: a product member without workspace lints silently escapes the ban
guard: CI member assertion and a clippy failure on a println in a hook path
except: the e2e harness and the fake agent must print, so they are exempt
@row_add n=56 row=decision/uploads-gated-on-secret-scan area=bootstrap-phases-derive-for-route-setup-project rec=35
name: CI uploads gated on the secret scan
rule: CI artifact uploads run only after the secret scan, which runs always, in the fixed observability step order.
guard: secret-scan
@row_add n=57 row=decision/sha-pinned-ci-actions area=bootstrap-phases-derive-for-route-setup-project rec=35
name: SHA-pinned CI actions
rule: The ripgrep install step and the upload-artifact action are pinned by SHA.
@row_none n=18 rec=36 type=0
what: the one-time ordering of bootstrap setup phases
@row_none n=19 rec=37 type=0
what: the one-time placement of a merged bootstrap setup phase
@row_none n=20 rec=39 type=S
@row_none n=21 rec=40 type=S
@row_add n=58 row=decision/heartbeat-file-not-logged area=heartbeat-ticks rec=41
name: Heartbeat file touched, not logged
rule: run touches its instance heartbeat file every second and logs nothing per tick; mcp, hook and short-lived verbs have no tick.
why: a one-second line into a never-rotated file would grow without bound
@row_add n=59 row=contract/list-json/liveness area=heartbeat-ticks rec=42
boundary: list --json
item: liveness
promise: list --json exposes each instance's liveness.
@row_add n=60 row=contract/list-liveness/stale-threshold area=heartbeat-ticks rec=43
boundary: list liveness
item: stale threshold
promise: An instance is live while its heartbeat is at most 5 s old and stale or gone beyond that.
@row_add n=61 row=contract/agent-run-status/state-degraded area=heartbeat-ticks rec=43
boundary: agent-run status
item: state degraded
promise: The status is degraded when a booted instance's item is stale or absent.
@row_add n=62 row=contract/diagnostics-files/paths area=log-file-location rec=44
boundary: diagnostics files
item: process log paths
promise: Process logs are written to <home>/diagnostics/ as run-<name>, hook-<name>, mcp and cli-<name> .ndjson files, and detail to <home>/instances/<name>/diagnostics/detail-<process>.ndjson.
@row_add n=63 row=contract/diagnostics-files/home-resolution area=log-file-location rec=44
boundary: diagnostics files
item: home resolution
promise: The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
@row_add n=64 row=decision/log-files-owner-only area=log-file-location rec=44
name: Owner-only log files
rule: Log files are created 0600 inside 0700 directories.
guard: secret-scan test
@row_add n=65 row=decision/no-log-rotation-in-v1 area=log-file-location rec=45
name: No log rotation in v1
rule: Diagnostics are not rotated and no rotation library is used; retention is the lifetime of the session home.
@row_add n=66 row=decision/tracing-appender-rejected area=log-file-location rec=45
name: tracing-appender rejected
rule: tracing-appender is not used as a log writer.
why: its non_blocking writer drops lines, and it pulls in an advisory-affected time crate
@row_add n=67 row=decision/obs-aligns-to-tests-log-format area=log-format-json-schema rec=46,47
name: Observability aligns to the tests log format
rule: The log format is bound by the test plan's harness contract; observability may add fields but must not rename or remove its fields.
why: the harness greps on them
@row_add n=68 row=contract/event-stream/line area=log-format-json-schema rec=47
boundary: the event stream
item: event line
promise: Each line of instances/<name>/events.ndjson is exactly {"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}.
@row_add n=69 row=contract/process-log-line/timestamp area=log-format-json-schema rec=47
boundary: the process log line
item: timestamp
promise: RFC 3339 UTC with milliseconds and Z.
@row_add n=70 row=contract/process-log-line/level area=log-format-json-schema rec=47
boundary: the process log line
item: level
promise: One of DEBUG, INFO, WARN, ERROR.
@row_add n=71 row=contract/process-log-line/target area=log-format-json-schema rec=47
boundary: the process log line
item: target
promise: The target is always present.
@row_add n=72 row=contract/process-log-line/message area=log-format-json-schema rec=47,55
boundary: the process log line
item: message
promise: The message is a static literal equal to the event name and never carries interpolated content.
@row_add n=73 row=contract/process-log-line/event area=log-format-json-schema rec=47
boundary: the process log line
item: event
promise: A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
@row_add n=74 row=contract/process-log-line/process area=log-format-json-schema rec=47,50
boundary: the process log line
item: process
promise: One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing cli-<name>.ndjson.
@row_add n=75 row=contract/process-log-line/instance area=log-format-json-schema rec=47
boundary: the process log line
item: instance
promise: The instance's name, or absent when none resolved.
@row_add n=76 row=contract/process-log-line/corr area=log-format-json-schema rec=47,55
boundary: the process log line
item: corr
promise: Copied unchanged as a JSON number or string, never renamed and never debug- or display-formatted; required only on the events whose rules always define it.
@row_add n=77 row=contract/corr-by-event/channel area=log-format-json-schema rec=47
boundary: corr by event
item: channel-request and channel-response
promise: corr is the JSON-RPC request id; null for an id-less hook.event notification and on a response to a frame that yielded no id.
@row_add n=78 row=contract/corr-by-event/dialog area=log-format-json-schema rec=47
boundary: corr by event
item: dialog-raised and dialog-answered
promise: corr is the dialog_id.
@row_add n=79 row=contract/corr-by-event/hook area=log-format-json-schema rec=47
boundary: corr by event
item: hook-invoked and hook-decision
promise: corr is the dialog_id for dialog hooks, otherwise null; a dialog hook-invoked before the id is known, or a hook-decision carrying detail, may be null.
@row_add n=80 row=contract/corr-by-event/send area=log-format-json-schema rec=47
boundary: corr by event
item: send-issued, send-confirmed, send-refused
promise: corr is the send cursor; a wrapper refusal before send-issued carries the target's event-stream end offset, never returned to the caller; wrapper send lines carry conn and rpc_id; the client-side send-refused has corr null.
@row_add n=81 row=contract/corr-by-event/release-from-driver area=log-format-json-schema rec=47,49
boundary: corr by event
item: release-from-driver
promise: corr is the JSON-RPC request id of the release refused with -32602 for carrying a string from.
@row_add n=82 row=contract/corr-by-event/process-and-panic area=log-format-json-schema rec=47
boundary: corr by event
item: process-start, process-exit, panic
promise: corr is null.
@row_add n=83 row=contract/corr-by-event/state-recovered area=log-format-json-schema rec=47,49
boundary: corr by event
item: state-recovered
promise: corr is null.
@row_add n=84 row=contract/corr-by-event/parse-rejected area=log-format-json-schema rec=47,49
boundary: corr by event
item: parse-rejected
promise: corr is null.
@row_add n=85 row=decision/null-as-key-absence area=log-format-json-schema rec=47,52
name: Null written as key absence
rule: A null corr or instance is written as an absent key, readers treat absence and null as equal, and the schema rejects a literal null.
why: tracing has no null field value
@row_add n=86 row=decision/hook-log-write-failure-swallowed area=log-format-json-schema rec=47
name: Hook log write failure swallowed
rule: A failed write to the hook log is swallowed, so the hook still exits 0 and never writes to stderr.
@row_add n=87 row=decision/no-secrets-in-log-lines area=log-format-json-schema rec=47
name: No secrets in log lines
rule: No log line may contain a Cookie header or any stripped CLAUDE* value.
guard: secret-scan test
@row_add n=88 row=decision/no-multiline-stack-traces area=log-format-json-schema rec=47
name: No multi-line stack traces
rule: Every log line is one physical line parseable by jq; a hook panic is caught and logged as one ERROR line with event panic.
@row_none n=22 rec=48 type=8
@row_none n=23 rec=51 type=S
@row_none n=24 rec=53 type=S
@row_none n=25 rec=54 type=0
what: an illustrative example log line
@row_add n=89 row=decision/tracing-logging-library area=logging-stack rec=55
name: tracing as the logging library
rule: tracing is the logging facade and tracing-subscriber's JSON formatter writes the lines.
why: Tokio-free and no C, and meets the security floor for tracing-subscriber
@row_add n=90 row=contract/obs-event-macro/attached-fields area=logging-stack rec=55
boundary: the obs_event macro
item: attached fields
promise: Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.
