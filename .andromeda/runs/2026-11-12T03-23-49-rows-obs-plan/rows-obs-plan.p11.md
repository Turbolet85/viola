@row_add n=1 row=contract/send-path-log-events/event-order area=obs-scope-summary rec=1
boundary: Send path log events
item: event order
promise: A send logs channel-request, then send-issued, then hook-invoked (UserPromptSubmit, or SessionStart for a confirmed /clear or /compact; none awaited for an unconfirmable send), then send-confirmed or send-refused, then channel-response.
@row_add n=2 row=contract/send-log-line/required-fields area=obs-scope-summary rec=2
boundary: Send log line
item: required fields
promise: A send line carries event, process, instance, corr, from, confirmed, refusal, detail and duration_ms.
@row_add n=3 row=contract/send-log-line/corr area=obs-scope-summary rec=2,28
boundary: Send log line
item: corr
promise: On send lines corr is the send cursor, which joins send-issued, send-confirmed and send-refused across CLI, MCP and wrapper; on channel lines corr is the JSON-RPC id.
@row_add n=4 row=contract/send-log-line/from area=obs-scope-summary rec=2,20
boundary: Send log line
item: from
promise: The from field is self-reported and logged as unauthenticated, never as an identity.
@row_add n=5 row=contract/send-log-line/detail area=obs-scope-summary rec=2
boundary: Send log line
item: detail
promise: The send detail is one of input-not-ready, no-prompt-submitted, control-character, empty-text or turn-running.
@row_add n=6 row=decision/send-text-never-logged area=obs-scope-summary rec=2,21
name: Send text never logged
rule: Send text never appears on stderr, in --json output, in MCP results or in any process-log field outside the instance diagnostics/ folder.
@row_none n=1 rec=3 type=S
@row_none n=2 rec=4 type=5
@row_add n=7 row=contract/wait-path-log-events/channel-pair area=obs-scope-summary rec=5
boundary: Wait path log events
item: channel pair
promise: A wait logs channel-request and channel-response, the response carrying the outcome.
@row_add n=8 row=contract/wait-log-line/required-fields area=obs-scope-summary rec=6
boundary: Wait log line
item: required fields
promise: A wait line carries event, process, instance, corr (the JSON-RPC id), after (a byte offset), timeout_ms, outcome, exit_code and duration_ms.
@row_add n=9 row=contract/wait-log-line/outcome area=obs-scope-summary rec=6
boundary: Wait log line
item: outcome
promise: The wait outcome is the woken kind, timed_out or instance-unreachable.
@row_add n=10 row=contract/wait-log-line/exit-code-21 area=obs-scope-summary rec=6
boundary: Wait log line
item: exit_code 21
promise: A wait exits 21 when the wrapper is killed mid-wait (detail endpoint-lost, during call), or when a CLI wait meets a refused socket directory before any frame (detail socket-dir-refused, during connect, no channel-request).
@row_none n=3 rec=7 type=S
@row_none n=4 rec=8 type=5
@row_add n=11 row=contract/dialog-path-log-events/event-order area=obs-scope-summary rec=9
boundary: Dialog path log events
item: event order
promise: A dialog logs hook-invoked, dialog-raised, dialog-answered, then hook-decision, whose decision is null on deadline expiry.
@row_add n=12 row=contract/dialog-log-line/required-fields area=obs-scope-summary rec=10
boundary: Dialog log line
item: required fields
promise: A dialog line carries event, process, instance, corr, dialog_kind, refusal or detail, deadline_hit and duration_ms.
@row_add n=13 row=contract/dialog-log-line/corr area=obs-scope-summary rec=10,28
boundary: Dialog log line
item: corr
promise: On dialog lines corr is the dialog_id, which joins hook, wrapper and driver lines.
@row_add n=14 row=contract/dialog-log-line/unknown-dialog area=obs-scope-summary rec=10
boundary: Dialog log line
item: unknown-dialog
promise: The detail unknown-dialog goes with exit 13.
@row_add n=15 row=contract/dialog-log-line/unverified-cli area=obs-scope-summary rec=10
boundary: Dialog log line
item: unverified-cli
promise: The detail unverified-cli goes with exit 12.
@row_add n=16 row=contract/dialog-log-line/server-verify-failed area=obs-scope-summary rec=10
boundary: Dialog log line
item: server-verify-failed
promise: A server-verify failure exits 21 on CLI answer with during connect, and on the dialog hook fails open with corr null and decision_emitted false; the line names no pid, start time, path or endpoint.
@row_add n=17 row=contract/dialog-log-line/socket-dir-refused area=obs-scope-summary rec=10
boundary: Dialog log line
item: socket-dir-refused
promise: A refused socket directory exits 21 on CLI answer with during connect, and on the dialog hook fails open with corr null and decision_emitted false; the line names no pid, start time, path or endpoint.
@row_add n=18 row=decision/dialog-content-never-logged area=obs-scope-summary rec=10,42
name: Dialog content never logged
rule: Dialog questions, answers, plan text and tool input never appear in a home-level process-log line; dialog events carry codes only, and the text reaches an instance detail file only inside a chain or drift report.
@row_add n=19 row=contract/cli-frame-verbs/strict-modes-failed area=obs-scope-summary rec=10,52
boundary: CLI frame verbs
item: strict-modes-failed
promise: A strict-modes refusal of send, wait, last, answer, pause or release writes no line and prints only the fixed instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, exit 21, with no pid, path or endpoint.
@row_add n=20 row=contract/dialog-hook/refused-instance-check area=obs-scope-summary rec=10
boundary: Dialog hook
item: refused instance check
promise: A refused instance check on the dialog hook exits 0 at once with empty streams, no frame and no line.
@row_none n=5 rec=11 type=S
@row_none n=6 rec=12 type=5
@row_add n=21 row=contract/wheel-path-log-events/pause-and-release area=obs-scope-summary rec=13
boundary: Wheel path log events
item: pause and release
promise: A wheel take-over logs send-refused for the refused send and a channel-request and channel-response pair for pause and for release.
@row_add n=22 row=contract/diag-line-event/release-from-driver area=obs-scope-summary rec=13,35
boundary: Diag line event
item: release-from-driver
promise: release-from-driver is its own logged event, admitted by the diag-line schema, not a generic -32602 wrapper fault.
@row_add n=23 row=contract/channel-response/detail area=obs-scope-summary rec=13,20,36,50
boundary: Channel response line
item: detail
promise: On a channel-response answering -32602, detail is one closed word: unsupported-version, invalid-params or release-from-driver, on both server and client lines; it is absent on every other channel-response line and names no param or value.
@row_add n=24 row=contract/wheel-log-line/human-typing area=obs-scope-summary rec=14
boundary: Wheel log line
item: human-typing
promise: A send refused while a human holds the wheel logs refusal human-typing, detail null or manual-pause, the wheel holder and exit_code 10.
@row_add n=25 row=decision/typed-keys-never-logged area=obs-scope-summary rec=14
name: Typed keys never logged
rule: Keys a human types into the wrapped session never appear in a log line.
@row_none n=7 rec=15 type=S
@row_none n=8 rec=16 type=5
@row_add n=26 row=contract/budget-path-log-events/event-set area=obs-scope-summary rec=17
boundary: Budget path log events
item: event set
promise: The budget path logs hook-invoked and hook-decision for statusline, and send-refused with budget-paused.
@row_add n=27 row=contract/budget-log-line/required-fields area=obs-scope-summary rec=18
boundary: Budget log line
item: required fields
promise: A budget line carries event, process, instance, corr, window, used_percentage, resets_at, paused, refusal, exit_code, shell_exit_status and duration_ms.
@row_add n=28 row=contract/budget-log-line/window area=obs-scope-summary rec=18
boundary: Budget log line
item: window
promise: The budget window is five-hour or seven-day.
@row_add n=29 row=contract/budget-log-line/budget-paused area=obs-scope-summary rec=18
boundary: Budget log line
item: budget-paused
promise: A send refused by the budget gate logs refusal budget-paused with exit_code 11.
@row_add n=30 row=decision/statusline-command-never-in-home-logs area=obs-scope-summary rec=18,44
name: Statusline command never in home-level logs
rule: The statusline_command string never appears outside diagnostics/, reaching a detail file only as part of an error chain; its shell-out records only shell_exit_status and duration_ms.
why: The statusline command is classed High for integrity.
@row_none n=9 rec=19 type=S
@row_add n=31 row=decision/channel-log-scrub-rule area=obs-scope-summary rec=20
name: Channel log scrub rule
rule: Channel request and response lines log only the method, id or corr, the result class (ok, refusal or error code), the sender version and the closed detail word, never params or result bodies.
@row_add n=32 row=contract/channel-response/internal-error area=obs-scope-summary rec=20
boundary: Channel response line
item: -32603
promise: A -32603 is logged as the fixed text internal error.
@row_add n=33 row=decision/send-refused-by-each-deciding-side area=obs-scope-summary rec=21
name: Send refusal logged by each deciding side
rule: A send-refused with detail control-character or empty-text is emitted by each side that decides it, client and wrapper, without the text.
@row_add n=34 row=decision/hook-fail-open-logged-to-hook-file area=obs-scope-summary rec=22
name: Hook fail-open logged to the hook file
rule: Hook fail-open paths (oversize stdin, malformed JSON, channel failure, server-verify mismatch, refused socket directory, missing VIOLA_NAME) are logged to hook-<name>.ndjson only.
@row_add n=35 row=decision/strict-modes-refusal-writes-no-line area=obs-scope-summary rec=22,25
name: Strict-modes refusal writes no line
rule: A strict-modes refusal of an instance or home check, on every viola hook arm and every other verb, writes no log line.
why: Ratified as the answer to whether a refused check logs.
except: The ledger-stamps read in run's version gate and a refused stamps update in viola verify each leave a line.
@row_add n=36 row=contract/hook-decision/server-verify-failed area=obs-scope-summary rec=22
boundary: Hook decision line
item: server-verify-failed
promise: A server-verify mismatch is one hook-decision at WARN with detail server-verify-failed and no frame sent; on a dialog hook, hook-invoked with corr null then hook-decision with corr null and decision_emitted false; no pid, start time, path or endpoint.
@row_add n=37 row=contract/hook-decision/socket-dir-refused area=obs-scope-summary rec=22,25
boundary: Hook decision line
item: socket-dir-refused
promise: A refused socket directory on viola hook writes one hook-decision in the server-verify form with detail socket-dir-refused, and the hook exits 0.
@row_add n=38 row=contract/hook-fail-open/exit area=obs-scope-summary rec=22
boundary: Hook fail-open
item: exit
promise: A hook that fails open exits 0 and leaves stderr empty.
@row_add n=39 row=decision/drift-reports-detail-only area=obs-scope-summary rec=22,43
name: Drift reports in the detail file only
rule: A full serde drift report goes only to the instance-scoped detail file, as its drift_report field.
@row_add n=40 row=decision/mcp-rejections-codes-only area=obs-scope-summary rec=23
name: MCP rejections carry codes only
rule: A rejected MCP tool call is reported with codes only (refusal, detail, instance-unreachable, wrapper-fault, invalid-params), with no upstream text, absolute paths or type names, and writes no log line.
@row_add n=41 row=contract/mcp-invalid-params/detail area=obs-scope-summary rec=23
boundary: MCP invalid-params
item: detail
promise: The detail of an MCP invalid-params is a closed field word: target, text, after, timeout_ms, dialog_id or response.
@row_add n=42 row=contract/mcp-frame-tool-refusal/strict-modes-failed area=obs-scope-summary rec=23,52
boundary: MCP frame tool refusal
item: strict-modes-failed
promise: A frame tool (send, wait, last, answer) whose instance check refuses returns instance-unreachable with detail strict-modes-failed, writes no frame and no line.
@row_add n=43 row=contract/mcp-frame-tool-refusal/no-live-endpoint area=obs-scope-summary rec=23
boundary: MCP frame tool refusal
item: no live endpoint
promise: A frame tool with no live endpoint returns instance-dead or unwrapped-name and writes no frame.
@row_add n=44 row=contract/mcp-frame-tool-refusal/socket-dir-refused area=obs-scope-summary rec=23,25
boundary: MCP frame tool refusal
item: socket-dir-refused
promise: A frame tool whose socket directory is refused returns socket-dir-refused, writes no frame and no line.
@row_add n=45 row=contract/mcp-frame-tool-refusal/server-verify-failed area=obs-scope-summary rec=23
boundary: MCP frame tool refusal
item: server-verify-failed
promise: A frame tool whose server check fails or whose server euid is refused returns server-verify-failed and writes no frame.
@row_add n=46 row=decision/messaging-secrets-never-logged area=obs-scope-summary rec=24,39
name: Messaging secrets never logged
rule: CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every R8-stripped CLAUDE* value are never logged in any sink, detail files and fixtures included.
guard: secret-scan test
@row_add n=47 row=decision/r8-strip-logged-as-names-only area=obs-scope-summary rec=24,39
name: R8 strip logged as names only
rule: The R8 environment strip is logged as a count and lists of stripped and kept variable names, values never read, and environment maps never pass to a span or event.
@row_add n=48 row=decision/serde-errors-become-fixed-message area=obs-scope-summary rec=24,43
name: Serde errors become a fixed message
rule: An error chain holding a serde source is replaced by a fixed message before it reaches stderr.
@row_add n=49 row=decision/redaction-cannot-be-disabled area=obs-scope-summary rec=24
name: Redaction cannot be disabled
rule: No environment variable, flag or config setting may disable log redaction.
@row_add n=50 row=decision/diagnostics-owner-only-permissions area=obs-scope-summary rec=25
name: Diagnostics owner-only permissions
rule: Files under diagnostics/ are 0600 inside 0700 directories.
guard: secret-scan test
@row_add n=51 row=contract/process-exit/socket-dir-refused-on-start area=obs-scope-summary rec=25
boundary: Process exit line
item: socket-dir-refused on a start
promise: A refused socket directory on a start writes one process-exit with detail socket-dir-refused, exit 1.
@row_add n=52 row=contract/process-exit/socket-dir-refused-on-frame-verb area=obs-scope-summary rec=25
boundary: Process exit line
item: socket-dir-refused on a CLI frame verb
promise: A refused socket directory on a CLI frame verb writes one process-exit with detail socket-dir-refused, exit 21, during connect.
@row_add n=53 row=decision/pinned-copy-race-adds-no-line area=obs-scope-summary rec=25
name: Pinned-copy race adds no telemetry
rule: A pinned-exe hash mismatch reached when a concurrent first start wins the no-clobber placement leaves the file as found and adds no line, field or detail code beyond pinned-hash-mismatch.
@row_add n=54 row=decision/skipped-counters area=obs-scope-summary rec=25,36
name: Skipped counters
rule: Unknown event kinds, unknown fields and torn lines are each counted in skipped counters.
@row_add n=55 row=contract/process-log/child-and-probe-lifecycle area=obs-scope-summary rec=26
boundary: Process log
item: child and probe lifecycle
promise: process-start and process-exit are logged for the claude child and for the claude agents --json and --version probes.
@row_add n=56 row=decision/parse-failures-surface-as-unknown area=obs-scope-summary rec=26
name: Parse failures surface as unknown
rule: A parse failure surfaces as the value unknown plus a counter.
@row_add n=57 row=decision/controls-escaped-in-terminal-output area=obs-scope-summary rec=26,48
name: Controls escaped in terminal output
rule: C0 and C1 control characters are escaped in any human terminal rendering of logged rows.
@row_add n=58 row=decision/tracing-subscriber-logger area=obs-scope-summary rec=27
name: tracing-subscriber is the logger
rule: The logger dependency is tracing-subscriber at version 0.3.20 or later.
@row_add n=59 row=decision/obs-deps-tokio-free area=obs-scope-summary rec=27
name: Obs dependencies Tokio-free
rule: Observability dependencies stay Tokio-free in the sync crates.
@row_add n=60 row=decision/obs-deps-no-c-building-crates area=obs-scope-summary rec=27
name: Obs dependencies build no C
rule: Observability dependencies add no C-building crates.
@row_add n=61 row=decision/obs-deps-pass-cargo-deny area=obs-scope-summary rec=27
name: Obs dependencies pass cargo deny
rule: Observability dependencies pass cargo deny on its one target, x86_64-unknown-linux-gnu.
guard: cargo deny
@row_add n=62 row=contract/hook-log-line/duration-ms area=obs-scope-summary rec=29
boundary: Hook log line
item: duration_ms
promise: duration_ms is on hook-decision, or on process-exit for process hook, for each hook event.
@row_add n=63 row=decision/heartbeat-staleness-threshold area=obs-scope-summary rec=29
name: Heartbeat staleness threshold
rule: A heartbeat flips to stale after more than 5 s: 4.9 s reads live, 5.1 s reads stale or gone.
@row_add n=64 row=decision/latency-aggregation-outside-process area=obs-scope-summary rec=29
name: Latency aggregation outside the process
rule: Percentile aggregation is done by the harness or jq over the ndjson, never by an in-process metrics pipeline.
@row_add n=65 row=absence/in-process-metrics-pipeline area=obs-scope-summary rec=29
capability: An in-process metrics pipeline, with no listener and no exporter
@row_add n=66 row=decision/fault-telemetry-per-class area=obs-scope-summary rec=30
name: Fault telemetry per class
rule: Each fault class emits its own telemetry: a healed torn ndjson line with file and offset, a corrupt or unsupported snapshot recovered by replay, a child exit seen without EOF, an endpoint vanished during wait, and a stale heartbeat against a live pid.
@row_add n=67 row=decision/one-linux-log-sink area=obs-scope-summary rec=31
name: One file sink on Linux
rule: One file sink and one line schema serve Linux, with no OS-specific exporter.
@row_add n=68 row=decision/log-append-atomicity area=obs-scope-summary rec=31
name: Append atomicity for concurrent writers
rule: Concurrent hook writers append to a log atomically through POSIX O_APPEND.
@row_add n=69 row=decision/server-check-peer-credentials area=obs-scope-summary rec=31
name: Server check by peer credentials
rule: The server check reads the peer pid by SO_PEERCRED and the peer euid; a mismatch of either reads server-verify-failed and an unreadable euid is refused.
@row_add n=70 row=decision/zero-unlogged-panics area=obs-scope-summary rec=32,53
name: Zero unlogged panics
rule: Every panic in run, hook, mcp and cli after role-file init produces exactly one single-line ERROR panic record in its home-level role file, with no multi-line backtrace, and hook still exits 0.
guard: gate G2
except: A process that opens no role file (pre-init checks, refused instance checks, a failed role-file open, capture arms) writes no panic line.
@row_add n=71 row=decision/no-formal-slo area=obs-scope-summary rec=32
name: No formal SLO
rule: No formal latency or availability SLO is held beyond the tests perf gates.
@row_add n=72 row=decision/no-log-rotation-v1 area=obs-scope-summary rec=33
name: No log rotation in v1
rule: events.ndjson and diagnostics/*.ndjson are not rotated in v1; any future rotation keeps 0600/0700 permissions and keeps wait after and send cursor offsets valid.
why: The trigger is operational, not compliance: the security tier is Minimal with no compliance triggers.
@row_add n=73 row=decision/structured-json-process-logs area=obs-scope-summary rec=34
name: Structured JSON process logs
rule: Every process role (run, hook, mcp, cli) writes structured JSON process logs under diagnostics/, which harness logs merges with events.ndjson.
@row_add n=74 row=decision/tests-event-list-adopted area=obs-scope-summary rec=34
name: Tests event list adopted
rule: The tests' field and event list is adopted verbatim, with send-issued and send-refused among the logged events.
@row_add n=75 row=contract/exit-21-detail/instance-dead area=obs-scope-summary rec=35
boundary: Exit 21 detail codes
item: instance-dead
promise: instance-dead is a distinct detail code for an exit-21 cause, naming no path or pid.
@row_add n=76 row=contract/exit-21-detail/unwrapped-name area=obs-scope-summary rec=35
boundary: Exit 21 detail codes
item: unwrapped-name
promise: unwrapped-name is a distinct detail code for an exit-21 cause, naming no path or pid.
@row_add n=77 row=contract/exit-21-detail/endpoint-lost area=obs-scope-summary rec=35
boundary: Exit 21 detail codes
item: endpoint-lost
promise: endpoint-lost is a distinct detail code for an exit-21 cause, naming no path or pid.
@row_add n=78 row=contract/exit-21-detail/strict-modes-failed area=obs-scope-summary rec=35
boundary: Exit 21 detail codes
item: strict-modes-failed
promise: strict-modes-failed is a distinct detail code for an exit-21 cause, naming no path or pid.
@row_add n=79 row=contract/exit-21-detail/server-verify-failed area=obs-scope-summary rec=35
boundary: Exit 21 detail codes
item: server-verify-failed
promise: server-verify-failed is a distinct detail code for an exit-21 cause, distinct from instance-dead, naming no path or pid.
@row_add n=80 row=contract/exit-21-detail/socket-dir-refused area=obs-scope-summary rec=35
boundary: Exit 21 detail codes
item: socket-dir-refused
promise: socket-dir-refused is a distinct detail code for an exit-21 cause, naming no path or pid.
@row_add n=81 row=contract/exit-1-detail/already-live area=obs-scope-summary rec=35
boundary: Exit 1 detail codes
item: already-live
promise: already-live is a distinct detail code for an exit-1 cause, naming no path or pid.
@row_add n=82 row=contract/exit-1-detail/stale-heartbeat area=obs-scope-summary rec=35
boundary: Exit 1 detail codes
item: stale-heartbeat
promise: stale-heartbeat is a distinct detail code for an exit-1 cause, naming no path or pid.
@row_add n=83 row=contract/exit-1-detail/squatted-name area=obs-scope-summary rec=35
boundary: Exit 1 detail codes
item: squatted-name
promise: squatted-name is a distinct detail code for an exit-1 cause, naming no path or pid.
@row_add n=84 row=contract/exit-1-detail/pinned-hash-mismatch area=obs-scope-summary rec=35
boundary: Exit 1 detail codes
item: pinned-hash-mismatch
promise: pinned-hash-mismatch is a distinct detail code for an exit-1 cause, naming no path or pid.
@row_add n=85 row=contract/exit-1-detail/socket-dir-refused area=obs-scope-summary rec=35
boundary: Exit 1 detail codes
item: socket-dir-refused
promise: socket-dir-refused is a distinct detail code for an exit-1 cause, naming no path or pid.
@row_add n=86 row=contract/exit-1-detail/no-session area=obs-scope-summary rec=35
boundary: Exit 1 detail codes
item: no-session
promise: no-session is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
@row_add n=87 row=contract/exit-1-detail/session-live area=obs-scope-summary rec=35
boundary: Exit 1 detail codes
item: session-live
promise: session-live is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
@row_add n=88 row=contract/exit-1-detail/cwd-missing area=obs-scope-summary rec=35
boundary: Exit 1 detail codes
item: cwd-missing
promise: cwd-missing is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
@row_add n=89 row=contract/viola-revive/strict-modes-failed area=obs-scope-summary rec=35
boundary: viola revive output
item: strict-modes-failed
promise: A strict-mode fail of viola revive writes no log line; it prints the stderr pair, or under --json {"v":1,"error":"strict-modes-failed","detail":null}, and exits 1.
@row_add n=90 row=contract/viola-link-unlink/strict-modes-failed area=obs-scope-summary rec=35
boundary: viola link and unlink output
item: strict-modes-failed
promise: A strict-mode fail of the driven instance's check writes no log line; it prints the fixed instance-state unable and hint: pair on stderr, or under --json {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, and exits 21.
@row_add n=91 row=contract/cli-process-exit/server-verify-failed area=obs-scope-summary rec=35,52
boundary: CLI process log
item: server-verify-failed
promise: send, wait, last, answer, pause, release, link and unlink write process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"} to the cli log (link and unlink: the driven instance's), with no pid, start time, path or endpoint.
@row_add n=92 row=decision/socket-dir-one-code area=obs-scope-summary rec=35
name: One code for socket-directory causes
rule: A refused check, a failed create or a lost race on the socket directory all read the one code socket-dir-refused.
@row_add n=93 row=contract/refusal-reason/unknown-fallback area=obs-scope-summary rec=36
boundary: Refusal reason
item: unknown fallback
promise: An unrecognised RefusalReason falls back to unknown with exit 14.
@row_add n=94 row=contract/channel-response/invalid-params-causes area=obs-scope-summary rec=36
boundary: Channel response line
item: invalid-params causes
promise: detail invalid-params covers send's invalid params, and a link or unlink whose driver or driven is missing, not a string or no valid name, whose driven is not the serving wrapper's own name, or whose driver equals driven.
@row_add n=95 row=decision/version-rejections-counted-by-detail area=obs-scope-summary rec=36
name: Version rejections counted by detail
rule: Higher-v rejections are counted by the detail unsupported-version, not by the -32602 code.
why: The -32602 code also carries invalid-params and release-from-driver.
@row_add n=96 row=decision/per-parser-parse-failure-counts area=obs-scope-summary rec=36
name: Per-parser parse-failure counts
rule: Each parser (paste validation, channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, vt100 under catch_unwind) keeps its own parse-failure count.
@row_add n=97 row=decision/deterministic-event-emission area=obs-scope-summary rec=37
name: Deterministic event emission
rule: Events are emitted deterministically, keyed on hook events and events.ndjson offsets.
@row_add n=98 row=decision/no-telemetry-from-screen area=obs-scope-summary rec=37
name: No telemetry from screen content
rule: No telemetry is derived from screen content; vt100 serves gate outcomes only.
@row_add n=99 row=decision/no-sleep-based-log-waits area=obs-scope-summary rec=37
name: No sleep-based log waits
rule: No wait on a log line is built on sleeps.
@row_add n=100 row=decision/cookie-header-never-logged area=pii-scrubbing-compliance rec=38
name: Cookie header never logged
rule: A Cookie header, in its case-insensitive form, is classed Critical and never logged in any diagnostics/ file.
guard: secret-scan test
@row_add n=101 row=decision/signing-credentials-never-logged area=pii-scrubbing-compliance rec=40
name: Signing credentials never logged
rule: v1.x signing credentials are classed Critical and never logged, and CI secrets are never echoed into artifacts.
@row_add n=102 row=decision/prompt-and-output-never-in-home-logs area=pii-scrubbing-compliance rec=41,6
name: Prompt and assistant output never in home logs
rule: Prompt text and assistant output, last_assistant_message included, never appear in a home-level process-log line; they reach an instance detail file only inside a chain or drift report, and only text_bytes is logged.
@row_add n=103 row=decision/veil-redaction area=pii-scrubbing-compliance rec=41,42
name: veil derives redaction
rule: veil 0.3.0's derived Redact is what redacts the payload fields of contract types, and instrumented spans skip all arguments.
@row_add n=104 row=decision/home-level-lines-path-free area=pii-scrubbing-compliance rec=45
name: Home-level lines path-free
rule: Home-level log lines are path-free: file fields are basenames, panic_location is workspace-relative and source file recording is off; absolute paths appear only in detail files.
why: The OS username inside absolute paths is the only PII in scope.
@row_add n=105 row=decision/endpoint-paths-never-logged area=pii-scrubbing-compliance rec=46
name: Endpoint and pinned binary never logged
rule: The snapshot endpoint and pinned_bin are never logged, only endpoint_kind.
@row_add n=106 row=decision/typed-ids-never-in-detail area=pii-scrubbing-compliance rec=47
name: Ids logged as typed fields
rule: pid, child_pid, agent_session_id, started_at, budget percentages and resets_at are logged as typed fields, never inside detail codes.
@row_add n=107 row=decision/agent-rows-never-verbatim area=pii-scrubbing-compliance rec=48
name: Agent rows never logged verbatim
rule: claude agents --json rows are untrusted and never logged verbatim, only parse-rejected counts.
@row_add n=108 row=decision/link-names-never-in-process-log area=pii-scrubbing-compliance rec=49
name: Link pair names never in a process log
rule: link and unlink driver and driven names appear only in the wrapper's link and unlink event lines in events.ndjson, never in a process-log line; the channel pair logs the method, never the params.
guard: link_verbs_log_the_channel_pair_and_no_names
@row_add n=109 row=decision/closed-protocol-codes area=pii-scrubbing-compliance rec=50
name: Closed protocol codes
rule: Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value.
guard: answer_invalid_params_names_its_cause_and_no_param
@row_add n=110 row=decision/fixtures-not-a-telemetry-source area=pii-scrubbing-compliance rec=51
name: Fixtures not a telemetry source
rule: Recorded fixtures are not a telemetry source, and observability adds no fixture content.
@row_add n=111 row=decision/secret-scan-before-upload area=pii-scrubbing-compliance rec=52
name: Secret scan before upload
rule: Every diagnostics/ file is scanned for the Cookie header, CLAUDE* values, the content canary and 0600 permissions in CI before any artifact upload.
guard: secret-scan test
@row_add n=112 row=decision/canary-only-in-detail-files area=pii-scrubbing-compliance rec=52
name: Canary only in detail files
rule: Every E2E and integration input is synthetic and embeds a fixed canary, which may appear in detail files but never in a home-level diagnostics file.
guard: mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line
@row_add n=113 row=decision/detail-upload-needs-synthetic-input area=pii-scrubbing-compliance rec=52
name: Detail upload needs synthetic input
rule: Detail files and harness captures are uploaded only while every input is synthetic; a non-synthetic input source must first drop detail files and the harness capture from the upload paths.
@row_add n=114 row=decision/mutation-diff-never-uploaded area=pii-scrubbing-compliance rec=52
name: Mutation diff never uploaded
rule: The mutation run's chunk.diff at the harness capture root is excluded from the harness upload, so it is never scanned and never uploaded.
why: The diff is repository source text by construction and hits the scan's Critical-class patterns.
@row_add n=115 row=decision/pre-push-document-path-free area=pii-scrubbing-compliance rec=52
name: Pre-push document path-free
rule: The local pre-push gate document is never uploaded and carries no absolute path, only codes and counts.
guard: pre_push_document_carries_no_absolute_path
@row_add n=116 row=decision/server-verify-refusal-names-nothing area=pii-scrubbing-compliance rec=52
name: Server-verify refusal names nothing
rule: A server-verification refusal carries no pid, start time, path or endpoint in any output or log line.
guard: security_negatives_server_identity
@row_add n=117 row=decision/socket-dir-refusal-names-nothing area=pii-scrubbing-compliance rec=52
name: Socket-directory refusal names nothing
rule: A socket-directory refusal carries no path, pid or uid in its lines and documents, and its human pair names only the parsed instance name.
guard: security_negatives_socket_dir
@row_add n=118 row=decision/error-bodies-clean area=pii-scrubbing-compliance rec=52
name: Clean error bodies
rule: Error bodies of CLI --json, MCP results and the channel faults hold no absolute path, upstream caused-by text or serde path, and every product error writer is named.
guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
@row_add n=119 row=decision/error-messages-interpolate-nothing area=pii-scrubbing-compliance rec=52
name: Error messages interpolate nothing
rule: thiserror messages interpolate no value.
guard: thiserror_messages_interpolate_nothing
@row_add n=120 row=decision/content-sink-behind-strict-check area=pii-scrubbing-compliance rec=52
name: Content sink behind a strict-modes check
rule: The diagnostics content sink opens only after a strict-modes check of the folder and file; a symlink or a folder or file another user may read refuses the open, the writer becomes a sink and the line is dropped.
why: High-class content is never written through a symlink or into a folder another user may read.
@row_add n=121 row=decision/unscanned-uploads-admissible-by-content area=pii-scrubbing-compliance rec=52
name: Unscanned uploads admissible by content
rule: An artifact uploaded without a scan must carry no host path or log content; a member that could carry one is scanned before upload or dropped.
why: Ratified.
@row_add n=122 row=decision/mutants-out-never-uploaded area=pii-scrubbing-compliance rec=52
name: Mutation outputs never uploaded
rule: mutants.out/ and the per-run archive of JUnit and outcomes files are never uploaded.
why: The outcomes file held absolute argv paths and the log folder held test output.
@row_add n=123 row=decision/fuzz-artifacts-need-synthetic-corpus area=pii-scrubbing-compliance rec=52
name: Fuzz artifacts need a synthetic corpus
rule: Fuzz crash artifacts upload only while the committed fuzz corpus is synthetic; a non-synthetic corpus seed must first drop that upload.
@row_add n=124 row=decision/scan-failure-blocks-upload area=pii-scrubbing-compliance rec=52
name: Scan failure blocks upload
rule: When the secret scan fails no diagnostics, harness or perf artifact uploads; a hit report with file, line, byte offset and pattern class, never the matched bytes, uploads instead.
why: The failing job still leaves agent-readable evidence.
@row_add n=125 row=decision/panic-count-home-level-only area=slo-invariants-telemetry-budgets rec=53
name: Panic count reads home-level files only
rule: The exactly-one panic rule, the viola.panic counter and gate G2 count home-level role files only; the companion panic line in a detail file is excluded.
guard: gate G2
except: G2 alone leaves out the fake-agent seam's deliberate panic, whose line is still written exactly once.
@row_add n=126 row=decision/no-retry-on-panic area=slo-invariants-telemetry-budgets rec=53
name: No retry on panic
rule: Retry-once policies for panic handling are forbidden.
why: Parallel to the tests' retries = 0.
@row_add n=127 row=decision/session-end-perf-gate area=slo-invariants-telemetry-budgets rec=54,29
name: Session-end hook perf gate
rule: viola hook session-end must keep its hyperfine max under 1.0 s; p50 and p95 are report-only.
guard: gate --require perf
@row_add n=128 row=decision/spine-hook-perf-gate area=slo-invariants-telemetry-budgets rec=55,29
name: Spine hook perf gate
rule: Spine hooks (session-start, user-prompt-submit, stop) and pre-tool-use on an unverified CLI keep their hyperfine max under the spine deadline, timed over every check a frame-writing hook makes before its first frame.
guard: gate --require perf
@row_add n=129 row=contract/hook-decision/channel-unreachable area=slo-invariants-telemetry-budgets rec=55
boundary: Hook decision line
item: channel-unreachable
promise: A hook whose snapshot read is replayed or absent fails open as channel-unreachable.
