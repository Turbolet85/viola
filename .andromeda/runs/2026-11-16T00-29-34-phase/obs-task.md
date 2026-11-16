@task_prose part=chunk
Documents and pins by tests the wake condition of `wait` (CLI `viola wait`, `wait --json`, MCP `wait`): which logged event kinds return a parked wait and which park it again, proved over the fake agent.
Adds the behaviour that a turn ending while a background task still holds the session wakes `wait` (the turn-ended line carrying background_tasks, or the question it raises), never parking past it.
Adds the behaviour that once a turn ends with no background task, a later notification-class (log-only) line gives no false wake to a fresh `wait` armed after that end.
Re-reads at P3 today's wake kinds versus log-only kinds and re-measures the driver-side instance (a turn end with background_tasks 0 while the session still waits on a notification returns the wait and the driver re-arms).
Takes the carried CI red: the stamped-home run-start test timing out at the ci profile's 20 s kill, fixed by measurement either by taking the stamped home's verify out of the measured time or by sizing the class's ci kill to the loaded time.
@task_file n=1 path=src/run/wait.rs mark=edits
what: the wrapper's wait feed — which logged kinds wake a parked wait and which are log-only — edits: the wake-kind set and its wake / log-only test cases
@task_file n=2 path=src/cmd/wait.rs
what: the CLI wait verb (`viola wait`, `wait --json`) a driver parks on
@task_file n=3 path=src/human/wait.rs
what: the CLI wait verb's human rendering of a wait outcome
@task_file n=4 path=crates/viola-mcp/src/tools.rs
what: the viola-mcp stdio server tools — the MCP `wait` tool
@task_file n=5 path=crates/viola-core/src/lib.rs
what: the product event kinds — the turn-ended, question and notification-class kinds of the event stream
@task_file n=6 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent that CI tests drive in place of claude — edits: its turn scripts (turn end with and without background tasks, later notification)
@task_file n=7 path=fixtures/fake-scripts mark=edits
what: the fake agent's scripted turn fixtures — the turn-end-with-background-task and task-free-end-then-notification scripts
@task_file n=8 path=tests/support/verify.rs mark=edits
what: the stamped-home integration-test support — a test home stamped by an unbounded `viola verify` — edits: the stamped_home fixture's verify call
@task_file n=9 path=.config/nextest.toml mark=edits
what: the nextest ci profile — the test class's slow-timeout kill at 20 s — edits: the ci profile's kill for the stamped-home class
@task_item n=1 part=needs
The `wait` outcome a driver receives — CLI wait, wait --json and the MCP wait tool — whose wake condition the chunk documents: the wait and last log fields, the channel-response outcome a wait returns with, and the wait log line's outcome.
@task_item n=2 part=needs
The event stream's line kinds the wait feed reads: turn-ended with its background_tasks count, the question a turn end raises, and the notification-class lines that are log-only; the event line's shape stays as it is and event values stay a closed set.
@task_item n=3 part=needs
New behaviour: a turn end while a background task still holds the session returns a parked wait, never parking past it.
@task_item n=4 part=needs
New behaviour: after a task-free turn end, a later notification-class line does not return a fresh wait armed after that end.
@task_item n=5 part=needs
The wait path's spans and log events (wait.block, run.wait_dispatch, the channel request and response pair of a wait) stay as declared; a wake that changes the outcome rides the existing closed outcome words.
@task_item n=6 part=needs
The wait's refusals on connect (socket-dir-refused, server-verify-failed, strict-modes-failed) on the CLI and MCP sides are unchanged by a wake-condition change and must still fire before any wait frame.
@task_item n=7 part=needs
Deterministic event emission: the fake agent's turn scripts must produce the same event order every run, so the wake and no-false-wake tests assert on a fixed stream.
@task_item n=8 part=needs
Tests that wait for a logged line must wait on the line itself, never by sleeping; a wake test bounds its park by a deadline, not by retry.
@task_item n=9 part=needs
No retry policies and no retry-once: neither the CI-red fix nor a re-armed wait may be a re-run of the timed-out test as its fix.
@task_item n=10 part=needs
CI verify runs against the fake agent only; a real-CLI verify never runs in CI and no live claude start is part of the proof.
@task_item n=11 part=needs
The stamped-home fixture's `viola verify`: its verify process-exit line, verify exit codes and its child spawn subjects (version-probe, verify-probe, verify-pty-probe, agents-probe) as the fixture's measured cost; verify lines carry no content and log only inside an instance.
@task_item n=12 part=needs
If the CI fix stamps a home once and shares or copies it: verify is the only writer of the capability-ledger stamps, an unreadable ledger reads as an unverified CLI, and the pinned copy is placed, never replaced — a copied home must keep those true and name the test isolation it costs.
@task_item n=13 part=needs
A shared or copied stamped home must still pass the home strict-modes check before diagnostics open: owner-only diagnostics files and directories, so a copied tree keeps the modes the home-tree check requires.
@task_item n=14 part=needs
The run start the timed-out test measures: the run start events' order in the events log and the run start process log lines, which the test still asserts after the fix.
@task_item n=15 part=needs
If the fix sizes the ci kill instead: the bound must be one the plan can show holding at load, asserted by an in-job exit code, and gates still run after failed or timed-out tests.
@task_item n=16 part=needs
Each new guard test carries a one-off remove-the-guard control named by the test's name alone; no mutation run is made for this chunk.
@task_standing n=1
convention: each new integration test's home lives under the one scanned test-home root and is kept until the gate steps run
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: lines emitted by a new integration test's processes are schema-checked against the diagnostics line schema
home: ci-integration · 9-ci-integration
@task_standing n=3
convention: a new test's run must leave zero panic lines in its home-level files; the zero-panics gate reads them
home: ci-integration · 9-ci-integration
@task_standing n=4
convention: a new or changed fake-agent script is synthetic input, never a recording of claude and never a telemetry source
home: pii-scrubbing-compliance · 8-pii-scrubbing-compliance
@task_standing n=5
convention: a new test waits on a logged line by reading it, never by a sleep, and asserts its budget by exit code
home: 10-slo-invariants-telemetry-budgets · slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
The send path — confirmed send spans, send log fields, send-refused details, the readiness gate and confirm window: send confirmation is named not this chunk's.
@task_item n=2 part=not_needed
The permission-dialog path — dialog-raised and dialog-answered lines, hook-decision details and fail-open codes, dialog hold: the wake condition reads turn-end lines, not dialogs.
@task_item n=3 part=not_needed
Panic capture and error reporting rows — panic outcome by role, detail-file chains and backtraces, future reporter rules: the chunk adds no catch site and no reporter.
@task_item n=4 part=not_needed
Redaction and scrubbing machinery — veil derives, default-deny field allow-list, secret-scan hit report: the chunk adds no logged field and no payload type.
@task_item n=5 part=not_needed
Hook performance budgets and their hyperfine perf job: the chunk touches no hook's latency.
@task_item n=6 part=not_needed
Link and unlink, wheel pause and release, revive preflight and revive list: no row of those verbs binds a wake condition.
@task_item n=7 part=not_needed
Metrics, gauges, histograms and the OTel, exporter, Sentry and hosted-deployment absences: the chunk adds no signal of those kinds.
@task_item n=8 part=not_needed
MCP stdin framing rows (MAX_FRAME, oversize line before initialize): the MCP wait tool's framing is not changed.
@task_item n=9 part=not_needed
Heartbeat ticks, liveness and staleness rows: the wait wake does not read liveness.
@task_item n=10 part=not_needed
The agents probe and hyprctl parse-rejected rows and claude agents output handling: not on the wait path.
