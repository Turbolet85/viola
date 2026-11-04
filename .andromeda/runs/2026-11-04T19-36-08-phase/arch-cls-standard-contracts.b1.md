@answer n=1 entity=standard-contracts#1.1 v=yes
why: the list payload item shape and its top-level budget are the list --json document, with the liveness and wrapped fields the parity list tests read on each OS (need 6, need 11)
@answer n=2 entity=standard-contracts#2.0 v=yes
why: the snapshot envelope and its replay fallback are the snapshot fields the one-schema rule covers (need 6), read by the list board and run's collision check
@answer n=3 entity=standard-contracts#3.0 v=no
why: the ledger stamps file is written only by viola verify and needs a live claude, which this chunk runs none of (not-needed 4)
@answer n=4 entity=standard-contracts#4.1 v=no
why: the stamps update path is viola verify's write, not reached by the chunk (not-needed 4)
@answer n=5 entity=standard-contracts#5.1 v=yes
why: the instance snapshot's links field is one of the instance snapshot fields the one-schema rule covers (need 6), so each OS's snapshot must carry the same shape; the link verbs themselves are not reached
@answer n=6 entity=standard-contracts#5.2 v=no
why: it states when the link set changes through the link and unlink verbs and the implicit link, none reached (not-needed 3)
@answer n=7 entity=standard-contracts#5.3 v=no
why: the already_linked answer belongs to the link verb's document, the link verb not reached (not-needed 3)
@answer n=8 entity=standard-contracts#5.4 v=no
why: the unlink verb's {} answer and its absent-pair rule, the unlink verb not reached (not-needed 3)
@answer n=9 entity=standard-contracts#5.5 v=no
why: the -32602 and -32603 faults of link and unlink write no line, and those verbs are not reached (not-needed 3)
@answer n=10 entity=standard-contracts#6.1 v=yes
why: the pre-first-frame peer euid check, the peer pid and start-time comparison and macOS's euid-only carve-out are the Unix IPC rules of need 8 and need 9 that the parity suite exercises
@answer n=11 entity=standard-contracts#7.1 v=yes
why: the strict-modes check and the endpoint directory check before each client's connect are the socket-dir rule of need 8, checked before every connect
@answer n=12 entity=standard-contracts#8.1 v=yes
why: send's result document carries the closed unconfirmable detail and the cursor, which the parity suite's send tests read on every OS through the CLI --json documents (need 6)
@answer n=13 entity=standard-contracts#9.1 v=yes
why: wait's {event, cursor} and {timed_out:true} documents and its wake set are read by the parity suite's wait tests on each OS (need 6)
@answer n=14 entity=standard-contracts#10.1 v=no
why: it states that the link and unlink verbs read no from, and those verbs are not reached (not-needed 3)
@answer n=15 entity=standard-contracts#11.1 v=yes
why: the cause words of ProtocolError are the closed detail words of the -32602 fault documents the one-schema rule covers (need 6)
@answer n=16 entity=standard-contracts#12.1 v=yes
why: the hook.event notification and its append to events.ndjson are the hook path the fake-agent suite runs on every OS, with the ndjson event line the one-schema rule covers (need 6, need 18)
@answer n=17 entity=standard-contracts#13.1 v=yes
why: list's board and its --json document carry the liveness, status and unwrapped-item fields that the v1-40 witness and the list colour decision read under the outer PTY (need 11, need 12)
@answer n=18 entity=standard-contracts#14.1 v=yes
why: the prompt-submitted per-kind data is the ndjson event line's data that the one-schema rule covers (need 6)
@answer n=19 entity=standard-contracts#15.1 v=no
why: it states the link and unlink verbs' documents and their refusal, the link verbs not reached (not-needed 3); the shared exit-21 writer is covered by standard-contracts#16.2
@answer n=20 entity=standard-contracts#15.2 v=yes
why: the six closed exit-21 causes are the UnreachableCause words the one-schema rule covers (need 6, need 8)
@answer n=21 entity=standard-contracts#16.1 v=yes
why: the exit-21 strict-modes and socket-dir-refused documents of send, wait, last, answer, pause and release are CLI --json documents with closed words (need 6, need 8)
@answer n=22 entity=standard-contracts#16.2 v=yes
why: unreachable_document is the one writer of the exit-21 document that the reached CLI verbs share, so the one-schema rule reads it on each OS (need 6)
@answer n=23 entity=standard-contracts#16.3 v=yes
why: sanitised_fault fixes the closed fault messages and data of the CLI --json fault documents the one-schema rule covers (need 6)
@answer n=24 entity=standard-contracts#16.4 v=yes
why: the literal wrapper fault output word on --json is a closed word that each OS must read as documented (need 6, need 7)
@answer n=25 entity=standard-contracts#16.5 v=yes
why: run's refused_with and revive's json refusal document carry the revive refusal codes and socket-dir-refused, the closed words of need 6 and need 8
