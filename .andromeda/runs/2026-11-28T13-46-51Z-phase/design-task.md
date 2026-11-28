@task_prose part=chunk
The fleet console gains actions over the view the fleet-console-view chunk built: a selection of one row of the board, and verbs reached from that selection by keyboard alone.
Each action acts through the CLI verb it names, with the same outcome and the same refusal as that verb typed at a shell, and no second code path.
A send shows its readback as the frame verb's own reply, never the view's STATUS cell; a refused action shows as the verb's unable line on the console's own surface.
No key trap: every mode the actions add is left by keyboard alone; a decoder of key sequences replaces the scaffold's one-class-per-read key decoder where the actions need it.
Which verbs, their keys, the selection model, the send's text entry, any confirm and any scroll stay the operator's open forks; capability needing a live reading is claimed gated.
@task_item n=1 part=needs
The fleet console's key bindings: new action keys join the existing q quit, Ctrl-C interrupt and any-other-key unbound bindings, read by a decoder of key sequences rather than one class per read; q and Ctrl-C keep their meaning wherever the console stands.
@task_item n=2 part=needs
No key trap: every mode an action adds (a selection, a text entry, a confirm) is left by keyboard alone, and the console's quit still restores the terminal from raw to cooked mode on its own terminal.
@task_item n=3 part=needs
A selection of one row of the board: it names its row by the board's item (the session), never by the drawn text, since a cut line ends in ... and an unwrapped NAME is cut on its own.
@task_item n=4 part=needs
The view's caption row, line cut and more line: rows past the body show only as one ... <n> more line, so a row reached by keyboard needs the view to bring it into the body; nothing else of the view's content changes beyond what a selection and an action's result need.
@task_item n=5 part=needs
Showing which strip is selected under the board's styling rules: colour only as a second cue, arrival amber reserved for dialog-pending, bold only for the NAME column, no background colours, ASCII-only text glyphs, no decorative icons, stable session slots and no re-sorting of strips.
@task_item n=6 part=needs
Every action acts through its CLI verb (the verbs named for every-verb-from-the-console: open, focus, send, answer, last, pause, release, revive, and the switch of model and effort): the same outcome and the same refusal as the verb at a shell, no second code path.
@task_item n=7 part=needs
The send readback: a console send shows the send frame verb's own reply (read back, unable, unconfirmable, and the slash-command readbacks), never the STATUS cell, which comes from the agents thread at most every 5 s and reads unknown after a failed read.
@task_item n=8 part=needs
Nothing announces success before confirmation: no optimistic send confirmation, and a turn-ended never fills the readback box; the console shows a send as done only once its frame verb has replied.
@task_item n=9 part=needs
The readback box as the signature element and its CLI mirror: what the console shows for a send's result follows the readback box and the viola send readback mirror's states.
@task_item n=10 part=needs
Waiting for an action's reply draws no spinner, elapsed-time counter or animation; busy is not animated, and the step counter stays the one progress pattern.
@task_item n=11 part=needs
A refused action shows as the verb's own unable line (the fixed unable form, its two-space form, and the hint where the verb gives one) on the console's surface, with no internals, no stack trace and no path or pid beyond what the verb's line carries.
@task_item n=12 part=needs
Refusal hints and peer reply screening reach the console through the verbs it calls: the refusal codes of a send (not-delivered and its causes, not-running, human-typing, endpoint-lost, unwrapped-name, socket-dir, server-verify, unverified-cli) and the screening of a peer's reply code word decide the line shown.
@task_item n=13 part=needs
Unwrapped sessions are never a target: a bare strip may be selected but an action on it shows the verb's unwrapped-name refusal; a bare strip is never cocked.
@task_item n=14 part=needs
Pause and release as control transfer: the console reaches viola pause and viola release (release a human verb, the console driven by the human) through the CLI verbs, showing their verb output and the handoff transfer marker with ASCII arrows and no handoff blue; no hint the console shows names viola release.
@task_item n=15 part=needs
Answer from the console: a dialog-pending strip, shown as a cocked strip, is answered through viola answer; a held PermissionRequest question is answered through that verb, and its verb output and refusals are what the console shows.
@task_item n=16 part=needs
Focus, last, open and revive from the console: each shows that verb's own output and refusals (the viola focus exit-1 codes and exit-21 refusal, the viola last header and message, the viola revive refusals and output after the child starts) on the console's surface.
@task_item n=17 part=needs
The switch of a session's model and effort reached by keyboard alone through the CLI switch verb, which stays CLI-only; its takes-effect-for-the-next-turn claim is not this chunk's.
@task_item n=18 part=needs
The console carve-out: the fleet console is governed by no verb-output rule, so the no-interactive-prompt-or-pager rule and the verb-output streams do not bind the console's own text entry or confirm, while each verb it calls keeps its own output contract.
@task_item n=19 part=needs
The send's text entry, if the operator's fork keeps one: the send verb takes prompt text from stdin or a file only, and its control-character, empty-text and paste-cut refusals stand for text typed in the console.
@task_item n=20 part=needs
Every line the console draws from a verb's reply or a row escapes every control character, is never wrapped, reads no COLUMNS, and takes its width from the terminal's own width source.
@task_item n=21 part=needs
The viola console refusals stand around the actions: no terminal, home failing its read rule, and home widened mid-run still refuse as they do.
@task_item n=22 part=needs
The fleet console's own capability clauses that need a live reading are claimed gated, the reading the operator's to grant; the console's tests drive it over its own pseudo-terminal by keystrokes, read state rather than the screen, and start with the test-scoped PATH, never a real claude or a real home.
@task_standing n=1
convention: Every new line the console draws for an action, a selection or a result is ASCII-only, using the CLI's text glyphs and separators, with no emoji or check-mark prefix.
home: surface-cli · iconography
@task_standing n=2
convention: Every new human-facing line escapes control characters and carries no internals in its errors or hints.
home: anti-patterns-never-do-these
@task_standing n=3
convention: Every new console state or key added to the fleet console is a contract of the console surface: its keys, its view parts and its refusals.
home: surface-console
@task_standing n=4
convention: Every new coloured span the console adds uses only the CLI colour tokens (attention, dim, bold, reset) and amber and dim alone, colour never the only cue.
home: color-palette · surface
@task_item n=1 part=not_needed
The web-spa surface and the rendered-page absence: viola renders no page, and the console adds none.
@task_item n=2 part=not_needed
viola verify output and refusals, and the CLI verification state: the console offers no verify action here.
@task_item n=3 part=not_needed
viola up output and outcome words: up is not among the verbs the console reaches.
@task_item n=4 part=not_needed
viola link and unlink output and their --json results: link and unlink are not among the console's verbs.
@task_item n=5 part=not_needed
The hook and mcp verbs, viola plugin install output, and viola adopt refusals: none is reached from the console.
@task_item n=6 part=not_needed
viola wait output: the console reads its own state and does not wait on events through that verb.
@task_item n=7 part=not_needed
viola run start refusals and the viola --help verb groups: the console starts no wrapped child through run and changes no help text.
