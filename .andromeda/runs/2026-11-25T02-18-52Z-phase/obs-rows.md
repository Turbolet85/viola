# obs-plan · the rows that bind 2026-11-25-placing-without-moving-the-view — picked 262 of 1055 (167 by every draw) · rule any · pin 4974

## obs-plan-viola · (above the first heading)
decision/agent-readable-telemetry · Agent-readable telemetry only — Every telemetry signal must be readable by an agent from a snapshot or pasted log excerpt: no human-only dashboards, no proprietary APM lock-in, no human-review-gated log analysis. · except: A surface that cannot be agent-readable is flagged not-instrumentable and handled through boundary instrumentation only.

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/agents-probe-parse-failure-unknown · Agents probe parse failure becomes unknown — A parse failure of the claude agents --json probe becomes status "unknown" plus a parse-rejected counter; an unparsed --version answer leaves cli_version absent and the version unverified.
decision/claude-internals-not-a-contract · Wrapped claude internals are not a contract — The wrapped claude CLI is a third-party binary run unmodified and instrumented at its boundary only (spawn and exit, paste writes, the vt100 readiness-gate outcome); its internal state, transcripts and bundle strings, the MCP client's tool-call bound among them, are never read as a contract. · except: A send's echo confirmation counts its stdout tag in the transcript, keeping no byte; verify writes and removes its temp folder's trust record; a verified start sets its folder's trust flag only.
decision/cli-file-sink-only · Short-lived CLI logs to a file sink only — Short-lived CLI verbs log only to a JSON-per-line file sink in diagnostics when an instance resolves, never to stdout. · why: Stdout carries the --json results.
decision/distinct-detail-code-per-exit-cause · Distinct detail code per exit cause — Each cause behind exit 21 and exit 1 gets a distinct detail code with no path or pid. · except: A strict-modes check refused before any log opens writes no line; its cause is the fixed stderr pair or the --json document.
decision/error-chain-fixed-stderr-message · Error chains reach stderr as fixed messages — In the viola root binary, the only crate that uses anyhow, an error chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to the diagnostics logs only.
decision/exit-1-detail-codes · Distinct detail codes for exit-1 causes — On the `viola run` start sequence to child spawn, each exit-1 cause logs a distinct `detail` code with no path or pid in it.
decision/obs-deps-tokio-free · Obs dependencies Tokio-free and C-free — Obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass cargo deny. · guard: cargo deny
decision/os-apis-at-call-sites · OS APIs observed at call sites — OS APIs (openpty, Unix socket, filesystem locks and atomic rename, sysinfo) and the hyprctl spawn are instrumented at viola's call sites only, never in their internals.
decision/parse-failures-unknown-counter · Parse failures surface as unknown plus a counter — A child-output parse failure surfaces as `unknown` plus a counter. · except: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown.
decision/refusal-telemetry · Refusal telemetry for filesystem refusals — Pinned-exe hash mismatches and ignored symlinks produce refusal telemetry. · except: The transcript echo read and the verify and start trust-record writes refuse a symlinked, non-regular or foreign file with no refusal line; the send's lines, stamps or trust-record word show it.
decision/run-silent-terminal · viola run silent on the terminal — viola run writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns.
decision/run-sync-instrumentation · Sync instrumentation in viola run — Instrumentation in viola run is sync, with no Tokio.
decision/serde-errors-fixed-message · Serde error chains become a fixed message — A serde-sourced anyhow error chain is replaced by a fixed message before it reaches stderr.
decision/spans-not-wire-contract · Tracing spans are not part of the wire contract — Span names are internal `tracing` spans in `<area>.<operation>` snake_case form; the agent-facing contract is the closed `event` enum plus `corr`. · why: the config sets `with_current_span(false)`, so spans do not reach the line
decision/stderr-fixed-strings · Fixed strings on stderr — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/strict-modes-failure-lines · Where a strict-modes failure is logged — A strict-modes failure has a log line only on the `ledger-stamps` read in the `run` version gate, on `verify`'s own `process-exit` for a refused stamp write, and on `viola up`'s own `process-exit` for a home tree refused after its board scan.
contract/diagnostics-files/cli · the diagnostics files · cli — Role cli writes <home>/diagnostics/cli-<name>.ndjson, only when an instance resolves.
contract/diagnostics-files/run · the diagnostics files · run — Role run writes <home>/diagnostics/run-<name>.ndjson.
contract/process-exit-detail/strict-modes-failed · process-exit detail codes · strict-modes-failed — Carried by `verify`'s `process-exit` for a refused stamp write, and by `viola up`'s `process-exit` for a home tree refused after its board scan, each with exit 1.
contract/run-log-file/path · the viola run log file · path — viola run logs only to a file sink at diagnostics/run-<name>.ndjson.
decision/agents-probe-output-untrusted · Agents probe output untrusted — The output rows of the claude agents --json probe are untrusted.
decision/child-exit-from-process-handle · Child exit from the process handle — The wrapped child's exit and its process-exit event come from the wait on the process handle, never from reader EOF.
decision/stripped-claude-env-never-logged · R8-stripped CLAUDE values never logged — R8-stripped CLAUDE* environment values are never logged anywhere. · why: The viola-agent-claude outputs are the most sensitive in the product.
decision/stripped-env-never-logged · Stripped CLAUDE env values never logged — No R8-stripped `CLAUDE*` value appears in any sink or fixture; the strip is logged as a count or variable names only.

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/child-exit-from-handle-wait · Child exit from the process handle — The child's process-exit comes from the wait on its process handle, never from reader end-of-file. · why: Child exit is detected on the process handle.
decision/claude-boundary-only-instrumentation · Boundary-only instrumentation of claude — The hosted claude binary is instrumented only at its boundary: spawn and exit, paste writes, the vt100 readiness-gate outcome, and the start's one trust-record line for the trust record it writes in the claude config. · why: claude is a third-party binary run unmodified.
decision/claude-exit-on-process-handle · Claude exit detected on the process handle — The exit of the hosted claude process is detected on the process handle, never on EOF.
decision/detail-codes-no-paths · Detail codes carry no path or pid — No path or pid appears in a detail code.
decision/exit-code-log-field · Exit code as a log field — The exit code (0, 1, 2, 10, 12–14, 20, 21), the machine-readable outcome, is carried as a log field; 11 is retired and never reused.
decision/fixed-stderr-messages · Fixed stderr refusal and fault lines — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/fixtures-not-telemetry · Recorded fixtures are no telemetry source — Recorded fixtures are scrub-required data, not a telemetry source, and are excluded from the observability plan. · why: Not product runtime.
decision/instrument-call-sites-not-os-internals · OS operations instrumented at viola's call sites — OS-level operations (bind, connect, lock, rename, kill fallback, the server check's peer reads, the strict-modes stat reads, the socket directory check, the hyprctl spawn) are instrumented at viola's own call sites, not inside the OS.
decision/messaging-secrets-never-logged · Messaging secrets never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every R8-stripped CLAUDE* value are never logged in any sink, detail files and fixtures included. · guard: secret-scan test
decision/no-secrets-in-log-lines · No secrets in log lines — No log line may contain any stripped CLAUDE* value. · guard: secret-scan test
decision/parse-failures-surface-as-unknown · Parse failures surface as unknown — A parse failure surfaces as the value unknown plus a counter. · except: A hyprctl reply failing its parse reads oversize or malformed, and a StopFailure error word outside the CLI's set reads cause api-error, each with one parse-rejected line, never unknown.
decision/per-parser-parse-failure-counts · Per-parser parse-failure counts — Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, the hyprctl replies, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead.
decision/process-init-order · Process logging init order — Each process installs its panic hook, parses arguments, resolves home and instance and runs its strict-modes check, opens its diagnostics file append-mode 0600 in a 0700 directory, installs the subscriber, then emits process-start. · why: For run this completes before the start sequence so the collision check and version gate are logged; for mcp, before the stdio transport starts.
decision/r8-strip-logged-as-names-only · R8 strip logged as names only — The R8 environment strip is logged as a count and lists of stripped and kept variable names, and environment maps never pass to a span or event; no value is read, bar the kept CLAUDE_CONFIG_DIR value that viola verify and the run, revive and adopt start read, and no value is logged.
decision/run-silent-on-terminal · run silent on the terminal — run writes nothing to the terminal while the child runs; its process log goes only to diagnostics/run-<name>.ndjson. · why: The terminal carries the child's screen.
decision/serde-chain-fixed-message · Serde error chains become a fixed message on stderr — An error chain that contains a serde source becomes a fixed message before it reaches stderr.
decision/serde-errors-become-fixed-message · Serde errors become a fixed message — An error chain holding a serde source is replaced by a fixed message before it reaches stderr.
decision/strict-modes-refusal-writes-no-line · Strict-modes refusal writes no line — A strict-modes check refused before any log opens ends the process with nothing under the home opened and no log line; its cause is carried only by the fixed stderr pair or the --json document. · why: Ratified as the answer to whether a refused strict-modes check logs. · except: The ledger-stamps read in run's version gate and a refused update_stamps in viola verify each leave a line.
decision/stripped-claude-values-never-logged · Stripped CLAUDE values never logged — R8-stripped CLAUDE* values are never logged anywhere. · why: Its outputs are the most sensitive in the product.
decision/sync-instrumentation-no-tokio · Sync instrumentation without Tokio — Instrumentation in the sync crates runs on std threads with no Tokio. · why: Hybrid concurrency. · guard: cargo deny ban
contract/cli-internal-error/stderr-line · CLI internal error · stderr line — Without --json a catch-site fault prints the fixed line `error: internal error` to stderr.
contract/cli-log-file/revive-start · CLI process-log file · viola revive start arm — Logs as the wrapper, process="run", into diagnostics/run-<name>.ndjson.
contract/exit-1-detail/already-live · Exit 1 detail codes · already-live — already-live is a distinct detail code for an exit-1 cause, naming no path or pid.
contract/exit-1-detail/pinned-hash-mismatch · Exit 1 detail codes · pinned-hash-mismatch — pinned-hash-mismatch is a distinct detail code for an exit-1 cause, naming no path or pid.
contract/exit-1-detail/socket-dir-refused · Exit 1 detail codes · socket-dir-refused — socket-dir-refused is a distinct detail code for an exit-1 cause, naming no path or pid.
contract/exit-1-detail/squatted-name · Exit 1 detail codes · squatted-name — squatted-name is a distinct detail code for an exit-1 cause, naming no path or pid.
contract/exit-1-detail/stale-heartbeat · Exit 1 detail codes · stale-heartbeat — stale-heartbeat is a distinct detail code for an exit-1 cause, naming no path or pid.
contract/exit-cause-detail/socket-dir-refused · Exit cause detail codes · socket-dir-refused — A refused socket directory has its own detail code socket-dir-refused and writes its line (MCP writes none); on run and on a start made by viola revive that line is one process-exit{subject:"self", exit_code:1} before any snapshot, child or bind.
contract/process-exit/socket-dir-refused-on-start · Process exit line · socket-dir-refused on a start — A refused socket directory on a start writes one process-exit with detail socket-dir-refused, exit 1.
contract/process-log-corr/process-exit · Process log corr by event · process-exit — corr is null.
contract/process-log-corr/process-start · Process log corr by event · process-start — corr is null.
contract/process-log/child-and-probe-lifecycle · Process log · child and probe lifecycle — process-start and process-exit are logged for the claude child, for the claude agents --json and --version probes and for verify's print-mode sender; the terminal viola open or viola up spawns gets a process-start alone.
contract/run-exit-1-detail/already-live · run exit-1 detail codes · already-live — A live holder or a fresh replayed beat.
contract/run-exit-1-detail/sha-256-mismatch · run exit-1 detail codes · sha-256 mismatch — A distinct detail code for a sha-256 mismatch.
contract/run-exit-1-detail/squatted-name · run exit-1 detail codes · squatted-name — A distinct detail code for a squatted name.
contract/run-exit-1-detail/stale-heartbeat · run exit-1 detail codes · stale-heartbeat — A stale holder.
contract/run-log/cli-verified · run log line · cli_verified — A required field on run's log lines.
contract/run-log/cli-version · run log line · cli_version — An optional field on run's claude-child process-start line; absent when the CLI version is unreadable.
contract/run-log/exit-code · run log line · exit_code — A required field on run's log lines.
contract/run-start-events/order · run start events in events.ndjson · order — A run start writes wheel{cause:"start"}, then one lineage line, then one launch line before session-start; on a start whose folder-trust keys pass through the wrapper, wheel{human-input} also precedes session-start, and a later human key adds no wheel line, the wheel being already human.
contract/run-start-events/window-line · run start events in events.ndjson · window line — A start made under --origin writes one window line {address, workspace, placement, origin} with source wrapper after its launch line.
contract/viola-verify/home-tree-refused · viola verify · refused home-tree check — A refused home-tree check exits 1 with the home pair, opens no log and writes no line; a refused update_stamps exits 1 with the home pair and, when an instance is named, ends the opened log with a process-exit line carrying detail strict-modes-failed.
decision/fault-telemetry-per-class · Fault telemetry per class — Each fault class emits its own telemetry: a healed torn ndjson line with file and offset, a corrupt or unsupported snapshot recovered by replay, a child exit seen without EOF, an endpoint vanished during wait, and a stale heartbeat against a live pid.
decision/log-file-basenames-follow-tests · Log file names follow tests — Process-log files are <home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson, as tests require. · why: Tests is upstream; arch and security place diagnostics under the instance directory instead.
decision/no-process-log-without-instance · No process log without an instance — A short-lived CLI verb run with no resolved instance writes no process-log file; its human stderr carries only fixed-message lines, and its outcome is the exit code and stdout.
decision/no-telemetry-from-screen · No telemetry from screen content — No telemetry is derived from screen content; vt100 serves gate outcomes only.
contract/dialog-log-line/unverified-cli · Dialog log line · unverified-cli — The detail unverified-cli goes with exit 12.
contract/event-line/shape · events.ndjson event line · line shape — Exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
contract/exit-1-detail/cwd-missing · Exit 1 detail codes · cwd-missing — cwd-missing is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
contract/exit-1-detail/no-session · Exit 1 detail codes · no-session — no-session is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
contract/exit-1-detail/session-live · Exit 1 detail codes · session-live — session-live is a distinct detail code for the exit-1 cause of viola revive and of viola adopt, naming no path or pid.
contract/exit-cause-detail/server-verify-failed · Exit cause detail codes · server-verify-failed — A failed server check of peer euid, peer pid or start time is logged by the CLI and the hook as server-verify-failed; MCP writes no line for it.
contract/process-log/event · Process log line · event — A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
contract/run-exit-1-detail/cwd-missing · run exit-1 detail codes · cwd-missing — On a start made by viola revive, a distinct detail code.
contract/run-exit-1-detail/no-session · run exit-1 detail codes · no-session — On a start made by viola revive, a distinct detail code.
contract/run-exit-1-detail/session-live · run exit-1 detail codes · session-live — On a start made by viola revive or viola adopt, a distinct detail code.

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/span-naming · Span naming as <area>.<operation> — Span names are <area>.<operation> in snake_case, <area> one of run, pty, channel, state, hook, send, wait, last, answer, pause, release, cli, mcp, hyprland; cli.<verb> names the dispatch span of a short-lived verb.
contract/process-log-line/enum-values · the process log line · enum values — Enum values are kebab-case.
decision/machine-parseable-signals · Every signal machine-parseable — Every signal produces machine-parseable output: JSON-per-line process logs, the ndjson product event stream, JSON harness documents, hyperfine JSON for perf gates and nextest JUnit XML in CI.
decision/no-human-gated-log-analysis · No human-review-gated log analysis — Every log verdict is a jq or harness assertion that sets an exit code, never a human review.
contract/process-log-line/event · the process log line · event — The event field is a closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/span-naming-skip-all · Span declaration convention — Every span is declared with instrument, skip_all and a name of the form "<area>.<operation>".
decision/spans-not-serialized-in-v1 · Spans declared but not serialized in v1 — Spans are declared on the must-trace paths and every crate seam but are not written to output in v1. · why: they give the code its structure and are ready for the deferred OpenTelemetry bridge
decision/pre-open-strict-modes-refusal-silent · Strict-modes refusal before a log opens writes no line — A strict-modes refusal made before any process log opens writes no line and carries no logged detail code; its machine-readable form is the exit code and, under --json, its closed document. · except: a refused socket directory always logs detail socket-dir-refused, except at MCP's frame tool, which writes no line
contract/socket-dir-refused-exit/codes · socket-dir-refused exit codes · socket-dir-refused — A refused socket directory logs detail socket-dir-refused with no path, pid or uid, and exits 21 at the eight CLI frame verbs and 1 at the start of run, revive and adopt.
contract/strict-modes-refusal-exit/1 · strict-modes refusal exit codes · 1 — run, revive's start, adopt's start, verify, open and up's home-tree check exit 1 on a strict-modes refusal; revive, adopt, open and up under --json print the strict-modes-failed document.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
contract/run-terminal-output/child-output · run terminal output · child output — run writes nothing to the terminal except the child's own output and, before the child, a keyed start's automatic-verify lines and the verify child's own output, none under revive --json; while the child runs, neither run nor a revived start writes anything of its own.
contract/run-terminal-output/start-refusal · run terminal output · start refusal — A start refusal before any child is spawned writes exactly two fixed stderr lines with no path or pid: the home pair for a home-level refusal, the instance pair for an instance-level one.
decision/codes-only-process-logs · Codes-only process logs, content in detail files — Every role's codes-only process log goes to its home-level role file under diagnostics, and content-bearing detail goes only to the instance's per-role detail file.
decision/obs-owns-logger-and-format · Obs owns the logger and format — The logger and the log format are owned by observability.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/browser-tools-prompt-reading-uninstrumented · Browser-tools prompt reading uninstrumented — The reading of the CLI's browser-tools prompt as a modal adds no log line, span, event or detail code of its own; verify's reading of it reaches only the ledger stamp and verify's stdout, whose one fixed line on a met prompt is the only record that the prompt stood at start.
decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.
decision/detail-codes-only · detail is a closed code — The detail field is a closed kebab-case code catalog and never carries a path or pid.
decision/env-names-only · Environment names only, never values — The strip, every log, the hook capture arm's env-names file and the verify stamp and record carry environment variable names only, never values, and no env map is passed to a span or event. · guard: E2
decision/ledger-contents-never-logged · Capability ledger path and stamps never logged — The capability ledger's path and its stamp contents never appear in a log line.
decision/no-endpoint-path-or-env-in-spans · No endpoint, binary path or env values in spans — A span never carries the endpoint name, the pinned_bin path or environment values.
decision/span-field-allow-list · Spans declared with explicit field allow-list — Every span is declared skipping all arguments with an explicit field allow-list.
decision/static-span-names · Static span names — Span names are static strings; instance names, dialog ids and paths never appear in a span name.
decision/trust-record-write-uninstrumented · Trust-record write and removal uninstrumented — viola verify's trust-record write and removal emit no log line and no span, the outcome riding only the stamps; a start's trust step opens no span and writes only its one trust-record line; no path, key or value of the claude config reaches any line, stamp or summary.
decision/unreadable-ledger-is-unverified · Unreadable ledger counts as unverified — A missing stamp, or an unreadable, malformed or strict-modes-refused capability ledger, counts as unverified (cli_verified false).
decision/verify-lines-redacted · verify lines carry no content — No verify or plugin install line carries row text, a prompt, a session id or a path.
contract/viola-verify-refusal/stamp-write-refused · viola verify strict-modes refusal · stamp write refused — A ledger/stamps.json the stamp write refuses prints the home pair and exits 1 through verify's own process-exit line.
decision/child-settings-never-logged · Child settings argument never logged — The claude child's --settings argument appears in no log line or span.
decision/dialog-content-never-logged · Dialog content never logged — Dialog questions, answers, plan text and tool input never appear in a home-level process-log line; dialog events carry codes only, and the text reaches an instance detail file only inside a chain or drift report.
decision/no-assistant-message-in-logs · No assistant message or result body in logs — last_assistant_message never appears in a log, and no result body reaches a log line.
decision/no-err-ret-span-options · err and ret span options banned — The err and ret instrumentation options are banned on spans.
decision/no-trace-and-span-ids · No trace_id or span_id — Log lines never emit trace_id or span_id; correlation is by corr plus conn. · why: No W3C trace context exists, and spans are not attached to lines.
decision/verify-logs-only-with-instance · verify and plugin install log only inside an instance — With VIOLA_NAME set, verify and plugin install log process-start / process-exit to cli-<name>.ndjson; from a plain terminal with no instance they write no process-log file.
contract/viola-verify-refusal/home-tree-refused · viola verify strict-modes refusal · home tree refused — Before logging, verify holds the home, ledger/, bin/ and plugin/ to the strict-modes check; a refused tree exits 1 with the home pair and writes no line.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/error-chain-in-detail-log-only · Full error chain only in the cli detail log — The full anyhow error chain of a cli verb goes to the cli detail log only.
decision/home-tree-check-before-start-log · Home-tree check before the start's log — The start of run, of viola revive and of viola adopt checks the home tree (the home, instance set, ledger, bin, plugin, pinned exe and plugin hooks) before its log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
decision/no-paths-in-run-spans · No endpoint, path or env values on start spans — Start spans never carry the endpoint name, the pinned binary path or environment values.
decision/real-cli-verify-never-in-ci · Real-CLI verify never runs in CI — In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
decision/revive-start-logs-as-run · viola revive start logs as run — The start arm of viola revive is not a cli verb and logs as process run once its instance check has passed.
decision/unreadable-ledger-means-unverified · Unreadable capability ledger means an unverified CLI — A capability ledger that is missing a stamp, unreadable, malformed or refused by the strict-modes check is treated as an unverified CLI (cli_verified false).
decision/verify-lines-carry-no-content · verify log lines carry no content — No verify log line carries row text, a prompt, a session id, the probe command or a path.
decision/verify-sole-stamps-writer · verify is the only stamps writer — verify is the only writer of the capability ledger's stamps file.
decision/window-line-untraced · Window line has no trace of its own — The window line a start under --origin writes after its launch line, and a switch writes again after its own launch line, is log-only product audit data: it wakes no wait, opens no span and is never copied into a process-log line. · provisional
contract/cli-log-file/open · CLI process-log file · viola open — Logs as cli into the opened instance's cli-<name>.ndjson, its work inside the cli.open dispatch span.
contract/cli-log-file/up · CLI process-log file · viola up — Logs as cli into the caller's cli-<VIOLA_NAME>.ndjson only under a parsed VIOLA_NAME, after its strict board scan, inside the cli.up dispatch span: its own process-start and process-exit, one process-start{subject:"terminal"} per spawn and state-recovered; a refused scan writes no line.
contract/cli-stderr/internal-error · CLI stderr · internal fault message — An internal fault prints exactly "error: internal error".
contract/cli-stderr/refusal-pair · CLI stderr · unable and hint pair — A refusal is printed to stderr as a fixed "unable:" line followed by a "hint:" line.
contract/run-log-parse-rejected/ledger-stamps · the run process log parse-rejected line · parser ledger-stamps — An unreadable, malformed or strict-modes-refused capability ledger gives one parse-rejected line with parser "ledger-stamps", detail "unreadable", "malformed" or "strict-modes-failed", and a count, in the run log.
contract/run-process-log/auto-verify-stop · run process log · stop during the automatic verify — A SIGINT or SIGTERM during the start's automatic verify TERMs the verify child, KILLs it after a 30 s grace, writes the auto-verify process-exit, then re-raises the signal: the start ends with no process-exit of its own, no stamp, no snapshot and no session.
contract/run-process-log/process-start-claude-child · run process log · process-start claude-child — On success, process-start{subject:"claude-child", child_pid, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept}.
contract/run-span-attributes/run-version-gate · viola run start span attributes · run.version_gate — run.version_gate carries cli_version and cli_verified.
contract/run-span-attributes/run-window · viola run start span attributes · run.window — run.window carries the field outcome alone, never an address, pid or workspace.
contract/run-start-spans/span-chain · viola run start spans · span chain — run.start (INTERNAL, root) › state.snapshot_recover › run.collision_check › run.pin_copy › run.version_gate › channel.bind › state.snapshot_write › state.heartbeat_start › pty.spawn (CLIENT); under --origin, run.window follows the trust step, before state.snapshot_write.
contract/verify-process-exit/strict-modes-failed · verify process-exit line · detail strict-modes-failed — A stamps store refused by its strict-modes check, under a parsed VIOLA_NAME, ends with process-exit carrying detail "strict-modes-failed" and the home pair on stderr.
decision/adopt-start-logs-as-run · viola adopt start logs as run — The start of viola adopt is not a cli verb and logs as process run into the run log of its name, through run's own lines and spans, adding no event, detail word, field, subject or span of its own. · except: The detail word not-resumed, on adopt's own self process-exit exit 1 when the resumed child ends before its session starts.
decision/claude-env-strip · CLAUDE environment strip — The child loses every inherited CLAUDE* name, matched exactly, except the persistent set kept in the claude_env_keep config key.
decision/no-obs-gate-on-verify-lines · No obs gate depends on verify lines — No observability gate depends on verify's own log lines; its home-level lines fall under the same panic, schema and secret-scan gates as any other.
decision/product-order-from-events-log · Start product order read from the events log — The start product order, wheel{cause:"start"} then lineage then launch before session-start{source:"hook"} with wheel{human-input} also before it where folder-trust keys pass through the wrapper, is verified from that start's own lines of the instance's events log, not duplicated into process logs.
decision/refused-home-tree-writes-no-line · Refused home tree writes no line — verify and plugin install check the home tree before their log opens, and a refused tree opens no log and writes no line.
contract/cli-exit/1-internal-error · send.client process-exit · 1 internal-error — exit_code 1 internal-error on a reply with neither a parseable result nor an error.code.
contract/run-process-log/common-fields · run process log · common fields — Every run line carries the common fields with process="run" and corr null.
contract/run-start-failure-detail/socket-dir-refused · run start process-exit · socket-dir-refused — A socket directory the start refuses is one process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"} before any snapshot, child or bind, on viola revive's start too; a bind failure keeps its own outcome.
contract/run-start-spans/run-start-close · viola run start spans · run.start close — run.start closes when pty.spawn returns.
contract/span-kinds/internal · Span kinds · INTERNAL — The INTERNAL spans are run.*, state.* and pty.paste_write; a hook's decision is the decision_emitted field, not a span.
contract/verify-exit-codes/0 · verify exit codes · 0 — Every relied ledger row counted: each passed, or the browser-tools-prompt row read absent.
contract/verify-exit-codes/1 · verify exit codes · 1 — A row failed, verify refused, or verify was stopped.
contract/verify-process-exit/interrupted · verify process-exit line · detail interrupted — A verify stopped by SIGINT or SIGTERM starts no further spawn, closes every pair it opened and ends with process-exit subject self, exit_code 1, detail "interrupted".
contract/verify-stop-stderr/unable-hint-pair · verify stderr on a stop · the stop pair — A stopped verify prints exactly "unable: verify was stopped before it finished" and "hint: run viola verify again to stamp this CLI" on stderr, with empty stdout and no stamp written.

