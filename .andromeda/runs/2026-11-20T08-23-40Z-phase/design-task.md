@task_prose part=chunk
Changes the pre-send gate of the wrapper's send, the screen predicate viola run reads before it types a driver's text, so a compiled modal literal quoted inside the session's own transcript is no longer read as a standing modal.
A driver's send into an idle session whose screen holds such a quoting row is delivered, not refused not-delivered input-not-ready.
A real modal standing, every compiled modal literal on its recorded screens, still refuses the send exactly as today; the tests carry both directions, with the remove-the-guard control run on the refusal side too.
The discriminator is chosen from the screen model and the recorded screens with no live start; whether status, list and the dialog relay move with the predicate or stay apart is settled at P3.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: the agent's screen model, holding the pre-send readiness predicate (the input-box settle reading that decides viola run's pre-send gate and that verify's framing pastes also use) — edits: the pre-send readiness predicate, its reading of compiled modal literals
@task_item n=1 part=needs
The send refusal not-delivered input-not-ready: after the change it is given only when the input box is truly not ready, never because a transcript row quotes a modal literal on an idle session; its refusal line, hint and exit code stay as they are.
@task_item n=2 part=needs
The send refusal for a standing dialog (not-delivered unknown-dialog and the input-not-ready refusal while a real modal stands) keeps refusing exactly as today: the false-negative side, a driver typing into a dialog, is the dangerous one.
@task_item n=3 part=needs
Exit 13 for an undelivered send: a quoting row on an idle session no longer produces it, a standing modal still does.
@task_item n=4 part=needs
The send readback for a prompt send into an idle session with a quoting row: the send is delivered and read back as any prompt send; nothing announces success before confirmation.
@task_item n=5 part=needs
The send readback mirror's unable form: it is still what a refused send prints for a real standing modal, and is no longer printed for the quoted-literal case.
@task_item n=6 part=needs
The dialog-pending state: a real modal standing still reads as dialog-pending; a quoted literal in a chat row must not be read as dialog-pending where status, list or the dialog relay share the predicate, or they stay deliberately apart as P3 says.
@task_item n=7 part=needs
The board's STATUS column in viola list and its dialog-pending cocked strip: what it shows for an idle session whose transcript quotes a modal literal, kept consistent with the send gate or deliberately apart as P3 says.
@task_item n=8 part=needs
The conservative side when the gate cannot tell a quoted literal from a standing modal: refuse, never type into a possible dialog.
@task_item n=9 part=needs
Refused send folds into its send line: the quoted-literal case no longer adds a refusal; a real-modal refusal still folds into the send line as today.
@task_item n=10 part=needs
Hints never quote text: a refusal over a modal never echoes the transcript row or the literal it quoted.
@task_item n=11 part=needs
The chunk's new tests read recorded fixture screens and the fake agent's screens as inputs; a screen no recorded set holds is a synthetic screen beside the recorded sets, named as synthetic, got without a live start.
@task_item n=12 part=needs
No relied ledger row or its verify probe changes, and the CLI verification state a session carries is read as before: the change is viola's reading of its own screen model.
@task_item n=13 part=needs
Control characters in the transcript row a test screen carries stay escaped in any human output that shows it.
@task_item n=1 part=not_needed
Brand identity, colour palette, typography, spacing and iconography decisions about how strips, glyphs, colours and columns look: the chunk changes no rendering, only which screens count as a standing modal.
@task_item n=2 part=not_needed
The web-page surface and the rendered-page absences: viola renders no page and the chunk adds none.
@task_item n=3 part=not_needed
viola run and viola revive start refusals and the revive --json refusal codes: no start path or start refusal changes, and there is no live start in this phase.
@task_item n=4 part=not_needed
Exit 21 causes, their --json causes, socket-directory and strict-modes refusals and the fixed refusal pairs: instance reachability and home-tree checks are untouched.
@task_item n=5 part=not_needed
viola link and unlink output and their --json documents, and the handoff transfer marker: linking and control transfer are not part of the send gate.
@task_item n=6 part=not_needed
Wrapper fault --json detail codes and peer reply screening of code words: the wire between the CLI and the wrapper carries the same refusal codes as before.
@task_item n=7 part=not_needed
CLI colour decision order and colour depth contracts, and the --help verb groups: no verb, flag or colour path is added or changed.
@task_item n=8 part=not_needed
viola wait output forms and viola last output: they read turn events, not the pre-send gate.
@task_item n=9 part=not_needed
The command name register: no verb is added, renamed or removed.
