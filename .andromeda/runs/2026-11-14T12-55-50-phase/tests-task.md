@task_prose part=chunk
Test-only chunk on the one Hyprland seam (viola-hyprland): pin the exact-frame case on both MAX_FRAME bounds the Epoch 1 code audit left as survivors (`>` mutated to `>=`).
The bounded read of a hyprctl reply: a test drives it through the fake hyprctl with a reply of exactly MAX_FRAME bytes (accepted, not oversize) and one of MAX_FRAME + 1 bytes (oversize).
The dispatch parse: a test gives parse_dispatch exactly MAX_FRAME bytes (not oversize; Refused, as they are not the success text) and MAX_FRAME + 1 bytes (oversize).
Each new guard test carries its one-off remove-the-guard control on its site, so the next audit's mutation of either site is caught; no product line changes unless a test proves one wrong.
It re-reads, without re-pinning, the clients parse bound, already pinned by its existing exact-frame test, and adds no mutation run, no requirement and no capability.
@task_file n=1 path=crates/viola-hyprland/src/read.rs mark=edits
what: the Hyprland seam's bounded read of the hyprctl child's reply, capped at MAX_FRAME and refusing more as oversize — edits: its test module (a new exact-frame guard test of read_within through the fake hyprctl)
@task_file n=2 path=crates/viola-hyprland/src/reply.rs mark=edits
what: the Hyprland seam's reply parsers (parse_dispatch, parse_clients), each bounded by MAX_FRAME and refusing more as oversize — edits: its test module (a new exact-frame guard test of parse_dispatch)
@task_item n=1 part=needs
The bounded read of a hyprctl reply (read_within): a reply of exactly MAX_FRAME bytes is accepted, one of MAX_FRAME + 1 bytes is refused as oversize, the child's stdout drained only up to MAX_FRAME + 1 bytes.
@task_item n=2 part=needs
The dispatch reply parse (parse_dispatch): exactly MAX_FRAME bytes are not oversize and read as Refused (not the success text); MAX_FRAME + 1 bytes are oversize.
@task_item n=3 part=needs
The frame cap every bounded external reader keeps: a reader of an external process's output or reply stops at MAX_FRAME and refuses more, and the new tests pin that boundary byte for byte on both sites.
@task_item n=4 part=needs
The remove-the-guard witness each new guard test owes: a one-off bounded control that mutates the guard site itself (`>` to `>=`), names the test's case by its name alone, reaches that case past fail-fast under a kill deadline, and holds only when that case ran and failed.
@task_item n=5 part=needs
The mutation audit at the epoch boundary whose two survivors this chunk closes: the next audit's mutation of either bound must be caught by a test, while no mutation run, local or dispatched, happens inside the chunk.
@task_item n=6 part=needs
The read's test reaches hyprctl only through the fake hyprctl, a real stub executable at the seam the read's tests already use; no real hyprctl, no live desktop step and no live claude start.
@task_item n=7 part=needs
Doubles only at the product's own seams: the Hyprland seam is the one seam the fake replaces, and the dispatch parse, a pure function, is called directly with its bytes.
@task_item n=8 part=needs
The bound values the tests use come from the product constant MAX_FRAME (exactly it and one more), not from a copied number, and the expected outcomes (accepted, Refused, oversize) are written as literal oracles.
@task_item n=9 part=needs
The read's test waits on the fake child under a bounded deadline, with no sleep synchronisation and no retry.
@task_item n=10 part=needs
The clients reply parse's bound stays pinned by its existing exact-frame test (takes MAX_FRAME bytes, refuses one more as oversize); the chunk reads it and does not re-pin it.
@task_item n=11 part=needs
Test-only change: no product line moves unless a new test proves one wrong, in which case the fix folds into the chunk with its own acceptance; no requirement added and no capability claimed.
@task_standing n=1
convention: every new test function is named by the test-function naming rule (the behaviour and its outcome in the name)
home: 2-test-strategy
@task_standing n=2
convention: every new unit test lives inline in its source file's test module
home: 2-test-strategy
@task_standing n=3
convention: every new test synchronises without sleeps and is never retried on failure
home: 2-test-strategy
@task_standing n=4
convention: every new test writes its oracles as literals and holds no shared mutable state, running in parallel with the rest
home: 11-test-anti-patterns-never-do-these
@task_item n=1 part=not_needed
The other MAX_FRAME bounds of viola (the wrapper channel frame cap, the MCP stdin line cap, the answer verb's stdin cap, the statusline shell-out cap): they belong to other surfaces this chunk neither touches nor re-tests.
@task_item n=2 part=not_needed
The fake agent standing in for the claude CLI, its command line, receipts, turn scripts and recorded hook fixtures: the chunk's only double is the fake hyprctl.
@task_item n=3 part=not_needed
Hook, dialog, send, wheel, budget, statusline, list, revive and run-start contracts and their exit codes: no product surface beyond the Hyprland seam's two bounds is reached.
@task_item n=4 part=not_needed
The test harness commands and their documents (boot, cleanup, status, session record, supervisor, pre-push, run --mutants verdicts): no mutation run happens and the chunk adds no harness surface.
@task_item n=5 part=not_needed
The declined mutation units of viola-e2e and viola: they stay out, waiting on a pipeline mend.
@task_item n=6 part=not_needed
Performance budgets and hyperfine timing of hooks: the chunk times nothing.
@task_item n=7 part=not_needed
Process log lines and their fields, the event log and diagnostics schemas: the chunk adds no log line and no event.
@task_item n=8 part=not_needed
CI artifacts, their retention and the artifact secret scan: the chunk adds no CI job and no artifact.
@task_item n=9 part=not_needed
Fixture recording, scrubbing and hygiene of recorded claude payloads: the chunk records no fixture.
