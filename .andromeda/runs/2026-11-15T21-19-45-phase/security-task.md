@task_prose part=chunk
A turn the claude CLI ends on an API error is written to events.ndjson as a turn-ended event carrying a cause code from a closed viola set; the error text reaches no event, snapshot or log line.
After that turn end the instance reads idle, the wheel's turn-running state is cleared as a Stop clears it, a wait on the turn end returns and the driver's next send is accepted, with no human pause and release.
It registers a hook for the CLI signal that ends such a turn (a StopFailure-class hook is the hypothesis P3 re-measures), reads its payload in the Claude agent crate, maps the CLI error string into the closed cause set homed neutrally in the core crate, and teaches the fake agent to emit it.
A test drives the API-error end through the fake agent with recorded or synthetic payloads and fails on the tree before the fix; no live claude start and no mutation run.
P3 re-reads from the code which state reads turn-running while the board reads idle, and re-measures the ending mechanism against the recorded fixtures.
@task_file n=1 path=plugin/hooks/hooks.json mark=edits
what: the plugin's hook registration list, the Claude Code hook contract the plugin installs — edits: the hook registration for the CLI signal that ends a turn on an API error
@task_file n=2 path=src/run/send/slot.rs mark=edits
what: the run pump's send slot, the wheel's turn-running state that gates a driver send — edits: the turn-end path that clears turn-running
@task_file n=3 path=src/run/wait.rs mark=edits
what: the wrapper's wait on a turn end, the wait channel method's wake — edits: the turn-end path a waiter returns on
@task_item n=1 part=needs
A new hook registration in the plugin's hook contract for the API-error turn end: its command runs viola hook through the absolute pinned viola path, and the plugin files carrying it are rewritten at every start.
@task_item n=2 part=needs
viola hook on the new event: the instance strict-modes check comes before any read, stdin is read under the one external read cap with oversize and malformed payloads detected, the hook stays silent and exits zero, and it fails open to the human.
@task_item n=3 part=needs
The hook payload is content and parsed tolerantly: an unknown hook event exits silently, and the payload's error field, a security-relevant field, is read only into a closed enum whose fallback member stands for any value outside the set, never the CLI's string.
@task_item n=4 part=needs
The closed cause set is a closed protocol type with every member named, homed in the core crate with no Claude-specific name, as a shared validator the hook, the wrapper and the readers all call.
@task_item n=5 part=needs
The error text on this path, last_assistant_message and error_details of the API-error payload, is upstream text: it reaches no events.ndjson line, no snapshot, no log line and no stderr, and is never interpolated into an error message (the never-log floor).
@task_item n=6 part=needs
Whether that error text may reach the content-bearing diagnostics tier is left to P3/P4; whatever is decided must hold the rules that full detail and user content go only to owner-only diagnostics files.
@task_item n=7 part=needs
The hook.event channel frame carrying the new turn end: the wrapper re-validates it authoritatively, drops an invalid hook frame, and accepts only cause values of the closed set.
@task_item n=8 part=needs
The turn-ended event's data contract gains a cause key beside last_assistant_message and background_tasks (background tasks still carried as a count only); readers skip and count unknown keys and tolerate torn lines, so mixed binary versions stay readable.
@task_item n=9 part=needs
The wheel's turn-running state is cleared on the API-error turn end exactly as on Stop, so the send method stops refusing not-delivered turn-running and the instance's board state and snapshot agree on idle.
@task_item n=10 part=needs
A wait on the turn end returns on the API-error turn end, through the wrapper's wait channel method and the MCP and CLI wait surfaces that read it.
@task_item n=11 part=needs
The fake agent's ability to emit the API-error turn end is a test seam compiled only under the fake-agent feature and out of the release build.
@task_item n=12 part=needs
The new test runs on the fake agent with recorded or synthetic payloads only: no real claude CLI in CI or in the test, no live start without a cap asked first, and any synthetic payload text is synthetic, never a real session's.
@task_item n=13 part=needs
A probe proves the error text's absence from events, snapshots and log lines, checked against the plan's own test names and fixture keys before the card.
@task_item n=14 part=needs
Each new guard test carries its one-off remove-the-guard control under a kill deadline, its case the test's name alone; no mutation run, local or dispatched, the mutation testing kept to the epoch audit.
@task_item n=15 part=needs
Test sockets go under the short temp root, never a socket at the temp root itself, each test in its own home, and a home given to record through the recording home helper.
@task_item n=16 part=needs
The new code in the Claude agent crate and the core crate keeps those sync crates free of tokio.
@task_item n=17 part=needs
If P3/P4 make the StopFailure dependence a capability-ledger row with a viola verify probe, that probe reads only signatures against compiled literals, answers no dialog, keeps its effects in its own probe directory and removes them.
@task_standing n=1
convention: each new test uses its own viola home and leaves no home or secret in git
home: secret-management
@task_standing n=2
convention: each new test seam lives only under the fake-agent feature and is compiled out of release
home: security-anti-patterns
@task_standing n=3
convention: each new recorded or synthetic fixture is scrubbed of home paths, usernames and emails and reviewed before commit
home: data-protection
@task_standing n=4
convention: each new log line carries no upstream text, no CLAUDE value and no path, its detail a closed code word
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=5
convention: each new error message is a fixed message interpolating nothing
home: error-handling
@task_item n=1 part=not_needed
The rmcp feature set and its banned HTTP and auth features: the chunk changes no MCP server dependency or transport.
@task_item n=2 part=not_needed
viola revive: its arguments, its recorded cwd and its --json codes and sessions document; the chunk does not touch revive.
@task_item n=3 part=not_needed
link and unlink: their channel methods, events.ndjson link lines and exit codes; the chunk adds no link behaviour.
@task_item n=4 part=not_needed
The paste text character set, bracketed-paste typing and delivery confirmation by exact match: send confirmation is out of this chunk, which changes only when a send is gated, not what it types.
@task_item n=5 part=not_needed
The pinned binary and plugin marketplace: placement, re-hash, content hash and the plugin-install live round; the chunk adds a hook registration, not a pinning change.
@task_item n=6 part=not_needed
The child environment strip: inherited CLAUDE credentials, the identity floor names and the process-start env lines; the chunk spawns no new child.
@task_item n=7 part=not_needed
The Hyprland recorder, terminal colour facts and the board's human colour output: untouched by a turn-end change.
@task_item n=8 part=not_needed
config.json keys (claude_env_keep, diagnostics_level, dialog_hold_s) and their bad-value handling: the chunk adds no configuration.
@task_item n=9 part=not_needed
Release, signing, deploy and attestation decisions, and the absences of hosting, remote access, public listeners, outbound network calls and stored secrets: nothing here ships or listens.
@task_item n=10 part=not_needed
The 2.1.325 recorded fixture set and its verify --record recording, and verify's own refusal codes: that recording is out of this chunk.
@task_item n=11 part=not_needed
The driver release guard and the pause and release methods' params and refusals: the fix removes the need for a human pause and release without changing those methods.
