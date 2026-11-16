@task_prose part=chunk
A key the human presses to answer or dismiss the claude CLI's session-feedback survey edits no prompt, so it leaves the wheel with its holder; every other key keeps taking the wheel.
The survey's grammar (which keys answer or dismiss it, and the screen state in which they do) is measured live on the installed CLI before it joins the closed non-editing list; a prompt shape not measured still takes the wheel.
The measured grammar becomes a capability-ledger row with a viola verify probe and a compiled screen signature; the fake agent learns to draw the survey screen for CI, and any recorded survey screen joins the per-version fixtures.
P3 re-measures the closed list, the wheel classification and what is known of the survey; live claude starts are named at P3's end and wait on the founder's word.
@task_file n=1 path=src/run/wheel.rs mark=edits
what: the wheel classifier — the component that decides whether a human stdin read holds an editing key and so takes the wheel, and the closed non-editing list it observes — edits: the Classifier / Observed path
@task_file n=2 path=crates/viola-agent-claude mark=edits
what: the claude capability ledger — its rows, their compiled screen signatures and the viola verify probe of each row — edits: a new ledger row for the survey grammar, its screen signature, its verify probe
@task_file n=3 path=fixtures/claude/<version>/ mark=edits
what: recorded Claude fixtures per CLI version, written by live verify and replayed by the fake agent — edits: a recorded survey screen
@task_item n=1 part=needs
Human editing takes the wheel: a key that answers or dismisses the session-feedback survey, in the survey's measured screen state, is non-editing and leaves the wheel with its holder; every other human key still takes the wheel and logs its wheel human-input record.
@task_item n=2 part=needs
The closed non-editing list grows by the survey shape only after a live measurement: no shape joins unmeasured, and a prompt shape not measured still takes the wheel; the existing members (focus reports, mouse reports, terminal replies, resizes) stay as they are.
@task_item n=3 part=needs
No environment variable, flag or config key widens the non-editing list; it is a closed list changed only by decision.
@task_item n=4 part=needs
The wheel classifier only observes: the human's survey key reaches the child unchanged, and viola never answers the survey nor types a survey key itself in the product.
@task_item n=5 part=needs
The survey's screen state reaches the wheel decision through the pump's vt100 screen model, read only by a compiled screen signature taken from a compiled ledger row — never by parsing the rendered child screen beyond that signature; a vt100 panic stays contained and keeps human passthrough.
@task_item n=6 part=needs
Every dependence on an undocumented claude behaviour is a capability-ledger row with a viola verify probe: the survey grammar row has its probe, joins the ledger probe contract suite, and counts among the rows the version gate and stamps rely on.
@task_item n=7 part=needs
Live readings of the real claude CLI are local only and are no test layer: CI never starts the real CLI; the live starts, their readings, how the survey is made to appear and any key a rig types are bounded by the founder's cap and named at P3's end.
@task_item n=8 part=needs
The fake agent (viola-fake-agent) can draw the survey screen for CI, held byte for byte to the recorded fixtures and gated by its control-file or script steps; fake-agent behaviour is no proof of real-CLI behaviour.
@task_item n=9 part=needs
A recorded survey screen is a per-CLI-version fixture: recorded only by live verify, scrubbed of real data and checked by the screen-fixture scrub-as-detector and hygiene walk, its box rows kept as literals alone, replayed by every fixture-set replay.
@task_item n=10 part=needs
The wheel records in the event log: a survey key logs no wheel human-input record and no wheel change; any other key logs the human-input wheel record as today; tests assert on these logged events, never on the child's screen.
@task_item n=11 part=needs
A survey key pressed while a driver send waits on the readiness gate does not count as human typing, so it neither refuses the send as human-typing nor moves the wheel; an editing key during the gate still does.
@task_item n=12 part=needs
Waits in the new tests key on logged events with bounded waits; no sleep synchronisation and no retries.
@task_item n=13 part=needs
Fake agent input of a keystroke is recorded in its receipt (key line), so a test proves the survey key reached the child unchanged through the receipt.
@task_standing n=1
convention: each new test follows the test function naming rule and table-driven unit cases in rstest, with unit tests inline in the module they test
home: 2-test-strategy
@task_standing n=2
convention: each new test creates its own fresh private home and never reads the host's claude sessions
home: test-strategy · test-data-fixtures
@task_standing n=3
convention: each new test's oracle is a literal, asserts on the contract and observable behaviour, never on a Debug string or fixture internals, and uses no sleep, retry or shared mutable state
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: each new fixture is synthetic, scrubbed, carries the canary and is produced by viola or recorded by verify, never hand-written product state
home: 7-test-data-fixtures
@task_standing n=5
convention: each new E2E test runs over the fake agent with a drop-guarded wrapper and asserts only cross-process behaviour through logged events
home: e2e-test-strategy · 6-e2e-test-strategy
@task_standing n=6
convention: each new integration test of the wheel runs on a real openpty with real disk and no mocks beyond product seams
home: integration-test-strategy · 5-integration-test-strategy
@task_item n=1 part=not_needed
MCP surfaces — tools/list, tool results, invalid-params fields, reply screens and MCP stdin caps: the chunk changes no MCP tool or payload.
@task_item n=2 part=not_needed
Dialog hooks and their decision bodies, hook exit and stderr, hook deadlines and fail-open: the survey is no hook dialog and the chunk touches no hook path.
@task_item n=3 part=not_needed
viola revive refusals, exit codes and documents, and run start refusal causes: the chunk changes no start or revive path.
@task_item n=4 part=not_needed
The wrapper channel JSON-RPC errors, link and unlink methods and their event records: the chunk adds no channel method or error.
@task_item n=5 part=not_needed
viola list board and list --json rows, liveness and heartbeat staleness: untouched by the wheel classification.
@task_item n=6 part=not_needed
Instance-unreachable causes, strict-modes refusals, socket-directory checks and server identity checks: the chunk opens no new endpoint or file.
@task_item n=7 part=not_needed
CI artifacts, supply-chain gates, workflow pinning and the agent-run status document and harness session record shapes: the chunk adds no CI job or harness surface.
@task_item n=8 part=not_needed
Performance budgets for hook deadlines and hyperfine invocation: no hook timing changes.
@task_item n=9 part=not_needed
/clear and /compact confirmation and local-command post-conditions, send confirmation window and trailing trim: the chunk changes no send confirmation.
@task_item n=10 part=not_needed
Cross-user peer admission and peer euid checks, and pinned-exe hash checks: no IPC or exe-pinning change.
