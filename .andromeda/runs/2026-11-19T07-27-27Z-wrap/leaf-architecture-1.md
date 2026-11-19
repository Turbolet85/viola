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
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=54 class=omission
leaf-text: and the fake agent and the rig, which read a config's trust record)
source: dec-uun4jqhm «VH real-config key names, needle; fake agent, rig: trust records.»
replace-with: and VH, which reads the real config's key names and a needle, never a value, and the fake agent and the rig, which read a config's trust record)
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=6 class=omission
leaf-text: the fake agent and the rig read a config's trust record)
source: dec-zypgxbrh «VH real-config key names, needle; fake agent, rig: trust records.»
replace-with: VH reads the real config's key names and a needle, never a value; the fake agent and the rig read a config's trust record)
