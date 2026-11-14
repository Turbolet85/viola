@task_prose part=session
The session added only tests: two unit tests in the Hyprland seam crate pin its two frame-cap bounds at the exact frame.
A hyprctl reply of exactly MAX_FRAME bytes is read, and one byte more is refused as oversize with the detail word oversize.
A dispatch reply of exactly MAX_FRAME bytes passes the size check and reads as refused, and one byte more reads as oversize.
No product line, CLI verb, output line, exit code, refusal word, hint or JSON field changed; no capability was added or retired.
The crate's own test count moved from 82 to 84; CI read green on the pushed commit.
@task_file n=1 path=crates/viola-hyprland/src/read.rs
what: the Hyprland seam's read of a hyprctl reply — the spawned hyprctl call, its bounded drain under the frame cap and its oversize refusal, and its unit tests
@task_file n=2 path=crates/viola-hyprland/src/reply.rs
what: the Hyprland seam's reply parser — the clients and dispatch reply parse under the frame cap, its oversize and refused errors, and its unit tests
@task_item n=1 part=touches
Any row whose cell holds the Hyprland reply frame cap (MAX_FRAME) as a bound or a limit: the session pins that bound at the exact frame, its value unchanged.
@task_item n=2 part=touches
Any row naming the oversize refusal or its detail word oversize for a Hyprland reply: the session tests that word at one byte past the cap, the word unchanged.
@task_item n=3 part=touches
Any row naming the refused outcome of a Hyprland dispatch reply: the session tests that a reply exactly at the cap reads as refused, not as success.
@task_item n=4 part=touches
Indirect: any CLI row whose output rests on the Hyprland seam (a window or focus verb such as viola open, or a refusal it surfaces): the seam's product behaviour is unchanged, only its tests grew.
@task_item n=1 part=untouched
The CLI exit codes, the --json error document and the exit-21 detail words: no code, detail or field moved.
@task_item n=2 part=untouched
The viola list board, the BAY line, its columns, colour depth and truncation: no board line changed.
@task_item n=3 part=untouched
viola send, wait, last, answer and the readback mirror: no output line, word column or hint changed.
@task_item n=4 part=untouched
viola run and viola revive starts, their refusals and their --json codes: untouched.
@task_item n=5 part=untouched
viola mcp tools and their results, including the mcp stdin line frame cap: the session reaches no MCP surface.
@task_item n=6 part=untouched
The CLI token rendering, styling, hints and help output, viola verify output and viola plugin install: untouched.
@task_item n=7 part=untouched
The web-surface absence and the deep-links absence: no capability added or removed.
