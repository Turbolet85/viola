@task_prose part=chunk
Builds one standing, committed live rig that every later live round reuses: it starts the real claude CLI (or viola wrapping it) in a pty it owns, keeps a terminal screen model of what the CLI draws, keeps the screens it reads as evidence, and reads the trust dialog and modal dialogs by their drawn literals and selector rows.
The rig owns its isolation from the founder's real Claude config and login (scratch root, private config dir, guarded credentials copy) and its process hygiene (it ends what it starts on every exit path), each proven once on the fake agent with no live start.
Its home (a test-only binary of the e2e crate, a script, or both) and its screen model are settled at P3; the product binary gains no verb and no surface.
Re-measures at P3, from the earlier rounds' evidence scripts and reports, what each earlier rig built, got wrong, and what a standing rig must own.
Carries live readings to take on it: the whitespace the CLI trims at submit, the hook-latency window before a send's paste, and whether a fired scheduled wake's prompt-submit payload carries a source field and makes turn-ended scheduled_wakes read 1; how many fit is decided at the end of P3.
@task_item n=1 part=needs
The rig is test-only tooling: if it lives in the e2e crate it is a test-only binary kept out of release builds and product builds, and the product binary gains no subcommand, channel verb, flag or surface from it.
@task_item n=2 part=needs
The rig's terminal screen model: if it takes the vt100 crate the product already pins for readiness, it uses that pinned version and adds no second screen-model dependency; the product's own readiness gate and screen signatures stay as they are.
@task_item n=3 part=needs
Reading the trust dialog and modal dialogs by drawn literal and selector row: screens kept as evidence keep only what the reading needs, and any screen fixture keeps only signature rows and is scrubbed of host paths, usernames and emails before it is kept.
@task_item n=4 part=needs
viola itself still types into no trust dialog and verify never answers a CLI dialog; any key the rig sends to a dialog is the rig's own, outside viola.
@task_item n=5 part=needs
Isolation owned by the rig: a scratch root outside every repository, a private Claude config dir, a credentials copy removed on every exit path, and no write to the founder's real home Claude config left unexplained.
@task_item n=6 part=needs
A live start is refused under 20 minutes of token left, by a token-window read whose value is counted in no record.
@task_item n=7 part=needs
No host path, nothing of the founder's status line and nothing of his Claude config reaches any record the rig keeps; the global status-line config stays untouched.
@task_item n=8 part=needs
Each isolation property is proven once by a named check on the fake agent with no live start; live claude never runs in CI and real-CLI probes run only locally.
@task_item n=9 part=needs
Process hygiene: the rig, and any booted test it stands for, ends every process it starts on every exit path including its own kill deadline, proven by a census of processes under its own scratch root at exit or a stronger means; test homes removed only by their owner, and test evidence survives a killed test.
@task_item n=10 part=needs
The child's environment as the rig starts it: inherited CLAUDE names stripped by viola's start path as today, the rig's own knobs carrying the harness prefix and never read by viola, and the VIOLA_ namespace left to product plumbing.
@task_item n=11 part=needs
The record-home rule's live half: the home a live viola verify --record takes comes from a helper like the tests' record_home() and holds no path component named home; fixtures are recorded only from a clean verify.
@task_item n=12 part=needs
The whitespace-trim reading on the interactive input box (leading, trailing, inside a paste): the delivery confirmation's tolerance stays trailing-only (ASCII space, TAB, CR, LF) until the reading lands; send text normalisation and the submitted-form prompt match are unchanged by this chunk.
@task_item n=13 part=needs
The hook-latency reading: a turn the CLI starts whose UserPromptSubmit has not reached the wrapper before a send's paste; the send flight lock closes the race only for a turn the wrapper has learned of, and is measured, not changed.
@task_item n=14 part=needs
The scheduled-wake reading: whether a fired wake's prompt-submit payload carries the optional source field (schedule_wakeup, loop_wakeup) on which CLI versions, the raw shape captured as closed words, and whether turn-ended scheduled_wakes reads 1 for a real wake; the hook still drops the field and harness-prompt classification is not widened here.
@task_item n=15 part=needs
The fake agent's DEL fidelity and modal fidelity (the trust dialog's literal and selector rows as the live CLI draws them) ride the rig; any new fake-agent seam variable follows the fake-agent test seam rule and is no configuration channel.
@task_item n=16 part=needs
Every step needing no live start is planned on the fake agent; no mutation run is made in this chunk.
@task_item n=17 part=needs
A test waiting on a channel or thread takes a bound shared with the root test waits; a test socket lives under the short /tmp root outside the product namespace; a stamped home is taken from the shared stamp's copy.
@task_item n=18 part=needs
No product behaviour change is in scope beyond what a carried reading forces and the founder places; the capability ledger rows and their stamps stay as they are unless a reading forces a change.
@task_standing n=1
convention: each new test runs in its own isolated viola home, cleaned by its owner, with its evidence kept after a kill
home: occupied-resources · established-decisions
@task_standing n=2
convention: each new test that drives a channel uses a socket under the short /tmp test root outside the product endpoint namespace, and each wait it makes is bounded
home: occupied-resources
@task_standing n=3
convention: each new Rust test or test binary passes formatting and clippy with warnings denied, uses no print or dbg macro, and logs only through the one event emitter
home: build-system · project-directory-structure
@task_standing n=4
convention: each new OS-gated test names its reason and names no other OS
home: cross-cutting-patterns
@task_standing n=5
convention: each new end-to-end run leaves zero panic lines
home: project-directory-structure
@task_standing n=6
convention: each new test runs against the fake agent in CI; real claude only on the local host
home: ci-cd-approach
@task_item n=1 part=not_needed
MCP server tools, their results and their stdin caps: the rig neither adds nor changes an MCP tool.
@task_item n=2 part=not_needed
Channel protocol frames, fault codes, methods and refusal orders: the rig adds no channel method and changes no frame.
@task_item n=3 part=not_needed
CLI exit codes, --json documents and human error lines of the product verbs: no verb gains or changes an output.
@task_item n=4 part=not_needed
Hyprland window features and the compositor seam: the rig owns its pty and opens no window.
@task_item n=5 part=not_needed
The board, list payload and agents listing: nothing in the rig reads or changes them.
@task_item n=6 part=not_needed
Revive, its refusals and its program resolution: the rig revives no session.
@task_item n=7 part=not_needed
Link and unlink recording, dialog hold and the wheel's move rules: no product behaviour of these is changed.
@task_item n=8 part=not_needed
Fuzz targets, release tooling, licences and deployment model: the rig is test tooling and ships in no release.
@task_item n=9 part=not_needed
Snapshot envelope, instance snapshot fields and config.json keys: the chunk adds no persisted field or config key.
