@row_set n=1 row=contract/viola-send-output/unconfirmable cell=promise seen=1 fact=F2 detector=none
value: `[  ] unconfirmable …` prints on stdout with exit 0 for a local command with no measured post-condition: /remote-control, /cost and /usage; /compact, /clear or /mcp on an unverified CLI version; and /mcp when the session declared no readable transcript.
basis: fct-ywi7ddhz
because: The local command list grew to six by name, with /cost and /usage given no post-condition and /mcp confirmed only through a readable transcript, so the row's list of unconfirmable cases no longer covers what the send renders.
@row_add n=1 row=contract/viola-send-output/read-back-mcp area=surface-cli fact=F1 detector=none
boundary: viola send output
item: read back for /mcp
promise: On a verified CLI version, /mcp, alone or with arguments, is read back as `[RB] read back` when a new stdout tag lands in the session transcript after the send, the transcript read from its end as it stood before the send.
basis: fct-ywi7ddhz
because: The send now confirms /mcp by its own measured echo in the transcript, a read-back case beside /clear and /compact that no row of the surface states.
@row_add n=2 row=decision/local-command-by-name area=surface-cli fact=F3 detector=none
name: Local commands recognised by name
rule: viola send treats a text as a local command when it is a known local command name alone, or that name, one space and a non-empty rest; any other slash text still reads not-delivered.
why: Ratified as a standing rule, not provisional.
basis: fct-ywi7ddhz
because: Classification moved from exact text to the command's name, a decision that decides which slash texts the send reads back, and no row of the surface states it.
@row_none n=1 fact=F4 type=5
what: the relied-row count 22 and the 22nd row's probe paste are the ledger's inventory and count; the step-line and summary rows already state the counter as the relied count, and the ledger code and architecture hold the row itself.
