@answer n=1 entity=standard-contracts#1.1 v=no
why: the list payload item and budget shapes are unchanged, and the Windows-only remnants in list.rs leave the statusline window rule that already covers an omitted five_hour window
@answer n=2 entity=standard-contracts#2.0 v=no
why: the snapshot envelope, the replay fallback and the board read rule are unchanged, and the events.rs and strict.rs changes are Windows-only remnants
@answer n=3 entity=standard-contracts#3.0 v=no
why: the ledger stamps envelope and the all-pass rule still hold, since the ledger stays at twenty-four rows and the dev home's stamps are not moved
@answer n=4 entity=standard-contracts#4.1 v=no
why: stamps.rs changed only Windows-only remnants, and update_stamps still runs the strict check and returns the same StampsError variants
@answer n=5 entity=standard-contracts#5.1 v=no
why: the links restore and its replay through events::read_from are unchanged, and events.rs changed only Windows-only remnants
@answer n=6 entity=standard-contracts#5.2 v=no
why: the implicit and served link and unlink changes are unchanged, and no channel method was added
@answer n=7 entity=standard-contracts#5.3 v=no
why: the already_linked and since answers of a link are unchanged
@answer n=8 entity=standard-contracts#5.4 v=no
why: unlink still answers an empty object whether or not the pair was held
@answer n=9 entity=standard-contracts#5.5 v=no
why: the -32602 and -32603 fault causes for link and unlink are unchanged, and no fault code was added
@answer n=10 entity=standard-contracts#6.1 v=no
why: client.rs changed only Windows-only remnants, and the Linux peer euid, pid and start-time checks still hold
@answer n=11 entity=standard-contracts#7.1 v=no
why: refuse_untrusted, the hook trusted check and the endpoint directory check keep their behaviour, and client.rs and hook.rs changed only Windows-only remnants
@answer n=12 entity=standard-contracts#8.1 v=no
why: send's reply shape and its budget-paused refusal are unchanged, and the removed Git Bash warning was never part of the channel reply
@answer n=13 entity=standard-contracts#9.1 v=no
why: wait.rs is not reached, and wait's wake set, its reply and its start offset are unchanged
@answer n=14 entity=standard-contracts#10.1 v=no
why: link and unlink still read no from, and no from-handling was touched
@answer n=15 entity=standard-contracts#11.1 v=no
why: viola-channel is not reached, and the ProtocolError cause table is unchanged
@answer n=16 entity=standard-contracts#12.1 v=no
why: hook.rs changed only for the Windows exec-form and EXE_SUFFIX reasons, and the hook.event validation and append rule still hold
@answer n=17 entity=standard-contracts#13.1 v=no
why: list.rs and the claude resolution changed only Windows-only remnants, and the claude agents join and the list document rules are unchanged
@answer n=18 entity=standard-contracts#14.1 v=no
why: viola-agent-claude changed only Windows-only remnants, and the pasted-content unwrapping and origin rules are unchanged
@answer n=19 entity=standard-contracts#15.1 v=no
why: the link and unlink JSON documents and the exit-21 refusal are unchanged, and client.rs lost only Windows wording
@answer n=20 entity=standard-contracts#15.2 v=no
why: the six closed exit-21 causes are unchanged, and no detail or exit code was added
@answer n=21 entity=standard-contracts#16.1 v=no
why: the strict-modes and socket-dir refusal documents for the six verbs are unchanged, and client.rs lost only Windows wording
@answer n=22 entity=standard-contracts#16.2 v=no
why: client::unreachable_document and list's refused_document remain the only writers, and no new writer was added
@answer n=23 entity=standard-contracts#16.3 v=no
why: viola-channel is not reached, and the sanitised_fault reader is unchanged
@answer n=24 entity=standard-contracts#16.4 v=no
why: the fixed wrapper fault text and its human line are unchanged, and no fault code was added
@answer n=25 entity=standard-contracts#16.5 v=no
why: run.rs is not reached, and the Launch json documents for pinned-hash-mismatch, squatted-name and socket-dir-refused are unchanged
