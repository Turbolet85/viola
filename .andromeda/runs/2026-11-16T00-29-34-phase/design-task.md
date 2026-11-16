@task_prose part=chunk
Documents and pins by tests the wake condition of viola wait: which logged event kinds return a parked wait (CLI viola wait, wait --json, the MCP wait tool) and which park it again, proved over the fake agent.
A turn end while a background task still holds the session wakes wait (the turn-ended event carrying background_tasks, or the question it raises); a task-free turn end's later notification-class, log-only line gives no false wake.
Re-measures from the code and the fake agent whether a turn end with background_tasks 0 while the session still waits on a notification returning the wait is documented behaviour, a defect, or the driver's to re-arm; re-reads today's wake kinds versus log-only kinds.
Takes the carried CI red as its own item: the stamped-home fixture's unbounded viola verify pushes a run-start test past the ci profile's 20 s kill; P3 chooses by measurement between stamping a home once and sharing it, or sizing the class's ci kill to the fixture's loaded time.
@task_file n=1 path=src/run/wait.rs mark=edits
what: the wrapper's wait feed — which logged kinds wake a parked wait and which are log-only — edits: the wake-kind selection and its wake and log-only test cases
@task_file n=2 path=src/cmd/wait.rs mark=edits
what: the viola wait CLI verb and its --json result
@task_file n=3 path=src/human/wait.rs mark=edits
what: the viola wait human output — the waiting line, event form, dialog form and timed-out form
@task_file n=4 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP tool surface — edits: the MCP wait tool
@task_file n=5 path=crates/viola-core/src/lib.rs mark=edits
what: the event kinds catalog — the turn-ended event and its background_tasks field, the question and notification kinds
@task_file n=6 path=crates/viola-agent-claude/src/hook.rs mark=re-reads
what: the claude hook path that produces the turn-end and notification events, re-read at P3 to measure the background_tasks 0 wake
@task_file n=7 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent — edits: its turn scripts for a turn end held by a background task and a task-free turn end followed by a notification
@task_file n=8 path=fixtures/fake-scripts mark=edits
what: the fake agent's scripted turn fixtures
@task_file n=9 path=tests/support/verify.rs mark=edits
what: the stamped-home test support — edits: the stamped_home fixture's viola verify run (within: None)
@task_file n=10 path=.config/nextest.toml mark=edits
what: the nextest test profiles — edits: the ci profile's slow-test kill for the stamped-home test class
@task_item n=1 part=needs
The viola wait output contracts: the waiting line, the event form a wake prints, the dialog form a raised question prints, and the timed-out form — a newly pinned wake on a background-task turn end or its question must print in these forms.
@task_item n=2 part=needs
A turn-ended event never fills the readback box: a wake on turn-ended carrying background_tasks reports the turn end, not a readback.
@task_item n=3 part=needs
The API-error turn end shown as any turn end: a StopFailure turn end wakes wait exactly as any other turn end does, so the documented wake kinds keep it among them.
@task_item n=4 part=needs
The dialog-pending state: a question raised at a turn end held by a background task is the dialog-pending case a wake surfaces, shown as a cocked strip and the arrival amber reserved to it.
@task_item n=5 part=needs
The wait --json result is one stdout document, with data on stdout and messages on stderr; the waiting line and any hint stay off stdout under --json.
@task_item n=6 part=needs
CLI exit codes for viola wait: the exit a wake returns, the exit a timed-out wait returns, and the exit 21 causes when the instance is unreachable — a new wake or re-park must not change them.
@task_item n=7 part=needs
The LLM driver as a CLI caller: the driver's waiter over viola wait --json is the consumer that re-arms, so what the result carries must let it tell a real wake from a re-arm case.
@task_item n=8 part=needs
The MCP wait tool and the mcp hidden verb: the MCP wait wakes on the same kinds as the CLI verb, and the mcp verb's stdout carries only its protocol.
@task_item n=9 part=needs
No success announced before confirmation, and no optimistic confirmation: a wake on a background-task turn end must not read as a finished session while the task still holds it.
@task_item n=10 part=needs
viola wait output names no internals: no stack trace, no path or pid, no hint under --json, and control characters escaped in any human line that carries logged text.
@task_item n=11 part=needs
The viola verify run inside the stamped-home fixture: if the plan stamps a home once and shares or copies it, the verify output's step line, summary and refusals stay as they are; only where and how often the fixture runs it changes.
@task_item n=12 part=needs
No spinner or elapsed-time counter while wait is parked: the waiting line stays the one static progress pattern.
@task_standing n=1
convention: every new or changed human output line is ASCII-only, plain words with no glyph or emoji prefix, colour only as a second cue
home: surface-cli · anti-patterns-never-do-these
@task_standing n=2
convention: every new --json output keeps one document on stdout, messages on stderr, and clean failure documents
home: surface-cli · surface
@task_item n=1 part=not_needed
Strip-bay layout rows — strip density, stable session slots, stale strip dimming, no auto-sorting, pause and release as control transfer: the chunk changes no board layout.
@task_item n=2 part=not_needed
Typography, spacing and iconography rows — bold NAME column, two-space columns, list's six fixed columns, transfer arrows: the chunk adds no table or column.
@task_item n=3 part=not_needed
viola list board and list table contracts — columns, row order, skip counts, NAME cut and width, strict-modes scan: the chunk does not touch the board.
@task_item n=4 part=not_needed
Send readback and send mirror contracts and the not-delivered refusal hints: send confirmation and the readiness-gate turn race are out of this chunk.
@task_item n=5 part=not_needed
viola revive, viola run start refusals and their --json refusal codes: no start or revive path changes.
@task_item n=6 part=not_needed
viola link, unlink, pause, release, answer and plugin install verb output contracts: none of those verbs changes.
@task_item n=7 part=not_needed
Peer reply screening and wrapper fault --json detail rows: the wire between wrapper and peer is not changed.
@task_item n=8 part=not_needed
Web-spa surface and the rendered-page absence: viola renders no page.
@task_item n=9 part=not_needed
Self-validation protocol design tests — squint, swap, domain test: no new visual expression is designed.
@task_item n=10 part=not_needed
viola --help verb groups and the flat CLI verb set: no verb is added or renamed.
