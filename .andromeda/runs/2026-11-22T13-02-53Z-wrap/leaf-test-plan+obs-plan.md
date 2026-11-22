@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=27 class=omission
leaf-text: Channel lines carry `conn = "<process>-<pid>-<t0>-<n>"`; wrapper `send-*` lines carry `conn` + `rpc_id`;
source: ctr-47amdxiq «A server receiving no conn, or one outside ^[a-z]+-\d+-\d+-\d+$ or over 64 bytes, logs srv_conn = "srv-<accept counter>" instead.»
replace-with: Channel lines carry `conn = "<process>-<pid>-<t0>-<n>"` (a server receiving no conn, or one outside `^[a-z]+-\d+-\d+-\d+$` or over 64 bytes, logs `srv_conn = "srv-<accept counter>"` instead); wrapper `send-*` lines carry `conn` (or `srv_conn`) + `rpc_id`;
@leaf_edit n=2 leaf=.claude/rules/observability.md line=35 class=stale
leaf-text: the two `run` PTY pump threads
source: dec-4alyzcfr «The three run PTY threads, the two pump copy threads and the paste writer thread, each run inside catch_unwind»
replace-with: the three `run` PTY threads (the two pump copy threads and the paste writer thread)
@leaf_edit n=3 leaf=.claude/rules/observability.md line=41 class=omission
leaf-text: and carry no key and no prompt text.
source: dec-4fx5qzj2 «The cut mark a cut paste leaves is in-memory wrapper state that opens no span and writes no line of its own; its set shows as the cut send's send-refused paste-cut record»
replace-with: and carry no key and no prompt text. The cut mark a cut paste leaves is in-memory wrapper state too: it opens no span and writes no line of its own, its set showing as the cut send's `send-refused` `paste-cut` record and its clears as their own `prompt-submitted`, `session-start` and `session-end` records.
