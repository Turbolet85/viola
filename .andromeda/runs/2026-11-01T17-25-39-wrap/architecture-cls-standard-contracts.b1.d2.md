# Classified — architecture · standard-contracts.b1

## Answers
standard-contracts#1.1 · no · the session moves only the board's refusal rule, and the item and budget shapes this row states are unchanged
standard-contracts#2.0 · no · the new socket-dir check reads the recorded endpoint, not the snapshot, and the six readers keep their strict-modes order
standard-contracts#3.0 · no · the ledger stamps envelope and its writers are not reached, and viola_state::stamps is untouched
standard-contracts#4.1 · no · update_stamps still runs strict::check_stamps, which the session leaves as it was beside the new check_socket_dir
standard-contracts#5.1 · no · the links restore before server.serve and the LinkSet are not changed by the session
standard-contracts#5.2 · no · the change points of the set on implicit and served link and unlink calls are unchanged
standard-contracts#5.3 · no · link's already_linked and since answers are not changed by the session
standard-contracts#5.4 · no · unlink's {} answer and the absent-pair rule are not changed by the session
standard-contracts#5.5 · no · the -32602 and -32603 link and unlink faults are unchanged, and a refused peer writes no line rather than a fault
standard-contracts#6.1 · yes · add-to · F3 and F4: a peer euid check joins the pid and start-time comparison before the first frame on Linux and macOS, and a refused euid writes no frame
standard-contracts#7.1 · yes · add-to · F3 and F8: the socket-dir check runs before connect and its refusal writes a WARN process-exit line, which this row's no-line wording does not carry
standard-contracts#8.1 · no · send's wrapper rungs, budget gate and link recording are unchanged, and the new socket refusal is client-side and records no event
standard-contracts#9.1 · no · wait's wrapper behaviour, start offset and wake set are unchanged
standard-contracts#10.1 · no · link and unlink still read no from field
standard-contracts#11.1 · no · the ProtocolError::cause mapping is unchanged and no fault code moves
standard-contracts#12.1 · yes · add-to · F6: a refused socket directory also sends nothing and fails the hook open with one hook-decision WARN line, a case this row's refusal list does not name
standard-contracts#13.1 · no · list's CLI and MCP order, join and the single strict-modes-failed refusal word are unchanged by the read-rule move
standard-contracts#14.1 · no · prompt-submitted fields and origin values are event data, which the session does not move
standard-contracts#15.1 · yes · add-to · viola link and unlink gain the socket-dir check, so a refused socket directory also exits 21 with socket-dir-refused and writes a line, which this row's strict-modes-only wording does not carry
standard-contracts#15.2 · yes · stale · the exit-21 detail's closed causes move from five to six with socket-dir-refused
standard-contracts#16.1 · yes · add-to · a frame verb's refused socket directory is CallError::SocketDirRefused, exit 21 with socket-dir-refused and a process-exit WARN line (F8), which this row's strict-modes-only, no-line wording does not cover
standard-contracts#16.2 · yes · add-to · the reached src/cmd/client.rs routes a refused socket directory to exit 21 (F8), which the one-writer statement of this row must carry
standard-contracts#16.3 · no · sanitised_fault and its five fixed messages are unchanged and no fault code moves
standard-contracts#16.4 · no · the wrapper-fault literal and its output shape are unchanged
standard-contracts#16.5 · yes · add-to · F1: run's start refuses a socket directory with exit 1 and socket-dir-refused before spawn_child, a refusal the three refused_with cases this row names do not list
