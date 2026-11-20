@task_prose part=chunk
Under viola run, a prompt that a CLI-scheduled wake submits is told apart from a prompt the human typed, so a fired wake leaves the wheel with its holder (capability v2-36).
After a fired wake the driver's next send is confirmed, not refused human-typing, and the next question dialog is held for the driver; a prompt the human types still takes the wheel.
The means is this phase's to find: P3 weighs each candidate (the wrapper's own pty byte source, the hook payload's optional source field, the turn end's scheduled-wake count, a ledger row with a verify probe) and its false positives, with human as the conservative side when it cannot tell.
It changes no ledger stamp, makes no live start in this phase, and carries the open hint-cell reading, the intermittent rig-rehearsal red and the boot-timeout watch, no bound moved.
@task_item n=1 part=needs
The origin decision of a submitted prompt in the wrapper: the prompt-submitted event's origin, today human or driver, and how a CLI-scheduled wake's prompt is filed there instead of as human.
@task_item n=2 part=needs
The byte-source wheel claim: the wrapper's pty pump that reads the human's keys, the closed list of non-editing bytes, and the rule that a human prompt confirms a wheel take; a human-typed prompt must still take the wheel.
@task_item n=3 part=needs
The send path after a fired wake: the driver's next send is confirmed and not refused with the human-typing refusal reason; the refusal set and its order stay as they are.
@task_item n=4 part=needs
The dialog hold: after a fired wake the next question dialog is held for the driver, not rendered at once as for a human-held wheel; the hold bound and the one-pending-dialog rule apply unchanged.
@task_item n=5 part=needs
The human always wins the wheel: where the product cannot tell a wake from a typed prompt it files human, failing toward the human, never toward the driver.
@task_item n=6 part=needs
The wheel is decided in wrapper memory and the wheel classifier only observes; a wheel move after a wake is told is appended as a wheel event in the events log as today.
@task_item n=7 part=needs
The UserPromptSubmit hook payload parse in the Claude agent crate: external payloads parsed tolerantly, an unknown or absent optional field (the schema's source, schedule_wakeup or loop_wakeup) read as unknown, Claude payload shapes kept in that one crate, which does no file I/O.
@task_item n=8 part=needs
The normalised prompt-submitted event in the core crate and the hook-to-kind map entry for UserPromptSubmit: event kinds and any new origin value added only in the core crate, wire enum values kebab-case, absent optional fields omitted, older readers tolerant.
@task_item n=9 part=needs
The turn-ended event's scheduled_wakes count, as a fact the means may rest on (a wake scheduled at the turn end before a prompt arrives).
@task_item n=10 part=needs
The precedent of a prompt that is not a human take: harness prompts classified by raw prefix before unwrapping, and the classify-before-unwrap order of prompt normalisation.
@task_item n=11 part=needs
The capability ledger as the single gate for CLI behaviour: whether a relied row with its verify probe is part of the means, a version verified only when every relied row counts, and what an older or unverified CLI gets (transport-only degrade, partial readiness gate).
@task_item n=12 part=needs
The hook tiers and failure posture: the UserPromptSubmit hook on the sync spine with its deadline, losing the event never the turn, and hooks failing open toward the human.
@task_item n=13 part=needs
run never reads screen content and passthrough never waits on the screen model; no human key is held behind a resize or a wake decision.
@task_item n=14 part=needs
The live rig and the fake agent as the places a wake is rehearsed: fake-agent test seam variables, the rig's own scratch stamp, and real claude probes only locally, never in CI.
@task_item n=15 part=needs
The open hint-cell carry: a third reading of the placeholder hint's cell attribute needs a fresh box with keys typed before any send, reading attributes only, never text; its start and keys are the founder's word.
@task_item n=16 part=needs
The intermittent rig rehearsal red (box read empty after the x and del keys) is acted on only at its next red, from the rig summary and the kept after-del frame; no retry rule and no loosened wait, root test waits keeping one shared bound.
@task_item n=17 part=needs
The boot-timeout watch (a boot that never gets ready times out within its 10 s wall bound) is read at each run under a contended host; no bound is moved.
@task_item n=18 part=needs
Every send confirmed after the fact and a send never queued: telling a wake apart must not let a send through unconfirmed nor queue one behind a wake's turn.
@task_standing n=1
convention: each new test runs in its own isolated viola home, cleaned by its owner
home: established-decisions
@task_standing n=2
convention: each new test runs against the fake agent in CI, with no live claude; test seams are not configuration
home: ci-cd-approach
@task_standing n=3
convention: each new fake-agent test knob carries the fake-agent or harness prefix and is never read by the product
home: conventions
@task_standing n=4
convention: each new recorded fixture is scrubbed and hygiene-walked, keeping only what the rehearsal relies on
home: occupied-resources
@task_standing n=5
convention: each new end-to-end test leaves zero panic lines and keeps coverage floors
home: project-directory-structure · ci-cd-approach
@task_standing n=6
convention: each new log line is a codes-only process log line through the one event emitter, no upstream prompt text in it
home: cross-cutting-patterns
@task_item n=1 part=not_needed
MCP server rows (rmcp, stdin line caps, tool results, list cacheScope): the chunk changes no MCP tool or its output.
@task_item n=2 part=not_needed
Revive, list and board rows (revive refusals and JSON codes, agents listing, liveness, list payload): the chunk touches no revive or board read.
@task_item n=3 part=not_needed
Link and unlink, pause and release verb rows: the wheel's explicit release and the link set stay as they are.
@task_item n=4 part=not_needed
Trust writer code words, plugin install and marketplace rows, and config.json keys: no trust record, plugin or configuration key changes.
@task_item n=5 part=not_needed
Hyprland seam and window feature rows, and the planned fleet console: no compositor or view is reached.
@task_item n=6 part=not_needed
Dependency policy, licence, toolchain, release and deploy rows: no dependency, toolchain or release change.
@task_item n=7 part=not_needed
Socket directory, endpoint naming and peer-credential rows of the channel transport: the channel frames and endpoint are untouched.
