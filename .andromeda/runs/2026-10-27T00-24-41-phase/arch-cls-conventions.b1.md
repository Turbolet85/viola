# Classified — arch · conventions.b1

## Answers
conventions#1.1 · no · the RefusalReason set is the send and answer refusal vocabulary; the race fix sends no frame and changes no method, and its new failure is a thiserror variant, not a refusal reason
conventions#2.1 · yes · the run row for pinned-hash-mismatch (exit 1, the file left as found) binds because the chunk re-asserts the hash-before-reuse rule; the start-refusal rule (exit 1, no new code outside this table) binds the race fix's failure path
conventions#3.1 · yes · the run start's home-tree check (checked_exe then refuse_tree, before any log) binds: the scope names that check, the start pin_bytes call and the per-start plugin rewrite must follow it, and a new .lock or temp entry under bin/<key>/ or plugin/<key>/ must pass its write rule
conventions#4.1 · no · verify's pre-log home-tree check is another unit of verify.rs that the change does not edit (its edit is the pin_exe call after it), so it is a re-check every change owes
conventions#5.1 · no · exit 21 for the frame verbs' server-identity check on send, wait, last, answer, pause, release, link and unlink; the race fix sends no frame and changes no method
conventions#6.1 · no · exit 21 for the frame verbs' strict-modes check on instances/<name>; the chunk touches no frame verb and no instance directory
conventions#7.1 · no · the MCP result shapes and the recorded tools/list exchange; nothing the driver calls changes and the chunk touches no MCP tool or its test
conventions#8.1 · no · the frame tools' instance-unreachable result on a server-identity mismatch; the chunk touches no frame tool and no MCP result
conventions#9.1 · no · the frame tools' check_instance ordering before live_endpoint; the chunk touches no frame tool and runs no instance check, its own check being check_home_tree over bin/ and plugin/
