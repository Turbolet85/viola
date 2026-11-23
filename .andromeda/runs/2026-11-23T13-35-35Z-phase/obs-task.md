@task_prose part=chunk
Adds `monitors`, a new read-only query, to the Hyprland seam's closed request set, with its closed argv builder and a parse that drops reply text except what a closed per-request keep-list names; the test-only tap records it with no window and the fake agent replays it.
Adds ONE closed move-to-workspace dispatch that moves a validated window address to an integer workspace, never an exec; its exact argv, recorded fixture and replay case come only from the one live reading R1, before which implement stops once.
Settles the hyprctl read deadline at 5 s and adds a test that times a real read against it, in place of the 200 ms drive alone.
Gives the hyprctl reply bound a witness that drives the read's own stdout drain past MAX_FRAME, in place of the parse test it names today.
Re-reads at P3 the carried figures of the seam (6 requests, WORDS of 6, HyprlandError of 5 closed words, 4 replay modes) where the plan rests on them; claims no capability.
@task_file n=1 path=crates/viola-hyprland/src/lib.rs mark=edits
what: the Hyprland seam crate, the closed set of hyprctl requests with their words, argv builders, parses and closed error words — edits: the Request set, WORDS, the monitors and move-to-workspace argv builders, the monitors parse and its keep-list
@task_file n=2 path=crates/viola-hyprland/src/read.rs mark=edits
what: the seam's bounded hyprctl read, its stdout drain bounded by MAX_FRAME and its read deadline — edits: HYPRCTL_DEADLINE, the deadline test and the drain-past-MAX_FRAME witness
@task_file n=3 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs mark=edits
what: the test-only hyprctl tap that records real compositor replies into versioned fixtures — edits: the monitors record and the R1 window verb that opens, moves and ends its own foot window
@task_file n=4 path=src/bin/viola-fake-agent/hyprctl.rs mark=edits
what: the fake agent standing as hyprctl, replaying recorded fixtures by exact argv in its recorded, oversize, malformed and absent modes — edits: the monitors replay case and the move-to-workspace replay case
@task_file n=5 path=fixtures/hyprland/0.56.2
what: the recorded Hyprland fixtures of version 0.56.2, one per request, that the fake agent replays — a monitors fixture and, at R1, the move-to-workspace fixture join it
@task_file n=6 path=tests/contract_bounded_readers.rs mark=edits
what: the bounded-readers contract, one Boundary row per bounded reader naming its bound and witness test — edits: the hyprctl reply Boundary row's witness
@task_item n=1 part=needs
The new monitors query: a hyprctl JSON reply parsed under the frame bound, whose rejected reply (oversize, malformed, unknown value) surfaces as a parse-rejected line with parser hyprctl-json and a closed detail word, never as reply text.
@task_item n=2 part=needs
A parse failure of the monitors reply reads unknown plus a per-parser counter, the same as every other hyprctl parse; no new parser name and no new detail word enters the diagnostic schema.
@task_item n=3 part=needs
The hyprctl probe child as a spawn subject (hyprctl-probe) on the diag-line subject enum: the new query and dispatch spawn hyprctl through the seam's existing call site and add no subject word.
@task_item n=4 part=needs
The move-to-workspace dispatch argv carries only a validated window address and an integer; no window address, workspace, path or reply body is logged by it, and its failure maps to one of the seam's closed error words with a fixed message.
@task_item n=5 part=needs
The seam's errors are typed with fixed Display text that interpolates nothing; a new request or the drain witness adds no error word beyond the closed five unless written as a fork of the operator's.
@task_item n=6 part=needs
The read deadline: HYPRCTL_DEADLINE settled at 5 s, and a test that times a read against it; the timing is asserted by the test's exit, not by a sleep-based wait or a retry.
@task_item n=7 part=needs
The hyprctl reply bound, MAX_FRAME: its witness drives the read's own stdout drain past the bound and sees it refused as oversize, not a parse given an oversize buffer.
@task_item n=8 part=needs
Recorded hyprctl fixtures are no telemetry source and carry no content beyond the closed keep-list: the monitors fixture and the R1 fixture keep only what the parse keeps, and the replay reads them by exact argv.
@task_item n=9 part=needs
The seam is a dev-dependency with no product caller: OS and compositor calls stay observed at viola's call sites only, and this chunk adds no product-path instrumentation, span or log line over the seam.
@task_item n=10 part=needs
The tap's R1 verb opens one foot window with no claude in it, moves it once and kills only the foot pid it started, writing only fixture files; nothing of the operator's real home, screen content or typed keys is read or logged.
@task_item n=11 part=needs
No retry policy: a timed-out or refused hyprctl read is not retried once, by the seam, the tap or the deadline test.
@task_item n=12 part=needs
No telemetry is drawn from screen content: the monitors and window replies are compositor metadata kept by the keep-list, never logged verbatim.
@task_standing n=1
convention: every new integration test keeps its test home under the one scanned e2e root, kept until the gate steps run
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: every new recorded fixture is synthetic or scrubbed, carries no content or path, and is never a telemetry source
home: pii-scrubbing-compliance · 8-pii-scrubbing-compliance
@task_standing n=3
convention: every new test waits on no sleep and asserts its time bound by its exit code on the maximum sample
home: 10-slo-invariants-telemetry-budgets · slo-invariants-telemetry-budgets
@task_standing n=4
convention: every new log line goes through the obs_event macro with closed event values, no raw tracing or print macro, and no bare instrument attribute
home: 11-obs-anti-patterns-never-do-these
@task_item n=1 part=not_needed
The send, dialog, wait, last and wheel paths — their log fields, span chains, corr by event and refusal detail words: the chunk drives no instance and sends no text.
@task_item n=2 part=not_needed
The run start, revive and verify process logs — their exit-1, exit-21 and verify detail codes and start span attributes: the chunk adds no verb and no start.
@task_item n=3 part=not_needed
The MCP server tools, MCP stdin frame bound and MCP frame-tool refusals: the chunk touches no MCP surface.
@task_item n=4 part=not_needed
The hook roles and their stdout, stderr, fail-open and decision lines: the chunk adds no hook behaviour.
@task_item n=5 part=not_needed
The channel JSON-RPC lines, conn and srv_conn joins and peer-credential checks: the seam speaks to hyprctl, not to the channel socket.
@task_item n=6 part=not_needed
The events log, snapshot replay, heartbeat and liveness rows: the chunk records no window, writes no product state and claims nothing.
@task_item n=7 part=not_needed
The future error reporter, OTel exporters and the absences of network egress, databases and hosted deployment: the chunk adopts none of them.
@task_item n=8 part=not_needed
The config.json keys, diagnostics level and home resolution order: the chunk reads no configuration.
