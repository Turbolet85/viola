@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=16 class=stale
leaf-text: each decoded to one closed class (0x03 anywhere interrupt, exactly q quit, anything else unbound), no key byte reaching any log line or output.
source: dec-dsjweani «viola console reads the human's keys from its own terminal in raw mode in reads of at most 64 bytes, each read decoded into keys of closed classes, an escape sequence bounded in its wait and its length; no key byte reaches any log line.»
replace-with: each read decoded into keys of closed classes, an escape sequence bounded in its wait and its length, its editing line held to at most `MAX_FRAME` bytes, no key byte reaching any log line.
