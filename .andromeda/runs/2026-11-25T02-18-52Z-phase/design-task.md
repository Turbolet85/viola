@task_prose part=chunk
Changes the wrapper's window step so a placed window reaches its workspace while the compositor's active workspace stays where it was (the operator's F4, no focus change for an opened window), in a form measured from the compositor's own help through the one Hyprland seam and its closed request set.
Every caller of the move (viola open, viola up, viola revive under --origin) reads the same answer, its form or a named difference decided at P3.
The seam's tap, fake and recorded move fixture follow the form; the fixture is re-recorded only in a gated, operator-fired live leg, and the fake replies only what the real compositor answers.
v2-07 gains an acceptance clause a gate entry reads (the active workspace read before a placing open equals the one read after) and is claimed and proved again, its live half gated on the operator.
Folded item: where the automatic verify of a new CLI runs at a start and what it stamps when its own preconditions fail (an untrusted folder, a modal, a red it caused itself), with its own acceptance and a witness that cannot pass vacuously.
@task_file n=1 path=src/run/window.rs mark=edits
what: the wrapper's window step, which sends the move of a placed window to its workspace through the Hyprland seam — edits: the MoveToWorkspace request's form
@task_file n=2 path=crates/viola-hyprland mark=edits
what: the Hyprland seam, its closed request set, its tap and its fake reply set
@task_file n=3 path=fixtures/hyprland/0.56.2 mark=edits
what: the recorded compositor fixtures of the Hyprland seam — edits: the move fixture
@task_file n=4 path=src/run/auto_verify.rs mark=edits
what: the automatic verify of a new CLI at a start and the stamp it writes — edits: where the verify runs, what it stamps when its own preconditions fail
@task_file n=5 path=src/cmd/run.rs mark=edits
what: the viola run start, which fires the automatic verify — edits: the start
@task_item n=1 part=needs
The placement word a placing open, up or revive reports for a window (placed, unplaced, refused, unreachable, unsupported, window-unseen, malformed, oversize) stays truthful under the new move form: placed only once the compositor confirmed the move.
@task_item n=2 part=needs
viola up output and its outcome words, as up opens windows through the changed move: its line order and live outcome unchanged in meaning while the screen no longer follows the placed window.
@task_item n=3 part=needs
viola revive under --origin, its output after the child starts, as it places its window through the same move.
@task_item n=4 part=needs
viola open's placing of a window, read by the new acceptance clause: the active workspace before equals the one after.
@task_item n=5 part=needs
Nothing announces success before confirmation: a placed outcome is said only after the compositor's reply, never on sending the move.
@task_item n=6 part=needs
viola focus remains the one verb that changes what the operator looks at; placing a window is no focus change.
@task_item n=7 part=needs
The automatic-verify line of viola run and revive (verifying, verifying elsewhere, did not verify, no stamp, earlier red, its line form) for a start whose verify could not run in a trusted place or met its own failed precondition.
@task_item n=8 part=needs
The CLI verification state (verified, unverified-cli): a stamp from a verify that failed its own preconditions must not leave the version reading unverified for every later start.
@task_item n=9 part=needs
viola verify's stamp writer and its output: the stamps.json update and its refusal, the summary, the fail word, a stopped or refused run, as the folded item decides what is stamped.
@task_item n=10 part=needs
The unverified-cli refusal hint a later start shows, reached through a wrongly kept failing stamp.
@task_item n=11 part=needs
viola run start output and refusals, silent while wrapped, as the start's automatic verify changes where it runs.
@task_item n=12 part=needs
The exit codes a placing verb and a start return stay as they are; a move form change adds no new exit code unless P3 names one.
@task_standing n=1
convention: every new or changed human output line is ASCII only, with words not glyphs, and escapes control characters
home: surface-cli
@task_standing n=2
convention: data goes on stdout and messages on stderr; a --json result stays one stdout document
home: anti-patterns-never-do-these
@task_standing n=3
convention: no internals (paths, pids, compositor dispatch names) in any new error, refusal or hint line
home: anti-patterns-never-do-these
@task_standing n=4
convention: no spinner or elapsed counter while a verify or a move is waiting; the step counter is the one progress pattern
home: surface-cli · surface
@task_item n=1 part=not_needed
Brand identity rows on the strip board, slots, dialog-pending cocking and the readback box: the chunk draws no strip and changes no board.
@task_item n=2 part=not_needed
Colour palette, typography and spacing rows: no colour, type or spacing changes in this chunk.
@task_item n=3 part=not_needed
Iconography and the viola list columns: the list board and its table are not touched.
@task_item n=4 part=not_needed
The web-page surface absence: viola has no web surface.
@task_item n=5 part=not_needed
Send readback and send mirror rows, wait output, last output and link or unlink output: no send, wait or link path changes.
@task_item n=6 part=not_needed
Peer reply screening and wrapper fault --json detail rows: the wrapper socket protocol is not touched.
@task_item n=7 part=not_needed
Exit 21 causes, strict-modes refusals and socket-directory refusals: the chunk changes no instance-reachability or home-tree check.
@task_item n=8 part=not_needed
viola adopt refusals and the revive --json refusal codes: no refusal of adopt or revive changes.
@task_item n=9 part=not_needed
The self-validation protocol's squint, swap and domain tests: they review visual design, which this chunk does not change.
