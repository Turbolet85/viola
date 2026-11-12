@row_add n=1 row=contract/parse-rejected/mcp-stdin-oversize area=log-coverage rec=1
boundary: parse-rejected diagnostic line
item: parser mcp-stdin, detail oversize
promise: Written at WARN into mcp.ndjson once per session, after the session ends, only when the stdin reader cut a line past MAX_FRAME, with count 1; the line holds no byte of the cut line.
@row_add n=2 row=contract/send-refused-detail/refusal-reason-words area=log-coverage rec=2
boundary: send-refused detail
item: RefusalReason detail words
promise: Refusal details are the arch RefusalReason details verbatim: manual-pause, five-hour, seven-day, input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text.
@row_add n=3 row=contract/send-refused-detail/empty-text area=log-coverage rec=2
boundary: send-refused detail
item: empty-text
promise: Written for an empty typed text on send; the word adds no field, and send-refused detail stays an open string in the diag-line schema.
@row_add n=4 row=contract/diag-detail/strict-modes-failed area=log-coverage rec=3,20
boundary: diagnostic detail words
item: strict-modes-failed
promise: Written on the process-exit exit 1 of viola verify under a parsed VIOLA_NAME and on parse-rejected ledger-stamps from the version gate of run, at warn, with no path; every other refused strict-modes check writes no log line, its exit code or JSON document its only record.
@row_add n=5 row=contract/diag-detail/pinned-hash-mismatch area=log-coverage rec=3
boundary: diagnostic detail words
item: pinned-hash-mismatch
promise: A filesystem refusal word written on process-exit with exit 1.
@row_add n=6 row=decision/symlink-skip-logged-as-count area=log-coverage rec=3
name: Skipped state entries logged as a parse-rejected count
rule: A scan of instance state that skips a symlinked or malformed entry writes one parse-rejected line with parser state-entry, detail symlink-ignored or malformed and a count, once per scan, at warn, into the scanning process's role file, naming no entry and no link target.
why: A skipped symlink has no exit or decision to carry it.
@row_add n=7 row=contract/diag-detail/socket-dir-refused area=log-coverage rec=3,19
boundary: diagnostic detail words
item: socket-dir-refused
promise: A closed detail on process-exit and hook-decision for a socket dir refused as a symlink, no directory, foreign owner or mode bit: run and revive's start write process-exit exit 1, CLI frame verbs process-exit exit 21 during connect at WARN, viola hook a hook-decision at WARN; MCP writes none.
@row_add n=8 row=contract/diag-subject/self area=log-coverage rec=4
boundary: diag-line subject enum
item: self
promise: One value of the closed subject enum of the diag-line schema.
@row_add n=9 row=contract/diag-subject/claude-child area=log-coverage rec=4
boundary: diag-line subject enum
item: claude-child
promise: One value of the closed subject enum of the diag-line schema.
@row_add n=10 row=contract/diag-subject/version-probe area=log-coverage rec=4
boundary: diag-line subject enum
item: version-probe
promise: Subject of the process-start and process-exit pair of a version probe spawn.
@row_add n=11 row=contract/diag-subject/verify-probe area=log-coverage rec=4
boundary: diag-line subject enum
item: verify-probe
promise: Subject of the process-start and process-exit pair of the print-mode probe of viola verify.
@row_add n=12 row=contract/diag-subject/verify-pty-probe area=log-coverage rec=4
boundary: diag-line subject enum
item: verify-pty-probe
promise: Subject of the process-start and process-exit pair of each of the five interactive PTY runs of viola verify, Run A to Run E.
@row_add n=13 row=contract/diag-subject/agents-probe area=log-coverage rec=4
boundary: diag-line subject enum
item: agents-probe
promise: Subject of the process-start and process-exit pair of the claude agents read, written by viola list, viola revive's session-live reading, verify's Run B and the list tool of viola mcp into their role files.
@row_add n=14 row=contract/diag-subject/statusline-shell area=log-coverage rec=4
boundary: diag-line subject enum
item: statusline-shell
promise: Subject of the process-start and process-exit pair of a statusline shell spawn.
@row_add n=15 row=contract/verify-process-exit/child-exit-status-and-duration area=log-coverage rec=4
boundary: viola verify spawn process-exit lines
item: child_exit_status and duration_ms
promise: Each of the eight verify spawn pairs' process-exit carries child_exit_status and duration_ms, including a child of an interactive PTY run killed at a Ctrl-C or TERM stop; a stopped run writes only the pairs it started.
@row_add n=16 row=contract/parse-rejected/claude-agents-json area=log-coverage rec=4
boundary: parse-rejected diagnostic line
item: parser claude-agents-json
promise: A claude agents --json parse failure, oversize or malformed, gives one parse-rejected line with parser claude-agents-json and the value unknown.
@row_add n=17 row=decision/counters-from-home-role-files area=metric-coverage rec=5
name: Counters derived from home-level role files
rule: A counter of an event, detail or refusal value is derived with jq over the home-level diagnostics role files only, skipping a torn line.
why: Detail files repeat lines.
@row_add n=18 row=decision/histograms-as-raw-samples area=metric-coverage rec=6
name: Histograms as raw samples
rule: A latency distribution is the raw duration_ms samples per event, with no bucketing; quantiles are taken by sort and index in jq.
@row_add n=19 row=decision/gauges-from-harness-snapshots area=metric-coverage rec=7
name: Gauges from harness snapshots
rule: A current-value gauge is read from harness snapshots: budget.read_at, and liveness via agent-run status.
@row_add n=20 row=contract/derived-metrics/viola-hook-duration-ms area=metric-coverage rec=8
boundary: derived metrics
item: viola.hook.duration_ms
promise: Hook latency per hook_event, a histogram in ms of raw hook-decision duration_ms samples; its gate is the hyperfine max.
@row_add n=21 row=contract/derived-metrics/viola-channel-duration-ms area=metric-coverage rec=9
boundary: derived metrics
item: viola.channel.duration_ms
promise: Channel call latency per method, a histogram in ms of raw channel-response duration_ms samples.
@row_add n=22 row=contract/derived-metrics/viola-send-confirm-ms area=metric-coverage rec=10
boundary: derived metrics
item: viola.send.confirm_ms
promise: Send to readback latency, a histogram in ms of the duration_ms of send-confirmed lines whose confirmed is not false; a sample runs from the send's arrival at the wrapper, so it includes the readiness gate's wait ahead of the window.
@row_add n=23 row=contract/derived-metrics/viola-send-outcome area=metric-coverage rec=11
boundary: derived metrics
item: viola.send.outcome
promise: Send terminal state, a counter of send-confirmed by confirmed and send-refused by refusal, detail and side.
@row_add n=24 row=contract/derived-metrics/viola-wait-outcome area=metric-coverage rec=12,19
boundary: derived metrics
item: viola.wait.outcome
promise: A counter of channel-response outcome for method wait, counting only the wait calls answered by a channel-response; a wait refused before any frame writes none.
@row_add n=25 row=contract/derived-metrics/viola-state-recovered area=metric-coverage rec=13
boundary: derived metrics
item: viola.state.recovered
promise: Heal and recovery under chaos, a counter of state-recovered by detail.
@row_add n=26 row=contract/derived-metrics/viola-parse-skipped area=metric-coverage rec=14
boundary: derived metrics
item: viola.parse.skipped
promise: Skipped records, a gauge read from the skipped field of list --json.
@row_add n=27 row=contract/derived-metrics/viola-parse-rejected area=metric-coverage rec=15
boundary: derived metrics
item: viola.parse.rejected
promise: Parse rejections per parser, a counter of parse-rejected by parser, detail and count.
@row_add n=28 row=contract/derived-metrics/viola-compat-unknown area=metric-coverage rec=16
boundary: derived metrics
item: viola.compat.unknown
promise: A counter of the unknown fallback, from send-refused with refusal unknown.
@row_add n=29 row=contract/derived-metrics/viola-compat-v-rejected area=metric-coverage rec=16
boundary: derived metrics
item: viola.compat.v_rejected
promise: A counter of higher-version rejections alone, from channel-response with error_code -32602 and detail unsupported-version; invalid send params and a bad link or unlink name read invalid-params, and release carrying from reads release-from-driver.
@row_add n=30 row=contract/derived-metrics/viola-child-exit area=metric-coverage rec=17
boundary: derived metrics
item: viola.child.exit
promise: Child and probe exits, a counter of process-exit by subject, child_exit_status and exit_source.
@row_add n=31 row=contract/derived-metrics/viola-panic area=metric-coverage rec=18
boundary: derived metrics
item: viola.panic
promise: A counter of panic events, an SLO that must stay 0.
@row_add n=32 row=contract/cli-process-exit/server-verify-failed area=metric-coverage rec=19
boundary: CLI process-exit diagnostic line
item: server-verify-failed
promise: The CLI's server-verification refusal writes one process-exit with subject self, exit_code 21, detail server-verify-failed and during connect at WARN in cli-<name>.ndjson; MCP's wait tool writes no line for it.
@row_add n=33 row=decision/quantiles-report-only area=metric-coverage rec=21
name: Quantiles are report-only
rule: Latency quantiles are for reports only; the gates use the max, and the quantile read skips a torn line rather than aborting.
@row_add n=34 row=decision/no-print-macros area=obs-anti-patterns rec=22
name: No print macros in product code paths
rule: The hook, run and mcp code paths of the product crates never use print!, println!, eprint!, eprintln! or dbg!; only the output modules owning --json, human CLI stderr and the hook decision body may carry a local allow, and only where they use a print macro.
why: Print macros bypass the file sink and break the Hook contract, the run terminal rule and MCP framing.
guard: workspace clippy lints print_stdout, print_stderr, dbg_macro set to deny
except: viola-harness and the fake agent are exempt.
@row_none n=1 rec=23 type=5
@row_none n=2 rec=24 type=5
@row_add n=35 row=decision/linux-only-os-target area=obs-scope-summary rec=25
name: Linux-only OS target
rule: The OS target is Linux only: the core on any current distribution, CI on ubuntu-latest.
@row_none n=3 rec=26 type=5
@row_none n=4 rec=27 type=5
@row_none n=5 rec=28 type=9
@row_none n=6 rec=29 type=9
@row_none n=7 rec=30 type=9
@row_none n=8 rec=31 type=9
@row_none n=9 rec=32 type=S
@row_none n=10 rec=33 type=0
what: an instrumentability tag with no entity named
@row_add n=36 row=decision/anyhow-error-capture-boundary area=obs-scope-summary rec=34
name: The anyhow crate is the error-capture boundary
rule: The only crate that uses anyhow is the error-capture boundary, with context chains only at dispatch; the full chain goes to diagnostics only.
@row_add n=37 row=decision/serde-chain-fixed-message area=obs-scope-summary rec=34
name: Serde error chains become a fixed message on stderr
rule: An error chain that contains a serde source becomes a fixed message before it reaches stderr.
@row_none n=11 rec=35 type=S
@row_none n=12 rec=36 type=0
what: an instrumentability tag with no entity named
@row_add n=38 row=contract/unreachable-cause/strict-modes-failed area=obs-scope-summary rec=37
boundary: exit 21 cause words
item: strict-modes-failed
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_add n=39 row=contract/unreachable-cause/instance-dead area=obs-scope-summary rec=37
boundary: exit 21 cause words
item: instance-dead
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_add n=40 row=contract/unreachable-cause/unwrapped-name area=obs-scope-summary rec=37
boundary: exit 21 cause words
item: unwrapped-name
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_add n=41 row=contract/unreachable-cause/endpoint-lost area=obs-scope-summary rec=37
boundary: exit 21 cause words
item: endpoint-lost
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_add n=42 row=contract/unreachable-cause/server-verify-failed area=obs-scope-summary rec=37
boundary: exit 21 cause words
item: server-verify-failed
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_add n=43 row=contract/unreachable-cause/socket-dir-refused area=obs-scope-summary rec=37
boundary: exit 21 cause words
item: socket-dir-refused
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_none n=13 rec=38 type=S
@row_none n=14 rec=39 type=0
what: an instrumentability tag with no entity named
@row_add n=44 row=decision/pty-byte-stream-never-logged area=obs-scope-summary rec=40
name: PTY byte stream never logged
rule: The PTY byte stream is never logged; the openpty OS API behind the PTY seam is instrumented at its boundary only.
why: The PTY byte stream is user content, and nobody parses the rendered screen for content.
@row_none n=15 rec=41 type=S
@row_none n=16 rec=42 type=0
what: an instrumentability tag with no entity named
@row_add n=45 row=decision/ipc-crate-sync-instrumentation area=obs-scope-summary rec=43,41
name: Synchronous instrumentation in the IPC crate
rule: Instrumentation in the interprocess IPC crate must be synchronous.
why: Stack forbids Tokio in the crate's graph unless its feature is enabled.
@row_none n=17 rec=44 type=S
@row_none n=18 rec=45 type=0
what: an instrumentability tag with no entity named
@row_add n=46 row=decision/state-crate-sync-instrumentation area=obs-scope-summary rec=46,44
name: Synchronous instrumentation in the state crate
rule: Instrumentation in the state crate built on ndjson, atomic-write-file and sysinfo must be synchronous.
why: Stack bans Tokio in that crate.
@row_none n=19 rec=47 type=S
@row_none n=20 rec=48 type=0
what: an instrumentability tag with no entity named
@row_add n=47 row=decision/drift-reports-diagnostics-only area=obs-scope-summary rec=49
name: Drift reports go to diagnostics only
rule: Drift reports go to diagnostics only and nowhere else.
why: Its outputs are the most sensitive in the product.
@row_add n=48 row=decision/stripped-claude-values-never-logged area=obs-scope-summary rec=49
name: Stripped CLAUDE values never logged
rule: R8-stripped CLAUDE* values are never logged anywhere.
why: Its outputs are the most sensitive in the product.
@row_none n=21 rec=50 type=S
@row_none n=22 rec=51 type=0
what: an instrumentability tag with no entity named
@row_add n=49 row=decision/mcp-no-telemetry-on-stdout area=obs-scope-summary rec=52
name: No telemetry on the MCP server's stdout
rule: The MCP server sends no telemetry to stdout.
why: stdout is reserved for MCP frames.
@row_none n=23 rec=53 type=S
@row_none n=24 rec=54 type=0
what: an instrumentability tag with no entity named
@row_add n=50 row=decision/claude-boundary-only-instrumentation area=obs-scope-summary rec=55
name: Boundary-only instrumentation of claude
rule: The hosted claude binary is instrumented only at its boundary: spawn and exit, paste writes, and the vt100 readiness-gate outcome.
why: claude is a third-party binary run unmodified.
@row_add n=51 row=decision/claude-exit-on-process-handle area=obs-scope-summary rec=55
name: Claude exit detected on the process handle
rule: The exit of the hosted claude process is detected on the process handle, never on EOF.
@row_add n=52 row=decision/claude-internals-not-a-contract area=obs-scope-summary rec=55
name: Claude internals never read as a contract
rule: The hosted claude's internal state and transcripts are never read as a contract.
