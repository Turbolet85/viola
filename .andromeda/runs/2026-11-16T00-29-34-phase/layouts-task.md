@task_prose part=chunk
Documents and pins by tests the wake condition of wait: which logged kinds return a parked wait (CLI viola wait, wait --json, MCP wait) and which park it again, proved over the fake agent.
A turn end while a background task still holds the session wakes wait: it returns on the turn-ended line carrying background_tasks, or on the question that end raises, never parking past it.
After a task-free turn end, a later log-only notification line gives no false wake to a fresh wait armed after that end; the driver-side instance (background_tasks 0 while the session still waits on a notification) is re-measured at P3.
Re-reads the wake kinds versus the log-only kinds as the wait feed's existing cases state them before the plan states them.
Carry: fixes the CI timeout of the stamped-home run-start test, either by taking the fixture's unbounded viola verify stamping out of the measured time or by sizing the ci profile's kill to it, chosen at P3 by measurement.
@task_file n=1 path=src/run/wait.rs mark=edits
what: the wrapper's wait feed, the component that decides which logged kinds wake a parked wait and which park it again — edits: the wake kinds and log-only kinds, the wake-kind and log-only-kind cases
@task_file n=2 path=src/cmd/wait.rs mark=edits
what: the CLI verb viola wait <target> and its --json form
@task_file n=3 path=src/human/wait.rs mark=edits
what: the human output of viola wait — the waiting line and the turn-ended, dialog and timed-out lines
@task_file n=4 path=crates/viola-mcp/src/tools.rs mark=edits
what: the viola mcp tools — edits: the wait tool
@task_file n=5 path=crates/viola-core/src/lib.rs mark=edits
what: the event kinds — the turn-ended event and its background_tasks field, the notification-class log-only kinds
@task_file n=6 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent, the test double that plays claude's turn scripts — edits: its turn scripts
@task_file n=7 path=fixtures/fake-scripts mark=edits
what: the fake agent's turn scripts, the fixtures a wait scenario plays
@task_file n=8 path=tests/support/verify.rs mark=edits
what: the stamped-home test support — edits: the stamped_home fixture and its unbounded viola verify run
@task_file n=9 path=.config/nextest.toml mark=edits
what: the nextest profiles — edits: the ci profile's kill for the stamped-home test class
@task_item n=1 part=needs
The viola wait <target> CLI verb: which events return it and which keep it parked, as its contract states the wait.
@task_item n=2 part=needs
The viola wait human output for turn-ended: a turn end with a background task still holding the session returns the wait with its turn-ended line, never parking past it.
@task_item n=3 part=needs
The viola wait output when the turn end raises a question: the dialog line returns the wait, and the dialog-unknown and dialog-expired lines stay as they are.
@task_item n=4 part=needs
The viola wait waiting line and timed-out line: a wait armed after a task-free turn end stays parked through a later log-only notification and ends only on a wake kind or its timeout.
@task_item n=5 part=needs
The viola wait --json document: one document per call, carrying the turn-ended outcome with its background_tasks value where a turn end wakes it.
@task_item n=6 part=needs
The MCP wait tool: the same wake condition as the CLI verb, and its success, refusal and invalid-params results shaped as the MCP tool results rule.
@task_item n=7 part=needs
turn-ended is not a send outcome: the wake on a turn end belongs to wait, never to send's readback outcome words.
@task_item n=8 part=needs
The CLI exit codes viola wait returns on a wake, a timeout and an unreachable instance, and the --json form of them.
@task_item n=9 part=needs
Human output is a contract and results go to stdout, the rest to stderr: any wait output line the chunk adds or changes keeps its exact wording and stream.
@task_item n=10 part=needs
The frame verbs' unreachable-instance hints and exit-21 --json detail as viola wait carries them when the wrapper is gone during a parked wait.
@task_item n=11 part=needs
The stamped-home fixture's viola verify run, whose time the CI fix either takes out of the test's measured time or sizes the ci kill to; the viola verify output and summary it produces stay as they are.
@task_item n=12 part=needs
The CLI targets are wrapped ViolaNames: viola wait and the MCP wait tool take the same target forms as today.
@task_item n=13 part=needs
No live claude start: every new wait scenario runs over the fake agent; a clause needing a live reading is asked before the card, under a private CLAUDE_CONFIG_DIR if granted.
@task_standing n=1
convention: every new wait test asserts the exact human output lines and the one --json document, since human output is a contract
home: surface-cli
@task_standing n=2
convention: every new guard test carries its one-off remove-the-guard control, the control's case the test's name alone; no mutation run
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page, and no wait change reaches one.
@task_item n=2 part=not_needed
The viola list board, its columns, BAY line, stale rows, colour depth and truncation: the chunk changes no board rendering.
@task_item n=3 part=not_needed
The viola send output, readback mirror and send refusals: send confirmation and the readiness-gate turn race are not this chunk's.
@task_item n=4 part=not_needed
The viola run and viola revive start outputs, refusals and --json error codes: the chunk changes no start path.
@task_item n=5 part=not_needed
The viola plugin install output and refusals: not reached by a wait or CI-timeout change.
@task_item n=6 part=not_needed
The viola link and unlink output: not reached.
@task_item n=7 part=not_needed
The wheel verb output for pause and release, and the hiding of wheel and handoff verbs from MCP: no wheel or handoff behaviour changes.
@task_item n=8 part=not_needed
The viola answer output: answering a dialog is unchanged; the chunk only decides when wait returns on one.
@task_item n=9 part=not_needed
The viola last output and its MCP last tool: the last-message read is not touched.
@task_item n=10 part=not_needed
Deep links or URL coupling in the CLI: no such surface exists or is added.
