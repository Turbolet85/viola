# Classified — arch · conventions.b1

## Answers
conventions#1.1 · yes · the closed RefusalReason set and its per-reason details, extended only in viola-core, and the reader rule that an unrecognised code word is opaque; the chunk's new cause codes must fit that home and reader rule (Needs: closed refusal vocabulary)
conventions#2.1 · yes · viola_core::is_code_word (1 to 64 bytes of [a-z0-9-], lower-case letter first); code words crossing a client read through it (Needs: sanitised error surfaces), so each new code must pass it
conventions#3.1 · yes · reply_of and of_reply read a non-code-word refusal as unknown and a non-code-word string detail as null; binds where the chunk carries a cause's code in detail, which Needs leaves open (error or detail)
conventions#4.1 · yes · the exit-code table: the exit 1 refusals (run's already-live, pinned-hash-mismatch, batch-script-child, squatted-name; revive's preflight refusals and strict-modes) and the exit 21 refusals (list, plugin install, link, unlink), which the chunk gives codes and must not re-map
conventions#5.1 · yes · run's and revive's start-arm home-tree refusal (checked_exe, refuse_tree), exit 1 with no log opened, which the chunk edits in run.rs and which now owes a code
conventions#6.1 · yes · verify's refused home tree (check_home_tree, before its cli log opens), edited by the chunk in verify.rs, which now owes a code
conventions#7.1 · yes · exit 21 for a server that is not the recorded wrapper on send, wait, last, answer, pause, release, link and unlink; the chunk's server check in client.rs and its document
conventions#8.1 · yes · exit 21 and the instance-unreachable document of client::refuse_untrusted, which the chunk edits, for the frame verbs and the strict-modes check ahead of any log
conventions#9.1 · yes · MCP isError results shaped like CLI --json (structuredContent {error, detail}) and JSON-RPC errors kept to connection faults; the chunk edits viola-mcp result.rs and tools.rs
conventions#10.1 · yes · frame tools' server-not-wrapper check returning the instance-unreachable isError result with no frame and no line; the chunk's server-verification cause reaches the MCP frame tools
conventions#11.1 · yes · frame tools' check_instance ahead of live_endpoint returning the instance-unreachable isError result with detail null; the chunk's strict-modes refusals and their check order ahead of any log
