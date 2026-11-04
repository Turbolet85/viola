@task_prose part=chunk
Brings Linux and macOS to parity with Windows: on both Unix OSes the `viola run` wrapper runs its child under portable-pty's openpty backend, behind the same pty seam as ConPTY (spawn, read, write, resize, wait, kill).
Runs the full fake-agent suite on ubuntu-latest and macos-latest; a test left cfg(windows), cfg(unix) or cfg(target_os) carries the reason its behaviour belongs to one OS. Adds the Unix integration-tier witness of dim only on stale rows (v1-40).
Re-reads the obs gates G1 to G4 and the CI test, perf and lint jobs, which must be green on ubuntu-latest and macos-latest beside windows-2025.
Re-asserts one diagnostics schema on every OS: process-log lines, events.ndjson, snapshots and every --json document carry the same fields and closed words. Where an OS cannot fill a field, it reads its documented null, never an OS-only word.
Windows stays the live-supported target. No live claude session: everything runs on the fake agent.
@task_file n=1 path=.github/workflows/ci.yml mark=re-reads
what: the test, perf and lint jobs over matrix.os (windows-2025, macos-latest, ubuntu-latest) and the obs gate steps G1 to G4, read green on both Unix OSes
@task_file n=2 path=schemas/diag-line.v1.json mark=re-reads
what: one diagnostics schema, the same fields and closed words for process-log lines on every OS
@task_file n=3 path=tests/run_cli.rs mark=edits
what: its cfg(windows) cases and the named reason each keeps; the pin run_child_ends_when_its_wrapper_is_terminated stays cfg(windows)
@task_file n=4 path=tests/support/home.rs
@task_item n=1 part=needs
The viola-pty seam on Unix: openpty behind spawn, read, write, resize, wait and kill. It is instrumented at the seam only; the openpty OS API is boundary-only and the PTY byte stream is never logged.
@task_item n=2 part=needs
PTY-seam logging on every OS: spawn gives process-start with subject claude-child. The child's process-exit comes from the handle-wait thread, never from reader EOF. Kill-fallback use is logged as exit_source kill-fallback, and paste logs text_bytes only.
@task_item n=3 part=needs
The multi-platform exporter-compat trigger: the same file sink and line schema on every OS, with identical fields and no OS-specific exporter or sink.
@task_item n=4 part=needs
The required fields on every line (timestamp, level, target, message, event, corr, process, instance) and the additive field catalog. Null is encoded by key absence (the Null encoding rule), and the chunk's documented-null reading of a field one OS cannot fill must act on that rule.
@task_item n=5 part=needs
The closed event enum and the closed detail code catalog: no OS-only word, and no path or pid inside a detail code. This includes the Unix-reached details socket-dir-refused, server-verify-failed and strict-modes-failed.
@task_item n=6 part=needs
The macOS carve-out: macOS has no peer pid, so server verification there is an euid mismatch alone. A CLI wait or last ends with process-exit exit 21, detail server-verify-failed, during connect; MCP returns instance-unreachable with no line.
@task_item n=7 part=needs
Unix channel endpoint telemetry. A peer that accept_loop refuses by euid is dropped before any frame and writes no line; the client reads end-of-file. A refused socket directory gives detail socket-dir-refused before any channel-request line. channel-request and channel-response carry corr and conn on both sides.
@task_item n=8 part=needs
run's pre-child refusals on Unix: a socket directory that socket_dir_ready refuses (exit 1, before any snapshot, child or bind), or a home tree another user could write or read, writes exactly the fixed two-line stderr pair through human::refuse. No path or pid, and nothing of run's own on the terminal while the child runs.
@task_item n=9 part=needs
The viola revive start arm used by the kill-and-revive case: it logs as process run, makes the home-tree check before that log, and has its own refusals (state files another user could write, no logged session, a missing recorded directory).
@task_item n=10 part=needs
The panic hook and catch sites on every OS. There is one single-line event panic record in the home-level role file and none on stderr. The run PTY pump and handle-wait threads each run inside catch_unwind, and every profile keeps panic unwind.
@task_item n=11 part=needs
The zero-unlogged-panics invariant and the error budget: zero panic lines as G2 counts them, zero secret-scan hits and zero schema-conformance failures, measured by in-job gate exit codes on each OS.
@task_item n=12 part=needs
Gate G1 (bare instrument), G2 (scripts/g2-zero-panics.sh, fail-closed, --probe first, home-level role files only, one exemption for the fake-agent seam's forced panic at src/cmd/hook/seam.rs), G3 (no abort panic strategy) and G4 (both schemas, if always). Each is its own step with shell bash on all three OSes.
@task_item n=13 part=needs
G4 schema conformance against schemas/diag-line.v1.json and schemas/diag-detail.v1.json. Both are default-deny through a top-level unevaluatedProperties false, as measured in tests/contract_diag_schema.rs. An unknown event, a missing required field or a field outside the catalog fails the build.
@task_item n=14 part=needs
CI step order and conditions in the per-OS test job: tests first, then G2 and G4 if always, with AGENT_RUN_KEEP_HOMES=1. Harness capture runs on failure. The secret scan (id secret-scan) runs if always. The diag, harness and junit uploads per matrix.os are gated on the scan's success, and the hit report is uploaded on scan failure.
@task_item n=15 part=needs
The secret-scan test over every diagnostics file on every OS: Cookie header, CLAUDE values, the content canary, and diagnostics files at 0600 inside 0700 directories (DACL on Windows). It runs before artifact upload.
@task_item n=16 part=needs
The per-OS perf job: hyperfine max gates through gate --require perf over the five timed hook rows. Obs overhead on hook holds only when every hook command runs with VIOLA_NAME set and a home under target/e2e-home/.
@task_item n=17 part=needs
The lint job on every OS: clippy disallowed-macros, print_stdout, print_stderr and dbg_macro; scripts/lint-probes.sh (raw event outside viola_core obs); and cargo deny (no Tokio in the sync crates, no C-building crates, veil toggle banned).
@task_item n=18 part=needs
The liveness signal behind the v1-40 witness: run touches heartbeat every 1 s, and the state flips at more than 5 s (4.9 s live, 5.1 s stale). Short-lived readers read liveness through list --json. No line is logged per tick or per transition.
@task_item n=19 part=needs
The hook contract under the Unix suite: exit 0, stdout only the decision body (or the status-line output), stderr empty. A failed log write is swallowed. Without VIOLA_NAME, the hook is a silent no-op with no file opened (E1).
@task_item n=20 part=needs
The log file location and modes on every OS: home-level role files run, hook, mcp and cli under diagnostics, and content-bearing detail only in the instance detail file.
@task_item n=21 part=needs
Snapshot and --json documents across OSes: absolute paths in snapshot.json and config values are scrubbed from external output, and state-recovered lines (snapshot-replayed, torn-line-healed) keep the same detail words on every OS.
@task_item n=22 part=needs
The fake-agent boundary: in CI, viola verify runs only against the fake agent, and the fake-agent seam's deliberate panic is G2's only exemption. Neither may widen to make a Unix case pass.
@task_standing n=1
convention: every integration-test viola home lives under target/e2e-home/, so G2, G4, the secret scan and the per-OS diag upload cover it (the one chaos_feed_panic home excepted)
home: ci-integration
@task_standing n=2
convention: every new log line goes through obs_event! with the closed event enum and catalog-named typed fields only: no raw tracing level macros, no print or eprint, corr passed as a typed value, no path or pid in detail
home: log-coverage
@task_standing n=3
convention: every new span uses instrument with skip_all first, a static area.operation name and an explicit field allow-list; never err or ret, and never content in span fields
home: span-trace-coverage
@task_standing n=4
convention: every new CI gate is its own run step with shell bash on all three OSes, if always where it guards diagnostics; uploads come only after the secret scan passes, actions are pinned by commit SHA, and retention is 7 days
home: ci-integration
@task_standing n=5
convention: every test and fixture input is synthetic, so a detail-file upload stays admissible; no CLAUDE value, Cookie header or real content reaches any diagnostics file
home: pii-scrubbing-compliance
@task_standing n=6
convention: no retry-once policy for panics or flaky telemetry assertions (retries 0), and no log-based wait relying on sleeps; emission keys on hook events and events.ndjson offsets
home: slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
External error reporting: sentry or any reporter, the before_send conditions and the future-reporter Decisions Log precondition. v1 has no reporter, and the chunk adds none.
@task_item n=2 part=not_needed
OTel SDK init, the OTLP exporter, the deferred tracing-opentelemetry bridge, span kinds mapped to otel.kind, and W3C traceparent. None exists in v1, and parity adds none.
@task_item n=3 part=not_needed
The derived metric catalog and its naming (viola area measure, viola.compat.v_rejected) and label cardinality discipline. The chunk derives no new metric; its perf reading is the hyperfine gate named in the Needs.
@task_item n=4 part=not_needed
The not-triggered triggers (compliance-audit-trail, self-observing-inversion) and the obs tier justification in section 1. The chunk changes neither.
@task_item n=5 part=not_needed
Section 1's historical alignment gaps (a) to (c) for Phase 3 (release-from-driver as its own event, the hook-invoked corr timing). Earlier Decisions Log entries settled them, and parity reopens none.
@task_item n=6 part=not_needed
The Windows ConPTY sideload companion: the conpty owner and DACL check, sideload_fallback strict-modes-failed on process-start, and the windows-sys TerminateProcess fallback. These are Windows behaviours the scope holds unmoved.
@task_item n=7 part=not_needed
The dispatch-only windows-mutants.yml workflow (the boundary audit's Windows mutation leg). The chunk neither runs it nor changes it.
@task_item n=8 part=not_needed
Operational retention and the audit-log-retention trigger: no rotation of events.ndjson or diagnostics in v1. The chunk adds no rotation.
@task_item n=9 part=not_needed
Metrics-endpoint and in-process-instrument bans, proprietary APM and dashboards-only bans, and the self-instrumentation ban. The chunk adds no listener, meter, dashboard or collector.
@task_item n=10 part=not_needed
Service identity resolution (service.name viola and version from viola-core constants, never from OTEL env vars). Parity changes no resource attribute.