## 5-metric-coverage · ## 5. Metric Coverage · twin metric-coverage
decision/metric-grouping-closed-enums · Metric grouping by closed enums only — Aggregate reports group only by closed enums (event, detail, refusal, method, hook_event, subject, parser, process); corr, conn, pid, child_pid and instance are join keys, never grouping keys.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/additive-field-naming · Additive log field naming — Additive log fields are snake_case with kebab-case values, and carry codes only.
decision/file-only-per-role-log-sinks · File-only log sinks per role — Diagnostics are written to files only, one sink per role, under the home's diagnostics directory.
decision/json-to-file-only-log-mode · JSON-to-file as the only log mode — JSON to file is the only log mode: there is no agent-mode flag and no stdout pretty sink on any role.
contract/child-spawn-log/subject · child and shell spawn log events · subject — process-start and process-exit lines for child spawns carry a subject from a closed set.
contract/log-levels/error · log levels · error — Used for a panic, a process-exit that carries a detail, and a channel-response with error_code -32603.
contract/log-levels/info · log levels · info — INFO carries all other boundary, lifecycle and state-transition events, including refusals, which are normal product outcomes.
contract/log-levels/warn · log levels · warn — WARN carries state-recovered; parse-rejected; hook-decision with a fail-open detail; process-exit with exit 21; channel-response with -32600 or -32602.
contract/log-line/event · every log line · event — A value of the closed kebab-case event enum.
contract/process-exit-detail/exit-1-up · process-exit detail · exit 1 from viola up — viola up ends exit 1 with process-exit detail not-live when any instance reads not-live, else with no detail (an exit 1 from no-session, cwd-missing, session-live or not-taken alone); its refusals each write their own detail word.
contract/start-trust-log/trust-record · the start's trust-record log line · trust-record — Each wrapped start of viola run, viola revive and viola adopt on a verified CLI writes exactly one INFO trust-record line to the run role file, its detail one of a closed set of 16 start words, carrying no path, project key or config value; the word never refuses the start.
decision/content-detail-in-instance-file · Content-bearing detail in the instance detail file — Content-bearing detail goes only to the per-process detail file inside the instance's diagnostics directory.
decision/ipc-calls-logged-without-arguments · IPC calls logged without arguments — IPC params and result bodies are never recorded, not even as a hash or digest.
decision/log-schema-additive-only · Log schema additive to the tests' format — The observability log schema only adds fields and enum values to the tests' log format; it renames nothing and removes only a field no build emits. · why: The tests' log format is the binding text.
decision/unlisted-targets-log-off · Unlisted log targets off — The per-module log filter turns logging OFF for every target it does not list.
decision/viola-crates-log-at-info · Viola crates log at INFO — The per-module log filter sets the viola, viola_core, viola_pty, viola_channel, viola_state, viola_agent_claude and viola_mcp targets to INFO, or DEBUG via diagnostics_level.
contract/log-line/corr · every log line · corr — Set per the tests' table and copied unchanged as a JSON number or string; null is represented by the key's absence.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
contract/diag-detail/strict-modes-failed · diagnostic detail words · strict-modes-failed — Written on the process-exit exit 1 of viola verify and of viola up under a parsed VIOLA_NAME, and on parse-rejected ledger-stamps from the version gate of run, at warn, with no path; every other refused strict-modes check writes no log line, its exit code or JSON document its only record.
contract/diag-line-fields/process-exit · diagnostic line additive fields · process-exit — subject, exit_code (self), child_exit_status, exit_source (handle-wait or kill-fallback), detail, during (connect or call), duration_ms.
contract/diag-line-fields/process-start · diagnostic line additive fields · process-start — subject (one word of the diag-line subject enum), service_name, version, os, pid, child_pid, endpoint_kind, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept (names only).
contract/diag-subject/auto-verify · diag-line subject enum · auto-verify — Subject of the process-start and process-exit pair of the viola verify child that a viola run start spawns, the exit carrying child_exit_status and duration_ms, written to the run log.
contract/diag-subject/hyprctl-probe · diag-line subject enum · hyprctl-probe — Subject of the process-start and process-exit pair that the Hyprland seam writes around each hyprctl spawn.
contract/parse-rejected-detail/malformed · parse-rejected detail · malformed — Any parser.
contract/parse-rejected-detail/oversize · parse-rejected detail · oversize — channel-frame beyond MAX_FRAME, hook-stdin, mcp-stdin past MAX_FRAME, vt100-feed when the bounded queue drops a copy (one line per poisoning episode), claude-agents-json past MAX_FRAME, hyprctl-json past MAX_FRAME.
contract/parse-rejected-detail/strict-modes-failed · parse-rejected detail · strict-modes-failed — ledger-stamps: run's version gate refused the stamps file on the strict-modes check, and the read counts as unverified.
contract/parse-rejected-detail/unreadable · parse-rejected detail · unreadable — config-json, ledger-stamps.
contract/parse-rejected/hyprctl-json · parse-rejected diagnostic line · parser hyprctl-json — A hyprctl reply that is oversize or malformed gives one parse-rejected line with parser hyprctl-json, written by the Hyprland seam and carrying codes only.
contract/process-exit-detail/exit-1 · process-exit detail · exit 1 — already-live, stale-heartbeat, squatted-name, pinned-hash-mismatch, internal-error, socket-dir-refused, from revive's preflight no-session, session-live, cwd-missing, and from viola open terminal-not-found and not-live; each one process-exit line with no child started, bar not-live's terminal.
contract/process-exit-detail/interrupted · process-exit detail · interrupted — A viola verify stopped by SIGINT or SIGTERM before its rows are checked ends with one process-exit exit 1 detail interrupted, after every spawn pair it opened is closed.
contract/verify-process-exit/child-exit-status-and-duration · viola verify spawn process-exit lines · child_exit_status and duration_ms — Each verify spawn pair's process-exit carries child_exit_status and duration_ms, including a child of an interactive PTY run killed at a Ctrl-C or TERM stop; a stopped run writes only the pairs it started.
contract/diag-line-fields/parse-rejected · diagnostic line additive fields · parse-rejected — parser (claude-agents-json, vt100-feed, config-json, ledger-stamps, hook-stdin, channel-frame, mcp-stdin, state-entry, hyprctl-json), detail, count.
contract/diag-line-fields/trust-record · diagnostic line additive fields · trust-record — A fixed message and a required detail drawn from the 16 start words; trust-record stands in the event enum of both the diag-line and the detail-line schema.
contract/process-exit-detail/verify-refusal · process-exit detail · viola verify refusal — Under a parsed VIOLA_NAME, verify writes its refusal code: program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture or interrupted; a failing row carries no code.

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/boundary-errors-end-in-logged-line · Crate-seam errors end in a logged line — Every typed error that crosses a crate seam ends in a logged process-exit detail, channel-response error, hook-decision detail or send-refused line, carrying codes only at WARN or ERROR.
decision/typed-errors-fixed-display · Typed errors with fixed Display messages — Every typed error has a fixed-message Display, written with thiserror, and holds no fields with paths or payloads. · except: PtyError's Display is hand-written, as viola-pty takes no thiserror.
contract/panic-outcome-by-role/run · panic outcome by role · run — After a panic, run logs process-exit{subject:"self", exit_code:1, detail:"internal-error"} and exits 1, writing nothing on the terminal while the child runs.
contract/panic-outcome-by-role/cli · panic outcome by role · cli — After a panic, cli logs process-exit{subject:"self", exit_code:1, detail:"internal-error"} when an instance resolves, puts the chain in the detail file only, writes exactly "error: internal error\n" on stderr in one write with no hint, and exits 1.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
contract/cli-internal-error/human-line · cli internal error · error: internal error — Without --json the cli catch site prints exactly error: internal error; run, revive's start, adopt's start, mcp and hook print neither.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/default-deny-log-fields · Default-deny for logged fields — A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure. · guard: gate G4 against schemas/diag-line.v1.json
decision/endpoint-and-pinned-bin-never-logged · endpoint and pinned_bin never logged — The endpoint and pinned_bin values are never logged.
decision/fixed-error-messages · Fixed error messages — Error Display texts are fixed, and serde-sourced error chains are mapped to fixed messages at dispatch.
decision/home-level-lines-path-free · Home-level lines path-free — Home-level log lines are path-free: file fields are basenames, panic_location is workspace-relative and source file recording is off; absolute paths appear only in detail files. · why: The OS username inside absolute paths is the only PII in scope.
decision/low-class-metadata-as-typed-fields · Low-class metadata as typed fields — Low-class metadata is logged as typed fields, never inside detail codes.
decision/schema-top-level-unevaluated-properties · Default-deny schema via top-level unevaluatedProperties — The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists. · why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then · guard: contract_diag_schema
decision/source-allow-list-logging · Source allow-list for logged fields — Spans skip all arguments and declare fields explicitly, and events go through obs_event! with explicit typed fields only; this source allow-list is the primary PII control. · guard: disallowed-macros; raw_event_hits; G1 bare instrument
decision/no-allow-list-widening · Allow-list cannot be widened — No config key, flag or env var can widen the logged-field allow-list or disable redaction.
decision/secrets-never-logged · Secrets never logged — Signing credentials and R8-stripped CLAUDE* values are never logged.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/closed-protocol-codes · Closed protocol codes — Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value. · guard: answer_invalid_params_names_its_cause_and_no_param
decision/endpoint-paths-never-logged · Endpoint and pinned binary never logged — The snapshot endpoint and pinned_bin are never logged, only endpoint_kind.
decision/error-bodies-clean · Clean error bodies — Error bodies of CLI --json, MCP results and the channel faults hold no absolute path, upstream caused-by text or serde path, and every product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — thiserror messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
decision/fixtures-not-a-telemetry-source · Fixtures not a telemetry source — Recorded fixtures are not a telemetry source, and observability adds no fixture content.
decision/typed-ids-never-in-detail · Ids logged as typed fields — pid, child_pid, agent_session_id and started_at are logged as typed fields, never inside detail codes.
decision/veil-redaction · veil derives redaction — veil 0.3.0's derived Redact is what redacts the payload fields of contract types, and instrumented spans skip all arguments.
decision/canary-only-in-detail-files · Canary only in detail files — Every E2E and integration input is synthetic and embeds a fixed canary, which may appear in detail files but never in a home-level diagnostics file. · guard: mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line
decision/prompt-and-output-never-in-home-logs · Prompt and assistant output never in home logs — Prompt text and assistant output, last_assistant_message included, never appear in a home-level process-log line; they reach an instance detail file only inside a chain or drift report, and only text_bytes is logged.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/g1-bare-instrument-gate · G1 bare-instrument gate fails closed — Every #[instrument] must have skip_all as its first argument; the G1 gate passes only on rg exit 1 (no match), so a violation, a regex or I/O error or a missing rg fails the CI step. · guard: G1
decision/g4-schema-conformance-gate · Schema conformance gate on every line — Every home-level line is validated against the diagnostics line schema and every detail line against the detail schema, in a step that always runs. · guard: G4

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-gates-fail-closed · CI gates fail closed — A gate whose tool is missing, errors or meets an empty scope fails the step rather than passing. · why: an erroring search with a must-be-empty check would pass on every commit · guard: G1, G3, scripts/g2-zero-panics.sh --probe
decision/instrument-requires-skip-all · Instrument attribute must skip all arguments — Every #[instrument] in the workspace takes skip_all as its first argument; a bare #[instrument] fails the build. · guard: gate G1
decision/sync-crates-without-tokio · Sync crates build without Tokio — The synchronous crates must compile without Tokio. · guard: cargo check of the sync crates without Tokio
decision/no-raw-tracing-macros · No raw tracing macros outside the obs module — Raw tracing info, warn, error, debug and trace macros and raw event! are refused outside viola_core::obs. · guard: clippy -D warnings disallowed-macros; raw-event! grep and ban probes in scripts/lint-probes.sh
decision/schema-conformance-gate · Schema conformance gate on real process output — Every role-file line of test homes must validate against schemas/diag-line.v1.json and every detail line against schemas/diag-detail.v1.json; a failing line fails the build, a torn non-JSON line is skipped and counted. · why: schema failures count as a build failure and unit tests check only in-memory lines · guard: gate G4 (schema-conformance step)

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/module-boundary-error-logging · Errors logged once at module boundaries — Every typed error that crosses a crate seam is logged once at WARN or ERROR with a closed detail or error_code; the full chain goes only to the instance detail file.
decision/schema-conformance-failure-fails · Schema-conformance failure is a failure — An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.
decision/deterministic-emission · Deterministic event emission — Events key on hook events and events.ndjson offsets; nothing is derived from screen content. · except: The readiness gate reads the vt100 screen before a paste and can refuse a send as input-not-ready; a local command's echo confirmation keys on its transcript's stdout tag count.
decision/lint-probes-failure-fails · Lint-probes failure is a failure — A lint-probes failure (a ban that did not fire, a control that did not pass, or a raw event! outside the core obs module) counts as an SLO invariant failure.
decision/print-and-macro-lint-failure-fails · Print and macro lint failure is a failure — A clippy disallowed-macros, print_stdout, print_stderr or dbg_macro failure counts as an SLO invariant failure.

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/claude-env-and-cookie-never-logged · CLAUDE* values never logged — CLAUDE* values are never logged, at any level, in any file, detail files included.
decision/machine-asserted-telemetry-verdicts · Machine-asserted telemetry verdicts — Log analysis is never human-review-gated; every CI telemetry verdict is a jq or hyperfine assertion that sets an exit code.
decision/no-instrument-err-ret-on-sensitive · No instrument err or ret on sensitive results — #[instrument(err)] and #[instrument(ret)] are never used where the error or return can carry a serde or serde_path_to_error source or user content.
decision/no-paths-or-pids-in-detail-codes · No paths or pids in detail codes — detail codes never contain paths or pids.
decision/no-pii-in-span-fields · No PII or content in span fields — Span fields never include PII or content. · why: Scrubbing at an exporter stage is too late, and v1 has no exporter stage at all.
decision/no-serde-source-in-error-output · No serde source in printed errors — An anyhow chain that still contains a serde_json or serde_path_to_error source is never printed to stderr, --json, MCP isError or channel error.data.
decision/no-telemetry-from-screen-content · No telemetry from screen content — Telemetry is never derived from screen content; vt100 is used for readiness and modal gate outcomes only.
decision/no-telemetry-on-stdout · No telemetry on stdout — Telemetry is never written to stdout on any role. · why: stdout carries each role's own output and run's terminal carries the child's screen.
decision/paths-only-in-instance-detail-files · Paths only in instance detail files — Paths appear only in instance detail files.
contract/stdout-by-role/run · stdout by role · run — The terminal of run carries the child's screen.
decision/closed-event-and-corr-on-every-line · Closed event enum on every line, corr where correlated — Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
decision/corr-set-explicitly-by-caller · corr set explicitly by the caller — The caller sets corr explicitly as a typed field on the events that carry one.
decision/explicit-corr-conn-instance · corr, conn and instance passed explicitly — corr, conn and instance are passed as explicit values across threads or tasks, never carried by span context. · why: Spans are not serialized.
decision/field-allow-list-default-deny · Field allow-list is default-deny — No field is logged unless named in the field catalog.
decision/low-cardinality-span-names · Low-cardinality span names — Span names never contain instance names, dialog ids or paths.
decision/no-bare-instrument · No bare #[instrument] — Instrumentation always uses #[instrument(skip_all, fields(...))], never a bare #[instrument]. · why: A bare #[instrument] records every argument with Debug and leaks send text, hook.dialog payloads and tool input.
decision/no-print-macros · No print macros in product code paths — print!, println!, eprint!, eprintln! and dbg! are never used in the hook, run or mcp code paths of the product crates, and every product member inherits the workspace lints. · why: They bypass the file sink and break the Hook contract, the run terminal rule and MCP framing. · guard: clippy print_stdout, print_stderr and dbg_macro at deny · except: viola-harness, the fake agent and the output modules that own --json, human CLI stderr and the hook decision body.
decision/no-stringified-corr · corr never recorded with ? or % — corr is never recorded with ?corr or %corr. · why: Those stringify numbers.
decision/obs-event-macro-only · obs_event! is the only event macro — tracing's event and level macros are never called directly outside viola_core::obs; events go through obs_event!. · why: Only obs_event! guarantees event, process and instance. · guard: clippy disallowed-macros; the raw-event! grep in lint-probes
decision/scrub-before-bytes-leave-process · Scrubbing before any byte leaves the process — Every byte is scrubbed of secrets before it leaves the process; in v1 the only destination is local files. · guard: the secret-scan test

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/content-only-in-instance-detail-file · Content only in instance-scoped detail files — Content-bearing detail goes only to the instance's detail-<process>.ndjson file; home-level log files are codes-only and carry no user content, token or CLAUDE* value. · why: security allows user-derived content only in instance-scoped diagnostics
decision/fixed-error-display · Fixed error Display text — Typed errors have fixed Display text that carries no interpolated content.
decision/instrument-skip-all-everywhere · Spans skip all arguments — Every instrumented function uses skip_all with explicit fields, so no argument is recorded implicitly. · guard: G1
decision/obs-event-single-macro · One obs_event macro for every line — Every log line is emitted through the one obs_event macro defined in viola-core, which expands to tracing's event at the caller, so viola-core stays I/O-free and tracing-free.
decision/raw-tracing-ban · Raw tracing macros banned — The tracing level macros are banned by clippy disallowed-macros and raw event calls by a fail-closed grep outside the obs module, so only obs_event emits lines. · why: clippy reports a disallowed macro expanded inside an exported macro at the caller, so no allow inside obs_event can exempt it · guard: clippy disallowed-macros and scripts/lint-probes.sh

