@task_prose part=session
The session measured whether real claude on Linux loses a key typed right after a resize of its own pty: on 2.1.330 and 2.1.328 each start kept 12 of 12 keys typed 0, 20 or 100 ms after a resize; no loss in 12 rounds at these delays on these versions, never a claim that it never loses.
The measurement is of the CLI unwrapped, not of viola's own poll lag or the driver's paste; no product change followed, the fix half was not needed.
It added a KR arm to the test-only live rig (labels KR, KR-2, KR8, KR8-2): it resizes its own pty, types x and DEL, and reads the box row's cell attributes only, never text, into closed summary words, counts, delays and settle times.
It added a KR-only settle that counts quiet from a byte after the key, a word that tells a late redraw from a dirty box, and two rehearse-only options (a quiet window of 100 to 400 ms and a DEL seam) that live starts refuse as a usage refusal, exit 2.
No product crate, no CLI verb, no viola output, no colour, no glyph, no hint and no exit code of viola changed; nothing was retired.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig's core: its arms, sessions, settle, summary keys and the KR arm with its tally, seam and quiet window
@task_file n=2 path=crates/viola-e2e/src/rig/keys.rs
what: the test-only live rig's key grants: which keys and actions a start may type, now a resize grant and the box-typing check
@task_file n=3 path=crates/viola-e2e/src/rig/screen.rs
what: the test-only live rig's screen model of the child's terminal, read as cell attributes, now resizable
@task_file n=4 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the test-only live rig's command line: its rehearse and live verbs and their options, a usage refusal at exit 2
@task_file n=5 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsals over the fake agent, now five KR rehearsals
@task_item n=1 part=touches
A row on Linux terminals as the CLI platform: the session adds a Linux measurement that real claude keeps a key typed 0, 20 or 100 ms after a resize of its own pty, on 2.1.330 and 2.1.328, in 12 rounds each; a picker checks whether the platform row states anything about keys or resize that this reading confirms or must qualify.
@task_item n=2 part=touches
Rows on the send path and its refusals after a size change (the input-not-ready and human-typing refusal hints, the send readback): the session measured the unwrapped CLI only, not viola's send, poll lag or the driver's paste; a picker checks that no such row claims a resize-loss behaviour this reading now contradicts, and that none is widened by it.
@task_item n=3 part=touches
Rows on width source and the terminal size (COLUMNS never read, human-table width source, lines never wrapped): the rig changes the child's pty size each round; these rows govern viola's own human output, which did not change, so a touch only if a row speaks of the child's resize.
@task_item n=4 part=touches
The usage exit code (exit 2) and the plain refusal forms: the test-only rig refuses a bad rehearse option or a live start with a seam as refused: usage, exit 2, before any ledger line; a touch only if a row extends viola's exit-code or refusal-line rules to the test rig.
@task_item n=5 part=touches
Rules against parsing screen content: the KR arm reads the box row's cell attributes, never text, and keeps closed words, counts, delays and settle times; a design-system row on screen reading or never quoting text is touched only if it speaks of the rig, otherwise it holds as stated.
@task_item n=1 part=untouched
Brand identity, colour palette, typography, spacing, depth, border radius and iconography: no colour, glyph, weight, column, strip or readback-box rendering of viola changed.
@task_item n=2 part=untouched
The CLI verb contracts (list board, revive, run start refusals, link and unlink, verify, wait, last, send mirror) and their --json documents: no verb, output line, hint, exit code or refusal code of viola changed.
@task_item n=3 part=untouched
The wrapper fault details, peer reply screening, wrapper socket and exit 21 causes: no product socket, MCP or instance path changed.
@task_item n=4 part=untouched
The CLI verification state and its stamps: the live starts ran claude 2.1.330 and 2.1.328 with no verify, stamp or trust-record change.
@task_item n=5 part=untouched
The anti-patterns and self-validation protocol: no viola output, spinner, status dot or confirmation was added.
@task_item n=6 part=untouched
Names: no command name was taken or freed; the rig's KR labels and summary keys are test-only and no viola command.
