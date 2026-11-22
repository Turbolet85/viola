@task_prose part=chunk
Answers whether real claude on Linux loses a key typed right after a resize: reads where the claim came from and whether it is a fact or a carried hypothesis.
Re-reads what viola does around a resize today: who sends the window-size change to the child's PTY, what the run pump writes after it and when, every settle or wait bound on that path, and the tests and fake-agent behaviours that hold it.
Reads the installed claude 2.1.328 and 2.1.330 bundles as strings only (redraw, input-mode reset, debounce) - a reading, never a measurement; makes no live start.
Prices the cheapest live start that would measure it (PTY sizes before and after, key bytes and delays, the empty input box as the state typed into, a closed word telling a lost key from a kept one, repetitions) and the fix options (a settle wait after a resize, a read-back and one re-type, a refusal), each with the rows it moves.
Stops at the end of P3 with one block of operator forks; the fix is chosen only after a reading, and the capability ledger gains a relied row only if the answer becomes one.
@task_file n=1 path=crates/viola-pty/src/lib.rs
what: the PTY seam - the resize that sends the window-size change to the child, and its test module's child bound comment
@task_file n=2 path=crates/viola-pty/src/pump.rs
what: the PTY pump - the writes typed into the child after a resize, and the paste handle's blocking write
@task_file n=3 path=src/run/mod.rs
what: the run pump - the wrapped session's loop that relays the terminal's resize and the keys to the child
@task_file n=4 path=src/run/wheel.rs
what: the wheel - the session's turn state as viola reads it from the screen
@task_file n=5 path=src/run/send.rs
what: send's gate and its confirmation - typing a prompt into the input box and reading it back
@task_file n=6 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's input box behaviour - its prompt, DEL erasing the last character, and its resize handling
@task_file n=7 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger - the relied rows on what the claude CLI does, verified against the installed version
@task_item n=1 part=needs
The resize path of a wrapped session: the window-size change relayed to the child's PTY and the keys written after it, while viola run prints nothing of its own while wrapped.
@task_item n=2 part=needs
send's confirmation after a resize: a key typed into the input box counts as sent only once the readback reads it back; nothing announces success before that confirmation, and a lost key may never be reported as delivered.
@task_item n=3 part=needs
The read-back and one re-type option: the send readback mirror's open, read back, unconfirmable and unable states, and the readback box as the one place a sent prompt is shown.
@task_item n=4 part=needs
The refusal option: a send refused because the input box was not ready after a resize folds into its send line as the fixed unable form with the not-delivered input-not-ready hint; any new refusal word is a closed code word with one hint, human mode only.
@task_item n=5 part=needs
The settle-wait option: a wait after a resize shows no spinner and no elapsed-time counter; the step counter is the one progress pattern, and a time bound is owned by the waiter that sets it.
@task_item n=6 part=needs
The state typed into: the empty input box, never a dialog; a dialog-pending session is shown as a cocked strip and is never the target of a typed key.
@task_item n=7 part=needs
The wheel: the session's turn state read across a resize and its redraw, shown as words in the WHEEL column, a turn-ended never filling the readback box.
@task_item n=8 part=needs
The terminal size relayed to the child comes from the terminal itself, never from COLUMNS; viola's own lines are never wrapped to a hardcoded width.
@task_item n=9 part=needs
The CLI verification state: verified versus unverified-cli, and what verified rests on for a relied row about the resize - the operator's word, asked in the P3 block.
@task_item n=10 part=needs
How a lost key is told from a kept one: by a closed word, never by quoting the typed or screen text; no internals, paths or screen literals in an error, hint or log line.
@task_item n=11 part=needs
Any human-facing or log line the measuring arm or a fix adds is ASCII-only, its control characters escaped, data on stdout and messages on stderr, one --json document where --json is asked.
@task_item n=12 part=needs
The live rig's measuring arm, if added, and the fake agent's resize and input behaviour as the rehearsal of it; a waiting test takes a bound under the 20 s lint and every test start runs under a test-owned user home.
@task_item n=13 part=needs
The handoff between human and driver: a key the human types after a resize while the driver holds the session, and pause and release as the transfer of control, stay as they are.
@task_standing n=1
convention: every new human-facing or log line is ASCII-only text glyphs, control characters escaped, no colour beyond amber and dim, no decorative icons
home: surface-cli · surface
@task_standing n=2
convention: every new refusal is a closed code word with the fixed unable line and one hint in human mode only, never quoting text or naming internals
home: surface
@task_item n=1 part=not_needed
Colour palette, typography, spacing and border radius tokens: the chunk adds no colour, weight or column spacing.
@task_item n=2 part=not_needed
The viola list board and its table: columns, NAME cut and width, BAY line, row order, skip counts and stale dimming.
@task_item n=3 part=not_needed
viola revive output, its refusals and its --json exit-1 codes.
@task_item n=4 part=not_needed
viola link and unlink output and their --json documents.
@task_item n=5 part=not_needed
exit 21 causes, their --json causes and the instance-unreachable hints per cause, and the wrapper fault --json detail codes.
@task_item n=6 part=not_needed
Strict-modes refusals, socket-directory refusals and the home-tree checks of run, revive and verify.
@task_item n=7 part=not_needed
viola --help verb groups, the flat verb set and the hidden mcp verb.
@task_item n=8 part=not_needed
The web surface and the rendered-page absences: viola renders no page.
@task_item n=9 part=not_needed
Brand expression rows: style direction, expression level, rejected presets, strip-board density and the self-validation squint and swap tests.
