@task_prose part=chunk
Fixes the pre-send gate of viola send, the screen check viola run makes before it types a driver's text into the session.
A modal literal quoted inside the session's own transcript, such as a chat message that quotes it, no longer counts as a standing modal: a send into an IDLE session is delivered, not refused not-delivered input-not-ready (exit 13).
A real standing modal (every compiled modal literal on its recorded screens) still refuses the send exactly as today, and the tests prove both directions, including a remove-the-guard control run on the refusal side.
The discriminator is picked from the screen model and the recorded screens with no live start, and P3 decides whether viola list, status and the dialog relay move together with the gate.
@task_item n=1 part=needs
The send outcome that changes: a driver's viola send into an idle session whose screen holds a row quoting a modal literal is now delivered and reports the normal delivered send output (issue line, read back, --json ok document), not the input-not-ready refusal.
@task_item n=2 part=needs
The refusal that stays: with a real modal standing, viola send still refuses not-delivered input-not-ready with exit code 13, the same refusal line, the same input-not-ready hint and the same --json refusal document, all unchanged.
@task_item n=3 part=needs
The conservative side: when the gate cannot tell a quoted literal from a standing modal it must refuse, because typing a driver's text into a dialog is the dangerous failure. A send must never be delivered into a real dialog.
@task_item n=4 part=needs
The pre-send readiness predicate is the one predicate that also decides viola run's pre-send gate, and verify's Run B framing pastes use it too, so the change must leave verify's reading of the input-box settle correct and its output unchanged.
@task_item n=5 part=needs
The dialog state other verbs show: if viola list (its DIALOG column and STATUS), the status reading or the dialog relay (viola wait's dialog outcome, viola answer) read the same predicate, the chunk either moves them with the gate or deliberately leaves them apart. Either way the DIALOG word on the board stays in step with what the gate decides.
@task_item n=6 part=needs
The new tests carry both directions: a quoted-literal row on an idle screen is delivered, and a real modal on each recorded screen is refused. Each has a remove-the-guard control showing the test goes red without the fix, including on the refusal side.
@task_item n=7 part=needs
Test inputs are the fake agent's screens and the recorded fixture sets per CLI version. A screen no recorded set holds is added as a synthetic screen beside the recorded sets, named as synthetic and obtained without a live start.
@task_item n=8 part=needs
No live start in this phase, nothing read from the operator's real Claude config, and no scheduled-wake or timer tool. Every check is proved on recorded or synthetic screens.
@task_item n=9 part=needs
No relied ledger row or verify probe is meant to change: the fix is viola's own reading of its screen model, and P3 checks this against the ledger code.
@task_item n=10 part=needs
No documentation leaf takes a figure the code states.
@task_item n=11 part=needs
The intermittent rig rehearsal red (the box read empty after x and del) is disposed of by name in the plan. It is acted on only at its next red: read the printed rig summary and the kept after-del frame and name the cause before mending the wait, with no retry rule and no loosened entry.
@task_item n=12 part=needs
The watched supervise test, where Ctrl-C is pressed again when the first is lost, is read at every run. If it fails again, its log (the fake agent's exit code and what the supervisor sent it) is read before any mend, and no bound or assertion moves to get past it.
@task_standing n=1
convention: human output is a contract: any new test of a send outcome asserts the exact human lines, the exit code and the --json document, with the refusal word column and the last-line hint on stderr as they stand
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page, and this chunk adds none.
@task_item n=2 part=not_needed
viola revive: its start, --list, refusals, --json error codes and preflight order. The chunk does not touch reviving a session.
@task_item n=3 part=not_needed
viola plugin install and viola link / unlink output. Neither verb reads the screen.
@task_item n=4 part=not_needed
The CLI token rendering rows (colour, spacing, radius, motion, roles) and the styling decisions (SGR module, banner, spinner, footer, progress glyphs). No styling changes.
@task_item n=5 part=not_needed
The exit-21 instance-unreachable family: frame verb hints, the --json detail codes and socket or strict-modes refusals. These concern reaching the wrapper, not what its screen holds.
@task_item n=6 part=not_needed
The send refusals made before any request (empty text, control character). They are decided on the client before the gate is consulted.
@task_item n=7 part=not_needed
viola --help and per-verb usage output, and the MCP verb hiding and tool listing. No verb, flag or tool is added or renamed.
@task_item n=8 part=not_needed
The board layout rows of viola list (ordering by name, column widths, NAME truncation, the rack separator, the empty board, unknown width). Only the dialog state the board reads might move.
@task_item n=9 part=not_needed
Deep links or URL coupling in the CLI, and the link column the board does not have.
