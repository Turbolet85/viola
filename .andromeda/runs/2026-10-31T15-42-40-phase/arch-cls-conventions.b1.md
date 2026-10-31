# Classified — arch · conventions.b1

## Answers
conventions#1.1 · yes · the closed RefusalReason and detail sets, extended only in viola-core under the opaque-reader rule, bind the new cause codes; the send and answer refusal order does not move, and the kebab-case wire form is standing S6
conventions#2.1 · yes · each new code word that crosses a client read must pass viola_core::is_code_word (1 to 64 bytes of [a-z0-9-], a lower-case letter first)
conventions#3.1 · yes · a new cause code carried as a refusal or string detail is kept by reply_of and of_reply only if it is a code word, so the new codes must pass that bound
conventions#4.1 · yes · the exit 1 and exit 21 refusal set, their stderr pairs and the revive --json documents, each of which the chunk gives its own code
conventions#5.1 · yes · run's and revive's start-arm home-tree refusals (checked_exe, refuse_tree), exit 1, each owe a code under revive --json
conventions#6.1 · yes · verify's refused home tree (check_home_tree, exit 1) is edited by the chunk and owes a code
conventions#7.1 · yes · the server-verification cause, the frame-writing client's peer pid and start-time check: exit 21, no frame written, a code the chunk adds
conventions#8.1 · yes · the strict-modes exit 21 refusal of the frame verbs, viola list, plugin install, link and unlink, which the chunk gives a code and whose check order must not move
conventions#9.1 · yes · MCP isError results and structuredContent shaped like CLI --json, named in the Needs for the unreachable instance's text and structuredContent
conventions#10.1 · yes · the frame tool's server-verification refusal, text refused: instance-unreachable, is the MCP form of the server-verification cause the chunk adds
conventions#11.1 · yes · the strict-modes check runs before live_endpoint and writes no frame or line, an order the chunk must not move
