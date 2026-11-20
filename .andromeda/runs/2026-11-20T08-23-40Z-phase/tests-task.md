@task_prose part=chunk
Fixes the pre-send readiness gate of viola send: a compiled modal literal that only stands inside the session's own transcript (a chat row quoting it) no longer reads as a standing modal.
A driver's send into an IDLE session whose screen holds such a quoting row is delivered, where today it is refused not-delivered input-not-ready (exit 13).
A real standing modal (every compiled modal literal on its recorded screens) still refuses the send exactly as today; tests prove both directions, with the remove-the-guard control run on each side.
The discriminator is chosen from the screen model and the recorded screens with no live start; status, list and the dialog relay move with the predicate or stay deliberately apart, said at P3.
Carries the intermittent rig rehearsal red (acted on only at its next red) and the watched supervise test red (read from its log before any mend).
@task_item n=1 part=needs
The pre-send readiness predicate in the Claude agent adapter (the input-box settle reading viola run reads before it types a driver's text): a modal literal quoted inside a transcript row of an idle session no longer refuses the send; the send is delivered and confirmed as any other driver send.
@task_item n=2 part=needs
The refusal side kept: a real modal standing on screen, for every compiled modal literal the product knows on its recorded screens, still refuses the driver's send not-delivered input-not-ready with exit 13 and the same refusal word, detail and send-refused record as today; typing into a dialog is the dangerous failure.
@task_item n=3 part=needs
Both directions carried by tests, each with a bounded remove-the-guard witness: removing the new discrimination makes the quoted-row delivery test fail, and removing the modal check makes the standing-modal refusal test fail.
@task_item n=4 part=needs
The readiness gate is tested by its refusals: the input-not-ready refusal stays inside the gate's bounded wait (GATE_MAX_WAIT) and keeps its place in the send refusal order against human-typing, manual-pause, turn-running and empty-text.
@task_item n=5 part=needs
When the gate cannot tell a quoting row from a standing modal it falls to the conservative side stated at research; the discriminator must not let a message's text defeat a real modal into a delivered send.
@task_item n=6 part=needs
The screen is read only for compiled signatures and the screen model: the discriminator may not parse the rendered child screen beyond them, and no assertion goes beyond compiled literals.
@task_item n=7 part=needs
The same settle reading serves verify's Run B framing pastes; the change must keep verify's framing paste behaviour and its stamped rows as they are, and no relied ledger row or its verify probe is meant to change (closed at P3 against the ledger code).
@task_item n=8 part=needs
Whether status, list and the dialog relay read the same predicate: the change moves them together or leaves them deliberately apart; list keeps reading the session live and idle with no dialog.
@task_item n=9 part=needs
Test inputs are the fake agent's screens and the recorded fixture sets per CLI version, replayed byte for byte; a screen no recorded set holds is a synthetic screen beside the recorded sets, named as synthetic, got without a live start, with synthetic text only.
@task_item n=10 part=needs
No live start of the real claude CLI in this phase or in any test; nothing read of the operator's real Claude config or the host's claude sessions; no scheduled-wake or timer tool.
@task_item n=11 part=needs
End-to-end proof runs over the fake agent and asserts on logged events and the send document (send-issued, prompt-submitted with origin driver, send-refused), never on the child's screen content.
@task_item n=12 part=needs
Carry: the intermittent rig rehearsal test that reads the box empty after x and del stays open with its cause unread; at its next red read the printed rig summary and the kept after-del frame, name the cause, then mend the wait — no retry rule, no loosened entry.
@task_item n=13 part=needs
Watch: the supervise test that presses Ctrl-C again when the first is lost (fake agent exit code read 1 where 0 is asserted); a recurrence is read from its log (the fake agent's exit code and what the supervisor sent it) before any mend, and no bound or assertion moves to get past it.
@task_item n=14 part=needs
Zero flakiness: the new tests use no retries and no sleep synchronisation; their waits key on logged events.
@task_item n=15 part=needs
No documentation leaf takes a figure the code states.
@task_standing n=1
convention: Every new test function follows the test function naming rule.
home: 2-test-strategy
@task_standing n=2
convention: New unit tests stay inline beside the code they test, with table-driven cases in rstest.
home: 2-test-strategy · 4-unit-test-strategy
@task_standing n=3
convention: Every test creates its own private home; nothing shared or mutable across parallel tests.
home: 2-test-strategy · 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: Test oracles are written as literals; no comparison by Debug string.
home: 11-test-anti-patterns-never-do-these
@task_standing n=5
convention: Every new fixture or synthetic screen carries synthetic text only, with a canary, and passes the fixture hygiene walk.
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=6
convention: Root integration and E2E binaries are named with their surface prefix so the layer filtersets select them.
home: 2-test-strategy · test-strategy
@task_item n=1 part=not_needed
MCP tool surface shapes: tools/list, invalid-params fields, schema-violation handling and the MCP fixture scrub refusal codes; the chunk changes no MCP argument or result shape.
@task_item n=2 part=not_needed
Hook decision bodies, dialog answer mapping, dialog hold expiry and hook deadlines and performance budgets; the chunk changes no hook path and no answer.
@task_item n=3 part=not_needed
Start, run and revive refusals and their exit 1 causes, revive preflight order and the child environment strip; the chunk changes nothing at start.
@task_item n=4 part=not_needed
The wrapper channel link and unlink methods, link records and implicit link on send; the chunk changes no link behaviour.
@task_item n=5 part=not_needed
wait and last: wake kinds, timeouts, scheduled wakes and turn-ended data; the chunk changes no wake.
@task_item n=6 part=not_needed
Heartbeat staleness, snapshot replay, torn append healing and the instance snapshot fields; no on-disk state changes.
@task_item n=7 part=not_needed
Socket directory checks, strict modes, server identity and peer euid admission, and their exit 21 details; no security check of the channel moves.
@task_item n=8 part=not_needed
Local command confirmation for /clear and /compact and the ledger-listed local command rows; the chunk does not reach slash-command sends.
@task_item n=9 part=not_needed
Harness command document shapes for boot, cleanup, status, the gate, pre-push and the secret scan; the chunk adds no harness command or field.
@task_item n=10 part=not_needed
Plugin install and marketplace, the generated plugin hook commands and pinned executable hash checks.
@task_item n=11 part=not_needed
CI workflow permissions, Actions SHA pinning, artifact retention and the weekly advisory and nightly fuzz runs; no workflow changes.
@task_item n=12 part=not_needed
Absences: a browser suite, Windows and macOS support, a database and outbound HTTP.
