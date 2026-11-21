@row_add n=1 row=decision/live-rig-mb-arm-keeps-no-output area=data-protection fact=F3 detector=none
name: Live rig's MB arm keeps only closed words of the client
rule: The live rig's MB arm runs a print-mode claude -p as the MCP client of viola mcp over the fake agent, in its scratch home and private config, one tool allowed (wait), no key, no dialog; it reads the client's output in memory and keeps only two booleans, one number and one closed word.
why: The shape the operator ratified; a long call counts only with timed-out true beside call-error false, so a result with no is_error key cannot turn a cut call green.
except: The rig reads the credentials file once per start, and the prompt and the tool result go to the model provider; nothing reaches a fixture, stamp or log, or the real ~/.claude* or ~/.viola.
state: provisional
basis: fct-wvnepvu3; fct-rkhivonu; fct-wl4q6mbh
because: The MB arm is owed a row of its own beside the recording arm's (P4-F2), carrying the shape ratified in «3 базовых + 1 запасной» and the long-call predicate the directive names (inputs#I7 item 6).
