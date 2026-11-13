# obs-plan · the rows that bind 2026-11-13-epoch-8-cleanup — picked 313 of 1059 (208 by every draw) · rule any · pin 885

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/absolute-paths-scrubbed-externally · Absolute paths scrubbed from external output — Absolute paths in snapshot.json (endpoint, pinned_bin) and in config values are scrubbed from external output and allowed in diagnostics/.
decision/diagnostics-owner-only-modes · Diagnostics files are owner-only — Every diagnostics file is mode 0600 inside a 0700 directory.
decision/distinct-detail-code-per-exit-cause · Distinct detail code per exit cause — Each cause behind exit 21 and exit 1 gets a distinct detail code with no path or pid. · except: A strict-modes check refused before any log opens writes no line; its cause is the fixed stderr pair or the --json document.
decision/error-chain-fixed-stderr-message · Error chains reach stderr as fixed messages — In the viola root binary, the only crate that uses anyhow, an error chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to the diagnostics logs only.
decision/event-payload-not-in-process-logs · Event payloads kept out of process logs — The payload content of events.ndjson is never copied into process-log lines. · why: events.ndjson is itself the product audit stream.
decision/exit-1-detail-codes · Distinct detail codes for exit-1 causes — On the `viola run` start sequence to child spawn, each exit-1 cause logs a distinct `detail` code with no path or pid in it.
decision/hook-fail-open-silent · Hook fails open silently — Hook fail-open paths log only to `hook-<name>.ndjson` and exit 0 with stderr empty.
decision/hook-log-write-failure-swallowed · Hook log write failure swallowed — A failed write to the hook log is swallowed, so the hook still exits 0 and never writes to stderr.
decision/hook-logger-init-no-blocking-io · Hook logger init adds no blocking I/O — Logger initialisation in viola hook reads the diagnostics level, runs the strict-modes checks, creates or narrows the home and diagnostics/ to 0700 and opens the role file once for append at 0600, adding no other I/O.
decision/hook-silent-no-op-without-instance · Hook silent no-op without an instance — Without VIOLA_NAME, on an instance the instance check refuses, or on a VIOLA_DIR naming an instance directory that does not exist, viola hook is a silent no-op: exit 0, no file opened, no line, no frame.
decision/hook-stderr-forbidden · No stderr from hooks — viola hook writes nothing to stderr.
decision/hook-stdout-decision-body-only · Hook stdout carries the decision body only — viola hook stdout carries only the decision body; for viola hook statusline, the output of the user's status-line command.
decision/mcp-log-file-sink · MCP logs to a file sink, never stdout — The `mcp` process always logs to the file sink `<home>/diagnostics/mcp.ndjson`, with no stderr fallback: a file that cannot be opened exits 1 printing nothing, and the log writer is set explicitly. · why: stdout is forbidden because it carries MCP frames
decision/refusal-telemetry · Refusal telemetry for filesystem refusals — Pinned-exe hash mismatches and ignored symlinks produce refusal telemetry.
decision/serde-errors-fixed-message · Serde error chains become a fixed message — A serde-sourced anyhow error chain is replaced by a fixed message before it reaches stderr.
decision/stderr-fixed-strings · Fixed strings on stderr — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/strict-modes-failure-lines · Where a strict-modes failure is logged — A strict-modes failure has a log line only on the `ledger-stamps` read in the `run` version gate and on `verify`'s own `process-exit` for a refused stamp write.
contract/diagnostics-files/hook · the diagnostics files · hook — Role hook writes <home>/diagnostics/hook-<name>.ndjson, shared by concurrent hook processes; a write failure is swallowed.
contract/diagnostics-files/mcp · the diagnostics files · mcp — Role mcp writes its role file mcp.ndjson in the home's diagnostics folder, which never carries user content; a role file that cannot be opened ends the server with exit 1 and nothing on stderr.
contract/hook-log-file/path · the viola hook log file · path — viola hook logs only to a file sink at diagnostics/hook-<name>.ndjson.
contract/mcp-stdin/max-frame · viola mcp stdin · MAX_FRAME — Stdin is read in lines of at most `MAX_FRAME`; a longer line is refused whole and the session ends as at a closed stdin.
contract/process-exit-detail/strict-modes-failed · process-exit detail codes · strict-modes-failed — Carried by `verify`'s `process-exit` for a refused stamp write, with exit 1.
decision/cli-file-sink-only · Short-lived CLI logs to a file sink only — Short-lived CLI verbs log only to a JSON-per-line file sink in diagnostics when an instance resolves, never to stdout. · why: Stdout carries the --json results.
decision/hook-log-line-single-append-write · One write per hook log line — Each viola hook log line is one write on an append-mode handle.
decision/mcp-stdout-frames-only · MCP stdout carries frames only — No telemetry from the viola-mcp server may go to stdout. · why: Stdout is reserved for MCP frames.
decision/no-traceparent-on-local-socket · No W3C traceparent on the local socket — Calls from CLI, MCP and hook to the wrapper carry no W3C `traceparent`; correlation uses `corr` (JSON-RPC `id`, `dialog_id`, send `cursor`) plus `process` and `instance`. · why: channel `params` declare no trace-context field
decision/parse-failures-unknown-counter · Parse failures surface as unknown plus a counter — A child-output parse failure surfaces as `unknown` plus a counter.
decision/refused-diagnostics-drops-line · Refused diagnostics file drops its line — A refused `diagnostics/` folder or file is treated as a failed open and its log line is dropped.
contract/process-exit-detail/socket-dir-refused · process-exit detail codes · socket-dir-refused — Carried by the line of a refused socket directory, where it has one; a refused socket directory is no strict-modes failure.

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/diagnostics-file-modes · Diagnostics file and directory modes — Diagnostics files are 0600 inside 0700 directories. · guard: secret-scan test
decision/diagnostics-owner-only-permissions · Diagnostics owner-only permissions — Files under diagnostics/ are 0600 inside 0700 directories. · guard: secret-scan test
decision/fixed-stderr-messages · Fixed stderr refusal and fault lines — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/hook-fail-open-logged-to-hook-file · Hook fail-open logged to the hook file — Hook fail-open paths (oversize stdin, malformed JSON, channel failure, server-verify mismatch, refused socket directory) are logged to hook-<name>.ndjson only; a missing VIOLA_NAME writes no line.
decision/hook-instance-check-before-obs-init · Hook instance check before logging starts — The hook's event and dialog paths run the instance strict-modes check before reading the diagnostics level and initialising logging. · except: The two --capture arms run ahead of the check.
decision/hook-lines-required · Hook invocation and decision lines required — Every hook process in a trusted wrapped session writes its hook-invoked and hook-decision lines, bar a PreToolUse for a tool outside the dialog matcher; a hook outside such a session or on a --capture probe writes neither.
decision/hook-log-failure-swallowed · Hook log write failure swallowed — A failed log write in a hook is swallowed and the hook still exits 0.
decision/hook-mcp-instrumented-at-boundary · Hook and MCP instrumented at the boundary — Hook and MCP traffic is instrumented only at viola's own boundary: hook-invoked and hook-decision, MCP tool-call handling, and the viola mcp stdin cut. · why: The caller is Claude Code, so viola sees only the inbound stdin payload and its own response, and no trace context arrives from Claude Code.
decision/hook-silent-without-trusted-instance · Hook silent without a trusted instance — A hook with no VIOLA_NAME, or whose instance is absent or refused by its strict-modes check, exits 0 with no config read, no stdin read, no log line, no stderr, no frame and no SessionEnd append.
decision/linux-only-os-target · Linux-only OS target — The OS target is Linux only: the core on any current distribution, CI on ubuntu-latest.
decision/log-file-basenames-follow-tests · Log file names follow tests — Process-log files are <home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson, as tests require. · why: Tests is upstream; arch and security place diagnostics under the instance directory instead.
decision/mcp-log-sink · MCP log sink — viola mcp always logs to <home>/diagnostics/mcp.ndjson once the home and its diagnostics/ pass strict-modes; a file that cannot be opened exits 1 printing nothing, with no stderr fallback.
decision/messaging-secrets-never-logged · Messaging secrets never logged — CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every R8-stripped CLAUDE* value are never logged in any sink, detail files and fixtures included. · guard: secret-scan test
decision/no-secrets-in-log-lines · No secrets in log lines — No log line may contain a Cookie header or any stripped CLAUDE* value. · guard: secret-scan test
decision/per-parser-parse-failure-counts · Per-parser parse-failure counts — Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead.
decision/pinned-copy-race-adds-no-line · Pinned-copy race adds no telemetry — A pinned-exe hash mismatch reached when a concurrent first start wins the no-clobber placement leaves the file as found and adds no line, field or detail code beyond pinned-hash-mismatch.
decision/strict-modes-refusal-writes-no-line · Strict-modes refusal writes no line — A strict-modes check refused before any log opens ends the process with nothing under the home opened and no log line; its cause is carried only by the fixed stderr pair or the --json document. · why: Ratified as the answer to whether a refused strict-modes check logs. · except: The ledger-stamps read in run's version gate and a refused update_stamps in viola verify each leave a line.
decision/stripped-claude-values-never-logged · Stripped CLAUDE values never logged — R8-stripped CLAUDE* values are never logged anywhere. · why: Its outputs are the most sensitive in the product.
contract/cli-frame-verbs/strict-modes-failed · CLI frame verbs · strict-modes-failed — A strict-modes refusal of send, wait, last, answer, pause or release writes no line and prints only the fixed instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, exit 21, with no pid, path or endpoint.
contract/cli-json-error/strict-modes-failed · CLI --json error document · instance-unreachable strict-modes-failed — A refused instance check of send, wait, last, answer, pause, release, link or unlink exits 21 with the instance pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
contract/cli-log-file/link-unlink · CLI process-log file · viola link / viola unlink — Log as cli into the driven instance's cli-<driven>.ndjson, never the caller's VIOLA_NAME, once their instance check of it has passed.
contract/dialog-hook/refused-instance-check · Dialog hook · refused instance check — A refused instance check on the dialog hook exits 0 at once with empty streams, no frame and no line.
contract/dialog-log-line/server-verify-failed · Dialog log line · server-verify-failed — A server-verify failure exits 21 on CLI answer with during connect, and on the dialog hook fails open with corr null and decision_emitted false; the line names no pid, start time, path or endpoint.
contract/dialog-log-line/socket-dir-refused · Dialog log line · socket-dir-refused — A refused socket directory exits 21 on CLI answer with during connect, and on the dialog hook fails open with corr null and decision_emitted false; the line names no pid, start time, path or endpoint.
contract/dialog-path-log-events/event-order · Dialog path log events · event order — A dialog logs hook-invoked, dialog-raised, dialog-answered, then hook-decision, whose decision is null on deadline expiry.
contract/event-line/shape · events.ndjson event line · line shape — Exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
contract/exit-21-detail/strict-modes-failed · Exit 21 detail codes · strict-modes-failed — strict-modes-failed is a distinct detail code for an exit-21 cause, naming no path or pid.
contract/hook-decision/server-verify-failed · Hook decision line · server-verify-failed — A server-verify mismatch is one hook-decision at WARN with detail server-verify-failed and no frame sent; on a dialog hook, hook-invoked with corr null then hook-decision with corr null and decision_emitted false; no pid, start time, path or endpoint.
contract/hook-decision/socket-dir-refused · Hook decision line · socket-dir-refused — A refused socket directory on viola hook writes one hook-decision in the server-verify form with detail socket-dir-refused, and the hook exits 0.
contract/hook-fail-open/exit · Hook fail-open · exit — A hook that fails open exits 0 and leaves stderr empty.
contract/hook-log-line/duration-ms · Hook log line · duration_ms — duration_ms is on hook-decision, or on process-exit for process hook, for each hook event.
contract/hook-stdout/hook-event · Hook stdout · viola hook <event> — stdout carries only the decision body.
contract/mcp-frame-tool-refusal/strict-modes-failed · MCP frame tool refusal · strict-modes-failed — A frame tool (send, wait, last, answer) whose instance check refuses returns instance-unreachable with detail strict-modes-failed, writes no frame and no line.
contract/mcp-iserror/instance-unreachable · MCP isError result · instance-unreachable — The instance-unreachable detail is one closed cause: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed or socket-dir-refused; the CLI's document less v, its text `refused: instance-unreachable`.
contract/mcp-stdin/line-within-max-frame · viola mcp stdin · line within MAX_FRAME — A line of at most MAX_FRAME bytes, its newline counted, passes whole.
contract/mcp-stdin/oversize-before-initialize · viola mcp stdin · oversize line before initialize — An oversize line before initialize exits the process 1 with both streams empty and the one parse-rejected record.
contract/process-log-corr/hook-decision · Process log corr by event · hook-decision — corr is the dialog_id for dialog hooks, otherwise null.
contract/process-log-corr/hook-invoked · Process log corr by event · hook-invoked — corr is the dialog_id for dialog hooks, otherwise null.
contract/replayed-snapshot/live-fields · A snapshot.json rebuilt by replay · endpoint, pid, started_at, statusline_command — A snapshot.json rebuilt from events.ndjson carries none of these, so the endpoint reads unreachable, hook connect fails open with channel-unreachable, hook statusline prints nothing, and run's collision check reads the heartbeat alone.
contract/unreachable-cause/strict-modes-failed · exit 21 cause words · strict-modes-failed — One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
contract/viola-link-unlink/strict-modes-failed · viola link and unlink output · strict-modes-failed — A strict-mode fail of the driven instance's check writes no log line; it prints the fixed instance-state unable and hint: pair on stderr, or under --json {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, and exits 21.
contract/viola-revive/strict-modes-failed · viola revive output · strict-modes-failed — A strict-mode fail of viola revive writes no log line; it prints the stderr pair, or under --json {"v":1,"error":"strict-modes-failed","detail":null}, and exits 1.
contract/viola-verify/home-tree-refused · viola verify · refused home-tree check — A refused home-tree check exits 1 with the home pair, opens no log and writes no line; a refused update_stamps exits 1 with the home pair and, when an instance is named, ends the opened log with a process-exit line carrying detail strict-modes-failed.
decision/channel-log-scrub-rule · Channel log scrub rule — Channel request and response lines log only the method, id or corr, the result class (ok, refusal or error code), the sender version and the closed detail word, never params or result bodies.
decision/detail-codes-no-paths · Detail codes carry no path or pid — No path or pid appears in a detail code.
decision/diagnostics-checked-at-open · Diagnostics checked before open — Before a role or detail file is created or narrowed, the existing diagnostics/ folder and the file are checked, a symlink refused and no group or other bit allowed; a refusal is a failed open.
decision/fault-telemetry-per-class · Fault telemetry per class — Each fault class emits its own telemetry: a healed torn ndjson line with file and offset, a corrupt or unsupported snapshot recovered by replay, a child exit seen without EOF, an endpoint vanished during wait, and a stale heartbeat against a live pid.
decision/hook-never-writes-stderr · Hook never writes stderr — A hook process never writes to stderr.
decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but the required ones are never renamed or removed. · why: The harness greps on them.
decision/mcp-no-telemetry-on-stdout · No telemetry on the MCP server's stdout — The MCP server sends no telemetry to stdout. · why: stdout is reserved for MCP frames.
decision/mcp-results-codes-only · MCP rejections carry codes only — MCP isError results and the log lines of rejected calls carry codes only.
decision/no-process-log-without-instance · No process log without an instance — A short-lived CLI verb run with no resolved instance writes no process-log file; its human stderr carries only fixed-message lines, and its outcome is the exit code and stdout.
decision/parse-failures-surface-as-unknown · Parse failures surface as unknown — A parse failure surfaces as the value unknown plus a counter.
decision/process-init-order · Process logging init order — Each process installs its panic hook, parses arguments, resolves home and instance and runs its strict-modes check, opens its diagnostics file append-mode 0600 in a 0700 directory, installs the subscriber, then emits process-start. · why: For run this completes before the start sequence so the collision check and version gate are logged; for mcp, before the stdio transport starts.
decision/serde-chain-fixed-message · Serde error chains become a fixed message on stderr — An error chain that contains a serde source becomes a fixed message before it reaches stderr.
decision/serde-errors-become-fixed-message · Serde errors become a fixed message — An error chain holding a serde source is replaced by a fixed message before it reaches stderr.
contract/budget-path-log-events/event-set · Budget path log events · event set — The budget path logs hook-invoked and hook-decision for statusline, and send-refused with budget-paused.
contract/channel-response/detail · Channel response line · detail — On a channel-response answering -32602, detail is one closed word: unsupported-version, invalid-params or release-from-driver, on both the server's and the client's line; it is absent on every other channel-response line and names no param or value.
contract/channel-response/invalid-params-causes · Channel response line · invalid-params causes — detail invalid-params covers send's invalid params, and a link or unlink whose driver or driven is missing, not a string or no valid name, whose driven is not the serving wrapper's own name, or whose driver equals driven.
contract/cli-internal-error/stderr-line · CLI internal error · stderr line — Without --json a catch-site fault prints the fixed line `error: internal error` to stderr.
contract/cli-log-file/revive-list · CLI process-log file · viola revive --list — Logs as cli into the caller's cli-<VIOLA_NAME>.ndjson once its strict-modes check has passed; no file with no VIOLA_NAME.
contract/exit-1-detail/pinned-hash-mismatch · Exit 1 detail codes · pinned-hash-mismatch — pinned-hash-mismatch is a distinct detail code for an exit-1 cause, naming no path or pid.
contract/exit-cause-detail/server-verify-failed · Exit cause detail codes · server-verify-failed — A failed server check of peer euid, peer pid or start time is logged by the CLI and the hook as server-verify-failed; MCP writes no line for it.
contract/hook-stdout/capture-arms · Hook stdout · --capture arms — The two hidden --capture arms write no log sink and print nothing, except --answers' probe decision body.
contract/json-rpc-error/32603 · JSON-RPC error · -32603 — The -32603 error carries the message "internal error" with data null.
contract/mcp-iserror/wrapper-fault · MCP isError result · wrapper-fault — The wrapper-fault detail is a closed sanitised object: the fixed message of the five codes with data null or closed values; a code outside the five reads message "wrapper fault", data null.

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
contract/process-log-line/at-instants · the process log line · *_at — Instants are named *_at and written in RFC 3339 UTC with milliseconds and a Z suffix.
contract/process-log-line/enum-values · the process log line · enum values — Enum values are kebab-case.
contract/process-log-line/event · the process log line · event — The event field is a closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
contract/process-log-line/field-names · the process log line · field names — Field names are snake_case.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/hook-silent-without-instance · Hook silent without an instance — A hook with no VIOLA_NAME, or with a refused or absent instance, exits 0 at once and writes no file, no line and no frame.
decision/pre-open-strict-modes-refusal-silent · Strict-modes refusal before a log opens writes no line — A strict-modes refusal made before any process log opens writes no line and carries no logged detail code; its machine-readable form is the exit code and, under --json, its closed document. · except: a refused socket directory always logs detail socket-dir-refused, except at MCP's frame tool, which writes no line
contract/strict-modes-refusal-exit/1 · strict-modes refusal exit codes · 1 — run, revive's start and verify exit 1 on a strict-modes refusal; revive under --json prints the strict-modes-failed document.
contract/strict-modes-refusal-exit/21 · strict-modes refusal exit codes · 21 — The six frame verbs and link / unlink exit 21 on a strict-modes refusal, under --json with the instance-unreachable document carrying detail strict-modes-failed.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
contract/hook-output/stdout · hook output streams · stdout — A hook writes only its decision body to stdout, and no telemetry reaches stdout.
contract/run-terminal-output/start-refusal · run terminal output · start refusal — A start refusal before any child is spawned writes exactly two fixed stderr lines with no path or pid: the home pair for a home-level refusal, the instance pair for an instance-level one.
contract/hook-output/stderr · hook output streams · stderr — A hook writes nothing to stderr.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.
decision/ledger-contents-never-logged · Capability ledger path and stamps never logged — The capability ledger's path and its stamp contents never appear in a log line.
decision/link-has-no-own-trace · Link and unlink have no trace of their own — Implicit and explicit link / unlink have no process-log line, event value or span of their own; their trace is the call's channel-request / channel-response pair, the product link / unlink line in events.ndjson and the snapshot's links. · guard: edge test E3
decision/no-endpoint-over-replayed-snapshot · No endpoint over a replayed snapshot — Over a replayed or absent snapshot read, endpoint lookup and the hook connect give no endpoint, so no frame is sent, and the statusline runs no command. · guard: edge test E5
decision/unreadable-ledger-is-unverified · Unreadable ledger counts as unverified — A missing stamp, or an unreadable, malformed or strict-modes-refused capability ledger, counts as unverified (cli_verified false).
decision/unwrapped-hook-no-op · Unwrapped hook is a no-op — A hook run without VIOLA_NAME writes no file and no line, exits 0 and leaves stdout and stderr empty. · guard: edge test E1, asserted by absence
contract/cli-wait-last-connect/strict-modes-failed · CLI wait / last connect refusal · strict-modes-failed — A home or instance tree refused by the instance check, checked first, ends the call before any role file opens with exit 21 and no line; under --json the output is exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
contract/mcp-wait-last-connect/strict-modes-failed · MCP wait / last connect refusal · strict-modes-failed — A home or instance tree refused by the instance check returns instance-unreachable with detail strict-modes-failed and writes no line.
contract/state-recovered-detail/snapshot-replayed · state-recovered detail · snapshot-replayed — A state-recovered line at WARN carries this detail, one line per snapshot read call.
contract/state-recovered-detail/snapshot-unsupported-v · state-recovered detail · snapshot-unsupported-v — A state-recovered line at WARN carries this detail, one line per snapshot read call.
contract/viola-verify-refusal/home-tree-refused · viola verify strict-modes refusal · home tree refused — Before logging, verify holds the home, ledger/, bin/ and plugin/ to the strict-modes check; a refused tree exits 1 with the home pair and writes no line.
contract/viola-verify-refusal/stamp-write-refused · viola verify strict-modes refusal · stamp write refused — A ledger/stamps.json the stamp write refuses prints the home pair and exits 1 through verify's own process-exit line.
decision/detail-codes-only · detail is a closed code — The detail field is a closed kebab-case code catalog and never carries a path or pid.
decision/link-verbs-log-to-driven-instance · Link verbs log to the driven instance — The viola link and viola unlink verbs log into the driven instance's cli-<driven>.ndjson, their request inside the cli.link / cli.unlink span.
decision/verify-lines-redacted · verify lines carry no content — No verify or plugin install line carries row text, a prompt, a session id or a path.
decision/verify-logs-only-with-instance · verify and plugin install log only inside an instance — With VIOLA_NAME set, verify and plugin install log process-start / process-exit to cli-<name>.ndjson; from a plain terminal with no instance they write no process-log file.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/dialog-hook-fails-open · Dialog hook fails open on a failed check — A dialog hook whose instance, socket-directory, euid or server check fails emits no decision and sends no frame, so the dialog falls back to the human.
decision/dialog-hook-invoked-after-response · Dialog hook-invoked written after the response — A dialog hook writes hook-invoked only once dialog_id is known, after the hook.dialog response; non-dialog hooks and fail-open paths before dialog_id write hook-invoked{corr:null} at once. · why: invoked_at still records the hook's true start, and the wrapper's dialog-raised gives live visibility during the wait.
decision/expired-dialog-renders-for-human · Expired dialog returns to the human — At the hold's end the reply stays null, the hook writes hook-decision with decision_emitted false, exits 0 with empty stdout, and the dialog renders for the human.
decision/home-tree-check-before-start-log · Home-tree check before the start's log — The start of run and of viola revive checks the home tree (the home, instance set, ledger, bin, plugin, pinned exe and plugin hooks) before its log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
decision/hook-always-exits-zero · viola hook keeps exit 0 — viola hook exits 0 even when its dispatch panics: a panic around dispatch is caught.
decision/hook-refused-instance-silent · Refused instance makes the hook silent — On the event and dialog paths the instance check runs before the diagnostics level and log init; a refused or unresolved instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append.
decision/implicit-link-on-send-and-answer · Implicit link recorded by send and answer — An issued send or a taken answer whose from names an instance other than the wrapper records the link, one link product line for a new pair; a refused call, one with no from, or one under the wrapper's own name records nothing.
decision/implicit-link-untraced · Implicit link has no line or span — The implicit link record has no process-log line, no event value and no span; its trace is the call's channel pair, the link product line and the snapshot's links.
decision/link-methods-untraced · Explicit link methods open no span — The explicit link and unlink methods open no span and write no process-log line of their own; their channel lines hold no driver, driven or other param.
decision/link-names-not-in-process-log · Link names stay out of process logs — The driver and driven names reach no process-log line.
decision/mcp-explicit-log-writer · Explicit log writer in viola mcp — The viola mcp log writer is always explicit: the mcp.ndjson file, with no stderr fallback.
decision/no-obs-gate-on-verify-lines · No obs gate depends on verify lines — No observability gate depends on verify's own log lines; its home-level lines fall under the same panic, schema and secret-scan gates as any other.
decision/no-raw-snapshot-reads · Product readers read the snapshot through replay — Every product reader of the snapshot uses the replaying read; no product reader reads the snapshot raw.
decision/pinned-copy-never-replaced · Pinned copy placed, never replaced — An absent pinned copy is placed, never replaced; a start that loses the placement re-hashes the copy that stands, reusing an equal one and refusing a different one as pinned-hash-mismatch with the file left as found.
decision/real-cli-verify-never-in-ci · Real-CLI verify never runs in CI — In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
decision/refused-check-writes-no-line · A refused check opens no process log — A cli verb whose strict-modes, tree or instance check refuses (exit 21) opens no process log and writes no line.
decision/refused-home-tree-writes-no-line · Refused home tree writes no line — verify and plugin install check the home tree before their log opens, and a refused tree opens no log and writes no line.
decision/replayed-snapshot-unreachable · Replayed snapshot is unreachable — Only a snapshot read whole leads to a connect; a replayed or absent snapshot is treated as unreachable and no frame is sent. · why: A replayed snapshot carries no endpoint, pid or started_at.
decision/statusline-silent-without-instance · Statusline arm silent without a valid instance — The statusline arm writes no line without a valid VIOLA_NAME or when the instance check refuses: it returns exit 0 at once with no role file opened.
decision/unreadable-ledger-means-unverified · Unreadable capability ledger means an unverified CLI — A capability ledger that is missing a stamp, unreadable, malformed or refused by the strict-modes check is treated as an unverified CLI (cli_verified false).
contract/cli-stderr/refusal-pair · CLI stderr · unable and hint pair — A refusal is printed to stderr as a fixed "unable:" line followed by a "hint:" line.
contract/dialog-log-fields/hook-lines · dialog log fields · hook-invoked and hook-decision — hook-invoked{corr:<dialog_id>, hook_event, invoked_at}, then hook-decision{corr:<dialog_id>, hook_event, decision_emitted, deadline_hit, detail, duration_ms}.
contract/events-log/link · events log · link — link appends one link product line {driver, driven} (source wrapper) for a new pair and nothing for a pair the set holds, and answers after its line and the snapshot write.
contract/events-log/link-before-send-issued · events log · implicit link and send-issued — A new pair's link product line stands directly before that send's send-issued, whose cursor is the offset its own line starts at; a link record that fails answers -32603 with nothing issued, nothing typed and no send-issued.
contract/events-log/unlink · events log · unlink — unlink appends one unlink product line {driver, driven} (source wrapper) only when it removes a pair, nothing for an absent one, and answers after its line and the snapshot write.
contract/hook-decision-detail/socket-dir-refused · hook-decision line · socket-dir-refused — A refused socket directory writes one hook-decision{detail:"socket-dir-refused"} at WARN, on the dialog path after hook-invoked{corr:null}, with decision_emitted:false; exit 0, empty streams, no frame.
contract/hook-decision/deadline-hit · dialog hook-decision line · deadline_hit — The hook writes deadline_hit:true only when its own read times out at its backstop bound, DIALOG_HOLD_MAX + 5 s (3605 s).
contract/instance-unreachable-json/strict-modes-failed · cli --json refusal document · strict-modes-failed — A refused instance check exits 21 with the instance pair on stderr, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, and sends no frame.
contract/link-methods/invalid-params · link and unlink methods · -32602 — A missing, non-string or invalid name, a driven that is not the serving wrapper's own name, or a driver equal to driven is -32602 with data null and no line; a record or remove error is -32603; neither method reads from.
contract/mcp-instance-unreachable/strict-modes-failed · viola mcp frame tool refusal · strict-modes-failed — A refused instance check, run before the live endpoint is read, returns instance-unreachable with detail strict-modes-failed, no frame and no line.
contract/mcp-refusal-text/instance-unreachable · viola mcp wait and last refusal · strict-modes-failed body — MCP wait and last refused by the instance check return the cli document less v (error instance-unreachable, detail strict-modes-failed), their text refused: instance-unreachable, with no line.
contract/plugin-install-exit-codes/21 · plugin install exit codes · 21 — The strict-modes tree check refused; no log is opened and no line is written, with the stderr pair or, under --json, the instance-unreachable document.
contract/run-log-parse-rejected/ledger-stamps · the run process log parse-rejected line · parser ledger-stamps — An unreadable, malformed or strict-modes-refused capability ledger gives one parse-rejected line with parser "ledger-stamps", detail "unreadable", "malformed" or "strict-modes-failed", and a count, in the run log.
contract/state-recovered/snapshot-replayed · state-recovered line · snapshot-replayed — state-recovered{detail:"snapshot-replayed", file:"snapshot.json"} at WARN with no corr for an unreadable snapshot, one line per call; none for a present or absent snapshot, and none when the replay itself fails.
contract/state-recovered/snapshot-unsupported-v · state-recovered line · snapshot-unsupported-v — state-recovered{detail:"snapshot-unsupported-v", file:"snapshot.json", v_seen} at WARN with no corr for a newer snapshot, one line per call.
contract/statusline-decision-detail/deadline · statusline hook-decision detail · deadline — The shell-out reaching its bound ends in hook-decision{detail:"deadline", deadline_hit:true}.
contract/statusline-decision-detail/malformed-json · statusline hook-decision detail · malformed-json — The statusline hook-decision carries detail malformed-json on that fail-open path.
contract/statusline-decision-detail/oversize-stdin · statusline hook-decision detail · oversize-stdin — The statusline hook-decision carries detail oversize-stdin on that fail-open path.
contract/statusline-log-fields/hook-decision · statusline hook log fields · hook-decision — hook-decision{hook_event:"statusline", budget_written, duration_ms}, deadline_hit only when true; budget_written is true only when the payload held a rate_limits object and the write succeeded.
contract/statusline-log-fields/hook-invoked · statusline hook log fields · hook-invoked — hook-invoked{hook_event:"statusline", corr:null}.
contract/verify-exit-codes/0 · verify exit codes · 0 — Every relied ledger row passed.
contract/verify-exit-codes/1 · verify exit codes · 1 — A row failed, verify refused, or verify was stopped.
contract/verify-process-exit/strict-modes-failed · verify process-exit line · detail strict-modes-failed — A stamps store refused by its strict-modes check, under a parsed VIOLA_NAME, ends with process-exit carrying detail "strict-modes-failed" and the home pair on stderr.
contract/verify-stdout/step-line · verify stdout · step line — One "[NN/MM] <row id> <row words>  pass|fail" line is printed for each relied ledger row, MM being the relied row count.
contract/verify-stdout/summary-line · verify stdout · summary line — The last stdout line is "stamped <version>  N pass  N fail", the two counts summing to MM.
contract/verify-stop-stderr/unable-hint-pair · verify stderr on a stop · the stop pair — A stopped verify prints exactly "unable: verify was stopped before it finished" and "hint: run viola verify again to stamp this CLI" on stderr, with empty stdout and no stamp written.
decision/hidden-capture-arms-uninstrumented · Hidden hook capture arms uninstrumented — The hidden hook capture arms used by verify's probes read no VIOLA_* variable, do no obs init, emit no hook lines, keep stderr empty and always exit 0. · why: uninstrumented by design, as ratified by the founder
decision/instance-check-before-role-file · Instance check before any role file — The frame verbs run the instance check on their named instance (the driven instance for link and unlink) before any role file opens.
contract/answer-link-record/failure · wrapper answer handling · failed link record — A link record that fails answers -32603: the answer is not taken, the dialog stays held and no dialog-answered line is written.
contract/diag-line-schema/os · diag-line v1 schema · os — os is closed to linux.
contract/hook-spans/span-chain · viola hook spans · span chain — viola hook traces hook.handle › channel.request, and writes the hook-decision line inside hook.handle.
contract/refusal-stderr-pair/socket-dir-refused · cli refusal stderr pair · socket-directory refusal — Printed on start and frame-verb refusals in plain ASCII with hint: last and no SGR, naming only the parsed ViolaName.
contract/revive-process-log/agents-probe · viola revive run process log · agents-probe — The session-live reading writes one process-start and process-exit{subject:"agents-probe"} pair, plus one parse-rejected{parser:"claude-agents-json"} on an oversize or malformed answer.
contract/run-start-failure-detail/pinned-hash-mismatch · run start process-exit · pinned-hash-mismatch — process-exit{subject:"self", exit_code:1, detail:"pinned-hash-mismatch"}, with no path or pid.
contract/send-log-fields/hook-invoked · confirmed send log fields · hook-invoked — hook-invoked{hook_event:"user-prompt-submit", corr:null} for an ordinary send; hook_event session-start for a /clear or /compact confirmed by its session; an unconfirmable send waits for no hook line.
contract/statusline-log-fields/parse-rejected · statusline hook log fields · parse-rejected — parse-rejected{parser:"hook-stdin", detail:"oversize"|"malformed", count:1} for stdin the arm rejects.
contract/verify-process-exit/interrupted · verify process-exit line · detail interrupted — A verify stopped by SIGINT or SIGTERM starts no further spawn, closes every pair it opened and ends with process-exit subject self, exit_code 1, detail "interrupted".

