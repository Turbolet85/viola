# Classified — architecture · standard-contracts.b1

## Answers
standard-contracts#1.1 · no · the list payload is untouched (Untouched: list payload) and its board, budget and list code lies in no reached file
standard-contracts#2.0 · no · the snapshot stays a mutable file replaced atomically by fs.rs's existing replace helper (Touches: crash-safe writes), and its envelope and replay rule are untouched (Untouched: snapshot field)
standard-contracts#3.0 · no · the ledger stamps envelope, its relied set and its stamped versions are untouched, and stamps.json stays atomically replaced (Touches: crash-safe writes)
standard-contracts#4.1 · no · update_stamps lies in stamps.rs, which no reached file holds, and strict.rs is reached but its product verdict is unchanged, so check_stamps still gates the read and write
standard-contracts#5.1 · no · the links restore and rewrite are untouched (Untouched: Links) and their code lies in no reached file
standard-contracts#5.2 · no · the set's change points are untouched (Untouched: Links) and their code lies in no reached file
standard-contracts#5.3 · no · the already_linked answer is untouched (Untouched: Links) and its code lies in no reached file
standard-contracts#5.4 · no · the empty unlink answer is untouched (Untouched: Links) and its code lies in no reached file
standard-contracts#5.5 · no · the -32602 and -32603 causes are untouched (no new refusal code or detail) and their code lies in no reached file
standard-contracts#6.1 · no · the server identity check is untouched (Untouched: server identity check)
standard-contracts#7.1 · no · the refused check still writes no frame and no line, with no src/cmd file changed and strict.rs's check_instance keeping its product verdict
standard-contracts#8.1 · no · send's shape, budget gate and link rung are untouched (no channel method changed; Delivery Confirmation, Budget Governor and Links untouched)
standard-contracts#9.1 · no · wait's shape and wake set are untouched (no channel method changed) and src/run/wait.rs lies in no reached file
standard-contracts#10.1 · no · link and unlink still read no from (Untouched: Links) and their code lies in no reached file
standard-contracts#11.1 · no · hook.event's shape and its re-validation are untouched (Untouched: Hook Transport and Hook Contract)
standard-contracts#12.1 · no · the list payload and the claude agents --json join are untouched (Untouched: Session Liveness)
standard-contracts#13.1 · no · prompt-submitted's data is untouched (no event kind or data field changed)
standard-contracts#14.1 · no · the link and unlink --json documents are untouched (no CLI --json document changed, no src/cmd file changed)
standard-contracts#15.1 · no · the --json refusal documents of send, wait, last, answer, pause and release are untouched (no CLI --json document changed, no src/cmd file changed)
standard-contracts#16.1 · no · the hook statusline arm is untouched (Untouched: Hook Contract, no src/cmd file changed)
standard-contracts#17.1 · no · the server identity read is untouched (Untouched: server identity check)
standard-contracts#18.1 · no · the claude agents --json join is untouched (Untouched: Session Liveness) and the fake agent change is its hook receipt alone
