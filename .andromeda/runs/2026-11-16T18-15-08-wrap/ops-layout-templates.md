@row_set n=1 row=contract/viola-wait-output/turn-ended cell=promise seen=1 fact=F1 detector=none
value: `turn-ended  <name>  <ts>  cursor <n>` on stdout; every kind without a dialog_id takes the turn-ended form, an API-error turn end included, whose cause word (one closed word of nine) is never printed and crosses only under --json and in the MCP wait tool, never with the CLI's error text.
basis: fct-xw6ru4by; fct-apskfot5; fct-pqfgw3kz
because: The turn-end cause grew from one word to a closed set of nine that reaches only `wait --json` and the MCP `wait` tool with no human-output change, so the human turn-ended row must say an API-error turn end prints the same form without its cause word.
