# Classified — architecture · standard-contracts.b1

## Answers
standard-contracts#1.1 · no · the payload's item, budget and skipped shape is unchanged; the session moves the board's refusal rule, which this entity does not state
standard-contracts#2.0 · no · the snapshot envelope and replay readers are unmoved; the new directory check reads only the endpoint already recorded
standard-contracts#3.0 · no · the ledger stamps envelope is untouched; no stamp, probe or ledger row moved
standard-contracts#4.1 · no · update_stamps still runs check_stamps and returns StampsError::StrictModes; strict.rs gains only check_socket_dir and a Refused variant, so this claim holds on the reached file
standard-contracts#5.1 · no · the link restore and the first snapshot write are unchanged; the new directory check only runs before them
standard-contracts#5.2 · no · the change points of the link set on implicit and served link/unlink are untouched
standard-contracts#5.3 · no · link's already_linked and since answers are unchanged
standard-contracts#5.4 · no · unlink's {} answer and its no-line rule are unchanged
standard-contracts#5.5 · no · the -32602 and -32603 link/unlink faults and the no-write rule are unchanged; fault codes do not move
standard-contracts#6.1 · yes · add-to · the pre-frame server check gains the peer euid check on Linux and macOS, a refused euid reading server-verify-failed; macOS's pid gap stays named
standard-contracts#7.1 · yes · add-to · the socket-dir check runs before this comparison and its refusal writes one process-exit WARN line, unlike the silent strict-modes refusal this entity states
standard-contracts#8.1 · no · send's cursor, budget-gate and link rules are unchanged; the session adds no send refusal
standard-contracts#9.1 · no · wait's arguments, start offset and wake rules are unchanged
standard-contracts#10.1 · no · link and unlink still read no from; nothing changed
standard-contracts#11.1 · no · ProtocolError::cause keeps its seven causes; frame and fault codes do not move
standard-contracts#12.1 · yes · add-to · hook.event's send gains the socket-dir check as a further refusal that sends nothing and fails open, writing one hook-decision WARN line
standard-contracts#13.1 · no · list's board refusal stays strict-modes-failed and its only one; the session moves only the home's rule, which this row does not name
standard-contracts#14.1 · no · prompt-submitted's shape and origin rules are unchanged; process-exit and hook-decision are diagnostic lines, not event kinds
standard-contracts#15.1 · yes · add-to · viola link and unlink gain a socket-dir refusal at exit 21 with detail socket-dir-refused and a process-exit line; the strict-modes refusal's silent form stands
standard-contracts#15.2 · yes · stale · the exit-21 detail's closed causes move from five to six with socket-dir-refused
standard-contracts#16.1 · yes · add-to · the frame verbs gain a socket-dir refusal at exit 21 beside strict-modes-failed, writing a process-exit line
standard-contracts#16.2 · yes · add-to · the socket-dir refusal's exit-21 document goes through the one writer, unreachable_document, beside the strict-modes writers
standard-contracts#16.3 · no · sanitised_fault's five codes and fixed texts are unchanged; fault codes do not move in viola-channel
standard-contracts#16.4 · no · the wrapper-fault reading for codes outside the five is unchanged
standard-contracts#16.5 · yes · add-to · revive --json's socket-dir start refusal (exit-1 codes ten to eleven) joins the causes refused_with prints as refused_document
