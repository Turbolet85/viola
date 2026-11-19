@row_set n=1 row=contract/viola-send-output/unconfirmable cell=promise seen=2 fact=F1 detector=D-design-cli-wording
value: `[  ] unconfirmable …` prints on stdout with exit 0 for a local command with no measured post-condition: /remote-control, /cost and /usage; /compact, /clear, /mcp or /context on an unverified CLI version; /mcp when the session declared no readable transcript; /context before its transcript exists.
basis: fct-62lgvvsj
because: /context is now a listed local command whose send reads unconfirmable on an unverified CLI and before its transcript exists, so the row's list of unconfirmable cases must name it.
@row_add n=1 row=contract/viola-send-output/read-back-context area=surface-cli fact=F1 detector=D-design-cli-wording
boundary: viola send output
item: read back for /context
promise: On a verified CLI version, /context is read back as `[RB] read back` when a new stdout tag lands in the session transcript after the send; /context submits no prompt and is never read back by a turn end.
basis: fct-z3derqaa
because: The chunk lists /context as a local command confirmed by its stdout tag, a new send read-back case beside /clear, /compact and /mcp that no row states.
