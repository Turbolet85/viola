# obs-plan · the rows that bind 2026-11-18-the-2-1-325-recorded-fixture-set — picked 258 of 1028 (141 by every draw) · rule any · pin 2816

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/agents-probe-output-untrusted · Agents probe output untrusted — The output rows of the claude agents --json probe are untrusted.
decision/agents-probe-parse-failure-unknown · Agents probe parse failure becomes unknown — A parse failure of the claude agents --json probe becomes status "unknown" plus a parse-rejected counter; an unparsed --version answer leaves cli_version absent and the version unverified.
decision/child-exit-from-process-handle · Child exit from the process handle — The wrapped child's exit and its process-exit event come from the wait on the process handle, never from reader EOF.
decision/claude-fixtures-scrub-required · Claude fixtures are scrub-required — Build and CI infrastructure is not instrumented, and the claude fixtures are scrub-required data, never a telemetry source. · why: Not product runtime.
decision/claude-internals-not-a-contract · Wrapped claude internals are not a contract — The wrapped claude CLI is a third-party binary run unmodified and instrumented at its boundary only (spawn and exit, paste writes, the vt100 readiness-gate outcome); its internal state and transcripts are never read as a contract. · except: A send's echo confirmation counts its stdout tag in the transcript, keeping no byte; verify writes and removes its temp folder's trust record; a verified start sets its folder's trust flag only.
decision/distinct-detail-code-per-exit-cause · Distinct detail code per exit cause — Each cause behind exit 21 and exit 1 gets a distinct detail code with no path or pid. · except: A strict-modes check refused before any log opens writes no line; its cause is the fixed stderr pair or the --json document.
decision/drift-reports-diagnostics-only · Drift reports go to diagnostics only — Drift reports of viola-agent-claude go to diagnostics/ only and nowhere else. · why: The outputs of viola-agent-claude are the most sensitive in the product.
decision/drift-reports-to-diagnostics · Drift reports go to diagnostics only — Hook drift reports are written to `diagnostics/` only.
decision/parse-failures-unknown-counter · Parse failures surface as unknown plus a counter — A child-output parse failure surfaces as `unknown` plus a counter. · except: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown.
decision/pty-byte-stream-never-logged · PTY byte stream never logged — The PTY byte stream is never logged; the viola-pty seam is instrumentable, and the openpty OS API behind it is boundary-only. · why: The PTY byte stream is user content.
decision/redaction-not-disableable · Redaction cannot be disabled — No verbosity control, env, flag or config may disable or bypass redaction.
decision/refusal-telemetry · Refusal telemetry for filesystem refusals — Pinned-exe hash mismatches and ignored symlinks produce refusal telemetry. · except: The transcript echo read and the verify and start trust-record writes refuse a symlinked, non-regular or foreign file with no refusal line; the send's lines, stamps or trust-record word show it.
decision/strict-modes-failure-lines · Where a strict-modes failure is logged — A strict-modes failure has a log line only on the `ledger-stamps` read in the `run` version gate and on `verify`'s own `process-exit` for a refused stamp write.
decision/stripped-claude-env-never-logged · R8-stripped CLAUDE values never logged — R8-stripped CLAUDE* environment values are never logged anywhere. · why: The viola-agent-claude outputs are the most sensitive in the product.
decision/stripped-env-never-logged · Stripped CLAUDE env values never logged — No R8-stripped `CLAUDE*` value appears in any sink or fixture; the strip is logged as a count or variable names only.
contract/diagnostics-files/hook · the diagnostics files · hook — Role hook writes <home>/diagnostics/hook-<name>.ndjson, shared by concurrent hook processes; a write failure is swallowed.
contract/hook-log-file/path · the viola hook log file · path — viola hook logs only to a file sink at diagnostics/hook-<name>.ndjson.
contract/process-exit-detail/strict-modes-failed · process-exit detail codes · strict-modes-failed — Carried by `verify`'s `process-exit` for a refused stamp write, with exit 1.
contract/viola-mcp-tools/list · viola-mcp stdio server tools · list — The viola-mcp stdio server offers a list tool.
decision/cli-file-sink-only · Short-lived CLI logs to a file sink only — Short-lived CLI verbs log only to a JSON-per-line file sink in diagnostics when an instance resolves, never to stdout. · why: Stdout carries the --json results.
decision/exit-1-detail-codes · Distinct detail codes for exit-1 causes — On the `viola run` start sequence to child spawn, each exit-1 cause logs a distinct `detail` code with no path or pid in it.
decision/hook-fail-open-silent · Hook fails open silently — Hook fail-open paths log only to `hook-<name>.ndjson` and exit 0 with stderr empty.
decision/hook-silent-no-op-without-instance · Hook silent no-op without an instance — Without VIOLA_NAME, on an instance the instance check refuses, or on a VIOLA_DIR naming an instance directory that does not exist, viola hook is a silent no-op: exit 0, no file opened, no line, no frame.
decision/hook-stdin-correlation · Hook stdin correlated by instance and dialog — Hook stdin from Claude Code carries no inbound trace context, so it is correlated by `instance` and `dialog_id`.
decision/mcp-stdout-frames-only · MCP stdout carries frames only — No telemetry from the viola-mcp server may go to stdout. · why: Stdout is reserved for MCP frames.
decision/run-silent-terminal · viola run silent on the terminal — viola run writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns.
decision/send-text-confined · Send text confined to instance diagnostics — `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
decision/send-text-never-logged · Send text never logged — Sent text never appears on stderr, in --json output, in MCP results or in any process-log field outside instances/<name>/diagnostics/.
decision/stderr-fixed-strings · Fixed strings on stderr — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
contract/diagnostics-files/cli · the diagnostics files · cli — Role cli writes <home>/diagnostics/cli-<name>.ndjson, only when an instance resolves.

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/agents-cli-instrumented-at-boundary · claude agents CLI instrumented at spawn, exit and parse — The external `claude agents --json` CLI is instrumented only at its spawn, its exit and its parse outcome. · why: An external CLI.
decision/child-exit-from-handle-wait · Child exit from the process handle — The child's process-exit comes from the wait on its process handle, never from reader end-of-file. · why: Child exit is detected on the process handle.
decision/claude-boundary-only-instrumentation · Boundary-only instrumentation of claude — The hosted claude binary is instrumented only at its boundary: spawn and exit, paste writes, the vt100 readiness-gate outcome, and the start's one trust-record line for the trust record it writes in the claude config. · why: claude is a third-party binary run unmodified.
decision/detail-codes-no-paths · Detail codes carry no path or pid — No path or pid appears in a detail code.
decision/deterministic-event-emission · Deterministic event emission — Events are emitted deterministically, keyed on hook events and events.ndjson offsets. · except: A local command's echo confirmation keys on the stdout tag count in its session transcript.
decision/drift-reports-detail-only · Drift reports in the detail file only — A full serde drift report goes only to the instance-scoped detail file, as its drift_report field.
decision/exit-code-log-field · Exit code as a log field — The exit code (0, 1, 2, 10, 12–14, 20, 21), the machine-readable outcome, is carried as a log field; 11 is retired and never reused.
decision/fixtures-not-telemetry · Recorded fixtures are no telemetry source — Recorded fixtures are scrub-required data, not a telemetry source, and are excluded from the observability plan. · why: Not product runtime.
decision/hook-fail-open-logged-to-hook-file · Hook fail-open logged to the hook file — Hook fail-open paths (oversize stdin, malformed JSON, channel failure, server-verify mismatch, refused socket directory) are logged to hook-<name>.ndjson only; a missing VIOLA_NAME writes no line.
decision/messaging-secrets-never-logged · Messaging secrets never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every R8-stripped CLAUDE* value are never logged in any sink, detail files and fixtures included. · guard: secret-scan test
decision/no-secrets-in-log-lines · No secrets in log lines — No log line may contain any stripped CLAUDE* value. · guard: secret-scan test
decision/no-telemetry-from-screen · No telemetry from screen content — No telemetry is derived from screen content; vt100 serves gate outcomes only.
decision/parse-failures-surface-as-unknown · Parse failures surface as unknown — A parse failure surfaces as the value unknown plus a counter. · except: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown.
decision/per-parser-parse-failure-counts · Per-parser parse-failure counts — Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, the hyprctl replies, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead.
decision/r8-strip-logged-as-names-only · R8 strip logged as names only — The R8 environment strip is logged as a count and lists of stripped and kept variable names, and environment maps never pass to a span or event; no value is read, bar the kept CLAUDE_CONFIG_DIR value that viola verify and the run and revive start read, and no value is logged.
decision/serde-chain-fixed-message · Serde error chains become a fixed message on stderr — An error chain that contains a serde source becomes a fixed message before it reaches stderr.
decision/serde-errors-become-fixed-message · Serde errors become a fixed message — An error chain holding a serde source is replaced by a fixed message before it reaches stderr.
decision/strict-modes-refusal-writes-no-line · Strict-modes refusal writes no line — A strict-modes check refused before any log opens ends the process with nothing under the home opened and no log line; its cause is carried only by the fixed stderr pair or the --json document. · why: Ratified as the answer to whether a refused strict-modes check logs. · except: The ledger-stamps read in run's version gate and a refused update_stamps in viola verify each leave a line.
decision/stripped-claude-values-never-logged · Stripped CLAUDE values never logged — R8-stripped CLAUDE* values are never logged anywhere. · why: Its outputs are the most sensitive in the product.
contract/process-log/child-and-probe-lifecycle · Process log · child and probe lifecycle — process-start and process-exit are logged for the claude child and for the claude agents --json and --version probes.
contract/run-log/cli-verified · run log line · cli_verified — A required field on run's log lines.
contract/run-log/cli-version · run log line · cli_version — An optional field on run's claude-child process-start line; absent when the CLI version is unreadable.
contract/send-path-log-events/event-order · Send path log events · event order — A send logs channel-request, then send-issued, then hook-invoked (UserPromptSubmit, or SessionStart for a confirmed /clear or /compact; none awaited for an unconfirmable send or a local command confirmed by its transcript echo), then send-confirmed or send-refused, then channel-response.
contract/viola-verify/home-tree-refused · viola verify · refused home-tree check — A refused home-tree check exits 1 with the home pair, opens no log and writes no line; a refused update_stamps exits 1 with the home pair and, when an instance is named, ends the opened log with a process-exit line carrying detail strict-modes-failed.
decision/claude-exit-on-process-handle · Claude exit detected on the process handle — The exit of the hosted claude process is detected on the process handle, never on EOF.
decision/fault-telemetry-per-class · Fault telemetry per class — Each fault class emits its own telemetry: a healed torn ndjson line with file and offset, a corrupt or unsupported snapshot recovered by replay, a child exit seen without EOF, an endpoint vanished during wait, and a stale heartbeat against a live pid.
decision/fixed-stderr-messages · Fixed stderr refusal and fault lines — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/home-level-mcp-log-no-user-content · Home-level MCP log holds no user content — An mcp.ndjson at home level never carries user content.
decision/hook-lines-required · Hook invocation and decision lines required — Every hook process in a trusted wrapped session writes its hook-invoked and hook-decision lines, bar a PreToolUse for a tool outside the dialog matcher; a hook outside such a session or on a --capture probe writes neither.
decision/hook-mcp-instrumented-at-boundary · Hook and MCP instrumented at the boundary — Hook and MCP traffic is instrumented only at viola's own boundary: hook-invoked and hook-decision, MCP tool-call handling, and the viola mcp stdin cut. · why: The caller is Claude Code, so viola sees only the inbound stdin payload and its own response, and no trace context arrives from Claude Code.
decision/instrument-call-sites-not-os-internals · OS operations instrumented at viola's call sites — OS-level operations (bind, connect, lock, rename, kill fallback, the server check's peer reads, the strict-modes stat reads, the socket directory check, the hyprctl spawn) are instrumented at viola's own call sites, not inside the OS.
decision/log-file-basenames-follow-tests · Log file names follow tests — Process-log files are <home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson, as tests require. · why: Tests is upstream; arch and security place diagnostics under the instance directory instead.
decision/mcp-log-sink · MCP log sink — viola mcp always logs to <home>/diagnostics/mcp.ndjson once the home and its diagnostics/ pass strict-modes; a file that cannot be opened exits 1 printing nothing, with no stderr fallback.
decision/mcp-no-telemetry-on-stdout · No telemetry on the MCP server's stdout — The MCP server sends no telemetry to stdout. · why: stdout is reserved for MCP frames.
decision/no-process-log-without-instance · No process log without an instance — A short-lived CLI verb run with no resolved instance writes no process-log file; its human stderr carries only fixed-message lines, and its outcome is the exit code and stdout.
decision/process-init-order · Process logging init order — Each process installs its panic hook, parses arguments, resolves home and instance and runs its strict-modes check, opens its diagnostics file append-mode 0600 in a 0700 directory, installs the subscriber, then emits process-start. · why: For run this completes before the start sequence so the collision check and version gate are logged; for mcp, before the stdio transport starts.
decision/process-logs-file-sink-only · Process logs to a file sink only — Process logs are written only as JSON lines to one file per process role under diagnostics/; there is no network exporter and no stdout sink. · why: viola makes no outbound network calls, and stdout carries --json results.
decision/single-line-log-records · Single-line log records — Every log record is one JSON line parseable by jq; never a multi-line stack trace.
decision/structured-json-process-logs · Structured JSON process logs — Every process role (run, hook, mcp, cli) writes structured JSON process logs under diagnostics/, which harness logs merges with events.ndjson.
decision/subscriber-explicit-writer · Subscriber installed with an explicit writer — The tracing subscriber is always installed with an explicit writer, never stdout. · why: tracing-subscriber fmt() writes to stdout unless a writer is configured, and stdout carries MCP frames.
contract/dialog-log-line/unverified-cli · Dialog log line · unverified-cli — The detail unverified-cli goes with exit 12.
contract/exit-1-detail/cwd-missing · Exit 1 detail codes · cwd-missing — cwd-missing is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
contract/exit-1-detail/no-session · Exit 1 detail codes · no-session — no-session is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
contract/exit-1-detail/session-live · Exit 1 detail codes · session-live — session-live is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
contract/hook-stdout/capture-arms · Hook stdout · --capture arms — The hidden --capture arm writes no log sink and prints nothing, except --answers' probe decision body.
contract/process-log-corr/process-exit · Process log corr by event · process-exit — corr is null.
contract/run-exit-1-detail/cwd-missing · run exit-1 detail codes · cwd-missing — On a start made by viola revive, a distinct detail code.
contract/run-exit-1-detail/no-session · run exit-1 detail codes · no-session — On a start made by viola revive, a distinct detail code.
contract/run-exit-1-detail/session-live · run exit-1 detail codes · session-live — On a start made by viola revive, a distinct detail code.
contract/run-start-events/order · run start events in events.ndjson · order — A run start writes wheel{cause:"start"} before session-start; on a start whose folder-trust keys pass through the wrapper, wheel{human-input} also precedes session-start, and a later human key adds no wheel line, the wheel being already human.

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/no-event-payloads-in-process-logs · No event payloads in process logs — Obs never copies event payloads into process logs.
decision/no-human-gated-log-analysis · No human-review-gated log analysis — Every log verdict is a jq or harness assertion that sets an exit code, never a human review.
contract/process-log-line/duration-ms · the process log line · duration_ms — Durations are duration_ms as an integer.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
contract/strict-modes-refusal-exit/1 · strict-modes refusal exit codes · 1 — run, revive's start and verify exit 1 on a strict-modes refusal; revive under --json prints the strict-modes-failed document.
decision/pre-open-strict-modes-refusal-silent · Strict-modes refusal before a log opens writes no line — A strict-modes refusal made before any process log opens writes no line and carries no logged detail code; its machine-readable form is the exit code and, under --json, its closed document. · except: a refused socket directory always logs detail socket-dir-refused, except at MCP's frame tool, which writes no line

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/codes-only-process-logs · Codes-only process logs, content in detail files — Every role's codes-only process log goes to its home-level role file under diagnostics, and content-bearing detail goes only to the instance's per-role detail file.
contract/mcp-output/stdout · mcp output streams · stdout — mcp writes only MCP frames to stdout.
contract/run-terminal-output/start-refusal · run terminal output · start refusal — A start refusal before any child is spawned writes exactly two fixed stderr lines with no path or pid: the home pair for a home-level refusal, the instance pair for an instance-level one.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/browser-tools-prompt-reading-uninstrumented · Browser-tools prompt reading uninstrumented — The reading of the CLI's browser-tools prompt as a modal adds no log line, span, event or detail code of its own; verify's reading of it reaches only the ledger stamp and verify's stdout.
decision/capture-arms-uninstrumented · Hidden capture arms uninstrumented — The hidden capture arm of verify / plugin install is uninstrumented by design.
decision/child-settings-never-logged · Child settings argument never logged — The claude child's --settings argument appears in no log line or span.
decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.
decision/compact-confirm-window · Compact confirmation window — A /compact send waits 40 s for its compacted-session post-condition, while every other confirmed send waits 10 s. · provisional
decision/detail-codes-only · detail is a closed code — The detail field is a closed kebab-case code catalog and never carries a path or pid.
decision/dialog-content-never-logged · Dialog content never logged — Dialog questions, answers, plan text and tool input never appear in a home-level process-log line; dialog events carry codes only, and the text reaches an instance detail file only inside a chain or drift report.
decision/env-names-only · Environment names only, never values — The strip and every log record environment variable names only, never values, and no env map is passed to a span or event. · guard: E2
decision/ledger-contents-never-logged · Capability ledger path and stamps never logged — The capability ledger's path and its stamp contents never appear in a log line.
decision/no-assistant-message-in-logs · No assistant message or result body in logs — last_assistant_message never appears in a log, and no result body reaches a log line.
decision/trust-record-write-uninstrumented · Trust-record write and removal uninstrumented — viola verify's trust-record write and removal emit no log line and no span, the outcome riding only the stamps; a start's trust step opens no span and writes only its one trust-record line; no path, key or value of the claude config reaches any line, stamp or summary.
decision/unreadable-ledger-is-unverified · Unreadable ledger counts as unverified — A missing stamp, or an unreadable, malformed or strict-modes-refused capability ledger, counts as unverified (cli_verified false).
decision/verify-lines-redacted · verify lines carry no content — No verify or plugin install line carries row text, a prompt, a session id or a path.
decision/verify-logs-only-with-instance · verify and plugin install log only inside an instance — With VIOLA_NAME set, verify and plugin install log process-start / process-exit to cli-<name>.ndjson; from a plain terminal with no instance they write no process-log file.
contract/claude-child-process-start/env-kept · process-start line of the claude child · env_kept — Reports the variables kept after the R8 environment strip on process-start{subject:"claude-child"} by names only, never values.
contract/claude-child-process-start/env-stripped-count · process-start line of the claude child · env_stripped_count — Reports the R8 environment strip on process-start{subject:"claude-child"}; never an environment value.
contract/claude-child-process-start/env-stripped-known · process-start line of the claude child · env_stripped_known — Reports the R8 environment strip on process-start{subject:"claude-child"} by variable names only, never values.
contract/viola-verify-refusal/home-tree-refused · viola verify strict-modes refusal · home tree refused — Before logging, verify holds the home, ledger/, bin/ and plugin/ to the strict-modes check; a refused tree exits 1 with the home pair and writes no line.
contract/viola-verify-refusal/stamp-write-refused · viola verify strict-modes refusal · stamp write refused — A ledger/stamps.json the stamp write refuses prints the home pair and exits 1 through verify's own process-exit line.
decision/channel-lines-no-params-or-results · Channel lines carry no params or results — Channel log lines never record request params or result bodies.
decision/no-endpoint-path-or-env-in-spans · No endpoint, binary path or env values in spans — A span never carries the endpoint name, the pinned_bin path or environment values.
decision/sent-text-never-logged · Sent text never logged — A sent message's text is never logged; only its size in text_bytes is.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/claude-env-strip · CLAUDE environment strip — The child loses every inherited CLAUDE* name, matched exactly, except the persistent set kept in the claude_env_keep config key.
decision/hidden-capture-arms-uninstrumented · Hidden hook capture arms uninstrumented — The hidden hook capture arm used by verify's probes reads no VIOLA_* variable, does no obs init, emits no hook lines, keeps stderr empty and always exits 0. · why: uninstrumented by design, as ratified by the founder
decision/home-tree-check-before-start-log · Home-tree check before the start's log — The start of run and of viola revive checks the home tree (the home, instance set, ledger, bin, plugin, pinned exe and plugin hooks) before its log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
decision/no-obs-gate-on-verify-lines · No obs gate depends on verify lines — No observability gate depends on verify's own log lines; its home-level lines fall under the same panic, schema and secret-scan gates as any other.
decision/pinned-copy-never-replaced · Pinned copy placed, never replaced — An absent pinned copy is placed, never replaced; a start that loses the placement re-hashes the copy that stands, reusing an equal one and refusing a different one as pinned-hash-mismatch with the file left as found.
decision/real-cli-verify-never-in-ci · Real-CLI verify never runs in CI — In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
decision/refused-home-tree-writes-no-line · Refused home tree writes no line — verify and plugin install check the home tree before their log opens, and a refused tree opens no log and writes no line.
decision/unreadable-ledger-means-unverified · Unreadable capability ledger means an unverified CLI — A capability ledger that is missing a stamp, unreadable, malformed or refused by the strict-modes check is treated as an unverified CLI (cli_verified false).
decision/verify-lines-carry-no-content · verify log lines carry no content — No verify log line carries row text, a prompt, a session id, the probe command or a path.
decision/verify-sole-stamps-writer · verify is the only stamps writer — verify is the only writer of the capability ledger's stamps file.
contract/claude-child-process-start/env-fields · process-start claude-child · env fields — env_stripped_count, env_stripped_known (every stripped name, comma-joined) and env_kept (every kept persistent-set name, comma-joined).
contract/run-log-parse-rejected/ledger-stamps · the run process log parse-rejected line · parser ledger-stamps — An unreadable, malformed or strict-modes-refused capability ledger gives one parse-rejected line with parser "ledger-stamps", detail "unreadable", "malformed" or "strict-modes-failed", and a count, in the run log.
contract/run-process-log/version-probe · run process log · version-probe — process-start and process-exit{subject:"version-probe", child_exit_status, duration_ms}.
contract/verify-exit-codes/0 · verify exit codes · 0 — Every relied ledger row counted: each passed, or the browser-tools-prompt row read absent.
contract/verify-exit-codes/1 · verify exit codes · 1 — A row failed, verify refused, or verify was stopped.
contract/verify-process-exit/interrupted · verify process-exit line · detail interrupted — A verify stopped by SIGINT or SIGTERM starts no further spawn, closes every pair it opened and ends with process-exit subject self, exit_code 1, detail "interrupted".
contract/verify-process-exit/strict-modes-failed · verify process-exit line · detail strict-modes-failed — A stamps store refused by its strict-modes check, under a parsed VIOLA_NAME, ends with process-exit carrying detail "strict-modes-failed" and the home pair on stderr.
contract/verify-spawn-subject/agents-probe · verify child spawn subjects · agents-probe — Run B's single agents --json read is logged as one process-start / process-exit pair with subject agents-probe.
contract/verify-spawn-subject/verify-probe · verify child spawn subjects · verify-probe — The print-mode probe is logged as one process-start / process-exit pair with subject verify-probe.
contract/verify-spawn-subject/verify-pty-probe · verify child spawn subjects · verify-pty-probe — Each of verify's six interactive PTY runs is logged as one process-start / process-exit pair with subject verify-pty-probe.
contract/verify-spawn-subject/version-probe · verify child spawn subjects · version-probe — The --version read of the agent CLI is logged as one process-start / process-exit pair with subject version-probe.
contract/verify-stdout/step-line · verify stdout · step line — One "[NN/MM] <row id> <row words>  pass|absent|fail" line is printed for each relied ledger row, MM being the relied row count; absent is the browser-tools-prompt row's word when no run met the prompt.
contract/verify-stdout/summary-line · verify stdout · summary line — The last stdout line is "stamped <version>  N pass  N fail", the two counts summing to MM; the pass count holds every row that counts, an absent browser-tools-prompt row among them.
decision/error-chain-in-detail-log-only · Full error chain only in the cli detail log — The full anyhow error chain of a cli verb goes to the cli detail log only.
decision/expired-dialog-renders-for-human · Expired dialog returns to the human — At the hold's end the reply stays null, the hook writes hook-decision with decision_emitted false, exits 0 with empty stdout, and the dialog renders for the human.
decision/hook-capture-arms-unlogged · Hidden capture arms write no lines — The hidden capture arm of viola hook (hook <event> --capture) initialises no observability and writes no lines.
decision/no-dialog-content-in-logs · Dialog content never logged — The question, the answer and the tool input never appear in a log line.
decision/no-last-assistant-message · last_assistant_message never logged — last_assistant_message never appears in a log line.
decision/no-params-in-channel-lines · No params or result bodies in channel lines — channel-request and channel-response lines never carry params or result bodies.
decision/no-paths-in-run-spans · No endpoint, path or env values on start spans — Start spans never carry the endpoint name, the pinned binary path or environment values.
decision/no-text-in-telemetry · Sent text never in telemetry — The sent text never appears in a span or log line; only its length does.
decision/portable-pty-log-target-off · portable-pty log target OFF — The log target of portable-pty is OFF in the sink, and PTY failures are captured at the pty seam instead.
decision/refused-check-writes-no-line · A refused check opens no process log — A cli verb whose strict-modes, tree or instance check refuses (exit 21) opens no process log and writes no line.
decision/transcript-echo-read-uninstrumented · Transcript echo read uninstrumented — The send's transcript echo read, the end offset before the paste and the tag count after it, opens no span and writes no line or span field of its own; its result shows only as the send-confirmed line inside run.confirm_window.
contract/diag-line-schema/os · diag-line v1 schema · os — os is closed to linux.
contract/diag-line-schema/pty-backend · diag-line v1 schema · pty_backend — pty_backend is any string.
contract/revive-process-log/agents-probe · viola revive run process log · agents-probe — The session-live reading writes one process-start and process-exit{subject:"agents-probe"} pair, plus one parse-rejected{parser:"claude-agents-json"} on an oversize or malformed answer.
contract/run-process-log/common-fields · run process log · common fields — Every run line carries the common fields with process="run" and corr null.
contract/run-process-log/process-start-claude-child · run process log · process-start claude-child — On success, process-start{subject:"claude-child", child_pid, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept}.
contract/run-span-attributes/run-version-gate · viola run start span attributes · run.version_gate — run.version_gate carries cli_version and cli_verified.
contract/verify-stop-stderr/unable-hint-pair · verify stderr on a stop · the stop pair — A stopped verify prints exactly "unable: verify was stopped before it finished" and "hint: run viola verify again to stamp this CLI" on stderr, with empty stdout and no stamp written.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/content-detail-in-instance-file · Content-bearing detail in the instance detail file — Content-bearing detail goes only to the per-process detail file inside the instance's diagnostics directory.
decision/file-only-per-role-log-sinks · File-only log sinks per role — Diagnostics are written to files only, one sink per role, under the home's diagnostics directory.
contract/child-spawn-log/subject · child and shell spawn log events · subject — process-start and process-exit lines for child spawns carry a subject from a closed set.
decision/json-to-file-only-log-mode · JSON-to-file as the only log mode — JSON to file is the only log mode: there is no agent-mode flag and no stdout pretty sink on any role.
contract/log-levels/error · log levels · error — Used for a panic, a process-exit that carries a detail, and a channel-response with error_code -32603.
contract/log-levels/info · log levels · info — INFO carries all other boundary, lifecycle and state-transition events, including refusals, which are normal product outcomes.
contract/log-levels/warn · log levels · warn — WARN carries state-recovered; parse-rejected; hook-decision with a fail-open detail; process-exit with exit 21; channel-response with -32600 or -32602.
contract/pty-seam-log/kill-fallback · PTY seam log events · kill-fallback — An exit through the kill fallback is logged with exit_source:"kill-fallback".
contract/start-trust-log/trust-record · the start's trust-record log line · trust-record — Each wrapped start of viola run and viola revive on a verified CLI writes exactly one INFO trust-record line to the run role file, its detail one of a closed set of 16 start words, carrying no path, project key or config value; the word never refuses the start.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
contract/diag-detail/pinned-hash-mismatch · diagnostic detail words · pinned-hash-mismatch — A filesystem refusal word written on process-exit with exit 1.
contract/diag-detail/strict-modes-failed · diagnostic detail words · strict-modes-failed — Written on the process-exit exit 1 of viola verify under a parsed VIOLA_NAME and on parse-rejected ledger-stamps from the version gate of run, at warn, with no path; every other refused strict-modes check writes no log line, its exit code or JSON document its only record.
contract/diag-line-fields/parse-rejected · diagnostic line additive fields · parse-rejected — parser (claude-agents-json, vt100-feed, config-json, ledger-stamps, hook-stdin, channel-frame, mcp-stdin, state-entry, hyprctl-json), detail, count.
contract/diag-line-fields/process-exit · diagnostic line additive fields · process-exit — subject, exit_code (self), child_exit_status, exit_source (handle-wait or kill-fallback), detail, during (connect or call), duration_ms.
contract/diag-line-fields/process-start · diagnostic line additive fields · process-start — subject (self, claude-child, version-probe, verify-probe, verify-pty-probe, agents-probe, hyprctl-probe), service_name, version, os, pid, child_pid, endpoint_kind, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept (names only).
contract/diag-subject/agents-probe · diag-line subject enum · agents-probe — Subject of the process-start and process-exit pair of the claude agents read, written by viola list, viola revive's session-live reading, verify's Run B and the list tool of viola mcp into their role files.
contract/diag-subject/verify-probe · diag-line subject enum · verify-probe — Subject of the process-start and process-exit pair of the print-mode probe of viola verify.
contract/diag-subject/verify-pty-probe · diag-line subject enum · verify-pty-probe — Subject of the process-start and process-exit pair of each of the six interactive PTY runs of viola verify, Run A to Run F.
contract/diag-subject/version-probe · diag-line subject enum · version-probe — Subject of the process-start and process-exit pair of a version probe spawn.
contract/parse-rejected-detail/strict-modes-failed · parse-rejected detail · strict-modes-failed — ledger-stamps: run's version gate refused the stamps file on the strict-modes check, and the read counts as unverified.
contract/parse-rejected-detail/unknown-value · parse-rejected detail · unknown-value — hook-stdin: a hook stdin field held a value outside its closed set, first the StopFailure error word; the line carries the code and count only.
contract/parse-rejected-detail/unreadable · parse-rejected detail · unreadable — config-json, ledger-stamps.
contract/parse-rejected/hook-stdin-unknown-value · parse-rejected diagnostic line · parser hook-stdin, detail unknown-value — A StopFailure error word outside the CLI's 13 writes one WARN parse-rejected line with parser hook-stdin, detail unknown-value and count 1, carrying the code and count only, and the event is still delivered as an api-error turn end; a known word writes no line.
contract/process-exit-detail/interrupted · process-exit detail · interrupted — A viola verify stopped by SIGINT or SIGTERM before its rows are checked ends with one process-exit exit 1 detail interrupted, after every spawn pair it opened is closed.
contract/process-exit-detail/verify-refusal · process-exit detail · viola verify refusal — Under a parsed VIOLA_NAME, verify writes its refusal code: program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture or interrupted; a failing row carries no code.
contract/verify-process-exit/child-exit-status-and-duration · viola verify spawn process-exit lines · child_exit_status and duration_ms — Each of the nine verify spawn pairs' process-exit carries child_exit_status and duration_ms, including a child of an interactive PTY run killed at a Ctrl-C or TERM stop; a stopped run writes only the pairs it started.
contract/diag-line-fields/trust-record · diagnostic line additive fields · trust-record — A fixed message and a required detail drawn from the 16 start words; trust-record stands in the event enum of both the diag-line and the detail-line schema.
contract/diag-subject/self · diag-line subject enum · self — One value of the closed subject enum of the diag-line schema.
contract/hook-decision-detail/fail-open-codes · hook-decision detail · fail-open codes — oversize-stdin, malformed-json, channel-unreachable, server-verify-failed, socket-dir-refused, deadline; no hook arm writes strict-modes-failed or unverified-cli.
contract/parse-rejected-detail/malformed · parse-rejected detail · malformed — Any parser.
contract/parse-rejected-detail/oversize · parse-rejected detail · oversize — channel-frame beyond MAX_FRAME, hook-stdin, mcp-stdin past MAX_FRAME, vt100-feed when the bounded queue drops a copy (one line per poisoning episode), claude-agents-json past MAX_FRAME, hyprctl-json past MAX_FRAME.

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/boundary-errors-end-in-logged-line · Crate-seam errors end in a logged line — Every typed error that crosses a crate seam ends in a logged process-exit detail, channel-response error, hook-decision detail or send-refused line, carrying codes only at WARN or ERROR.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/claude-agents-rows-never-logged · claude agents rows never logged verbatim — claude agents --json rows are never logged verbatim, only parse-rejected counts.
decision/content-only-in-detail-file · Content-bearing records only in the detail file — Content-bearing records go only to the instance-scoped detail-<process>.ndjson file.
decision/conversation-text-only-in-detail-file · Conversation text only in the instance detail file — Prompt, assistant, tool-input, plan and dialog text never reach a home-level line, only an instance's per-process detail file inside a chain or drift report.
decision/detail-upload-synthetic-inputs-only · Detail-file upload only with synthetic inputs — Detail files and the harness capture are uploaded only while every input is synthetic; a non-synthetic source first drops them from the upload paths. · why: every input is synthetic, so detail content carries no real data
decision/home-level-lines-path-free · Home-level lines path-free — Home-level log lines are path-free: file fields are basenames, panic_location is workspace-relative and source file recording is off; absolute paths appear only in detail files. · why: The OS username inside absolute paths is the only PII in scope.
decision/local-run-outputs-never-uploaded · Local run outputs never uploaded — mutants.out, the run archive and the local viola-harness pre-push document are never uploaded.
decision/secrets-never-logged · Secrets never logged — Signing credentials and R8-stripped CLAUDE* values are never logged.
contract/detail-line/drift-report · the detail-file line · drift_report — One of exactly four content fields a detail-file line may carry: the drift report.
decision/content-canary-never-home-level · Content canary never in home-level files — The content canary never appears in a home-level diagnostics ndjson file.
decision/detail-schema-self-contained · Separate self-contained detail-line schema — Detail-file lines are validated by their own schema with a top-level unevaluatedProperties false, which inlines the event enum and uses no cross-file reference.
decision/endpoint-and-pinned-bin-never-logged · endpoint and pinned_bin never logged — The endpoint and pinned_bin values are never logged.
decision/low-class-metadata-as-typed-fields · Low-class metadata as typed fields — Low-class metadata is logged as typed fields, never inside detail codes.
decision/no-regex-scrubbers · No regex scrubbers — Sensitive data is never scrubbed by regular expressions. · why: viola's sensitive data is known by type
decision/payload-types-derive-redact · Type-level redaction of payload types — Every channel, hook and MCP payload type that carries content derives veil Redact, so Debug never reveals content; a payload type with no content field derives none.
decision/veil-redact-second-layer · veil Redact as the second redaction layer — veil's derive Redact is the second redaction layer, so a stray value capture prints [REDACTED] instead of content.
contract/detail-line/event · the detail-file line · event — Required on every detail-file line, as on home-level lines.
contract/detail-line/instance · the detail-file line · instance — Required on every detail-file line, as on home-level lines.
contract/detail-line/level · the detail-file line · level — Required on every detail-file line, as on home-level lines.
contract/detail-line/message · the detail-file line · message — Required on every detail-file line, as on home-level lines.
contract/detail-line/process · the detail-file line · process — Required on every detail-file line, as on home-level lines.
contract/detail-line/target · the detail-file line · target — Required on every detail-file line, as on home-level lines.
contract/detail-line/timestamp · the detail-file line · timestamp — Required on every detail-file line, as on home-level lines.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/detail-upload-needs-synthetic-input · Detail upload needs synthetic input — Detail files and harness captures are uploaded only while every input is synthetic; a non-synthetic input source must first drop detail files and the harness capture from the upload paths.
decision/endpoint-paths-never-logged · Endpoint and pinned binary never logged — The snapshot endpoint and pinned_bin are never logged, only endpoint_kind.
decision/fixtures-not-a-telemetry-source · Fixtures not a telemetry source — Recorded fixtures are not a telemetry source, and observability adds no fixture content.
decision/mutants-out-never-uploaded · Mutation outputs never uploaded — mutants.out/ and the per-run archive of JUnit and outcomes files are never uploaded. · why: The outcomes file held absolute argv paths and the log folder held test output.
decision/mutation-diff-never-uploaded · Mutation diff never uploaded — The mutation run's chunk.diff at the harness capture root is excluded from the harness upload, so it is never scanned and never uploaded. · why: The diff is repository source text by construction and hits the scan's Critical-class patterns.
decision/prompt-and-output-never-in-home-logs · Prompt and assistant output never in home logs — Prompt text and assistant output, last_assistant_message included, never appear in a home-level process-log line; they reach an instance detail file only inside a chain or drift report, and only text_bytes is logged.
decision/agent-rows-never-verbatim · Agent rows never logged verbatim — claude agents --json rows are untrusted and never logged verbatim, only parse-rejected counts.
decision/canary-only-in-detail-files · Canary only in detail files — Every E2E and integration input is synthetic and embeds a fixed canary, which may appear in detail files but never in a home-level diagnostics file. · guard: mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line
decision/closed-protocol-codes · Closed protocol codes — Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value. · guard: answer_invalid_params_names_its_cause_and_no_param
decision/typed-ids-never-in-detail · Ids logged as typed fields — pid, child_pid, agent_session_id and started_at are logged as typed fields, never inside detail codes.
decision/veil-redaction · veil derives redaction — veil 0.3.0's derived Redact is what redacts the payload fields of contract types, and instrumented spans skip all arguments.

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/deterministic-emission · Deterministic event emission — Events key on hook events and events.ndjson offsets; nothing is derived from screen content. · except: The readiness gate reads the vt100 screen before a paste and can refuse a send as input-not-ready; a local command's echo confirmation keys on its transcript's stdout tag count.
decision/schema-conformance-failure-fails · Schema-conformance failure is a failure — An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/claude-env-and-cookie-never-logged · CLAUDE* values never logged — CLAUDE* values are never logged, at any level, in any file, detail files included.
decision/no-telemetry-from-screen-content · No telemetry from screen content — Telemetry is never derived from screen content; vt100 is used for readiness and modal gate outcomes only.
decision/paths-only-in-instance-detail-files · Paths only in instance detail files — Paths appear only in instance detail files.
decision/scrub-before-bytes-leave-process · Scrubbing before any byte leaves the process — Every byte is scrubbed of secrets before it leaves the process; in v1 the only destination is local files. · guard: the secret-scan test
decision/sensitive-content-detail-file-only · Sensitive content only in the instance detail file — Send text, tool input and last_assistant_message never reach a home-level file; they may reach only the instance-scoped detail-<process>.ndjson, inside a chain or drift report.
decision/transcripts-not-a-contract · Claude Code transcripts are not a contract — Claude Code transcripts are never read as a contract. · except: A local command's stdout tag in the session transcript, counted past the end recorded before the paste, is read as that send's echo confirmation; no transcript byte is kept.
decision/allow-list-not-widenable · Allow-list and redaction not configurable — config.json, a VIOLA_* env var or a CLI flag never widens the field allow-list or disables redaction.
decision/machine-asserted-telemetry-verdicts · Machine-asserted telemetry verdicts — Log analysis is never human-review-gated; every CI telemetry verdict is a jq or hyperfine assertion that sets an exit code.
decision/no-paths-or-pids-in-detail-codes · No paths or pids in detail codes — detail codes never contain paths or pids.
decision/upload-only-after-secret-scan · Diagnostics uploaded only after the secret scan — diagnostics artifacts are never uploaded before the secret-scan step passes; uploads run if: always() / failure() combined with the secret-scan step's success, never bare.
contract/stdout-by-role/cli · stdout by role · cli — stdout carries the --json results.
contract/stdout-by-role/mcp · stdout by role · mcp — stdout carries MCP frames.
contract/stdout-by-role/run · stdout by role · run — The terminal of run carries the child's screen.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/content-only-in-instance-detail-file · Content only in instance-scoped detail files — Content-bearing detail goes only to the instance's detail-<process>.ndjson file; home-level log files are codes-only and carry no user content, token or CLAUDE* value. · why: security allows user-derived content only in instance-scoped diagnostics

## log-file-location · key of ## 3. Observability Harness Contract
contract/diagnostics-files/home-resolution · diagnostics files · home resolution — The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
contract/diagnostics-files/paths · diagnostics files · process log paths — Process logs are written to <home>/diagnostics/ as run-<name>, hook-<name>, mcp and cli-<name> .ndjson files, and detail to <home>/instances/<name>/diagnostics/detail-<process>.ndjson.

## log-format-json-schema · key of ## 3. Observability Harness Contract
decision/no-multiline-stack-traces · No multi-line stack traces — Every log line is one physical line parseable by jq; a hook panic is caught and logged as one ERROR line with event panic.
contract/corr-by-event/parse-rejected · corr by event · parse-rejected — corr is null.
contract/corr-by-event/process-and-panic · corr by event · process-start, process-exit, panic — corr is null.
contract/process-log-line/message · the process log line · message — The message is a static literal equal to the event name and never carries interpolated content.

## logging-stack · key of ## 3. Observability Harness Contract
decision/content-records-bypass-subscriber · Content-bearing records bypass the subscriber — Content-bearing records never pass through the tracing subscriber, whose one writer holds only the home-level role file; the detail writer formats each detail line itself and writes it with one whole write.
decision/file-only-diagnostics-sinks · File-only diagnostics, one file per role — Every role logs only to its own file under the home's diagnostics directory; no role has a stdout sink, and there is no pretty or dev sink.
decision/no-content-in-home-level-lines · No content fields in home-level lines — No home-level diagnostics line carries chain, drift_report, panic_payload or backtrace. · guard: G4 diag-line.v1.json schema
decision/no-log-crate-bridge · No log-crate bridge — The log-crate bridge (tracing-log, LogTracer) stays off; failures are captured from returned values or handles, and a gap is fixed at the seam or by the researched json-subscriber fallback, never by enabling the bridge. · why: The accepted loss is only third-party log context emitted without a returned error. · guard: pty_exit_is_read_on_the_handle_while_the_output_is_held
contract/detail-line/fields · the detail line · fields — Each detail line carries timestamp (millisecond UTC format), level, target, message, event, process and instance, then its content fields, and no corr or conn key.
contract/diagnostics-files/detail · the diagnostics files · detail — Content-bearing records (full error chains, drift reports, panic payload and backtrace) go only to <home>/instances/<name>/diagnostics/detail-<process>.ndjson, file 0600 in a 0700 directory, and only when an instance resolves.
contract/diagnostics-line/role-file-line-size · the diagnostics line · role-file line size — A line in a role file (run, hook, mcp, cli) is codes-only and stays well under 4 KiB.
decision/third-party-targets-off · Third-party log targets off — Records of third-party targets (rmcp, portable_pty, sysinfo, interprocess) are off in every sink; their failures are captured at viola's seam from the returned results. · why: Their records lack the required fields.
contract/diagnostics-line/detail-line-size · the diagnostics line · detail-file line size — A line in an instance detail file (chains, drift reports, one-line backtraces) is not codes-only and may exceed 4 KiB.
contract/diagnostics-line/format · the diagnostics line · format — Each diagnostics line is one structured JSON object on one line, written whole by one write of the complete line on an append-mode handle.
contract/obs-event-macro/attached-fields · the obs_event macro · attached fields — Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.

## otel-sdk-init · key of ## 3. Observability Harness Contract
decision/home-resolution-order · Home resolution order — The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
decision/home-tree-check-before-log-init · Home tree check before log init — run, revive's start arm and viola verify check the home tree's modes before their log opens; a refusal writes no line, starts no child, leaves modes as found and exits 1.
decision/lazy-detail-file · Detail file opened lazily — The instance detail file is opened only by the first content-bearing record or by the panic hook, and a failed detail open drops only the detail line, writing nothing to stderr. · why: The error-free hook path keeps exactly one log open.
decision/flat-log-lines-and-one-level-gate · Log lines are flat and Targets is the only level gate — The subscriber flattens event fields to the top level of each line, and the Targets filter is the only configurable level gate; its level comes from the home's diagnostics level and never from the environment. · why: The diagnostics schema and the log readers read event fields as top-level keys, and only the viola crates' targets pass the gate.
decision/hook-refusal-exits-zero · Refused hook fails open — A hook whose instance is refused or absent exits 0 at once, reading nothing under the home and no stdin, writing no frame and no log line.
decision/instance-check-before-log-init · Instance check before log init — Commands acting on an instance (revive's start arm, the hook arms, link, unlink, send, wait, last, answer, pause, release) run the instance strict-modes check before opening any log; a refusal opens and writes nothing and leaves modes as found. · why: Opening the role file narrows an existing home to 0700, so a check made after it passes a home at 0770. · except: The hook --capture arm runs ahead of the check.
decision/instance-source-per-role · Instance source fixed per role — run takes its own name argument, never an inherited VIOLA_NAME; hook and mcp take VIOLA_NAME only; cli verbs that act on an instance (send, wait, last, answer, pause, release, link, unlink) take the target argument; list, verify, plugin install and revive --list take the caller's VIOLA_NAME only. · why: A wrapper never writes into another instance's run log.
decision/private-diagnostics-files · Owner-only diagnostics files — The diagnostics directory is 0700 and each diagnostics file 0600, opened without following symlinks; before the first write each must be owned by the effective user, have no group or other bits and not be a symlink, a failed check counting as a failed open. · why: A pre-existing foreign, permissive or symlinked diagnostics file is never written.
decision/strict-modes-before-diagnostics · Home strict-modes check before diagnostics — Nothing under the home, the diagnostics directory included, is created or opened before the role's strict-modes check on the home passes; a refused home gets no diagnostics line.

## service-identity · key of ## 3. Observability Harness Contract
decision/env-vars-not-configuration · Environment variables are not configuration — Environment variables are not a configuration channel: service identity has no env-var source or fallback, and the one CLAUDE_CONFIG_DIR value read by verify and the run and revive start, when the strip keeps that name, locates the claude config file, is never logged and configures nothing of viola.

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
