@answer n=1 entity=standard-contracts#1.1 v=no
why: the witness only exercises the unchanged stale row of the list liveness field, and no product file moved
@answer n=2 entity=standard-contracts#2.0 v=no
why: the snapshot envelope and the replay rule are untouched, and the Frozen guard reads ps rather than the snapshot
@answer n=3 entity=standard-contracts#3.0 v=no
why: the ledger stamps envelope and its relied row count are product-side, and the session adds no ledger row
@answer n=4 entity=standard-contracts#4.1 v=no
why: update_stamps and its strict-modes check are product code the session did not move
@answer n=5 entity=standard-contracts#5.1 v=no
why: the links set is product code in src/run that the session did not change
@answer n=6 entity=standard-contracts#5.2 v=no
why: LinkSet changes on implicit link and served link/unlink, which the session did not touch
@answer n=7 entity=standard-contracts#5.3 v=no
why: the already_linked answer of a link is unchanged and no link code moved
@answer n=8 entity=standard-contracts#5.4 v=no
why: unlink's empty answer is unchanged and no product code moved
@answer n=9 entity=standard-contracts#5.5 v=no
why: the -32602 and -32603 causes of link and unlink are unchanged and no product file moved
@answer n=10 entity=standard-contracts#6.1 v=no
why: the peer euid and pid checks and the macOS no-peer-pid carve-out are unchanged, the session only added test-side gating
@answer n=11 entity=standard-contracts#7.1 v=no
why: each client's strict-modes refusal is unchanged and no product file moved
@answer n=12 entity=standard-contracts#8.1 v=no
why: send's reply, budget refusal and cursor are unchanged and no product file moved
@answer n=13 entity=standard-contracts#9.1 v=no
why: wait's parameters, wake set and cursor are unchanged, the session touched no wait code
@answer n=14 entity=standard-contracts#10.1 v=no
why: link and unlink read no from and their reply is unchanged
@answer n=15 entity=standard-contracts#11.1 v=no
why: the ProtocolError cause mapping is unchanged and no product file moved
@answer n=16 entity=standard-contracts#12.1 v=no
why: hook.event validation and append are unchanged, and plugin/ and the hook sources were not touched
@answer n=17 entity=standard-contracts#13.1 v=no
why: the list ok payload and the CLI list reads are unchanged, the witness only runs viola list on a terminal
@answer n=18 entity=standard-contracts#14.1 v=no
why: the prompt-submitted text and origin rules are unchanged and the session touched no hook or delivery code
@answer n=19 entity=standard-contracts#15.1 v=no
why: the link and unlink CLI JSON documents and exit codes are unchanged
@answer n=20 entity=standard-contracts#15.2 v=no
why: the six closed exit-21 causes are unchanged and no product file moved
@answer n=21 entity=standard-contracts#16.1 v=no
why: the refusal document of the six channel verbs is unchanged and no product file moved
@answer n=22 entity=standard-contracts#16.2 v=no
why: unreachable_document and refused_document are product code the session did not change
@answer n=23 entity=standard-contracts#16.3 v=no
why: sanitised_fault's codes and fixed messages are unchanged and no product file moved
@answer n=24 entity=standard-contracts#16.4 v=no
why: the wrapper fault fallback text is unchanged and no product file moved
@answer n=25 entity=standard-contracts#16.5 v=no
why: the Launch json refusal path is unchanged and no src file moved
