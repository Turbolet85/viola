@task_prose part=chunk
Builds the lineage registry: a wrapped session's declared parent and role are recorded once, at wrapper start, as a new lineage event in that instance's own event log, and read back the same after a crash and a replay.
Adds a launch-time parent declaration to viola run (a flag or the launching session's own identity); what a starting wrapper can see of its parent is P3's first measurement, from a test or a no-start probe.
viola list shows the fleet as trees inside the board's six fixed columns, plain when stdout is not a terminal; what list --json gains is a fork for the operator.
revive keeps the parent the lineage recorded; a reused name never inherits an old parent; a parent edge is a drawing label only and never changes the wheel, the links or whether a send is accepted.
Observed links (0.1.0's link / unlink pairs) are drawn apart from parent edges on every view that shows both, and a link name is shown only after it parses as a viola name.
@task_file n=1 path=src/cmd/run.rs mark=edits
what: the viola run verb, its launch flags and start output — edits: the launch-time parent declaration
@task_file n=2 path=src/run/ mark=edits
what: the wrapper's start path, where a wrapped instance begins and writes its event log — edits: the start path that writes the one lineage line; where observed links are held
@task_file n=3 path=src/cmd/revive.rs mark=edits
what: the viola revive verb, which restarts a logged instance — edits: the parent carried into the revived instance
@task_file n=4 path=crates/viola-core/ mark=edits
what: the core event kinds and the event line contract every wrapper writes — edits: the event-kind list (a new lineage kind, never Claude-named)
@task_file n=5 path=crates/viola-state/ mark=edits
what: the board read model that viola list reads instances, links and states from — edits: the read model, the replay of the lineage event
@task_file n=6 path=src/human/board/ mark=edits
what: the viola list board drawing — its columns, rows, BAY line and link pairs — edits: the tree drawing, the piped plain form, links drawn apart from parent edges
@task_file n=7 path=src/cmd/list.rs mark=edits
what: the viola list verb and its --json payload — edits: the list --json payload contract
@task_file n=8 path=src/cmd/link.rs
what: the viola link verb and its output, the 0.1.0 observed session links
@task_file n=9 path=src/cmd/unlink.rs
what: the viola unlink verb and its output, the 0.1.0 observed session links
@task_item n=1 part=needs
viola list is the board: its six fixed columns, their captions, words and slot order stay fixed while the fleet is drawn as trees inside them.
@task_item n=2 part=needs
The board's row order, NAME column width, NAME cut and the unwrapped row and separator as the tree drawing must lay them out; one strip per instance and stable session slots hold under a tree.
@task_item n=3 part=needs
The piped form: when stdout is not a terminal the board prints plain, with no colour spans; the colour decision order (stdout not a terminal, NO_COLOR, TERM=dumb, --json) decides it.
@task_item n=4 part=needs
Tree edges and the parent label are drawn with the CLI's ASCII-only text glyphs and separators (the -> arrow, two spaces, space-hyphen-space); no decorative icons, no box-drawing glyphs.
@task_item n=5 part=needs
Lines never wrapped, COLUMNS never read, no hardcoded widths: the tree indent sits inside the human tables' width source and the WHEEL column padded to 23.
@task_item n=6 part=needs
Every name drawn on the tree or a link row escapes every control character; a link name is shown only after it parses as a viola name.
@task_item n=7 part=needs
Links drawn apart from parent edges: the viola link and unlink output (result line, printed names, since) and the board's link pairs stay as they are, and a parent edge is never shown as, nor merged into, a link.
@task_item n=8 part=needs
A parent edge is a drawing label only: the wheel shown as words, the STATUS and dialog-pending cocked strip, and whether a send is accepted or refused are unchanged by any parent.
@task_item n=9 part=needs
list --json is one stdout document with data on stdout and messages on stderr; whatever it gains for lineage is a fork for the operator, and no hint line prints under --json.
@task_item n=10 part=needs
The viola run launch declaration: viola run prints nothing while wrapped and its child-started output and start refusals (no path or pid shown) stay in their fixed forms; any new refusal for a bad parent declaration is a fork for the operator with its own fixed unable line and exit code.
@task_item n=11 part=needs
viola revive keeps the recorded parent: its output after the child starts, its --list line and --list --json, and its --json exit-1 refusal codes stay fixed; any new refusal or field is a fork.
@task_item n=12 part=needs
A reused name never inherits an old parent: the lineage belongs to one instance's launch, so a stale or dimmed strip under the same name never lends its parent to a new run.
@task_item n=13 part=needs
The lineage event is a new event kind in the core only, never Claude-named, on the event line contract, written once by the wrapper as its one writer and replayed the same after a crash.
@task_item n=14 part=needs
A strict-modes failure still refuses the whole board, and a failed agents read and the skip counts stay visible on a board drawn as trees.
@task_item n=15 part=needs
Unwrapped sessions are never a target and a bare strip is never cocked: an unwrapped session may be drawn on the tree but is never a declared parent's target of any verb.
@task_item n=16 part=needs
Every new closed word, event kind, field, column, refusal or time bound is a fork for the operator priced into one founder-forks block at P3's end; the measurements come first at P3, from a test or a no-start probe.
@task_item n=17 part=needs
No internals in errors or hints and no stack traces: what of a parent's identity a message shows, or lands in a file another session can read, stated at P3.
@task_standing n=1
convention: each new human-output line or column uses only the fixed phraseology vocabulary and ASCII-only glyphs, colour only as a second cue
home: anti-patterns-never-do-these
@task_standing n=2
convention: each new refusal names its fixed unable line, its hint where one is owed, its exit code and its --json code
home: surface · surface-cli
@task_standing n=3
convention: each new verb output is checked in both the TTY form and the plain piped form, and in --json as one stdout document
home: surface-cli
@task_item n=1 part=not_needed
The send readback and readback box rows and the viola send mirror: the chunk changes no send, its words or its confirmation.
@task_item n=2 part=not_needed
viola verify output and refusals, and the run and revive automatic-verify line: lineage does not touch verification or stamps.
@task_item n=3 part=not_needed
viola wait output and viola last output: no wait or last form gains a parent.
@task_item n=4 part=not_needed
Peer reply screening and the wrapper fault --json detail: no peer protocol code word is added by the lineage event.
@task_item n=5 part=not_needed
The pause and release control-transfer rows and the handoff transfer marker: a parent edge transfers no control.
@task_item n=6 part=not_needed
The hook and mcp verbs, viola plugin install output and the PermissionRequest question: none is reached by a launch declaration.
@task_item n=7 part=not_needed
The web-page surface absence and the rendered-page absence (radius, depth, spacing tokens): viola renders no page.
@task_item n=8 part=not_needed
The fleet console: Epoch 6's own entry, no verb's output; this chunk draws trees only on viola list.
@task_item n=9 part=not_needed
CLI colour depth rows (16-colour, 256-colour, truecolor) and the accent hex: the tree adds no colour.
