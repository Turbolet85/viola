@task_prose part=session
The session changed the input-box readiness predicate: a modal literal on the screen now refuses a send only when it stands as a dialog, not whenever any row holds it.
A modal stands when its row is option-shaped (any indent, an optional selector glyph, optional numbering, the literal, then only spaces) or when no input-box row stands below it; a footer literal never counts as box below.
A quoted modal label in a transcript row above an idle input box no longer refuses a send with input-not-ready; a standing modal still refuses at once, and the quiet period, GATE_MAX_WAIT, the poisoned and unverified arms are unchanged.
The one predicate still serves the run pre-send gate, verify's paste guard, the ledger box post-condition and the rig's box read; the start-screen read and verify's settle end keep the raw any-row presence by design.
Added unit cases for each option-shape clause and the box-below rule, two gate-tier cases, a synthetic quoting screen helper and one integration send case past a quoted label; no fixture, schema, stamp, ledger row, refusal word, hint or count moved.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the screen signatures component, the compiled-literal reading of the child's screen that decides input-box readiness and modal presence, with its inline unit cases
@task_file n=2 path=src/run/gate.rs
what: the viola run pre-send readiness gate, its verdict wait over the screen and its gate-tier unit cases
@task_file n=3 path=tests/support/fake.rs
what: the shared integration test helpers for the fake agent, including the synthetic start screens a test hands it
@task_file n=4 path=tests/cli_send.rs
what: the viola send CLI integration suite, run as a subprocess over the fake agent on a stamped home
@task_item n=1 part=touches
The decision that the screen is read only for compiled signatures: still true, but the modal refusal now reads a literal's row shape and whether box rows stand below it, not mere presence anywhere.
@task_item n=2 part=touches
The decisions that the rendered child screen is never parsed and that tests make no assertions on the child's screen, in both twins of the test scope summary and the anti-patterns list: the rig's box read moved with the predicate.
@task_item n=3 part=touches
The decision that discipline is held at every layer, where it names the screen read over compiled literals only.
@task_item n=4 part=touches
The decision that the readiness gate is tested by its refusals: the gate now also carries a delivered case, a send that goes past a quoted modal label to exit 0.
@task_item n=5 part=touches
The viola send input-not-ready contract item: it now fires on a standing modal (option-shaped, or no box below), not on any row holding a modal literal.
@task_item n=6 part=touches
The readiness gate GATE_MAX_WAIT contract item and the quiet period: unchanged, but they rule over the gate whose at-once modal refusal moved.
@task_item n=7 part=touches
The decision that passthrough verdicts never parse the rendered screen, and that no assertion goes beyond compiled literals: rules over the signatures component that now reads row shape.
@task_item n=8 part=touches
The decision that the vt100 screen feed is panic-safe and its chaos feed-panic check: rules over the screen component changed; re-run green.
@task_item n=9 part=touches
The decision on no blanket dialog approval: a standing dialog still refuses the send, every option-shaped form of each modal literal refuses.
@task_item n=10 part=touches
The decision that synthetic test inputs carry a canary and fixture text is synthetic: the new quoting screen is synthetic and the delivered send asserts the canary.
@task_item n=11 part=touches
The decision that box rows in screen fixtures are recorded as their literals alone: the synthetic quoting screen places the box rule, prompt and first input-box literal as rows below the quote.
@task_item n=12 part=touches
The decision that a fake-agent run is not proof of real-CLI behaviour: the mend is proved over the fake agent, recorded trust screens and synthetic screens; the imports dialog's real screen and the trust selector row stay unmeasured.
@task_item n=13 part=touches
The decisions on table-driven rstest unit cases, inline unit tests and test function naming: the new option-shape and box-below cases are rstest case tables inline in the component.
@task_item n=14 part=touches
The viola send confirmed-send contract items: a send past a quoted label emits send-issued, prompt-submitted with origin driver and send-confirmed, no send-refused.
@task_item n=15 part=touches
The decision on local commands confirmed by post-condition only and the ledger probe contract suite: the ledger box post-condition reads the moved predicate; every stamped set stayed green.
@task_fact n=1 slug=unit-test-strategy
fact: A modal literal refuses a send only when it stands as a dialog: its row is option-shaped (indent, optional selector glyph, optional numbering, the literal, then only spaces) or no input-box row stands below it; a literal quoted mid-row above the box reads ready.
@task_fact n=2 slug=1-test-scope-summary
fact: The readiness gate is held by its refusals and by one delivered case, a send on a verified CLI past a quoted modal label in a transcript row above an idle box.
@task_fact n=3 slug=unit-test-strategy
fact: The start-screen read and verify's settle end keep the raw any-row modal presence; only input-box readiness and the gate's at-once refusal read the standing-modal rule.
@task_item n=1 part=untouched
CI jobs, supply-chain gates, Actions pinning, toolchain and tool version pins: no workflow, dependency or tool moved.
@task_item n=2 part=untouched
Harness contract, its exit codes, session record, status shape, cleanup, boot readiness and pre-push detail: no harness surface changed.
@task_item n=3 part=untouched
Coverage floors, mutation thresholds, perf budgets and hook deadlines: no threshold, floor or bound moved.
@task_item n=4 part=untouched
MCP tools, wrapper channel JSON-RPC errors, link and unlink methods and their log lines: no IPC method, error or event kind added.
@task_item n=5 part=untouched
Hook exec entry points, dialog decision bodies and their insta snapshots: no hook or dialog surface changed.
@task_item n=6 part=untouched
Recorded fixtures, stamps, the ledger rows and their verify probes, fixture hygiene and drift: no fixture, stamp or ledger row moved.
@task_item n=7 part=untouched
Log format, process log line fields and log event corr: no log line, span field or refusal word added.
@task_item n=8 part=untouched
revive, list, wait, last, release verbs and the heartbeat, snapshot and strict-modes rules: not reached.
