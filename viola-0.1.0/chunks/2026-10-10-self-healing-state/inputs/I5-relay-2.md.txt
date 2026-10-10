The operator's answers at this wrap's P2 halt, 2026-10-10, given in this session to two questions asked together, verbatim (question, the option chosen, the operator's note):

1. Question: Architecture's KEYSTONE entry [Database / State Store] says "Readers heal a torn last line." As landed, the appender heals and a reader never writes. The approved expected amendment names only the "as landed" and "owed" sentences, so the decision sentence itself is a locked-decision reword. How should it read?
   Chosen: Reword to the appender (Recommended) — "The next append heals a torn last line; readers count it and never rewrite the log."
   Note: The invariant stands (a torn last line never costs the next record and the log is never rewritten); only the actor differs, as the approved plan fixed it.

2. Question: Where does the new route entry go? Proposed text: "Interrupted verify cleanup — `viola verify` stopped by Ctrl-C or TERM removes only what that run created; older, foreign and kill-or-crash leftovers stay by hand", carrying the third CARRY and the three dir classes. `viola revive` stays the head.
   Chosen: Epoch 5, before Paste newline row (Recommended) — first entry of Epoch 5, directly ahead of "Paste newline ledger row".
   Note: Epoch 4 stays at 10. The entry text as you propose it.
