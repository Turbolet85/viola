@answer n=1 entity=standard-contracts#1.1 v=no
why: the session moves no list payload item field or liveness reading, and its stale-row witness exercises the live-or-stale rule this entity states, which holds unchanged
@answer n=2 entity=standard-contracts#2.0 v=no
why: the snapshot envelope, the replay fallback and the STALE_AFTER read are unchanged, and the session adds no snapshot field or pid reading
@answer n=3 entity=standard-contracts#3.0 v=no
why: no ledger row, stamp envelope or relied() set changed, and the session's new files never write ledger/stamps.json
@answer n=4 entity=standard-contracts#4.1 v=no
why: update_stamps and strict::check_stamps are product code under src/ that the test-only session did not change
@answer n=5 entity=standard-contracts#5.1 v=no
why: the links set, its restore and the wrapper's LinkSet are product code under src/run/ that the session did not change
@answer n=6 entity=standard-contracts#5.2 v=no
why: LinkSet::link and LinkSet::unlink are unchanged product code, and the session adds no link path
@answer n=7 entity=standard-contracts#5.3 v=no
why: the already_linked and since answers of a link call are product behaviour the session did not change
@answer n=8 entity=standard-contracts#5.4 v=no
why: the empty answer of unlink is product behaviour the session did not change
@answer n=9 entity=standard-contracts#5.5 v=no
why: the -32602 and -32603 causes of link and unlink are product behaviour the session did not change
@answer n=10 entity=standard-contracts#6.1 v=no
why: the peer euid check and the macOS no-peer-pid carve-out are unchanged, and the session only counts the cases that carve-out produces
@answer n=11 entity=standard-contracts#7.1 v=no
why: the strict-modes checks and refuse_untrusted are product code the session did not touch
@answer n=12 entity=standard-contracts#8.1 v=no
why: send's reply shape, budget gate and link rungs are product behaviour the session did not change
@answer n=13 entity=standard-contracts#9.1 v=no
why: wait's wake set, deadline rule and feed are product behaviour the session did not change, and the Frozen guard is test support only
@answer n=14 entity=standard-contracts#10.1 v=no
why: link and unlink still read no from field, and the session adds no product path that reads one
@answer n=15 entity=standard-contracts#11.1 v=no
why: ProtocolError::cause in viola_channel is product code the session did not change
@answer n=16 entity=standard-contracts#12.1 v=no
why: hook.event's validation and append are product behaviour, and the session adds no hook event or hook output
@answer n=17 entity=standard-contracts#13.1 v=no
why: the list reads, the claude agents join and the payload shape are unchanged, and the session's witness only runs the existing list path under a PTY
@answer n=18 entity=standard-contracts#14.1 v=no
why: the prompt-submitted text rule and the CLI long-paste handling are unchanged, and the session adds no event or text handling
@answer n=19 entity=standard-contracts#15.1 v=no
why: the link and unlink --json documents and their exit-21 refusal are product output the session did not change
@answer n=20 entity=standard-contracts#15.2 v=no
why: the six closed exit-21 causes are unchanged, and the session adds no cause
@answer n=21 entity=standard-contracts#16.1 v=no
why: the exit-21 document of the six channel verbs is product output the session did not change
@answer n=22 entity=standard-contracts#16.2 v=no
why: client::unreachable_document and the other src/cmd writers are product code the session did not change
@answer n=23 entity=standard-contracts#16.3 v=no
why: viola_channel::sanitised_fault is product code the session did not change
@answer n=24 entity=standard-contracts#16.4 v=no
why: the fixed wrapper fault output word is product output the session did not change
@answer n=25 entity=standard-contracts#16.5 v=no
why: the Launch json flag and refused_with are product code under src/cmd/ that the session did not change
