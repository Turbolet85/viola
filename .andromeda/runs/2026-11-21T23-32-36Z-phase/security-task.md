@task_prose part=chunk
Answers whether the real claude CLI on Linux loses a key typed right after a resize: first states what can be told with no live start — where the claim came from, what viola does around a resize today, and what the installed 2.1.328 and 2.1.330 bundles do on a resize as read from their strings only.
Defines what an honest measurement is (PTY sizes, key bytes and their delays, the empty input box, a closed word telling lost from kept, repetitions) and prices the cheapest live start in ONE block that waits for the operator at the end of P3; no live start in this phase.
Prices the fix options before any start (a settle wait after a resize, a read-back and one re-type, a refusal), each with the rows it moves; the choice is made only after the reading.
Re-reads the open carries: the fake agent's DEL handling (reads met), the CHILD_WITHIN test comment, the paste's unbounded write, and the intermittent rehearsal red.
@task_file n=1 path=crates/viola-pty/src/lib.rs mark=edits
what: the PTY seam: the resize sent to the child's terminal, and the test module's child bound comment — edits: the resize path, the test-module comment over CHILD_WITHIN (reworded only if this chunk edits the PTY seam)
@task_file n=2 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY pump: the bytes written to the child after a resize and the paste handle's typed text — edits: the pump's writes after a resize, PasteHandle::paste and its blocking write (only if the change opens that write path)
@task_file n=3 path=src/run/ mark=edits
what: the run pump and the wheel: the resize path and what viola types after it — edits: the resize handling, the wait or settle bound before typing
@task_file n=4 path=src/bin/viola-fake-agent/ mark=edits
what: the fake agent test double: its resize and input behaviour, the prompt's DEL handling — edits: the resize and input behaviour, if a fix or a rehearsal needs it
@task_file n=5 path=crates/viola-e2e mark=edits
what: the standing live rig against the real claude CLI and its arms — edits: a measuring arm, only if one is added
@task_file n=6 path=crates/viola-agent-claude mark=edits
what: the claude adapter's capability ledger of relied rows — edits: a relied row, only if the answer becomes one
@task_item n=1 part=needs
The resize path: who sends the window-size change to the child's PTY, what is written to the PTY after it and when, and every wait or settle bound on that path — read and stated before any start.
@task_item n=2 part=needs
Typed text after a resize: send stays one bracketed paste, and any fix (a settle wait, a read-back and one re-type, a refusal) keeps that shape, refuses rather than strips, and never types a byte twice without a read-back saying the first was lost.
@task_item n=3 part=needs
Delivery confirmation: a read-back that decides a key was lost reads the screen only against compiled literals and signatures, with the trailing whitespace tolerance, and tells lost from kept by a closed word, never by echoed text; no compiled screen literal is printed in chat.
@task_item n=4 part=needs
Input box readiness: the key is typed only into the empty input box, read from compiled literals — never into a dialog, and nothing answers a dialog.
@task_item n=5 part=needs
The wheel: a human typing takes the wheel and send refuses with human-typing; a settle wait or re-type after a resize must not type over the human nor bypass that refusal.
@task_item n=6 part=needs
Time bounds and who waits: any settle wait added after a resize is a compiled bound, and an MCP send that crosses it stays inside the compiled call deadline whose expiry is the call-deadline cause.
@task_item n=7 part=needs
The real claude CLI runs only locally and only on the operator's word: this phase makes no live start, reads nothing of the operator's real Claude home or config, writes nothing under it, and reads the installed bundles as strings only.
@task_item n=8 part=needs
A measuring live-rig arm, if added: it keys only into its own PTY, runs its child isolated from the user's config under its own home with a transient credentials copy, and keeps only closed words and counts, no CLI output or screen text.
@task_item n=9 part=needs
The screen read for a read-back goes through the bounded screen feed, isolated from passthrough and under its panic catch.
@task_item n=10 part=needs
The fake agent's resize and input behaviour: any new behaviour or delay is a test seam behind the fake-agent feature, compiled out of the release build.
@task_item n=11 part=needs
Carry: the fake agent's prompt erases on DEL as the live CLI does — read as already met; P3 names which chunk met it.
@task_item n=12 part=needs
Carry: the paste's missing time bound (one blocking write in the paste handle) is taken only if this change opens that write path, and then as a fork of the P3 block; otherwise re-pinned whole.
@task_item n=13 part=needs
Carry: the intermittent rehearsal red reading the box empty after x and DEL is acted on only at its next red met here — read the rig summary and the kept after-del frame, name the cause, then mend the wait; no retry rule, no loosened entry.
@task_item n=14 part=needs
A relied ledger row, only if the answer becomes one: it rests on a measured reading, its OS and CLI version named, never on a bundle string reading.
@task_item n=15 part=needs
The pinned product (release build of a59da9da) stays untouched; the re-pin step is outside this chunk.
@task_item n=16 part=needs
No mutation run; a waiting test takes a bound under the 20 s lint; every test start runs under a test-owned user home.
@task_item n=17 part=needs
The P3 block: ONE block naming every reached master row (resize path, keys and their waits, send's gate and confirmation, time bounds, the live rig, the wheel), the start cap, any widening and what «verified» rests on — the operator's word, the phase holding there.
@task_standing n=1
convention: each new test runs under its own test-created home, never the user's
home: secret-management
@task_standing n=2
convention: each new fake-agent behaviour or delay seam lives only under the fake-agent feature and is compiled out of release
home: security-anti-patterns
@task_standing n=3
convention: each new live-rig arm keeps closed words and counts only, no CLI output, screen text or transcript byte
home: data-protection
@task_standing n=4
convention: each new recorded fixture is scrubbed, kept to signature rows, and reviewed before commit
home: data-protection
@task_standing n=5
convention: each new seam or environment read gets its log entry
home: security-anti-patterns-never-do-these
@task_item n=1 part=not_needed
The channel socket's peer checks: peer euid, server verification before a frame, the socket directory and its modes — the chunk opens no channel and changes no IPC.
@task_item n=2 part=not_needed
Strict-modes refusals and their per-verb contracts: no new state read or verb is added.
@task_item n=3 part=not_needed
revive, link and unlink, list and the snapshot: their arguments, JSON codes and refusal words are untouched.
@task_item n=4 part=not_needed
The hook contract: hook stdin, hook-decision details and the hook's silent exit — the resize path reaches no hook.
@task_item n=5 part=not_needed
verify's stamps, probes and refusal codes, and the unclean-fixture refusal codes: verify is not run nor changed.
@task_item n=6 part=not_needed
Plugin install, the marketplace tree and pinned-copy placement and re-hash: the product pin is untouched.
@task_item n=7 part=not_needed
MCP stdin line caps, MCP tool input schemas and rmcp feature bans: no MCP tool or input is added or changed.
@task_item n=8 part=not_needed
Event log rotation, config.json keys and the Hyprland recorder: nothing here reads or writes them.
@task_item n=9 part=not_needed
Release, signing, attestation and deploy rows: there is no release stage in this chunk.
