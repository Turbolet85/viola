@task_prose part=chunk
Changes the wrapper's window step so a placed window reaches its workspace while the compositor's active workspace stays where it was (the operator's «no focus change» for an opened window, v2-07).
Every caller of the move — viola open, viola up, viola revive under --origin — reads the same answer, each one's form decided at P3.
The Hyprland seam's tap, fake and recorded move fixture follow the new move form; the fixture is re-recorded only in one operator-granted live leg.
v2-07's acceptance gains a gate-read clause: the active workspace read before a placing open equals the one read after; v2-07 is claimed and proved again, its live half gated on the operator.
Folded item: fixes where the automatic verify of a new CLI runs at a start and what it stamps when its own preconditions fail (an untrusted folder, a modal, a red it caused itself).
@task_file n=1 path=src/run/window.rs mark=edits
what: the wrapper's window step that places a started instance's window on its workspace through the Hyprland seam — edits: the move-to-workspace request it sends
@task_file n=2 path=crates/viola-hyprland mark=edits
what: the Hyprland seam: its closed request set, its tap, its fake and its replies — edits: the move request's form, the fake's reply set
@task_file n=3 path=src/run/auto_verify.rs mark=edits
what: the automatic verify of a new CLI version run at a start, and the stamp it writes
@task_file n=4 path=src/cmd/run.rs mark=edits
what: the start of viola run (and the starts that share it), where the automatic verify is triggered — edits: the start
@task_item n=1 part=needs
Window placement of a started instance: viola open places its window on a workspace and must leave the active workspace unchanged (no focus change for an opened window).
@task_item n=2 part=needs
viola up places each started instance's window and must leave the active workspace unchanged, reading the same move answer as open.
@task_item n=3 part=needs
viola revive under --origin places the revived window; its window placement promise must read the same move answer (or a named difference).
@task_item n=4 part=needs
The open verb's options that steer placement (its parent/origin defaults) stay as they are; only the move's effect on the active workspace changes.
@task_item n=5 part=needs
The passed start output of open, up and revive must not claim or imply that the screen moves to the placed window.
@task_item n=6 part=needs
The automatic verify's stamp: what is recorded for a CLI version when the verify's own preconditions fail must not mark the version failed or unverified for every later start.
@task_item n=7 part=needs
viola verify's record and summary (pass, fail, absent counts) as the stamp writer that the automatic verify shares; a stamp made in an untrusted place must not stand as the version's verdict.
@task_item n=8 part=needs
viola verify refusals and stop: a verify that cannot run in a trusted place or meets a modal is a stop or refusal of its own, not a failing row.
@task_item n=9 part=needs
The start's preflight order: where the automatic verify sits among the start's steps, and that a start under a config with automatic verify still starts or refuses by the existing refusals.
@task_item n=10 part=needs
Human output is a contract: any line the chunk changes or adds on open, up, revive or verify keeps the shared refusal word column, hint rules and stdout/stderr split.
@task_item n=11 part=needs
--json documents of open, up, revive and verify keep their shape; a new field or code for the verify's own-precondition failure is added only within the existing document forms.
@task_item n=12 part=needs
Window placement is no interactive step: no prompt, no spinner, no progress glyph is added while the move is sent.
@task_standing n=1
convention: every changed or added human output line of a verb is a contract — words carry the state, results on stdout and the rest on stderr, a refusal ends with a hint naming the next verb
home: surface-cli
@task_standing n=2
convention: every changed --json output stays one document with the shared exit codes and their --json form
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page; nothing here reaches a web view.
@task_item n=2 part=not_needed
viola send output, its readback mirror, readback vocabulary and send refusals: the chunk sends no text to an instance.
@task_item n=3 part=not_needed
viola list board and BAY line rows (columns, truncation, tree, links block, stale rows, colour depth): the board is not changed.
@task_item n=4 part=not_needed
viola mcp tools and their results: no MCP tool changes.
@task_item n=5 part=not_needed
viola wait, viola last, viola answer and viola switch outputs: none of these verbs is touched.
@task_item n=6 part=not_needed
Wheel and handoff verb outputs (pause, release) and viola link / unlink output: not reached.
@task_item n=7 part=not_needed
CLI token rendering rows (colour, spacing, radius, motion, callsign and label roles): no styling changes.
@task_item n=8 part=not_needed
viola plugin install and viola adopt rows: not reached by the move or the automatic verify.
@task_item n=9 part=not_needed
Frame verb exit-21 hints and instance-unreachable details: the chunk talks to no running wrapper endpoint.