## log-file-location · key of ## 3. Observability Harness Contract
contract/diagnostics-files/paths · diagnostics files · process log paths — Process logs are written to <home>/diagnostics/ as run-<name>, hook-<name>, mcp and cli-<name> .ndjson files, and detail to <home>/instances/<name>/diagnostics/detail-<process>.ndjson.

## log-format-json-schema · key of ## 3. Observability Harness Contract
decision/obs-aligns-to-tests-log-format · Observability aligns to the tests log format — The log format is bound by the test plan's harness contract; observability may add fields but must not rename them or remove one the product still emits. · why: the harness greps on them
contract/corr-by-event/process-and-panic · corr by event · process-start, process-exit, panic — corr is null.
contract/process-log-line/message · the process log line · message — The message is a static literal equal to the event name and never carries interpolated content.
contract/process-log-line/process · the process log line · process — One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing cli-<name>.ndjson.

## logging-stack · key of ## 3. Observability Harness Contract
decision/file-only-diagnostics-sinks · File-only diagnostics, one file per role — Every role logs only to its own file under the home's diagnostics directory; no role has a stdout sink, and there is no pretty or dev sink.
decision/no-content-in-home-level-lines · No content fields in home-level lines — No home-level diagnostics line carries chain, drift_report, panic_payload or backtrace. · guard: G4 diag-line.v1.json schema
contract/diagnostics-line/role-file-line-size · the diagnostics line · role-file line size — A line in a role file (run, hook, mcp, cli) is codes-only and stays well under 4 KiB.
decision/level-changes-volume-only · Level changes volume only — Every diagnostics level uses the same field allow-list, so the level changes only how many lines are written. · why: No setting can disable redaction.
decision/no-internal-error-fallback · No stderr fallback for subscriber internal errors — The diagnostics subscriber never reports its own internal errors by printing to stderr. · why: The default falls back to eprintln, which would break the Hook contract and the run terminal rule.
decision/no-log-crate-bridge · No log-crate bridge — The log-crate bridge (tracing-log, LogTracer) stays off; failures are captured from returned values or handles, and a gap is fixed at the seam or by the researched json-subscriber fallback, never by enabling the bridge. · why: The accepted loss is only third-party log context emitted without a returned error. · guard: pty_exit_is_read_on_the_handle_while_the_output_is_held
decision/no-source-paths-in-logs · No source paths in diagnostics lines — Diagnostics lines carry no source file names or line numbers.
decision/no-span-list-in-lines · No span list in diagnostics lines — Diagnostics lines never carry the list of entered spans. · why: Closes the spans leak path.
contract/diagnostics-files/detail · the diagnostics files · detail — Content-bearing records (full error chains, drift reports, panic payload and backtrace) go only to <home>/instances/<name>/diagnostics/detail-<process>.ndjson, file 0600 in a 0700 directory, and only when an instance resolves.
contract/obs-event-macro/attached-fields · the obs_event macro · attached fields — Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.

