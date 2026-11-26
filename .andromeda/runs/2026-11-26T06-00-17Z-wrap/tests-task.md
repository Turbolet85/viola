@task_prose part=session
The session is the gated run of the chunk placing without moving the view: it changed no source, test, script or master, and no code file was reached.
It fired the gated live reading R7 once on the operator's host, with the operator's word, a stand-in claude and no real claude CLI; the reading was green, placement placed and the active workspace unchanged after a placing open.
The gated entry read that reading and went green, so the capability for a placing open that keeps the view was moved from planned to implemented, with its clearance.
Still unproven live: where keyboard focus lands after the workspace restore, and what a failed restore looks like (R7 read only a successful restore).
Operator entries already green in the take-up pass were carried over, not fired again; CI on the pushed records-only commit read green 7 of 7.
@task_item n=1 part=touches
Real window placement proven only live: the placing open's view-keeping (the window step putting back the active workspace its move took) now has its gated live reading, R7, read green with the active workspace unchanged.
@task_item n=2 part=touches
The sibling rules that each name one gated live reading as the only proof (window raise by R3, fleet re-open by R4, adopt by R5) get a fourth of the same kind: the kept view on a placing open, proven only by R7.
@task_item n=3 part=touches
Live reading recorded as a reading, never as verification: R7 is a single operator-fired reading of a successful restore only; keyboard focus after the restore and a failed restore stay unproven, and nothing here proves them.
@task_item n=4 part=touches
No manual smoke step and no human-in-loop verification: R7 is an operator-attended gated live reading at the real compositor, fired once on the operator's word; it reaches these rules through the same exception the R3, R4 and R5 readings already hold.
@task_item n=5 part=touches
Real claude CLI only locally, never in CI: R7 ran at the operator's host with a stand-in for claude, not the real CLI, and outside CI.
@task_item n=6 part=touches
Fake-agent run is not proof of real-CLI behaviour: R7's reading is a statement about the compositor and the window step, not about the real claude CLI, since a stand-in served the child.
@task_fact n=1 slug=e2e-test-strategy
fact: The kept view on a placing open (the window step putting back the active workspace its move took) is proven only by the gated live reading R7 at the real compositor, read green with the active workspace unchanged; where keyboard focus lands after the restore and how a failed restore looks live are not proven.
@task_item n=1 part=untouched
Contract items of the CLI, the MCP tools, the wrapper channel, hooks, send, wait, list, revive and the event log: no output, exit code, refusal word or record changed.
@task_item n=2 part=untouched
The viola verify refusal for an untrusted folder and the verify output and exit-detail rows: this session changed none of them; they belong to the earlier take-up of the same chunk.
@task_item n=3 part=untouched
CI integration, quality gates, coverage floors, mutation thresholds, performance budgets and the supply-chain gates: no threshold, floor, bound or gate moved, and no test was added.
@task_item n=4 part=untouched
The test harness contract, its commands, exit codes, session record, pre-push gate and the test-data bootstrap: none of them changed.
@task_item n=5 part=untouched
Test data and fixtures, the fake agent's command line, receipts and screens, and the mocking discipline: no fixture, fake-agent mode or seam changed.
@task_item n=6 part=untouched
Log format and the process log line, the diagnostics schema, the secret scan and the security controls (strict modes, socket directory, peer checks, environment strip): untouched.
