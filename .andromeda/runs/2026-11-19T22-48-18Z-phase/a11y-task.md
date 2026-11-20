@task_prose part=chunk
Under viola run, a prompt a CLI-scheduled wake submits is told apart from a prompt the human typed, so a fired wake does not take the wheel.
After a fired wake the driver's next send is confirmed, not refused human-typing, and the next question dialog is held for the driver, not routed to the human.
A prompt the human types still takes the wheel; where the wrapper cannot tell the two apart it keeps the conservative side, origin human.
The means of telling them apart is this phase's to find (the CLI sends no source field on a fired wake); a ledger row with a verify probe may be part of it.
It re-reads the placeholder hint's cell attribute (a third reading), and reads the boot-timeout watch and the intermittent rig rehearsal red at each run, moving no bound.
@task_item n=1 part=needs
The wheel's origin decision for a submitted prompt: who holds the wheel after the UserPromptSubmit hook payload and the human's keys are read, with a fired scheduled wake leaving the wheel with its holder.
@task_item n=2 part=needs
A human key takes the wheel: a prompt the human typed must still move the wheel to the human; the new means may never turn a human-typed prompt into a wake (the false positive).
@task_item n=3 part=needs
Send refused while the human types: the driver's send path after a fired wake must be confirmed, not refused human-typing, and still refused while the human truly types.
@task_item n=4 part=needs
The dialog hold bounds the driver: after a fired wake, the next question dialog is held for the driver for its dialog hold, not routed to the human.
@task_item n=5 part=needs
Critical paths the chunk changes: the human takes the wheel, the confirmed send and readback, and dialog pending then answer, each now with a fired scheduled wake before it.
@task_item n=6 part=needs
The human keystrokes the wrapper's pty pump reads: if the means rests on knowing no byte came from the human's side before a prompt was submitted, the closed non-editing stdin list decides which bytes count as human editing.
@task_item n=7 part=needs
Focus, mouse and resize sequences never move the wheel, and terminal sequences are not human editing: the means must not count them as human acts, nor let them mark a prompt as human-typed.
@task_item n=8 part=needs
Human keystrokes are never blocked: telling a wake apart may delay or filter nothing the human types on its way to the child.
@task_item n=9 part=needs
Zero viola bytes on the terminal and run diagnostics never reach the terminal: any new wake or origin reading the wrapper logs goes off the terminal.
@task_item n=10 part=needs
The boundary check never inspects claude content and no screen content is parsed beyond signatures: the means may not read the prompt's text or the screen to tell a wake from a typed prompt.
@task_item n=11 part=needs
The normalised event for a submitted prompt and the turn end's scheduled_wakes count, as values the origin decision may read; an older or unverified CLI falls to the conservative origin human.
@task_item n=12 part=needs
The hook payload parse for UserPromptSubmit in the claude agent, and the capability ledger if a relied row with a verify probe is part of the means.
@task_item n=13 part=needs
Live readings never gate: the hint cell attribute reading and any live wake rehearsal on the live rig are evidence, not a gate; no live start is made in this phase.
@task_item n=14 part=needs
The wrapped TUI reads as unwrapped: a fired wake's prompt and the child's screen pass through the wrapper exactly as without it.
@task_standing n=1
convention: each new test is an assertion inside an existing suite, run in the coverage suite, with no a11y suite or driver of its own
home: 1-a11y-scope-summary · a11y-scope-summary
@task_standing n=2
convention: each new boundary test is sequenced on records, never on sleeps or screen timing
home: keyboard-test-harness
@task_standing n=3
convention: each new test is held to zero flakiness with no retries, and a failing a11y assertion is a nextest failure that fails the build
home: 10-slo-invariants-a11y-budgets
@task_standing n=4
convention: the CI test gate allows no skips for any new test, and a live reading stays out of it
home: 1-a11y-scope-summary
@task_item n=1 part=not_needed
The viola list board, its caption, colour, stale-row dim, NAME truncation and the unwrapped separator: the chunk prints no board.
@task_item n=2 part=not_needed
Colour and SGR rules of visual design verification: the chunk adds no coloured output.
@task_item n=3 part=not_needed
viola revive, its --list output and its --json refusal documents: the chunk does not touch revive.
@task_item n=4 part=not_needed
viola link and unlink, viola plugin install and viola --help output: none is changed by a wake's origin.
@task_item n=5 part=not_needed
viola verify interrupted lines and its step line: the chunk changes no verify output.
@task_item n=6 part=not_needed
Screen-reader targets and manual Orca setup for the cli and tui: the chunk changes nothing a screen reader reads.
@task_item n=7 part=not_needed
Absences of internationalization, a touch surface, a web or graphical frontend and a rendered page: the chunk adds none.