## metric-coverage · ## 5. Metric Coverage · twin 5-metric-coverage
decision/panic-lines-stay-zero · Panic lines stay at zero — Gate G2 counts the panic lines of the home-level role files and fails on any; no viola.panic metric is derived.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
contract/log-levels/warn · log levels · warn — WARN carries state-recovered; parse-rejected; hook-decision with a fail-open detail; process-exit with exit 21; channel-response with -32600 or -32602.
decision/file-only-per-role-log-sinks · File-only log sinks per role — Diagnostics are written to files only, one sink per role, under the home's diagnostics directory.
decision/ipc-calls-logged-without-arguments · IPC calls logged without arguments — IPC params and result bodies are never recorded, not even as a hash or digest.
decision/json-to-file-only-log-mode · JSON-to-file as the only log mode — JSON to file is the only log mode: there is no agent-mode flag and no stdout pretty sink on any role.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/unemittable-schema-words-removed · Schema words no build can emit are removed — A closed word of the diag-line schema that no build can emit leaves the schema with its emitter; os is closed at linux.
contract/diag-detail/socket-dir-refused · diagnostic detail words · socket-dir-refused — A closed detail on process-exit and hook-decision for a socket dir refused as a symlink, no directory, foreign owner or mode bit: run and revive's start write process-exit exit 1, CLI frame verbs process-exit exit 21 during connect at WARN, viola hook a hook-decision at WARN; MCP writes none.
contract/diag-detail/strict-modes-failed · diagnostic detail words · strict-modes-failed — Written on the process-exit exit 1 of viola verify under a parsed VIOLA_NAME and on parse-rejected ledger-stamps from the version gate of run, at warn, with no path; every other refused strict-modes check writes no log line, its exit code or JSON document its only record.
contract/diag-line-fields/hook-events · diagnostic line additive fields · hook-invoked / hook-decision — hook_event (one of ten closed hook names incl. statusline), invoked_at, decision_emitted, deadline_hit, budget_written, detail, duration_ms.
contract/diag-line-fields/parse-rejected · diagnostic line additive fields · parse-rejected — parser (claude-agents-json, vt100-feed, config-json, ledger-stamps, hook-stdin, channel-frame, mcp-stdin, state-entry), detail, count.
contract/diag-line-fields/state-recovered · diagnostic line additive fields · state-recovered — detail (torn-line-healed, snapshot-replayed, snapshot-unsupported-v), file (basename only), offset (torn-line-healed only), v_seen (snapshot-unsupported-v only), scanned (snapshot details only: the instance name when valid, never a path or null).
contract/hook-decision-detail/channel-unreachable · hook-decision detail · channel-unreachable — Written when the channel cannot be reached, a replayed or absent snapshot included; no frame is sent, the hook fails open and SessionEnd takes its direct append.
contract/hook-decision-detail/fail-open-codes · hook-decision detail · fail-open codes — oversize-stdin, malformed-json, channel-unreachable, server-verify-failed, socket-dir-refused, deadline; no hook arm writes strict-modes-failed or unverified-cli.
contract/hook-decision-detail/server-verify-failed · hook-decision detail · server-verify-failed — viola hook writes it at WARN on a server of another euid, a server that is not the recorded wrapper, or a check that cannot be made; no frame is sent, the hook fails open, and on the dialog path it follows hook-invoked with decision_emitted false.
contract/hook-decision-detail/statusline-codes · hook-decision detail · statusline codes — viola hook statusline writes only oversize-stdin, malformed-json and deadline; a refused instance check writes no line.
contract/parse-rejected-detail/oversize · parse-rejected detail · oversize — channel-frame beyond MAX_FRAME, hook-stdin, mcp-stdin past MAX_FRAME, vt100-feed when the bounded queue drops a copy (one line per poisoning episode), claude-agents-json past MAX_FRAME.
contract/parse-rejected-detail/strict-modes-failed · parse-rejected detail · strict-modes-failed — ledger-stamps: run's version gate refused the stamps file on the strict-modes check, and the read counts as unverified.
contract/parse-rejected-detail/unreadable · parse-rejected detail · unreadable — config-json, ledger-stamps.
contract/parse-rejected/mcp-stdin-oversize · parse-rejected diagnostic line · parser mcp-stdin, detail oversize — Written at WARN into mcp.ndjson once per session, after the session ends, only when the stdin reader cut a line past MAX_FRAME, with count 1; the line holds no byte of the cut line.
decision/statusline-command-deadline · Statusline command deadline — The user's statusline command is bounded at 5 s, reported as deadline. · provisional
contract/diag-detail/pinned-hash-mismatch · diagnostic detail words · pinned-hash-mismatch — A filesystem refusal word written on process-exit with exit 1.
contract/parse-rejected-detail/malformed · parse-rejected detail · malformed — Any parser.
contract/process-exit-detail/exit-1 · process-exit detail · exit 1 — already-live, stale-heartbeat, squatted-name, pinned-hash-mismatch, internal-error, socket-dir-refused, and from revive's preflight no-session, session-live, cwd-missing; each one process-exit line with no child started.
contract/process-exit-detail/interrupted · process-exit detail · interrupted — A viola verify stopped by SIGINT or SIGTERM before its rows are checked ends with one process-exit exit 1 detail interrupted, after every spawn pair it opened is closed.
contract/process-exit-detail/verify-refusal · process-exit detail · viola verify refusal — Under a parsed VIOLA_NAME, verify writes its refusal code: program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture or interrupted; a failing row carries no code.

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/mcp-rejects-codes-only · MCP rejects are codes only — MCP rejects are isError codes only, and a logged reject's line holds codes only.
decision/boundary-errors-end-in-logged-line · Crate-seam errors end in a logged line — Every typed error that crosses a crate seam ends in a logged process-exit detail, channel-response error, hook-decision detail or send-refused line, carrying codes only at WARN or ERROR.
decision/instance-unreachable-not-logged · instance-unreachable logged by no line — An MCP instance-unreachable reject, from any instance, liveness, socket-directory, euid or server-verification check, is logged by no line; the isError code is its only record.
decision/serde-errors-become-internal-error · Serde error chains replaced by internal error — At dispatch, an error chain whose source is a serde_json or serde_path_to_error error is replaced by the fixed message "internal error" before it reaches stderr, --json, MCP isError or channel error.data. · why: A scrubbing layer at the source.
decision/typed-errors-fixed-display · Typed errors with fixed Display messages — Every typed error has a fixed-message Display, written with thiserror, and holds no fields with paths or payloads. · except: PtyError's Display is hand-written, as viola-pty takes no thiserror.
contract/panic-outcome-by-role/hook · panic outcome by role · hook — After a panic, hook exits 0 with empty stdout.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
decision/not-a-directory-refusal-message-fixed · NotADirectory refusal message is fixed and pathless — The NotADirectory strict-modes refusal reads the fixed message "a trusted state path is not a directory" and names no path. · why: The security plan's fixed-message rule keeps paths out of error text, and the fixed-messages unit test pins every refusal's text.
decision/pre-log-strict-refusals-write-no-line · Strict-modes refusals before the log opens write no line — Refusals of the instance and home-tree checks that run before a role file opens write no line in any log and open nothing under the home. · why: Opening a log narrows the home and would erase the refused state; the strict-modes negative tests assert that no diagnostics folder appears.
decision/refused-diagnostics-path-is-a-failed-open · A refused diagnostics path is a failed open — A refused diagnostics folder or file is exactly a failed open: the role file does not open, and a detail line is dropped with nothing written and nothing on stderr. · why: The hook carries on with no subscriber, the other roles end as an internal error, and no line is ever written through a widened folder or a symlink.
contract/exit-21-document/detail · exit 21 document · detail — One of six closed words: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused.
contract/mcp-frame-tools/strict-modes-failed · viola mcp frame tools · strict-modes-failed — A refused instance check returns isError instance-unreachable with detail strict-modes-failed, and writes no frame and no line.
contract/mcp-stdin/oversize-line · viola mcp stdin · line past MAX_FRAME — A line past MAX_FRAME is refused whole, gives no isError result and ends the session as a closed stdin: exit 0 after initialize, exit 1 before it with both streams empty; its one record is parse-rejected mcp-stdin oversize, no byte of the line.
decision/mcp-unreachable-carries-one-cause-word · Each MCP unreachable result carries one cause word — Each MCP unreachable outcome (a refused check, a dead or unwrapped instance, a lost endpoint, an unverified server or a refused socket directory) carries exactly one UnreachableCause word as its detail. · why: The MCP process writes no exit line for it, so the result's cause word is the cause's only record for the calling agent.
contract/answer-send-exit-2/stderr-line · viola answer and viola send exit 2 · stderr line — Exit 2 writes the fixed unusable-response or unusable-text line to stderr whatever --json says, with no document.
contract/cli-internal-error/human-line · cli internal error · error: internal error — Without --json the cli catch site prints exactly error: internal error; run, revive's start, mcp and hook print neither.
contract/cli-internal-error/json-document · cli internal error · --json document — Under --json, read among the arguments before a --, the cli catch site prints {"v":1,"error":"internal-error","detail":null} on stdout and stderr stays empty.
contract/exit-21-document/shape · exit 21 document · shape — Exit 21 prints one document per cause: {"v":1,"error":"instance-unreachable","detail":"<cause>"}.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/content-canary-never-home-level · Content canary never in home-level files — The content canary never appears in a home-level diagnostics ndjson file.
decision/fixed-error-messages · Fixed error messages — Error Display texts are fixed, and serde-sourced error chains are mapped to fixed messages at dispatch.
decision/hit-report-never-matched-bytes · Hit report never carries matched bytes — The secret-scan hit report never contains the matched bytes.
decision/local-run-outputs-never-uploaded · Local run outputs never uploaded — mutants.out, the run archive and the local viola-harness pre-push document are never uploaded.
decision/scan-failure-uploads-hit-report-only · Only the hit report uploaded on scan failure — When the secret scan fails, only its hit report is uploaded, as the secret-scan-<os> artifact.
decision/secret-scan-before-upload · Secret scan before artifact upload — CI runs the secret scan for the Cookie header, CLAUDE* values, the content canary and 0600 over every diagnostics/ file before any artifact upload. · guard: secret-scan test
decision/secrets-never-logged · Secrets never logged — The Cookie header, signing credentials and R8-stripped CLAUDE* values are never logged.
decision/upload-members-scanned-or-dropped · Upload members scanned or dropped — An uploaded member that could carry a host path or log content is scanned before upload or dropped; unscanned uploads are admissible by content only.
contract/secret-scan-hit-report/pattern-class · the secret-scan hit report · pattern class — Each hit names the class of pattern that matched.
decision/low-class-metadata-as-typed-fields · Low-class metadata as typed fields — Low-class metadata is logged as typed fields, never inside detail codes.
decision/source-allow-list-logging · Source allow-list for logged fields — Spans skip all arguments and declare fields explicitly, and events go through obs_event! with explicit typed fields only; this source allow-list is the primary PII control. · guard: disallowed-macros; raw_event_hits; G1 bare instrument
decision/statusline-command-out-of-home-logs · statusline_command out of home-level logs — statusline_command stays out of home-level logs.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/content-sink-behind-strict-check · Content sink behind a strict-modes check — The diagnostics content sink opens only after a strict-modes check of the folder and file; a symlink or a folder or file another user may read refuses the open as a failed open and the detail line is dropped. · why: High-class content is never written through a symlink or into a folder another user may read.
decision/cookie-header-never-logged · Cookie header never logged — A Cookie header, in its case-insensitive form, is classed Critical and never logged in any diagnostics/ file. · guard: secret-scan test
decision/error-bodies-clean · Clean error bodies — Error bodies of CLI --json, MCP results and the channel faults hold no absolute path, upstream caused-by text or serde path, and every product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
decision/error-messages-interpolate-nothing · Error messages interpolate nothing — thiserror messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
decision/link-names-never-in-process-log · Link pair names never in a process log — link and unlink driver and driven names appear only in the wrapper's link and unlink event lines in events.ndjson, never in a process-log line; the channel pair logs the method, never the params. · guard: link_verbs_log_the_channel_pair_and_no_names
decision/mutants-out-never-uploaded · Mutation outputs never uploaded — mutants.out/ and the per-run archive of JUnit and outcomes files are never uploaded. · why: The outcomes file held absolute argv paths and the log folder held test output.
decision/mutation-diff-never-uploaded · Mutation diff never uploaded — The mutation run's chunk.diff at the harness capture root is excluded from the harness upload, so it is never scanned and never uploaded. · why: The diff is repository source text by construction and hits the scan's Critical-class patterns.
decision/scan-failure-blocks-upload · Scan failure blocks upload — When the secret scan fails no diagnostics, harness or perf artifact uploads; a hit report with file, line, byte offset and pattern class, never the matched bytes, uploads instead. · why: The failing job still leaves agent-readable evidence.
decision/canary-only-in-detail-files · Canary only in detail files — Every E2E and integration input is synthetic and embeds a fixed canary, which may appear in detail files but never in a home-level diagnostics file. · guard: mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line
decision/closed-protocol-codes · Closed protocol codes — Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value. · guard: answer_invalid_params_names_its_cause_and_no_param

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/g2-zero-panics-gate · Zero-panics gate fails closed — The zero-panics gate probes itself first, fails closed, runs in the test and perf jobs, and counts panics over home-level role files only. · guard: G2 · except: the fake-agent seam's forced panic at exactly its seam location, compared as a whole string
decision/homes-kept-until-gates-run · Test homes kept until the gates run — Integration-test homes are kept until the gate steps have run.
decision/secret-scan-always-runs · Secret scan always runs — The secret scan step runs even when an earlier step failed. · why: otherwise a failed E2E step skips the scan and the gated uploads, which loses the failing job's diagnostics
decision/secret-scan-scope · Secret scan scope — The secret scan covers every test-home diagnostics file, the harness capture and the nextest JUnit file. · why: nextest stores failing tests' captured output in the JUnit file · except: the mutants run's chunk diff, skipped by exact path and excluded from the harness upload
decision/test-homes-under-e2e-home · Integration-test homes under one root — Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: the one chaos_feed_panic test home
decision/uploads-gated-on-secret-scan · CI uploads gated on the secret scan — The diag, harness and junit CI artifact uploads run only when the secret scan, which always runs, succeeded. · guard: secret-scan
decision/g4-schema-conformance-gate · Schema conformance gate on every line — Every home-level line is validated against the diagnostics line schema and every detail line against the detail schema, in a step that always runs. · guard: G4

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-artifacts-gated-on-secret-scan · CI artifacts uploaded only after a passing secret scan — Diagnostics, harness logs, JUnit XML and perf samples are uploaded on failing runs too, but only when the job's secret scan succeeded; a failed scan withholds every artifact. · why: a bare always() or failure() would also upload after a failed scan
decision/obs-code-mutation-tested · Obs code mutation-tested like product code — Obs code is mutation-tested like product code at the epoch boundary and on demand, not on push or pull request; a timed-out or missed obs-code mutant, or unviable outnumbering caught, is red. · guard: test-plan Mutation gate
decision/os-gated-test-census · OS-gated test census — Every OS-gated test function or file must be named in the census with a non-empty reason, and a gate predicate naming windows or macos is a fault even when named. · guard: contract_os_gated_tests
decision/test-homes-kept-until-gates · Test homes kept until gate steps run — CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it. · why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
decision/test-homes-under-scan-root · Test homes live under the scanned root — Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
decision/zero-panics-gate · Zero panics gate — No panic line may appear in the home-level role files of test runs; detail files repeat the panic line and are excluded. · why: ratified · guard: gate G2 (scripts/g2-zero-panics.sh, --probe first) · except: a panic whose panic_location is exactly src/cmd/hook/seam.rs:<digits>, the fake-agent seam's forced panic, compared as a whole string
decision/schema-conformance-gate · Schema conformance gate on real process output — Every role-file line of test homes must validate against schemas/diag-line.v1.json and every detail line against schemas/diag-detail.v1.json; a failing line fails the build, a torn non-JSON line is skipped and counted. · why: schema failures count as a build failure and unit tests check only in-memory lines · guard: gate G4 (schema-conformance step)

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/build-fails-on-any-panic · Build fails on any panic line — The build fails on any panic event line in the kept homes' role files, bar the fake-agent seam's forced panic; no gate infers a panic from an exit code.
decision/diagnostics-files-mode-0600 · Diagnostics files are 0600 — A diagnostics file whose mode is not 0600 counts as an SLO invariant failure.
decision/module-boundary-error-logging · Errors logged once at module boundaries — Every typed error that crosses a crate seam is logged once at WARN or ERROR with a closed detail or error_code; the full chain goes only to the instance detail file.
decision/surviving-obs-mutant-fails · Surviving obs mutant is a failure — A surviving cargo-mutants mutant in obs code at a run --mutants (the epoch-boundary audit or on demand) counts as an SLO invariant failure. · except: A missed mutant the host never compiles is not counted as surviving.
decision/zero-unlogged-panics · Zero unlogged panics — Every panic in run, hook, mcp and cli after init step 4 produces exactly one single-line ERROR panic record in its home-level role file, with no multi-line backtrace there, and hook still exits 0; retry-once policies for panic handling are forbidden. · guard: G2 · except: the bounded exemptions where no file exists, so no line can be written; G2 alone leaves out the fake-agent seam's deliberate panic from its count.
decision/error-budget-measured-by-gate-exits · Error budget measured by in-job gate exit codes — The error budget is measured by in-job gate exit codes, not by the uploaded artifact: panics by G2, schema failures by G4, secret hits by the secret-scan step. · why: the diagnostics artifact is not uploaded when the scan fails, so it is the diagnosis input, not the measurement surface
decision/error-budget-zero-tolerance · Zero-tolerance error budget — The error budget tolerates zero panic lines in any CI artifact, zero secret-scan hits and zero schema-conformance failures. · except: the fake-agent seam's deliberate panic, as the zero-panics gate counts it
decision/hook-panic-without-name-unobservable · Hook panic without VIOLA_NAME unobservable — A hook panic without VIOLA_NAME set is unobservable by design.
decision/nonzero-count-blocks-chunk · Nonzero error count blocks the chunk — Any nonzero error-budget count makes the chunk red until the root cause is fixed within that chunk. · why: the tests quarantine rule
decision/secret-scan-hit-fails · Secret-scan hit is a failure — A secret-scan hit counts as an SLO invariant failure.

## slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin 10-slo-invariants-telemetry-budgets
contract/hook-decision/channel-unreachable · Hook decision line · channel-unreachable — A hook whose snapshot read is replayed or absent fails open as channel-unreachable.
decision/panic-count-home-level-only · Panic count reads home-level files only — The exactly-one panic rule and gate G2 count home-level role files only; the companion panic line in a detail file is excluded. · guard: gate G2 · except: G2 alone leaves out the fake-agent seam's deliberate panic, whose line is still written exactly once.

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/claude-env-and-cookie-never-logged · CLAUDE* values and Cookie never logged — CLAUDE* values and a Cookie header are never logged, at any level, in any file, detail files included.
decision/failed-scan-uploads-hit-report · Failed scan uploads the hit report — On a failed secret scan the hit report is uploaded instead of the diagnostics.
decision/failing-job-diagnostics-kept · Failing job's diagnostics kept — A failing job's diagnostics are never lost: G2, G4 and the secret scan run if: always().
decision/scrub-before-bytes-leave-process · Scrubbing before any byte leaves the process — Every byte is scrubbed of secrets before it leaves the process; in v1 the only destination is local files. · guard: the secret-scan test
decision/upload-only-after-secret-scan · Diagnostics uploaded only after the secret scan — diagnostics artifacts are never uploaded before the secret-scan step passes; uploads run if: always() / failure() combined with the secret-scan step's success, never bare.
contract/stdout-by-role/cli · stdout by role · cli — stdout carries the --json results.
contract/stdout-by-role/hook · stdout by role · hook — stdout carries the decision body or, for hook statusline, the user's status-line output.
decision/closed-event-and-corr-on-every-line · Closed event enum on every line, corr where correlated — Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
decision/log-field-names-frozen · Log field names frozen — Log fields such as corr and event are never renamed without a Decisions Log entry.
decision/must-trace-paths-always-traced · Must-trace paths always traced — Spans and events are never skipped on the 6 must-trace paths. · why: The agent loses its ability to debug without them.
decision/no-telemetry-on-stdout · No telemetry on stdout — Telemetry is never written to stdout on any role. · why: stdout carries each role's own output and run's terminal carries the child's screen.
contract/stdout-by-role/mcp · stdout by role · mcp — stdout carries MCP frames.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/fixed-error-display · Fixed error Display text — Typed errors have fixed Display text that carries no interpolated content.
decision/subscriber-only-in-root-bin · tracing-subscriber only in the root binary — tracing-subscriber is a product dependency of the root binary only, without its chrono feature; the product crates depend on tracing alone. · except: a library crate may take it as a dev-dependency for in-memory line capture in its own unit tests

