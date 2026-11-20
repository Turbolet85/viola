@task_prose part=chunk
Under viola run, a prompt that a CLI-scheduled wake submits is told apart from a prompt the human typed (capability v2-36).
A fired wake leaves the wheel with its holder: the driver's next send is confirmed, not refused human-typing, and the next question dialog is held for the driver.
A prompt the human types still takes the wheel; where the wrapper cannot tell, the conservative side is human.
The means is this phase's to find (the CLI sends no source field on a fired wake's prompt-submit payload); the hypothesis is the wrapper's own pty knowing no byte came from the human while a wake was scheduled.
It re-reads the boot-timeout watch at each run, and the placeholder hint's cell attribute stays a carried third reading on the founder's word.
@task_item n=1 part=needs
The wheel state and who holds it: a fired scheduled wake no longer moves the wheel to human, while a human-typed prompt still does.
@task_item n=2 part=needs
How the wheel is shown in words on the flight-progress strip's WHEEL field and in the board's WHEEL column, for a session whose wake fired while the driver held it.
@task_item n=3 part=needs
The human-typing refusal and its hint: after a fired wake the driver's next send is no longer refused human-typing; after a human-typed prompt it still is.
@task_item n=4 part=needs
The send readback for a prompt send after a fired wake: confirmed only on the CLI's own confirmation, never announced as success before it, with the unconfirmable and unable forms unchanged.
@task_item n=5 part=needs
The dialog-pending state: the next question dialog after a fired wake is held for the driver, not routed to the human, and how a held dialog is shown and reported to viola wait in its dialog form.
@task_item n=6 part=needs
Pause and release as control transfer, and the handoff transfer marker: a fired wake is no transfer of control and draws no transfer marker; a human-typed prompt still is one.
@task_item n=7 part=needs
viola run prints nothing while wrapped: any trace of a told wake goes to the log, never to the human's terminal.
@task_item n=8 part=needs
The CLI verification state: what an older or unverified CLI gets when the wake cannot be told (the conservative human side), and whether a relied capability-ledger row with a verify probe backs the means, read in viola verify's step and summary lines.
@task_item n=9 part=needs
A submitted prompt's origin as carried in the normalised prompt-submit event and the turn end's scheduled-wake count, as values the wheel's origin decision reads.
@task_item n=10 part=needs
Any new log line or detail code for a told wake stays ASCII-only, escapes control characters and names no internals.
@task_item n=11 part=needs
The hint rules: a hint never quotes text, never names viola release, and hints show in human mode only, for any refusal whose cause a fired wake changes.
@task_standing n=1
convention: every new human-facing or log line is ASCII-only, words not glyphs, with control characters escaped
home: surface-cli
@task_standing n=2
convention: data on stdout and messages on stderr; no colour, spinner or decoration added to any new output
home: anti-patterns-never-do-these
@task_item n=1 part=not_needed
Colour palette, typography, spacing, border-radius and depth tokens: the chunk adds no colour, type or spacing choice.
@task_item n=2 part=not_needed
Any web page or rendered-page surface: viola renders no page.
@task_item n=3 part=not_needed
viola revive refusals, its --json exit-1 codes and its output: the chunk touches no revive path.
@task_item n=4 part=not_needed
viola link and unlink output and their --json documents: no pairing changes.
@task_item n=5 part=not_needed
Exit-21 causes, strict-modes refusals, socket-directory and home-tree refusals: the chunk changes no start or instance-reach check.
@task_item n=6 part=not_needed
viola list board's NAME column width, truncation, row order, skip counts and unwrapped rows: only how the wheel reads changes, not the board's layout.
@task_item n=7 part=not_needed
viola --help verb groups, the hidden mcp verb and the hook verb's exit code: no verb is added or regrouped.
@task_item n=8 part=not_needed
Send readbacks for /clear, /compact, /context and /mcp: no local-command readback changes.
@task_item n=9 part=not_needed
Wrapper fault --json detail codes and peer reply screening: no wire fault or peer reply form changes.
