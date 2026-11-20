@task_prose part=chunk
Changes the pre-send gate of the wrapper's send (the screen predicate viola run reads before it types a driver's text) so a compiled modal literal quoted inside the session's own transcript is no longer read as a modal standing.
A driver's send into an IDLE session whose screen holds such a quoting row is delivered, not refused not-delivered input-not-ready (exit 13).
A real modal standing, every compiled modal literal on its recorded screens, still refuses the send exactly as today; the chunk's tests carry both directions, with a remove-the-guard control on the refusal side too.
The discriminator is chosen from the screen model and the recorded screens with no live start; no relied ledger row or its verify probe is meant to change.
@task_item n=1 part=needs
The pre-send readiness predicate in viola-agent-claude (the input-box settle reading): a modal literal standing inside a transcript row (a chat message quoting it) no longer counts as a standing modal, so a driver send into an idle session is delivered.
@task_item n=2 part=needs
The viola send refusal input-not-ready and its exit code 13: still returned, unchanged in word, code and layout, whenever a real modal stands; only the false refusal on a quoting transcript row is removed.
@task_item n=3 part=needs
The false-negative side is the dangerous one: the driver must never type into a standing dialog; where the gate cannot tell a quoted literal from a standing modal, it keeps to the side that refuses the send.
@task_item n=4 part=needs
The same settle reading also frames verify's Run B pastes: the change to the predicate reaches that use as well and must keep it reading a standing modal as before.
@task_item n=5 part=needs
Whether viola status, viola list and the dialog relay read the same predicate: the fix moves them together or leaves them deliberately apart, and a dialog-pending reading must stay true for a real dialog.
@task_item n=6 part=needs
The send is refused while the human types: the gate keeps that refusal; the change narrows only the modal reading, never the human-typing guard.
@task_item n=7 part=needs
Human keystrokes are never blocked and a human key takes the wheel: the gate binds the driver's send only, never the human's input.
@task_item n=8 part=needs
The discriminator reads the vendor's screen only by its modal signatures: no screen-content parsing beyond signatures and no inspection of claude's own content beyond what the signature match needs.
@task_item n=9 part=needs
Test inputs are the fake agent's screens and the recorded fixture sets under fixtures/claude/<cli-version>/; a screen no set holds is added as a synthetic screen beside the recorded sets, named as synthetic, got without a live start.
@task_item n=10 part=needs
No live start, no read of the operator's real Claude config, no scheduled-wake or timer tool: live readings never gate, and the chunk's checks run on recorded and synthetic screens only.
@task_item n=11 part=needs
The open carry on the intermittent rig rehearsal red (box read empty after x and del): at its next red, read the printed rig summary and the kept after-del frame and name the cause before mending the wait; no retry rule, no loosened entry.
@task_item n=12 part=needs
The watched supervise test stopping with ctrl-c pressed again (fake agent exit code read 1 where 0 is asserted): a recurrence is read from its log before any mend, and no bound or assertion moves to get past it.
@task_item n=13 part=needs
No documentation leaf takes a figure the code states.
@task_standing n=1
convention: Each new test runs inside the existing test suites, and an a11y failure is a nextest failure in that suite, with no separate a11y driver
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: Each new test is deterministic: zero flakiness, no retries, and the CI test gate allows no skips
home: 10-slo-invariants-a11y-budgets
@task_standing n=3
convention: A fixed CLI string a new test asserts, such as a refusal's code word and hint, is pinned by its literal
home: cognitive-accessibility · 8-cognitive-accessibility
@task_standing n=4
convention: A new boundary test is sequenced on recorded screens, never on a live reading
home: keyboard-test-harness
@task_item n=1 part=not_needed
Colour and SGR rows (amber DIALOG word, bold NAME, dim stale rows, colour decision inputs, name truncation): the chunk prints no new coloured output.
@task_item n=2 part=not_needed
The viola list board's caption order, empty board and unwrapped separator layout: the chunk changes no board rendering.
@task_item n=3 part=not_needed
viola revive refusals, revive --list and revive --json documents: the chunk does not touch revive.
@task_item n=4 part=not_needed
viola link and unlink, and viola plugin install, their result lines and exit codes 20 and 21.
@task_item n=5 part=not_needed
Hook call, frame verb and MCP frame tool refusals for socket-directory, strict-modes and server-verify failures, and the instance-unreachable details.
@task_item n=6 part=not_needed
viola --help rows: usage line, group labels, handoff row, served verbs.
@task_item n=7 part=not_needed
Screen-reader targets, setups and the manual Orca and keyboard passes: the chunk changes no output a screen reader reads.
@task_item n=8 part=not_needed
Absences of a web or graphical frontend, a touch surface, internationalization and a rendered page, and the structured violation JSON rows.
@task_item n=9 part=not_needed
Log identity tags (process, service_name): the chunk adds no log identity.
@task_item n=10 part=not_needed
config.json dialog_hold_s, the MCP wait tool's timeout_ms and viola wait's waiting and timed_out lines: the chunk moves no time bound.
@task_item n=11 part=not_needed
viola verify's interrupted pair and interrupted exit: the chunk does not change verify's interruption handling.
