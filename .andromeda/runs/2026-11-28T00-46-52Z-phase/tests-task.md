@task_prose part=chunk
Builds the fleet console's scaffold: its surface section in the masters (filling the 0.1.0 stub deferred to 0.2.0) and the minimal console code behind it, a frame that stays whole at a quarter of the screen and degrades by a named rule below its smallest size.
Reconciles the redraw-ban rows with an interactive console: the conservative arm is an exception named for the console's own surface only, the rule for CLI surfaces never generalised.
Adds a test-only keystroke driver feeding the console keys (quit and what the frame needs) through its own pseudo-terminal under a test-owned home, asserting on the console's exposed state or log lines, never on screen bytes, over the fake agent and fixtures.
Gives the console process its own diagnostics role beside run, hook, mcp and ui, reachable through the logs command, with obs event catalog rows for it.
Folds two carries: re-reads advisory RUSTSEC-2026-0543 (chrono, unmaintained) at the epoch boundary; and audit items M1 (one step function for the per-paste probes of framing_turns, a look at the guarded section of send_through) and M3 (relied-row count from one source, the stamped-set table held once).
@task_file n=1 path=deny.toml mark=edits
what: the cargo-deny supply-chain gate's configuration and its advisory ignore list — edits: the RUSTSEC-2026-0543 ignore entry and its reason line saying it is re-read at each boundary
@task_item n=1 part=needs
The console's own diagnostics role: structured JSON process logs written under a new role beside run, hook, mcp and ui, reachable through the logs command; the obs event catalog rows for its events; the roles' list and its count wherever a row or test pins it.
@task_item n=2 part=needs
The console's process log lines follow the process log line fields (timestamp, level, process, event, corr, target, message, null encoding), codes-only role logs with detail apart, and the diagnostics schema conformance check over them.
@task_item n=3 part=needs
The console's diagnostics files are private: owner-only modes, umask-independent, and they carry no secrets.
@task_item n=4 part=needs
The keystroke test driver asserts on the console's state (a structured read exposed for tests, or its log lines) and never on rendered screen bytes: the no-screen-parsing and E2E-asserts-on-events rules, and test waits keyed on logged events rather than sleeps or retries.
@task_item n=5 part=needs
The driver feeds keys through a real pseudo-terminal of its own (real openpty in integration tests, outer PTY reader drained on its own thread, child exit read on the process handle, never on EOF).
@task_item n=6 part=needs
Every console test runs under a test-owned home and its own pseudo-terminal, reads nothing of the host's claude sessions or viola home, drops inherited session environment, and is bounded under the 20 s per-test kill line.
@task_item n=7 part=needs
The console scaffold is agent-runnable headless over the fake agent and its recorded fixtures as the contract; no real claude CLI, no live window, compositor or real terminal in any test.
@task_item n=8 part=needs
The test-only state read the console exposes for the driver stays test-only: it must not reach the release build, which carries viola only.
@task_item n=9 part=needs
Where the keystroke driver and any test-only crate code live: the single test-only workspace member and tokio test code confined to the E2E crate; no new crate without a real consumer and a test.
@task_item n=10 part=needs
Redraw-ban reconciliation: the rows banning spinners, in-place rewrites and cursor-moving status and the no-SGR / no-escape-without-a-terminal rules for CLI human output get an exception named for the console's surface alone; the CLI verbs keep them unchanged.
@task_item n=11 part=needs
Text the console's frame draws escapes control characters, as human-mode output does.
@task_item n=12 part=needs
The quarter-screen layout: the frame stays whole at its smallest fixed size and degrades by a named rule below it, proved through the driver's state read after a resize (a key after a resize waits for the new size).
@task_item n=13 part=needs
The console's start verb or flag is a new CLI surface: tested as a subprocess, listed in the grouped verb table of viola --help, its human output pinned by literals.
@task_item n=14 part=needs
Any new dependency that draws or reads a terminal passes the cargo-deny gate: tokio and C-crate bans, tokio-free sync crates that compile alone, one version source per tool, MSRV floor.
@task_item n=15 part=needs
Carry cry-h6vzhlce: RUSTSEC-2026-0543 re-read against the no-advisory-ignored rule and the weekly advisory scan; a safe upgrade lifts the ignore, none keeps it one more epoch with its reason line.
@task_item n=16 part=needs
Carry cry-boujwuyr M1, a restructure with no change of behaviour: the per-paste probes of framing_turns brought under one step function and the guarded section of send_through looked at; the send's typed framing, confirmation and refusals stay as they are, held by the existing send checks.
@task_item n=17 part=needs
Carry cry-boujwuyr M3: the relied-row count taken from one source instead of hand-moved literals, and the stamped-set table held once, with the version gate still needing every relied row and the identity floor tested against its literal copy.
@task_standing n=1
convention: every new test creates its own fresh private home under the shared test-home root and seeds its own data
home: test-data-bootstrap
@task_standing n=2
convention: every new test waits on logged events, never sleeps, never retries, never shares mutable state and runs in parallel
home: 11-test-anti-patterns-never-do-these
@task_standing n=3
convention: every new test oracle is a literal and asserts on the contract, never on Debug strings or fixture internals
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: every new test function follows the test naming rule, and a new root integration binary is prefixed by its surface and selected by its layer filterset
home: 2-test-strategy
@task_standing n=5
convention: every new OS-gated test is registered with its reason in the binary's root file
home: test-strategy
@task_standing n=6
convention: every new test input is synthetic, carries the canary and passes the fixture hygiene walk
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=7
convention: every new log line carries the process log line fields and its event's corr
home: log-format
@task_standing n=8
convention: new code stays within the comprehensive-tier coverage floors, child viola processes counted
home: 10-quality-gates-coverage-targets · quality-gates-coverage-targets
@task_item n=1 part=not_needed
Hook dialogs, dialog decision bodies and their insta snapshots: the scaffold raises and answers no dialog.
@task_item n=2 part=not_needed
The MCP server, MCP tools and their reply screens and invalid-params fields: the console adds no tool.
@task_item n=3 part=not_needed
The wrapper channel's link and unlink methods, JSON-RPC error bodies and their log lines: the scaffold calls no frame verb.
@task_item n=4 part=not_needed
viola answer, release, wait and last and their exit codes and documents: the console's verbs are «Console actions»', not this chunk's.
@task_item n=5 part=not_needed
viola list's human board columns, liveness and stale rows: the fleet rows inside the frame are «Fleet console view»'s.
@task_item n=6 part=not_needed
Revive refusal codes and run and revive start refusals: the scaffold changes no start of a wrapped agent.
@task_item n=7 part=not_needed
Live rig readings, KR arms and window placement, raise and re-open proven live: the phase makes no live arm and opens no window.
@task_item n=8 part=not_needed
Hook performance budgets, hyperfine invocation and the perf CI job: no hook path changes.
@task_item n=9 part=not_needed
Mutation run verdicts, mutants kill lines and host-excluded mutants: the phase makes no mutation run.
@task_item n=10 part=not_needed
Heartbeat staleness classification, snapshot replay and torn-append healing: the scaffold reads no instance state.
@task_item n=11 part=not_needed
Fuzz workspace, corpus replay and nightly fuzzing: the chunk adds no parser of external input.
