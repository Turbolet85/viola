@task_prose part=chunk
Adds `monitors`, a new query, to the Hyprland seam's closed request set: its closed argv builder and a parse that drops reply text save a closed per-request keep-list; recorded read-only by the test-only hyprctl tap with no window and replayed by the fake agent standing as hyprctl.
Adds ONE closed move-to-workspace dispatch moving a validated window address to an integer workspace, not an exec; its exact argv, recorded fixture and replay case come from the one live reading R1.
Settles the hyprctl read deadline at 5 s (from provisional) with a test that times a read against it, and gives the bounded-readers census hyprctl reply row a witness that drives the read's own stdout drain past MAX_FRAME.
Builds the tap's R1 window verb (one foot window with no claude, moved once, only the pid it started killed, only fixture files left); implement stops once before R1, which is run only at the operator's host.
Claims no capability, adds no verb, no terminal spawn in the product, no window record; the seam keeps no product caller.
@task_file n=1 path=crates/viola-hyprland/src/lib.rs mark=edits
what: the Hyprland seam: its closed Request set, the WORDS list, the HyprlandError closed words and the closed argv builders — edits: the Request set (monitors, move-to-workspace), WORDS, the argv builders
@task_file n=2 path=crates/viola-hyprland/src/reply.rs mark=edits
what: the seam's reply parses, where Hyprland reply text is dropped save a closed keep-list — edits: the monitors parse and its keep-list
@task_file n=3 path=crates/viola-hyprland/src/read.rs mark=edits
what: the hyprctl child read bounded by a deadline and by MAX_FRAME on its stdout drain — edits: HYPRCTL_DEADLINE, the read's deadline and drain tests
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs mark=edits
what: the test-only hyprctl recorder that writes Hyprland fixtures through a closed string allowlist — edits: a monitors record, the R1 window verb
@task_file n=5 path=src/bin/viola-fake-agent/hyprctl.rs mark=edits
what: the fake agent's hyprctl replay by exact argv in its recorded, oversize, malformed and absent modes — edits: the monitors replay case, the move-to-workspace replay case
@task_file n=6 path=fixtures/hyprland/0.56.2/Monitors.recorded-1.json
what: a recorded Hyprland fixture for the monitors query, scrubbed and under the fixture hygiene walk
@task_file n=7 path=tests/contract_bounded_readers.rs mark=edits
what: the bounded-readers census check: every product reader bounded or named, each Boundary row with its witness — edits: the hyprctl reply Boundary row's witness
@task_item n=1 part=needs
The monitors query joins the seam's closed request set and WORDS; its argv is built only from compiled literals, and its parse drops Hyprland reply text save what the closed per-request keep-list names.
@task_item n=2 part=needs
The move-to-workspace dispatch: only a validated window address of closed shape and an integer workspace enter its argv, as separate argv elements with no shell and no exec; upstream text never becomes a command.
@task_item n=3 part=needs
The seam's error set stays closed: HyprlandError keeps its five closed words, its display messages fixed and carrying no upstream text, path or pid; a new closed word, a third request or a reading beyond R1 and R3 is a fork, written and stopped on.
@task_item n=4 part=needs
The hyprctl read deadline decision moves from provisional to settled at 5 s, and a new test times a real read against that deadline rather than a shortened one.
@task_item n=5 part=needs
The hyprctl child's stdout drain stays capped at the one shared MAX_FRAME, and the bounded-readers census row for the hyprctl reply gets a witness that drives the read's own drain past that cap.
@task_item n=6 part=needs
The tap records monitors read-only (a hyprctl -j query, nothing dispatched) with no window; what it writes keeps strings only by the recorder's closed allowlist, scrubbed or refused whole, reviewed before commit.
@task_item n=7 part=needs
The tap's R1 window verb opens one foot window with no claude in it, tells it from the standing windows, moves it once, ends exactly the pid it started and leaves only fixture files; it runs only at R1, never in CI or in this phase.
@task_item n=8 part=needs
The fake agent's hyprctl replay gains the monitors and move-to-workspace cases by exact argv; this test seam stays under the fake-agent feature and compiled out of release.
@task_item n=9 part=needs
The tap and the fake agent stay test-only binaries the release build refuses, and the seam crate stays a root dev-dependency only, with no product caller.
@task_item n=10 part=needs
The phase's own runs: no live start, no window opened, moved, focused or closed, no key sent, nothing read of the operator's real Claude home, no mutation tier and no timer tool; tests run under their own homes.
@task_item n=11 part=needs
Any dependency the seam or tap change pulls in stays inside the root graph's bans: no tokio in the sync crates, no C-building crates, exact pins from crates.io only.
@task_standing n=1
convention: every new recorded fixture is scrubbed or refused whole, held to the closed string allowlist, under the fixture hygiene walk and reviewed before commit
home: data-protection
@task_standing n=2
convention: every new product reader is bounded by MAX_FRAME or a deadline and named in the bounded-readers census with a witness
home: input-validation
@task_standing n=3
convention: every new test seam or test-only binary lives only under the fake-agent feature or test-only targets, compiled out of release
home: security-anti-patterns
@task_standing n=4
convention: every new seam or environment read needs a log entry
home: security-anti-patterns-never-do-these
@task_standing n=5
convention: every new test uses its own home and holds no secret or credential literal
home: secret-management
@task_item n=1 part=not_needed
MCP tool rows, MCP stdin caps and MCP error and structuredContent contracts: the chunk adds and changes no MCP tool.
@task_item n=2 part=not_needed
Wrapper channel methods, channel frames, channel JSON-RPC errors and channel-response log lines: the chunk touches no channel method.
@task_item n=3 part=not_needed
CLI verb, exit-code and --json document contracts, revive and run refusal codes: the chunk adds no verb; viola open and its flags belong to another entry.
@task_item n=4 part=not_needed
Hook stdin, hook payload and hook-decision rows: no hook path changes.
@task_item n=5 part=not_needed
Paste and typed-text character sets, send and answer refusals, PTY screen feed and vt100 rows: no text reaches a pty.
@task_item n=6 part=not_needed
Snapshot, revive, lineage, launch pair and board or list rows: the chunk records no window and adds no snapshot field.
@task_item n=7 part=not_needed
Socket directory, peer euid, server verification and strict-modes refusal contracts: the chunk makes no IPC change.
@task_item n=8 part=not_needed
Pinned binary, plugin install, marketplace and trust record rows: untouched.
@task_item n=9 part=not_needed
Verify's probes, stamps, dialog answers and the live Claude rig's arms: R1 is a tap reading with no claude, not a verify or rig run.
@task_item n=10 part=not_needed
GitHub workflow rows (action SHA pins, workflow permissions, release workflow cache, artifact attestations): the chunk edits no workflow and no release stage.
