# The answer to the wrap's P7.1 basis question (AskUserQuestion), 2026-10-26, verbatim

Question: how the wrap gets a basis and an owner for entry 21's red (the cut-short fake-agent profile), the census having cleared the chunk's holders.
Selected: "Parent pre-push loop (Recommended)"
Note: The overseer's technical answer (founder-delegated), 11:18Z: measure the basis, don't override. Run the parent pre-push loop on 6bdfdda8 in a clean worktree, up to 8 runs, stopping at the first refusal; the owner then is a WATCH on «Explicit session links». If 8 read green, report back without resuming. Journal the loop in the run dir before you wait on it, and remove the worktree with `git worktree remove` when done.