## log-file-location · key of ## 3. Observability Harness Contract
decision/log-files-owner-only · Owner-only log files — Log files are created 0600 inside 0700 directories. · guard: secret-scan test
contract/diagnostics-files/paths · diagnostics files · process log paths — Process logs are written to <home>/diagnostics/ as run-<name>, hook-<name>, mcp and cli-<name> .ndjson files, and detail to <home>/instances/<name>/diagnostics/detail-<process>.ndjson.

## log-format-json-schema · key of ## 3. Observability Harness Contract
contract/corr-by-event/hook · corr by event · hook-invoked and hook-decision — corr is the dialog_id for dialog hooks, otherwise null; a dialog hook-invoked before the id is known, or a hook-decision carrying detail, may be null.
contract/corr-by-event/parse-rejected · corr by event · parse-rejected — corr is null.
contract/process-log-line/message · the process log line · message — The message is a static literal equal to the event name and never carries interpolated content.
decision/null-as-key-absence · Null written as key absence — A null corr or instance is written as an absent key, readers treat absence and null as equal, and the schema rejects a literal null. · why: tracing has no null field value
contract/event-stream/line · the event stream · event line — Each line of instances/<name>/events.ndjson is exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
contract/process-log-line/corr · the process log line · corr — Copied unchanged as a JSON number or string, never renamed and never debug- or display-formatted; required only on the events whose rules always define it.
contract/process-log-line/instance · the process log line · instance — The instance's name, or absent when none resolved.
contract/process-log-line/level · the process log line · level — One of DEBUG, INFO, WARN, ERROR.
contract/process-log-line/process · the process log line · process — One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing cli-<name>.ndjson.
contract/process-log-line/target · the process log line · target — The target is always present.
contract/process-log-line/timestamp · the process log line · timestamp — RFC 3339 UTC with milliseconds and Z.

