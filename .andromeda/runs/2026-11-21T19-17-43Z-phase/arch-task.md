@task_prose part=chunk
Establishes the claude CLI's MCP client tool-call timeout for the versions viola stamps (2.1.328, 2.1.330), as a reading from the installed bundles' strings only: its default, the names that move it, stdio difference, progress reset, and what the client does at the bound.
Bounds in code the longest block of every tool viola mcp serves (send, wait, last, answer, list) below that timeout, send's whole path and wait's caller-supplied wait among them.
Where a tool's block can exceed the client's bound today, closes the gap by a compiled bound, a cap on a caller's argument, or a documented split, each with a test that can fail.
Prices at P3 how the bound stands: a 30th relied capability-ledger row with a viola verify probe, or a compiled bound with a test and a drift reading; the operator decides.
@task_file n=1 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP server's tool handlers for send, wait, last, answer and list — edits: the tool handlers and their deadlines
@task_file n=2 path=crates/viola-mcp/src/lib.rs mark=edits
what: the rmcp stdio MCP server, viola mcp, the only place Tokio runs — edits: the tool deadlines
@task_file n=3 path=crates/viola-mcp/src/result.rs mark=edits
what: the MCP tool results — success, refusal, isError, instance-unreachable and wrapper-fault — edits: the result a tool returns when its deadline is reached
@task_item n=1 part=needs
The MCP tools send, wait, last, answer and list: each tool's longest block bounded in code and sitting below the client's tool-call timeout.
@task_item n=2 part=needs
send's whole path as the MCP send tool shares it with the CLI send: the client-side checks before the first frame (strict modes, socket directory, server verification), the frame, the wrapper's wheel read, typing under the flight lock, the paste-hint wait and the delivery confirmation window; the send time bound must sit below the client's timeout.
@task_item n=3 part=needs
wait's caller-supplied wait: the MCP wait cap and the channel wait's timeout_ms, after and wake kinds; the cap must stay below the client's timeout and a cap on the caller's argument is one way to close a gap.
@task_item n=4 part=needs
answer, last and list blocks: the channel answer and last methods, and list's read of state and of claude agents on each call, each bounded below the client's timeout.
@task_item n=5 part=needs
The CLI's MCP client tool-call timeout as a reading, not a measurement: read from the 2.1.328 and 2.1.330 bundles' strings, the names that move it named only, no live start and nothing of the operator's real claude home read.
@task_item n=6 part=needs
What the client does at its bound (the call's error class; the call alone or the server torn down) and the result viola's MCP tool returns at its own deadline: the MCP tool result shapes, isError text, refusal details from a closed set, and the fault document on MCP.
@task_item n=7 part=needs
A gap closed by a compiled bound, an argument cap or a documented split, each with a test that can fail; a cap refusing an argument answers through the MCP invalid-params error.
@task_item n=8 part=needs
The fork priced at P3: a relied capability-ledger row with a viola verify probe (the 30th row, joining the per-behaviour ledger, the stamps envelope's rows and measured fields, and the rule that a version is verified only when every relied row counts) against a compiled bound with a test and a drift reading.
@task_item n=9 part=needs
If verify gains a probe: verify's bounded child set, its sole stamps writer, its own probe folders removed by drop guard, its transcripts as an accepted residual, and fixtures recorded only from a clean verify and scrubbed; anything verify or a rig would newly read, write or spawn is said whole at P3 with the proof that no text reaches a file, stamp, fixture or log.
@task_item n=10 part=needs
Bounds compiled as constants in the way the confirmation window and readiness gate timings are compiled; no documentation leaf gains a figure the code states.
@task_item n=11 part=needs
Tokio containment: any deadline lives inside the MCP crate or behind viola-channel's Tokio-client feature; sync crates stay tokio-free.
@task_item n=12 part=needs
A deadline must block, never poll: the channel is never polled and wait blocks.
@task_item n=13 part=needs
Time bounds and who waits: the dialog hold bound, the hook connect deadline and the spine deadline as neighbouring waits a tool's block may sit behind, read at P3 for whether they reach an MCP call.
@task_item n=14 part=needs
No live claude CLI in CI and real-CLI probes only locally; the chunk's tests prove the bound against the fake agent, its pump delay seam among the means of a test that can fail.
@task_item n=15 part=needs
The carry on the rig rehearsal that reads the box empty after x and del: acted on only at its next red met during the chunk, by reading the printed rig summary and the kept after-del frame and naming the cause before mending the wait; no retry rule, no loosened entry; not met, re-pinned whole.
@task_item n=16 part=needs
Any new process-log line from the MCP role holds codes only, and error chains reach only the detail file; sanitised error surfaces on MCP results.
@task_standing n=1
convention: each new test runs in its own viola home
home: established-decisions
@task_standing n=2
convention: each new test runs against the fake agent, never a live claude CLI, and stays inside the coverage floors
home: ci-cd-approach
@task_standing n=3
convention: each new fake-agent test seam variable is named in the seam register and is never a configuration channel
home: conventions
@task_standing n=4
convention: each new OS-gated test names its reason
home: cross-cutting-patterns
@task_standing n=5
convention: no print macros or tracing level macros in new code; obs_event! is the only event emitter
home: build-system
@task_standing n=6
convention: root test waits share one bound and a wait report is kept on failure
home: occupied-resources
@task_item n=1 part=not_needed
Hyprland window features and the compositor seam: no tool's bound reaches the window spawn.
@task_item n=2 part=not_needed
Revive, its start refusals, its JSON error codes and the snapshot replay rules: the chunk bounds tool calls, not restarts.
@task_item n=3 part=not_needed
The trust writer's code words and the trust-record ledger row: no trust record is written or read by this change.
@task_item n=4 part=not_needed
The hook-to-kind map and the normalised event kinds: no event kind is added or remapped.
@task_item n=5 part=not_needed
Link and unlink recording, pause and release: CLI-only controls no MCP tool reaches.
@task_item n=6 part=not_needed
The PTY seam, vt100 screen model and screen signatures: the chunk reads no screen and adds no compiled screen literal.
@task_item n=7 part=not_needed
Plugin install, its marketplace tree and the plugin's own MCP config: the server's registration is unchanged.
@task_item n=8 part=not_needed
Pinned copy placement, re-hash and re-pin: the pinned product is untouched and the re-pin step stays outside this chunk.
@task_item n=9 part=not_needed
Release tooling, dist, licences, crates.io publishing and deployment: nothing is released.
@task_item n=10 part=not_needed
The fuzz workspace, its nightly toolchain and weekly fuzzing: no parser changes.
@task_item n=11 part=not_needed
Status-line interception, the usage-budget gate and model API calls: absences the chunk does not touch.
@task_item n=12 part=not_needed
Environment strip of CLAUDE names and the child's argv and environment: no start is made or changed.
