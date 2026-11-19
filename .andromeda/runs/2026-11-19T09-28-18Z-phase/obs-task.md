@task_prose part=chunk
Lands a fourth recorded claude fixture set for claude 2.1.328, made by a clean live `viola verify --record` and a `viola-mcp-tap` re-record under the rig's private config, copied whole from the recorders' scrubbed output with no file edited by hand.
Points the capability-ledger probes' STAMPED value at the new set.
Re-runs the drift suites and the recorded-exchange suites over the new set, which the CI fake agent replays. A red drift suite on the landed set stops implement.
Disposes of the entry's three carries by name: the placement, the cap word needed before any live start, and the half-unread /context post-condition on 2.1.328, which is read from a readable transcript beside the recording or named as leaving.
This phase makes no live start and reads nothing of the operator's real claude home.
@task_file n=1 path=fixtures/claude/2.1.328/
what: the new recorded claude fixture set for 2.1.328: recorded fixtures of the wrapped claude CLI (verify recording plus the MCP tap re-record), scrub-required, which the fake agent replays in CI
@task_item n=1 part=needs
The recorded fixture set as an artifact: claude fixtures are scrub-required. Every recorded file passes the recorder's hygiene and scrub, and a payload the hygiene refuses is a red, never scrubbed by hand. Recorded fixtures are no telemetry source.
@task_item n=2 part=needs
`viola verify --record` against the real claude CLI, run live and only outside CI. verify is the only stamps writer. Its log lines carry no content, and it logs only inside an instance. Its stdout step and summary lines, exit codes and process-exit detail (interrupted, strict-modes-failed) are what the recording run shows.
@task_item n=3 part=needs
verify's child spawn subjects during a recording (version-probe, agents-probe, verify-probe, verify-pty-probe) and their process-exit lines with child_exit_status and duration_ms. claude agents output is untrusted, and a parse failure reads as unknown.
@task_item n=4 part=needs
The viola mcp stdio server, its tools (answer, last, list, send, wait) and its stdout carrying frames only, as the MCP tap re-record captures them. MCP rejections carry codes only.
@task_item n=5 part=needs
The capability ledger and its stamps: the probes' STAMPED names the new set. Ledger path and stamps are never logged. An unreadable ledger counts as an unverified CLI. The run log's parse-rejected line for the ledger-stamps parser applies.
@task_item n=6 part=needs
CI verify runs against the fake agent only, and real-CLI verify never runs in CI. The drift suites and recorded-exchange suites replay recorded sets against the fake agent, and the fake's version-keyed literals are what a 2.1.328 difference would reach.
@task_item n=7 part=needs
Drift reports go to the diagnostics detail file only (the detail line's drift_report field), never to the terminal or home-level lines.
@task_item n=8 part=needs
The 2.1.328 differences read statically from the bundle: a trust prompt appears only with --add-dir, and the trust dialog has a decline text. Dialog content and the browser-tools prompt text are never logged. Trust-record writes and the reading of the browser-tools prompt stay uninstrumented.
@task_item n=9 part=needs
The /context post-condition carry: its stdout-tag half is read from a readable session transcript beside the next recording. Claude Code transcripts are not a contract, and the transcript echo read stays uninstrumented. The not-delivered:no-prompt-submitted send-refused detail is the reading carried over from the rig's L2 send.
@task_item n=10 part=needs
CLAUDE environment strip on the claude child during recording: stripped CLAUDE values and credentials are never logged, the strip is logged as names only, and the identity floor is always stripped.
@task_item n=11 part=needs
Secrets and personal content stay out of everything recorded or uploaded: no secrets in log lines, prompt and assistant output never in home-level logs, and conversation text only in the instance detail file. CI artifacts are uploaded only after a passing secret scan.
@task_item n=12 part=needs
Tests and test homes: every test start runs under a test-owned user home, kept under the scanned integration-test root until the gates run. A test that waits takes a bound under the 20 s lint and never a sleep-based log wait, and every process started is ended.
@task_item n=13 part=needs
The CI gates the landed set must pass: zero panic lines, schema conformance of emitted lines, grep gates and the secret scan, all failing closed. A nonzero error count blocks the chunk, and a re-run is no substitute for diagnostics.
@task_item n=14 part=needs
The live recording round is bounded: no retry policies or retry-once, and a host reading that is contended stops the round and is re-read rather than retried into. No mutation run in this chunk.
@task_item n=15 part=needs
The PTY seam during the recorded live session (spawn, paste, kill-fallback log events). The PTY byte stream and typed keys are never logged, and there is no telemetry from screen content.
@task_standing n=1
convention: each recorded fixture file is scrub-required, passes the recorder's hygiene, and is never a telemetry source
home: 1-obs-scope-summary · obs-scope-summary
@task_standing n=2
convention: each new or re-run test runs under a test-owned home inside the scanned integration-test root, kept until the gates run, and is gated by the panic, schema and secret-scan gates
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: each test that waits on a log or process uses a bounded wait under the lint, never a sleep, with no retry
home: 10-slo-invariants-telemetry-budgets · slo-invariants-telemetry-budgets
@task_standing n=4
convention: each fixture or artifact uploaded from CI holds only synthetic or scrubbed inputs and passes the secret scan first
home: 8-pii-scrubbing-compliance · pii-scrubbing-compliance
@task_item n=1 part=not_needed
Error capture and panic reporting rules (panic outcome by role, the anyhow edge, the future error reporter and its transport): the chunk adds no error path and no reporter.
@task_item n=2 part=not_needed
Span declarations and span attribute contracts (run start, confirmed send, wait and last, wheel, dialog and hook span chains, span kinds): the chunk declares or changes no span.
@task_item n=3 part=not_needed
Metric coverage (derived counters, gauges, histograms, quantiles): the chunk adds no metric and changes no grouping.
@task_item n=4 part=not_needed
Logging stack and logger initialization (subscriber setup, explicit writer, init order, home resolution, service identity constants, level filter): the chunk changes no logger code.
@task_item n=5 part=not_needed
Correlation and trace-context propagation (corr and conn fields, channel join keys, corr by event): the chunk adds no correlated line.
@task_item n=6 part=not_needed
Exit-code detail words and refusal contracts of the CLI, run start, revive preflight, link and unlink, and the exit 21 cause words: the chunk changes no exit path or refusal.
@task_item n=7 part=not_needed
Hook performance budgets and their hyperfine perf gates (spine hook, session-end hook): the chunk touches no hook code.
@task_item n=8 part=not_needed
Absences of an OTel exporter, network egress, hosted deployment, database, gRPC or a graphical frontend: the chunk adds none of these.
@task_item n=9 part=not_needed
Agent-run status and logs-wrapper output records and the list liveness and heartbeat staleness contracts: the chunk changes no harness reader.
