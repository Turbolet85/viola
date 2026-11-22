@task_prose part=chunk
Builds the lineage registry: a wrapped session's declared parent and role recorded once, at wrapper start, as a lineage event in that instance's own event log, read back the same after a crash and a replay.
Adds a launch-time parent declaration through viola run, and shows the fleet as trees on viola list in the board's fixed columns, plain when stdout is not a terminal; what list --json gains is an operator fork.
Keeps the parent on revive, never lets a reused name inherit an old parent, and keeps a parent edge a drawing label only (no effect on the wheel, the links or send acceptance).
Draws observed links (0.1.0's session links) apart from parent edges on every view showing both, a link name shown only after it parses as a viola name.
Measures first at P3, from tests or no-start probes: what a starting wrapper can see of its parent, what each new record writes to disk, what outlives a SIGKILL, and what of a parent's identity lands in a file another session can read.
@task_file n=1 path=crates/viola-core/src/lib.rs mark=edits
what: the event kinds and the event line contract of viola-core — edits: the event kind list (a new lineage kind, never Claude-named)
@task_file n=2 path=crates/viola-core/src/obs.rs mark=edits
what: the obs event catalog — edits: the catalog entry for the lineage event
@task_file n=3 path=src/run/mod.rs mark=edits
what: the viola run start path of the wrapper, the one writer of an instance's events — edits: the start step that writes the lineage line
@task_file n=4 path=src/cmd/run.rs mark=edits
what: the viola run verb and its launch arguments — edits: the parent and role declaration at launch
@task_file n=5 path=src/cmd/revive.rs mark=edits
what: the viola revive verb and its start — edits: the parent carried from the recorded lineage into the revived instance
@task_file n=6 path=crates/viola-state/src/board.rs mark=edits
what: the board read model behind viola list — edits: the parent and role per row and the fleet tree order
@task_file n=7 path=crates/viola-state/src/replay.rs mark=edits
what: the event replay that rebuilds an instance's state after a crash — edits: the lineage event read back
@task_file n=8 path=src/human/board/rows.rs mark=edits
what: the viola list board rows in the fixed columns — edits: the tree drawing and its plain piped form
@task_file n=9 path=src/cmd/list.rs mark=edits
what: the viola list verb, its board and its --json document — edits: the list --json payload contract
@task_file n=10 path=src/run/links.rs mark=edits
what: where the wrapper stores and shows observed session links — edits: links shown apart from parent edges, a link name shown only after it parses as a viola name
@task_file n=11 path=src/cmd/link.rs mark=edits
what: the 0.1.0 viola link verb and its output
@task_file n=12 path=src/cmd/unlink.rs mark=edits
what: the 0.1.0 viola unlink verb and its output
@task_item n=1 part=needs
The viola list board: its fixed columns and captions (the six captions, their words and slot order fixed) carrying the fleet as trees, with no new link column.
@task_item n=2 part=needs
The viola list board's plain lines and uncoloured modes when stdout is not a terminal: the tree drawn plain, TTY detected the standard way.
@task_item n=3 part=needs
The viola list --json document and the one-document rule of --json mode: any field list --json gains for parent or role is an operator fork, and the human board and the JSON stay a contract.
@task_item n=4 part=needs
The viola run verb's usage and its passed start output: a launch-time parent and role declaration (a flag or the launching session's identity), its argument form and per-verb help.
@task_item n=5 part=needs
The viola revive start and its --list row: a revived instance keeps the parent its lineage recorded; the revive preflight order and refusals stay as they are.
@task_item n=6 part=needs
Board row ordering, name truncation and column widths: the tree order must live inside the board ordered by name, NAME truncation of unwrapped names only, and the computed column widths.
@task_item n=7 part=needs
ViolaName rules over the parent and link names shown: a name never case-transformed, a CLI target always a ViolaName, and a link name drawn only after it parses as a viola name.
@task_item n=8 part=needs
Board row escaping: a parent, role or link name drawn on the board is escaped like any other row cell.
@task_item n=9 part=needs
viola link and unlink output and their printed names: observed links shown apart from parent edges wherever both appear.
@task_item n=10 part=needs
Words carry every state and the hand-written SGR styling: a parent edge or tree glyph never carries state by colour alone, and the viola list styling and colour depth rules hold for the new tree.
@task_item n=11 part=needs
A parent edge changes no wheel, no link and no send acceptance: the board versus strip verbs and the send refusals stay unaffected by lineage.
@task_item n=12 part=needs
Any new refusal for a bad parent declaration is an operator fork; if it lands it joins the shared refusal word column, the refusal line form, an exit code, and a hint naming the next verb, last on stderr, never quoting text or naming paths.
@task_item n=13 part=needs
Results on stdout, the rest on stderr, with no banner, spinner, prompt, pager or footer added by the tree view or the launch declaration.
@task_item n=14 part=needs
The viola list strict-modes refusal, failed agents read, empty board and unwrapped rack separator stay as they are when trees are drawn.
@task_standing n=1
convention: every new human output line or board form is a contract, pinned by a test of its exact words and slot order
home: surface-cli
@task_standing n=2
convention: every new data field shown on a verb also has its --json form inside that verb's one JSON document
home: surface-cli
@task_standing n=3
convention: every new refusal carries its own hint as the last stderr line and a fixed refusal word in the shared column
home: surface-cli
@task_item n=1 part=not_needed
The web surface absence: viola renders no page, and this chunk adds none.
@task_item n=2 part=not_needed
viola verify output and refusals: the chunk runs no verify and changes none of its lines.
@task_item n=3 part=not_needed
viola plugin install output and refusals: the plugin install path is not reached.
@task_item n=4 part=not_needed
viola answer, viola wait and viola last output rows: dialog answering, waiting and the last message are not changed by lineage.
@task_item n=5 part=not_needed
The readback mirror and the viola send output lines (read back, unable, unconfirmable, paste-cut): the send's own output is untouched.
@task_item n=6 part=not_needed
Deep links or URL coupling in the CLI: nothing in the chunk links out.
