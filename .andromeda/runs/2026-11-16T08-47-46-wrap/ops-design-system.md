@row_set n=1 row=contract/send-readback/unconfirmable cell=promise seen=1 fact=F2 detector=none
value: A local command with no post-condition (`/remote-control`, `/cost`, `/usage`) stays `unconfirmable`; so does a `/compact` send on an unverified CLI, and a `/mcp` send on an unverified CLI or in a session that declared no transcript.
basis: fct-ywi7ddhz, fct-7qwkgyz4
because: The local-command list now holds commands with no post-condition and an echo command that confirms nothing without a verified CLI and a declared transcript, and the row names `/compact` alone.
@row_add n=1 row=contract/send-readback/mcp area=brand-identity fact=F1 detector=none
boundary: Send readback
item: /mcp
promise: On a verified CLI a `/mcp` send counts, and reads back, only once a new local-command stdout tag appears in the session transcript after the send; it prints no new word.
basis: fct-ywi7ddhz, fct-7qwkgyz4, fct-mlisq4em
because: The chunk adds the stdout echo as a further confirmer of a local command, and the send readback contract holds one row a command.
@row_add n=2 row=decision/local-commands-by-name area=surface-cli fact=F3 detector=none
name: Local commands recognised by name
rule: A sent text is a local slash command when it is exactly one of the six known names (`/clear`, `/remote-control`, `/compact`, `/mcp`, `/cost`, `/usage`) or that name followed by one space and its arguments.
why: Ratified.
basis: fct-ywi7ddhz, fct-vjdxki2s, fct-vrnudrgx
because: Classification moved from exact text to by-name over six known names, a ratified decision no row of the surface states.
@row_none n=1 fact=F4 type=5
what: The relied-row count and the 22nd row's id are the ledger's inventory; the code holds them, and the step-line and summary rows state the count as the relied count.