## otel-sdk-init · key of ## 3. Observability Harness Contract
decision/diagnostics-init-before-role-work · Diagnostics init before the role's work — Diagnostics init and the process-start line come before the role's work: in run before its collision check and the child spawn, in mcp before its stdio transport starts. · why: Exit-1 causes are logged.
decision/home-tree-check-before-log-init · Home tree check before log init — run, revive's start arm, adopt's start and viola verify check the home tree's modes before their log opens; a refusal writes no line, starts no child, leaves modes as found and exits 1.
decision/strict-modes-before-diagnostics · Home strict-modes check before diagnostics — Nothing under the home, the diagnostics directory included, is created or opened before the role's strict-modes check on the home passes; a refused home gets no diagnostics line. · except: viola up's home-tree check before its first spawn runs after its board scan has opened its log, and a refusal there ends that log with process-exit detail strict-modes-failed.
decision/tracing-is-the-instrumentation-facade · tracing as the instrumentation facade — tracing is the instrumentation facade in every instrumentable crate, and tracing-subscriber is the formatter and sink, a product dependency of the root bin only.
decision/instance-source-per-role · Instance source fixed per role — run takes its own name argument, never an inherited VIOLA_NAME; hook and mcp take VIOLA_NAME only; cli verbs that act on an instance (send, wait, last, answer, pause, release, link, unlink) take the target argument; list, verify, plugin install and revive --list take the caller's VIOLA_NAME only. · why: A wrapper never writes into another instance's run log.

## service-identity · key of ## 3. Observability Harness Contract
decision/env-vars-not-configuration · Environment variables are not configuration — Environment variables are not a configuration channel: service identity has no env-var source or fallback, and the one CLAUDE_CONFIG_DIR value read by verify and the run, revive and adopt start, when kept, locates the claude config file, is never logged and configures nothing of viola.

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
