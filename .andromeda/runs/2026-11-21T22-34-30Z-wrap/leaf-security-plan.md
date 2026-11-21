@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=46
last: - 2026-11-21: A promise that a value reaches no fixture cove
@leaf_edit n=1 leaf=.claude/rules/security.md line=40 class=omission
leaf-text: `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`, as in CLI `--json`'s exit-21 document, or `wrapper-fault`
source: ctr-4yst4ixk «A closed cause of instance-unreachable, written by the MCP frame tools alone when a call reaches its call deadline; the CLI exit-21 cause set stays six and never carries it.»
replace-with: `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`, as in CLI `--json`'s exit-21 document, plus `call-deadline` on MCP alone when a frame tool call reaches its call deadline (the CLI's six never carry it), or `wrapper-fault`
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=73 class=omission
leaf-text: its MCP config is a private 0600 file inside the scratch, removed with it.
source: dec-cqj5qfn7 «The live rig's MB arm runs a print-mode claude -p as the MCP client of viola mcp over the fake agent, in its scratch home and private config, one tool allowed (wait), no key, no dialog; it reads the»
replace-with: its MCP config is a private 0600 file inside the scratch, removed with it. The live rig's MB arm runs a print-mode `claude -p` as the MCP client of `viola mcp` over the fake agent, in its scratch home and private config, one tool allowed (`wait`), no key, no dialog; it reads the client's output in memory and keeps only two booleans, one number and one closed word; it reads the credentials file once per start, the prompt and the tool result go to the model provider, and nothing reaches a fixture, stamp or log, or the real `~/.claude*` or `~/.viola`.
