@task_prose part=chunk
Pins by tests, in the viola-hyprland seam, the exact-frame case on both MAX_FRAME bounds the Epoch 1 code audit left uncaught: a reply of exactly MAX_FRAME bytes is accepted, one byte more is oversize.
The bounded read of a hyprctl reply is driven through the fake hyprctl with an exactly-MAX_FRAME reply (not oversize) and a MAX_FRAME + 1 reply (oversize).
The dispatch parse is given exactly MAX_FRAME bytes (not oversize; Refused, as they are not the success text) and MAX_FRAME + 1 bytes (oversize).
Each new guard test carries a one-off remove-the-guard control that mutates its site (> to >=) and must see the test fail; test-only, no product line changes unless a test proves one wrong.
The clients parse's bound is not re-pinned: its existing exact-frame test is read back as already holding it.
@task_file n=1 path=crates/viola-hyprland/src/read.rs mark=edits
what: the Hyprland seam's bounded read of a hyprctl reply, refusing a reply over MAX_FRAME as oversize — edits: its test module (a new exact-frame test of read_within through the fake hyprctl)
@task_file n=2 path=crates/viola-hyprland/src/reply.rs mark=edits
what: the Hyprland seam's reply parsing, the dispatch parse and the clients parse each bounded by MAX_FRAME — edits: its test module (a new exact-frame test of parse_dispatch)
@task_item n=1 part=needs
The Hyprland seam's bounded read of a compositor reply: a reply of exactly MAX_FRAME bytes is accepted and one byte more is refused as oversize.
@task_item n=2 part=needs
The Hyprland seam's dispatch parse: exactly MAX_FRAME bytes that are not the success text come back Refused, not oversize; MAX_FRAME + 1 bytes come back oversize.
@task_item n=3 part=needs
The oversize refusal state of a Hyprland reply, and where its boundary sits: the bound is inclusive at MAX_FRAME on both the read and the dispatch parse.
@task_item n=4 part=needs
The fake hyprctl the read's tests already use is the only compositor the new read test reaches: no real hyprctl, no live desktop step, no live claude start.
@task_item n=5 part=needs
Each new guard test owes a one-off remove-the-guard control on its own site (> to >=) that runs the test by name past fail-fast and holds only when that case ran and failed; no mutation run, local or dispatched.
@task_item n=6 part=needs
Test-only: the chunk changes no product behaviour, adds no requirement and claims no capability; a product line changes only if a new test proves it wrong, and then as an inferred item with its own acceptance.
@task_item n=7 part=needs
The clients parse's exact-frame bound stays held by its existing test, read back and not rebuilt.
@task_item n=8 part=needs
Every statement about how hyprctl, a crate or the compositor behaves is measured at P3 and cited, or marked unmeasurable.
@task_item n=1 part=not_needed
Brand identity, the flight-progress strip, the readback box and the send readback: the chunk changes no session display and no send confirmation.
@task_item n=2 part=not_needed
Colour palette, typography, spacing, depth, border radius and iconography: the chunk renders nothing and changes no CLI glyph, colour or column.
@task_item n=3 part=not_needed
The CLI surface's verb outputs, exit codes, streams, help text, refusal lines and refusal hints: no verb's output or exit changes.
@task_item n=4 part=not_needed
The viola list board and table, revive, run, verify, wait, link and last outputs and their refusals: no verb is touched.
@task_item n=5 part=not_needed
CLI colour decision order, colour depth and colour tokens, and TTY detection: no human output is styled or printed by the chunk.
@task_item n=6 part=not_needed
Wrapper fault --json detail, exit-21 causes, revive --json refusal codes and the wrapper socket: no JSON document or wrapper path changes.
@task_item n=7 part=not_needed
The web-spa surface and rendered-page absences: viola renders no page and the chunk adds none.
@task_item n=8 part=not_needed
The command names register: the chunk adds and renames no command.
@task_item n=9 part=not_needed
The self-validation protocol's squint, swap and signature checks: they judge visual design the chunk does not change.
