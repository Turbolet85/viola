@task_prose part=chunk
Adds `monitors`, a new query, to the Hyprland seam's closed request set: its closed argv builder and its parse, which keeps only what a closed per-request keep-list names; recorded read-only by the test-only tap and replayed by the fake agent standing as hyprctl.
Adds ONE closed move-to-workspace dispatch to the seam: a validated window address and an integer workspace are the only things entering its argv, never an exec; its exact argv, fixture and replay case come from the one live reading R1.
Settles the hyprctl read deadline at 5 s and adds a test that times a real read against it; gives the hyprctl-reply bounded-reader row a witness that drives the read's own stdout drain past the frame cap.
Implement builds and greens everything that needs no live window and stops once, before R1; the seam keeps no product caller, and no verb, window record or terminal spawn enters the product.
@task_file n=1 path=crates/viola-hyprland/src/lib.rs mark=edits
what: the Hyprland seam crate: its closed Request set, its WORDS list and its closed error words — edits: the Request set (a new monitors query and a move-to-workspace dispatch), the WORDS list, the argv builders
@task_file n=2 path=crates/viola-hyprland/src/read.rs mark=edits
what: the seam's bounded hyprctl read and its read deadline — edits: the read deadline constant, the read's stdout drain witness
@task_file n=3 path=crates/viola-hyprland/src/reply.rs mark=edits
what: the seam's reply parses, dropping reply text save a per-request keep-list — edits: a monitors parse and its keep-list, the dispatch reply parse
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs mark=edits
what: the test-only tap that records hyprctl replies into fixtures — edits: a read-only monitors record, the R1 window verb run only at R1
@task_file n=5 path=src/bin/viola-fake-agent/hyprctl.rs mark=edits
what: the fake agent's hyprctl replay, answering by exact argv in recorded, oversize, malformed and absent modes — edits: the monitors replay case, the move-to-workspace replay case
@task_file n=6 path=fixtures/hyprland/0.56.2/Monitors.recorded-1.json
what: a recorded hyprctl fixture of the monitors query for Hyprland 0.56.2
@task_file n=7 path=tests/contract_bounded_readers.rs mark=edits
what: the bounded-readers contract table — edits: the hyprctl reply Boundary row's witness
@task_item n=1 part=needs
The seam's closed request set grows by exactly two: a monitors query and one move-to-workspace dispatch; a request beyond these two is an operator fork, written and stopped on.
@task_item n=2 part=needs
The move-to-workspace dispatch's argv carries only a validated window address and an integer workspace; it is never an exec and takes no free text.
@task_item n=3 part=needs
The monitors parse drops all reply text save what its closed per-request keep-list names.
@task_item n=4 part=needs
The closed error words stay closed: a new closed word is an operator fork, not a change this chunk makes on its own.
@task_item n=5 part=needs
The hyprctl read deadline is settled at 5 s by the operator's own answer, and a new test times a read against it; a time bound beyond 5 s is an operator fork.
@task_item n=6 part=needs
The hyprctl-reply bounded reader gets a witness that drives the read's own stdout drain one past the frame cap and sees it refused as oversize, not only a parse test.
@task_item n=7 part=needs
The test-only tap records the monitors query read-only, opening no window; the R1 window verb opens one foot window with no claude in it, moves it once, ends exactly the pid it started and leaves only fixture files.
@task_item n=8 part=needs
The fake agent standing as hyprctl replays the new monitors and move-to-workspace requests by exact argv in its existing modes: recorded, oversize, malformed, absent.
@task_item n=9 part=needs
New fixtures under the Hyprland version directory follow the fixture envelope the tap writes and the fake replays.
@task_item n=10 part=needs
The seam stays a root dev-dependency with no product caller; no verb, flag, window record, snapshot field or terminal spawn enters the product.
@task_item n=11 part=needs
This phase makes no live start, opens, moves, focuses or closes no window, sends no key, reads none of the operator's real claude home, runs no mutation tier and calls no timer tool; only a read-only compositor query is allowed.
@task_item n=12 part=needs
Implement stops once, before R1; the dispatch's exact argv, its recorded fixture and replay case are left to what R1 gives.
@task_item n=1 part=not_needed
Brand identity rows (flight-progress strips, the readback box, session slots, phraseology): the chunk renders no strip, board or readback; the seam has no product caller.
@task_item n=2 part=not_needed
Colour palette, typography and iconography rows (amber, dim, bold NAME, glyphs, list columns): the chunk prints no styled human output.
@task_item n=3 part=not_needed
The CLI surface's verb-output, exit-code, refusal-line, hint and --json contracts for run, revive, send, list, link, wait, verify, last, pause, release and plugin install: the chunk adds and changes no verb.
@task_item n=4 part=not_needed
Wrapper fault, wrapper socket, peer reply screening and exit 21 cause rows: the chunk touches no wrapper, socket or instance-unreachable path.
@task_item n=5 part=not_needed
The command name register (answer, hook, last, link, list, mcp and the rest): the chunk names no new command; viola open belongs to a later chunk.
@task_item n=6 part=not_needed
Web page and rendered page absences, and the self-validation protocol's squint, swap and readback-box signature checks: they rule over designed output, which the chunk adds none of.
