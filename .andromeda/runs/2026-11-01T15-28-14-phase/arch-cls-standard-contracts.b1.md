# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · no · the list item and budget payload fields are Not needed; the chunk changes the home refusal and its exit, not the item shape
standard-contracts#2.0 · yes · a replayed snapshot carries no endpoint, so live_endpoint and the hook's connect read it unreachable, and the client's directory check acts on the recorded endpoint
standard-contracts#3.0 · no · the ledger stamps envelope and its verdict are Not needed, and the chunk's strict read does not change the envelope
standard-contracts#4.1 · yes · check_stamps is one of viola_state::strict's checks the Unix read and write rules run through, so its refusal must read and write nothing and run ahead of creating ledger/
standard-contracts#5.1 · no · the link set restore rules the in-memory set and first-snapshot links, which the chunk's checks do not change
standard-contracts#5.2 · no · the set changes only on implicit links and served link/unlink calls, and a refused client's call stops before its first frame, so the rule stands unchanged
standard-contracts#5.3 · no · already_linked answers belong to served link calls the chunk does not change
standard-contracts#5.4 · no · unlink's {} answer is unchanged and a refused client's call is stopped before its frame
standard-contracts#5.5 · no · the -32602 and -32603 faults of link and unlink are unchanged, and the new refusals add no fault
standard-contracts#6.1 · yes · the existing server check is this peer pid and start-time comparison, with the new euid check beside it and the macOS carve-out recorded against it
standard-contracts#7.1 · yes · refuse_untrusted, trusted and check_instance are the strict checks the new client checks sit beside, and the one snapshot read feeds the comparison
standard-contracts#8.1 · no · send's budget gate and from-link recording run after the first frame, a refused client's send never reaches them, and the budget governor is Not needed
standard-contracts#9.1 · no · wait's parameters, start offset and wake rules are unchanged, and a refused client's wait stops before its first frame
standard-contracts#10.1 · no · link and unlink ignore from, a rule the chunk does not change, and the client check acts before the frame
standard-contracts#11.1 · no · cause reads only the three -32602 faults, and the new refusals come before the first frame and add no fault
standard-contracts#12.1 · yes · hook.event is gated by the instance check and the server check, the hook's connect gains the client euid check, and a refusal reaches the hook only as fail-open
standard-contracts#13.1 · yes · CLI list reads board::scan then board::read and MCP list maps the board's refusal to strict-modes-failed, the outcome the scan home check changes
standard-contracts#14.1 · no · prompt-submitted text and origin are unchanged, and the chunk adds no event kind or data
standard-contracts#15.1 · yes · viola link and unlink are client verbs gaining the new refusal, whose --json document and exit 21 must fit this shape
standard-contracts#15.2 · yes · the chunk extends the closed UnreachableCause set in viola-core with its new refusal causes, so the five-cause list changes
standard-contracts#16.1 · yes · send, wait, last, answer, pause and release gain the client euid check before their first frame, and their refusal documents must fit this shape
standard-contracts#16.2 · yes · the new refusals' exit-21 documents must be written through client::unreachable_document, the one writer
standard-contracts#16.3 · no · sanitised_fault reads only the five wrapper faults, and the new refusals come before the first frame and add no fault code
standard-contracts#16.4 · no · the wrapper-fault fallback for codes outside the five is not reached, since the chunk's refusals add no fault
standard-contracts#16.5 · yes · run's socket directory refusal at start must return before spawn_child and print refused_document under --json, as revive does
