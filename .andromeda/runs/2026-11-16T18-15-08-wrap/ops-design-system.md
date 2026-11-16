@row_set n=1 row=decision/api-error-turn-end-shown-as-turn-end cell=rule seen=1 detector=D-design-cli-wording
value: A turn the CLI ends on an API error shows like any turn end: STATUS reads `idle`, `wait` reads `turn-ended`, no distinct status, colour or hint marks it, the only trace is one closed cause word of nine in `--json` and MCP `wait` event data, and the CLI's error text is shown nowhere.
basis: fct-pqfgw3kz, fct-xw6ru4by
because: The turn-end cause grew from the one word `api-error` to a closed set of nine words reaching `--json` and MCP `wait` with no human-output change, so the rule's «the closed cause word `api-error`» no longer holds.
