# Classified — architecture · conventions.b1

## Answers
conventions#1.1 · no · RefusalReason gains nothing and the new word has no serde path, so the refusal-reason words and details stand
conventions#2.1 · no · the is_code_word rule is unchanged, and socket-dir-refused already fits its grammar
conventions#3.1 · no · reply_of and of_reply read no change; the session adds no reply word and no refusal detail
conventions#4.1 · yes · contradict · UnreachableCause gains SocketDir, so the five-word closed cause set is now six
conventions#5.1 · yes · add-to · socket-dir-refused joins exit 1's start refusals (F1) and exit 21's refused checks at the frame verbs and link/unlink (F8)
conventions#6.1 · no · the catch site and prints_internal_error stay as they are; the socket refusal is a handled refusal, not an internal error
conventions#7.1 · yes · contradict · the socket-directory refusal now has a code, socket-dir-refused, read the same for a failed create (F1), so "no code" and OPEN are no longer true
conventions#8.1 · no · the refused home-tree list and its order in refuse_tree are unchanged; socket_dir_ready is a separate check (F1)
conventions#9.1 · no · viola verify and its refuse_tree check are not reached by the session
conventions#10.1 · no · verify's refusal codes are not reached by the session
conventions#11.1 · yes · add-to · a peer euid mismatch joins pid and OS start time as a server-verify-failed cause on Linux and macOS (F3, F4); the no-frame and document shape stand
conventions#12.1 · yes · add-to · a socket-directory unable/hint pair joins src/human/hints.rs beside the two exit-21 hints, plain ASCII, hint last (F12)
conventions#13.1 · yes · contradict · server-verify-failed is no longer claimed off macOS only (now Linux, Windows and macOS), and its route owner is retired
conventions#14.1 · no · the strict-modes instance check and its exit-21 document stay first and unchanged; the socket-directory check is a separate check
conventions#15.1 · yes · add-to · answer_of gains the mapping CallError::SocketDirRefused to Unreachable::SocketDir (F8); its Connect, Closed and Io arms stand
conventions#16.1 · yes · add-to · the instance-unreachable cause list gains socket-dir-refused for MCP's isError result from frame (F7), and a refused euid reaches server-verify-failed
conventions#17.1 · yes · add-to · a peer euid mismatch now also makes a frame tool return the instance-unreachable isError (F3, F4), beside the pid and start-time case
conventions#18.1 · no · check_instance still runs first, before live_endpoint; the new socket-directory check runs on the recorded endpoint at the connect step (F3, F7), not ahead of it
