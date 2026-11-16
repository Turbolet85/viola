@task_prose part=chunk
A key the human presses to answer or dismiss the claude CLI's session-feedback survey edits no prompt, so it leaves the wheel with its holder; every other key keeps taking the wheel.
The survey's grammar (which keys answer or dismiss it, and the screen state in which they do) is measured live on the installed CLI before it joins the closed non-editing list; a prompt shape not measured still takes the wheel.
The measured grammar becomes a capability-ledger row with a compiled screen signature and a viola verify probe; the fake agent learns to draw a survey screen for CI.
P3 re-measures the closed list, the wheel classification and what is known of the survey; live claude starts are named at P3's end and wait on the founder's word.
@task_file n=1 path=src/run/wheel.rs mark=edits
what: the wheel classifier, deciding whether a stdin read from the human holds an editing key and so takes the wheel — edits: the Classifier / Observed path and the closed non-editing list
@task_file n=2 path=crates/viola-agent-claude mark=edits
what: the claude agent's capability ledger and its compiled screen signatures, and the viola verify probes over them — edits: a new ledger row for the survey grammar, its screen signature, its verify probe
@task_file n=3 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent CI drives in place of claude — edits: a survey screen it can draw
@task_file n=4 path=fixtures/claude/<version>/ mark=edits
what: recorded claude screen payloads per CLI version — edits: a recorded survey screen
@task_item n=1 part=needs
The wheel: which holder has it, and the rule that a human key which edits a prompt takes it while a key that edits none leaves it with its holder; a survey answer or dismiss key now leaves it.
@task_item n=2 part=needs
The closed non-editing list: it grows by the measured survey shape only; focus reports, mouse reports, terminal replies and resizes stay as they are, and no env var, flag or config key widens it.
@task_item n=3 part=needs
The survey screen state as a state the wheel decision reads: a survey key is non-editing only while the survey is on screen; outside it the same key is typing and takes the wheel.
@task_item n=4 part=needs
The wheel as shown to the human and to a driver: the WHEEL field of the flight-progress strip and the wheel words keep showing the holder unchanged after a survey key.
@task_item n=5 part=needs
Pause and release as the only control transfers: a survey key transfers no control and shows no transfer marker.
@task_item n=6 part=needs
The human-typing refusal a driver send meets: after a survey key the builder's next send is no longer refused as human typing, while any editing key still refuses it.
@task_item n=7 part=needs
The new ledger row's probe as a viola verify step: its step line, fail word and summary follow the viola verify output forms, and a claude version whose survey shape fails the probe leaves the CLI unverified.
@task_item n=8 part=needs
CLI verification state: verified versus unverified-cli as read from the ledger probes, and its refusal hint where the survey row is not yet verified.
@task_item n=9 part=needs
Mechanism only: viola never answers the survey and never types a survey key itself; the human's key reaches the child unchanged, the classifier only observes.
@task_item n=10 part=needs
viola run stays silent while wrapped: the classification change prints nothing to the human's terminal.
@task_item n=11 part=needs
No optimistic confirmation: nothing reads the survey key as delivered or the wheel as kept before the measured screen state confirms it.
@task_item n=12 part=needs
The fake agent's survey screen and any recorded survey fixture: drawn to the measured shape so CI proves the classification without a live claude start.
@task_item n=13 part=needs
Live claude starts capped per chunk: their count, readings, how the survey is made to appear and any key a rig types are named at P3's end and wait on the founder's word; every other step runs on the fake agent and recorded payloads.
@task_standing n=1
convention: every new viola verify probe prints its step in the fixed step-line form, ASCII only, with the fixed fail word and counted in the summary line
home: surface-cli · surface
@task_standing n=2
convention: every human-facing line a new verify step or refusal adds is ASCII only, control characters escaped, with no internals, no glyph prefixes and no spinner
home: anti-patterns-never-do-these
@task_item n=1 part=not_needed
Colour palette, typography, spacing, border-radius and depth rows: the chunk adds no colour, style or layout; the wheel's display is unchanged.
@task_item n=2 part=not_needed
Web surface and rendered-page absences: viola has no web page and the chunk adds none.
@task_item n=3 part=not_needed
Send readback contracts for prompt, /clear, /compact and /mcp, and the readback box: how a send is confirmed is not changed.
@task_item n=4 part=not_needed
viola list board and table contracts (columns, NAME cut, skip counts, row order, row escaping): the board's layout is untouched.
@task_item n=5 part=not_needed
viola revive and viola run start refusals and their --json exit-1 codes: start and revive paths are not reached.
@task_item n=6 part=not_needed
viola link and unlink output and their --json documents: linking is not reached.
@task_item n=7 part=not_needed
Exit 21 causes, strict-modes refusals, socket-directory and home-tree refusals, wrapper fault details and peer reply screening: the instance endpoint and its faults are unchanged.
@task_item n=8 part=not_needed
The hook and mcp verb contracts and the help verb groups: no verb is added or changed.
@task_item n=9 part=not_needed
viola wait and viola last output forms: event and dialog reporting is unchanged.
@task_item n=10 part=not_needed
Command names: the chunk adds no command.
