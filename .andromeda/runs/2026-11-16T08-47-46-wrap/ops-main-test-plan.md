@row_set n=1 row=decision/local-command-unconfirmable cell=rule seen=1 detector=none
value: A local command with no measured post-condition, sent to an unverified CLI, or confirmed by its transcript echo with no readable transcript, gives unconfirmable with no confirmation window.
basis: fct-ywi7ddhz
because: An echo-confirmed command with no remembered or readable transcript now reads unconfirmable, a case the standing rule leaves out; the proposer's longer set named the command inventory the code holds (fit: not_master type 5).
@row_retire n=1 row=decision/local-command-exact-match seen=1 detector=none
why: Local commands are no longer recognised by exact equality of the typed text but by their name; the rule's one home is architecture decision/local-commands-by-ledger.
basis: fct-vjdxki2s
because: LOCAL_COMMANDS moved from 3 to 6 names and classification from exact text to by name, so the exact-match row is false, and the by-name rule keeps its one home in architecture (check 2).
