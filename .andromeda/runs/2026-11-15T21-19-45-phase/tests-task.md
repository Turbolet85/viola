@task_prose part=chunk
Makes a claude turn that ends on an API error (the CLI's StopFailure-class hook signal) end the turn in viola: a turn-ended event carrying a cause code from a closed viola set, never the error text.
After that turn end the instance reads idle on the board and in viola list, the wheel's turn-running state is cleared exactly as a Stop clears it, a wait on the turn end returns, and the driver's next send is accepted with no human pause and release.
Adds the hook registration, the payload read in the Claude agent crate, the cause set's neutral home in the core crate, the fake agent's ability to emit the signal, and a cause key in the turn-ended event data contract.
Adds a test over the fake agent and synthetic payloads that fails on the tree before the fix; P3 re-reads the measured defect (list idle while send is refused turn-running) from the code and re-measures the mechanism.
@task_file n=1 path=plugin/hooks/hooks.json mark=edits
what: the generated plugin's hook commands, the hooks viola registers with claude — edits: the hook registration for the CLI signal that ends a turn on an API error
@task_file n=2 path=src/run/send/slot.rs mark=edits
what: the run pump's send slot and the wheel's turn-running state that refuses a send with turn-running — edits: the turn-end path that clears turn-running
@task_file n=3 path=src/run/wait.rs mark=edits
what: the wait wake on a turn-ended event in the run pump — edits: the turn-end path a waiter wakes on
@task_item n=1 part=needs
The turn-ended event written to the instance event log on an API-error turn end: its event line format and keys, and a cause value from a closed set in its data.
@task_item n=2 part=needs
The turn-ended event data keys: today exactly last_assistant_message and background_tasks; adding a cause key changes the event data contract, and readers that meet an unknown key skip and count it, so mixed binary versions stay readable.
@task_item n=3 part=needs
The closed cause set: every member named, a CLI error string outside it mapped to one fixed fallback member, never passed through; a closed set changes only by a recorded decision.
@task_item n=4 part=needs
Where the cause set lives: a neutral home in the core crate with no Claude-specific name, per the event-kind rule; the mapping from the CLI's error string lives only in the Claude agent crate, the one crate that knows Claude.
@task_item n=5 part=needs
The error text (last_assistant_message and error_details on this path) reaches no event line, no snapshot and no process log line: logs carry no secrets, errors are sanitized at every surface, role logs carry codes only, and the absence is proven by a probe against the plan's test names and fixture keys; whether either reaches the content-bearing detail tier is left to P3/P4 under the never-log floor.
@task_item n=6 part=needs
The instance state after the API-error end: the board and viola list --json read idle, the same state send reads, never idle on the board while send still holds turn-running.
@task_item n=7 part=needs
viola send after the API-error end is accepted and confirmed, not refused not-delivered turn-running with exit 13; send refusal order and the turn-running refusal stay as they are for a turn still running.
@task_item n=8 part=needs
viola wait and the channel wait method wake on the API-error turn-ended record, including when it is already past the cursor; a waiter on the turn end returns.
@task_item n=9 part=needs
The wheel after the API-error end: turn-running cleared exactly as a Stop clears it, no human pause and release needed, and the wheel's driver kept.
@task_item n=10 part=needs
The new hook entry point of viola hook: exec-form entry, exit code, empty stdout, stderr and deadline as every hook verb; hooks fail open; unknown payload fields tolerated; the instance check comes first; a hook outside a wrapped session is a silent no-op; the hook never writes a frame to an impostor server.
@task_item n=11 part=needs
The hook payload reader for the StopFailure-class payload (error, optional error_details, optional last_assistant_message): a bounded reader with no unbounded JSON depth, a parser surface under property tests.
@task_item n=12 part=needs
The fake agent firing the API-error hook on a step: its fired events, hook execution with its stdin payload, script format and control-file gating; the fake agent stands in for claude at that boundary.
@task_item n=13 part=needs
No recorded fixture holds a StopFailure payload: the test uses synthetic payloads carrying a canary, named by hook event and checked against the tolerant fixture schema; fixtures stay free of real data and none is hand-written as product state.
@task_item n=14 part=needs
A dependence on undocumented claude behaviour (whether StopFailure fires for every API-error end) is a capability-ledger row with a viola verify probe; a fake-agent run is no proof of real-CLI behaviour.
@task_item n=15 part=needs
No live claude start without the cap asked first: tests run on the fake agent only, real claude never in CI, live readings are not a test layer, and a needed live reading is a counted request through the overseer.
@task_item n=16 part=needs
The new test must fail on the tree before the fix, and each new guard test carries a bounded remove-the-guard witness run through the gate control under a kill deadline, its case the test's name alone.
@task_item n=17 part=needs
No mutation run, local or dispatched, for this chunk; mutation testing stays off CI and outside this chunk's proof.
@task_item n=18 part=needs
The end-to-end proof over the real viola hook and fake agent asserts on logged events and the list document, never on the child's screen; waits key on logged events, never on sleeps.
@task_item n=19 part=needs
Test sockets go under the short /tmp root and any home given to verify --record goes through record_home(); every test makes its own private home.
@task_item n=20 part=needs
The hook performance budget for spine hooks and the hook deadline bind the new hook verb if it joins the spine; a fire-and-forget hook still answers within the hook deadline.
@task_standing n=1
convention: Each new test function is named by the test-naming rule and lives in the layer its binary prefix selects
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: Each new test creates its own fresh private home, seeds its own data, and drops inherited session environment
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=3
convention: Each new test uses literal oracles, no sleeps, no retries, no Debug-string comparison and no process-wide env mutation
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: Each new log line is one structured JSON line with its fixed fields and codes only, its content apart in the detail tier
home: log-format
@task_standing n=5
convention: Each long-lived wrapper a test spawns is held in a drop guard and cleaned up
home: integration-test-strategy · 5-integration-test-strategy
@task_standing n=6
convention: Each new table of unit cases is an rstest case table inline beside its unit
home: 2-test-strategy · 4-unit-test-strategy
@task_item n=1 part=not_needed
viola revive: its preflight order, refusal codes, exit codes and documents; the chunk does not start or revive an instance.
@task_item n=2 part=not_needed
Dialog decision bodies (permission, question, plan) and dialog hold expiry; the API-error hook is fire-and-forget and returns no decision body.
@task_item n=3 part=not_needed
Send confirmation by exact normalised match, the confirmation window, whitespace normalisation and the readiness-gate race; those belong to another entry.
@task_item n=4 part=not_needed
The wait wake condition for background-task questions; another entry owns it.
@task_item n=5 part=not_needed
The wrapper channel link and unlink methods, link records and their name validation; nothing here links instances.
@task_item n=6 part=not_needed
Start-time checks: strict modes, socket directory refusal, server identity, pinned executable hash, squatted endpoint, stale heartbeat and the claude child environment strip; the chunk changes no start path.
@task_item n=7 part=not_needed
The harness's own commands and documents (boot, cleanup, status, supervise, pre-push, secret-scan classes, run reasons) and the CI artifact names; the chunk adds no harness surface.
@task_item n=8 part=not_needed
MCP tool input validation tables, invalid-params fields and the tools/list set; the chunk adds no tool and no argument.
@task_item n=9 part=not_needed
Human-mode passthrough, outer-PTY rendering, focus, mouse and resize handling and vt100 panic containment; the chunk changes no terminal path.
@task_item n=10 part=not_needed
Fuzz workspace, nightly fuzzing and corpus replay jobs; the chunk adds no fuzz target.
