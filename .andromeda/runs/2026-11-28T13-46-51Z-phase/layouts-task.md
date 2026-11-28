@task_prose part=chunk
viola console gains actions over the fleet view: a selection of one board row and the verbs reached from it by keyboard alone.
Each action acts through the CLI verb it names, with the same outcome and the same refusal as that verb typed at a shell, and no second code path.
A send shows its readback as the send frame verb's own reply; a refused action shows the verb's unable line on the console's own surface.
No key trap: every mode the actions add is left by keyboard alone, read by a key decoder of key sequences that replaces the scaffold's one-class-per-read decoder.
Which verbs, their keys, the selection model, send text entry, confirm, scroll and any split stay open operator forks; a clause needing a live reading is claimed gated.
@task_item n=1 part=needs
The console surface itself: the terminal fleet console as its own surface, its frame, its body lines and its redraw, now carrying a selected row and an action's result.
@task_item n=2 part=needs
Every action goes through the CLI verb it names: the CLI holds every action, so the console's outcome and refusal for a verb are that verb's, never a console-only path.
@task_item n=3 part=needs
The verbs the console may reach (open a child, focus, send, answer, last, pause, release, revive, and switch's console leg): the CLI verb boundary of each one the console offers, its target and its arguments.
@task_item n=4 part=needs
A CLI target is a wrapped ViolaName: the selection names its row by the board's item, never by the drawn or truncated NAME text, and passes that name to the verb uncut and never case-transformed.
@task_item n=5 part=needs
A send's readback: the send verb's read-back and readback mirror lines and its readback vocabulary words are what the console shows after a send, never the board's STATUS cell; turn-ended is no send outcome.
@task_item n=6 part=needs
A refused action shows as the verb's unable line: the CLI refusal line form, the shared refusal word column, the hint as the verb gives it, and the frame verb exit-21 hints and instance-unreachable refusal where a frame verb fails.
@task_item n=7 part=needs
The send verb's refusals before any request (empty text, control character) and its text taken from stdin or a file reach the console's send text entry through the verb, never re-implemented.
@task_item n=8 part=needs
Switch's console leg (v2-31): viola switch is CLI-only, so the console reaches it through the switch verb and shows the switch verb's own switched or refusal output.
@task_item n=9 part=needs
Release is a human verb: the console, driven by the human, may reach it, and the wheel verb output of pause and release is what it shows; no hint the console draws names viola release.
@task_item n=10 part=needs
Answer, last, focus, open, revive and wait reached from the console show those verbs' own output lines and refusals, including the answer verb's answered, not-delivered and unknown-dialog lines and the revive refusals.
@task_item n=11 part=needs
No key trap and a key decoder of key sequences: the console's key handling grows from one class per read (interrupt, quit, unbound) to sequences, and every added mode is left by keyboard alone.
@task_item n=12 part=needs
The console's restore case stays read on the console's own terminal: line mode raw while the frame stands and cooked after the quit, never that the mode after equals the mode before.
@task_item n=13 part=needs
Rows past the body show as one more-rows line with no scroll built, so a row reached by keyboard needs the view to move to it; scrolling itself is an operator fork.
@task_item n=14 part=needs
The view's content changes only by what a selection and an action's result need: the board columns, its order by name, truncation, STATUS from the agents thread and its unknown after a failed read stay as the view chunk built them.
@task_item n=15 part=needs
The agents read stays off the key loop and the re-read and redraw stay on a change, so an action's verb call or a slow send reply must not block the keyboard or the view's re-read.
@task_item n=16 part=needs
The console frame floor of 80x24 bounds where a selection mark, a send's text entry, a readback and an unable line can be drawn.
@task_item n=17 part=needs
Human output is a contract and words carry every state: the selection and an action's result are shown in words, with no spinner, no progress glyph, no banner, no persistent footer and no added prompt or pager beyond the forks the operator grants.
@task_item n=18 part=needs
Amber means dialog only and the readback mirror has no colour: any styling the console adds to a selection or a result stays inside the hand-written SGR styling and the token rendering of the CLI.
@task_item n=19 part=needs
New console tests drive the console through its keystroke driver over its own pseudo-terminal, read state never the screen, keep the test-scoped PATH and never read the host's sessions, a real claude or the real home trees.
@task_item n=20 part=needs
The capability claim: matrix v2-15 (every verb from the console) and v2-31's console leg are claimed gated where a clause needs a live reading, its reading the operator's to grant; the gated fleet console view chunk stays gated.
@task_standing n=1
convention: every new human output line the console adds is a contract, results and refusals written in the CLI's own line forms and words
home: surface-cli · surface-console
@task_item n=1 part=not_needed
The web surface: viola renders no page and this chunk builds only terminal console actions.
@task_item n=2 part=not_needed
The MCP server and its tools and tool results: the console acts through the CLI verbs, not through viola mcp.
@task_item n=3 part=not_needed
viola verify output and its refusals: verify is no verb the console reaches.
@task_item n=4 part=not_needed
viola link and unlink output: linking is no verb in the console's reach.
@task_item n=5 part=not_needed
viola plugin install output and refusals: plugin install is setup, not a console action.
@task_item n=6 part=not_needed
viola --help and per-verb --help output: the console adds no help verb and changes no usage line.
@task_item n=7 part=not_needed
Deep links or URL coupling in the CLI: the console's actions name targets by ViolaName only.
