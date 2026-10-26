# arch · picked accounting · group 3 of 3

## Picked
conventions#1.1 · carried · the new pre-frame check is no `RefusalReason`, and the CLI's own pre-frame `control-character` / `empty-text` checks keep their place beside it in `send` and `answer`
conventions#2.1 · carried · exit 21 is every CLI verb's mismatch outcome, and `wait` / `last`'s exit 1 versus 21 split rules over the failed-check case
conventions#3.1 · carried · the MCP tools' mismatch outcome is the `instance-unreachable` `isError` result with `detail: null` and no frame written
standard-contracts#2.0 · carried · the held outcome for an unreadable or newer snapshot and the five `read_snapshot` callers the chunk re-reads are stated here
standard-contracts#5.1 · not in scope · the wrapper's `send` handling (budget gate, link record) is a component the chunk only writes a frame to, its params and `ok` payload unchanged
standard-contracts#6.1 · not in scope · the wrapper's `wait` handling and wake are a component the chunk only writes a frame to, unchanged by a client-side check
standard-contracts#7.1 · carried · `hook.event` is an id-less notification with no reply, the frame the hook client's check precedes and the basis of the Windows exiting-wrapper CARRY the scope re-reads
standard-contracts#10.1 · carried · the statusline arm's order places the `read_snapshot` call the chunk re-reads after its instance check
occupied-resources#1.1 · carried · every client connects to the snapshot's recorded `endpoint`, the server the check verifies, and the rebuild-on-unreadable rule meets the held item
occupied-resources#8.1 · carried · `VIOLA_DIR` is how `hook` finds the recorded endpoint and the SessionEnd direct-append fallback the scope re-reads
occupied-resources#13.1 · carried · the CLI, `mcp` and `hook` role-file line lists record the exit-21 detail a mismatch outcome adds
occupied-resources#14.1 · carried · the readers that check instance files before use, the `check_instance` call-site count and the check-before-log order bind if P4 takes the strict-modes half here
cross-cutting-patterns#1.1 · not in scope · the home resolution the clients only use, unchanged by the chunk; its test-isolation clause is standing S1
