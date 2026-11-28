@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=47
last: - 2026-11-24: A new verb that starts through `run::start` inh
@leaf_edit n=1 leaf=.claude/rules/security.md line=25 class=stale
leaf-text: each decoded to one closed class (0x03 anywhere interrupt, exactly `q` quit, anything else unbound; no key byte reaches any log line or output)
source: dec-dsjweani «viola console reads the human's keys from its own terminal in raw mode in reads of at most 64 bytes, each read decoded into keys of closed classes, an escape sequence bounded in its wait and its length; no key byte reaches any log line.»
replace-with: each read decoded into keys of closed classes, an escape sequence bounded in its wait and its length (no key byte reaches any log line), its editing line holding at most `MAX_FRAME` bytes (a char past the bound counted unbound and appended to nothing)
