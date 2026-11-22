@task_prose part=chunk
Answers whether real claude on Linux loses a key typed right after a resize, measured; fixes it only if it does.
Reads where the claim comes from, and what viola does around a resize today: who sends the window-size change to the child's PTY, what is written after it and when, every wait on that path.
Reads the installed claude 2.1.328 and 2.1.330 bundles as strings only (a redraw, an input-mode reset, a debounce), a reading, never a measurement.
Prices the cheapest live start that measures it and the fix options (a settle wait after a resize, a read-back and one re-type, a refusal), each with the rows it moves, in ONE block at P3's end; no live start is made and the fix is chosen only after the reading.
@task_file n=1 path=crates/viola-pty/src/lib.rs
what: the PTY seam: the child's PTY, the window-size change sent to it on a resize, and its test module's CHILD_WITHIN bound comment
@task_file n=2 path=crates/viola-pty/src/pump.rs
what: the PTY pump: the bytes it writes to the child's PTY and the paste write path, PasteHandle::paste
@task_file n=3 path=src/run/mod.rs
what: the viola run TUI passthrough pump: the resize path from the outer terminal and the keys written to the child after it
@task_file n=4 path=src/run/wheel.rs
what: the wheel: who holds the child's input, the human or the driver, and what moves it
@task_file n=5 path=src/run/send.rs
what: send: the driver's typed prompt, its gate and its confirmation read-back
@task_file n=6 path=src/bin/viola-fake-agent/input.rs mark=re-reads
what: the fake agent's input box behaviour, plain(): a typed DEL erases the prompt's last character, as the live CLI's input box does
@task_file n=7 path=crates/viola-e2e/src/lib.rs
what: the standing live rig against real claude, only if a measuring arm is added
@task_file n=8 path=crates/viola-agent-claude/src/ledger.rs
what: the claude capability ledger of relied rows, only if the answer becomes a relied row
@task_item n=1 part=needs
The resize path through the TUI passthrough: focus, mouse and resize sequences from the outer terminal reach the child's PTY as a window-size change, and what viola writes to the child after it.
@task_item n=2 part=needs
A resize never moves the wheel: a terminal event is not a human keystroke and does not take the wheel from the driver.
@task_item n=3 part=needs
Human keystrokes after a resize are never blocked, delayed or dropped by viola; any settle wait a fix adds falls on the driver's typed keys only, never on the human's.
@task_item n=4 part=needs
Any settle wait after a resize is a time bound that falls on the driver, never on the human, and stays inside the dialog hold and the driver's own deadlines.
@task_item n=5 part=needs
send's gate and confirmation: a key lost after a resize shows as a send refused with input-not-ready or no-prompt-submitted, its code word beside unable and its one hint, never a silent loss.
@task_item n=6 part=needs
A read-back and one re-type fix reads the input box by a screen signature only, never by parsing claude's content beyond signatures; what tells a lost key from a kept one is a closed word, never a text.
@task_item n=7 part=needs
Viola writes zero bytes of its own to the outer terminal around a resize; vendor content passes through untouched, and run diagnostics stay off the terminal.
@task_item n=8 part=needs
The vt100 feed and screen read after a resize: a resized screen feeds the parser without panic, and after a contained feed panic the passthrough behaves as its contract says.
@task_item n=9 part=needs
A refusal fix, if chosen, is a CLI refusal with an unable line, a kebab-case reason code, a hint line and its exit code, its fixed copy pinned by literal, and a clean --json failure document.
@task_item n=10 part=needs
Live readings of real claude never gate CI: any measuring arm in the live rig is a reading the operator runs, never a CI check.
@task_item n=11 part=needs
The fake agent's resize and input behaviour, if a fix or rehearsal needs it, mirrors the live CLI's input box; its DEL erase is read back as met.
@task_item n=12 part=needs
A waiting test takes a bound under the 20 s lint and starts under a test-owned user home; no mutation run.
@task_item n=13 part=needs
Boundary tests of the resize path are sequenced on records, not on sleeps, with zero flakiness and no retries.
@task_item n=14 part=needs
The paste write's missing time bound is taken only if this chunk's change opens that write path, and then as a fork of the P3 block.
@task_item n=15 part=needs
Linux only: the reading and any fix hold on Linux with portable-pty as the outer-PTY boundary driver.
@task_standing n=1
convention: each new a11y assertion sits inside an existing nextest suite and its failure is a nextest failure, run in the GitHub Actions push/PR gate with no skips
home: bootstrap-phases-derive-for-route-setup-project · 9-ci-integration
@task_standing n=2
convention: each new TUI boundary test asserts at the boundary only, sequenced on records, never inspecting claude content
home: keyboard-test-harness
@task_standing n=3
convention: each new fixed CLI string a fix adds is pinned by its literal in a test and comes from design copy
home: 8-cognitive-accessibility · cognitive-accessibility
@task_item n=1 part=not_needed
The viola list board, its caption, colour, SGR, dim stale rows and NAME truncation: a resize of the wrapped TUI never reaches the board's printed output.
@task_item n=2 part=not_needed
Link and unlink, plugin install, revive --list and revive refusals, hook calls and MCP frame tool refusals: none of these surfaces lies on the resize path.
@task_item n=3 part=not_needed
viola --help, its usage line, group labels and handoff row: the chunk adds no verb and no flag.
@task_item n=4 part=not_needed
Screen-reader targets and manual Orca and keyboard passes: the chunk changes no read-out structure, and manual passes gate nothing.
@task_item n=5 part=not_needed
Web, graphical, touch and internationalization absences, the WCAG mapping, contrast and structured violation JSON: viola has none of these surfaces.
@task_item n=6 part=not_needed
Log identity tags and socket-directory, strict-modes and server-verify refusals: the resize path opens no socket and writes no new log identity.
