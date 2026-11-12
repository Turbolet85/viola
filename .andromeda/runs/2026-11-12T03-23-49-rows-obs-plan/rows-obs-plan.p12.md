@row_add n=1 row=decision/heartbeat-staleness-flip area=slo-invariants-telemetry-budgets rec=1
name: Heartbeat staleness flip
rule: An instance whose heartbeat is 4.9 s old reads live, and one whose heartbeat is 5.1 s old reads stale or gone, as reported in the liveness field of list --json.
@row_add n=2 row=decision/hook-obs-overhead-budget area=slo-invariants-telemetry-budgets rec=2
name: Observability overhead on viola hook
rule: Logger initialisation in viola hook costs one file open plus appends and stays inside the hook's max < 1.0 s performance gate.
@row_add n=3 row=decision/hook-benchmark-on-passing-instance area=slo-invariants-telemetry-budgets rec=2
name: Hook benchmark runs against a passing instance
rule: Every hyperfine hook command runs with VIOLA_NAME set, a home under target/e2e-home/ and an instance directory that exists and passes the strict-modes check.
why: Without a name, or with an absent or refused instance, the hook times its no-op path and the gate passes vacuously.
@row_add n=4 row=decision/perf-job-runs-g2 area=slo-invariants-telemetry-budgets rec=2
name: Perf job runs G2 over the benchmark home
rule: The perf job runs the G2 check over the hook benchmark home.
why: A hook panic exits 0 and hyperfine cannot see it.
@row_add n=5 row=contract/span-kinds/client area=span-trace-coverage rec=3
boundary: Span kinds
item: CLIENT
promise: The CLIENT spans are send.client, wait.block, last.client, answer.client, pause.client, release.client, cli.link, cli.unlink, channel.request (client side), statusline.shell_out and pty.spawn.
@row_add n=6 row=contract/span-kinds/server area=span-trace-coverage rec=4
boundary: Span kinds
item: SERVER
promise: The SERVER spans are channel.dispatch (the wrapper side of every channel.request), mcp.tool_call, hook.handle and hook.statusline.
@row_add n=7 row=contract/span-kinds/internal area=span-trace-coverage rec=5
boundary: Span kinds
item: INTERNAL
promise: The INTERNAL spans are run.*, state.*, hook.decision_emit and pty.paste_write.
@row_add n=8 row=decision/cli-manual-dispatch-span area=span-trace-coverage rec=6
name: Manual dispatch span per cli verb
rule: Each short-lived cli verb opens one manual dispatch span of its own.
why: clap 4.6.7 has no tracing integration.
@row_add n=9 row=decision/cli-log-only-under-viola-name area=span-trace-coverage rec=6
name: cli process log only under a parsed name
rule: A short-lived cli verb opens its process log and writes process-start and process-exit only when an instance resolves under a parsed VIOLA_NAME; with no name it writes no log.
@row_add n=10 row=decision/refused-check-writes-no-line area=span-trace-coverage rec=6,16
name: A refused check opens no process log
rule: A cli verb whose strict-modes, tree or instance check refuses (exit 21) opens no process log and writes no line.
@row_add n=11 row=contract/revive-list-process-exit/no-session area=span-trace-coverage rec=6
boundary: viola revive --list process log
item: no-session
promise: An empty session chain ends process-exit with exit_code 1 and detail no-session; any other listing ends exit_code 0 with no detail.
@row_add n=12 row=decision/revive-start-logs-as-run area=span-trace-coverage rec=6
name: viola revive start logs as run
rule: The start arm of viola revive is not a cli verb and logs as process run once its instance check has passed.
@row_add n=13 row=decision/error-chain-in-detail-log-only area=span-trace-coverage rec=6
name: Full error chain only in the cli detail log
rule: The full anyhow error chain of a cli verb goes to the cli detail log only.
@row_add n=14 row=decision/portable-pty-log-target-off area=span-trace-coverage rec=7
name: portable-pty log target OFF
rule: The log target of portable-pty is OFF in the sink, and PTY failures are captured at the pty seam instead.
@row_add n=15 row=decision/manual-readiness-gate-span area=span-trace-coverage rec=7
name: Manual span on the readiness gate
rule: The readiness gate of viola run carries a manual span, run.readiness_gate.
why: vt100 has no tracing.
@row_add n=16 row=contract/claude-child-process-log/handle-wait-thread area=span-trace-coverage rec=7,27
boundary: run process log, claude-child lines
item: process-start and process-exit of claude-child
promise: Both are emitted from the handle-wait thread; process-exit{subject:"claude-child"} carries child_exit_status and exit_source handle-wait or kill-fallback, and is never emitted on reader EOF.
@row_add n=17 row=contract/hook-spans/span-chain area=span-trace-coverage rec=8
boundary: viola hook spans
item: span chain
promise: viola hook traces hook.handle › channel.request › hook.decision_emit.
@row_add n=18 row=decision/hook-always-exits-zero area=span-trace-coverage rec=8
name: viola hook keeps exit 0
rule: viola hook exits 0 even when its dispatch panics: a panic around dispatch is caught.
@row_add n=19 row=decision/hook-refused-instance-silent area=span-trace-coverage rec=8
name: Refused instance makes the hook silent
rule: On the event and dialog paths the instance check runs before the diagnostics level and log init; a refused or unresolved instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append.
@row_add n=20 row=decision/hook-capture-arms-unlogged area=span-trace-coverage rec=8
name: Hidden capture arms write no lines
rule: The hidden capture arms of viola hook (hook <event> --capture and hook statusline --capture) initialise no observability and write no lines.
@row_add n=21 row=contract/channel-lines/fields area=span-trace-coverage rec=9
boundary: channel-request and channel-response lines
item: fields
promise: Both lines carry method, corr, conn, result_class, error_code and sender, plus detail on a -32602.
@row_add n=22 row=decision/no-params-in-channel-lines area=span-trace-coverage rec=9
name: No params or result bodies in channel lines
rule: channel-request and channel-response lines never carry params or result bodies.
@row_add n=23 row=decision/accept-drops-foreign-euid area=span-trace-coverage rec=10
name: Unadmitted peers dropped at accept
rule: The wrapper drops an accepted stream whose peer euid is not admitted on the accept thread, before any worker thread or frame, with no span and no line; the peer reads end-of-file and the wheel does not move.
@row_add n=24 row=decision/rmcp-tracing-target-off area=span-trace-coverage rec=11
name: rmcp tracing target OFF
rule: The native tracing target of rmcp is OFF in the sink.
why: Its records lack required fields and can carry frame content.
@row_add n=25 row=contract/mcp-spans/mcp-tool-call area=span-trace-coverage rec=11
boundary: viola mcp spans
item: mcp.tool_call
promise: viola mcp opens one mcp.tool_call span per tool (send, wait, last, answer, list); rejects log codes-only send-refused or channel-response lines.
@row_add n=26 row=decision/mcp-explicit-log-writer area=span-trace-coverage rec=11
name: Explicit log writer in viola mcp
rule: The viola mcp log writer is always explicit: a file, with stderr only as a fallback.
@row_add n=27 row=decision/verify-server-before-frame area=span-trace-coverage rec=12
name: Server verified before any frame
rule: Each frame verb, after its own client-side refusals and before any frame, checks the socket directory, connects, decides on the peer euid, then compares the peer's pid and OS start time against the one snapshot read.
@row_add n=28 row=contract/cli-exit-21/server-verify-failed area=span-trace-coverage rec=12,32
boundary: cli exit 21 process-exit
item: server-verify-failed
promise: A refused euid, a server that is not the recorded wrapper, or a check that cannot be made opens no channel.request span, writes no channel-request line and ends process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}.
@row_add n=29 row=decision/peer-euid-must-match area=span-trace-coverage rec=13
name: Server euid must match the client's
rule: Every client seam refuses a server of another euid, or one whose euid cannot be read, after the connect and before the pid comparison; reading the euid writes and logs nothing.
@row_add n=30 row=decision/server-refusal-inside-root-span area=span-trace-coverage rec=14
name: Server-verification refusal inside the root span
rule: A frame verb's or the hook's server-verification refusal happens inside its scenario's root span.
guard: request_refuses_an_unverified_server_inside_its_dispatch_span; hook_refuses_an_unverified_server_inside_hook_handle
@row_add n=31 row=decision/socket-dir-refusal-inside-root-span area=span-trace-coverage rec=15
name: Socket-directory refusal inside the root span
rule: A frame verb's or the hook's socket-directory refusal happens inside its dispatch span or hook span.
guard: connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span
@row_add n=32 row=decision/instance-check-before-role-file area=span-trace-coverage rec=16
name: Instance check before any role file
rule: The frame verbs run the instance check on their named instance (the driven instance for link and unlink) before any role file opens.
@row_add n=33 row=contract/instance-unreachable-json/strict-modes-failed area=span-trace-coverage rec=16,35
boundary: cli --json refusal document
item: strict-modes-failed
promise: A refused instance check exits 21 with the instance pair on stderr, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, and sends no frame.
@row_add n=34 row=contract/cli-exit-21/socket-dir-refused area=span-trace-coverage rec=17
boundary: cli exit 21 process-exit
item: socket-dir-refused
promise: A frame verb meeting a refused socket directory writes one process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"} at WARN inside its dispatch span, with no channel-request line and no frame.
@row_add n=35 row=decision/socket-dir-check-changes-no-mode area=span-trace-coverage rec=17
name: Socket-directory check repairs nothing
rule: The socket-directory check on the recorded endpoint's parent changes no mode: a refused directory stays as found.
@row_add n=36 row=contract/refusal-stderr-pair/socket-dir-refused area=span-trace-coverage rec=18
boundary: cli refusal stderr pair
item: socket-directory refusal
promise: Printed on start and frame-verb refusals in plain ASCII with hint: last and no SGR, naming only the parsed ViolaName.
@row_add n=37 row=decision/replayed-snapshot-unreachable area=span-trace-coverage rec=19
name: Replayed snapshot is unreachable
rule: Only a snapshot read whole leads to a connect; a replayed or absent snapshot is treated as unreachable and no frame is sent.
why: A replayed snapshot carries no endpoint, pid or started_at.
@row_add n=38 row=contract/hook-decision-detail/channel-unreachable area=span-trace-coverage rec=19
boundary: hook-decision line
item: channel-unreachable
promise: A replayed or absent snapshot makes the hook fail open with hook-decision{detail:"channel-unreachable"}; a SessionEnd takes the direct append.
@row_add n=39 row=contract/hook-decision-detail/server-verify-failed area=span-trace-coverage rec=19,39
boundary: hook-decision line
item: server-verify-failed
promise: A refused euid, a server that is not the recorded wrapper or a check that cannot be made sends no frame and fails open (exit 0, empty streams) with hook-decision{detail:"server-verify-failed"}; a SessionEnd takes the direct append.
@row_add n=40 row=contract/hook-decision-detail/socket-dir-refused area=span-trace-coverage rec=20,39
boundary: hook-decision line
item: socket-dir-refused
promise: A refused socket directory writes one hook-decision{detail:"socket-dir-refused"} at WARN inside hook.handle after hook-invoked{corr:null}, with decision_emitted:false; exit 0, empty streams, no frame.
@row_add n=41 row=contract/channel-response/detail area=span-trace-coverage rec=21,31
boundary: channel-response line
item: detail
promise: On a -32602 only, channel-response carries detail, a word of the closed unsupported-version|invalid-params|release-from-driver, on both the server's and the client's line; every other channel-response line carries none.
@row_add n=42 row=contract/mcp-instance-unreachable/strict-modes-failed area=span-trace-coverage rec=22
boundary: viola mcp frame tool refusal
item: strict-modes-failed
promise: A refused instance check, run before the live endpoint is read, returns instance-unreachable with detail strict-modes-failed, no frame and no line.
@row_add n=43 row=contract/mcp-instance-unreachable/socket-dir-refused area=span-trace-coverage rec=22
boundary: viola mcp frame tool refusal
item: socket-dir-refused
promise: A refused socket directory returns instance-unreachable with detail socket-dir-refused, no frame and no line.
@row_add n=44 row=contract/mcp-instance-unreachable/server-verify-failed area=span-trace-coverage rec=22
boundary: viola mcp frame tool refusal
item: server-verify-failed
promise: A refused euid, a mismatch or a check that cannot be made returns the codes-only isError instance-unreachable with detail server-verify-failed, no frame and no log line.
@row_none n=1 rec=23 type=4
relied: The CLI and MCP clients screen a peer reply to code words and a sanitised fault before using it, adding no span and no line.
@row_add n=45 row=contract/run-start-spans/span-chain area=span-trace-coverage rec=24
boundary: viola run start spans
item: span chain
promise: run.start (INTERNAL, root) › state.snapshot_recover › run.collision_check › run.pin_copy › run.version_gate › channel.bind › run.budget_eval (trigger start) › state.snapshot_write › state.heartbeat_start › pty.spawn (CLIENT).
@row_add n=46 row=decision/statusline-settings-steps-unlogged area=span-trace-coverage rec=24
name: Statusline read and settings write unlogged
rule: The start's statusline source read and settings override write carry no span and no log line.
why: Ratified as by design.
@row_add n=47 row=decision/no-statusline-settings-in-telemetry area=span-trace-coverage rec=24,46
name: No settings or statusline command in telemetry
rule: The child's --settings argument, the statusline source path and the statusline command appear in no log line or span.
@row_add n=48 row=contract/run-start-failure-detail/socket-dir-refused area=span-trace-coverage rec=24,26
boundary: run start process-exit
item: socket-dir-refused
promise: A socket directory the start refuses is one process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"} before any snapshot, child or bind, on viola revive's start too; a bind failure keeps its own outcome.
@row_add n=49 row=contract/run-span-attributes/run-start area=span-trace-coverage rec=25
boundary: viola run start span attributes
item: run.start
promise: run.start carries instance.
@row_add n=50 row=contract/run-span-attributes/run-collision-check area=span-trace-coverage rec=25
boundary: viola run start span attributes
item: run.collision_check
promise: outcome is free, already-live or squatted-name; a live or stale holder is already-live, a gone one free; over a replayed snapshot a beat no older than STALE_AFTER is already-live, an older one free; an absent snapshot or failed read is free.
@row_add n=51 row=contract/run-span-attributes/run-pin-copy area=span-trace-coverage rec=25,28
boundary: viola run start span attributes
item: run.pin_copy
promise: outcome is ok or pinned-hash-mismatch, and the set stays those two.
@row_add n=52 row=contract/run-span-attributes/run-version-gate area=span-trace-coverage rec=25
boundary: viola run start span attributes
item: run.version_gate
promise: run.version_gate carries cli_version and cli_verified.
@row_add n=53 row=contract/run-span-attributes/channel-bind area=span-trace-coverage rec=25
boundary: viola run start span attributes
item: channel.bind
promise: channel.bind carries endpoint_kind, unix-socket.
@row_add n=54 row=contract/run-span-attributes/state-snapshot-write area=span-trace-coverage rec=25
boundary: viola run start span attributes
item: state.snapshot_write
promise: state.snapshot_write carries v.
@row_add n=55 row=contract/run-span-attributes/pty-spawn area=span-trace-coverage rec=25
boundary: viola run start span attributes
item: pty.spawn
promise: pty.spawn carries pty_backend (openpty) and env_stripped_count.
@row_add n=56 row=decision/no-paths-in-run-spans area=span-trace-coverage rec=25
name: No endpoint, path or env values on start spans
rule: Start spans never carry the endpoint name, the pinned binary path or environment values.
@row_add n=57 row=contract/run-process-log/common-fields area=span-trace-coverage rec=26
boundary: run process log
item: common fields
promise: Every run line carries the common fields with process="run" and corr null.
@row_add n=58 row=contract/run-process-log/process-start-self area=span-trace-coverage rec=26
boundary: run process log
item: process-start self
promise: process-start{subject:"self", service_name, version, os, pid, endpoint_kind}.
@row_add n=59 row=contract/run-process-log/version-probe area=span-trace-coverage rec=26
boundary: run process log
item: version-probe
promise: process-start and process-exit{subject:"version-probe", child_exit_status, duration_ms}.
@row_add n=60 row=contract/run-start-failure-detail/already-live area=span-trace-coverage rec=26
boundary: run start process-exit
item: already-live
promise: process-exit{subject:"self", exit_code:1, detail:"already-live"} for a live holder or a fresh replayed beat, with no path or pid.
@row_add n=61 row=contract/run-start-failure-detail/stale-heartbeat area=span-trace-coverage rec=26
boundary: run start process-exit
item: stale-heartbeat
promise: process-exit{subject:"self", exit_code:1, detail:"stale-heartbeat"} for a stale holder, with no path or pid.
@row_add n=62 row=contract/run-start-failure-detail/squatted-name area=span-trace-coverage rec=26
boundary: run start process-exit
item: squatted-name
promise: process-exit{subject:"self", exit_code:1, detail:"squatted-name"}, with no path or pid.
@row_add n=63 row=contract/run-start-failure-detail/pinned-hash-mismatch area=span-trace-coverage rec=26
boundary: run start process-exit
item: pinned-hash-mismatch
promise: process-exit{subject:"self", exit_code:1, detail:"pinned-hash-mismatch"}, with no path or pid.
@row_add n=64 row=decision/home-tree-check-before-start-log area=span-trace-coverage rec=26
name: Home-tree check before the start's log
rule: The start of run and of viola revive checks the home tree (the home, instance set, ledger, bin, plugin, pinned exe and plugin hooks) before its log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
@row_add n=65 row=contract/revive-preflight-detail/no-session area=span-trace-coverage rec=26
boundary: viola revive preflight process-exit
item: no-session
promise: A preflight refusal adds detail no-session before any child is started.
@row_add n=66 row=contract/revive-preflight-detail/session-live area=span-trace-coverage rec=26
boundary: viola revive preflight process-exit
item: session-live
promise: A preflight refusal adds detail session-live before any child is started; the reading is skipped under --fork and passes on a failed read.
@row_add n=67 row=contract/revive-preflight-detail/cwd-missing area=span-trace-coverage rec=26
boundary: viola revive preflight process-exit
item: cwd-missing
promise: A preflight refusal adds detail cwd-missing before any child is started.
@row_add n=68 row=contract/revive-process-log/agents-probe area=span-trace-coverage rec=26
boundary: viola revive run process log
item: agents-probe
promise: The session-live reading writes one process-start and process-exit{subject:"agents-probe"} pair, plus one parse-rejected{parser:"claude-agents-json"} on an oversize or malformed answer.
@row_add n=69 row=contract/run-process-log/process-start-claude-child area=span-trace-coverage rec=26
boundary: run process log
item: process-start claude-child
promise: On success, process-start{subject:"claude-child", child_pid, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept}.
@row_add n=70 row=contract/diag-line-schema/pty-backend area=span-trace-coverage rec=26
boundary: diag-line v1 schema
item: pty_backend
promise: pty_backend is any string.
@row_add n=71 row=contract/diag-line-schema/os area=span-trace-coverage rec=26
boundary: diag-line v1 schema
item: os
promise: os is closed to linux.
@row_add n=72 row=decision/product-order-from-events-log area=span-trace-coverage rec=26
name: Start product order read from the events log
rule: The product order wheel{cause:"start"} then budget-gate then session-start{source:"hook"} is verified from the instance's events log, not duplicated into process logs.
@row_add n=73 row=contract/run-start-spans/run-start-close area=span-trace-coverage rec=27
boundary: viola run start spans
item: run.start close
promise: run.start closes when pty.spawn returns.
@row_add n=74 row=contract/run-process-log/process-exit-self area=span-trace-coverage rec=27
boundary: run process log
item: process-exit self
promise: After the child's process-exit, run emits process-exit{subject:"self", exit_code, duration_ms}.
@row_add n=75 row=decision/pinned-copy-never-replaced area=span-trace-coverage rec=28
name: Pinned copy placed, never replaced
rule: An absent pinned copy is placed, never replaced; a start that loses the placement re-hashes the copy that stands, reusing an equal one and refusing a different one as pinned-hash-mismatch with the file left as found.
@row_add n=76 row=contract/confirmed-send-spans/span-chain area=span-trace-coverage rec=29
boundary: confirmed send spans
item: span chain
promise: send.client (CLIENT) › channel.request (CLIENT) → channel.dispatch (SERVER) › run.readiness_gate › pty.paste_write › run.confirm_window; an unconfirmable send ends at pty.paste_write. In the hook: hook.handle › channel.request(hook.event).
@row_add n=77 row=decision/implicit-link-on-send-and-answer area=span-trace-coverage rec=29,50
name: Implicit link recorded by send and answer
rule: An issued send or a taken answer whose from names an instance other than the wrapper records the link, one link product line for a new pair; a refused call, one with no from, or one under the wrapper's own name records nothing.
@row_add n=78 row=contract/events-log/link-before-send-issued area=span-trace-coverage rec=29
boundary: events log
item: implicit link and send-issued
promise: A new pair's link product line stands directly before that send's send-issued, whose cursor is the offset its own line starts at; a link record that fails answers -32603 with nothing issued, nothing typed and no send-issued.
@row_add n=79 row=decision/link-names-not-in-process-log area=span-trace-coverage rec=29,51
name: Link names stay out of process logs
rule: The driver and driven names reach no process-log line.
@row_add n=80 row=contract/no-endpoint-instance/classification area=span-trace-coverage rec=29
boundary: no-endpoint instance reading
item: instance-dead or unwrapped-name
promise: An instance with no endpoint whose path is a directory reads instance-dead; anything else (absent, a regular file, a symlink) reads unwrapped-name; the reading writes no line and the instance check stays first.
@row_add n=81 row=contract/send-span-attributes/run-readiness-gate area=span-trace-coverage rec=30
boundary: confirmed send span attributes
item: run.readiness_gate
promise: outcome (ready or input-not-ready) and vt100_panicked (bool).
@row_add n=82 row=contract/send-span-attributes/text-bytes area=span-trace-coverage rec=30,31
boundary: confirmed send span attributes
item: text_bytes
promise: text_bytes, on pty.paste_write and send-issued alike, is the typed text's length: the sent text with each CR LF pair and each other CR as one LF, trailing CR and LF removed; pty.paste_write also carries paste_mode="bracketed".
@row_add n=83 row=contract/send-span-attributes/window-ms area=span-trace-coverage rec=30
boundary: confirmed send span attributes
item: run.confirm_window window_ms
promise: window_ms is the bound the send's own deadline reads: 40 000 for a /compact send waiting for its compacted-session post-condition, else 10 000.
@row_add n=84 row=decision/compact-confirm-window area=span-trace-coverage rec=30,32
name: Compact confirm window
rule: A /compact send waits 40 s for its compacted-session post-condition, longer than the 10 s every other confirmed send waits.
state: provisional
@row_add n=85 row=decision/no-text-in-telemetry area=span-trace-coverage rec=30,31
name: Sent text never in telemetry
rule: The sent text never appears in a span or log line; only its length does.
@row_add n=86 row=contract/send-log-fields/channel-request area=span-trace-coverage rec=31
boundary: confirmed send log fields
item: channel-request
promise: channel-request{corr:<id>, conn, method, from, from_trust:"self-reported", sender}.
@row_add n=87 row=contract/send-log-fields/send-issued area=span-trace-coverage rec=31
boundary: confirmed send log fields
item: send-issued
promise: send-issued{corr:<cursor>, from, text_bytes}.
@row_add n=88 row=contract/send-log-fields/hook-invoked area=span-trace-coverage rec=31
boundary: confirmed send log fields
item: hook-invoked
promise: hook-invoked{hook_event:"user-prompt-submit", corr:null} for an ordinary send; hook_event session-start for a /clear or /compact confirmed by its session; an unconfirmable send waits for no hook line.
@row_add n=89 row=contract/send-log-fields/send-confirmed area=span-trace-coverage rec=31
boundary: confirmed send log fields
item: send-confirmed
promise: send-confirmed{corr:<cursor>, confirmed, duration_ms}, confirmed false for unconfirmable, duration_ms from the send's arrival including the readiness gate's wait.
@row_add n=90 row=contract/send-refused-detail/input-not-ready area=span-trace-coverage rec=31
boundary: wrapper send-refused detail
item: input-not-ready
promise: send-refused{corr:<cursor>, refusal, detail:"input-not-ready", side:"wrapper", wheel}.
@row_add n=91 row=contract/send-refused-detail/no-prompt-submitted area=span-trace-coverage rec=31,32
boundary: wrapper send-refused detail
item: no-prompt-submitted
promise: Confirm-window expiry is send-refused{detail:"no-prompt-submitted"} for ordinary, /clear and /compact sends alike, with no new detail.
@row_add n=92 row=contract/send-refused-detail/turn-running area=span-trace-coverage rec=31
boundary: wrapper send-refused detail
item: turn-running
promise: Covers a running turn or another send in flight, with no new field; the refusal has no cursor (corr the end offset at refusal), no send-issued, and the client exits 13.
@row_add n=93 row=contract/send-refused-client/control-character area=span-trace-coverage rec=31
boundary: client-side send validation refusal
item: control-character
promise: Made before any frame: send-refused{corr:null, refusal:"not-delivered", detail:"control-character", side:"client"}, one line per deciding side, never the text.
@row_add n=94 row=contract/send-refused-client/empty-text area=span-trace-coverage rec=31
boundary: client-side send validation refusal
item: empty-text
promise: An empty typed text, read after control-character and ahead of both wheel reads; a wrapper deciding it writes its own send-refused{side:"wrapper"} with the same pair, corr, conn and rpc_id, and no send-issued.
@row_add n=95 row=decision/send-line-correlation area=span-trace-coverage rec=31
name: Send lines correlate with channel lines
rule: Every wrapper send-* line carries conn (or srv_conn) and rpc_id, so that pair on the send line equals (conn, corr) on the call's channel lines; the client-side send-refused carries neither.
@row_add n=96 row=contract/confirm-window-close/ordinary-send area=span-trace-coverage rec=32
boundary: run.confirm_window close
item: ordinary send
promise: Closes on the matching prompt-submitted{origin:"driver"}, or on window expiry.
@row_add n=97 row=contract/confirm-window-close/clear area=span-trace-coverage rec=32
boundary: run.confirm_window close
item: /clear
promise: On a verified CLI, closes on the session-start whose cause is clear and whose agent_session_id differs from the id the send slot remembers; no prompt-submitted claims the send.
@row_add n=98 row=contract/confirm-window-close/compact area=span-trace-coverage rec=32
boundary: run.confirm_window close
item: /compact
promise: On a verified CLI, closes on the session-start whose cause is compact and whose agent_session_id equals the id the send slot remembered; nothing remembered claims nothing, and no prompt-submitted claims the send.
@row_add n=99 row=decision/channel-response-always-logged area=span-trace-coverage rec=32
name: channel-response always logged
rule: channel.dispatch logs channel-response on every path, an early return or error included.
why: It is emitted from a drop guard so an early return or error still logs it.
@row_add n=100 row=contract/cli-exit/1-internal-error area=span-trace-coverage rec=32
boundary: send.client process-exit
item: 1 internal-error
promise: exit_code 1 internal-error on a reply with neither a parseable result nor an error.code.
@row_add n=101 row=contract/cli-exit/20-wrapper-fault area=span-trace-coverage rec=32
boundary: send.client process-exit
item: 20 wrapper-fault
promise: Exit 20 carries detail wrapper-fault.
@row_add n=102 row=contract/cli-exit-21/instance-dead area=span-trace-coverage rec=32,35
boundary: cli exit 21 process-exit
item: instance-dead
promise: detail instance-dead with during connect: no live snapshot, a replayed one included, or the connect failed.
@row_add n=103 row=contract/cli-exit-21/unwrapped-name area=span-trace-coverage rec=32,35
boundary: cli exit 21 process-exit
item: unwrapped-name
promise: detail unwrapped-name with during connect: no instance directory.
@row_add n=104 row=contract/cli-exit-21/endpoint-lost area=span-trace-coverage rec=32,35
boundary: cli exit 21 process-exit
item: endpoint-lost
promise: detail endpoint-lost with during call: the connection was lost after the connect; it is the only exit-21 detail whose during is call.
@row_add n=105 row=contract/wait-last-spans/span-chain area=span-trace-coverage rec=33
boundary: wait and last spans
item: span chain
promise: wait.block (CLIENT) › channel.request → channel.dispatch › run.wait_dispatch (INTERNAL); for last, last.client › channel.request.
@row_add n=106 row=contract/wait-span-attributes/wait-block area=span-trace-coverage rec=34
boundary: wait span attributes
item: wait.block
promise: wait.block carries after and timeout_ms.
@row_add n=107 row=contract/wait-span-attributes/run-wait-dispatch area=span-trace-coverage rec=34
boundary: wait span attributes
item: run.wait_dispatch
promise: woken_kind, one of turn-ended, question, permission, plan, session-end, dialog-expired, or timed_out.
@row_add n=108 row=contract/wait-log-fields/channel-request area=span-trace-coverage rec=35
boundary: wait and last log fields
item: channel-request
promise: channel-request{corr:<id>, conn, method:"wait"|"last"} on both sides, plus after and timeout_ms for wait only and only as u64 values.
@row_add n=109 row=contract/wait-log-fields/channel-response-outcome area=span-trace-coverage rec=35
boundary: wait and last log fields
item: channel-response outcome
promise: For wait only, outcome is the waking kind or timed-out, derived once over the shared wake set; the client side alone writes instance-unreachable when the reply never arrived; last carries no outcome and no result body reaches either line.
@row_add n=110 row=contract/mcp-refusal-text/instance-unreachable area=span-trace-coverage rec=35
boundary: viola mcp wait and last refusal
item: strict-modes-failed body
promise: MCP wait and last refused by the instance check return the cli document less v (error instance-unreachable, detail strict-modes-failed), their text refused: instance-unreachable, with no line.
@row_add n=111 row=decision/no-last-assistant-message area=span-trace-coverage rec=35
name: last_assistant_message never logged
rule: last_assistant_message never appears in a log line.
@row_add n=112 row=decision/disconnect-logged-by-client area=span-trace-coverage rec=36
name: Mid-call disconnect logged once by the client
rule: A mid-call disconnect is logged once, by the client.
@row_add n=113 row=contract/dialog-spans/span-chain area=span-trace-coverage rec=37
boundary: dialog spans
item: span chain
promise: hook.handle (SERVER) › channel.request(hook.dialog) → channel.dispatch › run.dialog_register › run.dialog_await › hook.decision_emit; driver side answer.client › channel.request(answer) → channel.dispatch.
@row_add n=114 row=contract/answer-link-record/failure area=span-trace-coverage rec=37
boundary: wrapper answer handling
item: failed link record
promise: A link record that fails answers -32603: the answer is not taken, the dialog stays held and no dialog-answered line is written.
@row_add n=115 row=decision/dialog-hook-fails-open area=span-trace-coverage rec=37
name: Dialog hook fails open on a failed check
rule: A dialog hook whose instance, socket-directory, euid or server check fails emits no decision and sends no frame, so the dialog falls back to the human.
@row_add n=116 row=decision/second-dialog-gets-null area=span-trace-coverage rec=38
name: A concurrent second dialog gets null
rule: A second concurrent dialog gets a null response.
@row_add n=117 row=contract/config-json/dialog-hold-s area=span-trace-coverage rec=38
boundary: config.json
item: dialog_hold_s
promise: The dialog hold, read in ms as 60 000 to 3 600 000, default 60 000.
@row_add n=118 row=contract/dialog-log-fields/dialog-raised area=span-trace-coverage rec=39
boundary: dialog log fields
item: dialog-raised
promise: dialog-raised{corr:<dialog_id>, dialog_kind, hook_event}.
@row_add n=119 row=contract/dialog-log-fields/dialog-answered area=span-trace-coverage rec=39
boundary: dialog log fields
item: dialog-answered
promise: dialog-answered{corr:<dialog_id>, dialog_kind, from, from_trust, deadline_hit, duration_ms}; from and from_trust only on an answer that carried a from.
@row_add n=120 row=contract/dialog-log-fields/hook-lines area=span-trace-coverage rec=39
boundary: dialog log fields
item: hook-invoked and hook-decision
promise: hook-invoked{corr:<dialog_id>, hook_event, stdin_bytes, invoked_at}, then hook-decision{corr:<dialog_id>, hook_event, decision_emitted, deadline_hit, detail, duration_ms}.
@row_add n=121 row=decision/dialog-hook-invoked-after-response area=span-trace-coverage rec=39
name: Dialog hook-invoked written after the response
rule: A dialog hook writes hook-invoked only once dialog_id is known, after the hook.dialog response; non-dialog hooks and fail-open paths before dialog_id write hook-invoked{corr:null} at once.
why: invoked_at still records the hook's true start, and the wrapper's dialog-raised gives live visibility during the wait.
@row_add n=122 row=contract/answer-refusal/not-delivered area=span-trace-coverage rec=39
boundary: driver-side answer refusal
item: not-delivered
promise: channel-response{result_class:"refusal", refusal:"not-delivered"} with no detail on the line, exit 13.
@row_add n=123 row=contract/answer-refusal/unverified-cli area=span-trace-coverage rec=39
boundary: driver-side answer refusal
item: unverified-cli
promise: refusal unverified-cli, exit 12, the answer withheld.
@row_add n=124 row=decision/no-dialog-content-in-logs area=span-trace-coverage rec=39
name: Dialog content never logged
rule: The question, the answer and the tool input never appear in a log line.
@row_add n=125 row=contract/events-log/dialog-expired area=span-trace-coverage rec=40
boundary: events log
item: dialog-expired
promise: At the hold's end the wrapper appends one dialog-expired{dialog_id, dialog_kind} line (source wrapper, no question, plan or tool text) that wakes a parked wait, and writes dialog-answered with deadline_hit:true and no from.
@row_add n=126 row=decision/expired-dialog-renders-for-human area=span-trace-coverage rec=40
name: Expired dialog returns to the human
rule: At the hold's end the reply stays null, the hook writes hook-decision with decision_emitted false, exits 0 with empty stdout, and the dialog renders for the human.
@row_add n=127 row=contract/hook-decision/deadline-hit area=span-trace-coverage rec=40
boundary: dialog hook-decision line
item: deadline_hit
promise: The hook writes deadline_hit:true only when its own read times out at its backstop bound, DIALOG_HOLD_MAX + 5 s (3605 s).
@row_add n=128 row=contract/wheel-spans/span-chain area=span-trace-coverage rec=41
boundary: wheel spans
item: span chain
promise: run.wheel_transition (INTERNAL) › channel.dispatch(send) refused; pause.client or release.client (CLIENT) › channel.request(pause|release) → channel.dispatch(pause|release).
@row_add n=129 row=contract/pause-release/answer-after-wheel-record area=span-trace-coverage rec=41
boundary: pause and release methods
item: reply timing
promise: pause and release answer only after the move's wheel record lands, -32603 if it did not.
@row_add n=130 row=contract/wheel-span-attributes/run-wheel-transition area=span-trace-coverage rec=42
boundary: wheel span attributes
item: run.wheel_transition
promise: wheel_from, wheel_to (human|driver), cause (human-input|manual-pause|release); a point-in-time span with a static name, opened once per wheel change on the wheel's worker thread.
@row_add n=131 row=decision/no-typed-keys-in-telemetry area=span-trace-coverage rec=42
name: Typed keys never in telemetry
rule: The keys the human types never appear in a span.
@row_add n=132 row=contract/send-refused/human-typing area=span-trace-coverage rec=43
boundary: wrapper send-refused
item: human-typing
promise: send-refused{corr:<cursor>, refusal:"human-typing", detail:null|"manual-pause", wheel:"human", side:"wrapper"}, with the client process-exit{exit_code:10}.
@row_add n=133 row=decision/release-from-driver-refused area=span-trace-coverage rec=43
name: A release from a driver is refused
rule: A release that carries a string from is refused with -32602, and viola release forwards VIOLA_NAME, so a driver session's own viola release is refused too.
@row_add n=134 row=contract/release-refusal/release-from-driver area=span-trace-coverage rec=43
boundary: release refusal lines
item: release-from-driver
promise: One release-from-driver{corr:<rpc id>, conn, from (only a valid name), from_trust:"self-reported"} at INFO beside the -32602 reply with data {"reason":"release-from-driver"}; its channel-response carries detail release-from-driver on both sides at WARN.
@row_add n=135 row=contract/release-refusal/invalid-params area=span-trace-coverage rec=43
boundary: release refusal lines
item: invalid-params
promise: A from of another type, or a non-bool budget, is a plain -32602 (data null) with no release-from-driver line, its channel-response carrying detail invalid-params on both sides at WARN.
@row_add n=136 row=contract/release-budget/budget-gate area=span-trace-coverage rec=43
boundary: release method
item: budget true
promise: A release with budget true does not touch the wheel and appends one budget-gate record when a pause stood, none otherwise; the string-from refusal and the budget type check come first.
@row_add n=137 row=decision/wheel-event-is-audit-record area=span-trace-coverage rec=44
name: Wheel product event is the audit record
rule: The wheel product event in the events log is the audit record of a wheel change; the wheel transition span is only a point in time.
@row_add n=138 row=contract/budget-spans/span-chain area=span-trace-coverage rec=45
boundary: budget governor spans
item: span chain
promise: hook.statusline (SERVER) › state.budget_write › statusline.shell_out (CLIENT); in run, run.budget_eval (INTERNAL).
@row_add n=139 row=contract/budget-span-attributes/state-budget-write area=span-trace-coverage rec=46
boundary: budget span attributes
item: state.budget_write
promise: state.budget_write carries five_hour_pct and seven_day_pct.
@row_add n=140 row=contract/budget-span-attributes/statusline-shell-out area=span-trace-coverage rec=46
boundary: budget span attributes
item: statusline.shell_out
promise: statusline.shell_out carries shell_exit_status and duration_ms.
@row_add n=141 row=contract/budget-span-attributes/run-budget-eval area=span-trace-coverage rec=46
boundary: budget span attributes
item: run.budget_eval
promise: trigger (start|send|turn-ended), paused, and window only when paused; opened once per evaluation: at start, at a send's arrival, and after a turn-ended hook line is appended.
@row_add n=142 row=decision/unreadable-budget-reads-as-none area=span-trace-coverage rec=46
name: Bad budget file reads as no reading
rule: A refused, unreadable, over-cap or other-version budget file reads as no reading.
@row_add n=143 row=contract/events-log/budget-gate area=span-trace-coverage rec=46
boundary: events log
item: budget-gate
promise: A budget-gate record is appended only when the triple (paused, window, override end) changes.
@row_add n=144 row=contract/statusline-log-fields/hook-invoked area=span-trace-coverage rec=47
boundary: statusline hook log fields
item: hook-invoked
promise: hook-invoked{hook_event:"statusline", corr:null}.
@row_add n=145 row=contract/statusline-log-fields/parse-rejected area=span-trace-coverage rec=47
boundary: statusline hook log fields
item: parse-rejected
promise: parse-rejected{parser:"hook-stdin", detail:"oversize"|"malformed", count:1} for stdin the arm rejects.
@row_add n=146 row=contract/statusline-log-fields/statusline-shell area=span-trace-coverage rec=47
boundary: statusline hook log fields
item: statusline-shell
promise: process-start and process-exit{subject:"statusline-shell", shell_exit_status, duration_ms}, only when the snapshot records a statusline_command.
@row_add n=147 row=contract/statusline-log-fields/hook-decision area=span-trace-coverage rec=47
boundary: statusline hook log fields
item: hook-decision
promise: hook-decision{hook_event:"statusline", budget_written, duration_ms}, deadline_hit only when true; budget_written is true only when the payload held a rate_limits object and the write succeeded.
@row_add n=148 row=contract/statusline-decision-detail/oversize-stdin area=span-trace-coverage rec=47
boundary: statusline hook-decision detail
item: oversize-stdin
promise: The statusline hook-decision carries detail oversize-stdin on that fail-open path.
@row_add n=149 row=contract/statusline-decision-detail/malformed-json area=span-trace-coverage rec=47
boundary: statusline hook-decision detail
item: malformed-json
promise: The statusline hook-decision carries detail malformed-json on that fail-open path.
@row_add n=150 row=contract/statusline-decision-detail/deadline area=span-trace-coverage rec=47,48
boundary: statusline hook-decision detail
item: deadline
promise: The shell-out reaching its bound ends in hook-decision{detail:"deadline", deadline_hit:true}.
@row_add n=151 row=decision/statusline-silent-without-instance area=span-trace-coverage rec=47
name: Statusline arm silent without a valid instance
rule: The statusline arm writes no line without a valid VIOLA_NAME or when the instance check refuses: it returns exit 0 at once with no role file opened.
@row_add n=152 row=contract/send-refused/budget-paused area=span-trace-coverage rec=47
boundary: wrapper send-refused
item: budget-paused
promise: send-refused{corr:<cursor>, refusal:"budget-paused", detail:"five-hour"|"seven-day", window, used_percentage, resets_at, paused:true}, with client exit_code 11.
@row_add n=153 row=decision/statusline-deadline area=span-trace-coverage rec=48
name: Statusline shell-out deadline
rule: The statusline shell-out is bound at 5 s.
state: provisional
@row_add n=154 row=decision/statusline-stdout-passthrough area=span-trace-coverage rec=48
name: Statusline stdout passes through unlogged
rule: The user's statusline stdout passes through unchanged and is never logged.
@row_add n=155 row=decision/rejected-statusline-stdin-runs-nothing area=span-trace-coverage rec=48
name: Rejected statusline stdin runs nothing
rule: Oversize or malformed statusline stdin prints nothing and runs no shell.
@row_add n=156 row=decision/claude-env-strip area=span-trace-coverage rec=49
name: CLAUDE environment strip
rule: The child loses every inherited CLAUDE* name, matched exactly, except the persistent set kept in the claude_env_keep config key.
@row_add n=157 row=decision/identity-floor-always-stripped area=span-trace-coverage rec=49
name: Identity floor always stripped
rule: The 11 names of the identity floor are always removed whatever the persistent set says; beyond the floor there is no compile-time list of known names.
@row_add n=158 row=decision/env-names-only area=span-trace-coverage rec=49
name: Environment names only, never values
rule: The strip and every log read and record environment names only, never values, and no env map is passed to a span or event.
@row_add n=159 row=contract/claude-child-process-start/env-fields area=span-trace-coverage rec=49
boundary: process-start claude-child
item: env fields
promise: env_stripped_count, env_stripped_known (every stripped name, comma-joined) and env_kept (every kept persistent-set name, comma-joined).
@row_add n=160 row=decision/implicit-link-untraced area=span-trace-coverage rec=50
name: Implicit link has no line or span
rule: The implicit link record has no process-log line, no event value and no span; its trace is the call's channel pair, the link product line and the snapshot's links.
@row_add n=161 row=contract/link-methods/invalid-params area=span-trace-coverage rec=51
boundary: link and unlink methods
item: -32602
promise: A missing, non-string or invalid name, a driven that is not the serving wrapper's own name, or a driver equal to driven is -32602 with data null and no line; a record or remove error is -32603; neither method reads from.
@row_add n=162 row=decision/link-methods-untraced area=span-trace-coverage rec=51
name: Explicit link methods open no span
rule: The explicit link and unlink methods open no span and write no process-log line of their own; their channel lines hold no driver, driven or other param.
@row_add n=163 row=contract/events-log/link area=span-trace-coverage rec=51
boundary: events log
item: link
promise: link appends one link product line {driver, driven} (source wrapper) for a new pair and nothing for a pair the set holds, and answers after its line and the snapshot write.
@row_add n=164 row=contract/events-log/unlink area=span-trace-coverage rec=51
boundary: events log
item: unlink
promise: unlink appends one unlink product line {driver, driven} (source wrapper) only when it removes a pair, nothing for an absent one, and answers after its line and the snapshot write.
@row_add n=165 row=contract/state-recovered/snapshot-replayed area=span-trace-coverage rec=52
boundary: state-recovered line
item: snapshot-replayed
promise: state-recovered{detail:"snapshot-replayed", file:"snapshot.json"} at WARN with no corr for an unreadable snapshot, one line per call; none for a present or absent snapshot, and none when the replay itself fails.
@row_add n=166 row=contract/state-recovered/snapshot-unsupported-v area=span-trace-coverage rec=52
boundary: state-recovered line
item: snapshot-unsupported-v
promise: state-recovered{detail:"snapshot-unsupported-v", file:"snapshot.json", v_seen} at WARN with no corr for a newer snapshot, one line per call.
@row_add n=167 row=contract/state-recovered/scanned area=span-trace-coverage rec=52
boundary: state-recovered line
item: scanned
promise: From the board read, the line carries instance (the caller) and scanned (the scanned instance's name when it parses, absent otherwise); with no VIOLA_NAME no log is open and the line goes nowhere.
@row_add n=168 row=decision/no-raw-snapshot-reads area=span-trace-coverage rec=52
name: Product readers read the snapshot through replay
rule: Every product reader of the snapshot uses the replaying read; no product reader reads the snapshot raw.
@row_add n=169 row=contract/state-recovered/torn-line-healed area=span-trace-coverage rec=53
boundary: state-recovered line
item: torn-line-healed
promise: state-recovered{detail:"torn-line-healed", file:"events.ndjson", offset} at WARN with no corr, by the process whose append healed, offset the log's length before the append; one line per heal.
@row_add n=170 row=decision/appender-heals-torn-line area=span-trace-coverage rec=53
name: The appender heals a torn line
rule: An append that finds the events log's last byte is not LF writes one LF and its line in a single write; the reader neither heals nor logs.
