@task_prose part=chunk
Builds one standing, committed live rig that later live rounds reuse instead of writing their own evidence scripts: it starts the real claude CLI, or viola wrapping it, in a pty it owns, keeps a terminal screen model and the screens it reads as evidence, and reads the CLI's trust dialog and modal dialogs by their drawn literals and selector rows.
The rig owns its isolation from the founder's real Claude config and login, and its process hygiene: it ends everything it starts on every exit path.
It raises the fake agent's DEL fidelity and its modal fidelity (the trust dialog's literal and selector rows as the live CLI draws them), and gives a live verify --record a record home that holds no path component named home.
It re-measures what each earlier round's evidence rig built for itself and got wrong, and it carries live readings: which whitespace the CLI trims at submit, the hook-latency window before a send's paste, and whether a fired scheduled wake's prompt-submit payload carries a source field, plus a real wake's scheduled_wakes count.
The product binary gains no verb and no surface. Product behaviour changes only where a carried reading forces it and the founder places it.
@task_item n=1 part=needs
The live rig as a test-only harness. It drives the real CLI in a pty it owns and types keys only into that pty. It never types into a session the human or another process owns.
@task_item n=2 part=needs
The rig's terminal screen model, fed from the CLI's pty output. It reads dialogs only by comparing them with compiled drawn literals and selector rows, and it treats screen text as content, never as a command. If it uses the vt100 screen model, it feeds it from a bounded reader under a panic catch.
@task_item n=3 part=needs
Screens the rig keeps as evidence stay local. A kept or recorded screen keeps only signature rows and is scrubbed or refused whole: no host path, username, email, status line or config of the founder's in any record or fixture.
@task_item n=4 part=needs
Isolation the rig owns. It uses a scratch root outside every repository and a private Claude config dir for the child. It writes nothing under the founder's real Claude home and leaves no write to his real ~/.claude.json unexplained.
@task_item n=5 part=needs
The credentials copy the rig hands the child. It is guarded by the token-window read and removed on every exit path. No credential value or token-window value reaches a record, a log line or an error message.
@task_item n=6 part=needs
Token-window gate. The rig refuses a live start when less than 20 minutes of token are left, and the window value it read is counted in no record.
@task_item n=7 part=needs
The child environment the rig builds for the CLI or for viola. It points the child at the private config dir. Inherited CLAUDE credentials and parent identity variables stay stripped as the run path already strips them, and nothing of the founder's config passes through the environment.
@task_item n=8 part=needs
Process hygiene. The rig, and any booted test it stands for, ends the wrapper and CLI processes it starts on every exit path, its own kill deadline included. A census of the processes under the rig's scratch root at exit, or a stronger means, proves it.
@task_item n=9 part=needs
Probe artefacts and scratch directories the rig creates are owner-only and removed by guards on every exit path. Probe prompts it types are synthetic.
@task_item n=10 part=needs
Each isolation property and the hygiene census is proven once by a named check on the fake agent, with no live start.
@task_item n=11 part=needs
Fake agent fidelity. The fake agent draws DEL as the live CLI does, and it draws the trust dialog's literal and selector rows as the live CLI draws them. These are test seams compiled only under the fake-agent feature.
@task_item n=12 part=needs
Record home for a live viola verify --record. It comes from a helper like the tests' record_home() and holds no path component named home.
@task_item n=13 part=needs
Reading the submit trim. The rig reads live, on the interactive input box, which whitespace the CLI trims at submit: leading, trailing, inside a paste. The delivery confirmation's tolerance stays trailing-only (ASCII space, TAB, CR, LF) until this reading lands.
@task_item n=14 part=needs
Reading the hook-latency window: a turn the CLI starts whose UserPromptSubmit has not yet reached the wrapper before a send's bracketed paste. The flight lock closes the race only for a turn the wrapper has learned of.
@task_item n=15 part=needs
Reading a fired scheduled wake. The rig reads whether the wake's prompt-submit payload carries the optional source field (schedule_wakeup, loop_wakeup) and on which CLI versions. The raw payload shape is captured only as closed words; the hook drops it today. The rig also reads whether turn-ended's scheduled_wakes count is 1 for a real wake.
@task_item n=16 part=needs
Live starts only on the founder's machine, never in CI, which holds no secrets and no real claude. Each live start is planned by its shape, its count and its typed keys. The live cap and the keys are the founder's.
@task_item n=17 part=needs
Every step that needs no live start is planned on the fake agent. No mutation run.
@task_item n=18 part=needs
A test that waits on a channel or a thread takes a bound. A test socket goes under the short /tmp root, never at the temp root itself. A stamped home comes from the shared stamp's copy.
@task_item n=19 part=needs
The rig's own errors and refusals print no absolute path, pid, id or upstream text.
@task_standing n=1
convention: Each new test or rig run uses its own home, never the user's real viola or Claude home.
home: secret-management
@task_standing n=2
convention: Each test home or scratch directory a new test creates stays out of git, and test-side removal touches only the directories that test created, following a linked base only where it is safe.
home: security-anti-patterns · security-anti-patterns-never-do-these
@task_standing n=3
convention: Each new test seam lives only under the fake-agent feature and is compiled out of release.
home: security-anti-patterns
@task_standing n=4
convention: Each new recorded fixture or kept screen passes fixture hygiene: scrubbed of host paths, usernames and emails, or refused whole, and reviewed before commit.
home: data-protection
@task_standing n=5
convention: No secret, credential copy or local home is committed to git.
home: security-anti-patterns-never-do-these
@task_standing n=6
convention: Test homes are gitignored.
home: bootstrap-phases
@task_item n=1 part=not_needed
The MCP tool list, MCP tool inputs and MCP error or structuredContent contracts. The rig adds no MCP tool and changes no MCP answer.
@task_item n=2 part=not_needed
The wrapper channel's method, frame-parameter and JSON-RPC error contracts. The channel protocol is unchanged.
@task_item n=3 part=not_needed
viola revive's arguments and its --json codes and documents, and the start collision-check contracts.
@task_item n=4 part=not_needed
config.json keys and the handling of a bad config value. No config key is added.
@task_item n=5 part=not_needed
viola link and unlink: their verbs, exit codes and events link lines.
@task_item n=6 part=not_needed
The rmcp feature set and its ban on HTTP and auth features. The rig adds no MCP transport.
@task_item n=7 part=not_needed
Release attestations, signed self-update and release cache-poisoning findings. No release stage is in scope.
@task_item n=8 part=not_needed
The Hyprland recorder's allowlist and the hyprctl reader's deadline.
@task_item n=9 part=not_needed
Event log growth and rotation.
@task_item n=10 part=not_needed
How API-error turn ends are classed and how their text is dropped. That landed in an earlier chunk, and this one does not change it.
@task_item n=11 part=not_needed
The fuzz workspace: its scope, scheduled fuzzing, nightly pin and lockfile audit.
@task_item n=12 part=not_needed
Instance-unreachable detail causes and CLI exit-code contracts for frame verbs. No verb or refusal word is added.
