# obs-plan · the rows that bind 2026-11-22-epoch-4-cleanup — picked 337 of 1037 (213 by every draw) · rule any · pin 3998

## obs-plan-viola · (above the first heading)
decision/agent-readable-telemetry · Agent-readable telemetry only — Every telemetry signal must be readable by an agent from a snapshot or pasted log excerpt: no human-only dashboards, no proprietary APM lock-in, no human-review-gated log analysis. · except: A surface that cannot be agent-readable is flagged not-instrumentable and handled through boundary instrumentation only.

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/agents-probe-output-untrusted · Agents probe output untrusted — The output rows of the claude agents --json probe are untrusted.
decision/agents-probe-parse-failure-unknown · Agents probe parse failure becomes unknown — A parse failure of the claude agents --json probe becomes status "unknown" plus a parse-rejected counter; an unparsed --version answer leaves cli_version absent and the version unverified.
decision/child-exit-from-process-handle · Child exit from the process handle — The wrapped child's exit and its process-exit event come from the wait on the process handle, never from reader EOF.
decision/claude-internals-not-a-contract · Wrapped claude internals are not a contract — The wrapped claude CLI is a third-party binary run unmodified and instrumented at its boundary only (spawn and exit, paste writes, the vt100 readiness-gate outcome); its internal state, transcripts and bundle strings, the MCP client's tool-call bound among them, are never read as a contract. · except: A send's echo confirmation counts its stdout tag in the transcript, keeping no byte; verify writes and removes its temp folder's trust record; a verified start sets its folder's trust flag only.
decision/cli-file-sink-only · Short-lived CLI logs to a file sink only — Short-lived CLI verbs log only to a JSON-per-line file sink in diagnostics when an instance resolves, never to stdout. · why: Stdout carries the --json results.
decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits.
decision/error-chain-fixed-stderr-message · Error chains reach stderr as fixed messages — In the viola root binary, the only crate that uses anyhow, an error chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to the diagnostics logs only.
decision/obs-deps-tokio-free · Obs dependencies Tokio-free and C-free — Obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass cargo deny. · guard: cargo deny
decision/os-apis-at-call-sites · OS APIs observed at call sites — OS APIs (openpty, Unix socket, filesystem locks and atomic rename, sysinfo) and the hyprctl spawn are instrumented at viola's call sites only, never in their internals.
decision/parse-failures-unknown-counter · Parse failures surface as unknown plus a counter — A child-output parse failure surfaces as `unknown` plus a counter. · except: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown.
decision/pty-byte-stream-never-logged · PTY byte stream never logged — The PTY byte stream is never logged; the viola-pty seam is instrumentable, and the openpty OS API behind it is boundary-only. · why: The PTY byte stream is user content.
decision/refusal-telemetry · Refusal telemetry for filesystem refusals — Pinned-exe hash mismatches and ignored symlinks produce refusal telemetry. · except: The transcript echo read and the verify and start trust-record writes refuse a symlinked, non-regular or foreign file with no refusal line; the send's lines, stamps or trust-record word show it.
decision/run-silent-terminal · viola run silent on the terminal — viola run writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns.
decision/run-sync-instrumentation · Sync instrumentation in viola run — Instrumentation in viola run is sync, with no Tokio.
decision/send-text-confined · Send text confined to instance diagnostics — `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
decision/send-text-never-logged · Send text never logged — Sent text never appears on stderr, in --json output, in MCP results or in any process-log field outside instances/<name>/diagnostics/.
decision/serde-errors-fixed-message · Serde error chains become a fixed message — A serde-sourced anyhow error chain is replaced by a fixed message before it reaches stderr.
decision/spans-not-wire-contract · Tracing spans are not part of the wire contract — Span names are internal `tracing` spans in `<area>.<operation>` snake_case form; the agent-facing contract is the closed `event` enum plus `corr`. · why: the config sets `with_current_span(false)`, so spans do not reach the line
decision/state-sync-instrumentation · Sync instrumentation in viola-state — Instrumentation in viola-state must be sync. · why: The Stack bans Tokio in viola-state.
decision/stderr-fixed-strings · Fixed strings on stderr — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/strict-modes-failure-lines · Where a strict-modes failure is logged — A strict-modes failure has a log line only on the `ledger-stamps` read in the `run` version gate and on `verify`'s own `process-exit` for a refused stamp write.
decision/stripped-claude-env-never-logged · R8-stripped CLAUDE values never logged — R8-stripped CLAUDE* environment values are never logged anywhere. · why: The viola-agent-claude outputs are the most sensitive in the product.
decision/stripped-env-never-logged · Stripped CLAUDE env values never logged — No R8-stripped `CLAUDE*` value appears in any sink or fixture; the strip is logged as a count or variable names only.
contract/diagnostics-files/cli · the diagnostics files · cli — Role cli writes <home>/diagnostics/cli-<name>.ndjson, only when an instance resolves.
contract/diagnostics-files/run · the diagnostics files · run — Role run writes <home>/diagnostics/run-<name>.ndjson.
contract/run-log-file/path · the viola run log file · path — viola run logs only to a file sink at diagnostics/run-<name>.ndjson.
decision/absolute-paths-scrubbed-externally · Absolute paths scrubbed from external output — Absolute paths in snapshot.json (endpoint, pinned_bin) and in config values are scrubbed from external output and allowed in diagnostics/.
decision/corr-field-propagation · Propagation through the corr field — Correlation across the viola-channel is carried by the corr field in log lines of the process on each side. · why: Channel params declare no trace-context field.
decision/distinct-detail-code-per-exit-cause · Distinct detail code per exit cause — Each cause behind exit 21 and exit 1 gets a distinct detail code with no path or pid. · except: A strict-modes check refused before any log opens writes no line; its cause is the fixed stderr pair or the --json document.
decision/drift-reports-diagnostics-only · Drift reports go to diagnostics only — Drift reports of viola-agent-claude go to diagnostics/ only and nowhere else. · why: The outputs of viola-agent-claude are the most sensitive in the product.
decision/instance-never-from-working-directory · Instance never from the working directory — The instance comes from VIOLA_NAME or an argument, never from the working directory.
decision/tracing-json-line-file-pipeline · tracing JSON-per-line file pipeline — Telemetry is tracing plus tracing-subscriber writing JSON-per-line files under diagnostics/, correlated by corr and consumed through agent-run logs and status. · why: The pipeline is the one the tests plan binds.
decision/tracing-json-pipeline · tracing with the tracing-subscriber JSON formatter — The `tracing` facade plus the tracing-subscriber JSON formatter is the binding logging pipeline, and JSON-to-file is the only logging mode.
decision/typed-keys-never-logged · Typed keys never logged — Keys a human types never appear in a log line; the wheel product event in events.ndjson is the audit record.
contract/diag-line/instance · diagnostic log line · instance — The instance's `ViolaName`; the key is absent when the process has no instance.
contract/diag-line/process · diagnostic log line · process — The role of the writing process, one of run, hook, mcp, cli; required on every line and matching `sender` / `writer`.
contract/diag-line/service-name · diagnostic log line · service.name — Hardcoded to "viola".
contract/diag-line/service-version · diagnostic log line · service.version — The crate version, fixed at compile time.
contract/log-line/process · every log line · process — Every log line carries a process key naming its role: one of run, hook, mcp, cli.
contract/process-exit-detail/strict-modes-failed · process-exit detail codes · strict-modes-failed — Carried by `verify`'s `process-exit` for a refused stamp write, with exit 1.

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/agents-cli-instrumented-at-boundary · claude agents CLI instrumented at spawn, exit and parse — The external `claude agents --json` CLI is instrumented only at its spawn, its exit and its parse outcome. · why: An external CLI.
decision/agents-cli-parse-failure-unknown · claude agents parse failure reads unknown — A failed parse of `claude agents --json` output reads "unknown" plus a counter.
decision/child-exit-from-handle-wait · Child exit from the process handle — The child's process-exit comes from the wait on its process handle, never from reader end-of-file. · why: Child exit is detected on the process handle.
decision/claude-boundary-only-instrumentation · Boundary-only instrumentation of claude — The hosted claude binary is instrumented only at its boundary: spawn and exit, paste writes, the vt100 readiness-gate outcome, and the start's one trust-record line for the trust record it writes in the claude config. · why: claude is a third-party binary run unmodified.
decision/correlation-in-corr-field · Correlation in the corr log field — Correlation across a channel call travels in the corr log field; channel params declare no trace-context field.
decision/detail-codes-no-paths · Detail codes carry no path or pid — No path or pid appears in a detail code.
decision/instrument-call-sites-not-os-internals · OS operations instrumented at viola's call sites — OS-level operations (bind, connect, lock, rename, kill fallback, the server check's peer reads, the strict-modes stat reads, the socket directory check, the hyprctl spawn) are instrumented at viola's own call sites, not inside the OS.
decision/linux-only-os-target · Linux-only OS target — The OS target is Linux only: the core on any current distribution, CI on ubuntu-latest.
decision/messaging-secrets-never-logged · Messaging secrets never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every R8-stripped CLAUDE* value are never logged in any sink, detail files and fixtures included. · guard: secret-scan test
decision/no-process-log-without-instance · No process log without an instance — A short-lived CLI verb run with no resolved instance writes no process-log file; its human stderr carries only fixed-message lines, and its outcome is the exit code and stdout.
decision/no-secrets-in-log-lines · No secrets in log lines — No log line may contain any stripped CLAUDE* value. · guard: secret-scan test
decision/no-telemetry-from-screen · No telemetry from screen content — No telemetry is derived from screen content; vt100 serves gate outcomes only.
decision/obs-deps-no-c-building-crates · Obs dependencies build no C — Observability dependencies add no C-building crates.
decision/obs-deps-pass-cargo-deny · Obs dependencies pass cargo deny — Observability dependencies pass cargo deny on its one target, x86_64-unknown-linux-gnu. · guard: cargo deny
decision/parse-failures-surface-as-unknown · Parse failures surface as unknown — A parse failure surfaces as the value unknown plus a counter. · except: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown.
decision/per-parser-parse-failure-counts · Per-parser parse-failure counts — Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, the hyprctl replies, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead.
decision/r8-strip-logged-as-names-only · R8 strip logged as names only — The R8 environment strip is logged as a count and lists of stripped and kept variable names, and environment maps never pass to a span or event; no value is read, bar the kept CLAUDE_CONFIG_DIR value that viola verify and the run and revive start read, and no value is logged.
decision/single-line-log-records · Single-line log records — Every log record is one JSON line parseable by jq; never a multi-line stack trace.
decision/stripped-claude-values-never-logged · Stripped CLAUDE values never logged — R8-stripped CLAUDE* values are never logged anywhere. · why: Its outputs are the most sensitive in the product.
decision/tests-event-list-adopted · Tests event list adopted — The tests' field and event list is adopted verbatim, with send-issued and send-refused among the logged events.
contract/process-log-corr/process-exit · Process log corr by event · process-exit — corr is null.
contract/process-log-corr/process-start · Process log corr by event · process-start — corr is null.
contract/process-log-corr/send-refused · Process log corr by event · send-refused — corr is the send cursor.
contract/process-log/child-and-probe-lifecycle · Process log · child and probe lifecycle — process-start and process-exit are logged for the claude child, for the claude agents --json and --version probes, and for verify's print-mode sender.
contract/process-log/corr · Process log line · corr — Copied unchanged as a JSON number or string, never renamed.
contract/process-log/event · Process log line · event — A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
contract/process-log/instance · Process log line · instance — A ViolaName or null.
contract/process-log/level · Process log line · level — One of DEBUG, INFO, WARN, ERROR.
contract/process-log/message · Process log line · message — A required field, as fields.message or the flattened message.
contract/process-log/timestamp · Process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
contract/send-log-line/corr · Send log line · corr — On the wrapper's send lines corr is the send cursor, which joins send-issued, send-confirmed and send-refused (a refusal before send-issued carries the log's end offset); the CLI's and MCP's client-side send-refused lines carry no corr; on channel lines corr is the JSON-RPC id.
contract/send-log-line/detail · Send log line · detail — The send detail is one of input-not-ready, no-prompt-submitted, control-character, empty-text, turn-running or paste-cut.
contract/send-log-line/required-fields · Send log line · required fields — A send line carries event, process, instance, corr, from, confirmed, refusal, detail and duration_ms.
contract/send-path-log-events/event-order · Send path log events · event order — A send logs channel-request, then send-issued, then hook-invoked (UserPromptSubmit, or SessionStart for a confirmed /clear or /compact; none awaited for an unconfirmable send, a cut paste or a local command its transcript echo confirms), then send-confirmed or send-refused, then channel-response.
decision/agents-cli-output-untrusted · claude agents output untrusted — The output rows of `claude agents --json` are treated as untrusted.
decision/anyhow-error-capture-boundary · The anyhow crate is the error-capture boundary — The only crate that uses anyhow is the error-capture boundary, with context chains only at dispatch; the full chain goes to diagnostics only.
decision/fault-telemetry-per-class · Fault telemetry per class — Each fault class emits its own telemetry: a healed torn ndjson line with file and offset, a corrupt or unsupported snapshot recovered by replay, a child exit seen without EOF, an endpoint vanished during wait, and a stale heartbeat against a live pid.
decision/instance-from-name-not-cwd · Instance from its name, never the working directory — The instance comes from VIOLA_NAME or an explicit argument, never from the working directory.
decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but a required one the product still emits is never renamed or removed. · why: The harness greps on them.
decision/log-file-basenames-follow-tests · Log file names follow tests — Process-log files are <home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson, as tests require. · why: Tests is upstream; arch and security place diagnostics under the instance directory instead.
decision/process-init-order · Process logging init order — Each process installs its panic hook, parses arguments, resolves home and instance and runs its strict-modes check, opens its diagnostics file append-mode 0600 in a 0700 directory, installs the subscriber, then emits process-start. · why: For run this completes before the start sequence so the collision check and version gate are logged; for mcp, before the stdio transport starts.
decision/run-silent-on-terminal · run silent on the terminal — run writes nothing to the terminal while the child runs; its process log goes only to diagnostics/run-<name>.ndjson. · why: The terminal carries the child's screen.
decision/send-refused-by-each-deciding-side · Send refusal logged by each deciding side — A send-refused with detail control-character or empty-text is emitted by each side that decides it, client and wrapper, without the text.
decision/state-crate-sync-instrumentation · Synchronous instrumentation in the state crate — Instrumentation in the state crate built on ndjson, atomic-write-file and sysinfo must be synchronous. · why: Stack bans Tokio in that crate.
decision/strict-modes-refusal-writes-no-line · Strict-modes refusal writes no line — A strict-modes check refused before any log opens ends the process with nothing under the home opened and no log line; its cause is carried only by the fixed stderr pair or the --json document. · why: Ratified as the answer to whether a refused strict-modes check logs. · except: The ledger-stamps read in run's version gate and a refused update_stamps in viola verify each leave a line.
decision/sync-instrumentation-no-tokio · Sync instrumentation without Tokio — Instrumentation in the sync crates runs on std threads with no Tokio. · why: Hybrid concurrency. · guard: cargo deny ban
contract/process-log-corr/panic · Process log corr by event · panic — corr is null.
contract/process-log-corr/send-confirmed · Process log corr by event · send-confirmed — corr is the send cursor.
contract/process-log-corr/send-issued · Process log corr by event · send-issued — corr is the send cursor.
contract/process-log/process · Process log line · process — The process role, one of run, hook, mcp, cli.
contract/process-log/target · Process log line · target — A required field.
contract/refusal-reason/unknown-fallback · Refusal reason · unknown fallback — An unrecognised RefusalReason falls back to unknown with exit 14.
contract/run-log/cli-verified · run log line · cli_verified — A required field on run's log lines.
contract/run-log/exit-code · run log line · exit_code — A required field on run's log lines.
contract/send-log-line/from · Send log line · from — The from field is self-reported and logged as unauthenticated, never as an identity.
contract/viola-verify/home-tree-refused · viola verify · refused home-tree check — A refused home-tree check exits 1 with the home pair, opens no log and writes no line; a refused update_stamps exits 1 with the home pair and, when an instance is named, ends the opened log with a process-exit line carrying detail strict-modes-failed.

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/one-line-panic-record · One structured line per panic — Every panic after init step 4 produces exactly one single-line structured record in its home-level role file with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file. · except: the bounded panic exemptions listed in the plan's Section 10
contract/process-log-line/duration-ms · the process log line · duration_ms — Durations are duration_ms as an integer.
contract/process-log-line/enum-values · the process log line · enum values — Enum values are kebab-case.
contract/process-log-line/event · the process log line · event — The event field is a closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
contract/process-log-line/field-names · the process log line · field names — Field names are snake_case.
decision/no-event-payloads-in-process-logs · No event payloads in process logs — Obs never copies event payloads into process logs.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/panic-caught-at-every-role · Panics caught at every role's catch site — A hand-written panic hook plus catch_unwind at every role's main-thread catch site (covering hook dispatch and run's inline handle-wait), the three run PTY threads (the two pump copy threads and the paste writer thread) and the vt100 feed turn a panic into one record.
decision/panic-one-line-record · One single-line record a panic — Every panic after obs init produces exactly one single-line structured record in its home-level role file, with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file. · except: the pre-init window and the paths that refuse or lack an instance or home tree before their role file opens
decision/events-stream-never-rotated · Event stream never rotated — The instance's events.ndjson is the product audit stream, never rotated by a generic library nor rewritten: a reused instance name keeps the earlier starts' lines at its head, so a start's own lines are read from where that start begins. · why: byte offsets in it are the wait-after and send cursors

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/codes-only-process-logs · Codes-only process logs, content in detail files — Every role's codes-only process log goes to its home-level role file under diagnostics, and content-bearing detail goes only to the instance's per-role detail file.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/child-settings-never-logged · Child settings argument never logged — The claude child's --settings argument appears in no log line or span.
decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.
decision/compact-confirm-window · Compact confirmation window — A /compact send waits 40 s for its compacted-session post-condition, while every other confirmed send waits 10 s. · provisional
decision/detail-codes-only · detail is a closed code — The detail field is a closed kebab-case code catalog and never carries a path or pid.
decision/env-names-only · Environment names only, never values — The strip, every log, the hook capture arm's env-names file and the verify stamp and record carry environment variable names only, never values, and no env map is passed to a span or event. · guard: E2
decision/ledger-contents-never-logged · Capability ledger path and stamps never logged — The capability ledger's path and its stamp contents never appear in a log line.
decision/no-assistant-message-in-logs · No assistant message or result body in logs — last_assistant_message never appears in a log, and no result body reaches a log line.
decision/sent-text-never-logged · Sent text never logged — A sent message's text is never logged; only its size in text_bytes is.
decision/trust-record-write-uninstrumented · Trust-record write and removal uninstrumented — viola verify's trust-record write and removal emit no log line and no span, the outcome riding only the stamps; a start's trust step opens no span and writes only its one trust-record line; no path, key or value of the claude config reaches any line, stamp or summary.
decision/unreadable-ledger-is-unverified · Unreadable ledger counts as unverified — A missing stamp, or an unreadable, malformed or strict-modes-refused capability ledger, counts as unverified (cli_verified false).
decision/verify-lines-redacted · verify lines carry no content — No verify or plugin install line carries row text, a prompt, a session id or a path.
decision/verify-logs-only-with-instance · verify and plugin install log only inside an instance — With VIOLA_NAME set, verify and plugin install log process-start / process-exit to cli-<name>.ndjson; from a plain terminal with no instance they write no process-log file.
contract/claude-child-process-start/env-kept · process-start line of the claude child · env_kept — Reports the variables kept after the R8 environment strip on process-start{subject:"claude-child"} by names only, never values.
contract/claude-child-process-start/env-stripped-count · process-start line of the claude child · env_stripped_count — Reports the R8 environment strip on process-start{subject:"claude-child"}; never an environment value.
contract/claude-child-process-start/env-stripped-known · process-start line of the claude child · env_stripped_known — Reports the R8 environment strip on process-start{subject:"claude-child"} by variable names only, never values.
decision/browser-tools-prompt-reading-uninstrumented · Browser-tools prompt reading uninstrumented — The reading of the CLI's browser-tools prompt as a modal adds no log line, span, event or detail code of its own; verify's reading of it reaches only the ledger stamp and verify's stdout, whose one fixed line on a met prompt is the only record that the prompt stood at start.
decision/dialog-content-never-logged · Dialog content never logged — Dialog questions, answers, plan text and tool input never appear in a home-level process-log line; dialog events carry codes only, and the text reaches an instance detail file only inside a chain or drift report.
decision/standing-modal-reading-uninstrumented · Standing-modal reading uninstrumented — The readiness gate's standing-modal reading adds no log line, span field, event or detail code of its own: a modal label quoted above a ready input box lets the send proceed with nothing written, and a standing modal refuses with the existing detail input-not-ready.
contract/viola-verify-refusal/stamp-write-refused · viola verify strict-modes refusal · stamp write refused — A ledger/stamps.json the stamp write refuses prints the home pair and exits 1 through verify's own process-exit line.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/claude-env-strip · CLAUDE environment strip — The child loses every inherited CLAUDE* name, matched exactly, except the persistent set kept in the claude_env_keep config key.
decision/cli-log-only-under-viola-name · cli process log only under a parsed name — A short-lived cli verb opens its process log and writes process-start and process-exit only when an instance resolves: for list, plugin install and revive --list under a parsed VIOLA_NAME (with no name no log), for the frame verbs under the named target (link and unlink the driven) instance.
decision/hidden-capture-arms-uninstrumented · Hidden hook capture arms uninstrumented — The hidden hook capture arm used by verify's probes reads no VIOLA_* variable, does no obs init, emits no hook lines, keeps stderr empty and always exits 0. · why: uninstrumented by design, as ratified by the founder
decision/no-text-in-telemetry · Sent text never in telemetry — The sent text never appears in a span or log line; only its length does.
decision/portable-pty-log-target-off · portable-pty log target OFF — The log target of portable-pty is OFF in the sink, and PTY failures are captured at the pty seam instead.
decision/real-cli-verify-never-in-ci · Real-CLI verify never runs in CI — In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
decision/send-cut-mark-uninstrumented · Send cut mark uninstrumented — The cut mark a cut paste leaves is in-memory wrapper state that opens no span and writes no line of its own; its set shows as the cut send's send-refused paste-cut record and its clears as their own prompt-submitted, session-start and session-end records.
decision/send-line-correlation · Send lines correlate with channel lines — Every wrapper send-* line carries conn (or srv_conn) and rpc_id, so that pair on the send line equals (conn, corr) on the call's channel lines; the client-side send-refused carries neither.
decision/unreadable-ledger-means-unverified · Unreadable capability ledger means an unverified CLI — A capability ledger that is missing a stamp, unreadable, malformed or refused by the strict-modes check is treated as an unverified CLI (cli_verified false).
decision/verify-lines-carry-no-content · verify log lines carry no content — No verify log line carries row text, a prompt, a session id, the probe command or a path.
decision/verify-sole-stamps-writer · verify is the only stamps writer — verify is the only writer of the capability ledger's stamps file.
contract/claude-child-process-start/env-fields · process-start claude-child · env fields — env_stripped_count, env_stripped_known (every stripped name, comma-joined) and env_kept (every kept persistent-set name, comma-joined).
contract/confirm-window-close/clear · run.confirm_window close · /clear — On a verified CLI, closes on the session-start whose cause is clear and whose agent_session_id differs from the id the send slot remembers; no prompt-submitted claims the send.
contract/confirm-window-close/compact · run.confirm_window close · /compact — On a verified CLI, closes on the session-start whose cause is compact and whose agent_session_id equals the id the send slot remembered; nothing remembered claims nothing, and no prompt-submitted claims the send.
contract/confirm-window-close/context · run.confirm_window close · /context — On a verified CLI, closes on a new local-command-stdout tag in the session transcript within the fallback window, with no prompt submitted; on an unverified CLI, or before the session transcript exists, the send reads unconfirmable.
contract/confirm-window-close/ordinary-send · run.confirm_window close · ordinary send — Closes on the prompt-submitted{origin:"driver"} whose prompt equals the typed text, or is non-empty and equals the typed text with a trailing run of ASCII space, TAB, CR or LF removed, or on window expiry.
contract/revive-process-log/agents-probe · viola revive run process log · agents-probe — The session-live reading writes one process-start and process-exit{subject:"agents-probe"} pair, plus one parse-rejected{parser:"claude-agents-json"} on an oversize or malformed answer.
contract/run-log-parse-rejected/ledger-stamps · the run process log parse-rejected line · parser ledger-stamps — An unreadable, malformed or strict-modes-refused capability ledger gives one parse-rejected line with parser "ledger-stamps", detail "unreadable", "malformed" or "strict-modes-failed", and a count, in the run log.
contract/run-span-attributes/state-snapshot-write · viola run start span attributes · state.snapshot_write — state.snapshot_write carries v.
contract/send-log-fields/send-confirmed · confirmed send log fields · send-confirmed — send-confirmed{corr:<cursor>, confirmed, duration_ms}, confirmed false for unconfirmable, duration_ms from the send's arrival including the readiness gate's wait.
contract/send-refused-detail/no-prompt-submitted · wrapper send-refused detail · no-prompt-submitted — Confirm-window expiry is send-refused{detail:"no-prompt-submitted"} for ordinary, /clear, /compact and echo-confirmed local command sends alike, with no new detail.
contract/send-refused-detail/paste-cut · wrapper send-refused detail · paste-cut — A paste cut at its bound refuses not-delivered paste-cut after send-issued, with the send's cursor; a send refused on a standing cut mark refuses paste-cut with no cursor and nothing issued.
contract/send-refused-detail/turn-running · wrapper send-refused detail · turn-running — Covers a running turn the wrapper knows of before the paste, or another send in flight, with no new field; the refusal has no cursor (corr the end offset at refusal), no send-issued, and the client exits 13.
contract/send-span-attributes/run-readiness-gate · confirmed send span attributes · run.readiness_gate — outcome (ready or input-not-ready) and vt100_panicked (bool).
contract/send-span-attributes/text-bytes · confirmed send span attributes · text_bytes — text_bytes, on pty.paste_write and send-issued alike, is the typed text's length: the sent text with each CR LF pair and each other CR as one LF, trailing CR and LF removed; pty.paste_write also carries paste_mode="bracketed".
contract/send-span-attributes/window-ms · confirmed send span attributes · run.confirm_window window_ms — window_ms is the bound the send's own deadline reads: 40 000 for a /compact send waiting for its compacted-session post-condition, else 10 000.
contract/verify-process-exit/interrupted · verify process-exit line · detail interrupted — A verify stopped by SIGINT or SIGTERM starts no further spawn, closes every pair it opened and ends with process-exit subject self, exit_code 1, detail "interrupted".
contract/verify-spawn-subject/agents-probe · verify child spawn subjects · agents-probe — Run B's single agents --json read is logged as one process-start / process-exit pair with subject agents-probe.
contract/verify-spawn-subject/verify-pty-probe · verify child spawn subjects · verify-pty-probe — Each of verify's six interactive PTY runs is logged as one process-start / process-exit pair with subject verify-pty-probe.
contract/verify-stdout/step-line · verify stdout · step line — One "[NN/MM] <row id> <row words>  pass|absent|fail" line is printed for each relied ledger row, MM being the relied row count; absent is the browser-tools-prompt row's word when no run met the prompt.
contract/verify-stdout/summary-line · verify stdout · summary line — The last stdout line is "stamped <version>  N pass  N fail", the two counts summing to MM; the pass count holds every row that counts, an absent browser-tools-prompt row among them.
decision/error-chain-in-detail-log-only · Full error chain only in the cli detail log — The full anyhow error chain of a cli verb goes to the cli detail log only.
decision/hook-capture-arms-unlogged · Hidden capture arms write no lines — The hidden capture arm of viola hook (hook <event> --capture) initialises no observability and writes no lines.
decision/identity-floor-always-stripped · Identity floor always stripped — Every identity-floor name is always removed whatever the persistent set says; beside the floor stands a compile-time list of known non-identity hook names, which the floor never strips.
decision/no-obs-gate-on-verify-lines · No obs gate depends on verify lines — No observability gate depends on verify's own log lines; its home-level lines fall under the same panic, schema and secret-scan gates as any other.
decision/no-typed-keys-in-telemetry · Typed keys never in telemetry — The keys the human types never appear in a span.
decision/refused-check-writes-no-line · A refused check opens no process log — A cli verb whose strict-modes, tree or instance check refuses (exit 21) opens no process log and writes no line.
decision/refused-home-tree-writes-no-line · Refused home tree writes no line — verify and plugin install check the home tree before their log opens, and a refused tree opens no log and writes no line.
decision/transcript-echo-read-uninstrumented · Transcript echo read uninstrumented — The send's transcript echo read, the end offset before the paste and the tag count after it, opens no span and writes no line or span field of its own; its result shows only as the send-confirmed line inside run.confirm_window.
contract/claude-child-process-log/handle-wait-exit · run process log, claude-child lines · process-start and process-exit of claude-child — process-start is logged once the child is spawned and process-exit once the pump returns; process-exit{subject:"claude-child"} carries child_exit_status and exit_source handle-wait or kill-fallback, and is never emitted on reader EOF.
contract/confirmed-send-spans/span-chain · confirmed send spans · span chain — send.client (CLIENT) › channel.request (CLIENT) → channel.dispatch (SERVER) › run.readiness_gate › pty.paste_write › run.confirm_window; an unconfirmable send ends at pty.paste_write. In the hook: hook.handle › channel.request(hook.event).
contract/run-process-log/common-fields · run process log · common fields — Every run line carries the common fields with process="run" and corr null.
contract/run-process-log/process-start-claude-child · run process log · process-start claude-child — On success, process-start{subject:"claude-child", child_pid, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept}.
contract/send-log-fields/hook-invoked · confirmed send log fields · hook-invoked — hook-invoked{hook_event:"user-prompt-submit", corr:null} for an ordinary send; hook_event session-start for a /clear or /compact confirmed by its session; an unconfirmable send, or a local command confirmed by its transcript echo, waits for no hook line.
contract/send-log-fields/send-issued · confirmed send log fields · send-issued — send-issued{corr:<cursor>, from, text_bytes}.
contract/span-kinds/internal · Span kinds · INTERNAL — The INTERNAL spans are run.*, state.* and pty.paste_write; a hook's decision is the decision_emitted field, not a span.
contract/verify-exit-codes/0 · verify exit codes · 0 — Every relied ledger row counted: each passed, or the browser-tools-prompt row read absent.
contract/verify-exit-codes/1 · verify exit codes · 1 — A row failed, verify refused, or verify was stopped.

## metric-coverage · ## 5. Metric Coverage · twin 5-metric-coverage
decision/panic-lines-stay-zero · Panic lines stay at zero — Gate G2 counts the panic lines of the home-level role files and fails on any; no viola.panic metric is derived.
decision/quantiles-report-only · Quantiles are report-only — Latency quantiles are for reports only; the gates use the max, and the quantile read skips a torn line rather than aborting.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/content-detail-in-instance-file · Content-bearing detail in the instance detail file — Content-bearing detail goes only to the per-process detail file inside the instance's diagnostics directory.
decision/log-schema-additive-only · Log schema additive to the tests' format — The observability log schema only adds fields and enum values to the tests' log format; it renames nothing and removes only a field no build emits. · why: The tests' log format is the binding text.
contract/child-spawn-log/claude-agents-json-parse-failure · child and shell spawn log events · claude agents --json parse failure — A claude agents --json parse failure gives one parse-rejected{parser:"claude-agents-json"} line and the value "unknown".
contract/child-spawn-log/subject · child and shell spawn log events · subject — process-start and process-exit lines for child spawns carry a subject from a closed set.
contract/log-levels/error · log levels · error — Used for a panic, a process-exit that carries a detail, and a channel-response with error_code -32603.
contract/log-levels/info · log levels · info — INFO carries all other boundary, lifecycle and state-transition events, including refusals, which are normal product outcomes.
contract/log-levels/warn · log levels · warn — WARN carries state-recovered; parse-rejected; hook-decision with a fail-open detail; process-exit with exit 21; channel-response with -32600 or -32602.
contract/log-line/corr · every log line · corr — Set per the tests' table and copied unchanged as a JSON number or string; null is represented by the key's absence.
contract/log-line/event · every log line · event — A value of the closed kebab-case event enum.
contract/log-line/level · every log line · level — One of DEBUG, INFO, WARN, ERROR; TRACE is never emitted.
contract/log-line/message · every log line · message — Flattened, a static literal equal to the event name.
contract/pty-seam-log/paste · PTY seam log events · paste — A paste is logged with text_bytes only.
contract/start-trust-log/trust-record · the start's trust-record log line · trust-record — Each wrapped start of viola run and viola revive on a verified CLI writes exactly one INFO trust-record line to the run role file, its detail one of a closed set of 16 start words, carrying no path, project key or config value; the word never refuses the start.
decision/additive-field-naming · Additive log field naming — Additive log fields are snake_case with kebab-case values, and carry codes only.
decision/file-only-per-role-log-sinks · File-only log sinks per role — Diagnostics are written to files only, one sink per role, under the home's diagnostics directory.
decision/no-event-type-field · No event_type field — Log lines carry no event_type field. · why: event already does that job, and a duplicate would amount to a rename.
contract/log-line/instance · every log line · instance — The ViolaName; the key is absent when there is none.
contract/log-line/target · every log line · target — The Rust module path, for example viola_channel::server.
contract/log-line/timestamp · every log line · timestamp — RFC 3339 UTC with milliseconds and Z.
contract/pty-seam-log/spawn · PTY seam log events · spawn — A spawn gives process-start{subject:"claude-child"}.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
contract/diag-detail/strict-modes-failed · diagnostic detail words · strict-modes-failed — Written on the process-exit exit 1 of viola verify under a parsed VIOLA_NAME and on parse-rejected ledger-stamps from the version gate of run, at warn, with no path; every other refused strict-modes check writes no log line, its exit code or JSON document its only record.
contract/diag-line-fields/parse-rejected · diagnostic line additive fields · parse-rejected — parser (claude-agents-json, vt100-feed, config-json, ledger-stamps, hook-stdin, channel-frame, mcp-stdin, state-entry, hyprctl-json), detail, count.
contract/diag-line-fields/process-exit · diagnostic line additive fields · process-exit — subject, exit_code (self), child_exit_status, exit_source (handle-wait or kill-fallback), detail, during (connect or call), duration_ms.
contract/diag-line-fields/process-start · diagnostic line additive fields · process-start — subject (one word of the diag-line subject enum), service_name, version, os, pid, child_pid, endpoint_kind, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept (names only).
contract/diag-line-fields/trust-record · diagnostic line additive fields · trust-record — A fixed message and a required detail drawn from the 16 start words; trust-record stands in the event enum of both the diag-line and the detail-line schema.
contract/diag-subject/agents-probe · diag-line subject enum · agents-probe — Subject of the process-start and process-exit pair of the claude agents read, written by viola list, viola revive's session-live reading, verify's Run B and the list tool of viola mcp into their role files.
contract/parse-rejected-detail/malformed · parse-rejected detail · malformed — Any parser.
contract/parse-rejected-detail/oversize · parse-rejected detail · oversize — channel-frame beyond MAX_FRAME, hook-stdin, mcp-stdin past MAX_FRAME, vt100-feed when the bounded queue drops a copy (one line per poisoning episode), claude-agents-json past MAX_FRAME, hyprctl-json past MAX_FRAME.
contract/parse-rejected-detail/panicked · parse-rejected detail · panicked — vt100-feed under catch_unwind.
contract/parse-rejected-detail/strict-modes-failed · parse-rejected detail · strict-modes-failed — ledger-stamps: run's version gate refused the stamps file on the strict-modes check, and the read counts as unverified.
contract/parse-rejected-detail/unreadable · parse-rejected detail · unreadable — config-json, ledger-stamps.
contract/parse-rejected/claude-agents-json · parse-rejected diagnostic line · parser claude-agents-json — A claude agents --json parse failure, oversize or malformed, gives one parse-rejected line with parser claude-agents-json and the value unknown.
contract/process-exit-detail/interrupted · process-exit detail · interrupted — A viola verify stopped by SIGINT or SIGTERM before its rows are checked ends with one process-exit exit 1 detail interrupted, after every spawn pair it opened is closed.
contract/send-refused-detail/refusal-reason-words · send-refused detail · RefusalReason detail words — Refusal details are the arch RefusalReason details verbatim: manual-pause, input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text, paste-cut.
contract/verify-process-exit/child-exit-status-and-duration · viola verify spawn process-exit lines · child_exit_status and duration_ms — Each verify spawn pair's process-exit carries child_exit_status and duration_ms, including a child of an interactive PTY run killed at a Ctrl-C or TERM stop; a stopped run writes only the pairs it started.
decision/session-live-check-fails-open · Session-live check fails open — A failed, oversize or malformed claude agents read passes the session-live check; the check is skipped under --fork.
contract/diag-line-fields/panic · diagnostic line additive fields · panic — panic_location (workspace-relative, or <crate>-<ver>/src/... for dependencies) and thread; payload and backtrace go to the detail file only.
contract/diag-line-fields/send-events · diagnostic line additive fields · send-issued / send-confirmed / send-refused — from, from_trust, text_bytes (normalised typed length), confirmed, refusal, detail, side (client or wrapper), wheel, duration_ms, and wrapper side only conn / srv_conn and rpc_id.
contract/diag-subject/claude-child · diag-line subject enum · claude-child — One value of the closed subject enum of the diag-line schema.
contract/diag-subject/verify-pty-probe · diag-line subject enum · verify-pty-probe — Subject of the process-start and process-exit pair of each of the six interactive PTY runs of viola verify, Run A to Run F.
contract/diag-subject/verify-sender · diag-line subject enum · verify-sender — Subject of the process-start and process-exit pair of the print-mode sender that viola verify's Run B cross-session turn spawns.
contract/process-exit-detail/verify-refusal · process-exit detail · viola verify refusal — Under a parsed VIOLA_NAME, verify writes its refusal code: program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture or interrupted; a failing row carries no code.
contract/send-refused-detail/empty-text · send-refused detail · empty-text — Written for an empty typed text on send; the word adds no field, and send-refused detail stays an open string in the diag-line schema.

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/boundary-errors-end-in-logged-line · Crate-seam errors end in a logged line — Every typed error that crosses a crate seam ends in a logged process-exit detail, channel-response error, hook-decision detail or send-refused line, carrying codes only at WARN or ERROR.
decision/full-error-chain-in-detail-file-only · Full error chain in the detail file only — The full error chain is written as a one-line JSON array chain:[...] only to the instance's per-process detail file, and only when an instance resolves.
decision/typed-errors-fixed-display · Typed errors with fixed Display messages — Every typed error has a fixed-message Display, written with thiserror, and holds no fields with paths or payloads. · except: PtyError's Display is hand-written, as viola-pty takes no thiserror.
decision/vt100-panics-contained · vt100 panics contained — vt100 panics are caught around each feed and resize on run's feed thread; each poisoning episode gives exactly one parse-rejected{parser:"vt100-feed"} at WARN, and run continues.
decision/error-correlation-without-trace-ids · Error correlation without trace ids — Errors correlate by corr, conn, process and instance, plus timestamp, not by trace_id or span_id.
decision/scrubbing-at-source · Scrubbing at the source — Error scrubbing sits at the source, not in a reporter's before_send hook. · why: No reporter exists.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
decision/vt100-feed-panic-contained · vt100 feed panic contained — A panic in a vt100 feed or resize is caught on the feed thread, poisons the screen model until the host size changes, writes one parse-rejected per episode, and run continues.
contract/answer-send-exit-2/stderr-line · viola answer and viola send exit 2 · stderr line — Exit 2 writes the fixed unusable-response or unusable-text line to stderr whatever --json says, with no document.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/claude-agents-rows-never-logged · claude agents rows never logged verbatim — claude agents --json rows are never logged verbatim, only parse-rejected counts.
decision/content-canary-never-home-level · Content canary never in home-level files — The content canary never appears in a home-level diagnostics ndjson file.
decision/content-only-in-detail-file · Content-bearing records only in the detail file — Content-bearing records go only to the instance-scoped detail-<process>.ndjson file.
decision/conversation-text-only-in-detail-file · Conversation text only in the instance detail file — Prompt, assistant, tool-input, plan and dialog text never reach a home-level line, only an instance's per-process detail file inside a chain or drift report.
decision/default-deny-log-fields · Default-deny for logged fields — A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure. · guard: gate G4 against schemas/diag-line.v1.json
decision/endpoint-and-pinned-bin-never-logged · endpoint and pinned_bin never logged — The endpoint and pinned_bin values are never logged.
decision/fixed-error-messages · Fixed error messages — Error Display texts are fixed, and serde-sourced error chains are mapped to fixed messages at dispatch.
decision/home-level-lines-path-free · Home-level lines path-free — Home-level log lines are path-free: file fields are basenames, panic_location is workspace-relative and source file recording is off; absolute paths appear only in detail files. · why: The OS username inside absolute paths is the only PII in scope.
decision/low-class-metadata-as-typed-fields · Low-class metadata as typed fields — Low-class metadata is logged as typed fields, never inside detail codes.
decision/schema-top-level-unevaluated-properties · Default-deny schema via top-level unevaluatedProperties — The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists. · why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then · guard: contract_diag_schema
decision/secrets-never-logged · Secrets never logged — Signing credentials and R8-stripped CLAUDE* values are never logged.
decision/source-allow-list-logging · Source allow-list for logged fields — Spans skip all arguments and declare fields explicitly, and events go through obs_event! with explicit typed fields only; this source allow-list is the primary PII control. · guard: disallowed-macros; raw_event_hits; G1 bare instrument
decision/detail-upload-synthetic-inputs-only · Detail-file upload only with synthetic inputs — Detail files and the harness capture are uploaded only while every input is synthetic; a non-synthetic source first drops them from the upload paths. · why: every input is synthetic, so detail content carries no real data

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/agent-rows-never-verbatim · Agent rows never logged verbatim — claude agents --json rows are untrusted and never logged verbatim, only parse-rejected counts.
decision/closed-protocol-codes · Closed protocol codes — Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value. · guard: answer_invalid_params_names_its_cause_and_no_param
decision/endpoint-paths-never-logged · Endpoint and pinned binary never logged — The snapshot endpoint and pinned_bin are never logged, only endpoint_kind.
decision/typed-ids-never-in-detail · Ids logged as typed fields — pid, child_pid, agent_session_id and started_at are logged as typed fields, never inside detail codes.
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — thiserror messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
decision/prompt-and-output-never-in-home-logs · Prompt and assistant output never in home logs — Prompt text and assistant output, last_assistant_message included, never appear in a home-level process-log line; they reach an instance detail file only inside a chain or drift report, and only text_bytes is logged.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/obs-code-mutation-tested · Obs code mutation-tested like product code — Obs code is mutation-tested like product code at the epoch boundary and on demand, not on push or pull request; a timed-out or missed obs-code mutant, or unviable outnumbering caught, is red. · guard: test-plan Mutation gate

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/cargo-deny-failure-fails · cargo deny failure is a failure — A cargo deny failure (Tokio, a C crate, veil toggle, banned obs crates) counts as an SLO invariant failure. · guard: cargo deny
decision/correlation-propagation · Correlation propagation — Every channel call logs corr and conn on both sides; a send line from send-issued on carries the send cursor, a wrapper refusal before it the events.ndjson end offset as corr, and a client-side refusal no corr; every dialog line carries dialog_id.
decision/deterministic-emission · Deterministic event emission — Events key on hook events and events.ndjson offsets; nothing is derived from screen content. · except: The readiness gate reads the vt100 screen before a paste and can refuse a send as input-not-ready; a local command's echo confirmation keys on its transcript's stdout tag count.
decision/module-boundary-error-logging · Errors logged once at module boundaries — Every typed error that crosses a crate seam is logged once at WARN or ERROR with a closed detail or error_code; the full chain goes only to the instance detail file.
decision/no-retry-policies · Retry policies forbidden — Retry-once policies are forbidden, for tests and for panics alike. · why: retry policies mask real signals
decision/no-sleep-based-log-waits · No sleep-based log waits — No log-based wait relies on sleeps.
decision/surviving-obs-mutant-fails · Surviving obs mutant is a failure — A surviving cargo-mutants mutant in obs code at a run --mutants (the epoch-boundary audit or on demand) counts as an SLO invariant failure. · except: A missed mutant the host never compiles is not counted as surviving.
decision/error-budget-zero-tolerance · Zero-tolerance error budget — The error budget tolerates zero panic lines in any CI artifact, zero secret-scan hits and zero schema-conformance failures. · except: the fake-agent seam's deliberate panic, as the zero-panics gate counts it
decision/schema-conformance-failure-fails · Schema-conformance failure is a failure — An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/claude-env-and-cookie-never-logged · CLAUDE* values never logged — CLAUDE* values are never logged, at any level, in any file, detail files included.
decision/closed-event-and-corr-on-every-line · Closed event enum on every line, corr where correlated — Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
decision/corr-set-explicitly-by-caller · corr set explicitly by the caller — The caller sets corr explicitly as a typed field on the events that carry one.
decision/default-panic-hook-never-runs · Default panic hook never runs — The default panic hook is never called or allowed to run. · why: Its stderr output breaks hook and the run terminal.
decision/event-values-closed · event values closed — No event value is added without a Decisions Log entry.
decision/explicit-corr-conn-instance · corr, conn and instance passed explicitly — corr, conn and instance are passed as explicit values across threads or tasks, never carried by span context. · why: Spans are not serialized.
decision/explicit-subscriber-writer · Explicit tracing-subscriber writer — with_writer is always set explicitly on the tracing-subscriber. · why: The default tracing-subscriber writer is stdout.
decision/field-allow-list-default-deny · Field allow-list is default-deny — No field is logged unless named in the field catalog.
decision/must-trace-paths-always-traced · Must-trace paths always traced — Spans and events are never skipped on the 6 must-trace paths. · why: The agent loses its ability to debug without them.
decision/no-paths-or-pids-in-detail-codes · No paths or pids in detail codes — detail codes never contain paths or pids.
decision/no-retry-once · No retry-once policies — No retry-once policy is added for panics or flaky telemetry assertions.
decision/no-serde-source-in-error-output · No serde source in printed errors — An anyhow chain that still contains a serde_json or serde_path_to_error source is never printed to stderr, --json, MCP isError or channel error.data.
decision/no-stringified-corr · corr never recorded with ? or % — corr is never recorded with ?corr or %corr. · why: Those stringify numbers.
decision/no-telemetry-from-screen-content · No telemetry from screen content — Telemetry is never derived from screen content; vt100 is used for readiness and modal gate outcomes only.
decision/obs-event-macro-only · obs_event! is the only event macro — tracing's event and level macros are never called directly outside viola_core::obs; events go through obs_event!. · why: Only obs_event! guarantees event, process and instance. · guard: clippy disallowed-macros; the raw-event! grep in lint-probes
decision/one-line-panics · Panics are one line — Multi-line stack traces are never written; panics are logged as one line.
decision/paths-only-in-instance-detail-files · Paths only in instance detail files — Paths appear only in instance detail files.
decision/sensitive-content-detail-file-only · Sensitive content only in the instance detail file — Send text, tool input and last_assistant_message never reach a home-level file; they may reach only the instance-scoped detail-<process>.ndjson, inside a chain or drift report.
decision/transcripts-not-a-contract · Claude Code transcripts are not a contract — Claude Code transcripts are never read as a contract. · except: A local command's stdout tag in the session transcript, counted past the end recorded before the paste, is read as that send's echo confirmation; no transcript byte is kept.
decision/file-sink-only-error-sink · File sink is the only error sink — Local logs are never optional: the file sink is the only error sink and works with every other feature off.
decision/latency-with-gate-statistic · Latency reported with the gate statistic — Latency is never reported without the gate statistic; p50 and p95 are report-only.
decision/low-cardinality-span-names · Low-cardinality span names — Span names never contain instance names, dialog ids or paths.
decision/no-bare-instrument · No bare #[instrument] — Instrumentation always uses #[instrument(skip_all, fields(...))], never a bare #[instrument]. · why: A bare #[instrument] records every argument with Debug and leaks send text, hook.dialog payloads and tool input.
decision/no-hot-path-instrumentation · No hot-path instrumentation — The PTY pump read loop and the vt100 feed get no per-byte or per-chunk span or event; only gate outcomes are logged.
decision/no-info-in-hot-loops · No info logging in hot loops — Nothing is logged at info inside hot loops such as the PTY pump; counts go on the boundary event instead.
decision/no-instrument-err-ret-on-sensitive · No instrument err or ret on sensitive results — #[instrument(err)] and #[instrument(ret)] are never used where the error or return can carry a serde or serde_path_to_error source or user content.
decision/no-pii-in-span-fields · No PII or content in span fields — Span fields never include PII or content. · why: Scrubbing at an exporter stage is too late, and v1 has no exporter stage at all.
decision/no-print-macros · No print macros in product code paths — print!, println!, eprint!, eprintln! and dbg! are never used in the hook, run or mcp code paths of the product crates, and every product member inherits the workspace lints. · why: They bypass the file sink and break the Hook contract, the run terminal rule and MCP framing. · guard: clippy print_stdout, print_stderr and dbg_macro at deny · except: viola-harness, the fake agent and the output modules that own --json, human CLI stderr and the hook decision body.
decision/no-telemetry-on-stdout · No telemetry on stdout — Telemetry is never written to stdout on any role. · why: stdout carries each role's own output and run's terminal carries the child's screen.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/content-only-in-instance-detail-file · Content only in instance-scoped detail files — Content-bearing detail goes only to the instance's detail-<process>.ndjson file; home-level log files are codes-only and carry no user content, token or CLAUDE* value. · why: security allows user-derived content only in instance-scoped diagnostics
decision/direct-chrono-dependency · Direct chrono dependency for timestamps — The root binary takes chrono directly, without default features, for the millisecond UTC timestamp, and keeps it though unmaintained: RUSTSEC-2026-0543 stands in the cargo deny ignore list on the unmaintained-upstream ground, re-read at each epoch boundary; jiff and time are not adopted. · why: tracing-subscriber's chrono feature does not re-export chrono
decision/fixed-error-display · Fixed error Display text — Typed errors have fixed Display text that carries no interpolated content.
decision/obs-event-single-macro · One obs_event macro for every line — Every log line is emitted through the one obs_event macro defined in viola-core, which expands to tracing's event at the caller, so viola-core stays I/O-free and tracing-free.
decision/raw-tracing-ban · Raw tracing macros banned — The tracing level macros are banned by clippy disallowed-macros and raw event calls by a fail-closed grep outside the obs module, so only obs_event emits lines. · why: clippy reports a disallowed macro expanded inside an exported macro at the caller, so no allow inside obs_event can exempt it · guard: clippy disallowed-macros and scripts/lint-probes.sh
decision/diag-schema-matches-event-enum · Diagnostic line schema matches the event enum — A hand-maintained JSON Schema of the process-log line is committed, and its event enum must equal the ObsEvent variants, asserted by a tests-owned check.
decision/instrument-skip-all-everywhere · Spans skip all arguments — Every instrumented function uses skip_all with explicit fields, so no argument is recorded implicitly. · guard: G1
decision/subscriber-only-in-root-bin · tracing-subscriber only in the root binary — tracing-subscriber is a product dependency of the root binary only, without its chrono feature; the product crates depend on tracing alone. · except: a library crate may take it as a dev-dependency for in-memory line capture in its own unit tests

## log-file-location · key of ## 3. Observability Harness Contract
contract/diagnostics-files/home-resolution · diagnostics files · home resolution — The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
contract/diagnostics-files/paths · diagnostics files · process log paths — Process logs are written to <home>/diagnostics/ as run-<name>, hook-<name>, mcp and cli-<name> .ndjson files, and detail to <home>/instances/<name>/diagnostics/detail-<process>.ndjson.

## log-format-json-schema · key of ## 3. Observability Harness Contract
decision/null-as-key-absence · Null written as key absence — A null corr or instance is written as an absent key, readers treat absence and null as equal, and the schema rejects a literal null. · why: tracing has no null field value
contract/corr-by-event/process-and-panic · corr by event · process-start, process-exit, panic — corr is null.
contract/process-log-line/message · the process log line · message — The message is a static literal equal to the event name and never carries interpolated content.
contract/process-log-line/timestamp · the process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
decision/no-multiline-stack-traces · No multi-line stack traces — Every log line is one physical line parseable by jq; a hook panic is caught and logged as one ERROR line with event panic.
decision/obs-aligns-to-tests-log-format · Observability aligns to the tests log format — The log format is bound by the test plan's harness contract; observability may add fields but must not rename them or remove one the product still emits. · why: the harness greps on them
contract/corr-by-event/send · corr by event · send-issued, send-confirmed, send-refused — corr is the send cursor; a wrapper refusal before send-issued carries the target's event-stream end offset, never returned to the caller; wrapper send lines carry conn and rpc_id; the client-side send-refused has corr null.
contract/process-log-line/corr · the process log line · corr — Copied unchanged as a JSON number or string, never renamed and never debug- or display-formatted; required only on the events whose rules always define it.
contract/process-log-line/instance · the process log line · instance — The instance's name, or absent when none resolved.
contract/process-log-line/level · the process log line · level — One of DEBUG, INFO, WARN, ERROR.
contract/process-log-line/process · the process log line · process — One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing cli-<name>.ndjson.
contract/process-log-line/target · the process log line · target — The target is always present.

## logging-stack · key of ## 3. Observability Harness Contract
decision/no-content-in-home-level-lines · No content fields in home-level lines — No home-level diagnostics line carries chain, drift_report, panic_payload or backtrace. · guard: G4 diag-line.v1.json schema
decision/no-span-list-in-lines · No span list in diagnostics lines — Diagnostics lines never carry the list of entered spans. · why: Closes the spans leak path.
contract/obs-event-macro/attached-fields · the obs_event macro · attached fields — Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.
decision/content-records-bypass-subscriber · Content-bearing records bypass the subscriber — Content-bearing records never pass through the tracing subscriber, whose one writer holds only the home-level role file; the detail writer formats each detail line itself and writes it with one whole write.
decision/no-log-crate-bridge · No log-crate bridge — The log-crate bridge (tracing-log, LogTracer) stays off; failures are captured from returned values or handles, and a gap is fixed at the seam or by the researched json-subscriber fallback, never by enabling the bridge. · why: The accepted loss is only third-party log context emitted without a returned error. · guard: pty_exit_is_read_on_the_handle_while_the_output_is_held
decision/third-party-targets-off · Third-party log targets off — Records of third-party targets (rmcp, portable_pty, sysinfo, interprocess) are off in every sink; their failures are captured at viola's seam from the returned results. · why: Their records lack the required fields.
contract/detail-line/fields · the detail line · fields — Each detail line carries timestamp (millisecond UTC format), level, target, message, event, process and instance, then its content fields, and no corr or conn key.
contract/diagnostics-files/detail · the diagnostics files · detail — Content-bearing records (full error chains, drift reports, panic payload and backtrace) go only to <home>/instances/<name>/diagnostics/detail-<process>.ndjson, file 0600 in a 0700 directory, and only when an instance resolves.
contract/diagnostics-line/format · the diagnostics line · format — Each diagnostics line is one structured JSON object on one line, written whole by one write of the complete line on an append-mode handle.
contract/diagnostics-line/role-file-line-size · the diagnostics line · role-file line size — A line in a role file (run, hook, mcp, cli) is codes-only and stays well under 4 KiB.

## otel-sdk-init · key of ## 3. Observability Harness Contract
decision/instance-source-per-role · Instance source fixed per role — run takes its own name argument, never an inherited VIOLA_NAME; hook and mcp take VIOLA_NAME only; cli verbs that act on an instance (send, wait, last, answer, pause, release, link, unlink) take the target argument; list, verify, plugin install and revive --list take the caller's VIOLA_NAME only. · why: A wrapper never writes into another instance's run log.
decision/millisecond-utc-timestamps · Millisecond UTC timestamps — Every diagnostics timestamp is RFC 3339 in UTC with millisecond precision and a Z suffix. · why: The default timer writes microseconds, and the chrono RFC 3339 timer writes +00:00 with automatic precision.
decision/tracing-subscriber-minimal-features · Minimal tracing-subscriber features — tracing-subscriber is taken with default features off and only fmt, json, registry and std; ansi, tracing-log and chrono stay off. · why: The chrono feature only enables ChronoUtc / ChronoLocal, and the millisecond timer uses the root bin's direct chrono dependency.
decision/flat-log-lines-and-one-level-gate · Log lines are flat and Targets is the only level gate — The subscriber flattens event fields to the top level of each line, and the Targets filter is the only configurable level gate; its level comes from the home's diagnostics level and never from the environment. · why: The diagnostics schema and the log readers read event fields as top-level keys, and only the viola crates' targets pass the gate.
decision/home-resolution-order · Home resolution order — The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
decision/lazy-detail-file · Detail file opened lazily — The instance detail file is opened only by the first content-bearing record or by the panic hook, and a failed detail open drops only the detail line, writing nothing to stderr. · why: The error-free hook path keeps exactly one log open.

## service-identity · key of ## 3. Observability Harness Contract
decision/env-vars-not-configuration · Environment variables are not configuration — Environment variables are not a configuration channel: service identity has no env-var source or fallback, and the one CLAUDE_CONFIG_DIR value read by verify and the run and revive start, when the strip keeps that name, locates the claude config file, is never logged and configures nothing of viola.
contract/process-start/os · the process-start line · os — process-start carries field os, the one word linux.
contract/process-start/pid · the process-start line · pid — process-start carries the process id as field pid.
contract/process-start/service-name · the process-start line · service_name — process-start carries the service name as field service_name.
contract/process-start/version · the process-start line · version — process-start carries the product version as field version.
contract/diagnostics-line/instance · the diagnostics line · instance — Every diagnostics line of a process with an instance carries it as field instance; a process without one, such as an mcp server with no instance name, writes no instance key.
contract/diagnostics-line/process · the diagnostics line · process — Every diagnostics line carries the role as field process.

## snapshot-paste-to-ai-integration · key of ## 3. Observability Harness Contract
contract/send-line/unconfirmable · the send line · unconfirmable send — An unconfirmable send logs one send-issued and one send-confirmed{confirmed:false} under the same corr, with no send-refused and no hook side.
contract/send-line/cursor-uniqueness · the send line · cursor corr uniqueness — A send cursor is a byte offset into the target's own events.ndjson, unique per target only; refusals and later sends can share it, so one send is identified by target instance, corr, conn and rpc_id.
contract/send-line/wrapper-side · the send line · wrapper side — send-* lines whose corr is a send cursor are emitted only by the target wrapper; only its send-refused line carries side:"wrapper", its send-issued and send-confirmed lines carry no side, and a client-side send-refused carries side:"client" and no corr.

## trace-context-propagation · key of ## 3. Observability Harness Contract
decision/explicit-correlation-values · Correlation values passed explicitly — corr, conn and instance are passed as explicit values into each thread or task and set on each event, never recovered from span context. · why: Spans are not serialized.
decision/span-carried-into-tasks-and-threads · Span not carried into tasks and threads — Spawned async tasks and std thread closures take no span from their spawner: spans never reach a log line, and correlation travels as explicit typed fields.
contract/diagnostics-line/corr-on-send-lines · the diagnostics line · corr on send lines — On send lines, corr carries the send cursor; a send refused before send-issued carries the target's events.ndjson end offset at refusal, a log join key only, never returned to the caller.
contract/send-line/conn-rpc-id · the send line · conn and rpc_id — Every wrapper send-* line carries conn (or srv_conn) and rpc_id, and (conn, rpc_id) equals (conn, corr) on that call's channel-request and channel-response on both sides.

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
