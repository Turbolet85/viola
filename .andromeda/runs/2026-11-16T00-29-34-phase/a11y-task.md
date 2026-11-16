@task_prose part=chunk
Documents and pins by tests the wake condition of `wait` (CLI `viola wait`, `wait --json`, MCP `wait`): which logged event kinds return a parked wait and which park it again, proved over the fake agent.
A turn end held by a background task wakes `wait` (the turn-ended line carrying background_tasks, or the question it raises); a task-free turn end's later log-only notification gives no false wake to a wait armed after that end.
Re-measures from the code and the fake agent whether a turn end reading 0 background tasks while the session still waits returns the wait and leaves the driver to re-arm, and re-reads today's wake kinds versus log-only kinds before stating them.
Takes the carried CI red as its own item: the stamped-home fixture's unbounded `viola verify` pushes the run-start test past the ci profile's 20 s kill; chosen by measurement between taking the stamping out of the measured time (a home stamped once and shared or copied) and sizing the class's ci kill to the loaded time.
No live claude start, no mutation run; each new guard test carries a one-off remove-the-guard control.
@task_file n=1 path=src/run/wait.rs mark=edits
what: the wrapper's wait feed — which logged kinds wake a parked wait and which are log-only — edits: the wake-kind and log-only-kind cases (wait_each_wake_kind_wakes, wait_log_only_kinds_never_wake)
@task_file n=2 path=src/cmd/wait.rs mark=edits
what: the CLI `viola wait` verb and its --json output
@task_file n=3 path=src/human/wait.rs mark=edits
what: the human output of `viola wait` — the waiting line and the timed-out result
@task_file n=4 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP server tools — edits: the MCP wait tool
@task_file n=5 path=crates/viola-core/src/lib.rs mark=edits
what: the event kinds of the event line catalog
@task_file n=6 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent and its turn scripts that the wait tests run over
@task_file n=7 path=fixtures/fake-scripts/ mark=edits
what: the fake agent's turn-script fixtures
@task_file n=8 path=tests/support/verify.rs mark=edits
what: the stamped-home test support — edits: the stamped_home fixture's `viola verify` call (within: None)
@task_file n=9 path=.config/nextest.toml mark=edits
what: the nextest profiles — edits: the ci profile's slow-timeout kill for the stamped-home test class
@task_item n=1 part=needs
The `viola wait` human output: the waiting line naming the session while it parks, unchanged in wording by the new wake condition.
@task_item n=2 part=needs
The `viola wait` timed_out result, human and --json, which a wait that is not woken still ends with.
@task_item n=3 part=needs
The MCP wait tool and its timeout_ms value: the tool returns on the same wake kinds as the CLI verb.
@task_item n=4 part=needs
The wait/last output as a critical path of the CLI: a wake on a background-task turn end or a question returns its result as static linear output.
@task_item n=5 part=needs
Time bounds fall on the driver only, never on the human: the re-arm after a turn end with no background task is the driver's, and no wait timeout is put on the human at the terminal.
@task_item n=6 part=needs
No harsh timeouts without warning: a wait's bound stays the caller-given timeout and ends in its timed-out result, never a silent drop.
@task_item n=7 part=needs
The event line kinds the wait feed reads: the turn-ended line with background_tasks, the question, and the log-only notification kinds, as the event catalog names them.
@task_item n=8 part=needs
--json output of `wait` stays one clean document with no hint line, on a wake as on a timeout.
@task_item n=9 part=needs
Fixed CLI strings of `wait` stay pinned by their literals in the existing tests when the wake tests are added.
@task_item n=10 part=needs
The `viola verify` step lines the stamped-home fixture produces: taking its run out of the measured time or bounding it changes no step line wording.
@task_item n=11 part=needs
The CI test gate allows no skips and no retries: the run-start test's TIMEOUT is fixed by a bound shown to hold at load, never by skipping or retrying the test.
@task_item n=12 part=needs
Live readings never gate: the wake condition is proved over the fake agent; any live claude reading is asked before and does not become a gating test.
@task_item n=13 part=needs
Gate breach fails the build: the sized ci kill or the shared stamped home must keep the coverage suite's gate a hard fail.
@task_standing n=1
convention: each new wait test is an assertion inside the existing suites, not a separate a11y driver or suite
home: 1-a11y-scope-summary
@task_standing n=2
convention: each new test's failure is a nextest failure, run in the existing test suites
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=3
convention: each new test is free of flakiness and passes with no retries
home: 10-slo-invariants-a11y-budgets
@task_standing n=4
convention: each new test runs inside the coverage suite in the GitHub Actions push/PR workflow
home: 9-ci-integration
@task_item n=1 part=not_needed
The TUI passthrough contracts and the boundary-driver rows (vendor content, viola terminal bytes, focus, mouse and resize sequences, human keystrokes, vt100 feed panic): the chunk changes no terminal passthrough.
@task_item n=2 part=not_needed
The `viola list` board rows (caption order, colour, stale-row dim, NAME truncation, the unwrapped separator): the chunk prints no board.
@task_item n=3 part=not_needed
The revive refusal and revive --list contracts: the chunk changes no revive path.
@task_item n=4 part=not_needed
The link and unlink, plugin install and --help contracts: none of those verbs is touched.
@task_item n=5 part=not_needed
The send refusals (input-not-ready, no-prompt-submitted) and send-refused-while-typing: send confirmation and the readiness-gate race are not this chunk's.
@task_item n=6 part=not_needed
The frame-verb, hook-call and MCP frame-tool refusals for server-verify, socket directory and strict modes: the chunk adds no frame or hook refusal.
@task_item n=7 part=not_needed
The manual Orca and keyboard passes and the screen-reader targets: supplemental, gate nothing, and the chunk adds no manual pass.
@task_item n=8 part=not_needed
The absences (web or graphical frontend, touch surface, internationalization) and the WCAG mapping and contrast harness areas: nothing in them applies to a wait wake condition.
