# Classified — architecture · conventions.b1

## Answers
conventions#1.1 · no · the Untouched list keeps RefusalReason and its details, and the stdin cut writes a parse-rejected diagnostic line, which is not a refusal
conventions#2.1 · no · the Untouched list keeps the exit table with no code added, and the mcp cut exits (0 once initialized, 11 before initialize) are the existing closed-stdin outcomes
conventions#3.1 · no · run's and revive's home-tree refusal lives in src/cmd/run.rs and strict, which the session does not reach, and the one reached change (the compare read in replace_private_shared) keeps its outcome with its callers unchanged
conventions#4.1 · no · verify's home-tree refusal and StampsError::StrictModes live in code the session does not reach, the Untouched list keeps the ledger stamps, and replace_private_shared's callers are pin_and_plugin and pin_companions, not update_stamps
conventions#5.1 · no · the server check lives in viola-channel and viola-state, which the session does not reach, and no endpoint or channel transport changed
conventions#6.1 · no · client::refuse_untrusted is outside the reached files and no endpoint or channel transport changed
conventions#7.1 · no · the Untouched list keeps the MCP result and refusal shapes and the tools/list fields, and the stdin cut reads as end of stream, never as a JSON-RPC error, so -32602 and -32603 stand
conventions#8.1 · no · the Untouched list keeps the frame tools' instance-unreachable result, and the reached viola-mcp change is the input-side stdin reader
conventions#9.1 · no · the Untouched list keeps the MCP result and refusal shapes, and the stdin cut writes its one line only at a cut, never on a frame tool's refusal
