@task_prose part=chunk
Answers whether the real claude CLI on Linux (2.1.328 and 2.1.330) loses a key typed right after a terminal resize, and fixes it if it does.
Reads first, with no live start: where the claim came from, what viola does around a resize today (who sends the window-size change, what is typed after it and when, every wait on that path), and what the installed CLI bundles' strings say a resize does.
Defines what an honest measurement is and prices the cheapest live start that gives it, and prices the fix options (a settle wait, a read-back and one re-type, a refusal), each with the rows it moves.
Stops at the end of P3 with one block of operator questions; the fix is chosen only after the reading.
@task_file n=1 path=crates/viola-pty/src/lib.rs mark=edits
what: the PTY seam that sends the window-size change to the child — edits: the resize, the test-module comment over CHILD_WITHIN
@task_file n=2 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY pump that writes keys to the child after a resize — edits: the pump's writes, PasteHandle::paste
@task_file n=3 path=src/run/gate.rs mark=edits
what: the send input-ready gate on the run pump's path that types into the CLI's input box after a resize
@task_file n=4 path=src/run/wheel.rs mark=edits
what: the wheel of viola run, which passes the resize on to the child
@task_file n=5 path=src/bin/viola-fake-agent/input.rs mark=edits
what: the fake agent's resize and input behaviour, including how a typed DEL erases the prompt — edits: plain()
@task_file n=6 path=crates/viola-e2e/src/bin/viola-live-rig.rs mark=edits
what: the standing live rig against the real claude CLI, edited only if a measuring arm is added
@task_file n=7 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the claude capability ledger of relied rows, edited only if the answer becomes a relied row
@task_item n=1 part=needs
The resize path: viola run passes a terminal size change to the child's PTY, and the pump decides what is written to the child after it and when.
@task_item n=2 part=needs
The keys viola types into the CLI's empty input box (the send text, the paste) and the waits or settle bounds placed before and after them.
@task_item n=3 part=needs
The send gate that waits for the input box to be ready before typing, and the send confirmation and read-back that tell whether the typed text arrived.
@task_item n=4 part=needs
The send outcome words a lost key would show as (read back, unable, unconfirmable) and the input-not-ready hint; a refusal fix would add or reuse one of these.
@task_item n=5 part=needs
Time bounds and who waits: any settle wait added after a resize is a fixed, bounded wait; no unbounded block is added to the write path.
@task_item n=6 part=needs
Indirect, through the pump: the paste's single blocking write has no time bound; it is bounded only if this chunk's change opens that write path, and then as a fork in the P3 block.
@task_item n=7 part=needs
What a measurement owes: the process, PTY sizes before and after, the key bytes and their delays after the resize, the CLI state typed into (empty input box, never a dialog), a closed word for kept or lost, repetitions and what a mixed reading means, and the home and config it runs under.
@task_item n=8 part=needs
The fake agent's resize and input behaviour, used for a rehearsal or a fix; its DEL already erases the prompt's last character as the live CLI does.
@task_item n=9 part=needs
The standing live rig, reached only if a measuring arm is added; it runs under a test-owned user home and never reads the operator's real claude config.
@task_item n=10 part=needs
The claude capability ledger: if the answer is a relied fact, it becomes a ledger row stamped with the CLI version it was read on (2.1.328 or 2.1.330).
@task_item n=11 part=needs
No live start in this phase: the installed CLI bundles are read as strings only, and any key typed into a real CLI, the start cap and what verified rests on are asked of the operator in one P3 block.
@task_item n=12 part=needs
Every new test start runs under a test-owned user home, and any waiting test takes a bound under the 20 s lint.
@task_item n=13 part=needs
The intermittent rig rehearsal red after x and DEL stays open: at its next red, the printed rig summary and the kept after-del frame are read and the cause named before any wait is mended; no retry rule, no loosened entry.
@task_item n=14 part=needs
The pinned product release build stays untouched; no mutation run.
@task_standing n=1
convention: every new or changed send outcome or refusal line carries its state in words, puts its hint last on stderr, and the hint names the next verb without quoting text or naming paths
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page.
@task_item n=2 part=not_needed
The viola list board and its JSON: columns, widths, truncation, colour depth, stale rows and the BAY line are not reached by a resize-and-type change.
@task_item n=3 part=not_needed
viola revive: its start preflight, refusals, list rows and JSON error codes are untouched.
@task_item n=4 part=not_needed
viola link and unlink output, and viola plugin install output and refusals.
@task_item n=5 part=not_needed
CLI token rendering rows (colour, spacing, radius, motion tokens, label and data roles) and the board styling decisions.
@task_item n=6 part=not_needed
viola --help and per-verb usage output, and the verb grouping of the CLI.
@task_item n=7 part=not_needed
viola answer output and viola last output.
