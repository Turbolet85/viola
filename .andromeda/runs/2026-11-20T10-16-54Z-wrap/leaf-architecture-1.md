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
@leaf_edit n=1 leaf=.claude/docs/services/viola-agent-claude.md line=11 class=stale
leaf-text: a modal row refuses at once
source: dec-xsqsrc2s «A modal literal's row is a quote only when it is not option-shaped and the box's own rows stand below it; a quote alone on its own line, or a code-block line holding only the label, still refuses.»
replace-with: a standing modal refuses at once (a modal literal's row is a quote, which refuses nothing, only when it is not option-shaped and the box's own rows stand below it; a quote alone on its own line, or a code-block line holding only the label, still refuses)
