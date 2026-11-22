@task_prose part=chunk
Puts a compiled time bound on the wrapper's paste write into the child's PTY, so the send never waits without an end on a child that stops reading.
At the bound the send ends with a closed word the driver reads on the CLI send and the MCP send tool, never upstream text, and the wheel is not moved.
First measures what the code does today with a fake agent that stops reading its PTY: whether the paste blocks the send past the readiness gate's and the confirm window's bounds.
Shows the bound's value against the send's own compiled total, the gate and confirm-window bounds and the MCP call deadline, so the paste bound plus the rest of the send stays under that deadline.
Disposes what a cut paste leaves in the child's input box and what the next send, the human's keys and the confirmation meet, while the paste is blocked and after it is cut.
@task_file n=1 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY seam's pump, its write contract into the child's PTY and the one blocking paste write — edits: the paste write, given a compiled time bound
@task_file n=2 path=crates/viola-pty/src/lib.rs mark=edits
what: the PTY seam crate root and its test module — edits: the test-module comment over the child-wait bound naming the mutation profile's kill
@task_item n=1 part=needs
The send time bound and its compiled total: the paste bound is a new part of that total and the send must end inside it.
@task_item n=2 part=needs
The atomic bracketed-paste send: the paste that can now be cut part-way, and what text without Enter it leaves in the child's input box.
@task_item n=3 part=needs
The MCP call deadline of 55 s and its expiry word: the paste bound plus the rest of the send must stay below it, the arithmetic shown.
@task_item n=4 part=needs
The readiness gate timings and the compiled confirmation window: the bounds the paste bound is read against and measured past.
@task_item n=5 part=needs
The send result when a paste fails or is cut after send-issued: the closed word the driver reads, the send never presumed delivered, an unconfirmed delivery reported as not delivered.
@task_item n=6 part=needs
The closed refusal reason, refusal detail and cause sets in the core crate, where a new closed word joins if one is added, kebab-case on the wire.
@task_item n=7 part=needs
The MCP send tool's result and the CLI send verb's exit code and json document at the bound: a refusal or fault from the closed sets, never upstream text.
@task_item n=8 part=needs
The wheel: a cut paste must not move it; the byte-source claim and the human-always-wins rule hold while the paste is blocked.
@task_item n=9 part=needs
The send flight lock: held over the paste, so its release at the bound decides what the next send meets.
@task_item n=10 part=needs
What a human key may be held behind: a human key is never held behind a blocked paste, and passthrough never waits on the screen model.
@task_item n=11 part=needs
The PTY pump's contract and the PTY seam boundary: the seam knows no agent and depends on no viola crate, so the bound and its expiry stay agent-free.
@task_item n=12 part=needs
Time bounds and who waits: every boundary read and write is bounded; nothing polls, a wait blocks to a compiled end.
@task_item n=13 part=needs
The fake agent's new behaviour that stops reading its PTY, reached through a fake-agent test seam variable, the measuring child for this chunk.
@task_item n=14 part=needs
No live start and no real claude in this chunk or in CI: the measure and the tests run against the fake agent alone.
@task_item n=15 part=needs
The send events appended by the wrapper on this path, send-issued and send-refused, and their order against a cut paste.
@task_item n=16 part=needs
Upstream text is untrusted content: nothing the child printed reaches the word the driver reads at the bound.
@task_standing n=1
convention: each new test starts under its own test-owned viola home and user home, cleaned by its owner
home: occupied-resources
@task_standing n=2
convention: each new test wait takes a compiled bound under the 20 s lint and shares the one root test bound
home: occupied-resources
@task_standing n=3
convention: each new fake-agent behaviour is a named test seam variable, never a configuration channel, kept out of product builds
home: conventions · occupied-resources
@task_standing n=4
convention: each new test runs against the fake agent on Linux, never a real claude
home: ci-cd-approach
@task_standing n=5
convention: each new process log line holds codes only, emitted through the one event emitter, no upstream text
home: cross-cutting-patterns · build-system
@task_standing n=6
convention: each new error variant joins its crate's one hand-written or thiserror enum, its message interpolating nothing
home: established-decisions · cross-cutting-patterns
@task_item n=1 part=not_needed
Hooks, hook tiers, dialog holds and dialog answers: the paste path writes into the PTY and answers no dialog.
@task_item n=2 part=not_needed
Revive, start refusals, the pinned copy and the child's argv and environment: the chunk starts no child the product way.
@task_item n=3 part=not_needed
The board, list payload, agents listing and session liveness: no listing reads the paste bound.
@task_item n=4 part=not_needed
Link and unlink, cross-session addressing and their events: the bound sits on one wrapper's own paste.
@task_item n=5 part=not_needed
Hyprland, window features and the terminal fleet console: no window or compositor is touched.
@task_item n=6 part=not_needed
Plugin install, the marketplace tree and the embedded plugin: nothing in the plugin changes.
@task_item n=7 part=not_needed
Snapshot envelopes, instance snapshot fields and config.json keys: the bound is compiled, never configured or persisted.
@task_item n=8 part=not_needed
Trust writer code words, ledger stamps and verify's refusal codes: no ledger row or stamp is written by this chunk.
@task_item n=9 part=not_needed
Release tooling, dist, deployment and licensing: no release or install path changes.
@task_item n=10 part=not_needed
Fuzz workspace and weekly fuzzing: no parser or frame boundary changes.
