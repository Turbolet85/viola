@task_prose part=chunk
Documents and pins by tests the wake condition of `wait` (CLI `viola wait`, `viola wait --json`, MCP `wait`): which logged event kinds return a parked wait and which park it again, proved over the fake agent.
Adds the case where a turn ends while a background task still holds the session: the wait returns on the `turn-ended` line carrying `background_tasks` or on the `question` it raises, never parking past it.
Adds the case where a task-free turn end is followed by a log-only notification-class line: a fresh wait armed after that end does not wake on it.
Re-reads at P3 today's wake kinds versus log-only kinds and re-measures the driver-side instance (a turn end whose `background_tasks` reads 0 while the session still waits on a notification returns the wait and the driver must re-arm) before the plan states it.
Takes the carried CI red: `path1_run_start_one_home_in_order` timing out at the `ci` profile's 20 s kill because the stamped-home fixture's unbounded `viola verify` grows under load; P3 chooses by measurement between taking the stamping out of the measured time and sizing the class's `ci` kill.
@task_file n=1 path=src/run/wait.rs mark=edits
what: the wrapper's wait feed — the channel wait method that returns a parked wait on a wake kind and parks it on a log-only kind — edits: the wake-kind cases `wait_each_wake_kind_wakes` and `wait_log_only_kinds_never_wake`
@task_file n=2 path=src/cmd/wait.rs mark=edits
what: the `viola wait` CLI verb and its `--json` document
@task_file n=3 path=src/human/wait.rs mark=edits
what: the `viola wait` human-mode output lines
@task_file n=4 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP tools, here the MCP `wait` tool and its result — edits: the `wait` tool
@task_file n=5 path=crates/viola-core/src/lib.rs mark=edits
what: the event kinds catalogue and event data keys, including `turn-ended` with `background_tasks`, `question` and the log-only notification kinds — edits: the event kinds
@task_file n=6 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent (viola-fake-agent) that serves the claude boundary, its turn-script steps and the hook events it fires — edits: its turn scripts
@task_file n=7 path=fixtures/fake-scripts mark=edits
what: the fake agent turn scripts — a turn ending with a background task held and a task-free turn end followed by a notification line
@task_file n=8 path=tests/support/verify.rs mark=edits
what: the stamped-home test support — the `stamped_home` fixture that stamps a test home by `viola verify` against the fake agent — edits: the fixture's `viola verify` call (`within: None`)
@task_file n=9 path=.config/nextest.toml mark=edits
what: the nextest `ci` profile and its per-test kill lines — edits: the stamped-home test class's `ci` kill
@task_file n=10 path=crates/viola-agent-claude/src/hook.rs mark=re-reads
what: the hook path that reads the Stop payload into the `turn-ended` line and its `background_tasks`, re-measured at P3 from the code
@task_item n=1 part=needs
The wait contract's wake kinds: which logged kinds return a parked wait and which (activity, wheel and other log-only records) park it again, stated and pinned for the channel wait method, `viola wait` and the MCP `wait` tool.
@task_item n=2 part=needs
A turn ending while a background task still holds the session wakes the wait: the wait returns on the `turn-ended` line carrying `background_tasks`, or on the `question` the turn end raises, and never parks past it.
@task_item n=3 part=needs
No false wake: after a turn ended with no background task, a later notification-class line that is log-only does not return a fresh wait armed after that end; the wait instead runs to its timeout or the next real wake.
@task_item n=4 part=needs
The `turn-ended` event data and its `background_tasks` key (and the Stop event data's null details), as the wake decision reads them; whether a 0 count while the session still waits on a notification is documented behaviour, a defect, or the driver's to re-arm is settled at P3 from the code before the plan states it.
@task_item n=5 part=needs
The wait's cursor: a wait armed after a turn end does not return on a line already past its after cursor, and a line already logged past the cursor returns at once.
@task_item n=6 part=needs
The wait timeout and the MCP wait bound (`timeout_ms`): a non-waking case is observed as a `timed_out` result inside the bound, never by a sleep.
@task_item n=7 part=needs
CLI and MCP return identical payloads: the wake and the timed-out result read the same through `viola wait --json` and the MCP `wait` tool.
@task_item n=8 part=needs
Proof over the fake agent: new turn-script steps (a background task held at turn end, a notification line after a task-free end) follow the fake agent turn-script format and gated steps, configured by argv alone, and the fake agent stays held to the recorded fixtures.
@task_item n=9 part=needs
Test waits key on logged events: the wake tests synchronise on event lines and the event log, with no sleep and no retry; wait wake latency is not timed.
@task_item n=10 part=needs
The carried CI red is this chunk's item: `path1_run_start_one_home_in_order` timing out at the `ci` profile's 20 s kill (run ci#43773402286, sha 7fd86141, job `test (ubuntu-latest)`), the extra time sitting in the stamped-home fixture's unbounded `viola verify`; acceptance is a bound the plan can show holding at load.
@task_item n=11 part=needs
Option (a) for the CI red: take the stamping out of the stamped-home tests' measured time — a home stamped once and shared or copied — naming the per-test-home isolation it costs, with stamps still written only by `viola verify` against the fake agent and no product state hand-written.
@task_item n=12 part=needs
Option (b) for the CI red: size the stamped-home class's `ci` kill to the fixture's loaded time, staying coherent with the 20 s kill line for stamped-home and verify-driven tests, the mutants kill lines below cargo-mutants' timeout, and root waits bounded below the kills.
@task_item n=13 part=needs
The CI test profile keeps no retries and the zero-flakiness budget: the CI red is fixed by a bound, never by a retry or a skipped or ignored test.
@task_item n=14 part=needs
Each new guard test carries its one-off remove-the-guard control through `gate.py control`, the control's case being the test's name alone; no mutation run is made in this chunk.
@task_item n=15 part=needs
No live `claude` start without asking first: the plan runs on the fake agent; a fake-agent run is not proof of real-CLI behaviour, so a wake clause that needs a live reading on claude 2.1.325 is named with its start count and asked before the card, and any granted start runs under a private `CLAUDE_CONFIG_DIR`, never in CI.
@task_item n=16 part=needs
Unknown or forward-compatible event lines met by the wait feed are counted and skipped, never a wake and never a failure.
@task_item n=17 part=needs
The `viola wait` failure exits stay as they are around the change: endpoint-lost and an unreachable or killed wrapper still end a parked wait with their own cause.
@task_standing n=1
convention: Test functions are named by the test-function naming rule and table-driven unit cases are written as rstest case tables inline in the module
home: 2-test-strategy
@task_standing n=2
convention: Root integration and E2E binaries are prefixed by the surface they test and share helpers from the one shared copy, never copied
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: Test oracles are written as literals, asserting on the contract and on events, never on fixture internals, Debug strings or the child's screen
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: No sleep synchronisation, no retries, no shared mutable state, no process-wide env mutation; tests run in parallel
home: 11-test-anti-patterns-never-do-these
@task_standing n=5
convention: Fake-agent turn scripts and other new fixtures carry synthetic text only, free of real data, and pass the fixture hygiene walk
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=6
convention: Each test creates its own private home under the test-home root, held by a drop guard, with test-started viola dropping inherited session variables
home: 5-integration-test-strategy · integration-test-strategy
@task_item n=1 part=not_needed
Send confirmation, the readiness-gate turn race and local-command delivery: the driver-send, send-result and send-confirmation rows are another chunk's.
@task_item n=2 part=not_needed
The StopFailure live reading and its ledger exception, and the 2.1.325 recorded fixture set: no fixture is recorded and no ledger row changes here.
@task_item n=3 part=not_needed
Dialog hook decision bodies, `viola answer` and the dialog hold and expiry rules: the chunk raises no new dialog answer path.
@task_item n=4 part=not_needed
Revive and run-start refusal codes and exit-1 causes: no start or revive refusal changes.
@task_item n=5 part=not_needed
Link and unlink methods, link records and implicit link on send: untouched.
@task_item n=6 part=not_needed
The `viola list` board, list --json fields and liveness classification: untouched.
@task_item n=7 part=not_needed
Strict-modes, socket-directory and server-identity refusals and their causes: no channel security check changes.
@task_item n=8 part=not_needed
Hook performance budgets and the hyperfine perf job: no hook path is timed or changed for speed.
@task_item n=9 part=not_needed
The harness pre-push, secret-scan, schema-check, session record and status document contracts: the harness commands are not changed.
@task_item n=10 part=not_needed
Fuzz targets, corpora and nightly fuzzing: no parser surface is added.
@task_item n=11 part=not_needed
PTY passthrough, the wheel, human typing and terminal replies: the wake condition does not reach the human side.
@task_item n=12 part=not_needed
MCP fixture recording through the tap and its scrub refusal codes: nothing is recorded.
