# Classified — arch · conventions.b1

## Answers
conventions#1.1 · yes · the closed RefusalReason set and its details, extended only in viola-core, which the chunk's new cause codes must fit (Needs: closed refusal vocabulary, kebab-case wire values)
conventions#2.1 · yes · the code-word rule (1 to 64 bytes of [a-z0-9-], lower-case letter first) that every code crossing a client read must pass through is_code_word
conventions#3.1 · yes · reply_of and of_reply's gate reading a non-code-word detail as null or unknown, which the no-path/pid/upstream-text codes and the unknown-word tolerance rely on
conventions#4.1 · yes · the typed exit set 0, 1, 2, 10–14, 20, 21 and the refusals each of exit 1 and exit 21 holds, which the chunk must not add to or re-map
conventions#5.1 · yes · run's and revive's start home-tree refusals (checked_exe, refuse_tree), edited by the chunk, each owing a code under --json and its hint pair
conventions#6.1 · yes · verify's refused home tree (check_home_tree), edited by the chunk, owing its code and its unable/hint pair
conventions#7.1 · yes · the server-verification cause on the frame verbs and link/unlink: a non-recorded server writes no frame and prints instance-unreachable with the server-verify hint
conventions#8.1 · yes · the strict-modes refusals of the frame verbs (client::refuse_untrusted, refused_document) as exit 21 with the instance-unreachable document, which the chunk must code and keep
conventions#9.1 · yes · MCP isError result shapes for the unreachable instance and refusals, and the JSON-RPC split for connection faults, which the Needs say must match CLI --json
conventions#10.1 · yes · the frame tools' server-identity check returning the instance-unreachable isError result with no frame and no line, the MCP side of the server-verification cause
conventions#11.1 · yes · the frame tools' check_instance running before live_endpoint with its instance-unreachable isError result, the order of checks the chunk must not move
