@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_edit n=1 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=stale
leaf-text: `LedgerRow`, twenty-four closed rows
source: dec-lbocs2om «The local-command-context relied ledger row holds that /context writes its stdout tag to the transcript and submits no prompt within the confirmation window»
replace-with: `LedgerRow`, twenty-five closed rows
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=omission
leaf-text: `local-command-echo` · `trust-record` (probed
source: dec-lbocs2om «The local-command-context relied ledger row holds that /context writes its stdout tag to the transcript and submits no prompt within the confirmation window»
replace-with: `local-command-echo` · `local-command-context` (relied; `/context` writes its stdout tag to the transcript and submits no prompt within the confirmation window) · `trust-record` (probed
@leaf_edit n=3 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=stale
leaf-text: in its order, all twenty-four)
source: dec-lbocs2om «The local-command-context relied ledger row holds that /context writes its stdout tag to the transcript and submits no prompt within the confirmation window»
replace-with: in its order, all twenty-five)
@leaf_edit n=4 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=stale
leaf-text: all twenty-four on every build (`local-command-echo` and `trust-record` among them, so a stamp of the older, shorter row set written before chunk 2026-11-16-local-command-delivery reads unverified)
source: dec-lbocs2om «The local-command-context relied ledger row holds that /context writes its stdout tag to the transcript and submits no prompt within the confirmation window»
replace-with: all twenty-five on every build (`local-command-echo`, `trust-record` and `local-command-context` among them, so a stamp of the older, shorter row set written before chunk 2026-11-19-context-as-a-local-command reads unverified)
@leaf_edit n=5 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=stale
leaf-text: Run B's seven compiled paste texts (the seventh
source: dec-lbocs2om «verify probes it by Run B's eighth paste, /context, and a failing read of it is fail, never absent.»
replace-with: Run B's eight compiled paste texts (the eighth `/context`, which probes `local-command-context`, a failing read of it `fail`, never `absent`; the seventh
