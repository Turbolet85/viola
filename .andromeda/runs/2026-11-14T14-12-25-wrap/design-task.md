@task_prose part=session
The session was test-only: it added two unit tests to the viola-hyprland crate, the one Hyprland seam, and changed no product line.
One test drives the seam's read of a hyprctl reply at exactly the frame cap (MAX_FRAME bytes), which is taken, and at one byte more, which is refused as oversize with the oversize log detail.
The other drives the parse of a hyprctl dispatch reply at exactly the frame cap, which reads as refused, and at one byte more, which reads as oversize.
The bounds, their values, the refusal words and every caller stand as they were; nothing was retired, renamed or added to any surface, and no capability is claimed.
Measured: the crate's own test count moved from 82 to 84, and both new tests were proven to fail when their bound is removed.
@task_file n=1 path=crates/viola-hyprland/src/read.rs
what: the Hyprland seam's read of a hyprctl reply: it spawns hyprctl, drains its stdout under the frame cap and the deadline, refuses an oversize reply and logs the process lines
@task_file n=2 path=crates/viola-hyprland/src/reply.rs
what: the Hyprland seam's reply parser: it turns a hyprctl clients or dispatch reply into a typed reply or a refusal, refusing any reply past the frame cap as oversize
@task_item n=1 part=touches
Any row stating the Hyprland seam's reply frame cap or its oversize refusal: the session pins that bound at the exact frame by test, its value unchanged.
@task_item n=2 part=touches
Any row stating that a hyprctl dispatch reply that is not the success text reads as refused: the session pins that word for a reply of exactly the frame cap.
@task_item n=3 part=touches
Any row over the oversize log detail a bounded reader writes when it refuses a reply: the session's read test checks that detail for the Hyprland read, unchanged.
@task_item n=4 part=touches
Any rule that a bounded reader takes a frame of exactly its cap and refuses one byte more: the session adds the Hyprland read and dispatch parse as witnesses of it, with no change to the rule.
@task_item n=5 part=touches
Any row naming a test as the witness of a reader's bound or holding the Hyprland crate's test count: the crate's count moved from 82 to 84.
@task_item n=1 part=untouched
The CLI's human and --json output forms, its streams, refusal lines, hints and exit codes: no verb's output, word or code changed.
@task_item n=2 part=untouched
The viola list board, its columns, widths, colours and row escaping: the board's rendering was not reached.
@task_item n=3 part=untouched
Brand identity, colour palette, typography, spacing, iconography and the self-validation protocol: no visual or vocabulary decision moved.
@task_item n=4 part=untouched
The wrapper socket's peer reply screening and wrapper fault details: these govern the wrapper's peers, not hyprctl replies, and were not reached.
@task_item n=5 part=untouched
The revive, run, verify, link, send and wait verb contracts and the command names: no verb was added, renamed or changed.
