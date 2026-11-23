@task_prose part=chunk
Adds a monitors query and one move-to-workspace dispatch to the Hyprland seam's closed set of requests, each with its closed argv builder and parse; monitors is recorded read-only by the test-only tap and replayed by the fake agent standing as hyprctl.
Settles the hyprctl read deadline at 5 s with a test that times a read against it, and gives the hyprctl reply drain cap a witness that drives the read's own stdout drain past the frame cap.
Implement builds and greens everything that needs no live window, then stops once before R1, the one live reading that records the move dispatch's exact argv and fixture.
The chunk adds no viola verb, no CLI output, no terminal spawn in the product and no claim; the seam keeps no product caller.
@task_file n=1 path=crates/viola-hyprland/src/lib.rs mark=edits
what: the Hyprland seam's closed request set, its word list and closed error words, and its argv builders — edits: the Request set (monitors query, move-to-workspace dispatch), WORDS, the argv builders
@task_file n=2 path=crates/viola-hyprland/src/reply.rs mark=edits
what: the seam's reply parses with their per-request keep-lists — edits: the monitors parse and its keep-list
@task_file n=3 path=crates/viola-hyprland/src/read.rs mark=edits
what: the seam's bounded hyprctl read, its stdout drain and its read deadline — edits: the read deadline constant, the deadline test
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs mark=edits
what: the test-only tap that records hyprctl replies as fixtures — edits: a monitors record, the R1 window verb
@task_file n=5 path=src/bin/viola-fake-agent/hyprctl.rs mark=edits
what: the fake agent's hyprctl replay by exact argv in recorded, oversize, malformed and absent modes — edits: the monitors replay case, the move dispatch replay case
@task_file n=6 path=fixtures/hyprland/0.56.2/Monitors.recorded-1.json
what: the recorded monitors reply fixture for Hyprland 0.56.2
@task_file n=7 path=tests/contract_bounded_readers.rs mark=edits
what: the bounded-readers contract list — edits: the hyprctl reply Boundary row's witness
@task_item n=1 part=needs
The monitors query: a new closed request whose reply text is dropped at the parse save what its closed keep-list names.
@task_item n=2 part=needs
The move-to-workspace dispatch: one closed dispatch whose argv takes only a validated window address and an integer workspace; never an exec.
@task_item n=3 part=needs
The hyprctl read deadline settled at 5 s, with a test that times a read against it.
@task_item n=4 part=needs
The hyprctl reply drain cap witnessed by a test that drives the read's own stdout drain past the frame cap.
@task_item n=5 part=needs
The test-only tap records monitors read-only with no window; its R1 window verb runs only at R1 and leaves only fixture files.
@task_item n=6 part=needs
The fake agent standing as hyprctl replays the new requests by exact argv in its existing modes.
@task_item n=7 part=needs
The seam's closed sets stay closed: no new error word, no request beyond the two, no time bound beyond 5 s; any such fork is written and the step stops.
@task_item n=1 part=not_needed
The cli surface: every verb, its human and --json output, its refusals, exit codes, hints and token rendering; the chunk adds no verb and no output any viola command prints, and viola open with its flags belongs to a later chunk.
@task_item n=2 part=not_needed
The board, the BAY line and the viola list rows and LINKS block; the chunk records no window and adds no snapshot field.
@task_item n=3 part=not_needed
The mcp tools and their results; the seam keeps no product caller and no tool reaches it.
@task_item n=4 part=not_needed
The web-spa surface; viola renders no page.
@task_item n=5 part=not_needed
The send, wait, answer, revive, run, verify and wheel verb rows; no start, send or wheel action is touched.
