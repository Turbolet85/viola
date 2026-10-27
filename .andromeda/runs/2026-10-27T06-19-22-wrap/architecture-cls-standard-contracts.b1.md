# Classified — architecture · standard-contracts.b1

## Answers
standard-contracts#1.1 · no · the report leaves the list payload untouched with no field added, and its rule lives in board and list code that no reached file holds
standard-contracts#2.0 · no · the snapshot envelope and its replay readers are untouched, and replace_private atomicity is re-read and unchanged in the reached fs.rs, which bounds only its pinned-copy compare read
standard-contracts#3.0 · no · the ledger stamps contract is untouched with no row added or re-stamped, and the reached diag-line schema only gains the mcp-stdin parser value so ledger-stamps stands
standard-contracts#4.1 · no · update_stamps and its strict-modes check live in stamps.rs and strict.rs, neither reached, and the report keeps replace_private_shared's callers pin_and_plugin and pin_companions unchanged
standard-contracts#5.1 · no · the instance snapshot's links set is untouched with no field or reader changed, and its rule lives in links code that no reached file holds
standard-contracts#5.2 · no · the implicit link and the served link and unlink changes are untouched, the report leaving links and the channel method set unchanged
standard-contracts#5.3 · no · the already_linked and since answers are untouched, the report leaving links and the channel ok payloads unchanged
standard-contracts#5.4 · no · the unlink answer is untouched, the report leaving links and the channel ok payloads unchanged
standard-contracts#5.5 · no · the link and unlink error paths sit in the wrapper, which no reached file holds
standard-contracts#6.1 · no · the peer-pid comparison is the session liveness path the report leaves unchanged, and the MCP tool set holds on the reached viola-mcp lib.rs
standard-contracts#7.1 · yes · stale · the CappedLines stdin wiring reaches viola-mcp lib.rs whole, and the report does not show the MCP frame tools' strict::check_instance still holds there
standard-contracts#8.1 · no · the send path lives in the wrapper, which no reached file holds, and the report leaves the budget governor, the channel method set and the MCP tool set unchanged
standard-contracts#9.1 · no · the wait path and its wake set live in the wrapper, which no reached file holds, and the report leaves the wheel, the channel method set and the MCP tool set unchanged
standard-contracts#10.1 · no · the link and unlink from rule lives in the wrapper, which no reached file holds, and the report leaves links and the channel params unchanged
standard-contracts#11.1 · no · the hook.event transport, its validation and its relabel are untouched, the report naming hook transport and the hook contract as unchanged
standard-contracts#12.1 · no · the list payload and the join are untouched, and the reached viola-mcp lib.rs keeps the MCP list result and refusal shapes, the report naming the MCP tool set unchanged
standard-contracts#13.1 · no · the prompt-submitted event data is untouched, the report leaving the event data per kind unchanged
standard-contracts#14.1 · no · the link and unlink CLI JSON shapes are untouched, the report leaving the channel ok payloads and the CLI exit code table unchanged
standard-contracts#15.1 · no · the CLI frame verbs' instance-unreachable document and exit 21 are untouched, the report leaving the exit code table and RefusalReason unchanged
standard-contracts#16.1 · no · the statusline arm is hook transport under the hook contract, whose code paths the report leaves unchanged
standard-contracts#17.1 · no · the server identity read is session liveness, whose code paths the report leaves unchanged
standard-contracts#18.1 · no · the agents read and join are session liveness, whose code paths the report leaves unchanged, and the MCP list refusal shape holds on the reached viola-mcp lib.rs
