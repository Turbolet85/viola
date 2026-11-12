@row_add n=1 row=decision/product-is-not-a-telemetry-collector area=11-obs-anti-patterns-never-do-these rec=1
name: Product is not a telemetry collector
rule: viola never self-instruments as if the product were telemetry: events.ndjson is not turned into a collector and no OTLP receiver is added.
why: viola is Normal mode (Founder Direction 3)
@row_add n=2 row=decision/service-identity-from-constants area=11-obs-anti-patterns-never-do-these rec=2
name: Service identity from compiled constants
rule: Service name and version come from viola-core constants, never from env vars such as OTEL_SERVICE_NAME or OTEL_RESOURCE_ATTRIBUTES; any future OTel Resource starts from an empty builder.
@row_add n=3 row=decision/scrub-before-bytes-leave-process area=11-obs-anti-patterns-never-do-these rec=3
name: Scrubbing before any byte leaves the process
rule: Every byte is scrubbed of secrets before it leaves the process; in v1 the only destination is local files.
guard: the secret-scan test
@row_none n=1 rec=4 type=S
@row_none n=2 rec=5 type=S
@row_add n=4 row=decision/ci-artifacts-gated-on-secret-scan area=ci-integration rec=6,7,8,9
name: CI artifacts uploaded only after a passing secret scan
rule: Diagnostics, harness logs, JUnit XML and perf samples are uploaded on failing runs too, but only when the job's secret scan succeeded; a failed scan withholds every artifact.
why: a bare always() or failure() would also upload after a failed scan
@row_add n=5 row=decision/harness-logs-captured-before-scan area=ci-integration rec=7
name: Harness logs captured before the secret scan
rule: On test failure the agent-run logs and status are captured before the secret scan, so the scan covers them, and they go in an artifact of their own, not the diagnostics artifact.
why: upload artifacts are immutable, so a second upload under the same name conflicts, and overwriting would replace the raw diagnostics
@row_add n=6 row=absence/otel-trace-export area=ci-integration rec=10
capability: An OpenTelemetry trace exporter
@row_add n=7 row=decision/no-raw-tracing-macros area=ci-integration rec=11
name: No raw tracing macros outside the obs module
rule: Raw tracing info, warn, error, debug and trace macros and raw event! are refused outside viola_core::obs.
guard: clippy -D warnings disallowed-macros; raw-event! grep and ban probes in scripts/lint-probes.sh
@row_add n=8 row=decision/no-print-macros area=ci-integration rec=11
name: No print or dbg macros
rule: Printing to stdout or stderr through print macros and dbg! is banned in the workspace.
guard: clippy -D warnings print_stdout, print_stderr, dbg_macro
@row_add n=9 row=decision/sync-crates-without-tokio area=ci-integration rec=11
name: Sync crates build without Tokio
rule: The synchronous crates must compile without Tokio.
guard: cargo check of the sync crates without Tokio
@row_add n=10 row=decision/banned-telemetry-dependencies area=ci-integration rec=11
name: Banned telemetry and logging dependencies
rule: opentelemetry-otlp, opentelemetry-stdout, sentry, tracing-appender, the tracing-subscriber env-filter feature and the veil toggle feature are not taken as dependencies.
guard: cargo deny check
@row_add n=11 row=decision/instrument-requires-skip-all area=ci-integration rec=11,17
name: Instrument attribute must skip all arguments
rule: Every #[instrument] in the workspace takes skip_all as its first argument; a bare #[instrument] fails the build.
guard: gate G1
@row_add n=12 row=decision/ci-gates-fail-closed area=ci-integration rec=17,18
name: CI gates fail closed
rule: A gate whose tool is missing, errors or meets an empty scope fails the step rather than passing.
why: an erroring search with a must-be-empty check would pass on every commit
guard: G1, G3, scripts/g2-zero-panics.sh --probe
@row_add n=13 row=decision/diag-lines-schema-checked-in-unit-tests area=ci-integration rec=12
name: Emitted lines schema-checked in unit tests
rule: Unit tests write to an in-memory writer and parse every emitted line against schemas/diag-line.v1.json.
@row_add n=14 row=decision/test-homes-under-scan-root area=ci-integration rec=13,20,26
name: Test homes live under the scanned root
rule: Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it.
why: a hook panic exits 0, so a home outside that root would hide it
except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
@row_add n=15 row=decision/perf-budget-asserted-on-max area=ci-integration rec=13,9
name: Perf budgets asserted on the maximum sample
rule: Performance budgets are asserted on the max of hyperfine samples, and a failing perf gate still leaves its samples.
@row_add n=16 row=decision/zero-panics-gate area=ci-integration rec=14,18
name: Zero panics gate
rule: No panic line may appear in the home-level role files of test runs; detail files repeat the panic line and are excluded.
why: ratified
guard: gate G2 (scripts/g2-zero-panics.sh, --probe first)
except: a panic whose panic_location is exactly src/cmd/hook/seam.rs:<digits>, the fake-agent seam's forced panic, compared as a whole string
@row_add n=17 row=decision/os-gated-test-census area=ci-integration rec=15
name: OS-gated test census
rule: Every OS-gated test function or file must be named in the census with a non-empty reason, and a gate predicate naming windows or macos is a fault even when named.
guard: contract_os_gated_tests
@row_add n=18 row=decision/obs-code-mutation-tested area=ci-integration rec=16
name: Obs code mutation-tested like product code
rule: Obs code is mutation-tested like product code at the epoch boundary and on demand, not on push or pull request; a timed-out or missed obs-code mutant, or unviable outnumbering caught, is red.
guard: test-plan Mutation gate
@row_add n=19 row=decision/schema-conformance-gate area=ci-integration rec=14,19
name: Schema conformance gate on real process output
rule: Every role-file line of test homes must validate against schemas/diag-line.v1.json and every detail line against schemas/diag-detail.v1.json; a failing line fails the build, a torn non-JSON line is skipped and counted.
why: schema failures count as a build failure and unit tests check only in-memory lines
guard: gate G4 (schema-conformance step)
@row_add n=20 row=decision/gate-output-omits-line-content area=ci-integration rec=19
name: Gate output never prints a line's content
rule: A failing schema check prints file, line number and failing keyword, never the line's content.
why: detail lines carry the canary
@row_add n=21 row=decision/panic-unwind-required area=ci-integration rec=19,24
name: Panic strategy is unwind
rule: Every Cargo profile keeps panic = "unwind"; no abort strategy in profiles, rustflags, env or workflows.
why: abort turns every catch_unwind into an abort, so hook would die by SIGABRT instead of exiting 0
guard: gate G3
@row_add n=22 row=decision/gates-run-after-failed-tests area=ci-integration rec=20
name: Gates run even after failed tests
rule: G2 and G4 run if: always(), so a failed test step never skips them.
why: failing runs are where panic lines are most likely, and panics are measured only through G2's exit code
@row_add n=23 row=decision/test-homes-kept-until-gates area=ci-integration rec=20,21
name: Test homes kept until gate steps run
rule: CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it.
why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
@row_add n=24 row=decision/raw-unsymbolised-backtrace area=error-capture-reporting rec=22
name: Panic backtraces captured raw, never symbolised
rule: A panic's detail line carries raw frames (ip, module path, base, offset), up to 62, for offline resolution against the pinned copy; nothing is symbolised in the panic path.
why: symbolising cost most of a hook run and pushed the forced-panic hook past the 1.0 s spine bound
@row_add n=25 row=decision/env-vars-not-config-channel area=error-capture-reporting rec=22
name: Env vars are not a configuration channel
rule: Backtrace capture is not gated by RUST_BACKTRACE or any env var.
why: env vars are not a configuration channel
@row_none n=3 rec=23 type=4
relied: A refused diagnostics-path check is exactly a failed open: the role writer ends at a sink and the detail line is dropped.
@row_add n=26 row=decision/exit-decided-at-catch-site area=error-capture-reporting rec=24
name: Exit decided at a catch site, not the panic hook
rule: The panic hook only writes; everything after the hook is set runs inside one catch_unwind in main, so no role ends through an uncaught unwind.
@row_add n=27 row=decision/role-classified-before-clap area=error-capture-reporting rec=24
name: Role taken from argv before clap
rule: The role is the first argument that is neither a global flag nor a flag's value and names a role subcommand, so viola --home <dir> hook stop is still a hook whose pre-init panic exits 0.
why: reading argv[1] alone would misclassify that form as cli and exit 1
@row_add n=28 row=contract/cli-internal-error/json-document area=error-capture-reporting rec=24
boundary: cli internal error
item: --json document
promise: Under --json the cli catch site prints {"v":1,"error":"internal-error","detail":null} on stdout, stderr empty.
@row_add n=29 row=contract/cli-internal-error/human-line area=error-capture-reporting rec=24
boundary: cli internal error
item: error: internal error
promise: Without --json the cli catch site prints exactly error: internal error; run, revive's start, mcp and hook print neither.
@row_add n=30 row=decision/run-worker-panic-ends-run area=error-capture-reporting rec=24
name: Run worker panic ends the session
rule: A panic in the PTY pump or handle-wait thread is sent to main, which takes run's exit path; a worker panic leaving main running with no process-exit is forbidden.
why: the session cannot continue without either thread
@row_add n=31 row=decision/channel-connection-panic-contained area=error-capture-reporting rec=24
name: Channel connection panic contained
rule: A panic in one channel connection worker logs channel-response error -32603, closes that connection, and run continues.
@row_add n=32 row=decision/vt100-feed-panic-contained area=error-capture-reporting rec=24,26
name: vt100 feed panic contained
rule: A panic in a vt100 feed or resize is caught on the feed thread, poisons the screen model until the host size changes, writes one parse-rejected per episode, and run continues.
@row_none n=4 rec=25 type=4
relied: Refusals of the instance and home-tree checks that run before a role file opens write no line in any log.
@row_none n=5 rec=27 type=5
@row_add n=33 row=contract/mcp-frame-tools/strict-modes-failed area=error-capture-reporting rec=28
boundary: viola mcp frame tools
item: strict-modes-failed
promise: A refused instance check returns isError instance-unreachable with detail strict-modes-failed, and writes no frame and no line.
@row_add n=34 row=contract/mcp-frame-tools/socket-dir-refused area=error-capture-reporting rec=29
boundary: viola mcp frame tools
item: socket-dir-refused
promise: A refused socket directory returns isError instance-unreachable with detail socket-dir-refused, and writes no frame and no line.
@row_none n=6 rec=30 type=4
relied: Each MCP refusal maps to exactly one UnreachableCause word.
@row_add n=35 row=contract/mcp-stdin/oversize-line area=error-capture-reporting rec=31,32
boundary: viola mcp stdin
item: line past MAX_FRAME
promise: An oversize stdin line gives no isError result and ends the session as a closed stdin: exit 0 after initialize, exit 1 before; its one record is parse-rejected mcp-stdin oversize.
@row_add n=36 row=decision/single-sanitising-fault-reader area=error-capture-reporting rec=33
name: One sanitising reader of peer faults
rule: A peer's fault is read only through one sanitiser: five known codes read fixed messages with data admitted only in validated shapes, a code outside them reads wrapper fault, and a fault without an integer code reads nothing.
@row_add n=37 row=decision/peer-replies-screened-as-code-words area=error-capture-reporting rec=34
name: Peer replies screened as code words
rule: A peer reply's refusal and detail are kept only as code words (1-64 bytes of [a-z0-9-], letter first); an unknown code word stays visible and exits 14.
why: by design
@row_none n=7 rec=35 type=5
@row_none n=8 rec=36 type=4
relied: The NotADirectory refusal message is fixed and names no path.
@row_add n=38 row=contract/answer-send-exit-2/stderr-line area=error-capture-reporting rec=37
boundary: viola answer and viola send exit 2
item: stderr line
promise: Exit 2 writes the fixed unusable-response or unusable-text line to stderr whatever --json says, with no document.
@row_add n=39 row=contract/exit-21-document/shape area=error-capture-reporting rec=38
boundary: exit 21 document
item: shape
promise: Exit 21 prints one document per cause: {"v":1,"error":"instance-unreachable","detail":"<cause>"}.
@row_add n=40 row=contract/exit-21-document/detail area=error-capture-reporting rec=38
boundary: exit 21 document
item: detail
promise: One of six closed words: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused.
@row_add n=41 row=contract/diag-line-fields/process-start area=log-coverage rec=39
boundary: diagnostic line additive fields
item: process-start
promise: subject (self, claude-child, version-probe, verify-probe, verify-pty-probe, agents-probe, statusline-shell), service_name, version, os, pid, child_pid, endpoint_kind, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept (names only).
@row_add n=42 row=contract/diag-line-fields/process-exit area=log-coverage rec=40
boundary: diagnostic line additive fields
item: process-exit
promise: subject, exit_code (self), child_exit_status, shell_exit_status, exit_source (handle-wait or kill-fallback), detail, during (connect or call), duration_ms.
@row_add n=43 row=contract/diag-line-fields/channel-request area=log-coverage rec=41
boundary: diagnostic line additive fields
item: channel-request
promise: method, conn / srv_conn, from, from_trust, sender, v, after, timeout_ms.
@row_add n=44 row=contract/diag-line-fields/channel-response area=log-coverage rec=42
boundary: diagnostic line additive fields
item: channel-response
promise: method (only a closed-list name), conn / srv_conn, result_class (ok, refusal, error), refusal, detail (only the three -32602 faults), error_code (-32700, -32600, -32601, -32602, -32603), outcome, duration_ms.
@row_add n=45 row=contract/diag-line-fields/send-events area=log-coverage rec=43
boundary: diagnostic line additive fields
item: send-issued / send-confirmed / send-refused
promise: from, from_trust, text_bytes (normalised typed length), confirmed, refusal, detail, side (client or wrapper), wheel, window, used_percentage, resets_at, paused, duration_ms, and wrapper side only conn / srv_conn and rpc_id.
@row_add n=46 row=contract/diag-line-fields/dialog-events area=log-coverage rec=44
boundary: diagnostic line additive fields
item: dialog-raised / dialog-answered
promise: dialog_kind, hook_event, from, from_trust, deadline_hit, duration_ms.
@row_add n=47 row=contract/diag-line-fields/hook-events area=log-coverage rec=45
boundary: diagnostic line additive fields
item: hook-invoked / hook-decision
promise: hook_event (one of ten closed hook names incl. statusline), stdin_bytes, invoked_at, decision_emitted, deadline_hit, budget_written, detail, duration_ms.
@row_add n=48 row=contract/diag-line-fields/panic area=log-coverage rec=46
boundary: diagnostic line additive fields
item: panic
promise: panic_location (workspace-relative, or <crate>-<ver>/src/... for dependencies) and thread; payload and backtrace go to the detail file only.
@row_add n=49 row=contract/diag-line-fields/release-from-driver area=log-coverage rec=47
boundary: diagnostic line additive fields
item: release-from-driver
promise: conn, from (only when a valid ViolaName), from_trust (self-reported).
@row_add n=50 row=contract/diag-line-fields/state-recovered area=log-coverage rec=48
boundary: diagnostic line additive fields
item: state-recovered
promise: detail (torn-line-healed, snapshot-replayed, snapshot-unsupported-v), file (basename only), offset (torn-line-healed only), v_seen (snapshot-unsupported-v only), scanned (snapshot details only: the instance name when valid, never a path or null).
@row_add n=51 row=contract/diag-line-fields/parse-rejected area=log-coverage rec=49
boundary: diagnostic line additive fields
item: parse-rejected
promise: parser (claude-agents-json, vt100-feed, config-json, ledger-stamps, hook-stdin, channel-frame, mcp-stdin, prompt-submitted, paste-text, state-entry), detail, count.
@row_add n=52 row=contract/process-exit-detail/exit-1 area=log-coverage rec=50
boundary: process-exit detail
item: exit 1
promise: already-live, stale-heartbeat, squatted-name, pinned-hash-mismatch, internal-error, socket-dir-refused, and from revive's preflight no-session, session-live, cwd-missing; each one process-exit line with no child started.
@row_add n=53 row=decision/revive-preflight-order area=log-coverage rec=50
name: Revive start preflight order
rule: Revive's start preflight reads the holder, then the session, then session-live, then the directory, and stops at the first refusal.
@row_add n=54 row=decision/session-live-check-fails-open area=log-coverage rec=50
name: Session-live check fails open
rule: A failed, oversize or malformed claude agents read passes the session-live check; the check is skipped under --fork.
@row_add n=55 row=contract/process-exit-detail/interrupted area=log-coverage rec=50
boundary: process-exit detail
item: interrupted
promise: A viola verify stopped by SIGINT or SIGTERM before its rows are checked ends with one process-exit exit 1 detail interrupted, after every spawn pair it opened is closed.
@row_add n=56 row=contract/process-exit-detail/verify-refusal area=log-coverage rec=50
boundary: process-exit detail
item: viola verify refusal
promise: Under a parsed VIOLA_NAME, verify writes its refusal code: program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture or interrupted; a failing row carries no code.
@row_add n=57 row=decision/unemittable-schema-words-removed area=log-coverage rec=51
name: Schema words no build can emit are removed
rule: A closed word of the diag-line schema that no build can emit leaves the schema with its emitter; os is closed at linux.
@row_add n=58 row=contract/process-exit-detail/exit-21 area=log-coverage rec=52
boundary: process-exit detail
item: exit 21
promise: instance-dead, unwrapped-name, endpoint-lost, strict-modes-failed, server-verify-failed, socket-dir-refused.
@row_add n=59 row=contract/process-exit-detail/server-verify-failed area=log-coverage rec=52
boundary: process-exit detail
item: server-verify-failed
promise: The eight CLI frame verbs write it at WARN with exit 21, during connect, on a server of another euid, not the recorded wrapper, or an unverifiable check, with no pid, start time, path or endpoint.
@row_add n=60 row=contract/process-exit-detail/exit-20 area=log-coverage rec=53
boundary: process-exit detail
item: exit 20
promise: wrapper-fault.
@row_add n=61 row=contract/hook-decision-detail/fail-open-codes area=log-coverage rec=54
boundary: hook-decision detail
item: fail-open codes
promise: oversize-stdin, malformed-json, channel-unreachable, server-verify-failed, socket-dir-refused, strict-modes-failed, unverified-cli, deadline; no hook arm writes strict-modes-failed.
@row_add n=62 row=contract/hook-decision-detail/statusline-codes area=log-coverage rec=54
boundary: hook-decision detail
item: statusline codes
promise: viola hook statusline writes only oversize-stdin, malformed-json and deadline; a refused instance check writes no line.
@row_add n=63 row=decision/statusline-command-deadline area=log-coverage rec=54
name: Statusline command deadline
rule: The user's statusline command is bounded at 5 s, reported as deadline.
state: provisional
@row_add n=64 row=contract/hook-decision-detail/server-verify-failed area=log-coverage rec=54
boundary: hook-decision detail
item: server-verify-failed
promise: viola hook writes it at WARN on a server of another euid, not the recorded wrapper, or an unverifiable check; on the dialog path after hook-invoked, with decision_emitted false.
@row_add n=65 row=contract/hook-decision-detail/channel-unreachable area=log-coverage rec=54
boundary: hook-decision detail
item: channel-unreachable
promise: Written when the channel cannot be reached, including a replayed or absent snapshot; the hook fails open and SessionEnd takes its direct append.
@row_add n=66 row=contract/parse-rejected-detail/malformed area=log-coverage rec=55
boundary: parse-rejected detail
item: malformed
promise: Any parser.
@row_add n=67 row=contract/parse-rejected-detail/unreadable area=log-coverage rec=55
boundary: parse-rejected detail
item: unreadable
promise: config-json, ledger-stamps, state-entry.
@row_add n=68 row=contract/parse-rejected-detail/oversize area=log-coverage rec=55
boundary: parse-rejected detail
item: oversize
promise: channel-frame beyond MAX_FRAME, hook-stdin, mcp-stdin past MAX_FRAME, vt100-feed when the bounded queue drops a copy (one line per poisoning episode), claude-agents-json past MAX_FRAME.
@row_add n=69 row=contract/parse-rejected-detail/unknown-keys area=log-coverage rec=55
boundary: parse-rejected detail
item: unknown-keys
promise: config-json skipped keys, counted in count.
@row_add n=70 row=contract/parse-rejected-detail/panicked area=log-coverage rec=55
boundary: parse-rejected detail
item: panicked
promise: vt100-feed under catch_unwind.
@row_add n=71 row=contract/parse-rejected-detail/control-character area=log-coverage rec=55
boundary: parse-rejected detail
item: control-character
promise: paste-text.
@row_add n=72 row=contract/parse-rejected-detail/symlink-ignored area=log-coverage rec=55
boundary: parse-rejected detail
item: symlink-ignored
promise: state-entry.
@row_add n=73 row=contract/parse-rejected-detail/strict-modes-failed area=log-coverage rec=55
boundary: parse-rejected detail
item: strict-modes-failed
promise: ledger-stamps: run's version gate refused the stamps file on the strict-modes check, and the read counts as unverified.