## logging-stack · key of ## 3. Observability Harness Contract
contract/diagnostics-files/detail · the diagnostics files · detail — Content-bearing records (full error chains, drift reports, panic payload and backtrace) go only to <home>/instances/<name>/diagnostics/detail-<process>.ndjson, file 0600 in a 0700 directory, and only when an instance resolves.
contract/diagnostics-line/format · the diagnostics line · format — Each diagnostics line is one structured JSON object on one line, written whole by one write of the complete line on an append-mode handle.
contract/obs-event-macro/attached-fields · the obs_event macro · attached fields — Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.

## otel-sdk-init · key of ## 3. Observability Harness Contract
decision/home-tree-check-before-log-init · Home tree check before log init — run, revive's start arm and viola verify check the home tree's modes before their log opens; a refusal writes no line, starts no child, leaves modes as found and exits 1.
decision/hook-refusal-exits-zero · Refused hook fails open — A hook whose instance is refused or absent exits 0 at once, reading nothing under the home and no stdin, writing no frame and no log line.
decision/instance-check-before-log-init · Instance check before log init — Commands acting on an instance (revive's start arm, the hook arms, link, unlink, send, wait, last, answer, pause, release) run the instance strict-modes check before opening any log; a refusal opens and writes nothing and leaves modes as found. · why: Opening the role file narrows an existing home to 0700, so a check made after it passes a home at 0770. · except: The two hook --capture arms run ahead of the check.
decision/private-diagnostics-files · Owner-only diagnostics files — The diagnostics directory is 0700 and each diagnostics file 0600, opened without following symlinks; before the first write each must be owned by the effective user, have no group or other bits and not be a symlink, a failed check counting as a failed open. · why: A pre-existing foreign, permissive or symlinked diagnostics file is never written.
decision/strict-modes-before-diagnostics · Home strict-modes check before diagnostics — Nothing under the home, the diagnostics directory included, is created or opened before the role's strict-modes check on the home passes; a refused home gets no diagnostics line.
contract/cli-exit-codes/21 · CLI exit codes · 21 — An instance strict-modes refusal exits 21 with the fixed instance-state pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
decision/failed-log-open-discards · Failed log open discards lines — When the role file cannot be opened, viola_obs_init returns the error and installs no subscriber, so no line is written; hook carries on and the other roles, mcp included, return the error.
decision/flat-log-lines-and-one-level-gate · Log lines are flat and Targets is the only level gate — The subscriber flattens event fields to the top level of each line, and the Targets filter is the only configurable level gate; its level comes from the home's diagnostics level and never from the environment. · why: The diagnostics schema and the log readers read event fields as top-level keys, and only the viola crates' targets pass the gate.
decision/hook-single-log-open · One config read and one log open per hook — A hook invocation makes one config.json read and exactly one log open, then only appends. · guard: SPINE_BOUND
decision/instance-source-per-role · Instance source fixed per role — run takes its own name argument, never an inherited VIOLA_NAME; hook and mcp take VIOLA_NAME only; cli verbs that act on an instance (send, wait, last, answer, pause, release, link, unlink) take the target argument; list, verify, plugin install and revive --list take the caller's VIOLA_NAME only. · why: A wrapper never writes into another instance's run log.

## service-identity · key of ## 3. Observability Harness Contract
contract/diagnostics-line/instance · the diagnostics line · instance — Every diagnostics line of a process with an instance carries it as field instance; a process without one, such as an mcp server with no instance name, writes no instance key.
contract/diagnostics-line/process · the diagnostics line · process — Every diagnostics line carries the role as field process.

## trace-context-propagation · key of ## 3. Observability Harness Contract
decision/hook-correlation-by-fields · Hook lines correlated by fields — Hooks receive no inbound trace context from Claude Code; hook lines are correlated by instance plus dialog_id for dialog hooks, or by instance plus hook_event plus timestamp.
contract/diagnostics-line/corr-on-dialog-lines · the diagnostics line · corr on dialog and hook lines — On dialog and hook lines, corr carries the dialog_id.

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
