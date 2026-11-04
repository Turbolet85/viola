@answer n=1 entity=standard-contracts#1.1 v=yes
why: the list item and document with its closed liveness, status and wheel words is a --json document the one-schema rule covers, and the stale row is the v1-40 witness
@answer n=2 entity=standard-contracts#2.0 v=yes
why: the snapshot envelope and the instance snapshot fields are covered by the one-schema rule, and the replay rule reads a field no event carries as absent
@answer n=3 entity=standard-contracts#3.0 v=no
why: the ledger stamps envelope belongs to the not-needed capability ledger and viola verify, and this chunk runs no live claude
@answer n=4 entity=standard-contracts#4.1 v=no
why: the strict-modes check before update_stamps is the viola verify write path, which the chunk does not reach
@answer n=5 entity=standard-contracts#5.1 v=no
why: the link set's replay and restore is the not-needed Links item, and the snapshot's links field is the same on every OS
@answer n=6 entity=standard-contracts#5.2 v=no
why: the link set's change on an implicit link or a served link and unlink call is the not-needed Links item
@answer n=7 entity=standard-contracts#5.3 v=no
why: the already_linked answer of a link call is the not-needed Links item
@answer n=8 entity=standard-contracts#5.4 v=no
why: unlink's empty answer and its absent-pair rule are the not-needed Links item
@answer n=9 entity=standard-contracts#5.5 v=no
why: a link or unlink fault writing no line is the not-needed Links item
@answer n=10 entity=standard-contracts#6.1 v=yes
why: the peer euid check and the macOS carve-out that holds the euid check alone bind the Unix IPC endpoint and the macOS gap
@answer n=11 entity=standard-contracts#7.1 v=yes
why: the strict-modes check and the endpoint directory check before every connect bind the socket-dir rule of the Unix endpoint
@answer n=12 entity=standard-contracts#8.1 v=yes
why: send's reply shape and its closed budget-paused and unconfirmable words are --json documents the one-schema rule covers, and the parity suite drives send through the PTY write seam under the fake agent
@answer n=13 entity=standard-contracts#9.1 v=yes
why: wait's reply shape, its timed_out document and its closed -32602 faults are --json documents the one-schema rule covers, and the parity suite drives wait under the fake agent
@answer n=14 entity=standard-contracts#10.1 v=no
why: link and unlink ignoring a from field is the not-needed Links item
@answer n=15 entity=standard-contracts#11.1 v=no
why: the internal cause classification of channel faults is not printed on any --json document, and the documents it reaches are covered by the sanitised_fault entity
@answer n=16 entity=standard-contracts#12.1 v=yes
why: hook.event's socket-directory and server checks and its fail-open rule bind the viola hook contract and the Unix endpoint's check on every connect
@answer n=17 entity=standard-contracts#13.1 v=yes
why: list's item and document shape with its closed liveness and status words is a --json document the one-schema rule covers and the v1-40 witness reads
@answer n=18 entity=standard-contracts#14.1 v=yes
why: the prompt-submitted per-kind event data with its origin closed words is the ndjson event data the one-schema rule covers
@answer n=19 entity=standard-contracts#15.1 v=yes
why: the refused-check exit-21 document of link and unlink is a closed-word CLI --json document the one-schema rule covers, though their success verbs are not reached
@answer n=20 entity=standard-contracts#15.2 v=yes
why: the six closed causes of the exit-21 document are the UnreachableCause words item 6 names, socket-dir-refused among them
@answer n=21 entity=standard-contracts#16.1 v=yes
why: the refused-check document of send, wait, last, answer, pause and release, with its socket-dir-refused case, is a --json document the one-schema rule covers
@answer n=22 entity=standard-contracts#16.2 v=yes
why: unreachable_document as the one writer of the exit-21 document fixes the refusal words every verb must keep identical across OSes
@answer n=23 entity=standard-contracts#16.3 v=yes
why: sanitised_fault's fixed messages and data words are the closed error words of the --json fault documents
@answer n=24 entity=standard-contracts#16.4 v=yes
why: the fixed wrapper fault word printed on --json for an unlisted code is a closed error word the one-schema rule covers
@answer n=25 entity=standard-contracts#16.5 v=yes
why: Launch's json refused_document for socket-dir-refused and the revive refusal codes are closed --json words the one-schema rule covers
